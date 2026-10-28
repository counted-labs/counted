# web

The browser entry point and the build artifact. `dx bundle --web --release --package web
--fullstack` produces both the WASM client and the native server binary; `src/main.rs` calls
`dioxus::serve` (server) and the hydrating launch (WASM). Neither target builds by default: check
with `cargo check --package web --features web` or `--features server`.
