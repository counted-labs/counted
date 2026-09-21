// `route::route` trips clippy's module_inception. Renaming the file is not worth it: `route.rs` is
// referenced by name in DOCUMENTATION.md §8 and in three audit documents, as the home of the
// catch-all route and the `catch_all_does_not_shadow_real_routes` guard.
#[allow(clippy::module_inception)]
mod route;
pub use route::{project_id_of, Route};
