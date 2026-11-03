// Copied into the generated project next to MainActivity.kt by patch_push in packages/mobile/android.mk, which
// also registers the <service> in the manifest and adds the UnifiedPush connector to
// build.gradle.kts.
//
// The server sends RFC 8030 Web Push (packages/api/src/server/push/webpush.rs): the connector
// decrypts it with keys that never leave the device, and this class builds every notification from
// the JSON inside - our channel, our icon, foreground included.
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
        // Rendered here from `kind`; the server's sentence only for a kind this build does not know.
        val title = "Counted"
        val lang = runCatching { File(filesDir, "push_lang").readText().trim() }.getOrDefault("en")
        val body = PushTexts.body(data.optString("kind"), lang) ?: data.optString("body").ifEmpty { return }
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

/// The four sentences a push may carry, in every locale the app ships. Rendered here from the
/// push's `kind` in the in-app language (`push_lang`, written by `ui::common::push_sync`), so the
/// server needs no language per device (audit 2026-10-19 L7). English for any other code.
object PushTexts {
    private val texts: Map<String, List<String>> = mapOf(
        "en" to listOf("New friend request", "Your friend request was accepted", "You've been invited to a project", "One of your projects was closed", "The key of one of your projects changed"),
        "bg" to listOf("Нова заявка за приятелство", "Заявката ти за приятелство беше приета", "Нова покана за проект", "Един от твоите проекти беше затворен", "Ключът на един от твоите проекти е сменен"),
        "bs" to listOf("Novi zahtjev za prijateljstvo", "Tvoj zahtjev za prijateljstvo je prihvaćen", "Nova pozivnica za projekat", "Jedan od tvojih projekata je zatvoren", "Ključ jednog od tvojih projekata je promijenjen"),
        "cs" to listOf("Nová žádost o přátelství", "Tvoje žádost o přátelství byla přijata", "Nová pozvánka do projektu", "Jeden z tvých projektů byl uzavřen", "Klíč jednoho z tvých projektů se změnil"),
        "da" to listOf("Ny venneanmodning", "Din venneanmodning blev accepteret", "Ny invitation til et projekt", "Et af dine projekter er blevet lukket", "Nøglen til et af dine projekter er ændret"),
        "de" to listOf("Neue Freundschaftsanfrage", "Deine Freundschaftsanfrage wurde angenommen", "Du wurdest zu einem Projekt eingeladen", "Eines deiner Projekte wurde abgeschlossen", "Der Schlüssel eines deiner Projekte hat sich geändert"),
        "el" to listOf("Νέο αίτημα φιλίας", "Το αίτημα φιλίας σου έγινε αποδεκτό", "Νέα πρόσκληση σε έργο", "Ένα από τα έργα σου έκλεισε", "Το κλειδί ενός από τα έργα σου άλλαξε"),
        "es" to listOf("Nueva solicitud de amistad", "Tu solicitud de amistad fue aceptada", "Te han invitado a un proyecto", "Uno de tus proyectos se ha cerrado", "Ha cambiado la clave de uno de tus proyectos"),
        "et" to listOf("Uus sõbrakutse", "Sinu sõbrakutse võeti vastu", "Uus kutse projekti", "Üks sinu projektidest suleti", "Ühe sinu projekti võti muutus"),
        "fi" to listOf("Uusi kaveripyyntö", "Kaveripyyntösi hyväksyttiin", "Uusi kutsu projektiin", "Yksi projekteistasi suljettiin", "Yhden projektisi avain vaihtui"),
        "fr" to listOf("Nouvelle demande d'ami", "Votre demande d'ami a été acceptée", "Vous êtes invité à rejoindre un projet", "Un de vos projets a été clôturé", "La clé d'un de vos projets a changé"),
        "ga" to listOf("Iarraidh chairdis nua", "Glacadh le d'iarraidh chairdis", "Cuireadh nua chuig tionscadal", "Dúnadh ceann de do thionscadail", "Athraíodh eochair ceann de do thionscadail"),
        "hr" to listOf("Novi zahtjev za prijateljstvo", "Tvoj zahtjev za prijateljstvo je prihvaćen", "Nova pozivnica za projekt", "Jedan od tvojih projekata je zatvoren", "Ključ jednog od tvojih projekata je promijenjen"),
        "hu" to listOf("Új barátkérelem", "Elfogadták a barátkérelmedet", "Új meghívó egy projektbe", "Az egyik projektedet lezárták", "Megváltozott az egyik projekted kulcsa"),
        "is" to listOf("Ný vinabeiðni", "Vinabeiðni þín var samþykkt", "Nýtt boð í verkefni", "Einu af verkefnunum þínum var lokað", "Lykli eins af verkefnunum þínum var breytt"),
        "it" to listOf("Nuova richiesta di amicizia", "La tua richiesta di amicizia è stata accettata", "Sei stato invitato a un progetto", "Uno dei tuoi progetti è stato chiuso", "La chiave di uno dei tuoi progetti è cambiata"),
        "lt" to listOf("Nauja draugystės užklausa", "Tavo draugystės užklausa priimta", "Naujas kvietimas į projektą", "Vienas iš tavo projektų uždarytas", "Vieno tavo projekto raktas pasikeitė"),
        "lv" to listOf("Jauns draudzības pieprasījums", "Tavs draudzības pieprasījums ir pieņemts", "Jauns ielūgums uz projektu", "Viens no taviem projektiem ir slēgts", "Viena tava projekta atslēga ir mainīta"),
        "mk" to listOf("Ново барање за пријателство", "Твоето барање за пријателство е прифатено", "Нова покана за проект", "Еден од твоите проекти е затворен", "Клучот на еден од твоите проекти е сменет"),
        "mt" to listOf("Talba ġdida ta' ħbiberija", "It-talba tiegħek ta' ħbiberija ġiet aċċettata", "Stedina ġdida għal proġett", "Wieħed mill-proġetti tiegħek ingħalaq", "Iċ-ċavetta ta' wieħed mill-proġetti tiegħek inbidlet"),
        "nl" to listOf("Nieuw vriendschapsverzoek", "Je vriendschapsverzoek is geaccepteerd", "Je bent uitgenodigd voor een project", "Een van je projecten is afgesloten", "De sleutel van een van je projecten is gewijzigd"),
        "no" to listOf("Ny venneforespørsel", "Venneforespørselen din ble godtatt", "Ny invitasjon til et prosjekt", "Et av prosjektene dine ble lukket", "Nøkkelen til et av prosjektene dine er endret"),
        "pl" to listOf("Nowe zaproszenie do znajomych", "Twoje zaproszenie do znajomych zostało przyjęte", "Nowe zaproszenie do projektu", "Jeden z twoich projektów został zamknięty", "Klucz jednego z twoich projektów się zmienił"),
        "pt" to listOf("Novo pedido de amizade", "O seu pedido de amizade foi aceite", "Foi convidado para um projeto", "Um dos seus projetos foi encerrado", "A chave de um dos teus projetos mudou"),
        "ro" to listOf("Cerere de prietenie nouă", "Cererea ta de prietenie a fost acceptată", "Invitație nouă la un proiect", "Unul dintre proiectele tale a fost închis", "Cheia unuia dintre proiectele tale s-a schimbat"),
        "sk" to listOf("Nová žiadosť o priateľstvo", "Tvoja žiadosť o priateľstvo bola prijatá", "Nová pozvánka do projektu", "Jeden z tvojich projektov bol uzavretý", "Kľúč jedného z tvojich projektov sa zmenil"),
        "sl" to listOf("Nova prošnja za prijateljstvo", "Tvoja prošnja za prijateljstvo je bila sprejeta", "Novo povabilo v projekt", "Eden od tvojih projektov je bil zaprt", "Ključ enega od tvojih projektov se je spremenil"),
        "sq" to listOf("Kërkesë e re miqësie", "Kërkesa jote për miqësi u pranua", "Ftesë e re në një projekt", "Një nga projektet e tua u mbyll", "Çelësi i njërit prej projekteve të tua ndryshoi"),
        "sr" to listOf("Novi zahtev za prijateljstvo", "Tvoj zahtev za prijateljstvo je prihvaćen", "Nova pozivnica za projekat", "Jedan od tvojih projekata je zatvoren", "Ključ jednog od tvojih projekata je promenjen"),
        "sv" to listOf("Ny vänförfrågan", "Din vänförfrågan har accepterats", "Ny inbjudan till ett projekt", "Ett av dina projekt har stängts", "Nyckeln till ett av dina projekt har ändrats"),
        "tr" to listOf("Yeni arkadaşlık isteği", "Arkadaşlık isteğin kabul edildi", "Bir projeye yeni davet", "Projelerinden biri kapatıldı", "Projelerinden birinin anahtarı değişti"),
        "uk" to listOf("Новий запит у друзі", "Твій запит у друзі прийнято", "Нове запрошення до проєкту", "Один із твоїх проєктів закрито", "Ключ одного з твоїх проєктів змінився"),
    )
    private val kinds = listOf("friend_request", "friend_accepted", "project_invitation", "project_closed", "key_changed")

    fun body(kind: String, lang: String): String? {
        val index = kinds.indexOf(kind).takeIf { it >= 0 } ?: return null
        return (texts[lang] ?: texts.getValue("en"))[index]
    }
}
