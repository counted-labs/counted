//! Guards over `packages/mobile/ios/Info.plist`.
//!
//! `docs/ios.md` records four things about that file that "must stay in sync" with something
//! elsewhere, and nothing enforced any of them. Each has a failure mode that is invisible until an
//! iOS build is in someone's hands: the app installs as "Mobile", the launch screen paints black,
//! a share link opens nothing.
//!
//! Lives in `ui` rather than `mobile` for the reason given in `stylesheet_guard.rs` - and because
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

/// The `<string>` items of the `<array>` following a `<key>`.
fn plist_array(plist: &str, key: &str) -> Vec<String> {
    let at = plist.find(&format!("<key>{key}</key>")).unwrap_or_else(|| panic!("no {key}"));
    let rest = &plist[at..];
    let open = rest.find("<array>").expect("an array") + "<array>".len();
    let close = rest[open..].find("</array>").expect("closing tag");
    rest[open..open + close]
        .split("<string>")
        .skip(1)
        .filter_map(|s| s.split("</string>").next())
        .map(|s| s.trim().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::APP_SCHEME;

    /// Fix 4's guard. dx's template points `UILaunchStoryboardName` at a `LaunchScreen` storyboard
    /// it never generates, so iOS resolves it to nothing and paints black until the webview draws -
    /// and a dangling key is worse than no key, because with neither iOS applies its own default.
    /// See docs/dioxus-issues/mobile-no-native-splash-screen.md.
    #[test]
    fn the_launch_screen_does_not_point_at_a_storyboard_that_is_never_generated() {
        let plist = read(PLIST);
        assert!(
            !plist.contains("<key>UILaunchStoryboardName</key>"),
            "UILaunchStoryboardName is back. dx generates no storyboard, so iOS paints black on \
             every cold start; use the <key>UILaunchScreen</key> dict instead."
        );
        assert!(
            plist.contains("<key>UILaunchScreen</key>"),
            "no UILaunchScreen key: iOS has nothing to paint before the first frame."
        );
    }

    /// The launch screen paints the asset catalog's `LaunchBackground`, which must be the colour wry
    /// paints next (`with_background_color` in packages/mobile/src/main.rs), light and dark, or a
    /// cold start shows a seam between the two.
    #[test]
    fn the_launch_colour_is_named_and_matches_the_webview_background() {
        let plist = read(PLIST);
        let at = plist.find("<key>UILaunchScreen</key>").expect("UILaunchScreen");
        assert_eq!(plist_string(&plist[at..], "UIColorName").as_deref(), Some("LaunchBackground"));

        let colorset = read("../mobile/ios/LaunchBackground.colorset/Contents.json");
        let channels: Vec<u8> = colorset
            .split("\"0x")
            .skip(1)
            .map(|s| u8::from_str_radix(&s[..2], 16).expect("hex channel"))
            .collect();
        assert_eq!(channels.len(), 6, "light then dark, red green blue: {channels:?}");
        assert!(colorset.contains("\"value\" : \"dark\""), "the second colour is the dark appearance");
        let main = read("../mobile/src/main.rs");
        for rgb in channels.chunks(3) {
            let tuple = format!("({}, {}, {}, 255)", rgb[0], rgb[1], rgb[2]);
            assert!(main.contains(&tuple), "main.rs paints no {tuple}: the launch colour and wry's differ");
        }
    }

    /// `CFBundleExecutable` names the binary inside `Mobile.app`, which dx derives from the cargo
    /// target name. Changing it to the display name produces a bundle that installs and then fails
    /// to launch.
    #[test]
    fn the_bundle_executable_is_the_cargo_target_name() {
        assert_eq!(plist_string(&read(PLIST), "CFBundleExecutable").as_deref(), Some("mobile"));
    }

    /// The plist replaces dx's template wholesale, so `[bundle] identifier` in Dioxus.toml no
    /// longer reaches iOS. Two sources of truth that must agree - and the provisioning profile is
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
    /// If it drifts from `APP_SCHEME`, `scheme_link_for` builds links the app cannot open - and
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
    /// for it. iOS **terminates the app** when a camera is opened without this key - and only on a
    /// real device, on first use, which is the worst possible place to find out. The plist replaces
    /// dx's template wholesale, so nothing else supplies it.
    #[test]
    fn the_camera_usage_description_is_present_and_meaningful() {
        let reason = plist_string(&read(PLIST), "NSCameraUsageDescription")
            .expect("NSCameraUsageDescription: iOS terminates the app without it");
        assert!(reason.len() > 20, "the OS shows this string verbatim: {reason:?}");
    }

    /// Every shipped locale translates both permission prompts, and French says exactly what the
    /// plist's fallback says. A missing table shows the French prompt; a missing key, the same.
    #[test]
    fn every_locale_translates_the_permission_prompts() {
        let plist = read(PLIST);
        let strings = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../mobile/ios/strings");
        let mut locales = 0;
        for entry in std::fs::read_dir(&strings).expect("ios/strings") {
            let dir = entry.expect("entry").path();
            let table = std::fs::read_to_string(dir.join("InfoPlist.strings"))
                .unwrap_or_else(|_| panic!("{} has no InfoPlist.strings", dir.display()));
            for key in ["NSCameraUsageDescription", "NSPhotoLibraryUsageDescription"] {
                let line = table
                    .lines()
                    .find(|l| l.starts_with(&format!("\"{key}\" = \"")))
                    .unwrap_or_else(|| panic!("{} lacks {key}", dir.display()));
                let value = line.split(" = \"").nth(1).and_then(|v| v.strip_suffix("\";")).expect("a value");
                assert!(value.len() > 20 && !value.contains('"'), "{} {key}: {value:?}", dir.display());
                if dir.ends_with("fr.lproj") {
                    assert_eq!(Some(value.to_string()), plist_string(&plist, key), "fr {key} differs from Info.plist");
                }
            }
            locales += 1;
        }
        let ftl = std::fs::read_dir(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("i18n")).expect("i18n").count();
        assert_eq!(locales, ftl, "one .lproj per app language");
    }

    /// The layout is one portrait column with no left/right safe-area insets, so the iPhone is
    /// locked to portrait; the iPad keeps all four, which multitasking requires.
    #[test]
    fn the_iphone_is_portrait_only_and_the_ipad_rotates() {
        let plist = read(PLIST);
        assert_eq!(plist_array(&plist, "UISupportedInterfaceOrientations"), ["UIInterfaceOrientationPortrait"]);
        assert_eq!(plist_array(&plist, "UISupportedInterfaceOrientations~ipad").len(), 4);
    }

    /// The other branch of the same sheet: "Choose Photo" reads the library.
    #[test]
    fn the_photo_library_usage_description_is_present_and_meaningful() {
        let reason = plist_string(&read(PLIST), "NSPhotoLibraryUsageDescription")
            .expect("NSPhotoLibraryUsageDescription");
        assert!(reason.len() > 20, "the OS shows this string verbatim: {reason:?}");
    }
}
