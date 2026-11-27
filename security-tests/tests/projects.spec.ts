import { randomUUID } from 'node:crypto';
import type { APIRequestContext } from '@playwright/test';
import { bytes, claimToken, expect, isRefused, json, pair, test, type Project } from '../lib/api';

type Row = { id: number; payload: { ct: string; iv: string } };

async function expenseOf(api: APIRequestContext, project: Project): Promise<Row> {
  return json<Row>(await api.get(`/api/v1/projects/${project.id}/expenses/${project.expenseId}`, project.withKey), 'read expense');
}

async function usersOf(api: APIRequestContext, project: Project): Promise<Row[]> {
  return json<Row[]>(await api.get(`/api/v1/projects/${project.id}/users`, project.withKey), 'read participants');
}

async function dataVersion(api: APIRequestContext, project: Project): Promise<number> {
  return (await json<{ dataVersion: number }>(await api.get(`/api/v1/projects/${project.id}/sync/0`, project.withKey), 'sync')).dataVersion;
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

  test("B2 one project's key gives nothing in another project", async ({ anon, project }) => {
    const a = await project();
    const b = await project();
    const expenseBefore = await expenseOf(anon, b);
    const usersBefore = await usersOf(anon, b);

    expect((await anon.get(`/api/v1/projects/${a.id}/expenses/${b.expenseId}`, a.withKey)).status(), 'read').toBe(404);

    const edit = await anon.put('/api/v1/expenses', {
      ...a.withKey,
      data: { id: b.expenseId, projectId: a.id, payload: pair(256), payers: [], debtors: [] },
    });
    expect(isRefused(edit.status()), `edit answered ${edit.status()}`).toBe(true);

    await anon.post('/api/v1/expenses/delete', { ...a.withKey, data: { id: b.expenseId, projectId: a.id } });
    await anon.put('/api/v1/users', { ...a.withKey, data: { id: b.userIds[0], projectId: a.id, payload: pair() } });
    await anon.put('/api/v1/users/batch', { ...a.withKey, data: [{ id: b.userIds[0], projectId: a.id, payload: pair() }] });
    await anon.delete(`/api/v1/projects/${a.id}/users/${b.userIds[1]}`, a.withKey);

    expect(await expenseOf(anon, b), "B's expense is unchanged").toEqual(expenseBefore);
    expect(await usersOf(anon, b), "B's participants are unchanged").toEqual(usersBefore);

    expect(await json<unknown[]>(await anon.get(`/api/v1/projects/${b.id}/payments`, b.withKey), 'B payments')).not.toHaveLength(0);
    const throughA = await anon.get(`/api/v1/projects/${a.id}/expenses/${b.expenseId}/payments`, a.withKey);
    if (throughA.ok()) {
      expect(await throughA.json(), "B's payment rows through A").toHaveLength(0);
    } else {
      expect(isRefused(throughA.status())).toBe(true);
    }
  });

  test('B3 claiming a participant needs the project key', async ({ account, project }) => {
    const { api } = await account();
    const p = await project();
    const claim = (token: number[] | undefined, headers = {}) =>
      api.post('/api/v1/account/projects', { headers, data: { projectId: p.id, userId: p.userIds[0], claimToken: token } });

    expect((await claim(undefined)).status(), 'no token').toBe(403);
    expect((await claim(bytes(32))).status(), 'wrong token').toBe(403);
    expect((await claim(claimToken(p.key), p.withKey.headers)).ok(), 'the right token').toBe(true);
  });

  test('B3 converting a project needs the project key', async ({ anon, project }) => {
    const p = await project();
    const convert = async (token: number[] | undefined, headers = {}) =>
      anon.post('/api/v1/projects/convert', {
        headers,
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
    expect((await convert(claimToken(p.key), p.withKey.headers)).ok(), 'the right token').toBe(true);
  });

  test('B5 what a project serves is ciphertext plus the documented fields', async ({ anon, project }) => {
    const p = await project();

    const dto = await json(await anon.get(`/api/v1/projects/${p.id}`, p.withKey), 'project');
    expect(keysOf(dto)).toEqual([
      'createdAt',
      'hasAdmin',
      'hasCreatorProof',
      'id',
      'keyEpoch',
      'keyLifecycle',
      'payload',
      'readOnly',
      'rev',
      'status',
    ]);

    // `eid` only beside a row created under an entity id, which this fixture never names.
    for (const user of await usersOf(anon, p)) {
      expect(keysOf(user)).toEqual(['admin', 'claimLabel', 'claimed', 'createdAt', 'id', 'payload', 'paymentMethods', 'rev']);
    }

    const expense = await expenseOf(anon, p);
    expect(keysOf(expense)).toEqual(['authorId', 'createdAt', 'id', 'payload', 'projectId', 'rev']);

    for (const payment of await json<unknown[]>(await anon.get(`/api/v1/projects/${p.id}/payments`, p.withKey), 'payments')) {
      expect(keysOf(payment)).toEqual(['createdAt', 'expenseId', 'id', 'payload', 'userId']);
    }

    const sync = await json<Record<string, unknown>>(await anon.get(`/api/v1/projects/${p.id}/sync/0`, p.withKey), 'sync');
    expect(keysOf(sync)).toEqual(['dataVersion', 'expenses', 'payments', 'project', 'recurring', 'serverDate', 'serverTime', 'users']);

    const history = await json<Record<string, unknown>[]>(await anon.get(`/api/v1/projects/${p.id}/history`, p.withKey), 'history');
    for (const entry of history) {
      expect(keysOf(entry)).not.toContain('actorAccountId');
    }

    for (const row of [dto, expense, ...(await usersOf(anon, p))] as { payload: Record<string, unknown> }[]) {
      expect(keysOf(row.payload)).toEqual(['ct', 'iv']);
    }
  });

  test('B7 a creation under a taken project id is refused and tells the caller nothing but taken', async ({ anon, project }) => {
    const [a, b] = [await project(), await project()];
    const before = await json(await anon.get(`/api/v1/projects/${a.id}`, a.withKey), 'project');
    const create = (id: string) => anon.post('/api/v1/projects', { data: { id, payload: pair(256), demo: false } });

    const underA = await create(a.id);
    const underB = await create(b.id);
    const refusal = await underA.text();
    expect(underA.status(), refusal).toBe(409);
    expect(refusal).toContain('This id is already in use');
    expect(refusal, 'nothing of the project').not.toContain(a.id);
    expect(underB.status()).toBe(409);
    expect(await underB.text(), 'the same answer for any taken id').toBe(refusal);
    expect(await json(await anon.get(`/api/v1/projects/${a.id}`, a.withKey), 'project'), 'the project is unchanged').toEqual(before);

    expect((await create('00000000-0000-0000-0000-000000000000')).status(), 'the all-zero id').toBe(400);
    const upper = randomUUID();
    expect(isRefused((await create(upper.toUpperCase())).status()), 'an id in another form').toBe(true);
    expect((await anon.get(`/api/v1/projects/${upper}`)).status(), 'and nothing was stored under it').not.toBe(200);

    const fresh = randomUUID();
    expect((await json<{ id: string }>(await create(fresh), 'a fresh id')).id, 'stored as sent').toBe(fresh);
  });

  test('B6 an id holder without the key writes rows below the floor, and nothing once it is reached', async ({ anon, project }) => {
    const p = await project();
    const write = await anon.post('/api/v1/expenses', { data: { projectId: p.id, payload: pair(256), payers: [], debtors: [] } });
    if (p.keyLifecycle) {
      const madeUp = { headers: { 'x-project-token': Buffer.from(bytes(32)).toString('base64url') } };
      expect(write.status(), 'from KEY_LIFECYCLE_FROM, the id alone writes nothing').toBe(403);
      expect((await anon.get(`/api/v1/projects/${p.id}/sync/0`)).status(), 'and reads nothing').toBe(403);
      expect((await anon.get(`/api/v1/projects/${p.id}/sync/0`, madeUp)).status(), 'nor with a made-up token').toBe(403);
    } else {
      expect(write.ok(), 'below the floor, SECURITY.md 5.6: the project id is an access token').toBe(true);
    }
  });
});
