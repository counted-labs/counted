/// `option_env!("COUNTED_SERVER_URL")` in main.rs is resolved at compile time, and cargo does not
/// track env vars read that way. Without this line a rebuild after changing the variable is a
/// no-op: the binary keeps the URL baked into the previous build, so an E2E APK silently talks to
/// production. See docs/e2e.md.
fn main() {
    println!("cargo:rerun-if-env-changed=COUNTED_SERVER_URL");
}
