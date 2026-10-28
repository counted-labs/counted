# api

The API contract. Each `*_controller.rs` declares one endpoint with `#[get]` / `#[post]` / `#[put]`
/ `#[delete]`: its route, method, request and response types. On the server the macro turns it into
an axum handler; on the client (WASM and the phone apps) into an HTTP call, with the body dropped.

Every body is one line delegating to `crate::server`, which lives in `src/server/`, compiles only
with the `server` feature and is not part of the public mirror. Server-only dependencies (sqlx,
axum, tokio, …) are optional and enabled by that feature.
