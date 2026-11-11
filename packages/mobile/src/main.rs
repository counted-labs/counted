use dioxus::prelude::*;
use ui::common::{
    use_app_contexts, use_client_ready, NativeClipboardReader, NativeDeepLinkReader, SplashScreen,
    UpdateRequiredScreen,
};
use ui::route::Route;

#[cfg(any(target_os = "android", target_os = "ios"))]
use ui::common::push_deep_link;

#[cfg(any(target_os = "android", target_os = "ios"))]
mod push;

/// Where `UpdateRequiredScreen` sends the user. `webbrowser` - what an anchor click in the WebView
/// goes through - only opens http(s), so the link is the store's web page, which hands off to the
/// store app. The F-Droid build overrides it at compile time (`android-apk-unsigned` in
/// packages/mobile/android.mk), since its users have no Play Store to update from.
#[cfg(target_os = "ios")]
const STORE_URL: &str = "https://apps.apple.com/app/id6772807915";
#[cfg(not(target_os = "ios"))]
const STORE_URL: &str = match option_env!("COUNTED_STORE_URL") {
    Some(url) => url,
    None => "https://play.google.com/store/apps/details?id=fr.counted.app",
};

const FAVICON: Asset = asset!("/assets/counted.ico");
const LOGO: Asset = asset!("/assets/counted.png");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

// Plus Jakarta Sans for `font-display` titles only: the body is the system font
// (tailwind.input.css). Self-hosted via asset!() so the files bundle with hashed URLs.
const FONT_JAKARTA_600: Asset = asset!("/assets/fonts/jakarta-600.woff2");
const FONT_JAKARTA_700: Asset = asset!("/assets/fonts/jakarta-700.woff2");
const FONT_JAKARTA_LATIN_EXT: Asset = asset!("/assets/fonts/jakarta-latin-ext.woff2");

const LATIN_EXT: &str = "U+0100-02BA,U+02BD-02C5,U+02C7-02CC,U+02CE-02D7,U+02DD-02FF,U+0304,U+0308,U+0329,U+1D00-1DBF,U+1E00-1E9F,U+1EF2-1EFF,U+2020,U+20A0-20AB,U+20AD-20C0,U+2113,U+2C60-2C7F,U+A720-A7FF";

/// The Latin files carry no unicode-range, so the Latin Extended subset must come after them:
/// faces with the same descriptors are tried last-declared first (CSS Fonts 4 §4.5). The subset
/// is one variable file, declared once per weight.
fn font_face_css() -> String {
    let face = |weight: u16, file: Asset, range: &str| {
        format!("@font-face{{font-family:\"Plus Jakarta Sans\";font-style:normal;font-weight:{weight};font-display:swap;src:url(\"{file}\") format(\"woff2\"){range}}}\n")
    };
    let ext = format!(";unicode-range:{LATIN_EXT}");
    [face(600, FONT_JAKARTA_600, ""), face(700, FONT_JAKARTA_700, ""), face(600, FONT_JAKARTA_LATIN_EXT, &ext), face(700, FONT_JAKARTA_LATIN_EXT, &ext)].concat()
}

/// dioxus-desktop's release script calls preventDefault on `contextmenu` from a bubbling listener
/// on `document`, and on Android and iOS that default action is the text-selection toolbar. A
/// capturing listener on `window` runs first and stops the event there.
const KEEP_SELECTION_TOOLBAR: &str =
    "window.addEventListener('contextmenu', e => e.stopImmediatePropagation(), true);";

/// `html[data-plat]` and, on iOS, `html[data-ios]` (the major version): the only switches the
/// stylesheets branch on (docs/styling.md). Set from the head, like the theme, so the first paint
/// already has them.
fn platform_markers(ios_major: Option<isize>) -> String {
    match ios_major {
        Some(major) => format!(
            "document.documentElement.dataset.plat='ios';document.documentElement.dataset.ios='{major}';"
        ),
        None => "document.documentElement.dataset.plat='android';".to_string(),
    }
}

#[cfg(target_os = "ios")]
fn ios_major() -> Option<isize> {
    Some(objc2_foundation::NSProcessInfo::processInfo().operatingSystemVersion().majorVersion)
}

#[cfg(not(target_os = "ios"))]
fn ios_major() -> Option<isize> {
    None
}

/// dioxus-desktop's template viewport (`assets/prod.index.html`, 0.7.10) plus `viewport-fit=cover`, so
/// `env(safe-area-inset-*)` is right on the first frame. A later viewport meta replaces the earlier one
/// whole, hence the full restatement.
const VIEWPORT: &str = r#"<meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, viewport-fit=cover">"#;

/// Dynamic Type, from the head so the first paint already has the scale. `-apple-system-body` is the
/// only handle a WKWebView has on the user's Text Size setting; 17px is the "Large" default, so the
/// ratio is the scale. Driven by `resize`, which iOS fires on a content-size-category change, so the
/// same listener covers rotation. Measured on `<html>` while the body does not exist yet: the
/// stylesheets above have loaded by then (they block the script), and the keyword is absolute.
///
/// Clamped at 2 (about AX2): the pages are vertical and absorb growth up there, and the header and
/// the dock stop growing at 1.35 on their own (`--chrome-zoom` in main.css).
///
/// **The sentinel is load-bearing.** Where the `font` shorthand does not parse the keyword, the
/// declaration is dropped and the span inherits instead - and what it inherits is
/// `16px * var(--dt-scale)`, the value this very script wrote. That is a feedback loop: each resize
/// divides the previous scale by 17/16 until it sticks at the 0.85 floor, shrinking the whole UI ~15%
/// with nothing to point at. Setting `font-size` to a value nothing would ever compute *before* the
/// shorthand detects it exactly: a parsed shorthand resets every font longhand, so 99px surviving
/// means the keyword was ignored and there is nothing to measure.
///
/// The same span carries Bold Text: `-apple-system-body` resolves to a heavier weight while it is
/// on, so a weight of 600 or more sets `data-bold-text` on `<html>` (main.css raises the weights
/// under it). Read before the span is removed: a detached element's computed style is empty.
///
/// iOS only. Android's WebView already applies the system font-size setting itself, through
/// `WebSettings.textZoom`, so there is nothing for this to add there.
const DYNAMIC_TYPE_SCRIPT: &str = "(function(){function measure(){var p=document.createElement('span');p.style.position='absolute';p.style.visibility='hidden';p.style.fontSize='99px';p.style.font='-apple-system-body';var host=document.body||document.documentElement;host.appendChild(p);var cs=getComputedStyle(p);var px=parseFloat(cs.fontSize);var fw=cs.fontWeight;p.remove();if(!(px>0)||px===99)return;var html=document.documentElement;html.toggleAttribute('data-bold-text',fw==='bold'||parseInt(fw,10)>=600);var scale=String(Math.min(2,Math.max(0.85,px/17)));var root=html.style;if(root.getPropertyValue('--dt-scale')!==scale)root.setProperty('--dt-scale',scale);}window.addEventListener('resize',measure);measure();})();";

/// The keypad already up when the add-expense sheet opens, iOS only (native-feel P4.4). WebKit shows
/// the keyboard for a focus made inside a user gesture and never for one made by script later, and
/// the sheet only exists a Rust round trip after the tap. So the add button's `touchend` focuses a
/// hidden `inputmode=decimal` proxy at once, and the amount field takes the focus over from it at the
/// end of the sheet's slide-in (`sheet-in`): focus moving between two fields keeps the keyboard up,
/// and a field focused mid-slide would make WebKit pan to a box still below the screen. A tap that
/// opened nothing gives the proxy up after 2 s. `modal_focus.rs` leaves the proxy's focus alone.
/// Android raises the keyboard on a script focus already. The tap's compatibility `mousedown`
/// arrives 40 ms after the `touchend` and its default moves the focus off the proxy (measured
/// 2026-11-10, docs/ios.md), so it is cancelled while the proxy holds the focus for this tap.
const KEYBOARD_PROXY_SCRIPT: &str = "(function(){var proxy=null,start=null,handing=0;function make(){proxy=document.createElement('input');proxy.type='text';proxy.setAttribute('inputmode','decimal');proxy.setAttribute('aria-hidden','true');proxy.setAttribute('data-keyboard-proxy','');proxy.setAttribute('autocomplete','off');proxy.tabIndex=-1;proxy.style.cssText='position:fixed;top:0;left:0;width:1px;height:1px;opacity:0;border:0;padding:0;font-size:16px;pointer-events:none';document.body.appendChild(proxy);}function afterSlide(box,then){var ran=false;function run(){if(ran)return;ran=true;then();}var a=box.getAnimations?box.getAnimations().filter(function(x){return x.animationName==='sheet-in'&&x.playState==='running';}):[];if(!a.length)return run();a[0].finished.then(run,run);setTimeout(run,700);}document.addEventListener('touchstart',function(e){var t=e.touches[0];start=e.touches.length===1&&e.target instanceof Element&&e.target.closest('#add-expense-btn')?{x:t.clientX,y:t.clientY}:null;},{capture:true,passive:true});document.addEventListener('touchend',function(e){var s=start;start=null;if(!s||e.touches.length)return;var t=e.changedTouches[0];if(Math.abs(t.clientX-s.x)>10||Math.abs(t.clientY-s.y)>10)return;if(!proxy||!proxy.isConnected)make();proxy.value='';proxy.focus({preventScroll:true});handing=Date.now();setTimeout(function(){handing=0;if(document.activeElement===proxy)proxy.blur();},2000);},{capture:true,passive:true});document.addEventListener('mousedown',function(e){if(handing&&document.activeElement===proxy&&e.target instanceof Element&&e.target.closest('#add-expense-btn'))e.preventDefault();},true);window._countedObserve(function(){if(!handing||document.activeElement!==proxy)return;var field=document.getElementById('expense-amount');var box=field&&field.closest('.modal-box');if(!box)return;handing=0;afterSlide(box,function(){if(document.activeElement===proxy&&field.isConnected)field.focus({preventScroll:true});});});})();";

/// What a status-bar tap does on iOS (native-feel P4.3): the page scrolls in its own `.app-container`,
/// which UIKit's scroll-to-top cannot reach, so `ScrollToTop` (the webview scroll view's delegate)
/// calls this and declines the native scroll.
const SCROLL_TO_TOP_SCRIPT: &str = "window._countedScrollToTop=function(){var s=document.querySelector('main > .page-shell:not(.page-exiting) .app-container');if(s)s.scrollTo({top:0,behavior:window.matchMedia('(prefers-reduced-motion: reduce)').matches?'auto':'smooth'});};";

/// Android predictive back inside the app (native-feel P4.2), driven by `MainActivity.kt`'s
/// `OnBackPressedCallback`, which is enabled only while a back is armed: `p` in 0..1 from
/// `handleOnBackStarted` / `handleOnBackProgressed` with the swipe edge (0 left, 1 right), -1 on
/// `handleOnBackCancelled`, 2 when the back commits. The live shell takes M3's in-app shape: scaled
/// down to 90%, shifted towards the side away from the swiped edge until 8px from the screen's
/// edge, corners rounded (main.css). On commit it stays shrunk and `page_transition.rs` fades it out
/// as it is; cancelled, it springs back. Nothing while a sheet is open (back closes the sheet) or
/// under Reduce Motion.
const PREDICTIVE_BACK_SCRIPT: &str = "(function(){var reduce=window.matchMedia('(prefers-reduced-motion: reduce)');function live(){return document.querySelector('main > .page-shell:not(.page-exiting)');}function reset(el){if(!el.hasAttribute('data-back-progress'))return;el.removeAttribute('data-back-progress');el.classList.add('back-settling');el.style.removeProperty('transform');setTimeout(function(){el.classList.remove('back-settling');},300);}window._countedBackProgress=function(p,edge){var el=live();if(!el)return;if(p<0)return reset(el);if(p>1){if(el.hasAttribute('data-back-progress'))setTimeout(function(){if(el.isConnected)reset(el);},1000);return;}if(reduce.matches||document.querySelector('.modal.modal-open'))return;var w=document.documentElement.clientWidth;var shift=(edge===1?-1:1)*p*Math.max(0,w*0.05-8);el.setAttribute('data-back-progress','');el.style.transform='translateX('+shift.toFixed(1)+'px) scale('+(1-0.1*p).toFixed(4)+')';};})();";

/// One `MutationObserver` on the whole document for every mobile script (`navigation_sync`,
/// `sheet_drag`, `page_transition` in `ui`), each registered with `window._countedObserve(fn)` and
/// handed every batch of records, in the same microtask a private observer would get them. A throw in
/// one does not starve the others.
const OBSERVE_SCRIPT: &str = "(function(){var fns=[];window._countedObserve=function(f){fns.push(f);};new MutationObserver(function(r){for(var i=0;i<fns.length;i++){try{fns[i](r);}catch(e){console.error(e);}}}).observe(document.documentElement,{childList:true,subtree:true});})();";

/// Stylesheets spliced into dioxus-desktop's index template before `</head>`, so they are
/// render-blocking - `document::Link` only reaches the head after the first render, leaving the
/// webview's first paint unstyled. This does not replace the template (docs/dioxus-overrides.md).
///
/// Order is tailwind, the shared native-feel sheet, then main: main.css is unlayered and must win.
///
/// Only the mobile config consumes it - not the host server binary that
/// `dx bundle --platform android` also builds.
#[cfg_attr(not(any(target_os = "android", target_os = "ios")), allow(dead_code))]
fn head_html(theme: ui::common::Theme) -> String {
    format!(
        // The empty icon is not decoration: a WebView asks for /favicon.ico on every document load,
        // wry's asset protocol has none to serve, and the 404 is logged as a console error - noise
        // in every session, and enough to make `boots without console errors` flake depending on
        // whether it lands before the spec attaches. `data:,` answers it without a request. No app
        // window ever shows an icon here, so there is nothing to supply instead.
        r#"{VIEWPORT}<link rel="icon" href="data:,"><link rel="stylesheet" href="{TAILWIND_CSS}"><link rel="stylesheet" href="{native_feel}"><link rel="stylesheet" href="{MAIN_CSS}"><style>{fonts}</style><script>{KEEP_SELECTION_TOOLBAR}{OBSERVE_SCRIPT}{markers}{boot}countedTheme('{theme}');{dynamic_type}{ios_scripts}{predictive_back}</script>"#,
        fonts = font_face_css(),
        native_feel = ui::common::NATIVE_FEEL_CSS,
        markers = platform_markers(ios_major()),
        boot = ui::common::BOOT_JS_SOURCE,
        dynamic_type = if cfg!(target_os = "ios") { DYNAMIC_TYPE_SCRIPT } else { "" },
        ios_scripts = if cfg!(target_os = "ios") { [KEYBOARD_PROXY_SCRIPT, SCROLL_TO_TOP_SCRIPT].concat() } else { String::new() },
        predictive_back = if cfg!(target_os = "android") { PREDICTIVE_BACK_SCRIPT } else { "" },
        theme = theme.as_str()
    )
}

/// Application Support via NSFileManager (created if missing), excluded from iCloud/iTunes backups
/// so the per-project E2EE keys never leave the device.
#[cfg(target_os = "ios")]
fn ios_data_dir() -> Option<String> {
    use objc2_foundation::{
        NSFileManager, NSNumber, NSSearchPathDirectory, NSSearchPathDomainMask,
        NSURLIsExcludedFromBackupKey,
    };
    unsafe {
        let fm = NSFileManager::defaultManager();
        let url = fm
            .URLForDirectory_inDomain_appropriateForURL_create_error(
                NSSearchPathDirectory::ApplicationSupportDirectory,
                NSSearchPathDomainMask::UserDomainMask,
                None,
                true,
            )
            .ok()?;
        let yes = NSNumber::new_bool(true);
        let value: Option<&objc2::runtime::AnyObject> = Some(&yes);
        let _ = url.setResourceValue_forKey_error(value, NSURLIsExcludedFromBackupKey);
        url.path().map(|p| p.to_string())
    }
}

/// Overrides `--sat`/`--sab`/`--sar` from `ui/assets/native-feel.css`. An inline property on the root
/// element beats the `:root` rule, so this is the only hook Android needs - every `.safe-*` rule
/// and `main { padding-top }` follows. Android reports physical pixels; CSS wants density-independent.
#[cfg_attr(not(target_os = "android"), allow(dead_code))]
fn css_inset_vars(top_px: i32, bottom_px: i32, right_px: i32, density: f32) -> String {
    let density = if density > 0.0 { density } else { 1.0 };
    let css = |px: i32| format!("{:.1}px", px as f32 / density);
    format!(
        "document.documentElement.style.setProperty('--sat','{}');\
         document.documentElement.style.setProperty('--sab','{}');\
         document.documentElement.style.setProperty('--sar','{}');",
        css(top_px),
        css(bottom_px),
        css(right_px)
    )
}

/// `getRootWindowInsets()` reports the system bars whether or not the view is laid out under them,
/// so the numbers alone cannot say whether the page owes for them.
///
/// Android goes edge-to-edge for free only when the app targets SDK 35+ *and* runs on Android 15+.
/// Only `android-release` / `android-release-apk` patch `targetSdk = 36`, so every other build
/// (`dx serve` included) gets an already-inset window, where adding the bars counts them twice -
/// the dock floats one navigation bar too high, with a matching gap under the status bar.
///
/// Comparing content view against decor view settles it either way: equal heights mean the WebView
/// spans the bars and CSS owes them; a shorter content view means the system already carved them.
#[cfg_attr(not(target_os = "android"), allow(dead_code))]
fn visible_insets(
    top: i32,
    bottom: i32,
    right: i32,
    decor_h: i32,
    content_h: i32,
) -> (i32, i32, i32) {
    // `decor_h == 0` is a decor view that has not been measured yet; assume edge-to-edge rather
    // than flashing a zero-inset layout on the first frame.
    if decor_h > 0 && decor_h - content_h > 1 {
        (0, 0, 0)
    } else {
        (top, bottom, right)
    }
}
/// API 30+: `insets.getInsets(WindowInsets.Type.systemBars())` - what Android 15 edge-to-edge
/// reports, so the path that matters.
#[cfg(target_os = "android")]
fn insets_modern(
    env: &mut jni::JNIEnv,
    insets: &jni::objects::JObject,
) -> Option<(i32, i32, i32)> {
    let ty = env
        .call_static_method("android/view/WindowInsets$Type", "systemBars", "()I", &[])
        .ok()?
        .i()
        .ok()?;
    let ins = env
        .call_method(insets, "getInsets", "(I)Landroid/graphics/Insets;", &[
            jni::objects::JValue::Int(ty),
        ])
        .ok()?
        .l()
        .ok()?;
    Some((
        env.get_field(&ins, "top", "I").ok()?.i().ok()?,
        env.get_field(&ins, "bottom", "I").ok()?.i().ok()?,
        env.get_field(&ins, "right", "I").ok()?.i().ok()?,
    ))
}

/// API 24-29 fallback. Deprecated since 30 but never removed, and it covers `min_sdk = 24`.
#[cfg(target_os = "android")]
fn insets_legacy(
    env: &mut jni::JNIEnv,
    insets: &jni::objects::JObject,
) -> Option<(i32, i32, i32)> {
    Some((
        env.call_method(insets, "getSystemWindowInsetTop", "()I", &[]).ok()?.i().ok()?,
        env.call_method(insets, "getSystemWindowInsetBottom", "()I", &[]).ok()?.i().ok()?,
        env.call_method(insets, "getSystemWindowInsetRight", "()I", &[]).ok()?.i().ok()?,
    ))
}

/// None until the decor view is attached - the caller retries on the next resize.
#[cfg(target_os = "android")]
fn android_insets() -> Option<(i32, i32, i32, f32)> {
    let ctx = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.ok()?;
    let mut env = vm.attach_current_thread().ok()?;
    let activity = unsafe { jni::objects::JObject::from_raw(ctx.context().cast()) };

    let window = env.call_method(&activity, "getWindow", "()Landroid/view/Window;", &[]).ok()?.l().ok()?;
    let decor = env.call_method(&window, "getDecorView", "()Landroid/view/View;", &[]).ok()?.l().ok()?;
    let insets = env
        .call_method(&decor, "getRootWindowInsets", "()Landroid/view/WindowInsets;", &[])
        .ok()?
        .l()
        .ok()?;
    if insets.is_null() {
        return None;
    }

    let resources =
        env.call_method(&activity, "getResources", "()Landroid/content/res/Resources;", &[]).ok()?.l().ok()?;
    let metrics = env
        .call_method(&resources, "getDisplayMetrics", "()Landroid/util/DisplayMetrics;", &[])
        .ok()?
        .l()
        .ok()?;
    let density = env.get_field(&metrics, "density", "F").ok()?.f().ok()?;

    let (top, bottom, right) = match insets_modern(&mut env, &insets) {
        Some(v) => v,
        None => {
            // A missing method on API < 30 leaves a pending exception that would poison
            // every later JNI call.
            let _ = env.exception_clear();
            insets_legacy(&mut env, &insets)?
        }
    };
    // Is the WebView laid out under those bars? `android.R.id.content` (0x01020002) is the view
    // the activity fills; the system insets it, not the decor. See `visible_insets`.
    let content = env
        .call_method(&activity, "findViewById", "(I)Landroid/view/View;", &[jni::objects::JValue::Int(0x0102_0002)])
        .ok()?
        .l()
        .ok()?;
    let decor_h = env.call_method(&decor, "getHeight", "()I", &[]).ok()?.i().ok()?;
    let content_h = env.call_method(&content, "getHeight", "()I", &[]).ok()?.i().ok()?;
    let (top, bottom, right) = visible_insets(top, bottom, right, decor_h, content_h);

    Some((top, bottom, right, density))
}

/// `Configuration.uiMode & UI_MODE_NIGHT_MASK == UI_MODE_NIGHT_YES`. Any JNI failure reads as light,
/// the theme the app shipped with.
#[cfg(target_os = "android")]
fn system_is_dark() -> bool {
    fn ui_mode() -> Option<i32> {
        let ctx = ndk_context::android_context();
        let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.ok()?;
        let mut env = vm.attach_current_thread().ok()?;
        let activity = unsafe { jni::objects::JObject::from_raw(ctx.context().cast()) };
        let resources =
            env.call_method(&activity, "getResources", "()Landroid/content/res/Resources;", &[]).ok()?.l().ok()?;
        let config = env
            .call_method(&resources, "getConfiguration", "()Landroid/content/res/Configuration;", &[])
            .ok()?
            .l()
            .ok()?;
        env.get_field(&config, "uiMode", "I").ok()?.i().ok()
    }
    ui_mode().is_some_and(|mode| mode & 0x30 == 0x20)
}

#[cfg(target_os = "ios")]
fn system_is_dark() -> bool {
    use objc2::MainThreadMarker;
    use objc2_ui_kit::{UIScreen, UITraitEnvironment, UIUserInterfaceStyle};

    let Some(mtm) = MainThreadMarker::new() else {
        return false;
    };
    let style = unsafe { UIScreen::mainScreen(mtm).traitCollection().userInterfaceStyle() };
    style == UIUserInterfaceStyle::Dark
}

/// Sets COUNTED_DATA_DIR on Android/iOS so the ui crate's `storage_file()` picks it up.
#[cfg(not(feature = "server"))]
fn init_data_dir() -> String {
    #[cfg(target_os = "android")]
    {
        fn android_files_dir() -> Option<String> {
            let ctx = ndk_context::android_context();
            let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.ok()?;
            let mut env = vm.attach_current_thread().ok()?;
            let activity = unsafe { jni::objects::JObject::from_raw(ctx.context().cast()) };
            let files_dir = env
                .call_method(&activity, "getFilesDir", "()Ljava/io/File;", &[])
                .ok()?
                .l()
                .ok()?;
            let abs = env
                .call_method(&files_dir, "getAbsolutePath", "()Ljava/lang/String;", &[])
                .ok()?
                .l()
                .ok()?;
            let path: String = env.get_string(&jni::objects::JString::from(abs)).ok()?.into();
            Some(path)
        }
        // Every JNI step above is `.ok()?`, so any of them failing lands here. `"."` resolves to
        // `/` for an Android process - unwritable - which meant reads returned an empty store and
        // writes failed with nothing but a log line: the app would look like a fresh install and
        // silently refuse to persist a single project key. Still the fallback, because there is
        // nothing better to do, but it is now loud.
        let dir = android_files_dir().unwrap_or_else(|| {
            tracing::error!(
                "counted: getFilesDir() failed - COUNTED_DATA_DIR falls back to '.', which is not \
                 writable on Android. Nothing will persist."
            );
            ".".to_string()
        });
        std::env::set_var("COUNTED_DATA_DIR", &dir);
        return dir;
    }
    #[cfg(target_os = "ios")]
    {
        let dir = ios_data_dir().unwrap_or_else(|| {
            format!(
                "{}/Library/Application Support",
                std::env::var("HOME").unwrap_or_else(|_| ".".to_string())
            )
        });
        let _ = std::fs::create_dir_all(&dir);
        std::env::set_var("COUNTED_DATA_DIR", &dir);
        return dir;
    }
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    std::env::var("HOME").unwrap_or_else(|_| ".".to_string())
}

/// reqwest uses the process-wide rustls provider when one is installed and falls back to ring, which
/// offers no ML-KEM group. aws-lc offers X25519MLKEM768 first, so a recorded login cannot be
/// decrypted later by a quantum computer (SPEC 3.1).
#[cfg(not(feature = "server"))]
fn install_tls_provider() {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
}

/// A reqwest client with a file-backed cookie store, registered as the global Dioxus fullstack
/// client. Must be called before any server function.
#[cfg(not(feature = "server"))]
fn init_persistent_client(data_dir: &str) {
    use reqwest_cookie_store::{CookieStore, CookieStoreMutex};
    use std::sync::Arc;

    let cookie_path = std::path::PathBuf::from(data_dir).join("counted_cookies.json");

    #[allow(deprecated)]
    let store = std::fs::File::open(&cookie_path)
        .ok()
        .and_then(|f| CookieStore::load_json(std::io::BufReader::new(f)).ok())
        .unwrap_or_default();
    let store = Arc::new(CookieStoreMutex::new(store));

    // Sibling tmp file and rename, as `ui::common::persist` does: a kill mid-write leaves the
    // previous jar intact rather than a truncated file that loads as "signed out".
    let save = {
        let store = store.clone();
        let tmp = cookie_path.with_extension("json.tmp");
        move || {
            #[allow(deprecated)]
            let written = std::fs::File::create(&tmp)
                .and_then(|mut f| store.lock().unwrap().save_json(&mut f).map_err(std::io::Error::other))
                .and_then(|_| std::fs::rename(&tmp, &cookie_path));
            if written.is_err() {
                let _ = std::fs::remove_file(&tmp);
            }
        }
    };

    // Flushed on demand after sign-in and sign-out (`ui::common::flush_session`), so a process
    // killed right after either does not keep the account key on disk with no session beside it.
    // The timer stays as the fallback for every other cookie change.
    ui::common::set_session_flusher(Arc::new(save.clone()));
    ui::common::set_app_version(env!("CARGO_PKG_VERSION"));
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_secs(5));
        save();
    });

    // The store-visible version, so the server can refuse a build it no longer speaks to.
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        api::app_version::HEADER,
        reqwest::header::HeaderValue::from_static(env!("CARGO_PKG_VERSION")),
    );
    install_tls_provider();
    let client = reqwest::Client::builder()
        .cookie_provider(store)
        .default_headers(headers)
        .build()
        .unwrap();

    // Before the first server function call. GLOBAL_REQUEST_CLIENT is a pub OnceLock re-exported
    // via dioxus::fullstack.
    let _ = dioxus::fullstack::GLOBAL_REQUEST_CLIENT.set(client);
}

#[cfg(target_os = "android")]
fn platform_read_clipboard() -> Option<String> {
    fn inner() -> jni::errors::Result<String> {
        let ctx = ndk_context::android_context();
        let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }?;
        let mut env = vm.attach_current_thread()?;
        let activity = unsafe { jni::objects::JObject::from_raw(ctx.context().cast()) };
        let service = env.new_string("clipboard")?;
        let clipboard = env.call_method(
            &activity, "getSystemService", "(Ljava/lang/String;)Ljava/lang/Object;",
            &[(&service).into()],
        )?.l()?;
        let clip = env.call_method(
            &clipboard, "getPrimaryClip", "()Landroid/content/ClipData;", &[],
        )?.l()?;
        let item = env.call_method(
            &clip, "getItemAt", "(I)Landroid/content/ClipData$Item;",
            &[jni::objects::JValue::Int(0)],
        )?.l()?;
        let cs = env.call_method(&item, "getText", "()Ljava/lang/CharSequence;", &[])?.l()?;
        let s = env.call_method(&cs, "toString", "()Ljava/lang/String;", &[])?.l()?;
        let result: String = env.get_string(&jni::objects::JString::from(s))?.into();
        Ok(result)
    }
    inner().ok()
}

/// Reads the URL the activity started with and **consumes it**, so a later resume cannot replay it.
///
/// tao has no Android intent plumbing (wry#1563), so the intent is read off the activity over JNI.
/// `setData(null)` makes this a take rather than a peek: `getIntent()` otherwise returns the same
/// URI on every `Resumed` and returning from background would yank the user back to that project.
/// The same link still works twice - `MainActivity.onNewIntent` installs a fresh intent.
#[cfg(target_os = "android")]
fn android_take_intent_url() -> Option<String> {
    fn inner() -> jni::errors::Result<Option<String>> {
        let ctx = ndk_context::android_context();
        let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }?;
        let mut env = vm.attach_current_thread()?;
        let activity = unsafe { jni::objects::JObject::from_raw(ctx.context().cast()) };

        let intent =
            env.call_method(&activity, "getIntent", "()Landroid/content/Intent;", &[])?.l()?;
        if intent.is_null() {
            return Ok(None);
        }
        let data = env.call_method(&intent, "getDataString", "()Ljava/lang/String;", &[])?.l()?;
        if data.is_null() {
            return Ok(None);
        }
        let url: String = env.get_string(&jni::objects::JString::from(data))?.into();

        let null = jni::objects::JObject::null();
        env.call_method(&intent, "setData", "(Landroid/net/Uri;)Landroid/content/Intent;", &[
            (&null).into(),
        ])?;

        Ok(Some(url))
    }
    match inner() {
        Ok(url) => url,
        Err(e) => {
            tracing::warn!("could not read the launch intent: {e}");
            None
        }
    }
}

#[cfg(target_os = "ios")]
fn platform_read_clipboard() -> Option<String> {
    use objc2::rc::autoreleasepool;
    use objc2_ui_kit::UIPasteboard;
    unsafe { autoreleasepool(|_| UIPasteboard::generalPasteboard().string().map(|s| s.to_string())) }
}

/// The window's interface style, which the status bar, the keyboard and the WebView's
/// `prefers-color-scheme` all follow. Unspecified hands it back to the system.
#[cfg(target_os = "ios")]
#[allow(deprecated)]
fn apply_native_theme(theme: ui::common::Theme) {
    use objc2::MainThreadMarker;
    use objc2_ui_kit::{UIApplication, UIUserInterfaceStyle};
    use ui::common::Theme;

    let Some(mtm) = MainThreadMarker::new() else {
        return;
    };
    let style = match theme {
        Theme::Light => UIUserInterfaceStyle::Light,
        Theme::Dark => UIUserInterfaceStyle::Dark,
        Theme::System => UIUserInterfaceStyle::Unspecified,
    };
    if let Some(window) = UIApplication::sharedApplication(mtm).keyWindow() {
        window.setOverrideUserInterfaceStyle(style);
    }
}

/// The status and navigation bar icons, which `values(-night)/styles.xml` set from the system's
/// mode. Through `MainActivity.setLightSystemBars`, which needs the UI thread.
#[cfg(target_os = "android")]
fn apply_native_theme(theme: ui::common::Theme) {
    fn call(env: &mut jni::JNIEnv, light: bool) -> jni::errors::Result<()> {
        let ctx = ndk_context::android_context();
        let activity = unsafe { jni::objects::JObject::from_raw(ctx.context().cast()) };
        env.call_method(&activity, "setLightSystemBars", "(Z)V", &[jni::objects::JValue::Bool(light.into())])?;
        Ok(())
    }

    let light = !theme.is_dark(system_is_dark());
    let ctx = ndk_context::android_context();
    let Ok(vm) = (unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }) else {
        return;
    };
    let Ok(mut env) = vm.attach_current_thread() else {
        return;
    };
    if let Err(e) = call(&mut env, light) {
        let _ = env.exception_clear();
        tracing::debug!("status bar appearance failed: {e}");
    }
}

/// UIKit's own feedback vocabulary. A generator is built per call rather than cached, and fired
/// at once: `prepare()` only pays off ahead of a later event, so right before firing it is a wasted
/// wake of the Taptic Engine, and holding a generator across calls would mean holding a
/// `MainThreadOnly` object in a `Send + Sync` closure.
///
/// `MainThreadMarker::new()` returning `None` is the guard - UIFeedbackGenerator is main-thread
/// only, and a haptic is never worth a crash. `initWithStyle`/`init` are deprecated in favour of
/// the `forView:` constructors, which need a `UIView` this layer has no handle on.
#[cfg(target_os = "ios")]
#[allow(deprecated)]
fn play_haptic(kind: ui::common::Haptic) {
    use objc2::{MainThreadMarker, MainThreadOnly};
    use objc2_ui_kit::{
        UIImpactFeedbackGenerator, UIImpactFeedbackStyle, UINotificationFeedbackGenerator,
        UINotificationFeedbackType, UISelectionFeedbackGenerator,
    };
    use ui::common::Haptic;

    let Some(mtm) = MainThreadMarker::new() else {
        return;
    };
    match kind {
        Haptic::Selection => UISelectionFeedbackGenerator::new(mtm).selectionChanged(),
        Haptic::Light | Haptic::Medium => {
            let style = if matches!(kind, Haptic::Light) {
                UIImpactFeedbackStyle::Light
            } else {
                UIImpactFeedbackStyle::Medium
            };
            UIImpactFeedbackGenerator::initWithStyle(UIImpactFeedbackGenerator::alloc(mtm), style)
                .impactOccurred();
        }
        Haptic::Success | Haptic::Warning | Haptic::Error => {
            let ty = match kind {
                Haptic::Success => UINotificationFeedbackType::Success,
                Haptic::Warning => UINotificationFeedbackType::Warning,
                _ => UINotificationFeedbackType::Error,
            };
            UINotificationFeedbackGenerator::init(UINotificationFeedbackGenerator::alloc(mtm))
                .notificationOccurred(ty);
        }
    }
}

/// `View.performHapticFeedback` on the decor view, not `Vibrator`: it needs no permission and it
/// respects the user's "Touch feedback" system setting, which a raw vibration would override.
///
/// The constants are `HapticFeedbackConstants`: SEGMENT_TICK (27, API 34+) or CLOCK_TICK (4) below
/// for a selection, LONG_PRESS (0) for a committed gesture, CONFIRM (16) / REJECT (17) for the
/// outcome pair. The selection constant is picked by `SDK_INT` because an unknown one plays
/// nothing. CONFIRM/REJECT are API 30+; the call simply does nothing on older devices, which is the
/// right failure mode for feedback. `Light` plays nothing: Material controls do not buzz on a plain
/// tap, which is all `Light` marks (D11 in docs/plans/native-feel.md).
#[cfg(target_os = "android")]
fn play_haptic(kind: ui::common::Haptic) {
    use ui::common::Haptic;

    if kind == Haptic::Light {
        return;
    }

    fn call(env: &mut jni::JNIEnv, kind: Haptic) -> jni::errors::Result<()> {
        let constant = match kind {
            Haptic::Selection => {
                let sdk = env.get_static_field("android/os/Build$VERSION", "SDK_INT", "I")?.i()?;
                if sdk >= 34 { 27 } else { 4 }
            }
            Haptic::Light | Haptic::Medium => 0,
            Haptic::Success => 16,
            Haptic::Warning | Haptic::Error => 17,
        };
        let ctx = ndk_context::android_context();
        let activity = unsafe { jni::objects::JObject::from_raw(ctx.context().cast()) };
        let window = env.call_method(&activity, "getWindow", "()Landroid/view/Window;", &[])?.l()?;
        let decor = env.call_method(&window, "getDecorView", "()Landroid/view/View;", &[])?.l()?;
        env.call_method(&decor, "performHapticFeedback", "(I)Z", &[jni::objects::JValue::Int(
            constant,
        )])?;
        Ok(())
    }

    let ctx = ndk_context::android_context();
    let Ok(vm) = (unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }) else {
        return;
    };
    let Ok(mut env) = vm.attach_current_thread() else {
        return;
    };
    if let Err(e) = call(&mut env, kind) {
        // Always clear: a pending exception poisons every later JNI call on this thread - the
        // inset reads above included.
        let _ = env.exception_clear();
        tracing::debug!("haptic failed: {e}");
    }
}

/// `ACTION_SEND` wrapped in `Intent.createChooser`, started from the activity itself - no
/// `FLAG_ACTIVITY_NEW_TASK` needed, and `startActivity` is a Binder call that is fine off the Java
/// UI thread, like every other JNI call here.
#[cfg(target_os = "android")]
fn share_text(text: &str) -> bool {
    fn inner(env: &mut jni::JNIEnv, text: &str) -> jni::errors::Result<()> {
        let ctx = ndk_context::android_context();
        let activity = unsafe { jni::objects::JObject::from_raw(ctx.context().cast()) };
        let action = env.new_string("android.intent.action.SEND")?;
        let intent = env.new_object(
            "android/content/Intent",
            "(Ljava/lang/String;)V",
            &[(&action).into()],
        )?;
        let mime = env.new_string("text/plain")?;
        env.call_method(
            &intent,
            "setType",
            "(Ljava/lang/String;)Landroid/content/Intent;",
            &[(&mime).into()],
        )?;
        let extra = env.new_string("android.intent.extra.TEXT")?;
        let value = env.new_string(text)?;
        env.call_method(
            &intent,
            "putExtra",
            "(Ljava/lang/String;Ljava/lang/String;)Landroid/content/Intent;",
            &[(&extra).into(), (&value).into()],
        )?;
        let null = jni::objects::JObject::null();
        let chooser = env
            .call_static_method(
                "android/content/Intent",
                "createChooser",
                "(Landroid/content/Intent;Ljava/lang/CharSequence;)Landroid/content/Intent;",
                &[(&intent).into(), (&null).into()],
            )?
            .l()?;
        env.call_method(
            &activity,
            "startActivity",
            "(Landroid/content/Intent;)V",
            &[(&chooser).into()],
        )?;
        Ok(())
    }

    let ctx = ndk_context::android_context();
    let Ok(vm) = (unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }) else {
        return false;
    };
    let Ok(mut env) = vm.attach_current_thread() else {
        return false;
    };
    match inner(&mut env, text) {
        Ok(()) => true,
        Err(e) => {
            // Always clear: a pending exception poisons every later JNI call on this thread.
            let _ = env.exception_clear();
            tracing::warn!("share sheet failed: {e}");
            false
        }
    }
}

/// `UIActivityViewController` presented from the key window's root controller. Native rather than
/// `navigator.share` through eval: a click reaches Rust over the WKScriptMessageHandler channel,
/// asynchronously, so the JS would run outside the click's activation that WebKit gates `share()`
/// on. On iPad the popover must have a source view or UIKit aborts.
#[cfg(target_os = "ios")]
fn share_text(text: &str) -> bool {
    use objc2::rc::Retained;
    use objc2_foundation::NSString;
    present_share_sheet(Retained::into_super(Retained::into_super(NSString::from_str(text))))
}

#[cfg(target_os = "ios")]
#[allow(deprecated)]
fn present_share_sheet(item: objc2::rc::Retained<objc2::runtime::AnyObject>) -> bool {
    use objc2::{MainThreadMarker, MainThreadOnly};
    use objc2_foundation::NSArray;
    use objc2_ui_kit::{UIActivityViewController, UIApplication};

    let Some(mtm) = MainThreadMarker::new() else {
        return false;
    };
    let Some(root) = UIApplication::sharedApplication(mtm)
        .keyWindow()
        .and_then(|w| w.rootViewController())
    else {
        return false;
    };
    let items = NSArray::from_retained_slice(&[item]);
    let vc = unsafe {
        UIActivityViewController::initWithActivityItems_applicationActivities(
            UIActivityViewController::alloc(mtm),
            &items,
            None,
        )
    };
    centre_popover(&vc, &root);
    root.presentViewController_animated_completion(&vc, true, None);
    true
}

/// iPad: a share sheet or an action sheet is a popover, which needs a source or UIKit aborts.
/// Centred over the app with no arrow, since nothing on screen is its anchor. A no-op on iPhone.
#[cfg(target_os = "ios")]
fn centre_popover(vc: &objc2_ui_kit::UIViewController, root: &objc2_ui_kit::UIViewController) {
    use objc2_ui_kit::UIPopoverArrowDirection;

    if let Some(popover) = vc.popoverPresentationController() {
        let view = root.view();
        if let Some(view) = view.as_deref() {
            let mut centre = view.bounds();
            centre.origin.x += centre.size.width / 2.0;
            centre.origin.y += centre.size.height / 2.0;
            centre.size.width = 0.0;
            centre.size.height = 0.0;
            popover.setSourceRect(centre);
        }
        popover.setSourceView(view.as_deref());
        popover.setPermittedArrowDirections(UIPopoverArrowDirection(0));
    }
}

/// The app's confirm as a `UIAlertController` (`ui::common::ConfirmDialog`): an alert for a
/// question, an action sheet for removing something, presented from the topmost controller (the
/// root may already present the share sheet). Each action's block replies once with its index,
/// Cancel with `None`; the shared `Option` makes the reply single-use. False when there is no
/// window, and the HTML dialog renders instead.
#[cfg(target_os = "ios")]
#[allow(deprecated)]
fn present_confirm(request: ui::common::ConfirmRequest, reply: ui::common::ConfirmReply) -> bool {
    use block2::RcBlock;
    use objc2::MainThreadMarker;
    use objc2_foundation::NSString;
    use objc2_ui_kit::{
        UIAlertAction, UIAlertActionStyle, UIAlertController, UIAlertControllerStyle, UIApplication,
    };
    use std::cell::RefCell;
    use std::ptr::NonNull;
    use std::rc::Rc;

    let Some(mtm) = MainThreadMarker::new() else {
        return false;
    };
    let Some(mut top) = UIApplication::sharedApplication(mtm)
        .keyWindow()
        .and_then(|w| w.rootViewController())
    else {
        return false;
    };
    while let Some(presented) = top.presentedViewController() {
        top = presented;
    }

    let style = if request.action_sheet {
        UIAlertControllerStyle::ActionSheet
    } else {
        UIAlertControllerStyle::Alert
    };
    let alert = UIAlertController::alertControllerWithTitle_message_preferredStyle(
        Some(&NSString::from_str(&request.title)),
        Some(&NSString::from_str(&request.message)),
        style,
        mtm,
    );

    let reply = Rc::new(RefCell::new(Some(reply)));
    let buttons = request
        .actions
        .iter()
        .enumerate()
        .map(|(i, (label, destructive))| {
            let style = if *destructive { UIAlertActionStyle::Destructive } else { UIAlertActionStyle::Default };
            (label.as_str(), style, Some(i))
        })
        .chain(std::iter::once((request.cancel.as_str(), UIAlertActionStyle::Cancel, None)));
    for (label, style, choice) in buttons {
        let reply = reply.clone();
        let handler = RcBlock::new(move |_: NonNull<UIAlertAction>| {
            if let Some(send) = reply.borrow_mut().take() {
                send(choice);
            }
        });
        let action = UIAlertAction::actionWithTitle_style_handler(
            Some(&NSString::from_str(label)),
            style,
            Some(&*handler),
            mtm,
        );
        alert.addAction(&action);
    }

    centre_popover(&alert, &top);
    top.presentViewController_animated_completion(&alert, true, None);
    true
}

/// Writes an export where the share sheet can reach it, replacing the previous one. `filename` is
/// reduced to its last component, so it can never climb out of `dir`.
#[cfg(any(target_os = "android", target_os = "ios"))]
fn write_export(dir: &std::path::Path, filename: &str, bytes: &[u8]) -> Result<std::path::PathBuf, String> {
    let name = std::path::Path::new(filename).file_name().ok_or("empty file name")?;
    let _ = std::fs::remove_dir_all(dir);
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let path = dir.join(name);
    std::fs::write(&path, bytes).map_err(|e| e.to_string())?;
    Ok(path)
}

/// `MainActivity.openUri` (packages/mobile/android-kotlin/MainActivity.kt): `ACTION_SENDTO` for a
/// `mailto:`, false when no app takes it.
#[cfg(target_os = "android")]
fn open_url(url: &str) -> bool {
    fn inner(env: &mut jni::JNIEnv, url: &str) -> jni::errors::Result<bool> {
        let ctx = ndk_context::android_context();
        let activity = unsafe { jni::objects::JObject::from_raw(ctx.context().cast()) };
        let url = env.new_string(url)?;
        env.call_method(&activity, "openUri", "(Ljava/lang/String;)Z", &[(&url).into()])?.z()
    }

    let ctx = ndk_context::android_context();
    let Ok(vm) = (unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }) else {
        return false;
    };
    let Ok(mut env) = vm.attach_current_thread() else {
        return false;
    };
    match inner(&mut env, url) {
        Ok(opened) => opened,
        Err(e) => {
            let _ = env.exception_clear();
            tracing::warn!("counted: openUri failed: {e}");
            false
        }
    }
}

#[cfg(target_os = "android")]
fn cache_dir(env: &mut jni::JNIEnv) -> jni::errors::Result<String> {
    let ctx = ndk_context::android_context();
    let activity = unsafe { jni::objects::JObject::from_raw(ctx.context().cast()) };
    let dir = env.call_method(&activity, "getCacheDir", "()Ljava/io/File;", &[])?.l()?;
    let abs = env.call_method(&dir, "getAbsolutePath", "()Ljava/lang/String;", &[])?.l()?;
    Ok(env.get_string(&jni::objects::JString::from(abs))?.into())
}

/// An export is a plaintext ledger. The share sheet is long done with the last one by the next
/// launch, and left there it would outlive logout and leaving the project.
#[cfg(target_os = "android")]
fn purge_exports() {
    let ctx = ndk_context::android_context();
    let Ok(vm) = (unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }) else {
        return;
    };
    let Ok(mut env) = vm.attach_current_thread() else {
        return;
    };
    match cache_dir(&mut env) {
        Ok(cache) => {
            let _ = std::fs::remove_dir_all(std::path::Path::new(&cache).join("exports"));
        }
        Err(_) => {
            let _ = env.exception_clear();
        }
    }
}

#[cfg(target_os = "ios")]
fn purge_exports() {
    let _ = std::fs::remove_dir_all(std::env::temp_dir().join("exports"));
}

/// Under `getCacheDir()/exports/`, the one directory `res/xml/file_paths.xml` exposes for this,
/// then `MainActivity.shareFile` builds the FileProvider URI and opens the chooser.
#[cfg(target_os = "android")]
fn share_file(filename: &str, bytes: &[u8], mime: &str) -> Result<(), String> {
    fn share(env: &mut jni::JNIEnv, path: &str, mime: &str) -> jni::errors::Result<()> {
        let ctx = ndk_context::android_context();
        let activity = unsafe { jni::objects::JObject::from_raw(ctx.context().cast()) };
        let path = env.new_string(path)?;
        let mime = env.new_string(mime)?;
        env.call_method(
            &activity,
            "shareFile",
            "(Ljava/lang/String;Ljava/lang/String;)V",
            &[(&path).into(), (&mime).into()],
        )?;
        Ok(())
    }

    let ctx = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.map_err(|e| e.to_string())?;
    let mut env = vm.attach_current_thread().map_err(|e| e.to_string())?;
    let result = cache_dir(&mut env).map_err(|e| e.to_string()).and_then(|cache| {
        let path = write_export(&std::path::Path::new(&cache).join("exports"), filename, bytes)?;
        share(&mut env, &path.to_string_lossy(), mime).map_err(|e| e.to_string())
    });
    if result.is_err() {
        let _ = env.exception_clear();
    }
    result
}

/// `openURL:options:completionHandler:`, not `openURL:` - the iOS 18 SDK turns the deprecated one
/// into a logged no-op that returns false.
#[cfg(target_os = "ios")]
fn open_url(url: &str) -> bool {
    use objc2::MainThreadMarker;
    use objc2_foundation::{NSDictionary, NSString, NSURL};
    use objc2_ui_kit::UIApplication;

    let Some(mtm) = MainThreadMarker::new() else {
        return false;
    };
    let Some(url) = NSURL::URLWithString(&NSString::from_str(url)) else {
        return false;
    };
    unsafe {
        UIApplication::sharedApplication(mtm).openURL_options_completionHandler(
            &url,
            &NSDictionary::new(),
            None,
        );
    }
    true
}

/// A file URL in the share sheet: Save to Files, Mail, AirDrop. Under the app's tmp directory,
/// which iOS may purge on its own.
#[cfg(target_os = "ios")]
fn share_file(filename: &str, bytes: &[u8], _mime: &str) -> Result<(), String> {
    use objc2::rc::Retained;
    use objc2_foundation::{NSString, NSURL};

    let path = write_export(&std::env::temp_dir().join("exports"), filename, bytes)?;
    let url = NSURL::fileURLWithPath(&NSString::from_str(&path.to_string_lossy()));
    if present_share_sheet(Retained::into_super(Retained::into_super(url))) {
        Ok(())
    } else {
        Err("no window to present the share sheet from".into())
    }
}

/// Bridges `ocr::scan` to the shape `ui` asks for, and installs the platform's temp-file purge.
///
/// The adapter lives here rather than in either crate: `ocr` must not know about Dioxus, and `ui`
/// must not depend on `ocr` - 31 MB of weights would land in the wasm bundle and the server binary.
/// Same seam as `set_haptics_player` below.
#[cfg(any(target_os = "android", target_os = "ios"))]
fn install_scanner() {
    use std::sync::Arc;
    use ui::common::{NativeScan, ScanError, ScanFields};

    ui::common::set_native_scan(NativeScan {
        recognise: Arc::new(|jpeg: Vec<u8>| match ocr::scan(&jpeg) {
            Ok(receipt) => Ok(ScanFields {
                title: receipt.title,
                amount: receipt.amount,
                date: receipt.date.map(|d| d.format("%Y-%m-%d").to_string()),
                amount_confident: receipt.amount_confident,
            }),
            // Every failure reads the same to the user: nothing usable on that photo. The
            // distinction between "not a JPEG" and "no text found" is a log line, not a message.
            Err(e) => {
                tracing::debug!("ocr failed: {e:?}");
                Err(ScanError::Unreadable)
            }
        }),
        arm: Arc::new(arm_capture_purge),
        purge: Arc::new(purge_captures),
    });

    #[cfg(target_os = "android")]
    warn_if_no_file_provider();
}

/// wry builds the capture URI from `packageName + ".fileprovider"`. With no such provider declared,
/// `FileProvider.getUriForFile` throws, `RustWebChromeClient` swallows it in `catch (ex: Exception)`
/// and falls back to the document picker - so **Scan opens the gallery instead of the camera**, and
/// the only trace is a `Tauri/FileChooser` log line from inside wry.
///
/// `dx serve` can never carry the provider: dx 0.7.9 interpolates `[android.raw.manifest]` at
/// `<manifest>` level, and a `<provider>` must be a child of `<application>`. Only the Makefile's
/// `patch_file_provider` - i.e. `make android-build` - puts it there. This turns that structural gap
/// into one obvious line at startup instead of a mystery on the device.
///
/// The button is deliberately left enabled: scanning a photo already in the gallery is a legitimate
/// flow, just not the one that was asked for.
#[cfg(target_os = "android")]
fn warn_if_no_file_provider() {
    fn inner(env: &mut jni::JNIEnv) -> jni::errors::Result<bool> {
        let ctx = ndk_context::android_context();
        let activity = unsafe { jni::objects::JObject::from_raw(ctx.context().cast()) };
        let pm = env
            .call_method(
                &activity,
                "getPackageManager",
                "()Landroid/content/pm/PackageManager;",
                &[],
            )?
            .l()?;
        let package =
            env.call_method(&activity, "getPackageName", "()Ljava/lang/String;", &[])?.l()?;
        let package: String = env.get_string(&jni::objects::JString::from(package))?.into();
        // Derived exactly the way wry derives it, so an applicationIdSuffix cannot slip past this -
        // the Makefile's grep hardcodes the authority and would not notice.
        let authority = env.new_string(format!("{package}.fileprovider"))?;
        let info = env
            .call_method(
                &pm,
                "resolveContentProvider",
                "(Ljava/lang/String;I)Landroid/content/pm/ProviderInfo;",
                &[(&authority).into(), jni::objects::JValue::Int(0)],
            )?
            .l()?;
        Ok(!info.is_null())
    }

    let ctx = ndk_context::android_context();
    let Ok(vm) = (unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }) else {
        return;
    };
    let Ok(mut env) = vm.attach_current_thread() else {
        return;
    };
    match inner(&mut env) {
        Ok(true) => {}
        Ok(false) => tracing::warn!(
            "no capture FileProvider declared: Scan will open the gallery, not the camera. \
             Expected under `dx serve` - build with `make android-build` to test capture."
        ),
        Err(e) => {
            let _ = env.exception_clear();
            tracing::warn!("could not check for the capture FileProvider: {e}");
        }
    }
}

/// Android's capture directory is the app's own and nothing else writes there, so there is nothing
/// to snapshot.
#[cfg(target_os = "android")]
fn arm_capture_purge() {}

/// wry's `RustWebChromeClient.createImageFile` writes the captured photo as `JPEG_<ts>_*.jpg` into
/// `getExternalFilesDir(Environment.DIRECTORY_PICTURES)` and never removes it. Nothing else in this
/// app writes there, so emptying the directory is safe and is what makes "the photo is not saved
/// anywhere" true rather than aspirational.
#[cfg(target_os = "android")]
fn purge_captures() {
    fn inner(env: &mut jni::JNIEnv) -> jni::errors::Result<()> {
        let ctx = ndk_context::android_context();
        let activity = unsafe { jni::objects::JObject::from_raw(ctx.context().cast()) };
        let kind = env.new_string("Pictures")?;
        let dir = env
            .call_method(
                &activity,
                "getExternalFilesDir",
                "(Ljava/lang/String;)Ljava/io/File;",
                &[(&kind).into()],
            )?
            .l()?;
        if dir.is_null() {
            return Ok(());
        }
        let files = env.call_method(&dir, "listFiles", "()[Ljava/io/File;", &[])?.l()?;
        if files.is_null() {
            return Ok(());
        }
        let files: jni::objects::JObjectArray = files.into();
        for i in 0..env.get_array_length(&files)? {
            let file = env.get_object_array_element(&files, i)?;
            env.call_method(&file, "delete", "()Z", &[])?;
        }
        Ok(())
    }

    let ctx = ndk_context::android_context();
    let Ok(vm) = (unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }) else {
        return;
    };
    let Ok(mut env) = vm.attach_current_thread() else {
        return;
    };
    if let Err(e) = inner(&mut env) {
        // Always clear: a pending exception poisons every later JNI call on this thread - the
        // safe-area inset reads included.
        let _ = env.exception_clear();
        tracing::warn!("could not purge captured photos: {e}");
    }
}

/// iOS shares `NSTemporaryDirectory()` between WKWebView's copy of the pick and WebKit's own
/// scratch files, so it cannot be emptied wholesale. Snapshot before the pick; delete only what is
/// new afterwards.
#[cfg(target_os = "ios")]
static CAPTURE_SNAPSHOT: std::sync::Mutex<Option<std::collections::HashSet<String>>> =
    std::sync::Mutex::new(None);

#[cfg(target_os = "ios")]
fn temp_dir_entries() -> Option<(std::path::PathBuf, std::collections::HashSet<String>)> {
    let dir = std::path::PathBuf::from(std::env::temp_dir());
    let names = std::fs::read_dir(&dir)
        .ok()?
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    Some((dir, names))
}

#[cfg(target_os = "ios")]
fn arm_capture_purge() {
    if let Some((_, names)) = temp_dir_entries() {
        if let Ok(mut guard) = CAPTURE_SNAPSHOT.lock() {
            *guard = Some(names);
        }
    }
}

/// Only entries that appeared during the pick, and only ones that look like an image. That cannot
/// touch anything WebKit still needs, and it needs no knowledge of WebKit's naming.
#[cfg(target_os = "ios")]
fn purge_captures() {
    const IMAGE_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "heic", "heif"];

    let Some((dir, now)) = temp_dir_entries() else {
        return;
    };
    let before = CAPTURE_SNAPSHOT.lock().ok().and_then(|mut g| g.take());
    let Some(before) = before else {
        return;
    };
    for name in now.difference(&before) {
        let path = dir.join(name);
        let is_image = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_ascii_lowercase())
            .is_some_and(|e| IMAGE_EXTENSIONS.contains(&e.as_str()));
        if is_image {
            let _ = std::fs::remove_file(&path);
        }
    }
}

fn main() {
    dioxus::logger::initialize_default();
    #[cfg(not(feature = "server"))]
    {
        // Baked in at compile time: a process env var cannot be set for an Android activity before
        // main() runs, so a runtime lookup always misses. build.rs declares the
        // rerun-if-env-changed, without which cargo reuses a stale bake.
        dioxus::fullstack::set_server_url(
            option_env!("COUNTED_SERVER_URL").unwrap_or("https://counted.fr"),
        );
        let data_dir = init_data_dir();
        init_persistent_client(&data_dir);
    }

    // Before launch, so no call site can race an unset player. Every other target leaves it unset
    // and `ui::common::haptic` is a no-op.
    #[cfg(any(target_os = "android", target_os = "ios"))]
    ui::common::set_haptics_player(std::sync::Arc::new(play_haptic));

    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        ui::common::set_native_sharer(std::sync::Arc::new(share_text));
        ui::common::set_native_opener(std::sync::Arc::new(open_url));
        ui::common::set_native_file_sharer(std::sync::Arc::new(share_file));
        purge_exports();
    }

    // Same contract: installed before launch, so no call site can race an unset scanner. Every
    // other target leaves it unset and the Scan button does not render.
    #[cfg(any(target_os = "android", target_os = "ios"))]
    install_scanner();

    #[cfg(any(target_os = "android", target_os = "ios"))]
    push::install();

    #[cfg(any(target_os = "android", target_os = "ios"))]
    let theme = ui::common::stored_theme();
    #[cfg(any(target_os = "android", target_os = "ios"))]
    ui::common::set_native_theme(std::sync::Arc::new(apply_native_theme));
    #[cfg(target_os = "ios")]
    ui::common::set_native_confirm(std::sync::Arc::new(present_confirm));

    let builder = dioxus::LaunchBuilder::new();
    // `disable_context_menu` is false only in debug builds: false is dioxus-desktop's debug mode,
    // which calls `with_devtools(true)` - the socket the Android e2e suite attaches to - and with
    // wry's `devtools` feature, which dioxus's defaults turn on, a release build would ship with
    // WebView debugging on (audit 2026-10-19 M1). The `contextmenu` preventDefault that true
    // injects would kill the native text-selection toolbar, so `head_html` stops that event
    // before it reaches the listener.
    //
    // The event handler covers only deep links arriving BEFORE the UI exists - installed ahead of
    // the event loop, so a cold-start link cannot race the first render. `DeepLinkListener` then
    // takes over; dioxus
    // dispatches registered wry handlers first, so without the stand-down both would handle the
    // same link. See packages/ui/src/common/deep_link.rs.
    #[cfg(any(target_os = "android", target_os = "ios"))]
    let builder = builder.with_cfg(
        dioxus::mobile::Config::new()
            // wry paints this before parsing any HTML. The window shows right after the first
            // rebuild's edits are *sent*, unflushed, so one frame has an empty document - this
            // makes it base-200 rather than white. sRGB of --color-base-200, of the chosen theme.
            .with_background_color(if theme.is_dark(system_is_dark()) { (27, 34, 31, 255) } else { (244, 248, 246, 255) })
            .with_custom_head(head_html(theme))
            .with_disable_context_menu(cfg!(not(debug_assertions)))
            .with_custom_event_handler(|event, _| {
                use dioxus::mobile::tao::event::Event;
                if ui::common::deep_link_listener_active() {
                    return;
                }
                match event {
                    // iOS delivers BOTH counted:// (application:openURL:) and universal links
                    // (application:continueUserActivity:) as Opened - see tao ios/view.rs.
                    Event::Opened { urls } => {
                        for url in urls {
                            push_deep_link(url.to_string());
                        }
                    }
                    // Android has no such event; the intent is read on every resume instead.
                    #[cfg(target_os = "android")]
                    Event::Resumed => {
                        if let Some(url) = android_take_intent_url() {
                            push_deep_link(url);
                        }
                    }
                    _ => {}
                }
            }),
    );
    builder.launch(app);
}

#[component]
fn app() -> Element {
    use_app_contexts();
    let ready = use_client_ready();
    // The window and its status bar exist only once mounted; the page took the theme from the head.
    #[cfg(any(target_os = "android", target_os = "ios"))]
    use_effect(|| apply_native_theme(ui::common::stored_theme()));
    // WKWebView's own left-edge back gesture stays off (its default): it slid a snapshot of the
    // same page, then the CSS pop played on top. The interactive swipe is the JS edge strip in
    // `ui::common::page_transition` (docs/mobile-navigation.md, "iOS Swipe Back").
    #[cfg(target_os = "ios")]
    use_effect(|| {
        use dioxus::mobile::wry::WebViewExtIOS;
        // Deferred to an effect rather than done at launch: the webview does not exist until the
        // window is built.
        let webview = dioxus::mobile::window().webview.webview();
        set_launch_background(&*webview);
        status_bar_tap::install(&*webview);
        keyboard_signal::install(unsafe { objc2::rc::Retained::cast_unchecked(webview.clone()) });
        voiceover_escape::install(unsafe { objc2::rc::Retained::cast_unchecked(webview) });
        hide_form_accessory_bar();
    });

    // Android's WebView maps only display cutouts into env(safe-area-inset-*), never the status or
    // navigation bars, so those stay 0px and every rule consuming them is a no-op. Read the real
    // insets over JNI, driven by the WebView's resize event so rotation and nav-mode changes
    // cannot leave stale values.
    #[cfg(target_os = "android")]
    use_effect(|| {
        let mut eval = document::eval(
            r#"
            const tick = () => dioxus.send("tick");
            window.addEventListener("resize", tick);
            tick();
        "#,
        );
        // The WebView fires `resize` on every keyboard step too, with the same insets; rewriting
        // them would restyle the whole document mid-keyboard for nothing.
        spawn(async move {
            let mut written = String::new();
            while eval.recv::<String>().await.is_ok() {
                if let Some((top, bottom, right, density)) = android_insets() {
                    let js = css_inset_vars(top, bottom, right, density);
                    if js == written {
                        continue;
                    }
                    tracing::info!(
                        "android safe-area insets: top={top} bottom={bottom} right={right} density={density}"
                    );
                    document::eval(&js);
                    written = js;
                }
            }
        });
    });
    use_context_provider(|| {
        #[cfg(any(target_os = "android", target_os = "ios"))]
        { Some::<NativeClipboardReader>(std::sync::Arc::new(platform_read_clipboard)) }
        #[cfg(not(any(target_os = "android", target_os = "ios")))]
        { None::<NativeClipboardReader> }
    });

    // Android reads its deep link off the activity's intent; iOS gets URLs from tao directly.
    use_context_provider(|| {
        #[cfg(target_os = "android")]
        { Some(NativeDeepLinkReader(std::sync::Arc::new(android_take_intent_url))) }
        #[cfg(not(target_os = "android"))]
        { None::<NativeDeepLinkReader> }
    });

    rsx! {
        // Duplicated by head_html() on android/ios (same URLs, cached - no refetch, no repaint).
        // Kept because with_custom_head is cfg-gated to those targets, so dropping these would
        // strip the CSS from any other build of this package.
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: ui::common::NATIVE_FEEL_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Style { {font_face_css()} }

        if !ready() {
            SplashScreen { logo: LOGO }
        }
        UpdateRequiredScreen { store_url: STORE_URL }

        main {
            class: "min-h-screen flex flex-col items-center",
            Router::<Route> {}
        }
    }
}

/// `evaluateJavaScript:completionHandler:` from a UIKit callback, where no Dioxus runtime is current
/// for `document::eval`. A raw selector, like `set_launch_background`'s: wry's own iOS `WKWebView`
/// binding is not public.
#[cfg(target_os = "ios")]
fn evaluate_js(webview: &impl objc2::Message, js: &str) {
    use objc2::msg_send;
    use objc2::runtime::AnyObject;
    use objc2_foundation::{NSError, NSString};

    let done: Option<&block2::Block<dyn Fn(*mut AnyObject, *mut NSError)>> = None;
    unsafe {
        let _: () = msg_send![webview, evaluateJavaScript: &*NSString::from_str(js), completionHandler: done];
    }
}

/// The keyboard's own signal (docs/plans/ios-sheets-keyboard.md G1). iOS posts
/// `UIKeyboardWillChangeFrameNotification` before the keyboard moves, with the end frame, the
/// duration and the curve; `ui::common::keyboard` turns it into a prediction the `visualViewport`
/// events then confirm or correct. The height is the end frame converted into the webview's own
/// coordinates and measured from the webview's bottom edge, 0 unless the keyboard is docked there
/// (a floating iPad keyboard, a hardware keyboard's strip); the JS applies its own 80 px floor and
/// drops a non-local or inactive-state notification. Installed once (the returned token is kept,
/// the centre holds observers weakly), with no queue, so the block runs inside UIKit's post on the
/// main thread, before WebKit's own observer has scheduled anything. Not compiled on Windows: a
/// scratch crate type-checked it for `aarch64-apple-ios`, the Codemagic build is the proof, and
/// the two `tracing` lines (install, each notification) are how to read it on device.
#[cfg(target_os = "ios")]
mod keyboard_signal {
    use objc2::rc::Retained;
    use objc2::runtime::{NSObjectProtocol, ProtocolObject};
    use objc2::MainThreadMarker;
    use objc2_foundation::{NSNotification, NSNotificationCenter, NSNumber, NSString, NSValue};
    use objc2_ui_kit::{
        NSValueUIGeometryExtensions, UIApplication, UIApplicationState, UIKeyboardAnimationCurveUserInfoKey,
        UIKeyboardAnimationDurationUserInfoKey, UIKeyboardFrameEndUserInfoKey, UIKeyboardIsLocalUserInfoKey,
        UIKeyboardWillChangeFrameNotification, UIView,
    };
    use std::ptr::NonNull;
    use std::time::Instant;

    thread_local! {
        static TOKEN: std::cell::OnceCell<Retained<ProtocolObject<dyn NSObjectProtocol>>> = const { std::cell::OnceCell::new() };
    }

    pub fn install(webview: Retained<UIView>) {
        let Some(mtm) = MainThreadMarker::new() else {
            return;
        };
        if TOKEN.with(|t| t.get().is_some()) {
            return;
        }
        let start = Instant::now();
        let block = block2::RcBlock::new(move |note: NonNull<NSNotification>| {
            let note = unsafe { note.as_ref() };
            let Some(info) = note.userInfo() else {
                return;
            };
            let number = |key: &NSString| info.objectForKey(key).and_then(|v| v.downcast::<NSNumber>().ok());
            let bounds = webview.bounds();
            let bottom = bounds.origin.y + bounds.size.height;
            let kb = info
                .objectForKey(unsafe { UIKeyboardFrameEndUserInfoKey })
                .and_then(|v| v.downcast::<NSValue>().ok())
                .map(|v| webview.convertRect_fromView(unsafe { v.CGRectValue() }, None))
                .filter(|end| end.origin.y + end.size.height >= bottom - 1.0)
                .map(|end| (bottom - end.origin.y).max(0.0))
                .unwrap_or(0.0);
            let ms = number(unsafe { UIKeyboardAnimationDurationUserInfoKey })
                .map(|n| n.doubleValue() * 1000.0)
                .unwrap_or(0.0);
            let curve = number(unsafe { UIKeyboardAnimationCurveUserInfoKey }).map(|n| n.integerValue()).unwrap_or(0);
            let local = number(unsafe { UIKeyboardIsLocalUserInfoKey }).map(|n| n.boolValue()).unwrap_or(true);
            let state = UIApplication::sharedApplication(mtm).applicationState();
            let state = if state == UIApplicationState::Active {
                "active"
            } else if state == UIApplicationState::Inactive {
                "inactive"
            } else {
                "background"
            };
            tracing::info!("keyboard will change frame: kb={kb:.1} ms={ms:.0} curve={curve} local={local} state={state}");
            let js = format!(
                "window._countedKeyboard&&window._countedKeyboard.willChange({kb:.1},{ms:.0},{curve},{local},'{state}',{:.1})",
                start.elapsed().as_secs_f64() * 1000.0
            );
            super::evaluate_js(&*webview, &js);
        });
        let token = unsafe {
            NSNotificationCenter::defaultCenter().addObserverForName_object_queue_usingBlock(
                Some(UIKeyboardWillChangeFrameNotification),
                None,
                None,
                &block,
            )
        };
        tracing::info!("keyboard-frame observer installed");
        TOKEN.with(|t| {
            let _ = t.set(token);
        });
    }
}

/// The status-bar tap (native-feel P4.3). UIKit scrolls a scroll view to the top on that tap, but
/// the page never scrolls WKWebView's own scroll view: each page scrolls in its `.app-container`.
/// So this object is that scroll view's delegate, and implements only `scrollViewShouldScrollToTop:`:
/// it runs `window._countedScrollToTop()` (`SCROLL_TO_TOP_SCRIPT`), which scrolls the visible page
/// smoothly to 0, and declines the native scroll. `WKScrollView` keeps WebKit's internal delegate
/// and forwards to an external one set through the public `delegate` property, the external
/// answer last, so ours is the one UIKit reads. UIKit may not ask a scroll view already at offset
/// 0, which this one always is: the install line and the per-tap line in the device log tell.
#[cfg(target_os = "ios")]
mod status_bar_tap {
    use objc2::rc::Retained;
    use objc2::runtime::{NSObject, NSObjectProtocol, ProtocolObject};
    use objc2::{define_class, msg_send, MainThreadMarker, MainThreadOnly};
    use objc2_ui_kit::{UIScrollView, UIScrollViewDelegate};

    define_class!(
        #[unsafe(super(NSObject))]
        #[thread_kind = MainThreadOnly]
        #[name = "CountedScrollToTop"]
        struct ScrollToTop;

        unsafe impl NSObjectProtocol for ScrollToTop {}

        unsafe impl UIScrollViewDelegate for ScrollToTop {
            #[unsafe(method(scrollViewShouldScrollToTop:))]
            fn should_scroll_to_top(&self, scroll_view: &UIScrollView) -> bool {
                tracing::info!("status-bar tap: scrolling the page to the top");
                if let Some(webview) = scroll_view.superview() {
                    super::evaluate_js(&*webview, "window._countedScrollToTop&&window._countedScrollToTop()");
                }
                false
            }
        }
    );

    thread_local! {
        // The scroll view holds its delegate weakly.
        static DELEGATE: std::cell::OnceCell<Retained<ScrollToTop>> = const { std::cell::OnceCell::new() };
    }

    pub fn install(webview: &impl objc2::Message) {
        let Some(mtm) = MainThreadMarker::new() else {
            return;
        };
        if DELEGATE.with(|d| d.get().is_some()) {
            return;
        }
        let scroll: Retained<UIScrollView> = unsafe { msg_send![webview, scrollView] };
        let delegate: Retained<ScrollToTop> =
            unsafe { msg_send![super(mtm.alloc::<ScrollToTop>().set_ivars(())), init] };
        unsafe { scroll.setDelegate(Some(ProtocolObject::from_ref(&*delegate))) };
        tracing::info!(
            "status-bar tap delegate installed on the webview's scroll view (scrollsToTop: {})",
            scroll.scrollsToTop()
        );
        DELEGATE.with(|d| {
            let _ = d.set(delegate);
        });
    }
}

/// VoiceOver's escape, the two-finger Z, as a back (native-feel P4.8): the top sheet closes, else the
/// page goes back, through `history.back()` and the same popstate listener as Android back
/// (`navigation_sync.rs`). UIKit sends `accessibilityPerformEscape` to the focused element and then
/// up the responder chain, which from the webview reaches tao's root view controller, so the method
/// is added to that class (`TaoUIViewController`, registered by vendor/tao before the window shows).
/// `class_addMethod` adds it to tao's class only, whose own list has none, never to `NSObject`,
/// whose default answers NO. The answer must be synchronous: YES when `ui::common::back_armed()`
/// says a back has something to do, so VoiceOver plays its escape sound rather than its error one.
/// The webview is kept in a thread-local, as no Dioxus runtime is current in a UIKit callback.
#[cfg(target_os = "ios")]
mod voiceover_escape {
    use objc2::rc::Retained;
    use objc2::runtime::{AnyClass, AnyObject, Bool, Sel};
    use objc2::{ffi, sel, Encode};

    const BACK_JS: &str = "if(window._dioxusBackArmed&&!window._dioxusSyntheticBack)window.history.back();";

    thread_local! {
        static WEBVIEW: std::cell::OnceCell<Retained<AnyObject>> = const { std::cell::OnceCell::new() };
    }

    extern "C-unwind" fn escape(_: &AnyObject, _: Sel) -> Bool {
        if !ui::common::back_armed() {
            return Bool::NO;
        }
        let sent = WEBVIEW.with(|w| match w.get() {
            Some(webview) => {
                super::evaluate_js(&**webview, BACK_JS);
                true
            }
            None => false,
        });
        Bool::new(sent)
    }

    pub fn install(webview: Retained<AnyObject>) {
        if WEBVIEW.with(|w| w.set(webview)).is_err() {
            return;
        }
        let Some(cls) = AnyClass::get(c"TaoUIViewController") else {
            tracing::info!("TaoUIViewController not registered; VoiceOver escape not wired");
            return;
        };
        let Ok(types) = std::ffi::CString::new(format!("{}@:", Bool::ENCODING)) else {
            return;
        };
        let added = unsafe {
            ffi::class_addMethod(
                (cls as *const AnyClass).cast_mut(),
                sel!(accessibilityPerformEscape),
                std::mem::transmute::<
                    extern "C-unwind" fn(&AnyObject, Sel) -> Bool,
                    unsafe extern "C-unwind" fn(),
                >(escape),
                types.as_ptr(),
            )
        };
        tracing::info!("VoiceOver escape added to TaoUIViewController: {}", added.as_bool());
    }
}

/// The launch screen's colour (`LaunchBackground` in the asset catalog, packages/mobile/ios) under
/// the page: on the window, the webview, its scroll view and the area past its content, which a
/// keyboard pan or a rubber band uncovers. A named colour resolves against the window's
/// `overrideUserInterfaceStyle`, so it follows every theme change with no per-change code, where
/// wry's launch-time RGB stayed on the theme the app started in. Missing under `dx serve`, which
/// compiles no catalog.
#[cfg(target_os = "ios")]
fn set_launch_background(webview: &impl objc2::Message) {
    use objc2::msg_send;
    use objc2::rc::Retained;
    use objc2_foundation::NSString;
    use objc2_ui_kit::{UIColor, UIScrollView, UIWindow};

    let Some(color) = UIColor::colorNamed(&NSString::from_str("LaunchBackground")) else {
        tracing::info!("no LaunchBackground colour in the asset catalog, keeping wry's background");
        return;
    };
    unsafe {
        let window: Option<Retained<UIWindow>> = msg_send![webview, window];
        if let Some(window) = window {
            window.setBackgroundColor(Some(&color));
        }
        let _: () = msg_send![webview, setBackgroundColor: &*color];
        let _: () = msg_send![webview, setUnderPageBackgroundColor: &*color];
        let scroll: Retained<UIScrollView> = msg_send![webview, scrollView];
        scroll.setBackgroundColor(Some(&color));
    }
}

/// Stops iOS drawing its form-assistant bar - the floating `↑ ↓ ✓` pill - over the web content.
///
/// `ui`'s `AmountOperatorBar` is the last row of `.modal-box` and is placed by that box's
/// keyboard inset alone (`ui::common::keyboard`), so it lands flush on the keyboard. That inset is
/// measured from `visualViewport.height`, which does not shrink for an accessory view which *floats*
/// above the keyboard rather than docking to it - so the pill painted straight over the operator
/// keys and none of them could be reached.
///
/// The override goes on the class, not on an instance: the first responder during text input is a
/// `WKContentView`, a private class with no public handle, and finding the live one would mean
/// walking the scroll view's subviews on every focus. Hence the name string - which is all this
/// reaches for, no private selector is ever sent. It is the Cordova/Capacitor
/// `hideFormAccessoryBar` pattern.
///
/// **`class_replaceMethod`, never `method_setImplementation`.** `-inputAccessoryView` is declared on
/// `UIResponder`, so `method_setImplementation` on the *resolved* method would clear the accessory
/// view of every responder in the process. `class_replaceMethod` reads only the class's own method
/// list, never the superclass's: it adds the override when `WKContentView` has no implementation of
/// its own, and swaps the IMP when it has one, and in neither case can it reach `UIResponder`.
///
/// **It was `class_addMethod`, and that silently did nothing.** `class_addMethod` fails, returning
/// NO, when the class already implements the selector - and WebKit's `WKContentView` does implement
/// `-inputAccessoryView` (`WKContentViewInteraction.mm`), which is the whole reason the pill is
/// drawn. So the override was never installed and the bar kept painting over the operator keys. The
/// old comment read that failure as harmless insurance against WebKit "ever" declaring one; it had
/// already declared it. The log line below is the tell: it reports whether an existing
/// implementation was replaced.
///
/// Called from the launch-background effect rather than at launch, and for the same reason: the class
/// is not registered with the runtime until WebKit has built a webview. A missing class is a
/// silent skip - the same posture `haptic()` takes; the pill coming back is not worth a crash.
///
/// The cost is that the numeric keypad is left with no dismiss control - it has no return key and
/// the `✓` was the only one - which is what `AmountOperatorBar`'s own `⌄` key replaces. See
/// docs/ios.md.
#[cfg(target_os = "ios")]
fn hide_form_accessory_bar() {
    use objc2::runtime::{AnyClass, AnyObject, Sel};
    use objc2::{ffi, sel};

    extern "C-unwind" fn no_accessory(_: &AnyObject, _: Sel) -> *mut AnyObject {
        std::ptr::null_mut()
    }

    let Some(cls) = AnyClass::get(c"WKContentView") else {
        tracing::info!("WKContentView not registered; form accessory bar left in place");
        return;
    };
    let previous = unsafe {
        ffi::class_replaceMethod(
            (cls as *const AnyClass).cast_mut(),
            sel!(inputAccessoryView),
            std::mem::transmute::<
                extern "C-unwind" fn(&AnyObject, Sel) -> *mut AnyObject,
                unsafe extern "C-unwind" fn(),
            >(no_accessory),
            c"@@:".as_ptr(),
        )
    };
    tracing::info!(
        "form accessory bar suppressed (replaced an existing implementation: {})",
        previous.is_some()
    );
}

#[cfg(test)]
mod tests {
    use super::{css_inset_vars, platform_markers, visible_insets};

    #[test]
    fn ios_carries_its_major_version_and_android_only_the_platform() {
        assert_eq!(
            platform_markers(Some(17)),
            "document.documentElement.dataset.plat='ios';document.documentElement.dataset.ios='17';"
        );
        assert_eq!(platform_markers(None), "document.documentElement.dataset.plat='android';");
    }

    #[test]
    fn the_keypad_proxy_survives_the_taps_mousedown() {
        let script = super::KEYBOARD_PROXY_SCRIPT;
        assert!(script.contains("proxy.focus({preventScroll:true})"));
        assert!(script.contains("field.focus({preventScroll:true})"));
        assert!(script.contains("document.addEventListener('mousedown',function(e){if(handing&&document.activeElement===proxy"));
        assert_eq!(script.matches("preventDefault").count(), 1, "the mousedown is the one default cancelled");
    }

    #[test]
    fn the_client_offers_the_post_quantum_hybrid_first() {
        super::install_tls_provider();
        let provider = rustls::crypto::CryptoProvider::get_default().expect("a provider is installed");
        assert_eq!(provider.kx_groups[0].name(), rustls::NamedGroup::X25519MLKEM768);
    }

    // The bug these pin: `getRootWindowInsets()` returns the same numbers whether or not the
    // WebView is laid out under the bars, so the CSS vars were added on top of an inset the
    // system had already applied and the bottom dock floated a navigation bar too high.

    #[test]
    fn edge_to_edge_window_pays_for_its_own_bars() {
        // Content fills the decor: targetSdk 35+ on Android 15+, or an explicit opt-in.
        assert_eq!(visible_insets(63, 132, 0, 2400, 2400), (63, 132, 0));
    }

    #[test]
    fn system_inset_window_owes_nothing() {
        // 2400 - 63 - 132: the system carved the bars out of the content view already.
        assert_eq!(visible_insets(63, 132, 0, 2400, 2205), (0, 0, 0));
    }

    #[test]
    fn a_landscape_gesture_bar_follows_the_same_rule() {
        assert_eq!(visible_insets(0, 0, 84, 1080, 1080), (0, 0, 84));
        assert_eq!(visible_insets(0, 0, 84, 1080, 996), (0, 0, 0));
    }

    #[test]
    fn an_unmeasured_decor_view_assumes_edge_to_edge() {
        // First frame: reporting zeros here would flash a differently-positioned dock.
        assert_eq!(visible_insets(63, 132, 0, 0, 0), (63, 132, 0));
    }

    #[test]
    fn a_one_pixel_rounding_difference_is_not_an_inset() {
        assert_eq!(visible_insets(63, 132, 0, 2400, 2399), (63, 132, 0));
    }

    #[test]
    fn converts_physical_pixels_to_css_pixels() {
        // Typical 2.75-density phone: a 63px status bar is 22.9 CSS px.
        let js = css_inset_vars(63, 132, 0, 2.75);
        assert!(js.contains("'--sat','22.9px'"), "{js}");
        assert!(js.contains("'--sab','48.0px'"), "{js}");
        assert!(js.contains("'--sar','0.0px'"), "{js}");
    }

    #[test]
    fn density_one_is_the_identity() {
        let js = css_inset_vars(24, 48, 12, 1.0);
        assert!(js.contains("'--sat','24.0px'"), "{js}");
        assert!(js.contains("'--sab','48.0px'"), "{js}");
        assert!(js.contains("'--sar','12.0px'"), "{js}");
    }

    #[test]
    fn zero_insets_stay_zero() {
        let js = css_inset_vars(0, 0, 0, 3.0);
        assert!(js.contains("'--sat','0.0px'"), "{js}");
    }

    /// A bogus density must not divide by zero and blank the layout out.
    #[test]
    fn non_positive_density_falls_back_to_one() {
        let js = css_inset_vars(24, 0, 0, 0.0);
        assert!(js.contains("'--sat','24.0px'"), "{js}");
    }
}
