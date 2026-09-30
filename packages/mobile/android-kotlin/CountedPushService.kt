// Copied into the generated project next to MainActivity.kt by patch_push in packages/mobile/android.mk, which
// also registers the <service> in the manifest and adds the UnifiedPush connector to
// build.gradle.kts.
//
// The server sends RFC 8030 Web Push (packages/api/src/server/push/webpush.rs): the connector
// decrypts it with keys that never leave the device, and this class builds every notification from
// the JSON inside — our channel, our icon, foreground included.
package dev.dioxus.main

import android.Manifest
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.content.Context
import android.content.pm.PackageManager
import android.os.Build
import android.util.Log
import androidx.core.app.NotificationCompat
import org.json.JSONObject
import org.unifiedpush.android.connector.FailedReason
import org.unifiedpush.android.connector.PushService
import org.unifiedpush.android.connector.data.PushEndpoint
import org.unifiedpush.android.connector.data.PushMessage
import java.io.File

/// The file the Rust side reads (`ui::common::read_push_token`): `filesDir/push_token`, which is
/// the COUNTED_DATA_DIR main.rs resolves from getFilesDir(). Sibling tmp file and rename for the
/// same reason as every other store there.
object PushToken {
    private fun file(context: Context) = File(context.filesDir, "push_token")

    fun write(context: Context, subscription: String) {
        val tmp = File(context.filesDir, "push_token.tmp")
        tmp.writeText(subscription)
        if (!tmp.renameTo(file(context))) {
            tmp.delete()
        }
    }

    fun delete(context: Context) {
        file(context).delete()
    }

    /// The proof from the server's verification push, for `ui::common::take_push_proof` to return.
    fun writeProof(context: Context, proof: String) {
        val tmp = File(context.filesDir, "push_proof.tmp")
        tmp.writeText(proof)
        if (!tmp.renameTo(File(context.filesDir, "push_proof"))) {
            tmp.delete()
        }
    }
}

class CountedPushService : PushService() {
    override fun onNewEndpoint(endpoint: PushEndpoint, instance: String) {
        val keys = endpoint.pubKeySet ?: return
        val subscription = JSONObject()
            .put("endpoint", endpoint.url)
            .put("p256dh", keys.pubKey)
            .put("auth", keys.auth)
        PushToken.write(this, subscription.toString())
    }

    override fun onMessage(message: PushMessage, instance: String) {
        // Anything the connector could not decrypt did not come from our server.
        if (!message.decrypted) {
            return
        }
        val data = runCatching { JSONObject(String(message.content, Charsets.UTF_8)) }.getOrNull() ?: return
        // The server's one push to a new endpoint: proof that this device receives it. Never shown.
        if (data.optString("kind") == "verify") {
            data.optString("proof").ifEmpty { return }.let { PushToken.writeProof(this, it) }
            return
        }
        val title = data.optString("title").ifEmpty { return }
        val body = data.optString("body").ifEmpty { return }
        if (Build.VERSION.SDK_INT >= 33 &&
            checkSelfPermission(Manifest.permission.POST_NOTIFICATIONS) != PackageManager.PERMISSION_GRANTED
        ) {
            return
        }

        val manager = getSystemService(NotificationManager::class.java)
        if (Build.VERSION.SDK_INT >= 26) {
            manager.createNotificationChannel(
                NotificationChannel(CHANNEL, title, NotificationManager.IMPORTANCE_DEFAULT)
            )
        }

        val open = PendingIntent.getActivity(
            this,
            0,
            packageManager.getLaunchIntentForPackage(packageName),
            PendingIntent.FLAG_IMMUTABLE or PendingIntent.FLAG_UPDATE_CURRENT,
        )
        val notification = NotificationCompat.Builder(this, CHANNEL)
            .setSmallIcon(fr.counted.app.R.drawable.ic_notification)
            .setContentTitle(title)
            .setContentText(body)
            .setContentIntent(open)
            .setAutoCancel(true)
            .build()
        manager.notify(data.optString("kind").hashCode(), notification)
    }

    override fun onRegistrationFailed(reason: FailedReason, instance: String) {
        Log.w(TAG, "push registration failed: $reason")
    }

    override fun onUnregistered(instance: String) {
        PushToken.delete(this)
    }

    companion object {
        const val CHANNEL = "counted"
        const val TAG = "counted"
    }
}
