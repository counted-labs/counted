import type { APIRequestContext } from '@playwright/test';
import { bytes, claimToken, expect, isRefused, json, pair, test, type Project } from '../lib/api';

type Row = { id: number; payload: { ct: string; iv: string } };

async function expenseOf(api: APIRequestContext, project: Project): Promise<Row> {
  return json<Row>(await api.get(`/api/v1/projects/${project.id}/expenses/${project.expenseId}`), 'read expense');
}

async function usersOf(api: APIRequestContext, project: Project): Promise<Row[]> {
  return json<Row[]>(await api.get(`/api/v1/projects/${project.id}/users`), 'read participants');
}

async function dataVersion(api: APIRequestContext, project: Project): Promise<number> {
  return (await json<{ dataVersion: number }>(await api.get(`/api/v1/projects/${project.id}/sync/0`), 'sync')).dataVersion;
}

function keysOf(value: unknown): string[] {
  return Object.keys(value as Record<string, unknown>).sort();
}

test.describe('B - project boundaries', () => {
  test('B1 there is no endpoint that lists projects without a session', async ({ anon }) => {
    const res = await anon.get('/api/v1/projects');
    const body = await res.text();
    let listed = false;
    try {
      listed = Array.isArray(JSON.parse(body));
    } catch {
      listed = false;
    }
    expect(listed, `GET /api/v1/projects answered ${res.status()} with a JSON list`).toBe(false);
  });

  test("B2 one project's id gives nothing in another project", async ({ anon, project }) => {
    const a = await project();
    const b = await project();
    const expenseBefore = await expenseOf(anon, b);
    const usersBefore = await usersOf(anon, b);

    expect((await anon.get(`/api/v1/projects/${a.id}/expenses/${b.expenseId}`)).status(), 'read').toBe(404);

    const edit = await anon.put('/api/v1/expenses', {
      data: { id: b.expenseId, projectId: a.id, payload: pair(256), payers: [], debtors: [] },
    });
    expect(isRefused(edit.status()), `edit answered ${edit.status()}`).toBe(true);

    await anon.post('/api/v1/expenses/delete', { data: { id: b.expenseId, projectId: a.id } });
    await anon.put('/api/v1/users', { data: { id: b.userIds[0], projectId: a.id, payload: pair() } });
    await anon.put('/api/v1/users/batch', { data: [{ id: b.userIds[0], projectId: a.id, payload: pair() }] });
    await anon.delete(`/api/v1/projects/${a.id}/users/${b.userIds[1]}`);

    expect(await expenseOf(anon, b), "B's expense is unchanged").toEqual(expenseBefore);
    expect(await usersOf(anon, b), "B's participants are unchanged").toEqual(usersBefore);

    expect(await json<unknown[]>(await anon.get(`/api/v1/projects/${b.id}/payments`), 'B payments')).not.toHaveLength(0);
    const throughA = await anon.get(`/api/v1/projects/${a.id}/expenses/${b.expenseId}/payments`);
    if (throughA.ok()) {
      expect(await throughA.json(), "B's payment rows through A").toHaveLength(0);
    } else {
      expect(isRefused(throughA.status())).toBe(true);
    }
  });

  test('B3 claiming a participant needs the project key', async ({ account, project }) => {
    const { api } = await account();
    const p = await project();
    const claim = (token: number[] | undefined) =>
      api.post('/api/v1/account/projects', { data: { projectId: p.id, userId: p.userIds[0], claimToken: token } });

    expect((await claim(undefined)).status(), 'no token').toBe(403);
    expect((await claim(bytes(32))).status(), 'wrong token').toBe(403);
    expect((await claim(claimToken(p.key))).ok(), 'the right token').toBe(true);
  });

  test('B3 converting a project needs the project key', async ({ anon, project }) => {
    const p = await project();
    const convert = async (token: number[] | undefined) =>
      anon.post('/api/v1/projects/convert', {
        data: {
          projectId: p.id,
          expectedDataVersion: await dataVersion(anon, p),
          expenses: [],
          recurring: [],
          history: [],
          claimToken: token,
        },
      });

    expect((await convert(undefined)).status(), 'no token').toBe(403);
    expect((await convert(bytes(32))).status(), 'wrong token').toBe(403);
    expect((await convert(claimToken(p.key))).ok(), 'the right token').toBe(true);
  });

  test('B5 what any id holder reads is ciphertext plus the documented fields', async ({ anon, project }) => {
    const p = await project();

    const dto = await json(await anon.get(`/api/v1/projects/${p.id}`), 'project');
    expect(keysOf(dto)).toEqual([
      'createdAt',
      'hasAdmin',
      'hasCreatorProof',
      'id',
      'keyEpoch',
      'keyLifecycle',
      'payload',
      'readOnly',
      'status',
    ]);

    for (const user of await usersOf(anon, p)) {
      expect(keysOf(user)).toEqual(['admin', 'claimLabel', 'claimed', 'createdAt', 'id', 'payload', 'paymentMethods']);
    }

    const expense = await expenseOf(anon, p);
    expect(keysOf(expense)).toEqual(['authorId', 'createdAt', 'id', 'payload', 'projectId']);

    for (const payment of await json<unknown[]>(await anon.get(`/api/v1/projects/${p.id}/payments`), 'payments')) {
      expect(keysOf(payment)).toEqual(['createdAt', 'expenseId', 'id', 'payload', 'userId']);
    }

    const sync = await json<Record<string, unknown>>(await anon.get(`/api/v1/projects/${p.id}/sync/0`), 'sync');
    expect(keysOf(sync)).toEqual(['dataVersion', 'expenses', 'payments', 'project', 'recurring', 'serverDate', 'serverTime', 'users']);

    const history = await json<Record<string, unknown>[]>(await anon.get(`/api/v1/projects/${p.id}/history`), 'history');
    for (const entry of history) {
      expect(keysOf(entry)).not.toContain('actorAccountId');
    }

    for (const row of [dto, expense, ...(await usersOf(anon, p))] as { payload: Record<string, unknown> }[]) {
      expect(keysOf(row.payload)).toEqual(['ct', 'iv']);
    }
  });

  test('B6 design: an id holder without the key can write rows, which members cannot read', async ({ anon, project }) => {
    const p = await project();
    const res = await anon.post('/api/v1/expenses', { data: { projectId: p.id, payload: pair(256), payers: [], debtors: [] } });
    expect(res.ok(), 'documented in SECURITY.md 5.6: the project id is an access token').toBe(true);
  });
});
