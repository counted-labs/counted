# Counted - client source

The web and mobile clients of [counted.fr](https://counted.fr), a zero-knowledge expense-splitting
app: every name, amount and project is encrypted on your device, and the server stores ciphertext
it cannot read.

Use it in the browser at [counted.fr](https://counted.fr), or install the app from the
[App Store](https://apps.apple.com/app/id6772807915),
[Google Play](https://play.google.com/store/apps/details?id=fr.counted.app) or
[F-Droid](https://f-droid.org/packages/fr.counted.app/).

This repository exists so that claim can be **checked rather than believed**. The WASM bundle
counted.fr serves is built from exactly these files, reproducibly, and its hashes are published at
[counted.fr/verify](https://counted.fr/verify) and at `https://counted.fr/SHA256SUMS.txt`.

## What is here, what is not

| Crate | Role |
| --- | --- |
| `packages/shared` | DTOs, the wire format, the encryption helpers, limits |
| `packages/ui` | Every screen, component and hook - Dioxus, compiled to WASM and into the phone apps |
| `packages/api` | The API contract: each endpoint's route, method, request and response types. Bodies delegate to `api::server`, which is **not published** |
| `packages/web` | The browser entry point |
| `packages/mobile`, `packages/ocr` | The Android/iOS apps and the on-device receipt OCR |
| `vendor/tao` | tao 0.34.8 with one upstream fix backported (see its README) |
| `vendor/jni-macros` | jni-macros 0.22.4 with its generated code made deterministic, for reproducible APKs |

The backend - `packages/api/src/server/`, the database, the deployment - stays private. The
client is what handles your keys and your plaintext, and it is the part you can audit and rebuild.
How it encrypts, what the server can still learn, and the known limitations:
[SPEC.md](SPEC.md). To review it: [REVIEW.md](REVIEW.md).

## Rebuilding what counted.fr serves

Every release is a tag `vX.Y.Z`, matching the version shown in the app's settings. With Docker:

```sh
git checkout vX.Y.Z
docker buildx build --platform linux/amd64 -f Dockerfile.client --target sums --output type=local,dest=out .
curl -s https://counted.fr/SHA256SUMS.txt | grep -v '  server$' | diff - out/SHA256SUMS.txt && echo identical
```

An empty diff means every file the site serves - the `.wasm`, the JS glue, the fonts, the CSS,
their brotli siblings - is byte for byte what these sources produce. The `server` line is the
native binary, built from the private module; it is skipped because you cannot rebuild it, and it
never touches your plaintext. The pins that make this deterministic (compiler digest, `dx`
version, `Cargo.lock`) are in `Dockerfile.client` and `rust-toolchain.toml`.

You do not have to take the first rebuild on trust either: every tag is rebuilt the same way on a
GitHub-hosted machine by [`.github/workflows/rebuild.yml`](.github/workflows/rebuild.yml), which
also checks the signature below. Its runs are public in this repository's
[Actions tab](https://github.com/counted-labs/counted/actions/workflows/rebuild.yml).

A rebuild says what a release contains, not that counted.fr serves it. Every six hours
[`.github/workflows/monitor.yml`](.github/workflows/monitor.yml) checks that too: the list
counted.fr serves must be a signed release's, every file in it must match, and its pages may run
no code outside it. It cannot see a swap aimed at one visitor. Runs:
[Actions tab](https://github.com/counted-labs/counted/actions/workflows/monitor.yml).

## Checking the signature

Each release carries `SHA256SUMS.txt.sigstore.json`: a Sigstore keyless signature of its
`SHA256SUMS.txt` by the private build's `main` pipeline, recorded in the public Rekor log. With
[cosign](https://github.com/sigstore/cosign), from the release's two files:

```sh
cosign verify-blob --bundle SHA256SUMS.txt.sigstore.json \
  --certificate-identity "https://gitlab.com/jbosi/counted//.gitlab-ci.yml@refs/heads/main" \
  --certificate-oidc-issuer https://gitlab.com SHA256SUMS.txt
curl -s https://counted.fr/SHA256SUMS.txt | cmp - SHA256SUMS.txt && echo served == signed
```

It proves where the list comes from, not that the code is safe; the rebuild above is that proof.

## The mobile apps

| Store | What can be checked |
| --- | --- |
| [F-Droid](https://f-droid.org/packages/fr.counted.app/) | F-Droid rebuilds each `mobile-vX.Y.Z` tag from this repository and ships our signed APK only if its own build matches it, signature aside. The same APK is attached to the tag's GitHub release |
| Google Play | Google re-signs the app with its own key (certificate below), so a Play install proves where it came from but cannot be compared with a build yet |
| App Store | Each copy is encrypted for the device that installs it (FairPlay); nobody can compare it with a build. Team ID `UK936HFQ24`, bundle ID `fr.counted.app` |

Signing certificate of the F-Droid and GitHub-release APK, package `fr.counted.app`, SHA-256:

```
D8:D4:02:6F:49:6F:3C:21:54:F1:38:E8:17:10:33:2E:A5:28:8A:AA:B8:56:7A:2F:8D:0E:71:3D:E1:D5:93:E8
```

Check an installed copy with [AppVerifier](https://github.com/soupslurpr/AppVerifier), or:

```sh
adb pull "$(adb shell pm path fr.counted.app | head -1 | cut -d: -f2)" counted.apk
apksigner verify --print-certs counted.apk | grep SHA-256
```

Google Play signing certificate, same package, SHA-256:

```
4C:1E:7D:F5:2C:FB:D6:03:14:C2:0B:E4:AA:7E:9E:7D:30:19:1C:50:24:3B:F5:09:1A:21:F4:B1:30:98:A6:DC
```

## Licence

Copyright (C) 2022-2026 Jonathan Bosi.

This program is free software: you can redistribute it and/or modify it under the terms of the
GNU Affero General Public License, **version 3 only** (`AGPL-3.0-only`), as published by the Free
Software Foundation. It is distributed without any warranty; see [LICENSE](LICENSE).
Third-party files keep their own licences: [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).

**Name and logo.** Under section 7(e) of the licence, no trademark rights are granted in the name
"Counted", its logo or the counted.fr domain. The files below carry the logo and store artwork;
they are licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/) instead of
the AGPL, in every published version. That licence grants no trademark rights either: a fork may
reuse the artwork but must not present itself as Counted.

- `packages/mobile/assets/counted*.{png,svg,ico}`, `packages/mobile/assets/play-feature-*.png`
- `packages/mobile/android-res/mipmap-*/ic_launcher*.png`
- `packages/mobile/fastlane/metadata/android/en-US/images/icon.png`
- `packages/web/assets/counted.png`, `packages/desktop/assets/favicon.ico`

**Contributions.** This repository is a one-way mirror, updated once per release. Pull requests
are not merged. Report bugs to [contact@counted.fr](mailto:contact@counted.fr), and security issues as
described in [SECURITY.md](SECURITY.md).
