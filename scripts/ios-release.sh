#!/usr/bin/env bash
# The iOS store build, one step per call, shared by Codemagic (codemagic.yaml) and its GitHub
# fallback (mirror/.github/workflows/ios-release.yml): check, toolchain, dx, build, icons, package.
# Each CI keeps its own way of installing the signing material and uploading the IPA.
set -e
cd "$(dirname "$0")/.."

step_check() {
node scripts/asc.mjs check-version
node scripts/asc.mjs build-number > /tmp/ios_build_number
echo "CFBundleVersion = $(cat /tmp/ios_build_number)"
}

step_toolchain() {
# A pinned rustup-init checked against its published sha256, not `sh.rustup.rs | sh`, which
# runs whatever script that URL serves today. The toolchain itself is rust-toolchain.toml's.
curl --proto '=https' --tlsv1.2 -sSf -o /tmp/rustup-init \
  https://static.rust-lang.org/rustup/archive/1.29.1/aarch64-apple-darwin/rustup-init
echo "ec1b9233e7f72990ecd8e62063fa7f6c3dfc2bec8e97f88bff165f9100ac696a  /tmp/rustup-init" | shasum -a 256 -c -
chmod +x /tmp/rustup-init && /tmp/rustup-init -y
source "$HOME/.cargo/env"
rustup target add aarch64-apple-ios
rustup component add llvm-tools

# `[profile.release] strip = true` makes dx strip the binary with the toolchain's
# rust-objcopy (build/request.rs post_process_executable). That binary loads
# @rpath/libLLVM.dylib, and its only rpath is <toolchain>/lib/rustlib/<host>/lib -
# where rustup does NOT put the dylib (it lives in <toolchain>/lib). dx compensates
# with LD_LIBRARY_PATH, which macOS ignores, so the strip dies with
#   dyld: Library not loaded: @rpath/libLLVM.dylib
#   ERROR dx build: Failed to strip binary
# Symlink the dylib into the rpath dir, under the exact name the rpath asks for.
# DYLD_LIBRARY_PATH is not usable as a workaround: /bin/sh is SIP-restricted and
# strips DYLD_* from the environment it hands to children.
SYSROOT=$(rustc --print sysroot)
HOST=$(rustc -vV | awk '/^host:/{print $2}')
RPATH_DIR="$SYSROOT/lib/rustlib/$HOST/lib"
OBJCOPY="$SYSROOT/lib/rustlib/$HOST/bin/rust-objcopy"
mkdir -p "$RPATH_DIR"
FOUND_LLVM=$(find "$SYSROOT" -name 'libLLVM*.dylib' -not -path "$RPATH_DIR/*" | head -1)
if [ -n "$FOUND_LLVM" ]; then
  echo "Linking $FOUND_LLVM -> $RPATH_DIR/libLLVM.dylib"
  ln -sf "$FOUND_LLVM" "$RPATH_DIR/libLLVM.dylib"
fi

# If the strip tool still can't run, ship an unstripped binary rather than losing the
# build: `strip = false` makes dx skip the objcopy step entirely (get_strip_setting →
# StripSetting::None), and the sign step falls back to Apple's own `strip`, which has
# no LLVM dependency. Bigger IPA, but the upload still goes through.
echo "=== rust-objcopy check ==="
if [ ! -x "$OBJCOPY" ] || ! "$OBJCOPY" --version; then
  echo "*** rust-objcopy is unusable - disabling strip for this build ***" >&2
  echo "  libLLVM candidates under $SYSROOT:" >&2
  find "$SYSROOT" -name 'libLLVM*' >&2
  sed -i '' 's/^strip = true$/strip = false/' Cargo.toml
  grep -n '^strip' Cargo.toml || true
  touch /tmp/dx_strip_disabled
fi
}

step_dx() {
source "$HOME/.cargo/env"
REQUIRED="0.7.10"
INSTALLED=$(dx --version 2>/dev/null | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1)
if [ "$INSTALLED" != "$REQUIRED" ]; then
  cargo install dioxus-cli --version "$REQUIRED" --locked --force
fi
}

step_build() {
source "$HOME/.cargo/env"

export SDKROOT="$(xcrun --sdk iphoneos --show-sdk-path)"
export DEVELOPER_DIR="$(xcode-select -p)"
export IPHONEOS_DEPLOYMENT_TARGET=16.0
SDK_VERSION=$(xcrun --sdk iphoneos --show-sdk-version)

APP=target/dx/mobile/release/ios/Mobile.app
BINARY=$APP/mobile

# dx MUST be the linker: it patches manganis asset hashes (the __ASSETS__ symbols)
# into the binary after linking. Bypassing dx (old cargo+swap) ships a binary whose
# asset!() URLs are still placeholders, so every CSS/font/icon 404s and the app renders
# unstyled. But dx links the iOS binary stamped platform 7 (simulator), which drops
# LC_ENCRYPTION_INFO_64 -> App Store error 90125. dx ignores linker overrides via env
# and .cargo (see docs/ios-app-store-90125.md), so we intercept the cc/clang dx invokes
# by shadowing it on PATH and force ios-device platform flags - but only for the iOS
# executable link; host/proc-macro links pass straight through.
SHIM_DIR=/tmp/ccshim
mkdir -p "$SHIM_DIR"
cat > "$SHIM_DIR/clang" << 'SHIM_EOF'
#!/bin/bash
REAL="$(xcrun -f clang)"
SDK_PATH="$(xcrun --sdk iphoneos --show-sdk-path)"
SDK_VERSION="$(xcrun --sdk iphoneos --show-sdk-version)"
is_compile=0
is_ios=0
is_sim=0
for a in "$@"; do
  case "$a" in -c|-E|-S|-fsyntax-only) is_compile=1 ;; esac
  case "$a" in *-apple-ios*|*iPhoneOS.sdk*) is_ios=1 ;; esac
  case "$a" in *-simulator*|*-apple-ios-sim*|*iPhoneSimulator*) is_sim=1 ;; esac
done
# Only rewrite the iOS *device* executable link. Compiles, host links, and
# simulator links (dx's default target) pass straight through untouched -
# forcing device flags onto a simulator link breaks it.
if [ "$is_compile" -eq 1 ] || [ "$is_ios" -eq 0 ] || [ "$is_sim" -eq 1 ]; then
  exec "$REAL" "$@"
fi
echo "iOS link intercepted $(date '+%T')" >> /tmp/ccshim.log
NEW_ARGS=()
skip=0
for arg in "$@"; do
  if [ "$skip" -gt 0 ]; then skip=$((skip-1)); continue; fi
  case "$arg" in
    -platform_version) skip=3; continue ;;
    -Wl,-platform_version,*) continue ;;
  esac
  NEW_ARGS+=("$arg")
done
exec "$REAL" -isysroot "$SDK_PATH" -Wl,-platform_version,ios,16.0,"$SDK_VERSION" "${NEW_ARGS[@]}"
SHIM_EOF
chmod +x "$SHIM_DIR/clang"
cp "$SHIM_DIR/clang" "$SHIM_DIR/cc"
cp "$SHIM_DIR/clang" "$SHIM_DIR/gcc"
export PATH="$SHIM_DIR:$PATH"

echo "=== Build environment ==="
rustc --version
echo "SDKROOT=$SDKROOT"
echo "iOS SDK version: $SDK_VERSION"
xcodebuild -version | head -2

echo ""
echo "=== dx build device target (dx links device binary -> patches assets; ccshim forces platform 2) ==="
# --target aarch64-apple-ios: dx build --platform ios defaults to the SIMULATOR
# (aarch64-apple-ios-sim); the device triple makes dx link (and asset-patch) the
# binary we actually ship. Per `dx build --help`, --target is required for device.
dx build --platform ios --package mobile --release --target aarch64-apple-ios || {
  echo "ERROR: dx build failed - see the dx error above." >&2
  exit 1
}

# dx may emit the device .app under a target-specific path; normalize it to the
# canonical location the icon/signing steps expect.
if [ ! -f "$BINARY" ]; then
  FOUND=$(find target/dx/mobile -type d -name Mobile.app 2>/dev/null | head -1)
  if [ -n "$FOUND" ] && [ "$FOUND" != "$APP" ]; then
    echo "Relocating $FOUND -> $APP"
    rm -rf "$APP"; mkdir -p "$(dirname "$APP")"; cp -r "$FOUND" "$APP"
  fi
fi

# Without this the later steps run on a missing bundle and the failure only surfaces
# as an unrelated "Failed to publish Counted.ipa" at the very end of the build.
if [ ! -f "$BINARY" ]; then
  echo "ERROR: dx build produced no binary at $BINARY" >&2
  find target/dx/mobile -maxdepth 4 >&2 2>/dev/null || true
  exit 1
fi

platform_of() { xcrun otool -l "$1" | awk '$1=="platform"{print $2; exit}'; }
PLAT=$(platform_of "$BINARY")
echo "ccshim iOS-link interceptions: $([ -f /tmp/ccshim.log ] && wc -l < /tmp/ccshim.log | tr -d ' ' || echo 0)"
echo "platform after dx build: ${PLAT:-none} (2=device, 7=simulator)"

# Fallback: if the shim never intercepted dx's link the binary is still platform 7,
# which fails 90125 on upload. Rebuild it the proven (asset-breaking) cargo+swap way so
# the upload still succeeds. Status quo, not a regression - assets stay unstyled until
# the shim path works. See docs/ios-app-store-90125.md.
if [ "$PLAT" != "2" ]; then
  echo "*** ccshim did NOT intercept dx's link (platform=${PLAT:-none}); falling back to cargo+swap ***"
  cat > /tmp/ios_linker.sh << 'LINKER_EOF'
#!/bin/bash
SDK_PATH="$(xcrun --sdk iphoneos --show-sdk-path)"
SDK_VERSION="$(xcrun --sdk iphoneos --show-sdk-version)"
export SDKROOT="$SDK_PATH"
NEW_ARGS=()
skip=0
for arg in "$@"; do
  if [ "$skip" -gt 0 ]; then skip=$((skip-1)); continue; fi
  case "$arg" in
    -platform_version) skip=3; continue ;;
    -Wl,-platform_version,*) continue ;;
  esac
  NEW_ARGS+=("$arg")
done
exec xcrun clang -isysroot "$SDK_PATH" -Wl,-platform_version,ios,16.0,"$SDK_VERSION" "${NEW_ARGS[@]}"
LINKER_EOF
  chmod +x /tmp/ios_linker.sh
  rm -f target/aarch64-apple-ios/release/mobile
  CARGO_TARGET_AARCH64_APPLE_IOS_LINKER=/tmp/ios_linker.sh \
  cargo build --package mobile --target aarch64-apple-ios --release --features web
  cp target/aarch64-apple-ios/release/mobile "$BINARY"
  PLAT=$(platform_of "$BINARY")
  echo "platform after fallback swap: ${PLAT:-none}"
fi

echo ""
echo "=== Asset-patch verification ==="
echo "hashed assets in bundle: $(ls "$APP/assets" 2>/dev/null | grep -Ec 'tailwind|main' || echo 0)"
if strings -a "$BINARY" | grep -q 'replaced by dx as part of the build'; then
  echo "*** WARN: binary still contains the manganis PLACEHOLDER -> asset!() URLs NOT patched; CSS will 404 ***"
elif strings -a "$BINARY" | grep -q 'tailwind-'; then
  echo "OK: binary references hashed assets (tailwind-*) -> manganis patched"
else
  echo "*** WARN: no hashed asset reference found in binary - inspect manually ***"
fi
echo ""
echo "=== POST-BUILD binary diagnostics (BEFORE vtool/codesign) ==="
echo ""
echo "--- Platform/version load commands (platform must be 2=iOS, NOT 7=Simulator) ---"
xcrun otool -l "$BINARY" | grep -B1 -A5 "LC_BUILD_VERSION\|LC_VERSION_MIN" || echo "(no version load command)"
echo ""
echo "--- LC_ENCRYPTION_INFO presence ---"
xcrun otool -l "$BINARY" | grep -B1 -A4 "LC_ENCRYPTION" || echo "*** LC_ENCRYPTION_INFO MISSING ***"
echo ""
echo "--- LC_UUID (must differ from previous runs to prove rebuild) ---"
xcrun otool -l "$BINARY" | grep -A2 LC_UUID || true
echo ""
echo "--- All load command types ---"
xcrun otool -l "$BINARY" | grep -E "^[[:space:]]+cmd " | sort | uniq -c
}

step_icons() {
# 1024x1024 and 24bpp, written by packages/mobile/gen-android-icons.ps1. Not
# assets/counted.png: that is the 96px in-app logo, and every AppIcon below - including
# the 1024 marketing one - used to be upscaled out of it. The App Store also rejects an
# icon carrying an alpha channel, which this source deliberately has none of.
SOURCE_ICON="packages/mobile/assets/counted-1024.png"
APP_DIR="target/dx/mobile/release/ios/Mobile.app"
APPICONSET_DIR="/tmp/Assets.xcassets/AppIcon.appiconset"
mkdir -p "$APPICONSET_DIR"
# The named colour UILaunchScreen.UIColorName and the window/webview background resolve
# (packages/mobile/ios/Info.plist, set_launch_background in packages/mobile/src/main.rs).
cp -R packages/mobile/ios/LaunchBackground.colorset /tmp/Assets.xcassets/

echo "Source icon exists: $(test -f "$SOURCE_ICON" && echo YES || echo MISSING)"
gen() { sips -z $1 $1 "$SOURCE_ICON" --out "$APPICONSET_DIR/$2.png"; }
gen 40   "AppIcon-20@2x"
gen 60   "AppIcon-20@3x"
gen 58   "AppIcon-29@2x"
gen 87   "AppIcon-29@3x"
gen 80   "AppIcon-40@2x"
gen 120  "AppIcon-40@3x"
gen 120  "AppIcon-60@2x"
gen 180  "AppIcon-60@3x"
gen 20   "AppIcon-20"
gen 29   "AppIcon-29"
gen 40   "AppIcon-40"
gen 76   "AppIcon-76"
gen 152  "AppIcon-76@2x"
gen 167  "AppIcon-83.5@2x"
gen 1024 "AppIcon-1024"

cat > "$APPICONSET_DIR/Contents.json" << 'CONTENTSEOF'
{
  "images": [
    {"filename":"AppIcon-20@2x.png","idiom":"iphone","scale":"2x","size":"20x20"},
    {"filename":"AppIcon-20@3x.png","idiom":"iphone","scale":"3x","size":"20x20"},
    {"filename":"AppIcon-29@2x.png","idiom":"iphone","scale":"2x","size":"29x29"},
    {"filename":"AppIcon-29@3x.png","idiom":"iphone","scale":"3x","size":"29x29"},
    {"filename":"AppIcon-40@2x.png","idiom":"iphone","scale":"2x","size":"40x40"},
    {"filename":"AppIcon-40@3x.png","idiom":"iphone","scale":"3x","size":"40x40"},
    {"filename":"AppIcon-60@2x.png","idiom":"iphone","scale":"2x","size":"60x60"},
    {"filename":"AppIcon-60@3x.png","idiom":"iphone","scale":"3x","size":"60x60"},
    {"filename":"AppIcon-20.png","idiom":"ipad","scale":"1x","size":"20x20"},
    {"filename":"AppIcon-20@2x.png","idiom":"ipad","scale":"2x","size":"20x20"},
    {"filename":"AppIcon-29.png","idiom":"ipad","scale":"1x","size":"29x29"},
    {"filename":"AppIcon-29@2x.png","idiom":"ipad","scale":"2x","size":"29x29"},
    {"filename":"AppIcon-40.png","idiom":"ipad","scale":"1x","size":"40x40"},
    {"filename":"AppIcon-40@2x.png","idiom":"ipad","scale":"2x","size":"40x40"},
    {"filename":"AppIcon-76.png","idiom":"ipad","scale":"1x","size":"76x76"},
    {"filename":"AppIcon-76@2x.png","idiom":"ipad","scale":"2x","size":"76x76"},
    {"filename":"AppIcon-83.5@2x.png","idiom":"ipad","scale":"2x","size":"83.5x83.5"},
    {"filename":"AppIcon-1024.png","idiom":"ios-marketing","scale":"1x","size":"1024x1024"}
  ],
  "info": {"author":"xcode","version":1}
}
CONTENTSEOF

xcrun actool /tmp/Assets.xcassets \
  --compile "$APP_DIR" \
  --platform iphoneos \
  --minimum-deployment-target 16.0 \
  --target-device iphone \
  --target-device ipad \
  --app-icon AppIcon \
  --output-partial-info-plist /tmp/actool_partial.plist \
  --compress-pngs \
  --errors --warnings --notices

echo "=== sips output files ==="
ls -la "$APPICONSET_DIR"/*.png | awk '{print $5, $9}'
echo "=== Assets.car present: $(test -f "$APP_DIR/Assets.car" && echo YES || echo MISSING) ==="
echo "=== actool partial plist ==="
cat /tmp/actool_partial.plist
xcrun assetutil --info "$APP_DIR/Assets.car" | grep -q 'LaunchBackground' \
  || { echo "ERROR: LaunchBackground colour missing from Assets.car" >&2; exit 1; }
}

step_package() {
PROFILES_HOME="$HOME/Library/MobileDevice/Provisioning Profiles"
SIGNING_IDENTITY=$(security find-identity -v -p codesigning | grep "Apple Distribution" | head -1 | sed 's/.*"\(.*\)"/\1/')

cp "$PROFILES_HOME/profile.mobileprovision" \
  target/dx/mobile/release/ios/Mobile.app/embedded.mobileprovision

security cms -D -i "$PROFILES_HOME/profile.mobileprovision" \
  | plutil -extract Entitlements xml1 -o entitlements.plist -

# Universal links. The profile only *grants* the capability - distribution profiles
# carry com.apple.developer.associated-domains as the wildcard "*". The signed binary
# must name the domain itself, so overwrite the wildcard with the real value. Without
# this the app is signed with "*", claims nothing, and https://counted.fr/projects/...
# keeps opening Safari. See docs/deep-links.md.
#
# Fail loudly if the profile lacks the capability: that means the App ID does not have
# Associated Domains enabled, or CM_PROVISIONING_PROFILE is a stale pre-capability
# profile. Signing with an entitlement the profile does not grant is rejected at
# install time, so catching it here beats shipping a broken TestFlight build.
if ! grep -q "associated-domains" entitlements.plist; then
  echo "ERROR: the provisioning profile has no com.apple.developer.associated-domains." >&2
  echo "  Enable Associated Domains on the fr.counted.app App ID, regenerate the" >&2
  echo "  profile, and update CM_PROVISIONING_PROFILE. See docs/deep-links.md." >&2
  exit 1
fi
# Full path, not $PB - that alias is defined further down, with the Info.plist edits.
/usr/libexec/PlistBuddy -c "Delete :com.apple.developer.associated-domains" entitlements.plist 2>/dev/null || true
/usr/libexec/PlistBuddy -c "Add :com.apple.developer.associated-domains array" entitlements.plist
/usr/libexec/PlistBuddy -c "Add :com.apple.developer.associated-domains:0 string applinks:counted.fr" entitlements.plist
# Password AutoFill: the login fields offer and save counted.fr passwords. The same
# capability, matched by the `webcredentials` entry in nginx/well-known/apple-app-site-association.
/usr/libexec/PlistBuddy -c "Add :com.apple.developer.associated-domains:1 string webcredentials:counted.fr" entitlements.plist

# Push. The profile carries aps-environment only when Push Notifications is enabled on
# the App ID; without it registerForRemoteNotifications fails at runtime with nothing but
# a log line, so the app would ship silently push-less. See docs/plans/push-notifications.md.
if ! grep -q "aps-environment" entitlements.plist; then
  echo "ERROR: the provisioning profile has no aps-environment entitlement." >&2
  echo "  Enable Push Notifications on the fr.counted.app App ID, regenerate the" >&2
  echo "  profile, and update CM_PROVISIONING_PROFILE. See docs/plans/push-notifications.md." >&2
  exit 1
fi

echo "=== Entitlements used for signing ==="
plutil -p entitlements.plist

BINARY="target/dx/mobile/release/ios/Mobile.app/mobile"

# Only when the Install Rust step had to disable dx's strip (broken rust-objcopy):
# strip with Apple's tool instead, which has no LLVM dependency. Runs before codesign,
# the same order Xcode uses for Strip Linked Product, and never fatal - an unstripped
# binary uploads fine, it is just large. Left off entirely on the normal path so the
# working build stays byte-for-byte what it was.
if [ -f /tmp/dx_strip_disabled ]; then
  echo "dx strip was disabled - stripping with Apple's strip instead"
  echo "size before strip: $(du -h "$BINARY" | cut -f1)"
  xcrun strip -x "$BINARY" || echo "(Apple strip failed with $? - shipping unstripped)"
  echo "size after strip:  $(du -h "$BINARY" | cut -f1)"
fi

SDK_VERSION=$(xcrun --sdk iphoneos --show-sdk-version)
xcrun vtool -arch arm64 \
  -set-build-version ios 16.0 "$SDK_VERSION" \
  -replace -output "$BINARY" "$BINARY"

XCODE_VERSION=$(xcodebuild -version | head -1 | awk '{print $2}' | awk -F. '{printf "%d%02d", $1, $2}')
XCODE_BUILD=$(xcodebuild -version | grep "Build version" | awk '{print $3}')
SDK_VERSION=$(xcrun --sdk iphoneos --show-sdk-version)
SDK_BUILD=$(xcrun --sdk iphoneos --show-sdk-build-version)
SDK_NAME="iphoneos${SDK_VERSION}"

PLIST="target/dx/mobile/release/ios/Mobile.app/Info.plist"
PB=/usr/libexec/PlistBuddy
plist_set() { $PB -c "Add :$1 string $2" "$PLIST" 2>/dev/null || $PB -c "Set :$1 $2" "$PLIST"; }
plist_set MinimumOSVersion 16.0
plist_set CFBundlePackageType APPL
plist_set DTPlatformName iphoneos

# Merge actool-generated icon keys (CFBundleIconName, CFBundleIcons, etc.)
$PB -c "Merge /tmp/actool_partial.plist" "$PLIST"

# UILaunchScreen - required for iPad multitasking (iOS 14+). The checked-in plist carries it
# with UIColorName = LaunchBackground, the colorset compiled into Assets.car above.
if [ "$($PB -c "Print :UILaunchScreen:UIColorName" "$PLIST" 2>/dev/null)" != "LaunchBackground" ]; then
  echo "ERROR: UILaunchScreen:UIColorName is not LaunchBackground - the launch screen would paint systemBackground" >&2
  exit 1
fi

# packages/mobile/ios/Info.plist hardcodes the marketing version (dx replaces its
# template with that file, so it no longer inherits Cargo.toml). Re-derive it here so a
# release can never ship a stale CFBundleShortVersionString.
# Delete+Add, not plist_set: the key always exists in the checked-in plist, and
# plist_set's `Add || Set` fallback relies on PlistBuddy's exit code for a duplicate key.
SHORT_VERSION=$(grep '^version' packages/mobile/Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/')
if [ -z "$SHORT_VERSION" ]; then
  echo "ERROR: could not read version from packages/mobile/Cargo.toml" >&2
  exit 1
fi
$PB -c "Delete :CFBundleShortVersionString" "$PLIST" 2>/dev/null || true
$PB -c "Add :CFBundleShortVersionString string $SHORT_VERSION" "$PLIST"
echo "CFBundleShortVersionString = $SHORT_VERSION"

plist_set DTPlatformVersion "$SDK_VERSION"
plist_set DTPlatformBuild "$SDK_BUILD"
plist_set DTSDKName "$SDK_NAME"
plist_set DTSDKBuild "$SDK_BUILD"
plist_set DTXcode "$XCODE_VERSION"
plist_set DTXcodeBuild "$XCODE_BUILD"

# CFBundleVersion must be unique per upload. The check step wrote the highest build number App
# Store Connect holds, across every version, plus one: Codemagic's own BUILD_NUMBER counter knows
# nothing of a build the GitHub fallback uploaded, and would collide with it.
# Fail loud if it is missing - an empty CFBundleVersion gets a confusing ASC error far downstream.
BUILD_NUMBER=$(cat /tmp/ios_build_number 2>/dev/null || true)
if [ -z "$BUILD_NUMBER" ]; then
  echo "ERROR: no build number in /tmp/ios_build_number - the check step must run first" >&2
  exit 1
fi
plist_set CFBundleVersion "$BUILD_NUMBER"
echo "CFBundleVersion = $BUILD_NUMBER"

# Encryption export compliance - eliminates the App Store Connect modal on each upload.
# ITSAppUsesNonExemptEncryption is a BOOLEAN, not a string, so direct PlistBuddy syntax.
# The compliance code (UUID) is a secret, APP_STORE_ENCRYPTION_CODE, in Codemagic's "app_store"
# group and in the GitHub mirror's app-store environment, so it doesn't appear in git.
if [ -z "$APP_STORE_ENCRYPTION_CODE" ]; then
  echo "ERROR: APP_STORE_ENCRYPTION_CODE is empty - set it in Codemagic's app_store group or the GitHub app-store environment" >&2
  exit 1
fi
$PB -c "Delete :ITSAppUsesNonExemptEncryption" "$PLIST" 2>/dev/null || true
$PB -c "Add :ITSAppUsesNonExemptEncryption bool true" "$PLIST"
plist_set ITSEncryptionExportComplianceCode "$APP_STORE_ENCRYPTION_CODE"

$PB -c "Delete :CFBundleSupportedPlatforms" "$PLIST" 2>/dev/null || true
$PB -c "Add :CFBundleSupportedPlatforms array" "$PLIST"
$PB -c "Add :CFBundleSupportedPlatforms: string iPhoneOS" "$PLIST"

echo "=== Final Info.plist ==="
plutil -p "$PLIST"
echo "=== Binary load commands ==="
xcrun otool -l "$BINARY" | grep -E "cmd |LC_ENCRYPTION|LC_BUILD_VERSION|cryptid|cryptoff"

# The push sentences, one table per shipped locale, for the loc-key pushes of the audit's
# phase C (docs/security-audit-2026-10-19.md L7). dx bundles no .lproj of its own. iOS
# picks the table from the system or per-app language; -lint fails a malformed one here
# rather than as a raw key on a lock screen. InfoPlist.strings, beside it, translates the
# camera and photo-library prompts (the plist's own values are the French fallback).
for d in packages/mobile/ios/strings/*.lproj; do
  plutil -lint "$d/Localizable.strings"
  plutil -lint "$d/InfoPlist.strings"
  cp -R "$d" target/dx/mobile/release/ios/Mobile.app/
done

codesign --force --deep \
  --sign "$SIGNING_IDENTITY" \
  --entitlements entitlements.plist \
  target/dx/mobile/release/ios/Mobile.app

mkdir -p Payload
cp -r target/dx/mobile/release/ios/Mobile.app Payload/
zip -r Counted.ipa Payload/
rm -rf Payload
}

case "${1:-}" in
	check | toolchain | dx | build | icons | package) "step_$1" ;;
	*)
		echo "usage: scripts/ios-release.sh check|toolchain|dx|build|icons|package" >&2
		exit 2
		;;
esac
