import { readdirSync, readFileSync } from 'node:fs';
import { join } from 'node:path';
import { expect, test } from '@playwright/test';

/**
 * E1: every route of the public `*_controller.rs` files is listed here, with the invariants that
 * test it, or why it needs none, or what is still owed. A new route fails this test until someone
 * decides which.
 */
type Coverage = { tested: string[] } | { publicByDesign: string } | { pending: string };

const coverage: Record<string, Coverage> = {
  'GET /api/v1/account/projects': { tested: ['A1', 'A6'] },
  'POST /api/v1/account/projects': { tested: ['A1', 'B3'] },
  'POST /api/v1/account/projects/batch': { pending: 'cross-account claim through the batch' },

  'POST /api/v1/auth/register': { tested: ['A5'] },
  'POST /api/v1/auth/login-salt': { publicByDesign: 'answers an unknown address like a known one (auth_controller.rs)' },
  'POST /api/v1/auth/login': { tested: ['A5', 'A6', 'C2', 'D1'] },
  'POST /api/v1/auth/verify-email/salt': { publicByDesign: 'the link holder owns the mailbox' },
  'POST /api/v1/auth/verify-email': { tested: ['A5'] },
  'POST /api/v1/auth/resend-verification': { pending: 'C1 rate limit, needs a stack behind prod nginx' },
  'POST /api/v1/auth/logout': { tested: ['A1'] },
  'DELETE /api/v1/auth/account': { tested: ['A6'] },
  'PUT /api/v1/auth/preferences': { tested: ['A1'] },
  'PUT /api/v1/auth/payment-methods': { tested: ['A1'] },
  'PUT /api/v1/auth/keypair': { tested: ['A1'] },
  'PUT /api/v1/auth/keypair/wrap': { pending: 'A3: refused unless the public key is the stored one' },
  'GET /api/v1/auth/me': { tested: ['A1', 'A6'] },

  'POST /api/v1/expenses': { tested: ['B6', 'D1'] },
  'POST /api/v1/expenses/batch': { tested: ['C3'] },
  'PUT /api/v1/expenses': { tested: ['B2'] },
  'GET /api/v1/projects/{project_id}/expenses': { tested: ['C3'] },
  'GET /api/v1/projects/{project_id}/expenses/{expense_id}': { tested: ['B2', 'B5'] },
  'POST /api/v1/expenses/delete': { tested: ['B2'] },

  'POST /api/v1/friends/requests': { tested: ['A1', 'A2'] },
  'POST /api/v1/friends/requests/from-project': { pending: 'A2 through a project participant' },
  'GET /api/v1/friends': { tested: ['A1', 'A2'] },
  'POST /api/v1/friends/requests/{id}/accept': { tested: ['A1', 'A2'] },
  'DELETE /api/v1/friends/requests/{id}': { tested: ['A1', 'A2'] },
  'DELETE /api/v1/friends/{id}': { tested: ['A1'] },

  'POST /api/v1/projects/{project_id}/invitations': { pending: 'A2: only to a friend' },
  'POST /api/v1/projects/{project_id}/invitations/batch': { pending: 'A2: only to a friend' },
  'GET /api/v1/projects/invitations': { tested: ['A1'] },
  'GET /api/v1/projects/{project_id}/invitations/sent': { tested: ['A1'] },
  'DELETE /api/v1/projects/invitations/{id}': { tested: ['A1'] },

  'GET /api/v1/fx/rates': { publicByDesign: 'public exchange rates, same answer for everyone' },
  'GET /api/v1/projects/{project_id}/history': { tested: ['B5'] },

  'GET /api/v1/projects/{project_id}/expenses/{expense_id}/payments': { tested: ['B2'] },
  'GET /api/v1/projects/{project_id}/payments': { tested: ['B2', 'B5'] },
  'GET /api/v1/projects/{project_id}/sync/{known_version}': { tested: ['B5'] },
  'POST /api/v1/projects/sync': { pending: 'B5 on the batched sync' },

  'GET /api/v1/projects/{project_id}': { tested: ['B5'] },
  'POST /api/v1/projects/batch': { pending: 'B5 on the batched read' },
  'POST /api/v1/projects': { tested: ['C3', 'D1'] },
  'PUT /api/v1/projects': { pending: 'B4: refused on a read-only or demo project' },
  'POST /api/v1/projects/convert': { tested: ['B3'] },
  'POST /api/v1/projects/members': { publicByDesign: 'anonymous membership, idempotent (projects_controller.rs)' },
  'POST /api/v1/projects/leave': { publicByDesign: 'anonymous leave; teardown is the sweep, never inline (projects_controller.rs)' },

  'PUT /api/v1/push/token': { pending: 'A4: a token registers for the caller only' },
  'POST /api/v1/push/unregister': { pending: 'A4' },
  'POST /api/v1/push/verify': { pending: 'A4' },

  'POST /api/v1/recurring': { pending: 'B2 for recurring rules' },
  'PUT /api/v1/recurring': { pending: 'B2 for recurring rules' },
  'POST /api/v1/recurring/delete': { pending: 'B2 for recurring rules' },
  'POST /api/v1/recurring/materialize': { pending: 'B2 for recurring rules' },

  'POST /api/v1/fetch/tricount': { pending: 'D2: only the Tricount host is ever called' },

  'DELETE /api/v1/projects/{project_id}/users/{user_id}': { tested: ['B2'] },
  'PUT /api/v1/users': { tested: ['B2'] },
  'PUT /api/v1/users/batch': { tested: ['B2'] },
  'POST /api/v1/users': { tested: ['B2'] },
  'GET /api/v1/projects/{project_id}/users': { tested: ['B2', 'B5'] },
};

function controllerFiles(dir: string): string[] {
  return readdirSync(dir, { withFileTypes: true }).flatMap((entry) => {
    const path = join(dir, entry.name);
    if (entry.isDirectory()) return entry.name === 'server' ? [] : controllerFiles(path);
    return entry.name.endsWith('_controller.rs') ? [path] : [];
  });
}

function publicRoutes(): string[] {
  const routes = controllerFiles(join(__dirname, '..', '..', 'packages', 'api', 'src')).flatMap((file) =>
    [...readFileSync(file, 'utf8').matchAll(/#\[(get|post|put|delete|patch)\("([^"]+)"\)\]/g)].map(
      ([, method, path]) => `${method.toUpperCase()} ${path}`,
    ),
  );
  return routes.sort();
}

test('E1 every public route is tested, public by design, or listed as owed', () => {
  const routes = publicRoutes();
  expect(routes.length).toBeGreaterThan(0);
  expect(routes, 'routes in the controllers vs routes listed here').toEqual(Object.keys(coverage).sort());

  const pending = Object.entries(coverage).filter(([, c]) => 'pending' in c);
  test.info().annotations.push({
    type: 'coverage',
    description: `${routes.length - pending.length} of ${routes.length} routes tested or public by design; ${pending.length} owed: ${pending.map(([r]) => r).join(', ')}`,
  });
});
