import { randomUUID } from 'node:crypto';
import { request, type APIRequestContext, type APIResponse } from '@playwright/test';
import {
  BASE_URL,
  bytes,
  claimToken,
  expect,
  isRefused,
  json,
  pair,
  register,
  sessionCookie,
  test,
  uniqueEmail,
  verificationToken,
  withCookie,
} from '../lib/api';

type Call = { name: string; send: (api: APIRequestContext) => Promise<APIResponse> };

const someId = randomUUID();

/** ML-KEM-768 sizes (FIPS 203), as `shared::KEM_PUBLIC_KEY_LENGTH` and `KEM_CIPHERTEXT_LENGTH`. */
const KEM_PUBLIC_KEY_LENGTH = 1184;
const KEM_CIPHERTEXT_LENGTH = 1088;

/** Every endpoint scoped to the signed-in account, with a body it accepts once signed in. */
const accountScoped: Call[] = [
  { name: 'GET /account/projects', send: (api) => api.get('/api/v1/account/projects') },
  { name: 'POST /account/projects', send: (api) => api.post('/api/v1/account/projects', { data: { projectId: someId, userId: null } }) },
  { name: 'GET /friends', send: (api) => api.get('/api/v1/friends') },
  { name: 'POST /friends/requests', send: (api) => api.post('/api/v1/friends/requests', { data: { email: uniqueEmail(), label: pair(32) } }) },
  { name: 'POST /friends/requests/{id}/accept', send: (api) => api.post(`/api/v1/friends/requests/${someId}/accept`) },
  { name: 'DELETE /friends/requests/{id}', send: (api) => api.delete(`/api/v1/friends/requests/${someId}`) },
  { name: 'DELETE /friends/{id}', send: (api) => api.delete(`/api/v1/friends/${someId}`) },
  { name: 'GET /projects/invitations', send: (api) => api.get('/api/v1/projects/invitations') },
  { name: 'DELETE /projects/invitations/{id}', send: (api) => api.delete(`/api/v1/projects/invitations/${someId}`) },
  { name: 'GET /projects/{id}/invitations/sent', send: (api) => api.get(`/api/v1/projects/${someId}/invitations/sent`) },
  { name: 'PUT /auth/preferences', send: (api) => api.put('/api/v1/auth/preferences', { data: pair(64) }) },
  { name: 'PUT /auth/payment-methods', send: (api) => api.put('/api/v1/auth/payment-methods', { data: { ...pair(64), shared: [] } }) },
  { name: 'PUT /auth/keypair', send: (api) => api.put('/api/v1/auth/keypair', { data: { publicKey: bytes(32), privateKey: pair(48) } }) },
];

async function refusedEverywhere(api: APIRequestContext): Promise<void> {
  for (const call of accountScoped) {
    const res = await call.send(api);
    expect.soft(res.status(), `${call.name} without a valid session`).toBe(401);
  }
  const me = await api.get('/api/v1/auth/me');
  expect(await me.json(), 'GET /auth/me without a valid session').toBeNull();
}

test.describe('A - account boundaries', () => {
  test('A1 every account endpoint refuses a request without a session, and serves one with it', async ({ anon, account }) => {
    await refusedEverywhere(anon);

    const garbage = await withCookie(`session_id=${randomUUID()}`);
    await refusedEverywhere(garbage);
    await garbage.dispose();

    const { api } = await account();
    for (const call of accountScoped) {
      const res = await call.send(api);
      expect.soft(res.status(), `${call.name} with a session`).not.toBe(401);
    }
  });

  test('A1 a session ends at logout', async ({ account }) => {
    const { api, cookie } = await account();
    expect((await api.post('/api/v1/auth/logout')).ok()).toBe(true);

    const stale = await withCookie(cookie);
    await refusedEverywhere(stale);
    await stale.dispose();
  });

  test('A2 a third account can neither withdraw nor accept a friend request between two others', async ({ account }) => {
    const a = await account();
    const b = await account();
    const c = await account();

    await json(await a.api.post('/api/v1/friends/requests', { data: { email: b.email, label: pair(32) } }), 'A asks B');
    const outgoing = async () =>
      (await json<{ outgoing: { id: string }[] }>(await a.api.get('/api/v1/friends'), 'A friends')).outgoing;
    const [requestOfA] = await outgoing();
    expect(requestOfA, 'A sees its request').toBeTruthy();

    expect(isRefused((await c.api.delete(`/api/v1/friends/requests/${requestOfA.id}`)).status())).toBe(true);
    expect(isRefused((await c.api.post(`/api/v1/friends/requests/${requestOfA.id}/accept`)).status())).toBe(true);
    expect((await outgoing()).map((r) => r.id), 'the request survives C').toContain(requestOfA.id);

    const friendsOfC = await json<{ friends: unknown[] }>(await c.api.get('/api/v1/friends'), 'C friends');
    expect(friendsOfC.friends).toHaveLength(0);
  });

  test('A5 an unverified account gets no session, from login or from a link without the password', async ({ anon }) => {
    const email = uniqueEmail();
    const proof = bytes(32);
    await register(anon, email, proof);

    const login = await anon.post('/api/v1/auth/login', { data: { email, proof } });
    expect(login.status(), 'login before verification').toBe(403);
    expect(sessionCookie(login)).toBeUndefined();

    const token = await verificationToken(anon, email);
    const wrongPassword = await anon.post('/api/v1/auth/verify-email', { data: { token, proof: bytes(32) } });
    expect(wrongPassword.status()).toBe(401);
    expect(sessionCookie(wrongPassword)).toBeUndefined();

    const linkOnly = await anon.post('/api/v1/auth/verify-email', { data: { token } });
    expect(linkOnly.status(), 'the link alone verifies nothing').toBe(400);
    expect(sessionCookie(linkOnly), 'the link alone mints no session').toBeUndefined();

    const verified = await anon.post('/api/v1/auth/verify-email', { data: { token, proof } });
    expect(verified.ok(), 'the link and the password verify').toBe(true);
    expect(sessionCookie(verified)).toBeTruthy();

    const afterwards = await anon.post('/api/v1/auth/login', { data: { email, proof } });
    expect(afterwards.status()).toBe(200);
    expect(sessionCookie(afterwards)).toBeTruthy();
  });

  test('A6 a deleted account cookie opens nothing', async ({ account }) => {
    const { api, cookie, email, proof } = await account();
    expect((await api.post('/api/v1/auth/account/delete', { data: { proof } })).ok()).toBe(true);

    const stale = await withCookie(cookie);
    await refusedEverywhere(stale);
    await stale.dispose();

    const anon = await request.newContext({ baseURL: BASE_URL });
    const login = await anon.post('/api/v1/auth/login', { data: { email, proof } });
    expect(login.status()).toBe(401);
    await anon.dispose();
  });

  test('A7 deleting an account needs the password, and a session alone is refused once the floor is reached', async ({ account }) => {
    const { api, email, proof } = await account();
    const wrong = await api.post('/api/v1/auth/account/delete', { data: { proof: bytes(32) } });
    expect(wrong.status(), 'a wrong proof is refused like a wrong password').toBe(401);
    expect((await api.get('/api/v1/auth/me')).ok(), 'and the account is still there').toBe(true);

    const me = await json<{ keyLifecycle: boolean }>(await api.get('/api/v1/auth/me'), 'me');
    const sessionOnly = await api.delete('/api/v1/auth/account');
    if (me.keyLifecycle) {
      expect(sessionOnly.status(), 'from the floor, the session-only route is refused').toBe(400);
      expect((await api.get('/api/v1/auth/me')).ok()).toBe(true);
      expect((await api.post('/api/v1/auth/account/delete', { data: { proof } })).ok()).toBe(true);
    } else {
      expect(sessionOnly.ok(), 'below the floor, older clients still delete with the session').toBe(true);
    }
    const anon = await request.newContext({ baseURL: BASE_URL });
    expect((await anon.post('/api/v1/auth/login', { data: { email, proof } })).status()).toBe(401);
    await anon.dispose();
  });

  test('A8 an invitation carries exactly one key box, the one the floor allows', async ({ account, project }) => {
    const a = await account();
    const b = await account();
    for (const { api } of [a, b]) {
      await json(await api.put('/api/v1/auth/keypair', { data: { publicKey: bytes(32), privateKey: pair(48) } }), 'keypair');
    }
    await json(await b.api.put('/api/v1/auth/kem-key', { data: { publicKey: bytes(KEM_PUBLIC_KEY_LENGTH), privateKey: pair(96) } }), 'KEM key');

    await json(await a.api.post('/api/v1/friends/requests', { data: { email: b.email, label: pair(32) } }), 'A asks B');
    const [incoming] = (await json<{ incoming: { id: string }[] }>(await b.api.get('/api/v1/friends'), 'B friends')).incoming;
    await json(await b.api.post(`/api/v1/friends/requests/${incoming.id}/accept`), 'B accepts');

    const p = await project(a.api);
    await json(
      await a.api.post('/api/v1/account/projects', { ...p.withKey, data: { projectId: p.id, userId: null, claimToken: claimToken(p.key) } }),
      'A holds the project',
    );

    const { hybridInvitations } = await json<{ hybridInvitations: boolean }>(await a.api.get('/api/v1/auth/me'), 'me');
    const classical = { key: pair(48) };
    const hybrid = { hybrid: { ephemeral: bytes(32), kemCt: bytes(KEM_CIPHERTEXT_LENGTH), nonce: bytes(24), ct: bytes(80) } };
    const [allowed, other] = hybridInvitations ? [hybrid, classical] : [classical, hybrid];
    const invite = (box: object) =>
      a.api.post(`/api/v1/projects/${p.id}/invitations`, { ...p.withKey, data: { toAccountId: b.id, ...box } });
    const pending = async () => (await json<unknown[]>(await b.api.get('/api/v1/projects/invitations'), 'B invitations')).length;

    expect((await invite({ ...classical, ...hybrid })).status(), 'both boxes').toBe(400);
    expect((await invite({})).status(), 'no box').toBe(400);
    expect((await invite(other)).status(), hybridInvitations ? 'a classical box from the floor' : 'a hybrid box before the floor').toBe(400);
    const batch = await a.api.post(`/api/v1/projects/${p.id}/invitations/batch`, {
      ...p.withKey,
      data: [{ toAccountId: b.id, ...classical, ...hybrid }],
    });
    expect(await json<string[]>(batch, 'batch with both boxes'), 'the batch refuses it too').toEqual([b.id]);
    expect(await pending(), 'nothing refused reached B').toBe(0);

    expect((await invite(allowed)).ok(), 'the box the floor allows').toBe(true);
    const [received] = await json<{ key?: unknown; hybrid?: unknown }[]>(await b.api.get('/api/v1/projects/invitations'), 'B invitations');
    expect([received.key !== undefined, received.hybrid !== undefined], 'B receives that box alone').toEqual([!hybridInvitations, hybridInvitations]);
  });
});
