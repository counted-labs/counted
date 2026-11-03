# Reviewing Counted's cryptography

We are asking for a review of the client cryptography: the design in [SPEC.md](SPEC.md) and the
code it cites. This is a request for review, not an audit we commissioned; no independent audit
has been done ([SECURITY.md](SECURITY.md) section 6). This page says where your time is best spent
and what we already know.

## Start here

1. [SPEC.md](SPEC.md) sections 1 and 2: what the client claims, the threat model, and which
   statements about the server nobody outside can check.
2. Section 14: the known limitations, grouped by what would close each. You do not need to report
   them again.
3. The section that interests you, and the files its source map row lists (section 17).

Everything runs from one Rust crate, [`packages/ui`](packages/ui), compiled to WebAssembly for
counted.fr and to native code for the Android and iOS apps. What counted.fr serves can be rebuilt
from this repository byte for byte ([README.md](README.md)).

## Where we most want your eyes

1. **The open questions of section 15.8**, on the format v2 draft: a per-project manifest instead of
   per-row revisions, an HKDF-based invitation construction in place of `crypto_box`, key
   commitment, binding project and account ids, and keeping an unproven held key. v2 is not built,
   so a better design costs nothing yet. One of those questions is a choice we made on purpose:
   invitation boxes are still classical X25519 (section 14.3) while rotation boxes are hybrid. The
   hybrid invitation we would build is written out in 15.8, with the sender's static share in the
   combiner and a typed plaintext. We would rather have that construction reviewed before it is
   built than after, so it is not in the code yet.
2. **Key adoption and invitations** (sections 6, 9.1, 9.2): when a device replaces a project key,
   and what an active server can still make it adopt.
3. **Key rotation and the hybrid box** (sections 8.5, 8.6): the switch rule built on
   `rotatedFrom` and the `chain_seen` pin, what a former member colluding with the server can
   still make a device adopt, the member-key directory and how each entry is derived from the
   device's own private halves, and the X25519 + ML-KEM-768 combiner.
4. **Escrow, typed wraps and the master key** (section 7): the downgrade window of 7.3, what a fresh
   device accepts, and the epochs and recovery-key derivation of 7.5.
5. **Reliance on "opens under K"** without key commitment (section 5.1): whether a ciphertext can
   open as a `ProjectPayload` under two keys, and whether any check depends on it.
6. **Everything else in sections 3 to 13**, and any statement in the SPEC that the code does not
   match.

An attack that goes further than section 14 says is in scope, even if the limitation it builds on
is listed.

## Out of scope

- The limitations accepted by design (section 14.1): members are not authenticated, the project id
  is a capability, no forward secrecy, mandatory escrow, unencrypted keys at rest.
- The server, which is closed source. Its behaviour is marked **[Not independently verified]**
  in the SPEC; section 13 lists its cryptography for context.
- Clients older than the current release (section 14.4).
- Denial of service, social engineering, and the publisher's own release pipeline (section 2 says
  what the stores and the reproducible builds do and do not catch).

## What we already check

- Known-answer vectors for every stored format and derivation, pinned by tests (section 16).
- Property tests that feed hostile plaintext to the payload decoders and every page's computations,
  and the same checks under libFuzzer (section 5.5).
- Reproducible builds of the web bundle and the F-Droid APK, and a public monitor comparing what
  counted.fr serves with the signed release ([README.md](README.md)).

## Sending findings

A vulnerability goes through [SECURITY.md](SECURITY.md) section 1, encrypted if you like. Anything
else (answers to section 15.8, a design objection, an inaccuracy in the SPEC) can go to the same
address without the "security" subject. With your permission, we credit reviewers in
SECURITY.md section 6 and say what they reviewed, and we never present a review as an audit.
