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
import android.content.ActivityNotFoundException
import android.content.ClipData
import android.content.Intent
import android.content.pm.PackageManager
import android.net.Uri
import android.os.Build
import androidx.core.content.FileProvider
import java.io.File
import org.unifiedpush.android.connector.UnifiedPush

typealias BuildConfig = fr.counted.app.BuildConfig

class MainActivity : WryActivity() {
    override fun onNewIntent(intent: Intent) {
        super.onNewIntent(intent)
        setIntent(intent)
    }

    // `open_url` and `share_file` in packages/mobile/src/main.rs call these two through JNI. Here
    // rather than in Rust because FileProvider is an androidx class, which JNI's FindClass cannot
    // see from a native thread; a method on the activity runs with the app's class loader.

    // mailto: only today. wry hands links to webbrowser::open, which refuses anything but http(s).
    fun openUri(uri: String): Boolean =
        try {
            startActivity(Intent(Intent.ACTION_SENDTO, Uri.parse(uri)))
            true
        } catch (e: ActivityNotFoundException) {
            false
        }

    // The file must sit under a <cache-path> of res/xml/file_paths.xml, or getUriForFile throws.
    fun shareFile(path: String, mime: String) {
        val uri = FileProvider.getUriForFile(this, "fr.counted.app.fileprovider", File(path))
        val send = Intent(Intent.ACTION_SEND)
            .setType(mime)
            .putExtra(Intent.EXTRA_STREAM, uri)
            .addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
        send.clipData = ClipData.newRawUri(null, uri)
        startActivity(Intent.createChooser(send, null))
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
