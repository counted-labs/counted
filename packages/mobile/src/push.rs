//! The two platform halves of `ui::common::push`: ask the OS for permission and a device token,
//! and get that token into `COUNTED_DATA_DIR/push_token`. See docs/plans/push-notifications.md.

/// Calls `MainActivity.requestPush()` (packages/mobile/android-kotlin/MainActivity.kt), which
/// prompts for POST_NOTIFICATIONS and writes the FCM token to the file itself. A `dx serve` build
/// has no such method: the call raises NoSuchMethodError, which is cleared here so it cannot
/// surface as a crash on the next JNI call.
#[cfg(target_os = "android")]
pub fn request_push() {
    fn inner(env: &mut jni::JNIEnv) -> jni::errors::Result<()> {
        let ctx = ndk_context::android_context();
        let activity = unsafe { jni::objects::JObject::from_raw(ctx.context().cast()) };
        env.call_method(&activity, "requestPush", "()V", &[])?;
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
        let _ = env.exception_clear();
        tracing::warn!("counted: requestPush unavailable: {e}");
    }
}

#[cfg(target_os = "ios")]
pub fn request_push() {
    use block2::RcBlock;
    use objc2::runtime::Bool;
    use objc2::MainThreadMarker;
    use objc2_foundation::NSError;
    use objc2_ui_kit::UIApplication;
    use objc2_user_notifications::{UNAuthorizationOptions, UNUserNotificationCenter};

    ios::add_delegate_methods();

    let granted_handler = RcBlock::new(|granted: Bool, _error: *mut NSError| {
        if !granted.as_bool() {
            return;
        }
        // The completion handler runs on a background queue; registration is main-thread only.
        dispatch2::DispatchQueue::main().exec_async(|| {
            if let Some(mtm) = MainThreadMarker::new() {
                UIApplication::sharedApplication(mtm).registerForRemoteNotifications();
            }
        });
    });
    UNUserNotificationCenter::currentNotificationCenter().requestAuthorizationWithOptions_completionHandler(
        UNAuthorizationOptions::Alert | UNAuthorizationOptions::Sound | UNAuthorizationOptions::Badge,
        &granted_handler,
    );
}

#[cfg(target_os = "ios")]
mod ios {
    use objc2::runtime::{AnyClass, AnyObject, Sel};
    use objc2::sel;
    use objc2_foundation::{NSData, NSError};

    extern "C-unwind" fn did_register(_this: &AnyObject, _cmd: Sel, _app: &AnyObject, token: &NSData) {
        let hex: String = token.to_vec().iter().map(|b| format!("{b:02x}")).collect();
        if let Err(e) = ui::common::write_push_token(&hex) {
            tracing::error!("counted: could not write the push token: {e}");
        }
    }

    extern "C-unwind" fn did_fail(_this: &AnyObject, _cmd: Sel, _app: &AnyObject, error: &NSError) {
        tracing::warn!("counted: APNs registration failed: {}", error.localizedDescription());
    }

    /// tao declares its `AppDelegate` class while building the event loop — inside `launch`, so
    /// after `main` — and implements none of the remote-notification callbacks. Adding them to the
    /// registered class at runtime is the only seam short of forking tao; it happens on the first
    /// `request_push`, which a Dioxus effect issues once the app is running.
    pub fn add_delegate_methods() {
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| {
            let Some(class) = AnyClass::get(c"AppDelegate") else {
                tracing::error!("counted: no AppDelegate class — tao renamed it");
                return;
            };
            let class = class as *const AnyClass as *mut AnyClass;
            unsafe {
                objc2::ffi::class_addMethod(
                    class,
                    sel!(application:didRegisterForRemoteNotificationsWithDeviceToken:),
                    std::mem::transmute::<
                        extern "C-unwind" fn(&AnyObject, Sel, &AnyObject, &NSData),
                        objc2::runtime::Imp,
                    >(did_register),
                    c"v@:@@".as_ptr(),
                );
                objc2::ffi::class_addMethod(
                    class,
                    sel!(application:didFailToRegisterForRemoteNotificationsWithError:),
                    std::mem::transmute::<
                        extern "C-unwind" fn(&AnyObject, Sel, &AnyObject, &NSError),
                        objc2::runtime::Imp,
                    >(did_fail),
                    c"v@:@@".as_ptr(),
                );
            }
        });
    }
}

#[cfg(any(target_os = "android", target_os = "ios"))]
pub fn install() {
    #[cfg(target_os = "android")]
    let platform = shared::PushPlatform::Android;
    #[cfg(target_os = "ios")]
    let platform = shared::PushPlatform::Ios;
    ui::common::set_native_push(ui::common::NativePush {
        platform,
        request: std::sync::Arc::new(request_push),
    });
}
