//! The API contract: every server function's route, signature and types. Bodies live in `server`,
//! which only the `server` feature compiles — the WASM client never sees them, and the module's
//! directory is absent from the public source mirror (docs/reproducible-builds.md).

pub mod account_projects;
pub mod app_version;
pub mod auth;
pub mod expenses;
pub mod friends;
pub mod fx;
pub mod history;
pub mod payments;
pub mod projects;
pub mod push;
pub mod recurring;
pub mod tricount;
pub mod users;

#[cfg(feature = "server")]
pub mod server;
