# Security tests

Black-box tests of the boundaries the Counted server must enforce. They talk to the API over HTTP
only, with synthetic accounts and random ciphertext. The server never decrypts anything, so it
cannot tell that ciphertext from a real client's. There is no test-only code path in the server:
accounts are verified through the mail the server sends, read from a local Mailpit.

The server code is not public. These tests are how you can see what it is held to.

## What a green run proves

That the invariants below held, on the release they ran against, for the requests the tests make.
It does not prove that nothing else is broken.

| # | Invariant |
| --- | --- |
| A1 | Every account endpoint refuses a request with no session, a made-up session or one ended by logout, and serves the same request with a session |
| A2 | A third account can neither withdraw nor accept a friend request between two others |
| A5 | An unverified account gets no session: not from login, not with a wrong password, and the verification link alone is refused (400); the link with the password verifies and signs in |
| A6 | A deleted account's session opens nothing, and its credentials no longer sign in |
| A7 | Deleting an account needs the password's proof: a wrong proof is refused and the account stays; once the server's floor is `KEY_LIFECYCLE_FROM`, the session-only route is refused too |
| B1 | No endpoint lists projects |
| B2 | Holding project A's id gives nothing in project B: B's expenses, participants and payment rows cannot be read, edited or deleted through A |
| B3 | Claiming a participant, or converting a project, needs proof of the project key |
| B5 | What any id holder reads is ciphertext plus the fields documented in SECURITY.md |
| B6 | By design: an id holder without the key can write rows (SECURITY.md 5.6) |
| C2 | Failed passwords count, but do not lock the owner out |
| C3 | Payloads over 64 KiB, nonces that are not 24 bytes, and batches over 500 are refused, and nothing of a refused batch is written |
| D1 | Malformed bodies are refused with a 4xx and no internal detail; unknown fields have no effect |
| E1 | Every route in `packages/api/src/**/*_controller.rs` is tested, public by design, or listed as still owed |

The project id is an access token, so "account A reads B's project" is allowed by design: whoever
holds a share link can read that project's ciphertext. See SECURITY.md 5.6.

Still owed, listed route by route in `tests/routes.spec.ts`: rate limits (they live in nginx, which
the test stack does not run), push tokens, invitations, recurring rules, read-only and demo projects,
and the Tricount proxy.

## Running

Against a local stack with Mailpit, never against production:

```sh
npm ci
eval "$(make -s -C .. e2e-env)"   # BASE_URL and MAILPIT_URL of this worktree's stack
npx playwright test
```

`results/security-tests.json` holds the outcome of each test.
