# Counted - cryptographic specification

This document describes how the Counted client encrypts, distributes and stores data, what the
server can and cannot learn, and what is planned next. It is written for cryptographers and
security researchers who want to review the design against the code in this repository.

## 1. Scope, status and evidence

It covers the client: the web app (Rust compiled to WebAssembly, served at counted.fr) and the
Android and iOS apps (the same Rust crate in a WebView). Both run the same code from
[`packages/ui`](packages/ui) and [`packages/shared`](packages/shared).

- **Format v1** (sections 3 to 13) is what runs in production.
- **Format v2** (section 15) is a draft. Nothing in it is built.
- **Key lifecycle**: the account master key, password change and recovery (7.5), project tokens
  (8.1), roles (8.4) and project key rotation (8.5, 8.6). Built from the client version named by
  `KEY_LIFECYCLE_FROM` in [`shared/src/lib.rs`](packages/shared/src/lib.rs) (mobile 3.2.9). The
  server turns it on only once it refuses older clients: until then it refuses every request that
  would move an account off master-key epoch 0 or start a rotation, and it does not require project
  tokens **[Not independently verified]**.

The server is not in this repository. Statements are therefore of three kinds:

- **Client code**: implemented in this repository, cited by path. What counted.fr serves can be
  rebuilt from it byte for byte ([README.md](README.md)).
- **Server behaviour**, marked **[Not independently verified]**: what the server is designed to do.
  Nobody outside can check it, and a hostile server would not do it.
- **Commitment**: an operational promise, listed in [SECURITY.md](SECURITY.md) section 5.

## 2. Threat model and trust

**Who is trusted with what.**

| Party | Holds | Can |
| --- | --- | --- |
| A project member (anyone holding the project key, signed in or not) | The key, from the share link, an invitation or escrow | Read, alter, add or delete every row of the project, under any participant's name. Nothing authenticates who wrote a row, an account holder's included: the author and actor fields are inside the payload and self-asserted |
| A project admin (an account member holding the role, section 8.4) | The key, and a role the server stores in clear | Change the key and remove members (8.5), grant and take the role. The server enforces the role **[Not independently verified]**; it gives no cryptographic power beyond the key, and no write to another member's claim label or payment-method copy (8.5) **[Not independently verified]**. One exception: during a rotation the admin can rewrite or set aside any history entry, since history is otherwise append-only, a rotation must re-encrypt it, and nothing tells a faithful reseal from a forged one until entries are signed |
| A former member | The key they held, and whatever they read or cached | Everything above, until an admin changes the key (8.5). After the switch: against an honest server, nothing (the old key's token is refused); against a hostile server alone, nothing written under the new key; **colluding with a hostile server**, get a key of their own adopted by a member device that has not switched yet, or boxed to by the admin (8.5, 8.6). A project with no admin cannot change its key (8.4): there a former member keeps everything above for as long as the project exists |
| Anyone holding a project id (a UUID), without the key | The id, which is in the share link | On a project with a verifier, once the server enforces project tokens (8.1): nothing, every request is refused. Otherwise: read the project's ciphertext, and write or delete its rows. Rows written without the key fail the AEAD check and are dropped by members' clients (section 5.5) |
| The server, passive | Every ciphertext and all metadata | Learn the metadata in section 12. Nothing else |
| The server, active | The above, and control over what it serves | Everything a UUID holder can do on every project. Also: serve old or missing rows, swap rows within a project (section 5.1), substitute public keys (section 9), make an account holder who accepts a forged invitation switch to a project key the server chose, in the cases left in section 9.1, withhold typed wraps from a device that has never opened one (section 7.3), withhold rotation boxes or serve a fresh device an escrow row from before a rotation (8.5), and serve altered JavaScript to web users |
| The publisher and the app stores | The release pipeline | Ship a client that exfiltrates keys. Detectable for the web bundle (if served to everyone) and for F-Droid builds, which are reproducible; not for App Store or Google Play installs, which cannot be compared with a build ([README.md](README.md)) |
| The network | TLS-protected traffic | Nothing beyond what TLS leaks (sizes, timing) |

**Goals.** A server that does not hold a project key cannot read the project's names, amounts,
dates or descriptions, nor who owes whom. One exception: in projects last saved by a client older
than mobile 3.2.5 or web v1.24.1 (2026-10-18) and not reopened since, the server still sees which participants are in each expense and
who entered it ([SECURITY.md](SECURITY.md) section 5.1). A server that does not hold an account's
password cannot read that account's display name, preferences, payment details or escrowed keys,
but it can guess the password offline at one Argon2id per guess (section 4), and passwords need only
be 8 bytes long. A server cannot forge a row that a member accepts, **unless it holds the project
key**. Section 9.1 lists the cases in which an active server can still make an account holder's
device switch to a key the server chose, if its user accepts a forged invitation: on clients up to
mobile 3.2.7 and web v1.24.3; on current clients only with a friend's public key substituted before
the device first saw that friend (section 9.2), which comparing codes detects, and only for a project the device holds no key for, or holds a key it has
never seen open and its user then confirms the switch (section 6). From then on the server reads that device's new writes to the project and can forge
rows that device accepts. It still cannot read the rows written under the genuine key. A key
rotation (8.5) cuts a former member off from the rows written after it, unless they collude with a
hostile server; it never takes back what they read.

**Not goals.** Integrity against members. Detecting withheld or rolled-back rows (section 5.1).
Protecting a device that is compromised while unlocked. Protecting web users against a server
that serves altered JavaScript to them alone: the reproducible build and the public monitor
([README.md](README.md)) detect a swap served to everyone, not one aimed at a single visitor.
Resistance to a quantum adversary, beyond what section 3.1 lists.

## 3. Primitives and randomness

| Use | Primitive | Implementation |
| --- | --- | --- |
| Symmetric encryption | XChaCha20-Poly1305 (the IETF AEAD construction with a 24-byte nonce) | `chacha20poly1305` 0.10.1 |
| Friend invitations, until `MIN_APP_VERSION` reaches 3.2.10 | X25519, then HChaCha20 to a key, then NaCl `crypto_secretbox` with XChaCha20 and Poly1305 (section 9) | `crypto_box` 0.9.1 (`ChaChaBox`), `crypto_secretbox` 0.1.1, `curve25519-dalek` 4.1.3 |
| Friend invitations, from 3.2.10 (section 9.1) | The rotation box below with the sender's static X25519 share as a third combiner input | the same as rotation boxes |
| Rotation boxes (section 8.6) | Ephemeral X25519 and ML-KEM-768 (FIPS 203) to the recipient's two public keys, HKDF-SHA256 over both shared secrets, then the XChaCha20-Poly1305 above | `curve25519-dalek` 4.1.3, `ml-kem` 0.3.4, `hkdf` 0.12.4 |
| Password hashing | Argon2id, version 0x13 | `argon2` 0.5.3 |
| Hashing | SHA-256 | `sha2` 0.10.9 |
| Recovery-key derivation (section 7.5) | HKDF-SHA256 | `hkdf` 0.12.4 |
| Randomness | OS CSPRNG through `getrandom` 0.2 with the `js` feature: `crypto.getRandomValues` on wasm, the OS generator on Android and iOS | [`crypto.rs`](packages/ui/src/crypto.rs) `random_bytes`, which panics if no CSPRNG is available; the ML-KEM seed and encapsulation randomness come from the same `getrandom` ([`hybrid.rs`](packages/ui/src/hybrid.rs)) |
| Client ids, the recurring seed (section 10), the anonymous member id | UUID v4 (122 random bits) | `uuid` 1.24.1: on wasm, `crypto.getRandomValues` through uuid's own WebCrypto binding (its `js` feature, enabled in [`packages/shared/Cargo.toml`](packages/shared/Cargo.toml)); on Android and iOS, `getrandom` 0.4.3. It panics if no CSPRNG is available |

Versions are those pinned in [`Cargo.lock`](Cargo.lock).

XChaCha20-Poly1305 was chosen over AES-GCM because WebAssembly has no AES instructions. ChaCha20
uses no secret-dependent table lookups or branches. Whether the compiled code runs in constant time
also depends on the crate and the wasm engine, and nothing in this repository measures it.

### 3.1 Quantum adversary

A quantum computer breaks X25519, not the symmetric primitives (XChaCha20-Poly1305 under 256-bit
keys, Argon2id, SHA-256, HKDF-SHA256) nor ML-KEM-768. Rows encrypted under a project key or the
account key hold against it, except through two paths:

- Classical invitation boxes and the public keys they were made with are stored by the server: a
  future quantum adversary with the database or a backup recovers every project key sent in one,
  and through it every row of those projects written before the project's next rotation. They are
  written until `MIN_APP_VERSION` reaches 3.2.10; hybrid invitations (9.1) are not exposed, and
  after that floor no device that has seen it writes a classical box (one the server never shows
  the floor to still does, 14.3). Rotation boxes (8.6) are not exposed either:
  their key needs the ML-KEM-768 shared secret as well as the X25519 one. An account member whose
  account has no ML-KEM key yet (no current client has started on it) gets no box, and the new key
  only through the new link.
- TLS carries the login proof and `kdf_salt`; an adversary who records a handshake now and breaks
  it later can run the offline password search of section 4. counted.fr offers the hybrid
  `X25519MLKEM768` key exchange first, so a browser that supports it (current Chrome, Edge and
  Firefox) is not exposed. Neither are the Android and iOS apps from mobile 3.2.8: their requests
  go through `reqwest` on `rustls`, which takes the process-wide provider when one is installed,
  and the app installs aws-lc's before building its client (`install_tls_provider` in
  [`packages/mobile/src/main.rs`](packages/mobile/src/main.rs)), which offers `X25519MLKEM768`
  first. Mobile installs up to 3.2.7 used reqwest's fallback, ring, which offers no ML-KEM group, so
  they negotiate X25519 alone.

## 4. Keys and their encodings

| Key | How it is made | Where it lives |
| --- | --- | --- |
| Project key | 32 random bytes (`generate_key`) | In the share-link fragment; in the local store of every member device; escrowed under the account key for account holders (section 7) |
| Password key | `Argon2id(password, kdf_salt)`, m = 65536 KiB (64 MiB), t = 3, p = 1, 32-byte output (`derive_account_key_v1`) | Derived on the device at login. Never sent |
| Account master key ("account key" below) | At `amk_epoch` 0, the password key itself. From the first password change or recovery key, 32 random bytes, wrapped under the password key (7.5) | On the device for the session, in the local store as base64url (`credentials.rs` `remember_account_key`). Never sent in clear |
| Login proof | `Argon2id(password, login_salt)` with the same parameters and Argon2's secret input `K = "counted-login-v2"` (`derive_login_proof`) | Sent at registration, at email verification and at every login. The server stores `SHA-256(proof)` **[Not independently verified]** |
| X25519 keypair | `crypto_box::SecretKey` from 32 random bytes | Public key in clear on the server; private key wrapped under the account key (section 7) |
| ML-KEM-768 key | A 64-byte seed (`generate_kem_seed`), expanded by `ml-kem`'s `DecapsulationKey::from_seed` | Encapsulation key (1184 bytes) in clear on the server; seed wrapped under the account key (7.1) |
| Recovery key | 32 random bytes, shown once as 52 characters of the RFC 4648 base32 alphabet in 13 groups of four ([`recovery_key.rs`](packages/ui/src/auth/recovery_key.rs)) | With the user. The server stores a wrap under a key derived from it and a hash of a second derived secret (7.5) |
| Creator secret | 32 random bytes, for a project created without an account (8.4) | In the creating device's local store only. The server stores its SHA-256 until it is used |

- `kdf_salt` and `login_salt` are 16 random bytes each, generated by the client at registration and
  stored by the server, which returns them at login. The client refuses a salt that is not 16
  bytes, and an `auth_version` other than 2, before deriving anything
  ([`credentials.rs`](packages/ui/src/auth/credentials.rs) `checked`, `account_key_of`).
- **`K` is a public constant**, not a secret. It makes the login proof a different function of the
  password than the password key, so that the server does not receive the password key even if it
  serves `login_salt = kdf_salt`. It adds no secrecy.
- The server receives the proof, not the password. Anyone holding `login_salt` and
  `SHA-256(proof)`, or `kdf_salt` and any blob under the password key (`amk_wrap`, or at epoch 0
  any account blob; a database dump or a backup),
  and any server that receives proofs, can run an offline guessing attack at one Argon2id
  (64 MiB, t = 3) per guess. A guessed password yields the account key and every escrowed project
  key (section 7.4).
- Salts are served by the server at login. A hostile server can serve the same `login_salt` to many
  accounts and then test each password guess against all of their proofs at once.
- `login_salt` is kept for the account's life, a password change, a recovery and a reset included,
  so that the salt served for an address never changes: an address with no account is answered
  with a stable fake salt, and a real one that changed would tell that the address has an account.
  An offline search run against one dump therefore also covers later proofs of the same account.
- The proof is a bearer credential: whoever captures one can log in (sessions, ciphertext, account
  deletion) without learning the account key. Deleting the account takes the proof, not the session
  alone, from the key-lifecycle clients on (section 1); the session-only route is refused once the
  server's floor reaches them **[Not independently verified]**.
- **Key lifecycle** (7.5). The password can be changed: the master key is wrapped under the new
  password key, and the account first moves to a random master key if it was at epoch 0. "Someone
  may know my password" also replaces the master key and both halves of the keypair. A recovery key
  restores the master key with the emailed link. Whoever held a leaked password keeps the project
  keys they already opened, until each of those projects changes its key (8.5).
- A new password must be 12 to 128 **bytes** of UTF-8 (`password_error_key`, checked when a
  password is set; clients up to mobile 3.2.7 and web v1.24.3 accepted 8). Login checks no length,
  so older accounts keep their password until they change it. There is no strength check.
  A new password is **NFC-normalised** before derivation (`nfc` in
  [`credentials.rs`](packages/ui/src/auth/credentials.rs)). A sign-in derives from the NFC form
  and, only when the typed password is not NFC and that proof is refused, from the raw bytes, which
  is what passwords set by earlier clients were derived from (`password_forms`). Such a password,
  composed differently on two platforms, still derives two keys until it is changed.
- p = 1 because the WebAssembly build is single-threaded.

**Encodings.** A key, wherever it appears as text (fragment, local store, inside a wrap), is
**base64url without padding** (RFC 4648 section 5), 43 characters. `ct` and `iv` are **standard
base64 with padding** (RFC 4648 section 4), and so are, as exceptions, the key inside a rotation
box and the public keys of a directory entry (8.6), the `rotatedFrom` hashes (8.5) and the creator
secret and its verifier (8.4). The project token travels as base64url without padding (8.1).

## 5. Wire format v1

### 5.1 The encrypted pair

Every encrypted value is an `EncryptedPair` ([`shared/src/lib.rs`](packages/shared/src/lib.rs)):

```
ct = base64(XChaCha20-Poly1305-Encrypt(key, nonce, plaintext, aad = empty))   // ciphertext || 16-byte tag
iv = base64(nonce)                                                           // 24 random bytes
```

- A fresh random nonce is drawn for every encryption (`seal` in [`crypto.rs`](packages/ui/src/crypto.rs)).
  With 192-bit random nonces, the probability of a nonce collision after q encryptions under one key
  is about q² / 2^193.
- `open` rejects an IV that does not decode to exactly 24 bytes, and any tag failure.
- **No associated data is bound.** A ciphertext is not tied to its project, table, row or purpose.
  Within one project, a UUID holder or the server can duplicate a row (a copied expense is a
  complete, valid duplicate), swap two rows' payloads, serve a row's earlier ciphertext (an expense
  as it was before an edit), or leave a row out. None of this is detected. Swapping two participant
  rows reassigns who paid and who owes in every expense, since expenses name participants by row
  id. Swapping two members' payment-method copies (8.3) shows one member's IBAN under another's
  name. Across projects the key differs, and a moved row does not open. Account-key blobs: 7.3.
- **The AEADs are not key-committing.** Several checks treat "the project row opens under K" (AEAD
  check passed and the plaintext parsed as `ProjectPayload`, `decrypt_project` in
  [`decrypted.rs`](packages/ui/src/decrypted.rs)) as evidence that K is the project's key: key
  adoption (section 6), the escrow pull (7.2), the invitation check (9.1), and the missing-key
  screen. The rotation switch (8.5) relies on it too, with a weaker parse: any JSON whose
  `rotatedFrom` names the held key. Ciphertexts that pass Poly1305 under two chosen keys are known to be easy to build (Len,
  Grubbs and Ristenpart, "Partitioning Oracle Attacks", USENIX Security 2021). Whether one can also
  parse as a `ProjectPayload` under both keys has not been analysed. In practice the active server
  does not need one: it can serve a row encrypted under its own key and offer that key (section
  9.1). Partitioning oracles against the password key give the server nothing
  beyond the offline search it already has (section 4).
- The plaintext is UTF-8. Every value is JSON (`encrypt_json`), except the untyped key wraps of
  section 7.1, the untyped friend labels of section 9.3 and the invitation box of section 9.1, whose
  plaintexts are bare strings. Padded values (5.3) are JSON followed by spaces.

### 5.2 Payloads

Each row's sensitive fields are serialised as one JSON object and encrypted as one pair. The
payload structs in [`shared/src/lib.rs`](packages/shared/src/lib.rs) (`ProjectPayload`,
`UserPayload`, `ExpensePayload`, `HistoryPayload`, `PaymentPayload`, `Preferences`,
`PaymentMethods`) and in [`recurring/model.rs`](packages/ui/src/recurring/model.rs)
(`RecurringPayload`, `Template`) are the source of truth. They use `camelCase` keys. Optional
fields are omitted when absent (`skip_serializing_if`), except `Preferences.language`, which is
written as `null`.

`ProjectPayload`, `ExpensePayload`, `Preferences`, `RecurringPayload` and `Template` keep the keys
they do not know in a `#[serde(flatten)] extra` map and write them back, so an older client that
edits a row does not drop a field added by a newer one.

### 5.3 Padding

`encrypt_padded_json` appends ASCII spaces to the JSON text until its length is
`max(256, next_power_of_two(len))` bytes (`pad_json` in [`shared/src/lib.rs`](packages/shared/src/lib.rs)).
JSON parsers ignore trailing whitespace, so reading needs no change.

A blob whose ciphertext the server caps (`encrypt_padded_json_within` in
[`crypto.rs`](packages/ui/src/crypto.rs)) is padded the same way, but never past the largest
plaintext whose ciphertext fits the cap: `floor(cap / 4) * 3 - 16` bytes, the cap counting base64
characters. The top bucket then holds every length from the previous power of two up to that
limit. A plaintext already over it is not padded, and the server refuses it as it would unpadded.
The caps are `MAX_PREFERENCES_LENGTH` (4096, top bucket 3056 bytes), `MAX_PAYMENT_METHODS_LENGTH`
(32768, top bucket 24560 bytes) and `MAX_FRIEND_LABEL_LENGTH` (1024, top bucket 752 bytes), in
[`shared/src/lib.rs`](packages/shared/src/lib.rs).

Not every blob is padded; see the table below. The length leaks are in section 12. Clients up to
mobile 3.2.7 and web v1.24.3 wrote preferences, payment methods and friend labels unpadded, and
those blobs keep their exact length until they are saved again.

### 5.4 Ciphertext inventory

The last column gives examples. Every writer outside tests follows its row's padding rule.
"account" is the account master key (7.5).

| Blob | Key | JSON | Padded | Written in (examples) |
| --- | --- | --- | --- | --- |
| Project payload | project | `ProjectPayload` | yes | [`add_project_modal.rs`](packages/ui/src/projects/modals/add_project_modal.rs), [`edit_project_modal.rs`](packages/ui/src/expenses/modals/edit_project_modal.rs), [`decrypted.rs`](packages/ui/src/decrypted.rs) |
| Participant payload | project | `UserPayload` | yes | [`add_project_modal.rs`](packages/ui/src/projects/modals/add_project_modal.rs), [`edit_project_modal.rs`](packages/ui/src/expenses/modals/edit_project_modal.rs) |
| Expense payload (amount, date, name, payers, debtors, author) | project | `ExpensePayload` | yes | [`add_expense_modal.rs`](packages/ui/src/expenses/modals/add_expense_modal.rs), [`edit_expense_modal.rs`](packages/ui/src/expenses/modals/edit_expense_modal.rs) |
| Recurring rule (template, schedule, cursor, seed) | project | `RecurringPayload` | yes | [`recurring/requests.rs`](packages/ui/src/recurring/requests.rs) |
| History entry (summary, actor, action, entity) | project | `HistoryPayload` | yes | [`delete_expense_action.rs`](packages/ui/src/expenses/helpers/delete_expense_action.rs), [`edit_project_modal.rs`](packages/ui/src/expenses/modals/edit_project_modal.rs), [`recurring/requests.rs`](packages/ui/src/recurring/requests.rs) |
| Claim label (claimant's display name) | project | string | yes | `claim_label` in [`crypto.rs`](packages/ui/src/crypto.rs) |
| Shared payment methods, per project | project | `PaymentMethods` | yes, within the cap | `project_copies` in [`payment_methods.rs`](packages/ui/src/payment_methods.rs) |
| Display name | account | string | yes | [`register_page.rs`](packages/ui/src/auth/register_page.rs) |
| Preferences | account | `Preferences` | yes, within the cap | [`preferences.rs`](packages/ui/src/preferences.rs) |
| Payment methods | account | `PaymentMethods` | yes, within the cap | `save_payment_methods` in [`payment_methods.rs`](packages/ui/src/payment_methods.rs) |
| Friend-request label (an email address or a participant name) | account | string; bare text from older clients | the string within the cap; bare text **no** | `write_label` in [`friends_service.rs`](packages/ui/src/friends/friends_service.rs) |
| Escrowed project key | account | `KeyWrap`, or bare fragment when untyped | no (fixed size) | `wrap_key` in [`crypto.rs`](packages/ui/src/crypto.rs) |
| Private key | account | `KeyWrap`, or bare fragment when untyped | no (fixed size) | `wrap_key` in [`crypto.rs`](packages/ui/src/crypto.rs) |
| ML-KEM seed | account | `KeyWrap` (`account-kem`) | no (fixed size) | `wrap_kem_seed` in [`crypto.rs`](packages/ui/src/crypto.rs) |
| Master-key wrap | password key | `KeyWrap` (`amk`) | no (fixed size) | `wrap_amk` in [`crypto.rs`](packages/ui/src/crypto.rs) |
| Recovery wrap | `rk_key` (7.5) | `KeyWrap` (`amk-recovery`) | no (fixed size) | `wrap_recovery` in [`crypto.rs`](packages/ui/src/crypto.rs) |
| Member-key directory entry (a member's two public keys) | project | `{"use":"member-key",..}` | no (fixed size) | `member_key_entry` in [`rotation.rs`](packages/ui/src/rotation.rs) |
| Invitation box (project key for a friend), until floor 3.2.10 | X25519 pair | bare fragment | no (fixed size) | `box_project_key` in [`crypto.rs`](packages/ui/src/crypto.rs) |
| Invitation box, from floor 3.2.10 | hybrid, sender-authenticated (9.1) | `{"use":"invite",..}` | no (fixed-length fields) | `box_for` in [`friends_service.rs`](packages/ui/src/friends/friends_service.rs) |
| Rotation box (new project key for a member who stays) | hybrid (8.6) | `{"use":"rotate",..}` | no | `seal_box` in [`rotation.rs`](packages/ui/src/rotation.rs) |
| Queued offline op: its label, and a project edit's name and description (local only, never sent) | project | string, tuple | no | [`offline_queue.rs`](packages/ui/src/common/offline_queue.rs), [`edit_project_modal.rs`](packages/ui/src/expenses/modals/edit_project_modal.rs) |
| `PaymentPayload` (amount, direction), rows written by older clients only | project | `PaymentPayload` | no | no longer written |

### 5.5 Rows that do not open

An expense row that fails to decrypt is dropped from every figure, with its payments, and counted,
and the page says that some expenses are unreadable rather than showing partial totals as complete
([`project_data.rs`](packages/ui/src/expenses/helpers/project_data.rs)). A participant row that fails
to decrypt is left out of the participant names and is not counted. A project row that does
not open under the held key shows the missing-key screen. A key rotation does not re-encrypt rows
that fail to decrypt; it sets them aside (8.5).

A row that opens is still hostile input: a key holder controls the JSON every member's client
parses. Property tests feed arbitrary strings to the project, participant, expense, payment,
recurring-rule, history, payment-method and claim-label decoders. They also feed arbitrary rows,
with any `f64` amount, ids at the `i32` bounds, dates at chrono's limits, junk types, and
participants carrying any claim label and payment-method copy, through what each page computes from them: the
expenses page (totals, balances, settlement), the charts, my spending and budgets, the recurring
rules (schedule, split, materialisation), the history and the exports. They check that nothing
panics and that totals stay finite. The strategies are in
[`hostile.rs`](packages/ui/src/common/hostile.rs); each page's test is a `hostile_payloads` module
next to its code. Not covered: the rendering itself and the locale formatting, which needs the UI
runtime.

The same checks run under libFuzzer ([`packages/ui/fuzz`](packages/ui/fuzz),
[`fuzz.rs`](packages/ui/src/fuzz.rs)): each page's property test calls an `exercise_hostile`
function that the fuzz target calls too. The `decoders` target feeds raw bytes as plaintext to
every decoder above, `Preferences`, both key-wrap uses and the share-link parser; the `pages` target
reads them as rows. Campaign of 2026-11-01, one hour per target: 1,245,755 inputs to `decoders`
and 71,455 to `pages`, no crash.

## 6. Key distribution

A share link is built from the project id and key, never read from the address bar
([`share_link.rs`](packages/ui/src/common/share_link.rs) `share_url`, `share_link_for`):

```
https://counted.fr/projects/<uuid>#<base64url(project_key)>
counted://projects/<uuid>#<base64url(project_key)>          (app deep link)
```

- Browsers do not send the fragment in HTTP requests or in `Referer`. It is readable by any script
  running in the page, and by every app, chat or clipboard tool the link passes through. The link is
  the key: whoever sees it can read the project.
- On the web, once the key is stored, `clear_hash` replaces the current session-history entry with
  the path alone ([`web_dom.rs`](packages/ui/src/common/web_dom.rs),
  [`use_project_store.rs`](packages/ui/src/expenses/hooks/use_project_store.rs)). Whether the browser
  had already written the full URL to its global history or a sync service is outside the app's
  control. A key that was refused (below) stays in the address bar.
- `parse_share_link` accepts only the project route, and is the only parser for links that arrive
  as deep links or are pasted into the app.
- The `counted://` scheme is not exclusive: any installed app can register it and receive the link,
  key included. Verified `https` app links do not have this problem.
- **Adopting a key** (`adoption`, `adopt_project_key` in
  [`membership.rs`](packages/ui/src/common/membership.rs)). "Opens" below means `decrypt_project`
  succeeds: AEAD check, then a parse as `ProjectPayload`.
  - A device that holds no key for the project takes the candidate, unchecked.
  - A held key that has opened the project on this device is never replaced, except by a key that
    passes the rotation rule of 8.5, whether it comes from a new link, an invitation, a rotation
    box or escrow. The device knows the held key has opened the project
    from `key_opened`, a flag set in its local store the first time the held key opens a project row
    the server returns, or because the held key opens the project row the device has cached (its own
    entry, or the cached projects list). The flag survives the caches being dropped and is reset
    only when the stored key changes.
  - A held key without that proof (a wrong link stored earlier, or a key pulled from escrow and
    never opened) is replaced only by a candidate that opens the cached project row, or, when
    nothing is cached, the row the server returns now (fetched with the candidate's claim token),
    and only once the user confirms (`adoption`, `answer_key_offer`): the device names the project
    the candidate opens and asks whether to use the new key. Cancel writes nothing, so the held
    key, its `key_opened` and its `chain_seen` stay as they were, and opening the same link again
    asks again. A confirmation is dropped if the held key changed or opened the project while the
    question was up. A candidate whose served row names the held key in `rotatedFrom` (8.5)
    switches without the question. Offline with nothing cached, the held key stays and nothing is
    asked.

  Both rows are the server's word: the check stops a wrong link given to an honest server, and the
  question stops a hostile server that chose the candidate and serves a row encrypted under it,
  since nothing replaces the held key without a link or invitation the user opened and then a
  second, explicit yes. Clients up to mobile 3.2.7 and web v1.24.3 checked every candidate against
  the served row only (section 9.1); clients built before 2026-11-06 replaced an unproven held key
  without asking.

## 7. Account-key blobs and key escrow

### 7.1 Wraps

A project key or the private key is stored under the account key as a wrap (`wrap_key`,
`open_wrap`, `unwrap_key` in [`crypto.rs`](packages/ui/src/crypto.rs)):

- **Untyped**: `encrypt(account_key, base64url(key))`. The plaintext says nothing about its purpose.
- **Typed**: `encrypt_json(account_key, KeyWrap)`, where `KeyWrap` serialises as
  `{"use":"escrow","project":"<uuid>","key":"<base64url>"}` for a project key, or
  `{"use":"account-sk","key":"<base64url>"}` for the private key.

`open_wrap(use)` decrypts. If the plaintext parses as a `KeyWrap`, it returns the key only if
`(use, project)` match the expected purpose. Otherwise it reads the plaintext as a bare fragment
and marks the result untyped. A JSON string (such as a typed friend label) parses as neither.

Three more uses exist only typed, each opened by its own function with no untyped fallback (7.5):

```
amk_wrap      = encrypt_json(password_key, {"use":"amk","account":"<uuid>","epoch":<n>,"key":"<base64url>"})   // open_amk
recovery_wrap = encrypt_json(rk_key, {"use":"amk-recovery","account":"<uuid>","key":"<base64url>"})          // open_recovery
kem_wrap      = encrypt_json(account_key, {"use":"account-kem","key":"<base64url of the 64-byte seed>"})      // open_kem_seed
```

`open_amk` returns the key only for the account and epoch the server serves, and `open_recovery`
only for the account.

### 7.2 Escrow

For an account holder, each project key is pushed to the server as a wrap on the membership row,
so that a new device can fetch it ([`membership.rs`](packages/ui/src/common/membership.rs)
`ensure_membership`). Escrow is not optional for an account holder. The membership write also
carries the claim token (section 8.1). The server stores the wrap and the claim label only if the
token matches the project's verifier; otherwise it drops both. If the write also claims a
participant, the claim is refused (the single-project write fails with 403, the batch keeps the
membership without the participant). Without a claim the membership is recorded if the account
already holds the project, or the token matches, or the project has no verifier;
otherwise a new membership is refused (403, or dropped from the batch) **[Not independently
verified]**.

When pulling (`apply_pull` in [`account_sync.rs`](packages/ui/src/common/account_sync.rs)), a device:

- ignores a wrap for a project it already holds a key for (after that project's key changed, a
  typed escrow row is instead a candidate for the switch of 8.5);
- refuses a key equal to the account's own private key;
- refuses an untyped wrap once a previous pull has opened a typed one for this account (7.3);
- if it has a cached copy of the project row, keeps the key only if it decrypts that row. **A fresh
  device has no cached row, so it adopts the escrowed key without this check.** The server cannot
  create a wrap without the account key: on a fresh device it can only substitute another genuine
  untyped blob under the same account key (7.3).

### 7.3 Typed wraps and their downgrade window

Mobile clients older than 3.2.6, and web builds served before the same release, read only untyped
wraps; the server refuses those mobile clients **[Not independently verified]**. Current clients
write every wrap typed, the private key's included at keypair creation (`wrap_key` in
[`crypto.rs`](packages/ui/src/crypto.rs)), and re-wrap an untyped private key or escrow row typed
at session start and on the projects page (`typed_rewrap` in
[`credentials.rs`](packages/ui/src/auth/credentials.rs), `to_push` in
[`account_sync.rs`](packages/ui/src/common/account_sync.rs)). `Account.typed_wraps`, which the
server sets, is no longer read. Clients up to mobile 3.2.7 and web v1.24.3 wrote typed wraps only
while the server reported it true.

A device records each account for which it has opened a typed wrap
(`LocalStorageState.typed_wrap_accounts`), in an escrow pull or at session start when the private
key it is served is typed, and refuses untyped wraps for it from then on.
Untyped rows served in the same response as the first typed one are still accepted
(`apply_pull` decides `raw_ok` once, before the loop). This is deliberate. While accounts migrate,
one response legitimately holds typed rows next to untyped rows that no device has re-wrapped yet,
and refusing those would leave their projects without a key on a new device. Closing the window
would also gain nothing against a hostile server: it chooses what each response holds, and can
leave the typed rows out of the response in which it serves an untyped one.

Residual: a device that has never opened a typed wrap for an account accepts untyped ones. A
hostile server can keep a device in that state, a new one for instance, by never serving it a
typed wrap (dropping re-wrap writes, serving untyped blobs it kept from before). Under an untyped
wrap, a server can serve one account-key blob in another's slot: an escrow row of one project as
another's (stopped by the cached-row check only when a row is cached), or a friend label as a key.
That last swap is stopped for labels written as JSON strings (section 9.3). A label written as
bare text opens as a key if it happens to be a valid 43-character fragment, which a co-member
choosing a participant's name can arrange.

Display name, preferences, payment methods and friend labels carry no type at all. A server can
serve one in another's slot. The client then shows the wrong content or fails to parse it.

### 7.4 What escrow costs

Escrow is mandatory for account holders. An escrowed project key changes only when its project's
key does (8.5), and a password change leaves it under the same master key (7.5). Without escrow,
the database and a user's password reveal that account's display name, preferences and payment
methods. With escrow they also reveal every project key the account holds, and so every project's
content. Escrow is the only key-recovery path for a lost device, and anonymous members (no account)
have none. A recovery key (7.5) covers a forgotten password; without one, a reset starts the
account over and its escrowed keys are lost to it.

### 7.5 Master key, password change and recovery

[`account_key.rs`](packages/ui/src/auth/account_key.rs),
[`account_rotation.rs`](packages/ui/src/auth/account_rotation.rs),
[`recovery_key.rs`](packages/ui/src/auth/recovery_key.rs). Off on the server until its client
floor allows (section 1).

- **Epochs.** At `amk_epoch` 0 the master key is the password key. From epoch 1, `amk_wrap` holds a
  random master key under the password key (7.1). A device opens it only for the account and
  epoch served, and uses it only if it opens the private key whose public half is served
  (`master_key`). It pins the highest epoch it has opened for the account and refuses a lower one.
  At epoch 0 a sign-in makes no such check: a served public key that is not the private key's is
  caught by the keypair check wherever a key is used (`own_keys`, `my_private_key`) and shown as a
  warning, and a session restored from the cookie runs no key check at all beyond the epoch.
  Every write of a blob under the master key names the epoch it was made under; the server refuses
  a stale one with 409 `ACCOUNT_KEY_CHANGED` **[Not independently verified]**, and the device then
  catches up with the wrap served or asks for the password again.
- **Password change.** The request carries the current proof, a new proof over the same
  `login_salt`, a new `kdf_salt` and the master key wrapped under the new password key. At epoch 0
  the account first moves to a fresh master key: every account blob is decrypted and the same
  bytes sealed under it, and the request must cover exactly the escrow rows and friend labels the
  server lists; a blob that does not open is sent as missing and cleared. Past epoch 0 only
  `amk_wrap` changes. The keypair is kept, so friend codes do not change. Other sessions are
  deleted **[Not independently verified]**.
- **Compromise rotation** ("Someone may know my password"): a password change that always moves to
  a fresh master key, with a fresh X25519 key and ML-KEM-768 key, and, if a recovery key is set, a
  new one that replaces it 72 hours later. Friends see this account's code change (9.2), and
  pending invitations are dropped **[Not independently verified]**. Whoever had the password keeps
  the project keys they opened; only each project's key rotation (8.5) cuts them off from later
  rows.
- **Recovery key.** With the account id (its 16 bytes) as salt, so that one search does not cover
  every account:

  ```
  rk_key            = HKDF-SHA256(salt = account_id, ikm = recovery_key, info = "counted-recovery-v1")       // 32 bytes
  auth              = HKDF-SHA256(salt = account_id, ikm = recovery_key, info = "counted-recovery-auth-v1")  // 32 bytes
  recovery_wrap     = the amk-recovery wrap of 7.1, under rk_key
  recovery_verifier = SHA-256(auth)                                                                          // stored by the server
  ```

  Setting a key needs the password's proof (at epoch 0 the account first moves to a fresh master
  key, as above); replacing one takes effect 72 hours later, and the current key's `auth` cancels
  the replacement. Recovering takes the emailed link and `auth`; the server then serves the wrap,
  and the client sets a new password over the same master key **[Not independently verified]**.
  The wrap carries no epoch, so a wrap kept from an earlier generation still opens.
- **Reset without the recovery key** gives the account new keys, 72 hours after the request.
  What it supersedes is kept 30 days and served only after the old password's proof **[Not
  independently verified]**. The new keys open none of the old escrow rows: a project comes back
  from a device that still holds its key, or from a share link.
- **What a routine password change does not do**: change the master key. An old password together
  with an old copy of `amk_wrap` (a backup, for instance) still yields the current master key and
  every blob under it. A compromise rotation is what replaces it.

## 8. Project keys: derived values, roles and rotation

### 8.1 Claim token and verifier

To claim a participant as their identity, to escrow the key, to rewrite a project's history, and
from the key-lifecycle clients on (section 1) for every request about a project, a client proves it
holds the project key:

```
claim_token    = SHA-256("counted-claim-v1" || project_key)       // 16 ASCII bytes, then 32 bytes
claim_verifier = SHA-256(claim_token)                             // stored by the server
```

The client sends the token. The server hashes it and compares it with the verifier **[Not
independently verified]**. A project created by a current client sends its verifier at creation. A
project created before verifiers existed has none until one of its admins (8.4) presents a token,
which becomes the verifier; a project without an admin keeps none. Until then claims and escrow are
unchecked **[Not independently verified]**. Neither value reveals the key. Once the server has seen
a token, which happens on every membership write and, below, on every request, it can present it
itself: a claim proves key possession to an honest server only.

**The key as the capability.** Key-lifecycle clients send the token with every request about a
project: in the `x-project-token` header, base64url without padding, never in the query string;
the batch routes carry one token per project in their body
([`project_token.rs`](packages/ui/src/common/project_token.rs)). Once the server enforces it, a
project with a verifier answers only to its token: 403 `PROJECT_KEY_REQUIRED` without it, and a
batch leaves the project out. Projects without a verifier stay open to anyone holding the id.
Leaving a project, creating one and the invitee's own invitation routes need no token **[Not
independently verified]**.

**After a key rotation** (8.5) the server keeps every earlier verifier. A request whose token
matches one of them answers 403 `PROJECT_KEY_ROTATED`, which sends the device looking for the new
key, and the projects list's batch sync answers such a project with `rotated` set and nothing else
(`ProjectSync.rotated` in [`shared/src/lib.rs`](packages/shared/src/lib.rs)) **[Not independently
verified]**. The server thus tells requests from holders of an older key apart from requests with
no key at all.

### 8.2 Claim label

When an account claims a participant, its display name is decrypted under the account key and
re-encrypted, padded, under the project key, so that the other members can see who claimed it
(`claim_label`). Anyone holding the project key can read it. A UUID holder without the key learns
only that the participant is claimed, and the label's padded length.

### 8.3 Shared payment methods

A user's payment methods (IBAN, wallet handles) are encrypted under the account key. The ones the
user marks as shared are copied, padded within the cap (5.3), under the key of every project where the device
holds the key and an identity (`project_copies`). Any project key holder can read the copy.

The copies have one write path: a save replaces all of them, and the client
sends `expected_iv`, the nonce of the account blob it started from, which the server must still
hold **[Not independently verified]**. Because every encryption draws a fresh nonce, the IV serves as
a compare-and-swap token (unordered, not a version number). Unsharing asks the server to delete copies. A dishonest server can keep serving
an old copy, and members cannot tell. A key rotation (8.5) re-seals only the rotating admin's own
copy and clears the others, which their owners' devices write back under the new key.

### 8.4 Project roles and proof of creator

Only an admin can change a project's key or remove a member (8.5). The role is a clear column on
the membership row, `admin` or `member`, and the server checks it on the role and rotation routes
**[Not independently verified]**. It is in clear because the server must act on it when the last
admin leaves, and the server already knows who belongs to each project.

- **Who.** The account that creates a project is its admin. An admin can make another account
  member admin, take the role from another admin, or give it up. Members without an account cannot
  hold it: they have no identity a server check can bind to.
- **Succession.** When the last admin's membership goes (leaving, account deletion, a reset, a
  removal) or the last admin gives the role up, the account member who joined first among those
  holding the key (an escrowed key or a claimed participant) becomes admin, in the same database
  transaction. With no such member the project has no admin **[Not independently verified]**.
- **Proof of creator.** A project created without an account gets a creator secret: 32 random
  bytes from the creating device, kept in its local store as standard base64, never in a link, an
  escrow row or a synced payload ([`creator_proof.rs`](packages/ui/src/common/creator_proof.rs)).
  The project is created with

  ```
  creator_verifier = base64(SHA-256(creator_secret))      // CreatableProject.creator_verifier
  ```

  When that device signs in, it writes its escrowed membership, then sends the secret with its
  session and the project token. The server makes the account admin only while the project has no
  admin and only for a membership that holds the key, then deletes the verifier, so the secret
  works once **[Not independently verified]**. The demo and projects created signed in carry none.
  A cleared store loses the secret. `ProjectDto.has_creator_proof` tells other devices that only
  the creator can still become admin.
- **Projects without an admin** (created without an account by a client before the key lifecycle,
  or whose last key-holding account member left) cannot change their key. Removing someone there
  only hides the participant (`removed` inside its payload), and the app says that this does not
  revoke access.
- **The role is not cryptographic.** Any key holder can still write any row (section 2), and a
  hostile server can grant or withhold the role as it likes. It cannot use the role to rotate a key
  it does not hold (8.5).

### 8.5 Key rotation

An admin's device replaces the project key K1 by a fresh K2, re-encrypts every row under it, and
switches the project in one transaction ([`rotation.rs`](packages/ui/src/rotation.rs),
[`rotation_flow.rs`](packages/ui/src/rotation_flow.rs); routes in
[`rotation_controller.rs`](packages/api/src/projects/rotation_controller.rs)). The server refuses a
rotation until it refuses older clients and enforces project tokens **[Not independently
verified]**: an older client could not follow it, and without tokens K1 would keep reading the
project by id.

1. **Freeze.** The admin starts a rotation (session, K1's token, role). For one hour, or until the
   commit or an abort, every write to the project's rows answers 409 `PROJECT_ROTATING`;
   membership rows are not frozen. One rotation at a time per project; read-only and demo
   projects are refused **[Not independently verified]**. Without the freeze, a member being
   removed, who holds K1 and a valid token until the commit, could keep writing rows and make every
   commit fail. The app asks whoever is refused to try again in a moment.
2. **Staged rewrite.** The device reads every row in pages of `ROTATION_PAGE` (200), set by set
   (`ROTATION_TABLES`: the project row, participants, expenses, payments, recurring rules,
   history, and the admin's own claim label and payment-method copy), and uploads them
   re-encrypted. Another member's label or copy has one writer, that member's account: the server
   refuses to stage it **[Not independently verified]**, the commit clears it, and the member's
   device writes it back under K2. The project row
   must open under K1, or the rotation stops there: a wrong key never reaches the commit. Every
   other row is decrypted to bytes and the same bytes are sealed under K2 with a fresh nonce
   (`reseal`), so padding, fields this client does not know and plaintext that does not parse
   survive byte for byte. Two rows are rewritten instead:
   - the project payload, which gets `rotatedFrom` (below) and is padded again;
   - in a removal, the removed participant's row, which gets `"removed": true` and is padded again
     (`reseal_removed`), every other field kept.

   A row whose AEAD tag fails under K1 is junk written by an id holder (5.5) and opens under no key
   a member holds. It is not re-encrypted. A participant, expense, payment, recurring or history row
   is listed as set aside (`quarantine`) and stays as it is, unreadable under K2 as it was under K1;
   the server keeps the list 30 days **[Not independently verified]**. The admin's own claim label
   or payment-method copy is left out and the commit clears it. There is no threshold: a cap would let
   the person being removed block the rotation with a few rows of random bytes.
3. **Commit**, one transaction **[Not independently verified]**. The staged and set-aside ids must
   be exactly the live ids, set by set (for claim labels and copies, the admin's own staged one must
   still exist). Then the ciphertexts are swapped; the memberships listed in `remove` are deleted, here
   and not earlier, since a member removed before the switch could rejoin with K1's still-valid
   token; the old verifier joins the previous verifiers, and the new one,
   `SHA-256(claim_token(K2))`, and `key_epoch + 1` are set; the admin's escrow of K2 and own
   directory entry (8.6) are stored, and every other member's escrow row and directory entry,
   both K1 ciphertext, cleared; pending invitations, which box K1, and older rotation boxes are
   deleted; the new boxes are stored for accounts still members; the history entry, under K2 with
   its actor inside the payload, is written. The members who stay get a push of a new kind, "a
   project's key changed". On iOS the push kind reaches Apple in clear.

From then on, a request with K1's token gets 403 `PROJECT_KEY_ROTATED` (8.1).

**`rotatedFrom`.** The new project payload carries one hash per earlier key, newest first, each
in standard base64:

```
rotatedFrom = [ base64(SHA-256("counted-rotated-from-v1" || K_n)), ..., base64(SHA-256("counted-rotated-from-v1" || K_1)) ]
```

Hashes, not keys: a member who joins later gets nothing that opens an earlier epoch's rows in a
backup. Not the claim token either, which the server sees.

**Which key a device switches to** (`may_switch`). A device holding K1 takes a rotation's K2 only if
K2 differs from K1, K2 opens the project row the server serves, that row's `rotatedFrom`
contains the hash of K1, and that chain is longer than any the device has opened for the project
(`chain_seen`, kept in its local store from every project row its key opens and reset with the
key). The chain has one hash per rotation and sits inside the ciphertext, so its length is the
key epoch as the row itself states it, which the clear `key_epoch` column is not: a box or escrow
row kept from an epoch the device has moved past does not roll it back. The candidates are the account's own escrow row for the project (typed
wraps only), the rotation boxes addressed to the account (8.6), and a new share link or invitation
(section 6). **A box never fills an empty slot**: a device that holds no key for the project takes
nothing from one, since otherwise a hostile server could box any account a key for any project. A
fresh device of the account fills its empty slot from escrow, as in 7.2. A device that switches
moves K1 to a list of previous keys with its epoch, drops its cached rows, re-seals its queued
writes under K2 (`rekey_value`: every `{ct, iv}` pair in a queued op that opens under K1), and, if
the key came from a box, escrows K2 and deletes the box.

A hostile server alone cannot build a row that passes this rule: it needs the hash of K1, which
only holders of K1 or of a later key can compute or read. A former member alone cannot get one
served by an honest server, which holds K2's verifier. **Together they can**: a K1 holder and a
hostile server can serve a row under a key of their choosing that names H(K1), with a box or a
link, to a device that has not switched yet. Devices already on K2 are safe. The same holds for a
fresh device served an escrow row, or a box, from before an earlier rotation, which a server can
keep, and for a device that was offline across two rotations and is served the first one's row and
box: `chain_seen` protects only a device that has opened the later epoch (the first-sight rollback
of 15.7). A K1-only former member cannot roll a device on K2 back, since H(K2) is inside K3's
ciphertext.

**Devices the new key does not reach** (no box, no escrow, or none that passes the rule) keep K1, so
that a new link's row can name it, delete their cached rows of the project, and show only that its
key changed. The projects list keeps such a project as a row with no name.

**What a rotation does not do.** A former member keeps everything they read or cached: deleting
the cached rows on their device is the app's courtesy, not a guarantee. Anonymous members, and
account members whose directory entry does not check, get K2 only through the new link, which the
admin sends them outside the app.

### 8.6 Rotation boxes and the member-key directory

**The directory.** Each account member's client writes, on its own membership row and once per key
epoch and per keypair, an entry under the project key (`member_key_entry` in
[`rotation.rs`](packages/ui/src/rotation.rs), from a project open):

```
member_key = encrypt_json(K, {"use":"member-key","account":"<account uuid>","x25519":"<base64>","kem":"<base64>"})
```

The two public keys in the entry are **derived from the private halves the device holds** (the
X25519 private key and the ML-KEM seed the account key opens, `own_keys` in
[`friends_service.rs`](packages/ui/src/friends/friends_service.rs)), never copied from the
`Account` the server serves, and the entry is written only while each served half equals the
derived one. A device served substituted public halves writes no entry, vouches for nothing and
shows a warning; it opens no box with the served keys either (the same check gates `open_box`).
Without this a server could have a member's own device vouch, under K1, for keys the server
holds, and get the next key boxed to itself with no former member's help.

At a rotation the admin's device fetches the account members with their account ids, the public
keys the server serves for them and their entries; the server serves this list to admins only
**[Not independently verified]**. The device boxes K2 to a member only if the entry opens under K1,
says `member-key`, names that member's account, and both keys are byte-equal to the `public_key`
and `kem_public_key` served (`recipient_keys`). A server that substitutes a member's keys must
therefore forge an entry under K1, which it cannot do alone; a K1 holder colluding with it can, and
then reads K2 (8.5). Every member whose entry checks gets a box: the client offers no list to leave
one out, and does not prefer a pinned friend key.

**The box** ([`hybrid.rs`](packages/ui/src/hybrid.rs)), sealed to the recipient's X25519 key `pk_x`
and ML-KEM-768 encapsulation key `ek`:

```
e         = 32 random bytes;  E = X25519(e, 9)
ss_x      = X25519(e, pk_x)                                   // RFC 7748; refused if pk_x has small order
(c, ss_k) = ML-KEM-768.Encaps(ek, m)                          // m: 32 random bytes
k         = HKDF-SHA256(salt = none, ikm = ss_k || ss_x,
                        info = "counted-hybrid-v1" || E || pk_x || SHA-256(c) || SHA-256(ek) || context)   // 32 bytes
ct        = XChaCha20-Poly1305-Encrypt(k, nonce, plaintext, aad = context)                                // nonce: 24 random bytes
box       = { ephemeral: E, kemCt: c, nonce, ct }
```

`E`, `pk_x` and the two digests are 32 bytes each and the context comes last, so the info string is
unambiguous. The key depends on both shared secrets, both ciphertexts and both recipient keys: a box
sealed to a substituted half, or with either ciphertext altered, does not open, and the box stays
secret as long as either X25519 or ML-KEM-768 holds. Opening also refuses an ephemeral key of small
order. **The sender is not authenticated**: the box binds no sender key and anyone can seal to a
recipient. What makes a box acceptable is its content and the switch rule of 8.5.

This is the shape of X-Wing and of the TLS 1.3 hybrid key schedule (both secrets into one KDF)
rather than X-Wing itself, for three reasons: the two public keys are separate account fields (the
X25519 key predates the ML-KEM key and is also the invitation and fingerprint key of section 9),
so there is no single X-Wing public key to encapsulate to; the client already depends on SHA-256
and HKDF and on nothing from the SHA-3 family; and the KDF here binds the ML-KEM ciphertext and
both recipient keys explicitly instead of relying on ML-KEM's own ciphertext binding. A reviewer
who sees a reason to prefer X-Wing's combiner is asked to say so (section 18).

For a rotation, `context = "counted-rotate-v1"` and the plaintext is the JSON

```
{"use":"rotate","project":"<uuid>","to":"<account uuid>","key_epoch":<n>,"key":"<base64 of K2>"}
```

The recipient opens it only if every field matches: `use`, the project, its own account, and the
`project_id` and `key_epoch` the server serves with the box (`open_box`). The key still has to pass
the switch rule. The recipient uses its ML-KEM seed only if the encapsulation key it expands to is
the one the server serves for the account, and its X25519 private key only if it matches the
served public key (as in 9.1).

## 9. Friends and boxed invitations

### 9.1 The box

An account holder can send a project key to a friend's account, in one of two boxes. Which one is
the floor's choice (`HYBRID_INVITATIONS_FROM` in [`packages/shared/src/lib.rs`](packages/shared/src/lib.rs)):
a 3.2.9 reader requires the classical `key`, so the server accepts only the classical box until
`MIN_APP_VERSION` reaches 3.2.10, and only the hybrid one from then on (`one_form_for_the_floor`;
the table's CHECK holds exactly one well-formed form per row). A device that has seen the floor
(`Account.hybrid_invitations`), sent a hybrid box or opened one records it
(`LocalStorageState.hybrid_invitations`) and never writes a classical box again, so a server cannot
turn it back. It can still withhold the floor from a device that never saw it, and a recipient
still opens a classical box (14.3). From the floor, a friend whose account has no ML-KEM key yet gets no invitation, and
the sender is told that friend has to open Counted once; never a classical box in its place.

**Hybrid box** (from 3.2.10; `seal_auth`/`open_auth` in [`hybrid.rs`](packages/ui/src/hybrid.rs)).
Sender static `sk_s`/`pk_s`; recipient `pk_r` and `ek_r`, both the keys this device pinned (9.2):

```
e random;  E = X25519(e, 9)
ss_e      = X25519(e, pk_r)
ss_s      = X25519(sk_s, pk_r)                         // what authenticates the sender
(c, ss_k) = ML-KEM-768.Encaps(ek_r)
k         = HKDF-SHA256(salt = none, ikm = ss_k || ss_e || ss_s,
                        info = "counted-hybrid-auth-v1" || E || pk_s || pk_r || SHA-256(c) || SHA-256(ek_r) || context)
ct        = XChaCha20-Poly1305(k, nonce, plaintext, aad = context)        // context = "counted-invite-v1"
plaintext = {"use":"invite","project":"<uuid>","from":"<sender account>","to":"<recipient account>","key":"<base64url>"}
```

It is HPKE Auth mode's shape. The recipient matches the served `from_public_key` against its
friends as for the classical box, opens with its own X25519 key and ML-KEM seed (both checked
against the halves the server serves), and accepts only if `from` is that friend's account id, `to`
its own and `project` the invitation's (`open_invitation`). Every X25519 input goes through the
small-order check, `pk_s` at open included. Properties:

- **Confidentiality** holds if either X25519 or ML-KEM-768 does: a quantum adversary with the
  database also needs the recipient's ML-KEM seed.
- **Sender authentication** stays classical: a server without `sk_s` or `sk_r` cannot make a box
  that opens under `pk_s`, but a quantum adversary recovering both static X25519 keys could.
  Key-compromise impersonation remains (`sk_r` alone forges a box to its holder).
- **Reflection** fails before any field is read: a sender reopening its own box computes
  `X25519(sk_s, E)` and holds no seed for `ek_r`. `from`/`to`/`project` close re-serving to another
  recipient or under another project.
- The plaintext has fixed-length fields, so it is not padded. The server checks the box's lengths
  (32, 1088, 24, a ciphertext of 16 to 512 bytes), and the same check now covers rotation boxes.

**Classical box** (until 3.2.10):

```
shared   = u([clamp(sender_private) mod l] * recipient_public)     // l: order of the prime subgroup
k        = HChaCha20(shared, 0^16)
box      = crypto_secretbox_XChaCha20Poly1305(k, nonce, base64url(project_key))
```

`shared` is the Montgomery ladder of `crypto_box` 0.9.1, which reduces the clamped scalar mod l
(`SecretKey::from_bytes`). On public keys in the prime-order subgroup, which every key the app
generates is, this equals RFC 7748 X25519. On low-order, mixed-order or twist points it differs.

`crypto_secretbox` derives a subkey with HChaCha20 from the first 16 nonce bytes and runs ChaCha20
(the original 64-bit-nonce variant) with the last 8. It takes the Poly1305 key from keystream bytes
0-31, encrypts the plaintext with the keystream from byte 32 on, and outputs `tag || ciphertext`.
The Poly1305 tag covers the ciphertext only. **This is not the IETF
XChaCha20-Poly1305 AEAD of section 5**, and `crypto_box` 0.9 refuses associated data. The box goes
on the wire as an `EncryptedPair` with a 24-byte random nonce.

Properties of the classical box, and of both where they say so:

- **Authenticated, not sealed** (both boxes): the invitation comes with a sender public key
  (`ProjectInvitation.from_public_key`) and a `from_email`, both served by the server. The recipient
  opens the box only if that key is byte-equal to the public key of one of its friends, as its
  friends list serves it: the key whose code it can compare (9.2). It then shows that friend's email,
  not the served one (`open_invitation`, `load_invitations` in
  [`friends_service.rs`](packages/ui/src/friends/friends_service.rs)). An invitation from any other
  key can only be declined. The invitation names no sender account, so the key is matched against
  every friend, and the sender shown is the friend whose key it is. When two accounts stop being friends,
  the server deletes the pending invitations between them **[Not independently verified]**.
- **Reflection**: the static-static key is symmetric (the box key of A to B equals that of B to A),
  so a box A sent to B can be served back to A as a box from B, and under any project id, since
  the box binds neither direction nor project. A then recovers a key it sent itself.
- **Static-static Diffie-Hellman**: no forward secrecy, and key-compromise impersonation (anyone
  holding the recipient's private key can forge a box from any sender to that recipient).
- **Small-order public keys are refused** before boxing or opening (`has_small_order` in
  [`crypto.rs`](packages/ui/src/crypto.rs)): a key P for which [8]P is the identity, which the
  Montgomery ladder of `curve25519-dalek` returns as u = 0. This covers every encoding on
  libsodium's list, including those with the ignored top bit set, which a test checks. Such a key
  would yield one of at most 8 shared secrets and leak the sender's reduced private scalar mod 8.
  Other points on the twist are not refused: X25519 maps them to a shared secret on the twist, which `crypto_box`
  uses as is. Serving one is a form of the key substitution of 9.2, and fingerprints catch it the
  same way. Clients up to mobile 3.2.7 and web v1.24.3 checked only the length.
- Before showing Accept, the client checks that the unboxed key decrypts the project row, when the
  server returns one (`opened_invitation` in [`friends_service.rs`](packages/ui/src/friends/friends_service.rs)).
  Accepting stores the key through `adopt_project_key` (section 6). The project row is served by
  the server, so this check catches a sender's mistake, not a hostile server.
- **Key replacement through a forged invitation.** To make a device use a key K' of its choosing
  for a project P, an active server must box K' under a key whose private half it holds, and serve a
  project row of P encrypted under K'. If the user taps Accept, the device stores K' as P's key. From
  then on it shows the server's version of P and encrypts its new rows of P under K', which the
  server reads and the other members drop as unreadable. Rows written under the genuine key stay
  unreadable to the server. On current clients this needs both of:
  - a friend's public key replaced by the server's in the recipient's friends list, which a comparison
    of codes (9.2) detects, since a box opens only with a friend's key; and
  - a project the device holds no key for, or holds a key it has never seen open (section 6), and
    in that second case a separate yes after Accept: the device asks whether to replace the key it
    holds, naming the project, and Cancel keeps it. A held key that has opened P is replaced only
    by a key whose served row names it in `rotatedFrom` (8.5), which the server cannot build
    without a holder of that key.

  Clients up to mobile 3.2.7 and web v1.24.3 opened a box with whatever sender key the server served
  and checked a candidate against the served row only, so on them a server could replace a held,
  genuine key with no substitution that codes would show. Clients built before 2026-11-06 replaced
  a held key that had never opened P without asking. Accepting an invitation for a project whose
  held key is kept, by the rule or by Cancel, leaves that key in place; the invitation is deleted
  all the same.
- The client uses its own private key only if the server's copy of its public key matches it
  (`my_private_key`). This stops a server from giving two accounts each other's keys so that their
  codes match.

### 9.2 Fingerprints

An active server can serve a public key it controls in place of a friend's and read the box. To
detect this, each friend's code is shown next to them and the account's own code at the top. Two
people compare them out of band (`key_fingerprint`):

```
code = base32(SHA-256(public_key)[0..10])     // 80 bits, RFC 4648 alphabet A-Z 2-7, most significant bit first,
                                              // 16 characters in four groups of four
```

Faking a code means finding an X25519 key whose code matches a given one: a preimage search on an
80-bit truncation of SHA-256, about 2^80 work against one target, about 2^80 / N against any one
of N targets. A code over both keys was rejected, because the server picks both substitutes,
which turns forging into a birthday search at half the bits.

Each device pins a friend's key the first time the friends list serves one
(`LocalStorageState.friend_keys`, `pin_keys` in
[`friends_service.rs`](packages/ui/src/friends/friends_service.rs)), and the friend's ML-KEM key
the same way (`friend_kem_keys`); a change of either takes that key off and flags the friend. A keypair changes only through
a compromise rotation (7.5), and only a verified account can log in to become a friend **[Not
independently verified]**, so a changed key is either that or a substitution. A key that differs
from the pin is taken off that friend on this device: nothing is boxed to it, no box opens with it,
and the row says the code changed and shows the new one. Once the two people have compared it out
of band, "The code matches" pins the new key (`trust_key`); removing the friend and adding them
again does the same. Limits: the pin is per device and is lost when the device is erased; a key
substituted before a device first sees the friend is pinned as is, which only a code comparison
catches; and there is no "verified" state. Codes cover the X25519 keys in the friends list: the
only keys a received box opens with (9.1), and one of the two a hybrid box is sealed to. They do
not cover the ML-KEM key. One substituted before first sight, or pinned through "The code matches"
after a change, leaves that friend's invitations at X25519 strength, never below the classical box.
A code over both keys stays rejected: beyond the birthday argument above, every friend's code would
change when their ML-KEM key first appears, invalidating every comparison already made.

### 9.3 Friend-request labels

When sending a friend request, the client stores a label (the address typed, or the participant's
name) under the account key, as a JSON string padded within the cap (5.3), so it can never open as
a bare key fragment (section 7.3). Labels written by clients before 3.2.6, and by later clients up
to mobile 3.2.7 and web v1.24.3 while the server reported `typed_wraps` false, are bare text,
unpadded.

## 10. Recurring rules

A rule's payload holds a random `seed` (UUID v4). Each occurrence it materialises is an expense
whose `client_op_id` is

```
id = SHA-256(seed_bytes || "YYYY-MM-DD")[0..16], with the UUID version nibble set to 8 and the variant bits to 10
```

([`recurring/schedule.rs`](packages/ui/src/recurring/schedule.rs) `occurrence_id`). The id is
deterministic, so that a unique index refuses a second write of the same occurrence from two
devices. Without the seed, the server cannot test candidate dates against stored ids.

A materialisation request (`MaterializeRecurringRequest`) names the rule and carries its new
occurrences and `due_through`, the date up to which they are written, in clear. The server therefore
links each occurrence expense to its rule, which the `recurring_id` inside the expense ciphertext
does not hide, and the number of occurrences written per call over time reveals the rule's cadence.

## 11. On the device

- **Keys at rest are not encrypted.** Project keys and the account key sit in the local store as
  base64url, next to the cache of encrypted rows: `localStorage` on the web, a JSON file in the app's
  private directory on mobile ([`local_storage.rs`](packages/ui/src/common/local_storage.rs)).
  Anyone who can copy that storage can read every project the device holds, and so can a browser
  extension with access to counted.fr. The creator secrets (8.4) and, after a rotation, each
  project's previous keys (8.5) sit there in clear too.
- The mobile store is excluded from backups: on Android from cloud backup and device-to-device
  transfer, through `allowBackup="false"` and [`data_extraction_rules.xml`](packages/mobile/android-res/xml/data_extraction_rules.xml)
  (patched into the manifest by [`android.mk`](packages/mobile/android.mk)); on iOS from iCloud and
  computer backups, with `NSURLIsExcludedFromBackupKey` on the data directory
  ([`main.rs`](packages/mobile/src/main.rs)).
- Signing out removes the account key. Project keys and caches stay, unless the user chooses
  "Sign out and erase this device".
- **Decryption cache.** `decrypt` keeps the plaintexts it has opened in memory, keyed by
  `SHA-256(key || iv || ":" || ct)`, where `iv` and `ct` are the base64 strings. Only plaintext that
  passed the AEAD check is stored. The cache is emptied at logout (`forget_decrypted`) and when it
  reaches 30,000 entries. `encrypt` never fills it.
- **No zeroisation.** The app's keys are plain `[u8; 32]` values and are never wiped. The
  dependencies wipe their own internal copies on drop: the `chacha20poly1305` cipher's key,
  `crypto_box`'s reduced scalar, shared secret and box key, `crypto_secretbox`'s key. `argon2` is
  built without its `zeroize` feature, so its 64 MiB working memory is not wiped. They are persisted as
  strings and, on the web, pass through the JavaScript heap, so wiping the wasm copy would not
  remove them. On the web, the Content Security Policy is the control between an injected script
  and the keys.

## 12. What the server learns

### 12.1 From the ciphertext

XChaCha20 is a stream cipher, so `len(ciphertext) = len(plaintext) + 16`, exactly.

- **Padded blobs** reveal their bucket: 256, 512, 1024... bytes of JSON. A project with many budgets
  can land a bucket above an ordinary group's.
- **Blobs padded within a cap** reveal their bucket, except that the top bucket runs up to the cap
  (5.3): preferences (which grow with the groups counted in personal tracking), payment methods and
  typed friend labels. Written by clients up to mobile 3.2.7 and web v1.24.3, and not saved since,
  they reveal their exact length: for payment methods, an IBAN's length is fixed per country, so one
  shared IBAN hints at the country of the bank account. Bare-text friend labels from older clients are not
  padded: they give the exact length of an address or name.
- Inside a bucket, optional fields still shift the total, which can move a payload to the next
  bucket: an expense with a description or category, or one entered in a foreign currency (three
  extra keys).
- An expense payload names each payer and debtor as a share of about 40 bytes of JSON, so its bucket
  bounds how many people take part in it.
- Rows written before padding was introduced, and not saved since, keep their exact length.

A length gives no amount, name or currency directly. It narrows them: name and label lengths, share
counts, the presence of optional fields, the exact length of legacy unpadded rows.

### 12.2 In clear

Ids, timestamps, memberships, account emails, the friend graph, invitations, push tokens, the
size and timing of every write, the operation each request performs (its route: create, edit or
delete, and of what), which projects one device syncs together, and, through
`EditableProject.notify_closed`, that an edit closed a project. The labelled list is
[SECURITY.md](SECURITY.md) section 5.1. The request types in
[`shared/src/lib.rs`](packages/shared/src/lib.rs) show every field a client sends: whatever is not an
`EncryptedPair` reaches the server in clear. The public route list is in the `*_controller.rs` files
of [`packages/api/src`](packages/api/src).

The key lifecycle adds, in clear: each account's master-key epoch, whether it has a recovery key
and when it was set or is being replaced, when its password last changed, and that and when it was
reset or rotated its keys after a compromise; its ML-KEM public key; each project's key epoch and
earlier verifiers, which tell a request with an older key from one with none (8.1); which accounts
administer each project, when each account joined it, and each promotion and succession; for each
rotation, which account rotated, which memberships it removed and which accounts got a box, so who
stays and who was removed; which members wrote a directory entry, and so roughly which client they
run; co-members' account ids and public keys, served to admins; the ids of rows set aside at a
rotation, for 30 days; which projects were created without an account by a key-lifecycle client
(their creator verifier), and which account claims the role with it; and, through Apple's push
service, that one of an account's projects changed its key.

A Tricount import is fetched by the server on the client's behalf and passes through it **in
clear**, before the client encrypts it.

## 13. Server-side cryptography

**[Not independently verified]**. This explains the dependencies a reviewer will find in
[`packages/api/Cargo.toml`](packages/api/Cargo.toml) under the `server` feature.

- Login: `SHA-256(proof)` compared in constant time (`subtle`). For an address with no verified account,
  `login-salt` answers `HMAC-SHA256(pepper, email)` truncated to 16 bytes, so that the answer does
  not reveal whether the account exists (`hmac`).
- Friend requests by email store `HMAC-SHA256(pepper, "friend-email\0" || origin || "\0" || email)`
  of the lowercased address, not the address. Requests written before this keyed hash existed hold
  an unkeyed SHA-256 of the address until that address next logs in or verifies, when they are
  rehashed (unless the requester already holds a keyed request to that address), or until a daily
  job deletes every request not accepted within 30 days.
- Sessions: the server stores the first 16 bytes of SHA-256 of the cookie; verification tokens and
  password-reset links are stored as SHA-256.
- Key lifecycle: a presented project token is hashed with SHA-256 and compared with the project's
  verifier and its previous verifiers (8.1), the recovery `auth` with `recovery_verifier` (7.5), and
  the creator secret with `creator_verifier` (8.4).
- Push: Web Push payload encryption (RFC 8291: ECDH P-256, HKDF, AES-128-GCM; `hkdf`, `aes-gcm`,
  `ring`) and ES256 JWTs for APNs and VAPID (`ring`). A push carries only which kind of event
  happened.
- Tricount import: an RSA key pair that the Tricount API requires to register a client (`rsa`).

## 14. Known limitations

Each entry is short; the detail is in the section it cites. They are grouped by what would change
them.

### 14.1 Accepted by design

- **Members are not authenticated** (section 2). Any key holder, signed in or not, can write or edit
  any row under any participant's name: the routes take no session (the `*_controller.rs` files),
  and the author, payers and actor are unsigned payload fields. Claiming a participant does not bind
  rows to the claimant.
- **The project id is a capability** (section 2) on a project without a verifier, and on every
  project until the server enforces project tokens (8.1): a UUID holder can read the ciphertext and
  delete the rows.
- **No forward secrecy**: a project key changes only at a rotation and the master key only at a
  compromise rotation; classical invitation boxes are static-static, and hybrid invitation and
  rotation boxes use an ephemeral sender share but the recipient's static keys.
- **Roles are in clear and enforced by the server only** (8.4).
- **A rotation does not take back what a former member read or saved** (8.5).
- **Escrow is mandatory** for account holders (7.4): the database and a guessed password reveal
  every project key the account holds.
- **Keys at rest are not encrypted** (section 11), and the account key stays on the device for the
  session.

### 14.2 Fixed by format v2 (section 15)

- **No associated data** (5.1): rows within a project can be duplicated, swapped or replayed. v2
  binds each ciphertext to its slot, and the mutable ones to a revision (15.4). A device that has
  never seen the newer revision, and a row left out, stay undetected (15.7).
- **Account-key blobs are interchangeable** (7.3): untyped wraps, display name, preferences, payment
  methods and friend labels can be served in another's slot. v2 leaves the same first-sight gap as
  typed wraps: a device that has never opened the account flag accepts v1 (15.5, 15.7).
- **The AEADs are not key-committing** (5.1), while key adoption, escrow and invitations treat "opens
  under K" as proof of K. Fixed only if the commitment of 15.8 is adopted.

### 14.3 Open

- **Projects without an admin cannot change their key** (8.4): one created without an account
  before the key lifecycle, or left with no key-holding account member, stays exposed to a leaked
  key or a former member for as long as it exists.
- **A former member colluding with the server** can get a key of their own adopted by a device that
  has not switched yet, or boxed to through a forged directory entry (8.5, 8.6). Key commitment
  (15.8) would not change this: the forged row really opens under the forged key.
- **Anonymous members have no recovery**: a lost device leaves only the share link, and after a
  rotation only the new link reaches them.
- **A friend key substituted before a device first sees that friend** is pinned as is (9.2); only a
  code comparison catches it, and through it a forged invitation can make the device adopt a key the
  server chose for a project it holds no key for, or, if its user confirms the switch, one it holds
  an unproven key for (9.1).
- **Classical invitation boxes fall to a quantum adversary** (3.1), and with them every project key
  sent in one, until that project rotates; they are written until the 3.2.10 floor. Hybrid
  invitation boxes and rotation boxes do not (9.1, 8.6). After that floor, admins of projects that
  ever received a classical invitation should rotate them.
- **The hybrid floor is the server's word** (9.1): a server can withhold
  `Account.hybrid_invitations` from a fresh install, an erased device or a web session, which then
  writes classical boxes the server stores, and recipients still open them. A build released after
  the floor will write and accept only the hybrid box, whatever the server reports.
- **Invitations carry no key epoch** (9.1): a server can serve an old invitation again after the
  project rotated, and a device holding no key for that project adopts the old key, as with a
  classical invitation or an old share link.
- **A friend's ML-KEM key is not covered by codes** (9.2): one substituted before first sight
  leaves that friend's hybrid invitations at X25519 strength.
- **Web code delivery** (section 2): a server can serve altered JavaScript to one visitor without
  detection. No shipping browser can refuse code that was not publicly logged; the WAICT drafts
  would let one, and only an opt-in extension does today. What the build does close: the HTML
  document is the one root of trust. Every script carries `integrity` (sha384), the loader fetches
  the wasm with one, and `Integrity-Policy: blocked-destinations=(script)` makes the browser refuse
  any script without integrity metadata, so altered code needs an altered document. The same
  server renders that document per request, so this narrows what a verifier must check; it does
  not stop the server.

### 14.4 Legacy clients and legacy data

- **Mobile installs up to 3.2.7 negotiate X25519 alone over TLS** (3.1): a recorded login can be
  broken later to start the offline password search.
- **Clients up to mobile 3.2.7 and web v1.24.3** open an invitation with any key the server serves,
  check a candidate key against the served row only, and accept small-order public keys (6, 9.1).
  The server's version floor refuses old mobile builds, but the attacker here is the server, which
  need not apply it: an install that never updates stays exposed. On the web a hostile server can
  serve an old build anyway (14.1).
- **Data written by older clients keeps its old form**: participant links in clear until the project
  is reopened (section 2), untyped wraps until re-wrapped (7.3), unpadded account blobs and
  bare-text friend labels until saved again (5.3, 9.3), and passwords of 8 to 11 bytes, or not
  NFC-normalised, until changed (section 4).
- **Clients before the key lifecycle** cannot follow a key rotation, open a master-key wrap or send
  project tokens. The server keeps all of it off until it refuses them (section 1).

### 14.5 Assurance gaps

- **Fuzzing is recent and short**: two hours of one CPU so far (5.5).
- **Constant time is not measured** for the compiled ciphers (section 3).
- **The ML-KEM implementation is unaudited**: `ml-kem` 0.3.4's README says it "has never been
  independently audited". The combiner of 8.6 bounds what a flaw in it alone would cost: the X25519
  half still has to break for a box to open.
- **Server behaviour cannot be checked** from outside (section 1).
- **No independent audit** ([SECURITY.md](SECURITY.md) section 6).

## 15. Format v2 (draft, not built)

v2 binds every ciphertext to the slot it belongs to among the blobs under the same key, so that a
moved or swapped blob fails the AEAD check. It makes rows refer to each other by ids the
ciphertext binds, and adds a revision counter the server enforces. The symmetric cipher does not
change.

### 15.1 Marker

`ct` and `iv` both begin with `v2:`. `:` is outside the standard base64 alphabet, so a v2 pair can
never be read as v1: a v1 client fails to base64-decode it (`ct` first) rather than misreading it.

### 15.2 Associated data

```
AAD      = field(label) || field(id_1) || ... || field(id_n) || field(rev)
field(x) = u32_be(len(x)) || x
label    = ASCII "counted/v2/" || slot
rev      = u64_be                                  // only in the slots listed in 15.4
```

UUIDs are their 16 bytes in RFC 9562 order. Every field is length-prefixed and the label comes
first, so the encoding is injective across all slots: the label fixes the number and types of the
fields that follow.

**Which ids.** The key already scopes a blob to its project or account: a blob moved to another
project or account fails anyway (section 5.1). The AAD therefore binds only what tells apart the
blobs under one key, and only ids the client knows **before** it encrypts. Project and account ids
are assigned by the server after the first encryption (the project row, the display name and the
private-key wrap are encrypted before they exist), so they are not bound where the key already
scopes them.

| Slot | Key | Ids after the label |
| --- | --- | --- |
| `project` | project | none (one project row per key) |
| `participant`, `expense`, `rule`, `history` | project | the row's entity id |
| `claim-label`, `payment-copy` | project | the claimed participant's entity id, the claimant's account id (known at claim time; the project key does not scope accounts, and without it a server could serve a former claimant's label or IBAN as the current one's) |
| `escrow` | account | project id (the project exists when its key is escrowed) |
| `account-sk`, `display-name`, `preferences`, `payment-methods` | account | none (one blob of each per account) |
| `friend-label` | account | a request id the client draws |

The entity id is a UUID v4 the client draws before its first encryption of the row. Expenses have
an optional `client_op_id`, set on every add that goes through the client's write queue (all of them since the native-feel build, older builds only offline) and on recurring occurrences; v2
makes it mandatory. The other tables need a new column. The offline queue (local only) and
`PaymentPayload` (no longer written) get no slot.

**References between rows.** Today an expense names its payers, debtors and author, and a history
entry its actor and entity, by server row id. In v2 every such reference is the referenced row's
entity id, and server ids become transport only. Without this, swapping two participant rows
together with their entity ids would still reassign every share.

**Invitation boxes cannot take AAD** (section 9.1). In v2:

- The invitation names its sender account, and the recipient opens the box only with the public key
  of that account from its own friends list. Current clients already refuse any key that is not a
  friend's (9.1), but accept any friend's.
- The plaintext is a typed object,
  `{"use":"invite","project":"<uuid>","from":"<account uuid>","to":"<account uuid>","key":"<base64url>"}`.
  It opens only if `to` is the recipient's own account, `project` the invitation's project and
  `from` the friend whose key opened it. This defeats reflection (9.1). Built ahead of v2 for the
  hybrid box, from floor 3.2.10.
- An invitation for a project the device already holds is shown as "already a member" and never
  adopted.

### 15.3 Key adoption

Built ahead of v2, as it does not depend on the format: a held key that has opened the project on
the device is never replaced (section 6), except under the rotation rule (8.5). A held key that
never has is replaced by a candidate that opens the cached row, or the served row when nothing is
cached, only once the user confirms the switch (2026-11-06), which keeps a wrong link correctable
without letting a served row decide; this answers the question formerly in 15.8. The server
learns nothing new from it: the question is local, and the candidate's claim token it sees on the
check is the one it already saw when a held key refused a new link.

### 15.4 Revision

`rev` is present in the `project`, `participant`, `expense`, `rule`, `claim-label`,
`payment-copy`, `preferences` and `payment-methods` slots, and absent from `history`, `escrow`,
`account-sk`, `display-name` and `friend-label`, whose content never changes once written. It is an unsigned 64-bit
integer, **in clear** on the row and bound in the AAD, and a row is created with `rev` = 1. The
server accepts a write only if its `rev` is the stored one plus one, the same compare-and-swap that
`expected_version` and `expected_iv` already provide; a concurrent writer gets a conflict and retries
from the newer row. A device keeps the highest `rev` it has opened per (slot, ids) and treats a
lower one as unreadable.

Without the server's check, a stale writer would produce a lower `rev` and other devices would drop
its genuine edit, and two writers at the same `rev` would fork. With it, an honest server prevents
forks. A hostile server can still serve a lower `rev` (refused by a device that has seen the higher
one), or accept two writes at the same `rev` and serve each to different devices; a device accepts an
equal `rev`.

### 15.5 Rollout

1. Clients that read v1 and v2 and write v1 ship, and the minimum client version the server
   accepts is raised to them.
2. Clients from a later version write v2 unconditionally: no server switch, which would reopen the
   downgrade of section 7.3. Step 2 starts only once the minimum version is a step-1 build.
3. Once the minimum version is a v2 writer, projects are converted in batches (conversion is capped
   per request today), each a compare-and-swap on `data_version`, with the project row written last
   and carrying a `converted` flag. Account blobs are re-encrypted at login, `preferences` last,
   carrying the account flag in its `extra` map (the display name is a bare JSON string and cannot
   carry one). From the first conversion, the server refuses v1 writes to a converted project (it
   sees the marker).
4. A device that has opened a project's flag, or the account flag, records it locally and refuses v1
   for it from then on.

### 15.6 What the server newly learns

- The entity ids: random, one per row, linking nothing the row id does not.
- `rev`, per row: how many times the row was written, which the server already observes as write
  requests.
- The marker: which rows are converted.

Each new plaintext column gets its row in the list of what the server sees before it ships.

### 15.7 What v2 does not fix

- **Deletion and withholding.** Leaving a row out stays undetectable without a per-project manifest
  authenticated under the project key.
- **Rollback on first sight.** The `rev` rule protects only a device that has already seen the newer
  revision. A fresh device, or one whose store was cleared, accepts any revision.
- **The conversion flag is itself a payload.** A server that keeps serving a project's last v1 rows
  to a device that has never seen the converted project row leaves that device reading v1.
- **Members.** A key holder can still write any slot.
- **Key commitment**, unless the proposal below is adopted.

### 15.8 Open questions for reviewers

- **Manifest instead of per-row `rev`.** A per-project manifest MACed under the project key, with a
  monotonic epoch pinned on each device, would also cover deletion. Is it worth its cost over 15.4?
- **Invitation construction, built.** The hybrid box of 9.1: the box of 8.6 with the sender's
  static X25519 share as a third combiner input and `pk_s` in the info, the typed plaintext of
  15.2, opened only with a friend's pinned key. Is the sender binding right: `pk_s` in the info
  rather than the ikm, and the order `ss_k || ss_e || ss_s`? Sender authentication stays classical
  until invitations are signed (format v2). It does nothing for the classical boxes already stored
  (3.1): only a rotation of those projects closes that.
- **Key commitment.** Store `commit = SHA-256("counted/v2/commit" || K || nonce)` in each pair and
  check it before decrypting, so that "opens under K" proves K?
- **Bound project and account ids.** Client-drawn project and account UUIDs would let the AAD bind
  them too. Does that add anything once the key scopes them?
- **An unproven held key, answered (2026-11-06).** A held key the device has never seen open the
  project is kept, cached row or not, and replaced only when the user confirms the new key
  (15.3, section 6). That closes the silent case of 9.1, and a wrong link stays correctable at the
  cost of one question.

## 16. Test vectors

All values below are pinned by known-answer tests in [`crypto.rs`](packages/ui/src/crypto.rs),
unless another file is named. The keys are test keys; the bytes are in the format production data
is stored in, so a change that breaks a vector breaks stored data. Byte arrays are written `[b; n]`
for n copies of byte b. Stored blobs are given as the pair that opens, since every nonce is
random; a derivation or a plaintext format is given as its exact output. There are no vectors for
v2, which is not built.

**v1 pair** (`stored_ciphertext_still_opens`). Key `[0x07; 32]`. The plaintext is the JSON string
`"Weekend trip"`, quotes included (14 bytes).

```
ct = /C2rdoVCtdYK3ZAej3L0eJOmwutfGEWl1v46733d
iv = CCqHN62xmXAe2DC7GSjfpPTREZeuOHm2
```

**Untyped wrap** (`a_stored_wrap_still_opens`). Account key `[0x07; 32]`. It opens as the private key
`[0x09; 32]`.

```
ct = RveOpblLHkKgFid5xZ0cHtR5kaqWQvHzqCuozAKfXoW+/sNVvKQR/vYp1hOD1iR7QwCbBKmouEXGclU=
iv = 8hX+KTHlA5Lnv0BUext3cE2sra5UKNw1
```

**Box** (`a_stored_box_still_opens`). Sender secret key `[0x01; 32]`, recipient secret key
`[0x02; 32]`. It opens as project key `[0x07; 32]`.

```
ct = D9WCNfn2fqB0KJnUUYlu62jNE+MhMBhBgXxPC38URyIYssGmA5YzUaKbcceqhN1Y4IsojuLVKlXcCjw=
iv = 3dibiCBYV3+7yGCn8BBp7Z/4zbNS5lki
```

**Derivations** (`derivations_are_unchanged`), outputs in standard base64:

```
claim_token([0x07; 32])                                   = 1KxVDogT+QG43nuv4CJUZaD2XTrcSRfgRtHfxV3loCg=
derive_account_key_v1("password", [0x11; 16])             = /SPWqp3PDq/+ghu8SN2qps4tQD8ZZkfH/Isz8fQXok8=
derive_login_proof(2, "password", [0x11; 16])             = DUkKmgemaxbhVxsKh20fRqjNBC/eb8WqiyQPJFKT7ic=
```

**Fingerprint** (`a_fingerprint_is_unchanged`): `key_fingerprint([0x00; 32]) = MZUH VLPY MK6X O3EP`.

**`rotatedFrom`** (`the_known_answer_for_rotated_from_is_pinned` in
[`rotation.rs`](packages/ui/src/rotation.rs)):
`rotated_from_hash([0x07; 32]) = qlxSNkAhIGk5XXSR7+HWwqugI9v848gRMGdmdZt52UA=`.

**Recovery derivation** (`the_derivation_matches_its_known_answer` in
[`recovery_key.rs`](packages/ui/src/auth/recovery_key.rs)). Recovery key `[0x01; 32]`, account
`00000000-0000-0000-0000-000000000002`:

```
rk_key = 7WPuQhHnKOkTShIg+MR9dnfOt/h2/B/EHoEvF3NzZOQ=
auth   = oWpyXzGTisnU5tMSLfOwkR9anADMPZlke6oqzYeNlUw=
```

**Hybrid box** (`the_construction_matches_its_known_answer` in
[`hybrid.rs`](packages/ui/src/hybrid.rs)). Recipient ML-KEM seed `[0x07; 64]` and X25519 secret
`[0x09; 32]`; context `counted-kat`, plaintext `kat plaintext`; ephemeral secret `[0x03; 32]`,
encapsulation message `[0x04; 32]`, nonce `[0x05; 24]`. The box is pinned by its digest:

```
SHA-256(ephemeral || kemCt || nonce || ct) = EsGlmnmZZnSBMZIDE+tgmIyLtUnNt0KDn0XKwQngE4g=
```

**Authenticated hybrid box** (`the_authenticated_construction_matches_its_known_answer` in
[`hybrid.rs`](packages/ui/src/hybrid.rs)). As above, plus sender X25519 secret `[0x02; 32]`:

```
SHA-256(ephemeral || kemCt || nonce || ct) = En87WjuyLSiVoWSs++KCKQsjpihTgLRFKCMUpJgKdcc=
```

**Typed wraps** (`stored_typed_wraps_still_open`). Wrapping key `[0x07; 32]` in every case; project
`00000000-0000-0000-0000-000000000001`, account `00000000-0000-0000-0000-000000000002`. Each pair
opens as the key `[0x09; 32]`, through its own function only: the escrow row does not open as a
private key, and the `amk` wrap does not open for epoch 2.

```
escrow       ct = 4EgoMjw6glRbGcADHVhU/+gkku6+Ro3JrxFUK4ecPoK87ID40NRoeEdXiUN6rTVR19oKvdoR+/6BRCthI2t7AfKMthCFwp4uyCx42Z56WHj00aPcMZNP7T0dj/XUgjGesMRunQvZ4Utq5+LxnRoBsPwmb7nCn2t6zrEGMa1BOB1kTX1llg==
             iv = eFBoH53JsWeknt68EbKTgQIccez8Or4J
account-sk   ct = +MWeqSq7YI3bZMMAl+ZXBjKMk0aW+M+MYBBtt3dFPiKtB2XfqD+ZPLNfdv55BsVTeMihStYuaM66vKeRcCpbwdQTArm7IRzgpqLCgBVb2oet8iva1oYKcg==
             iv = FsvOOstyX/DDggzcFwNVBpUDFtLPcPSI
amk, epoch 1 ct = xb/BOCpqw2zNvrInTfvUJN770eHJN7SE98Cz01p0zsPkrd48l/t4CMlPM8//LhXJzfOAzz5ag3fY+Nkla1CB9MwsQHyRxdcOnBQbQzdBgYXU1I0680R05ks0btwlDTWSBeAjGwWFgO/AKqghWLGVYGx83BDmvLUqsL7wnS9n6fKs1AceHs8UcO0B484=
             iv = j9A9HhA57ntuS+Sx2HGUo9NoU44KFcsg
amk-recovery ct = /K2Bfcg8Dm4DBQsep1jzC8TsTQkV+qRQydmDeU20SpjdSObcaNBqTlNrEFJ3jAlg7wBqUPDoDzKcu7fXYjYEYL3L+OTPKSVbpr+74vU1kYflNDcLjbgJLurDvlHakh4Bp6t4D7F0EtsDN3QmyjM6M0qnOWIkO/ua/npw8h1Fr5/wmkX7OeSI2Qukfg==
             iv = b+9DTnEXIF3segtubW/ohw6O1nVTqp9n
```

**ML-KEM seed wrap** (same test). Master key `[0x07; 32]`; opens as the seed `[0x07; 64]`.

```
ct = cyUY9NDec3e+y7oXyzeyK6JXGVngewz8lRc5e+BHU/BEd9izvss0ctSVod321pZ9M6ctRzsId73nyD06x81YJ1jx1k3tn7e7vagHlqpE0WgWGWxRP0Rg54Sf3yQ2Nm8VC5FB9jB169O0622wxq6/FT6BPsB8mxunq+8x8HMKbrVvydU/
iv = R0pZGQ7YYkGUZhQs9cUqzUdBWEndopkT
```

**Member-key entry and rotation plaintext**
(`the_member_key_entry_and_the_rotation_plaintext_are_pinned` in
[`rotation.rs`](packages/ui/src/rotation.rs)). Project key `[0x07; 32]`, account
`00000000-0000-0000-0000-000000000002`, X25519 public key `[0x01; 32]`, encapsulation key
`[0x02; 1184]`. The stored pair (its `ct` is 2,308 characters, in the test) has
`iv = 61ArW+WvBxSCi8QNg3nAOVScEustdYN6` and opens as

```
{"use":"member-key","account":"00000000-0000-0000-0000-000000000002","x25519":"AQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQE=","kem":"AgIC...
```

and `recipient_keys` accepts it for a member served exactly those two keys. The plaintext a rotation
box carries, for project `00000000-0000-0000-0000-000000000001`, that account, epoch 1 and K2 =
`[0x07; 32]`, is exactly

```
{"use":"rotate","project":"00000000-0000-0000-0000-000000000001","to":"00000000-0000-0000-0000-000000000002","key_epoch":1,"key":"BwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwc="}
```

**Padding** (`pad_json_doubles_from_256` in [`shared/src/lib.rs`](packages/shared/src/lib.rs)):
`pad_json("{}")` is `{}` followed by 254 spaces; 256 bytes stay 256, 257 become 512, 3000 become
4096.

**Occurrence id** (`the_occurrence_id_matches_its_known_answer` in
[`recurring/schedule.rs`](packages/ui/src/recurring/schedule.rs)). Seed `[0x07; 16]` as a UUID,
date 2026-01-31: `66a6c464-8843-8e0e-8282-42618503332b`.

## 17. Source map

| Topic | Files |
| --- | --- |
| Primitives, wraps, box, claim token, fingerprint, KDFs, test vectors | [`packages/ui/src/crypto.rs`](packages/ui/src/crypto.rs) |
| `EncryptedPair`, payload structs, `pad_json`, request and response types | [`packages/shared/src/lib.rs`](packages/shared/src/lib.rs) |
| Decrypting rows into what the screens show | [`packages/ui/src/decrypted.rs`](packages/ui/src/decrypted.rs), [`packages/ui/src/expenses/helpers/project_data.rs`](packages/ui/src/expenses/helpers/project_data.rs) |
| Share links | [`packages/ui/src/common/share_link.rs`](packages/ui/src/common/share_link.rs), [`packages/ui/src/expenses/hooks/use_project_store.rs`](packages/ui/src/expenses/hooks/use_project_store.rs) |
| Membership, key adoption, escrow push | [`packages/ui/src/common/membership.rs`](packages/ui/src/common/membership.rs) |
| Escrow pull | [`packages/ui/src/common/account_sync.rs`](packages/ui/src/common/account_sync.rs) |
| Login, account key, keypair | [`packages/ui/src/auth/credentials.rs`](packages/ui/src/auth/credentials.rs) |
| Master key, password change, compromise rotation, recovery key | [`packages/ui/src/auth/account_key.rs`](packages/ui/src/auth/account_key.rs), [`packages/ui/src/auth/account_rotation.rs`](packages/ui/src/auth/account_rotation.rs), [`packages/ui/src/auth/recovery_key.rs`](packages/ui/src/auth/recovery_key.rs) |
| Hybrid X25519 + ML-KEM-768 box | [`packages/ui/src/hybrid.rs`](packages/ui/src/hybrid.rs) |
| Project token | [`packages/ui/src/common/project_token.rs`](packages/ui/src/common/project_token.rs) |
| Proof of creator | [`packages/ui/src/common/creator_proof.rs`](packages/ui/src/common/creator_proof.rs) |
| Key rotation, member-key directory, rotation boxes | [`packages/ui/src/rotation.rs`](packages/ui/src/rotation.rs), [`packages/ui/src/rotation_flow.rs`](packages/ui/src/rotation_flow.rs), [`packages/api/src/projects/rotation_controller.rs`](packages/api/src/projects/rotation_controller.rs) |
| Friends, invitations | [`packages/ui/src/friends/friends_service.rs`](packages/ui/src/friends/friends_service.rs) |
| Payment methods | [`packages/ui/src/payment_methods.rs`](packages/ui/src/payment_methods.rs) |
| Recurring rules | [`packages/ui/src/recurring/`](packages/ui/src/recurring) |
| Hostile-input property tests and fuzz targets | [`packages/ui/src/common/hostile.rs`](packages/ui/src/common/hostile.rs), [`packages/ui/src/fuzz.rs`](packages/ui/src/fuzz.rs), [`packages/ui/fuzz`](packages/ui/fuzz) |
| Local store, offline queue | [`packages/ui/src/common/local_storage.rs`](packages/ui/src/common/local_storage.rs), [`packages/ui/src/common/offline_queue.rs`](packages/ui/src/common/offline_queue.rs) |
| API routes | `packages/api/src/**/*_controller.rs` |

## 18. Reporting

Report a flaw in this design or its implementation as described in [SECURITY.md](SECURITY.md).
