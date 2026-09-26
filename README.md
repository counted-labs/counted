# Counted — client source

The web and mobile clients of [counted.fr](https://counted.fr), a zero-knowledge expense-splitting
app: every name, amount and project is encrypted on your device, and the server stores ciphertext
it cannot read.

Use it in the browser at [counted.fr](https://counted.fr), or install the app from the
[App Store](https://apps.apple.com/app/id6772807915) or
[Google Play](https://play.google.com/store/apps/details?id=fr.counted.app).

This repository exists so that claim can be **checked rather than believed**. The WASM bundle
counted.fr serves is built from exactly these files, reproducibly, and its hashes are published at
[counted.fr/verify](https://counted.fr/verify) and at `https://counted.fr/SHA256SUMS.txt`.

## What is here, what is not

| Crate | Role |
| --- | --- |
| `packages/shared` | DTOs, the wire format, the encryption helpers, limits |
| `packages/ui` | Every screen, component and hook — Dioxus, compiled to WASM and into the phone apps |
| `packages/api` | The API contract: each endpoint's route, method, request and response types. Bodies delegate to `api::server`, which is **not published** |
| `packages/web` | The browser entry point |
| `packages/mobile`, `packages/ocr` | The Android/iOS apps and the on-device receipt OCR |
| `vendor/tao` | tao 0.34.8 with one upstream fix backported (see its README) |

The backend — `packages/api/src/server/`, the database, the deployment — stays private. The
client is what handles your keys and your plaintext, and it is the part you can audit and rebuild.

## Rebuilding what counted.fr serves

Every release is a tag `vX.Y.Z`, matching the version shown in the app's settings. With Docker:

```sh
git checkout vX.Y.Z
docker buildx build --platform linux/amd64 -f Dockerfile.client --target sums --output type=local,dest=out .
curl -s https://counted.fr/SHA256SUMS.txt | grep -v '  server$' | diff - out/SHA256SUMS.txt && echo identical
```

An empty diff means every file the site serves — the `.wasm`, the JS glue, the fonts, the CSS,
their brotli siblings — is byte for byte what these sources produce. The `server` line is the
native binary, built from the private module; it is skipped because you cannot rebuild it, and it
never touches your plaintext. The pins that make this deterministic (compiler digest, `dx`
version, `Cargo.lock`) are in `Dockerfile.client` and `rust-toolchain.toml`.

## Licence

[AGPL-3.0](LICENSE). Counted is a trademark of its authors; a fork must not present itself as
Counted or use counted.fr.
