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

/// Where `UpdateRequiredScreen` sends the user. iOS ships through TestFlight for now, and
/// `webbrowser` only opens http(s) — `itms-beta://` would silently do nothing — so the link is
/// TestFlight's own store page, whose "Open" lands in TestFlight. Swap for the App Store URL at
/// release (docs/ios.md).
#[cfg(target_os = "ios")]
const STORE_URL: &str = "https://apps.apple.com/app/testflight/id899247664";
#[cfg(not(target_os = "ios"))]
const STORE_URL: &str = "https://play.google.com/store/apps/details?id=fr.counted.app";

const FAVICON: Asset = asset!("/assets/counted.ico");
const LOGO: Asset = asset!("/assets/counted.png");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

// Self-hosted (no Google CDN), via asset!() so they bundle with hashed URLs; @font-face below
// is injected with those resolved URLs.
const FONT_INTER_400: Asset = asset!("/assets/fonts/inter-400.woff2");
const FONT_INTER_500: Asset = asset!("/assets/fonts/inter-500.woff2");
const FONT_INTER_600: Asset = asset!("/assets/fonts/inter-600.woff2");
const FONT_JAKARTA_600: Asset = asset!("/assets/fonts/jakarta-600.woff2");
const FONT_JAKARTA_700: Asset = asset!("/assets/fonts/jakarta-700.woff2");

fn font_face_css() -> String {
    const TEMPLATE: &str = r#"@font-face{font-family:"Inter";font-style:normal;font-weight:400;font-display:swap;src:url("$I4") format("woff2")}
@font-face{font-family:"Inter";font-style:normal;font-weight:500;font-display:swap;src:url("$I5") format("woff2")}
@font-face{font-family:"Inter";font-style:normal;font-weight:600;font-display:swap;src:url("$I6") format("woff2")}
@font-face{font-family:"Plus Jakarta Sans";font-style:normal;font-weight:600;font-display:swap;src:url("$J6") format("woff2")}
@font-face{font-family:"Plus Jakarta Sans";font-style:normal;font-weight:700;font-display:swap;src:url("$J7") format("woff2")}"#;
    TEMPLATE
        .replace("$I4", &FONT_INTER_400.to_string())
        .replace("$I5", &FONT_INTER_500.to_string())
        .replace("$I6", &FONT_INTER_600.to_string())
        .replace("$J6", &FONT_JAKARTA_600.to_string())
        .replace("$J7", &FONT_JAKARTA_700.to_string())
}

/// Stylesheets spliced into dioxus-desktop's index template before `</head>`, so they are
/// render-blocking — `document::Link` only reaches the head after the first render, leaving the
/// webview's first paint unstyled. This does not replace the template (docs/dioxus-overrides.md).
///
/// Order is tailwind then main: main.css is unlayered and must win.
///
/// Only the mobile config consumes it — not the host server binary that
/// `dx bundle --platform android` also builds.
#[cfg_attr(not(any(target_os = "android", target_os = "ios")), allow(dead_code))]
fn head_html() -> String {
    format!(
        // The empty icon is not decoration: a WebView asks for /favicon.ico on every document load,
        // wry's asset protocol has none to serve, and the 404 is logged as a console error — noise
        // in every session, and enough to make `boots without console errors` flake depending on
        // whether it lands before the spec attaches. `data:,` answers it without a request. No app
        // window ever shows an icon here, so there is nothing to supply instead.
        r#"<link rel="icon" href="data:,"><link rel="stylesheet" href="{TAILWIND_CSS}"><link rel="stylesheet" href="{MAIN_CSS}"><style>{fonts}</style>"#,
        fonts = font_face_css()
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

/// Overrides `--sat`/`--sab`/`--sar` from `assets/main.css`. An inline property on the root
/// element beats the `:root` rule, so this is the only hook Android needs — every `.safe-*` rule
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
/// (`dx serve` included) gets an already-inset window, where adding the bars counts them twice —
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
/// API 30+: `insets.getInsets(WindowInsets.Type.systemBars())` — what Android 15 edge-to-edge
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

/// None until the decor view is attached — the caller retries on the next resize.
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
        // `/` for an Android process — unwritable — which meant reads returned an empty store and
        // writes failed with nothing but a log line: the app would look like a fresh install and
        // silently refuse to persist a single project key. Still the fallback, because there is
        // nothing better to do, but it is now loud.
        let dir = android_files_dir().unwrap_or_else(|| {
            tracing::error!(
                "counted: getFilesDir() failed — COUNTED_DATA_DIR falls back to '.', which is not \
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
/// The same link still works twice — `MainActivity.onNewIntent` installs a fresh intent.
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

/// UIKit's own feedback vocabulary. A generator is built per call rather than cached: they are
/// cheap, `prepare()` is what actually costs (it wakes the Taptic Engine), and holding one across
/// calls would mean holding a `MainThreadOnly` object in a `Send + Sync` closure.
///
/// `MainThreadMarker::new()` returning `None` is the guard — UIFeedbackGenerator is main-thread
/// only, and a haptic is never worth a crash. `initWithStyle`/`init` are deprecated in favour of
/// the `forView:` constructors, which need a `UIView` this layer has no handle on.
#[cfg(target_os = "ios")]
#[allow(deprecated)]
fn play_haptic(kind: ui::common::Haptic) {
    use objc2::{MainThreadMarker, MainThreadOnly};
    use objc2_ui_kit::{
        UIImpactFeedbackGenerator, UIImpactFeedbackStyle, UINotificationFeedbackGenerator,
        UINotificationFeedbackType,
    };
    use ui::common::Haptic;

    let Some(mtm) = MainThreadMarker::new() else {
        return;
    };
    match kind {
        Haptic::Light | Haptic::Medium => {
            let style = if matches!(kind, Haptic::Light) {
                UIImpactFeedbackStyle::Light
            } else {
                UIImpactFeedbackStyle::Medium
            };
            let g = UIImpactFeedbackGenerator::initWithStyle(
                UIImpactFeedbackGenerator::alloc(mtm),
                style,
            );
            g.prepare();
            g.impactOccurred();
        }
        Haptic::Success | Haptic::Warning | Haptic::Error => {
            let ty = match kind {
                Haptic::Success => UINotificationFeedbackType::Success,
                Haptic::Warning => UINotificationFeedbackType::Warning,
                _ => UINotificationFeedbackType::Error,
            };
            let g = UINotificationFeedbackGenerator::init(UINotificationFeedbackGenerator::alloc(mtm));
            g.prepare();
            g.notificationOccurred(ty);
        }
    }
}

/// `View.performHapticFeedback` on the decor view, not `Vibrator`: it needs no permission and it
/// respects the user's "Touch feedback" system setting, which a raw vibration would override.
///
/// The constants are `HapticFeedbackConstants`: CONTEXT_CLICK (6) for a light selection,
/// LONG_PRESS (0) for a committed gesture, CONFIRM (16) / REJECT (17) for the outcome pair.
/// CONFIRM/REJECT are API 30+; the call simply does nothing on older devices, which is the right
/// failure mode for feedback.
#[cfg(target_os = "android")]
fn play_haptic(kind: ui::common::Haptic) {
    use ui::common::Haptic;

    let constant = match kind {
        Haptic::Light => 6,
        Haptic::Medium => 0,
        Haptic::Success => 16,
        Haptic::Warning | Haptic::Error => 17,
    };

    fn call(env: &mut jni::JNIEnv, constant: i32) -> jni::errors::Result<()> {
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
    if let Err(e) = call(&mut env, constant) {
        // Always clear: a pending exception poisons every later JNI call on this thread — the
        // inset reads above included.
        let _ = env.exception_clear();
        tracing::debug!("haptic failed: {e}");
    }
}

/// `ACTION_SEND` wrapped in `Intent.createChooser`, started from the activity itself — no
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
#[allow(deprecated)]
fn share_text(text: &str) -> bool {
    use objc2::rc::Retained;
    use objc2::runtime::AnyObject;
    use objc2::{MainThreadMarker, MainThreadOnly};
    use objc2_foundation::{NSArray, NSString};
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
    let item: Retained<AnyObject> = Retained::into_super(Retained::into_super(NSString::from_str(text)));
    let items = NSArray::from_retained_slice(&[item]);
    let vc = unsafe {
        UIActivityViewController::initWithActivityItems_applicationActivities(
            UIActivityViewController::alloc(mtm),
            &items,
            None,
        )
    };
    if let Some(popover) = vc.popoverPresentationController() {
        popover.setSourceView(root.view().as_deref());
    }
    root.presentViewController_animated_completion(&vc, true, None);
    true
}

/// Bridges `ocr::scan` to the shape `ui` asks for, and installs the platform's temp-file purge.
///
/// The adapter lives here rather than in either crate: `ocr` must not know about Dioxus, and `ui`
/// must not depend on `ocr` — 31 MB of weights would land in the wasm bundle and the server binary.
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
/// and falls back to the document picker — so **Scan opens the gallery instead of the camera**, and
/// the only trace is a `Tauri/FileChooser` log line from inside wry.
///
/// `dx serve` can never carry the provider: dx 0.7.9 interpolates `[android.raw.manifest]` at
/// `<manifest>` level, and a `<provider>` must be a child of `<application>`. Only the Makefile's
/// `patch_file_provider` — i.e. `make android-build` — puts it there. This turns that structural gap
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
        // Derived exactly the way wry derives it, so an applicationIdSuffix cannot slip past this —
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
             Expected under `dx serve` — build with `make android-build` to test capture."
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
        // Always clear: a pending exception poisons every later JNI call on this thread — the
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
    ui::common::set_native_sharer(std::sync::Arc::new(share_text));

    // Same contract: installed before launch, so no call site can race an unset scanner. Every
    // other target leaves it unset and the Scan button does not render.
    #[cfg(any(target_os = "android", target_os = "ios"))]
    install_scanner();

    #[cfg(any(target_os = "android", target_os = "ios"))]
    push::install();

    let builder = dioxus::LaunchBuilder::new();
    // Dioxus's release default `disable_context_menu = !debug_assertions` injects a global
    // `contextmenu` -> preventDefault, which on Android/iOS also kills the native text-selection
    // toolbar. Re-enable it.
    //
    // Covers only deep links arriving BEFORE the UI exists — installed ahead of the event loop, so
    // a cold-start link cannot race the first render. `DeepLinkListener` then takes over; dioxus
    // dispatches registered wry handlers first, so without the stand-down both would handle the
    // same link. See packages/ui/src/common/deep_link.rs.
    #[cfg(any(target_os = "android", target_os = "ios"))]
    let builder = builder.with_cfg(
        dioxus::mobile::Config::new()
            // wry paints this before parsing any HTML. The window shows right after the first
            // rebuild's edits are *sent*, unflushed, so one frame has an empty document — this
            // makes it base-200 rather than white. sRGB of --color-base-200.
            .with_background_color((244, 248, 246, 255))
            .with_custom_head(head_html())
            .with_disable_context_menu(false)
            .with_custom_event_handler(|event, _| {
                use dioxus::mobile::tao::event::Event;
                if ui::common::deep_link_listener_active() {
                    return;
                }
                match event {
                    // iOS delivers BOTH counted:// (application:openURL:) and universal links
                    // (application:continueUserActivity:) as Opened — see tao ios/view.rs.
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
    #[cfg(any(target_os = "android", target_os = "ios"))]
    use_effect(|| {
        document::eval(r#"
            (function() {
                var m = document.querySelector('meta[name="viewport"]');
                if (m) {
                    if (!m.content.includes('viewport-fit')) m.content += ', viewport-fit=cover';
                } else {
                    m = document.createElement('meta');
                    m.name = 'viewport';
                    m.content = 'width=device-width, initial-scale=1.0, viewport-fit=cover';
                    document.head.appendChild(m);
                }
            })();
        "#);
    });
    // WKWebView's left-edge back gesture, off by default. docs/mobile-navigation.md used to call
    // this a manual Xcode edit; there is no Xcode project — `dx bundle --platform ios` emits the
    // .app directly — so it is set here on wry's own WKWebView instead, and survives every bundle.
    //
    // `NavigationSync` is what gives the gesture something to go back to: it keeps one spare
    // window.history entry while a back press has anywhere to go, and the resulting popstate is
    // already wired to the router.
    //
    // Sent as a raw selector, not through a typed method: wry 0.53 vendors its own iOS `WKWebView`
    // binding (`src/wkwebview/ios/WKWebView.rs`) instead of using objc2-web-kit's, and that binding
    // declares no `allowsBackForwardNavigationGestures`. The selector exists on the real class
    // regardless, so this needs no extra dependency.
    #[cfg(target_os = "ios")]
    use_effect(|| {
        use dioxus::mobile::wry::WebViewExtIOS;
        use objc2::msg_send;
        // Deferred to an effect rather than done at launch: the webview does not exist until the
        // window is built.
        let webview = dioxus::mobile::window().webview.webview();
        unsafe {
            let _: () = msg_send![&*webview, setAllowsBackForwardNavigationGestures: true];
        }
    });

    // Dynamic Type. `-apple-system-body` is the only handle a WKWebView has on the user's Text Size
    // setting; 17px is the "Large" default, so the ratio is the scale. Driven by `resize`, which
    // iOS fires on a content-size-category change, so the same listener covers rotation.
    //
    // Clamped because the pages are `max-w-md` and vertical: they absorb growth, but the
    // Accessibility sizes at the top of the range would break the dock and the header.
    //
    // **The sentinel is load-bearing.** Where the `font` shorthand does not parse the keyword, the
    // declaration is dropped and the span inherits instead — and what it inherits is
    // `16px * var(--dt-scale)`, the value this very effect wrote. That is a feedback loop: each
    // resize divides the previous scale by 17/16 until it sticks at the 0.85 floor, shrinking the
    // whole UI ~15% with nothing to point at. Setting `font-size` to a value nothing would ever
    // compute *before* the shorthand detects it exactly: a parsed shorthand resets every font
    // longhand, so 99px surviving means the keyword was ignored and there is nothing to measure.
    //
    // iOS only. Android's WebView already applies the system font-size setting itself, through
    // `WebSettings.textZoom`, so there is nothing for this to add there.
    #[cfg(target_os = "ios")]
    use_effect(|| {
        document::eval(
            r#"
            (function() {
                const measure = () => {
                    const p = document.createElement('span');
                    p.style.position = 'absolute';
                    p.style.visibility = 'hidden';
                    p.style.fontSize = '99px';
                    p.style.font = '-apple-system-body';
                    document.body.appendChild(p);
                    const px = parseFloat(getComputedStyle(p).fontSize);
                    p.remove();
                    if (!(px > 0) || px === 99) return;
                    const scale = Math.min(1.6, Math.max(0.85, px / 17));
                    document.documentElement.style.setProperty('--dt-scale', String(scale));
                };
                window.addEventListener('resize', measure);
                measure();
            })();
        "#,
        );
    });

    // The on-screen keyboard, as a --kb CSS variable the bottom sheets pad themselves by.
    //
    // Needed because daisyUI's .modal is `position: fixed` and neither platform resizes the layout
    // viewport for the keyboard — WKWebView scrolls the *page* to reveal the focused input, the
    // Android theme is `adjustNothing` — and a fixed element does not move with page scroll, so a
    // sheet's field can sit behind the keyboard. Only visualViewport can see it.
    //
    // Same split as the safe-area bridge below: JS observes, Rust decides. `keyboard_inset` lives in
    // the ui crate and is unit-tested there; nothing here does arithmetic.
    //
    // rAF-coalesced and change-gated on purpose. The keyboard animation fires these ~20 times over
    // ~250ms, and every message to Rust costs a synchronous XHR over wry's bridge (see the module
    // docs on expense_row.rs) — so the listener sends at most one value per frame, and none at all
    // while the number is unchanged.
    //
    // No cfg split: the arithmetic is the same on both, and a build whose window *is* resized for
    // the keyboard shrinks both heights together, so `keyboard_inset` yields 0 and this is a no-op.
    #[cfg(any(target_os = "android", target_os = "ios"))]
    use_effect(|| {
        let mut eval = document::eval(
            r#"
            (function() {
                const vv = window.visualViewport;
                if (!vv) return;
                let queued = false, last = null;
                const report = () => {
                    queued = false;
                    const msg = [window.innerHeight, vv.height, vv.offsetTop];
                    const key = msg.join(',');
                    if (key === last) return;
                    last = key;
                    dioxus.send(msg);
                };
                const schedule = () => {
                    if (queued) return;
                    queued = true;
                    requestAnimationFrame(report);
                };
                vv.addEventListener('resize', schedule);
                vv.addEventListener('scroll', schedule);
                schedule();
            })();
        "#,
        );
        spawn(async move {
            while let Ok(m) = eval.recv::<[f64; 3]>().await {
                let px = ui::common::keyboard_inset(m[0], m[1], m[2]);
                document::eval(&ui::common::css_keyboard_var(px));
            }
        });
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
        spawn(async move {
            while eval.recv::<String>().await.is_ok() {
                if let Some((top, bottom, right, density)) = android_insets() {
                    tracing::info!(
                        "android safe-area insets: top={top} bottom={bottom} right={right} density={density}"
                    );
                    document::eval(&css_inset_vars(top, bottom, right, density));
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
        // Duplicated by head_html() on android/ios (same URLs, cached — no refetch, no repaint).
        // Kept because with_custom_head is cfg-gated to those targets, so dropping these would
        // strip the CSS from any other build of this package.
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Style { {font_face_css()} }

        if !ready() {
            SplashScreen { logo: LOGO }
        }
        UpdateRequiredScreen { store_url: STORE_URL }

        main {
            "data-theme": "counted",
            class: "min-h-screen flex flex-col items-center",
            Router::<Route> {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{css_inset_vars, visible_insets};

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
