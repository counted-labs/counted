# ui

Every screen, component and hook of the app, shared by the web build (compiled to WASM) and the
Android/iOS apps (native Rust in a WebView). One folder per feature (`projects/`, `expenses/`,
`charts/`, `auth/`, …), plus `common/` for what they share, `crypto.rs` for the client-side
encryption and `i18n.rs` with the Fluent messages in `i18n/`.

Code that differs per target is gated with `cfg(target_arch = "wasm32")` (web) or
`cfg(target_os = "android" | "ios")`, not with Cargo features.
