// Replaces the MainActivity.kt that `dx bundle` generates. Copied over the generated file by the
// Makefile / CI, in the same step that copies android-res/ — see docs/android.md.
//
// Why: dx's template is `class MainActivity : WryActivity()` with no body, and WryActivity never
// calls setIntent(). With launchMode=singleTask (patched into the manifest by the same step), a
// link tapped while the app is already running arrives through onNewIntent, but getIntent() would
// keep returning the original launch intent — so the Rust side would read a stale URL and the tap
// would only raise the app without navigating. Three lines fix warm-start deep links.
//
// requestPush is what the Rust side calls through JNI (`request_push` in packages/mobile/src/push.rs)
// after sign-in: the POST_NOTIFICATIONS prompt on 13+, then a UnifiedPush registration under our
// VAPID key; the subscription lands in the file CountedPushService.onNewEndpoint writes. The
// distributor is whatever the phone has (ntfy, Sunup, …), else the embedded FCM one in builds that
// include it, else none — and push is off, silently. `dx serve` builds have neither this file nor
// the connector, so the JNI call fails there and push is simply off.
//
// The typealias and package name mirror dx 0.7.9's MainActivity.kt.hbs; keep them in step when
// upgrading the Dioxus CLI.
package dev.dioxus.main

import android.Manifest
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Build
import org.unifiedpush.android.connector.UnifiedPush

typealias BuildConfig = fr.counted.app.BuildConfig

class MainActivity : WryActivity() {
    override fun onNewIntent(intent: Intent) {
        super.onNewIntent(intent)
        setIntent(intent)
    }

    fun requestPush(vapid: String) {
        runOnUiThread {
            if (Build.VERSION.SDK_INT >= 33 &&
                checkSelfPermission(Manifest.permission.POST_NOTIFICATIONS) != PackageManager.PERMISSION_GRANTED
            ) {
                requestPermissions(arrayOf(Manifest.permission.POST_NOTIFICATIONS), 0)
            }
            UnifiedPush.tryUseCurrentOrDefaultDistributor(this) { success ->
                if (success) {
                    UnifiedPush.register(this, vapid = vapid)
                }
            }
        }
    }
}
