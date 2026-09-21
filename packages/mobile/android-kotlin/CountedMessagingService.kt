// Copied into the generated project next to MainActivity.kt by patch_push in the Makefile, which
// also registers the <service> in the manifest and adds firebase-messaging to build.gradle.kts.
//
// The server sends data-only messages (packages/api/src/push/wire.rs), so this class builds every
// notification itself: the SDK's tray defaults would otherwise apply while the app is in the
// background and nothing at all while it is in the foreground.
package dev.dioxus.main

import android.Manifest
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.content.Context
import android.content.pm.PackageManager
import android.os.Build
import androidx.core.app.NotificationCompat
import com.google.firebase.messaging.FirebaseMessagingService
import com.google.firebase.messaging.RemoteMessage
import java.io.File

/// The token file the Rust side reads (`ui::common::read_push_token`): `filesDir/push_token`,
/// which is the COUNTED_DATA_DIR main.rs resolves from getFilesDir(). Sibling tmp file and rename
/// for the same reason as every other store there.
object PushToken {
    fun write(context: Context, token: String) {
        val file = File(context.filesDir, "push_token")
        val tmp = File(context.filesDir, "push_token.tmp")
        tmp.writeText(token)
        if (!tmp.renameTo(file)) {
            tmp.delete()
        }
    }
}

class CountedMessagingService : FirebaseMessagingService() {
    override fun onNewToken(token: String) {
        PushToken.write(this, token)
    }

    override fun onMessageReceived(message: RemoteMessage) {
        val title = message.data["title"] ?: return
        val body = message.data["body"] ?: return
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
        manager.notify(message.data["kind"].hashCode(), notification)
    }

    companion object {
        const val CHANNEL = "counted"
    }
}
