import { createHash, randomBytes, randomUUID } from 'node:crypto';
import { test as base, expect, request, type APIRequestContext, type APIResponse } from '@playwright/test';

export { expect };

export const BASE_URL = process.env.BASE_URL ?? 'http://127.0.0.1:8080';
export const MAILPIT_URL = process.env.MAILPIT_URL ?? 'http://127.0.0.1:8025';

export type Pair = { ct: string; iv: string };

/**
 * Ciphertext the server cannot tell from a client's: it never decrypts, so random bytes with a
 * 24-byte nonce are all it ever checks.
 */
export function pair(ctLength = 48): Pair {
  return { ct: randomBytes(ctLength).toString('base64'), iv: randomBytes(24).toString('base64') };
}

export function bytes(n: number): number[] {
  return [...randomBytes(n)];
}

export function claimToken(key: Buffer): number[] {
  return [...createHash('sha256').update('counted-claim-v1').update(key).digest()];
}

/** `X-Project-Token`: every project route requires it once the server's floor reaches `KEY_LIFECYCLE_FROM`. */
export function tokenHeaders(key: Buffer): Record<string, string> {
  return { 'x-project-token': Buffer.from(claimToken(key)).toString('base64url') };
}

function claimVerifier(key: Buffer): number[] {
  return [...createHash('sha256').update(Buffer.from(claimToken(key))).digest()];
}

export type Account = {
  api: APIRequestContext;
  id: string;
  email: string;
  proof: number[];
  cookie: string;
};

/** `withKey` is the request options of a key holder; `keyLifecycle` says whether the server enforces it. */
export type Project = {
  id: string;
  key: Buffer;
  userIds: number[];
  expenseId: number;
  keyLifecycle: boolean;
  withKey: { headers: Record<string, string> };
};

export const test = base.extend<{
  anon: APIRequestContext;
  account: () => Promise<Account>;
  project: (api?: APIRequestContext) => Promise<Project>;
}>({
  anon: async ({}, use) => {
    const api = await request.newContext({ baseURL: BASE_URL });
    await use(api);
    await api.dispose();
  },
  account: async ({}, use) => {
    const opened: APIRequestContext[] = [];
    await use(async () => {
      const account = await signUp();
      opened.push(account.api);
      return account;
    });
    for (const api of opened) await api.dispose();
  },
  project: async ({ anon }, use) => {
    await use((api = anon) => createProject(api));
  },
});

export function sessionCookie(res: APIResponse): string | undefined {
  return res
    .headersArray()
    .filter((h) => h.name.toLowerCase() === 'set-cookie')
    .map((h) => h.value.split(';')[0])
    .find((v) => v.startsWith('session_id='));
}

export async function withCookie(cookie: string): Promise<APIRequestContext> {
  return request.newContext({ baseURL: BASE_URL, extraHTTPHeaders: { cookie } });
}

export function uniqueEmail(): string {
  return `security-${randomUUID()}@example.test`;
}

/** Registers without verifying: the account exists and cannot sign in yet. */
export async function register(api: APIRequestContext, email: string, proof: number[]): Promise<void> {
  const res = await api.post('/api/v1/auth/register', {
    data: { email, proof, loginSalt: bytes(16), displayName: pair(), kdfSalt: bytes(16) },
  });
  expect(res.ok(), `register: ${res.status()} ${await res.text()}`).toBe(true);
}

/** The token of the newest verification mail to `email`, from Mailpit's API. */
export async function verificationToken(api: APIRequestContext, email: string): Promise<string> {
  let token: string | undefined;
  await expect
    .poll(
      async () => {
        const list = await api.get(`${MAILPIT_URL}/api/v1/search?query=${encodeURIComponent(`to:${email}`)}`);
        if (!list.ok()) return undefined;
        const { messages } = (await list.json()) as { messages?: { ID: string }[] };
        if (!messages?.length) return undefined;
        const message = await api.get(`${MAILPIT_URL}/api/v1/message/${messages[0].ID}`);
        if (!message.ok()) return undefined;
        const { Text, HTML } = (await message.json()) as { Text?: string; HTML?: string };
        token = /\/verify-email\/([A-Za-z0-9_-]+)/.exec(`${Text ?? ''}\n${HTML ?? ''}`)?.[1];
        return token;
      },
      { message: `no verification mail reached ${email}`, timeout: 20_000 },
    )
    .toBeTruthy();
  return token!;
}

export async function signUp(): Promise<Account> {
  const anon = await request.newContext({ baseURL: BASE_URL });
  try {
    const email = uniqueEmail();
    const proof = bytes(32);
    await register(anon, email, proof);
    const token = await verificationToken(anon, email);
    const verified = await anon.post('/api/v1/auth/verify-email', { data: { token, proof } });
    expect(verified.status(), await verified.text()).toBe(200);
    const cookie = sessionCookie(verified);
    expect(cookie, 'verify-email with a proof mints a session').toBeTruthy();
    const { id } = (await verified.json()) as { id: string };
    return { api: await withCookie(cookie!), id, email, proof, cookie: cookie! };
  } finally {
    await anon.dispose();
  }
}

export async function json<T>(res: APIResponse, what: string): Promise<T> {
  expect(res.ok(), `${what}: ${res.status()} ${await res.text()}`).toBe(true);
  const text = await res.text();
  return (text ? JSON.parse(text) : undefined) as T;
}

/** A project with two participants and one expense, written as a key holder without an account. */
export async function createProject(api: APIRequestContext): Promise<Project> {
  const key = randomBytes(32);
  const project = await json<{ id: string; keyLifecycle?: boolean }>(
    await api.post('/api/v1/projects', { data: { payload: pair(256), claimVerifier: claimVerifier(key), demo: false } }),
    'create project',
  );
  const withKey = { headers: tokenHeaders(key) };
  const users = await json<{ id: number }[]>(
    await api.post('/api/v1/users', {
      ...withKey,
      data: [
        { payload: pair(), projectId: project.id },
        { payload: pair(), projectId: project.id },
      ],
    }),
    'add participants',
  );
  const expense = await json<{ expense: { id: number } }>(
    await api.post('/api/v1/expenses', {
      ...withKey,
      data: {
        projectId: project.id,
        payload: pair(256),
        payers: [{ userId: users[0].id, payload: pair() }],
        debtors: [{ userId: users[1].id, payload: pair() }],
      },
    }),
    'add expense',
  );
  return {
    id: project.id,
    key,
    userIds: users.map((u) => u.id),
    expenseId: expense.expense.id,
    keyLifecycle: project.keyLifecycle === true,
    withKey,
  };
}

export function isRefused(status: number): boolean {
  return status >= 400 && status < 500;
}
