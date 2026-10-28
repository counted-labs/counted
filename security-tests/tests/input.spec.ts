import { randomBytes } from 'node:crypto';
import { bytes, expect, isRefused, json, pair, test } from '../lib/api';

const MAX_PAYLOAD_LENGTH = 65536;
const MAX_EXPENSE_BATCH = 500;

const internalDetail = /sqlx|postgres|panicked|serde|stack backtrace|\.rs:\d+|syntax error at/i;

test.describe('C - limits', () => {
  test('C2 failed passwords count but do not lock the owner out', async ({ account }) => {
    const { api, email, proof } = await account();
    await api.post('/api/v1/auth/logout');
    for (let i = 0; i < 5; i++) {
      const wrong = await api.post('/api/v1/auth/login', { data: { email, proof: bytes(32) } });
      expect(wrong.status()).toBe(401);
    }
    expect((await api.post('/api/v1/auth/login', { data: { email, proof } })).status()).toBe(200);
  });

  test('C3 a payload over the cap is refused, and so is an oversized batch', async ({ anon, project }) => {
    const oversized = { ct: randomBytes(MAX_PAYLOAD_LENGTH + 1).toString('base64'), iv: randomBytes(24).toString('base64') };
    const res = await anon.post('/api/v1/projects', { data: { payload: oversized, demo: false } });
    expect(res.status(), 'oversized project payload').toBe(400);

    const p = await project();
    const batch = Array.from({ length: MAX_EXPENSE_BATCH + 1 }, () => ({ projectId: p.id, payload: pair(), payers: [], debtors: [] }));
    expect((await anon.post('/api/v1/expenses/batch', { data: batch })).status(), 'oversized batch').toBe(400);
    const expenses = await json<unknown[]>(await anon.get(`/api/v1/projects/${p.id}/expenses`), 'expenses');
    expect(expenses, 'nothing of the refused batch was written').toHaveLength(1);
  });

  test('C3 a nonce that is not 24 bytes is refused', async ({ anon }) => {
    const res = await anon.post('/api/v1/projects', {
      data: { payload: { ct: randomBytes(32).toString('base64'), iv: randomBytes(12).toString('base64') }, demo: false },
    });
    expect(res.status()).toBe(400);
  });
});

test.describe('D - malformed input', () => {
  test('D1 malformed bodies are refused with no internal detail', async ({ anon, project }) => {
    const p = await project();
    const cases: [string, () => Promise<{ status: () => number; text: () => Promise<string> }>][] = [
      ['broken JSON', () => anon.post('/api/v1/projects', { headers: { 'content-type': 'application/json' }, data: '{"payload":' })],
      ['wrong types', () => anon.post('/api/v1/expenses', { data: { projectId: 42, payload: 'x', payers: {}, debtors: null } })],
      ['not base64', () => anon.post('/api/v1/expenses', { data: { projectId: p.id, payload: { ct: '%%%', iv: '***' } } })],
      ['login without a proof', () => anon.post('/api/v1/auth/login', { data: { email: 'x@example.test' } })],
      ['short proof', () => anon.post('/api/v1/auth/login', { data: { email: 'x@example.test', proof: bytes(31) } })],
    ];
    for (const [name, send] of cases) {
      const res = await send();
      const body = await res.text();
      expect.soft(isRefused(res.status()), `${name}: ${res.status()}`).toBe(true);
      expect.soft(body, `${name}: body`).not.toMatch(internalDetail);
    }
  });

  test('D1 unknown fields have no effect', async ({ anon }) => {
    const res = await anon.post('/api/v1/projects', {
      data: { payload: pair(256), demo: false, readOnly: true, ownerAccountId: '00000000-0000-0000-0000-000000000001', status: 'closed' },
    });
    const dto = await json<{ readOnly: boolean; status: string }>(res, 'create with extra fields');
    expect(dto.readOnly).toBe(false);
    expect(dto.status).toBe('ongoing');
  });
});
