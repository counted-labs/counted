/// `option_env!("COUNTED_SERVER_URL")` and `option_env!("COUNTED_STORE_URL")` in main.rs are
/// resolved at compile time, and cargo does not track env vars read that way. Without these lines a
/// rebuild after changing a variable is a no-op: the binary keeps the URL baked into the previous
/// build, so an E2E APK silently talks to production, or an F-Droid APK sends users to Play. See
/// docs/e2e.md and docs/android.md.
fn main() {
    println!("cargo:rerun-if-env-changed=COUNTED_SERVER_URL");
    println!("cargo:rerun-if-env-changed=COUNTED_STORE_URL");
}
