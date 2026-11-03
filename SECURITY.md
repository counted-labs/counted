# Security policy

Counted is an expense-sharing app built so that our server never sees what you write: names,
amounts, dates and notes are encrypted on your device. This page explains how to report a problem,
what you may test, and what we can and cannot prove about our own security.

This repository holds everything that runs on your device. The server is closed source. Every
statement in section 5 carries a label saying what backs it.

## 1. Reporting a vulnerability

Email **contact@counted.fr** with "security" in the subject. Please include what you did, what you
saw, and what you think an attacker could gain.

To encrypt your report, use our OpenPGP key, published by our mail provider Proton:
`https://api.protonmail.ch/pks/lookup?op=get&search=contact@counted.fr`, fingerprint
`AA49 C049 8CF4 48EB 8A8F 634D A240 03C1 8555 ACC6`. Mail sent from a Proton address is end-to-end
encrypted without it.

If you prefer to stay anonymous, you can report to ANSSI, the French national cybersecurity agency,
instead (https://cyber.gouv.fr/reglementation/cybersecurite-systemes-dinformation/declaration-de-vulnerabilites/).
Under article L2321-4 of the French Defence Code, ANSSI keeps the identity of a good-faith reporter
confidential.

Please do not open a public GitHub issue for a vulnerability. Issues on this repository are not
monitored anyway: it is a one-commit-per-release mirror.

What you can expect from us. Counted is run by one person, so these are the times we can actually
keep:

- an acknowledgement within **7 days**;
- a first assessment (do we reproduce it, how serious it is) within **14 days**;
- a fix, or a public status, within **90 days**. We will agree a disclosure date with you, and you
  may publish after it.

There is **no bug bounty**: we have no budget for one. With your permission, we
will credit you in the release notes and in a list of acknowledgements.

## 2. Scope

In scope:

- the app and API at `counted.fr` (`/projects`, `/api/v1/...`), the Android and iOS apps;
- the client code in this repository, including its cryptography;
- the release verification chain (`counted.fr/verify`, `SHA256SUMS.txt` and its signature).

Rules:

- use **only accounts and projects you created yourself**. If you reach someone else's data, stop,
  do not keep a copy, and tell us;
- no denial of service, no load testing, no automated scanning that exceeds the rate limits;
- no social engineering, no phishing, no physical attacks;
- third-party services are out of scope: Tricount, Apple, Google, the push services, our hosting
  and mail providers. Report to them directly.

There is no test environment: production is the only target, so please be gentle with it.

## 3. Safe harbour

If you act in good faith, follow section 2, and give us reasonable time before disclosing:

- we consider your research authorised by us, and we will not take legal action or file a complaint
  against you for it, including for an accidental breach of section 2 that you report to us;
- we will not share your identity with anyone without your written permission;
- if someone else takes action against you for research that followed this policy, we will tell
  them, and the authorities, that it did.

This promise binds us only: it cannot bind third parties or public prosecutors. Reporting to ANSSI
(section 1) is the protection French law provides. If you are unsure whether a test is covered,
ask us before running it.

## 4. Verifying what we ship

The web app is built from this repository, and the build is reproducible byte for byte.
`counted.fr/verify` lists the hashes and the commands to rebuild and compare them. The list of
hashes is signed through Sigstore. `.github/workflows/rebuild.yml` rebuilds the client from a
release tag on GitHub's machines and compares it with that list.

`.github/workflows/monitor.yml` checks every 6 hours that counted.fr serves a signed release's files
and nothing else.

The Android and iOS apps are built from the same sources. F-Droid rebuilds the Android app from this
repository and ships our signed APK only if its build matches it; the certificate is in the
README. Google Play re-signs what it distributes and the App Store encrypts each copy per device, so
those copies cannot be compared with a build.

## 5. Security model

This section is the summary. The full construction, with test vectors and every known limitation,
is in [SPEC.md](SPEC.md).

Each statement has one label:

- **[Public code]**: visible in this repository; what counted.fr serves can be rebuilt from it.
- **[Tested (private)]**: an automated test runs before each release; the test is not public yet.
- **[Commitment]**: an operational promise. Nobody outside can check it.
- **[Not independently verified]**: true to our knowledge, in server code nobody outside has seen.

### 5.1 What is encrypted, and what our server sees

- Project names, participant names, expense names, amounts, dates, categories, notes, who paid and
  who owes, your display name, preferences and payment details are encrypted on your device with
  XChaCha20-Poly1305, with a fresh random 24-byte nonce for each encryption. **[Public code]**
- The server sees, for accounts: your email, a hash of your login proof, your key-derivation salt,
  which projects your account belongs to and which participant you are in each, your friends
  (between accounts), keyed hashes of addresses you sent friend requests to, which friend you
  invited to which project, public keys, and, if you turn notifications on, your phone's push
  address and language. **[Not independently verified]**
- For changing keys: how many times your account key and each project's key changed, and when;
  whether you set a recovery key and when you last changed your password; who administers each
  project; for each change of a project's key, who made it, who was removed and who received the
  new key. The full list is [SPEC.md](SPEC.md) section 12.2. **[Not independently verified]**
- For everyone: IP address, user agent and time in technical logs; the size and timing of
  encrypted writes; how many participants and expenses a project has; which projects one device
  syncs together. **[Not independently verified]**
- Each project's history records, in clear, whether each change created, edited or deleted a row,
  of which kind (expense, participant...) and with which id, and when. Encrypting this is planned.
  **[Not independently verified]**
- In projects saved by an older app version and not reopened since, the server still sees which
  participants are in each expense, who entered it, who made each change, and whether the project is
  closed. Opening the project with a current version removes these links. A few old participant
  rows also still carry an unsalted hash of an email address. Removing all of these columns is
  planned. **[Not independently verified]**
- Daily totals (sign-ups, interface languages). On a day with a single sign-up, they describe that
  one account. **[Not independently verified]**
- A Tricount import passes through our server **in clear**: the server makes the request on behalf
  of your device, and your device then encrypts the result. It is neither stored nor logged.
  **[Commitment]**

### 5.2 Keys: creation, storage, recovery

- A project key is 32 random bytes, generated on the device that creates the project. It travels in
  the `#fragment` of the share link, which browsers never send to the server. **[Public code]**
- Your account key is protected by a key derived on your device from your password with Argon2id.
  At sign-in your device sends a login proof derived separately, from which the password cannot be
  recovered. The server stores a hash of that proof. **[Public code]** for the device half; **[Not
  independently verified]** for the storage.
- You can change your password. "Someone may know my password" also replaces your account key and
  your public keys, and signs out every other device; whoever had the password keeps what they
  already opened. **[Public code]** for the device half; **[Not independently verified]** for the
  server half.
- With an account, your project keys are stored on the server encrypted under your account key
  ("key escrow"), so a new device gets them back. The trade-off: someone holding **both** our
  database and your password could read your projects. **[Public code]**
- Inviting a friend boxes the project key to their public key (X25519). An active server could
  substitute a public key. To detect that, compare the friend codes (an 80-bit fingerprint) with
  your friend outside the app. Each device also remembers a friend's key the first time it sees it
  and blocks invitations if the key changes later. **[Public code]**
- On the device, keys and the encrypted cache live in `localStorage` (web) or in the app's private
  data file (mobile). The key is stored next to the encrypted cache, so anyone who can copy that
  storage can read your projects. The mobile apps exclude this file from cloud backups, and on
  Android from device-to-device transfer too. **[Public code]**
- Signing out removes the account key but keeps project keys and the cache on the device. On a
  shared computer, use "Sign out and erase this device" in the settings instead: it removes
  everything but the interface language, after checking that your account can give every project
  back. **[Public code]**
- We cannot recover your keys: we never hold them. A recovery key, 52 characters you write down,
  gives your account and its projects back with the link we email you. Without it, a reset gives
  your account new keys after 72 hours, and your projects come back only from a device that still
  holds them or from their share links. Without an account, a lost device means the share link is
  the only way back. **[Public code]** for the device half; **[Not independently verified]** for the
  delays.
- A project's admin can change its key, alone or to remove someone. Members with an account who
  have opened the project with a current version get the new key automatically, boxed with X25519
  and ML-KEM-768 (post-quantum); the others need the new link. Whoever is cut off keeps what they
  already read. A project created without an account by an older app version, or left with no
  account member holding its key, has no admin and cannot change its key. **[Public code]** for the device half; **[Not independently verified]** for the server's
  refusal of the old key.

### 5.3 Logs, backups, deletion

- Technical logs (IP address, user agent, time) are kept for at most 30 days, then deleted.
  **[Commitment]**
- The database is dumped nightly and kept 14 days on the server. A copy encrypted with `age` is
  kept 30 days at Proton Drive (Switzerland), which does not hold the key. **[Commitment]**
- Deleting your account deletes its rows at once. Expenses you wrote into a shared project stay
  there, because they belong to the other members' ledger; they remain encrypted under the project
  key. A daily sweep deletes expired sessions, accounts never verified after 24 hours, and projects
  with no member left after 24 hours. Deleted data stays in backups until those age out.
  **[Commitment]**

### 5.4 What the server enforces

- Session cookie: `HttpOnly`, `Secure`, `SameSite=Lax`, 30 days. The server stores only a hash of
  it. **[Not independently verified]**
- Rate limits: sign-in, registration and verification-mail requests are limited to 5 per minute per
  IP address. Each password guess also costs the guesser an Argon2id computation on their side,
  because the server only checks the login proof. **[Not independently verified]**
- Every request on an expense, payment, participant, recurring-expense or history row is scoped to the project
  it names: holding one project's id gives nothing in another. **[Not independently verified]**
- Every request about a project carries a token derived from its key. Once the server no longer
  accepts app versions before 3.2.9, it refuses a request without the right token, on every project
  that has a key check, and only an admin can change a project's key. **[Not independently
  verified]**
- Size caps on encrypted payloads and batches. **[Not independently verified]**
- There are no operator endpoints: the only admin routes are a project admin's, on their own
  project. The full list of routes is in the public `*_controller.rs` files of `packages/api/src/`.
  **[Public code]**

Some of these already have private tests. A public test suite is planned: its code in this
repository, run before each release, its result attached to the GitHub release. When it ships, the
rows it covers will move to a public label.

### 5.5 If our server were compromised

An attacker in control of our server **could**:

- see everything listed in 5.1 as visible to the server;
- delete or withhold data, or replay old encrypted rows;
- **serve altered JavaScript to web users**. This is the main risk of any web app with end-to-end
  encryption. The mobile apps are not affected: their code is fixed at install. `counted.fr/verify`
  can detect a swap served to everyone, and `.github/workflows/monitor.yml` checks for one every
  6 hours. Neither can detect a swap aimed at a single person. If that worries you,
  use the mobile app;
- substitute a public key during a friend invitation before your device first sees that friend
  (detectable by comparing friend codes, 5.2; a later change is blocked on the device);
- on app versions up to 3.2.7 and web versions up to v1.24.3, make a user who accepts a forged
  invitation switch to a project key the attacker chose, with nothing that comparing codes would show
  ([SPEC.md](SPEC.md) section 9.1);
- try to guess passwords offline from the stored login-proof hashes. Each guess costs an Argon2id
  computation, and a found password would unlock that account's escrowed project keys;
- with the help of someone who held a project's old key, get a key of its own adopted by a member's
  device that has not yet moved to the new one after a key change ([SPEC.md](SPEC.md) section 8.5).

It **could not**:

- read names, amounts, dates or notes already stored, without a project key or a password;
- forge an encrypted row that a member's device accepts as valid, without the project key.

### 5.6 Known limitations

The full list, grouped by what would close each, is [SPEC.md](SPEC.md) section 14. The ones that
matter most day to day:

- **The share link is a house key.** It carries the project's key. Until the server enforces
  project tokens (5.4), and for good on a few old projects that have no key check, anyone who holds
  a project's id can also read its encrypted rows and write or delete them, even without the key.
  Without the key they cannot read anything, and anything they write fails to decrypt and is
  ignored by members' devices.
- Every member of a project holds its key and can write any row in any participant's name. Nothing
  authenticates who wrote what.
- Changing a project's key protects what is written after the change, not before: a removed member
  keeps what they read or saved. A project with no admin cannot change its key at all (5.2).
- Metadata: see 5.1. We publish the full list rather than claim there is none.

## 6. Audits

Seven internal audit passes between August and October 2026, each with its findings and fixes
written up in internal reports, which are not public. **No independent audit has been done.** We
will say so until one has. A review of the client cryptography needs nothing from us:
[REVIEW.md](REVIEW.md) says where to start, what is in scope and what we already know.
