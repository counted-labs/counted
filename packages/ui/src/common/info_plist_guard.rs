//! Guards over `packages/mobile/ios/Info.plist`.
//!
//! `docs/ios.md` records four things about that file that "must stay in sync" with something
//! elsewhere, and nothing enforced any of them. Each has a failure mode that is invisible until an
//! iOS build is in someone's hands: the app installs as "Mobile", the launch screen paints black,
//! a share link opens nothing.
//!
//! Lives in `ui` rather than `mobile` for the reason given in `stylesheet_guard.rs` — and because
//! `APP_SCHEME`, one of the two sides being compared, is defined here.

use std::path::PathBuf;

const PLIST: &str = "../mobile/ios/Info.plist";
const DIOXUS_TOML: &str = "../mobile/Dioxus.toml";

fn read(rel: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("guard cannot read {}: {e}", path.display()))
}

/// The `<string>` following a `<key>`. Comment-free by construction: it searches for the key
/// element, so text inside `<!-- … -->` never matches.
fn plist_string(plist: &str, key: &str) -> Option<String> {
    let at = plist.find(&format!("<key>{key}</key>"))?;
    let rest = &plist[at..];
    let open = rest.find("<string>")? + "<string>".len();
    let close = rest[open..].find("</string>")?;
    Some(rest[open..open + close].trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::APP_SCHEME;

    /// Fix 4's guard. dx's template points `UILaunchStoryboardName` at a `LaunchScreen` storyboard
    /// it never generates, so iOS resolves it to nothing and paints black until the webview draws —
    /// and a dangling key is worse than no key, because with neither iOS applies its own default.
    /// See docs/dioxus-issues/mobile-no-native-splash-screen.md.
    #[test]
    fn the_launch_screen_does_not_point_at_a_storyboard_that_is_never_generated() {
        let plist = read(PLIST);
        assert!(
            !plist.contains("<key>UILaunchStoryboardName</key>"),
            "UILaunchStoryboardName is back. dx generates no storyboard, so iOS paints black on \
             every cold start; use <key>UILaunchScreen</key><dict/> instead."
        );
        assert!(
            plist.contains("<key>UILaunchScreen</key>"),
            "no UILaunchScreen key: iOS has nothing to paint before the first frame."
        );
    }

    /// `CFBundleExecutable` names the binary inside `Mobile.app`, which dx derives from the cargo
    /// target name. Changing it to the display name produces a bundle that installs and then fails
    /// to launch.
    #[test]
    fn the_bundle_executable_is_the_cargo_target_name() {
        assert_eq!(plist_string(&read(PLIST), "CFBundleExecutable").as_deref(), Some("mobile"));
    }

    /// The plist replaces dx's template wholesale, so `[bundle] identifier` in Dioxus.toml no
    /// longer reaches iOS. Two sources of truth that must agree — and the provisioning profile is
    /// matched against this one.
    #[test]
    fn the_bundle_identifier_matches_dioxus_toml() {
        let id = plist_string(&read(PLIST), "CFBundleIdentifier").expect("CFBundleIdentifier");
        let toml = read(DIOXUS_TOML);
        let declared = toml
            .lines()
            .find_map(|l| l.trim().strip_prefix("identifier = "))
            .map(|v| v.trim().trim_matches('"').to_string())
            .expect("[bundle] identifier in Dioxus.toml");
        assert_eq!(id, declared, "Info.plist and Dioxus.toml disagree on the bundle identifier");
    }

    /// `[deep_links]` in Dioxus.toml is inert on iOS, so the custom scheme is hand-maintained here.
    /// If it drifts from `APP_SCHEME`, `scheme_link_for` builds links the app cannot open — and
    /// AltStore builds have no universal links to fall back on.
    #[test]
    fn the_url_scheme_matches_app_scheme() {
        let plist = read(PLIST);
        let at = plist.find("<key>CFBundleURLSchemes</key>").expect("CFBundleURLSchemes");
        let rest = &plist[at..];
        let open = rest.find("<string>").expect("a scheme") + "<string>".len();
        let close = rest[open..].find("</string>").expect("closing tag");
        assert_eq!(rest[open..open + close].trim(), APP_SCHEME);
    }

    /// The Scan button opens a file input with `capture`, and WKWebView presents the system camera
    /// for it. iOS **terminates the app** when a camera is opened without this key — and only on a
    /// real device, on first use, which is the worst possible place to find out. The plist replaces
    /// dx's template wholesale, so nothing else supplies it.
    #[test]
    fn the_camera_usage_description_is_present_and_meaningful() {
        let reason = plist_string(&read(PLIST), "NSCameraUsageDescription")
            .expect("NSCameraUsageDescription: iOS terminates the app without it");
        assert!(reason.len() > 20, "the OS shows this string verbatim: {reason:?}");
    }

    /// The other branch of the same sheet: "Choose Photo" reads the library.
    #[test]
    fn the_photo_library_usage_description_is_present_and_meaningful() {
        let reason = plist_string(&read(PLIST), "NSPhotoLibraryUsageDescription")
            .expect("NSPhotoLibraryUsageDescription");
        assert!(reason.len() > 20, "the OS shows this string verbatim: {reason:?}");
    }
}
