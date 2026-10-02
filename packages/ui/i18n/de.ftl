# Deutsch. Vollständig bis auf die Rechtstexte (legal-, terms-, privacy-), die nur auf Englisch und
# Französisch vorliegen und pro Nachricht auf en.ftl zurückfallen.

### Common

loading = Wird geladen…
cancel = Abbrechen
retry = Erneut versuchen
delete = Löschen
back = Zurück
language = Sprache

### Navigation

nav-main = Hauptnavigation
nav-projects = Projekte
nav-charts = Statistiken
nav-settings = Einstellungen

### Connectivity

offline-banner = Offline
offline-pending =
    { $count ->
        [one] { $count } ausstehend
       *[other] { $count } ausstehend
    }

sync-conflict-edit = Konflikt: Bearbeiten von „{ $name }“ fehlgeschlagen (Element gelöscht). Übersprungen.
sync-conflict-delete = Konflikt: Löschen von „{ $name }“ fehlgeschlagen (Element gelöscht). Übersprungen.
sync-conflict-other = Konflikt: Vorgang für „{ $name }“ fehlgeschlagen (Element gelöscht). Übersprungen.
sync-error = Synchronisierungsfehler: { $reason }

### Errors

error-network = Der Server ist nicht erreichbar. Prüfe deine Internetverbindung.
error-generic = Etwas ist schiefgelaufen. Bitte versuche es erneut.

error-invalid-email = Diese E-Mail-Adresse ist ungültig.
error-invalid-password = Dieses Passwort ist ungültig.
error-password-too-short = Das Passwort muss mindestens 8 Zeichen lang sein.
error-client-outdated = Diese Version der App ist veraltet. Bitte aktualisieren Sie sie, um sich anzumelden.
error-invalid-link = Dieser Link ist ungültig.
error-batch-too-large = Zu viele Einträge auf einmal.
error-payers-required = Wähle mindestens eine zahlende Person.
error-debtors-required = Wähle mindestens eine Person, die etwas schuldet.
error-duplicate-participant = Eine Person kommt auf derselben Seite doppelt vor.
error-participant-not-in-project = Diese Person gehört nicht zu diesem Projekt.
error-too-many-participants = Zu viele Beteiligte für eine einzelne Ausgabe.
error-invalid-credentials = E-Mail oder Passwort ist falsch.
error-unauthenticated = Melde dich an, um das zu tun.
error-email-not-verified = Deine E-Mail-Adresse ist noch nicht bestätigt.
error-claim-proof-invalid = Dieses Gerät besitzt den Projektschlüssel nicht und kann daher keinen Teilnehmer beanspruchen. Öffnen Sie den Freigabelink erneut.
error-project-not-found = Dieses Projekt existiert nicht mehr.
error-expense-not-found = Diese Ausgabe existiert nicht mehr.
error-storage-full = Der Speicher ist voll: Der Schlüssel dieses Projekts konnte auf diesem Gerät nicht gespeichert werden. Bewahre den Freigabelink auf.
error-user-not-found = Diese Person existiert nicht mehr.
error-tricount-not-found = Tricount nicht gefunden, oder dessen API hat einen Fehler geliefert.
error-too-many-members = Dieses Projekt hat sein Limit an Beteiligten erreicht.
error-user-has-payments = Diese Person hat Ausgaben im Projekt und kann nicht entfernt werden.
error-resend-cooldown = Warte 60 Sekunden, bevor du eine weitere E-Mail anforderst.

### Auth

field-email = E-Mail
field-email-placeholder = du@beispiel.de
field-password = Passwort
field-name = Name
field-name-placeholder = Max Mustermann

login-title = Anmelden
login-submit = Anmelden
login-submitting = Anmeldung läuft…
login-password-placeholder = Dein Passwort
login-no-account = Noch kein Konto?
login-unverified = Deine E-Mail-Adresse ist noch nicht bestätigt. Sieh in deinem Postfach nach oder lass den Link erneut schicken.
login-resend = Bestätigungslink erneut senden
login-resending = Wird gesendet…
login-resend-sent = E-Mail gesendet – sieh in deinem Postfach nach.

register-submit = Konto erstellen
register-submitting = Wird erstellt…
register-have-account = Schon ein Konto?
register-password-placeholder = Mindestens 8 Zeichen
register-password-warning = Notiere dir dein Passwort. Wenn du es vergisst, lässt sich dein Konto nicht wiederherstellen.
register-check-email-title = Sieh in dein Postfach
register-email-sent = E-Mail gesendet
register-email-sent-hint = Klicke auf den Link in deinem Postfach, um dein Konto zu aktivieren.
register-not-received-prefix = Nichts bekommen? Sieh im Spam-Ordner nach, oder
register-sign-in-link = melde dich an
register-not-received-suffix = um den Link erneut zu senden.
register-terms-prefix = Mit dem Erstellen eines Kontos akzeptierst du unsere
register-terms-link = Nutzungsbedingungen
register-terms-and = und unsere
register-privacy-link = Datenschutzerklärung

settings-title = Einstellungen
settings-preferences = Voreinstellungen
settings-preferences-local = Auf diesem Gerät gespeichert.
settings-preferences-synced = Verschlüsselt mit Ihrem Konto synchronisiert.
settings-about = Über
settings-anonymous-title = Sie sind nicht angemeldet
settings-upsell-title = Ihre Projekte, auf jedem Gerät
settings-upsell-free = Kostenlos
settings-upsell-body = Counted funktioniert ohne Konto. Mit einem kostenlosen Konto begleiten Sie Ihre Projekte und Einstellungen auf Handy, Laptop und im Web - weiterhin verschlüsselt, weiterhin für uns unlesbar.
settings-locked-badge = Konto
settings-locked-friends = Erstellen Sie ein Konto, um Freunde hinzuzufügen und sie direkt aus der App in ein Projekt einzuladen - ohne Link zum Weitergeben.
settings-locked-payment-methods = Speichern Sie Ihre IBAN oder Bezahl-App einmal und teilen Sie sie mit den Projekten Ihrer Wahl. Wer Ihnen Geld schuldet, sieht sie neben Ihrem Namen.
settings-friends-hint = Fügen Sie Freunde hinzu und laden Sie sie in Ihre Projekte ein, ohne einen Link zu teilen.

account-member-since = Mitglied seit
account-logout = Abmelden
account-logging-out = Abmeldung läuft…
account-delete-title = Mein Konto löschen
account-delete-warning = Sofort und endgültig, ohne Papierkorb. Ausgaben, die du in einem geteilten Projekt eingetragen hast, bleiben für die anderen Beteiligten sichtbar - sie gehören zu deren Abrechnung.
account-delete-confirm-title = Konto löschen
account-delete-confirm-message = Dein Konto, deine Sitzungen und deine Projektliste werden endgültig gelöscht. Ohne dein Passwort sind die verschlüsselten Daten eines geteilten Projekts für dich nicht mehr lesbar - das lässt sich nicht rückgängig machen.

settings-payment-methods = Zahlungsdaten
settings-payment-methods-hint = Wie du dein Geld zurückbekommen möchtest. Mit deinem Konto verschlüsselt.
payment-method-kind = Methode
payment-method-kind-other = Andere
payment-method-label = Name
payment-method-label-placeholder = Hauptkonto
payment-method-value = Daten
payment-method-value-placeholder = IBAN, Telefonnummer, Benutzername…
payment-method-add = Hinzufügen
payment-method-remove = { $name } entfernen
payment-method-empty = Du hast noch keine Zahlungsdaten hinterlegt.
payment-method-deleted = Zahlungsmethode gelöscht.
payment-method-value-required = Fülle die Daten jeder Zahlungsmethode aus oder entferne sie.
payment-method-label-required = Gib deiner eigenen Methode einen Namen.
payment-method-too-long = Das ist zu lang - kürze es.
payment-method-invalid-characters = Entferne Zeilenumbrüche oder unsichtbare Zeichen.
payment-method-limit = Du kannst bis zu { $max } Zahlungsmethoden speichern.
payment-methods-saved = Zahlungsdaten gespeichert.
payment-methods-offline = Du musst online sein, um deine Zahlungsdaten zu speichern.
payment-methods-stale = Deine Zahlungsdaten wurden auf einem anderen Gerät geändert. Sie wurden neu geladen — bitte versuche es erneut.
payment-methods-key-missing = Melde dich erneut an, um deine Zahlungsdaten zu verwalten.

verify-email-checking = E-Mail-Adresse wird bestätigt…
verify-email-welcome = E-Mail bestätigt - willkommen bei Counted!
verify-email-back-to-login = Zurück zur Anmeldung

### Project status

project-close = Abschließen
project-archive = Archivieren
project-reopen = Wieder öffnen
project-unarchive = Aus dem Archiv holen
project-sheet-invite = Einladen
project-sheet-recurring = Wiederkehrend
project-sheet-edit = Projekt bearbeiten
project-sheet-close = Projekt abschließen
project-sheet-archive = Projekt archivieren
project-sheet-reopen = Projekt wieder öffnen
project-sheet-unarchive = Projekt aus dem Archiv holen
project-sheet-leave = Projekt verlassen

### Dates

date-long = { $day }. { $month } { $year }

month-1 = Januar
month-2 = Februar
month-3 = März
month-4 = April
month-5 = Mai
month-6 = Juni
month-7 = Juli
month-8 = August
month-9 = September
month-10 = Oktober
month-11 = November
month-12 = Dezember

month-short-1 = Jan
month-short-2 = Feb
month-short-3 = Mär
month-short-4 = Apr
month-short-5 = Mai
month-short-6 = Jun
month-short-7 = Jul
month-short-8 = Aug
month-short-9 = Sep
month-short-10 = Okt
month-short-11 = Nov
month-short-12 = Dez

### Actions

add = Hinzufügen
create = Erstellen
creating = Wird erstellt…
edit = Bearbeiten
leave = Verlassen
close = Schließen
paste = Einfügen
join = Beitreten
import = Importieren
importing = Import läuft…
field-description = Beschreibung
field-optional = Optional

### Projects

projects-filter-active = Aktiv
projects-filter-all = Alle
projects-count-label = Projekte
projects-empty = Keine Projekte
projects-empty-hint = Erstelle ein Projekt über den Button unten
projects-offline-banner = Offline-Daten - verbinde dich neu, um zu aktualisieren.
demo-banner = Demo-Projekt - nur lesen.
demo-start-own = Eigenes Projekt starten
projects-no-local-data = Keine lokalen Daten
projects-no-local-data-hint = Melde dich an, um deine Projekte zum ersten Mal zu laden.
projects-add = Projekt hinzufügen
projects-create = Projekt erstellen
projects-join = Projekt beitreten
projects-import-tricount = Aus Tricount importieren
project-actions = Projektaktionen

status-ongoing = Laufend
status-closed = Abgeschlossen
status-archived = Archiviert

nav-help = Hilfe
nav-privacy = Datenschutzerklärung
nav-terms = Nutzungsbedingungen
nav-legal = Impressum

leave-project-title = Projekt verlassen?
leave-project-message = Du verlierst den Zugriff von diesem Gerät. Bleibt niemand mehr übrig, werden das Projekt und alle Ausgaben endgültig gelöscht.

add-project-title = Neues Projekt
add-project-name-label = Projektname
add-project-name-placeholder = Meine Reise, WG 2024…
add-project-participants = Beteiligte
add-project-participant-name = Name der Person
add-project-participant-placeholder = Clark Kent
add-project-offline = Offline lässt sich kein Projekt erstellen. Verbinde dich neu und versuche es noch einmal.
add-project-name-required = Das Projekt braucht einen Namen.

join-link-label = Freigabelink
join-link-hint = Der Link enthält den Entschlüsselungsschlüssel - kopiere ihn vollständig.
join-invalid-link = Dieser Link ist ungültig. Füge den vollständigen Freigabelink ein, samt dem Teil nach dem #.
join-wrong-project = Dieser Link gehört zu einem anderen Projekt.

import-tricount-link-label = Tricount-Link oder -Schlüssel
import-tricount-key-required = Gib einen Tricount-Link oder -Schlüssel ein.
import-tricount-encryption-failed = Verschlüsselung fehlgeschlagen.
import-tricount-unimportable = Nichts wurde importiert: Dieser Tricount enthält Mitglieder mit Tricount-Konto oder Beträge, die nicht aufgehen (betroffene Einträge: { $count }).

### Expenses

save = Speichern
saving = Wird gespeichert…
adding = Wird hinzugefügt…
link-copied = Link kopiert
missing-encryption-key = Verschlüsselungsschlüssel fehlt.
missing-encryption-key-title = Verschlüsselungsschlüssel fehlt
missing-encryption-key-hint = Der Link, den du verwendet hast, enthält nicht den Schlüssel, mit dem sich dieses Projekt entschlüsseln lässt. Nimm den vollständigen Link der Person, die es angelegt hat.
project-locked-hint = Dieses Gerät hat den Schlüssel für dieses Projekt nicht. Öffne den Freigabelink, um es zu entsperren.
project-unlock = Entsperren
project-no-local-data-hint = Melde dich an, um die Daten dieses Projekts zum ersten Mal zu laden.
project-gone-hint = Es wurde gelöscht, als die letzte beteiligte Person es verlassen hat. Der Freigabelink funktioniert nicht mehr, auch wenn du ihn erneut öffnest.

expense-add = Ausgabe hinzufügen
transfer-add = Überweisung hinzufügen
expense-edit-title = Ausgabe bearbeiten
expense-category = Kategorie
expense-delete-title = Ausgabe löschen
expense-delete-message = „{ $name }“ wird endgültig gelöscht. Das lässt sich nicht rückgängig machen.
expense-inconsistent-amounts = Beträge passen nicht zusammen
expenses-empty = Keine Ausgaben
expenses-empty-hint = Füge über den Button unten die erste Ausgabe hinzu

expense-type-expense = Ausgabe
expense-type-transfer = Überweisung
expense-type-gain = Einnahme
expense-paid-by = bezahlt von
expense-sent-by = gesendet von
expense-contributed-by = beigetragen von

expense-name-required = Ein Name ist erforderlich.
expense-amount-not-positive = Der Betrag muss größer als 0 sein.
expense-no-payer = Wähle mindestens eine zahlende Person.
expense-no-debtor = Wähle mindestens eine Person, die etwas schuldet.
expense-invalid-date = Dieses Datum ist ungültig.
expense-payers-mismatch = Die Zahlenden ergeben { $sum }, das passt nicht zum Betrag der Ausgabe ({ $total }).
expense-debtors-mismatch = Die Schuldner ergeben { $sum }, das passt nicht zum Betrag der Ausgabe ({ $total }).

participants-none = Niemand
participants-everyone = Alle ({ $count })
participants-some = { $count } von { $total }
participants-select-all = Alle auswählen
participants-remaining = { $amount } fehlen
participants-over-by = { $amount } zu viel
participants-who-paid = Wer hat bezahlt?
participants-who-received = Wer hat bekommen?
participants-who-transfers = Wer überweist?
participants-who-receives = Wer bekommt?
participants-for-whom = Für wen?

stats-total-expenses = Ausgaben gesamt
stats-my-expenses = Meine Ausgaben

tab-expenses = Ausgaben
tab-balance = Bilanz
tab-reimbursements = Ausgleichen
balance-gets-back = Bekommt zurück
balance-owes = Schuldet
balance-settled = Ausgeglichen
reimbursements-empty-hint = Hier erscheinen Ausgleichsvorschläge, sobald die Konten nicht aufgehen
reimbursement-owes = { $debtor } schuldet { $creditor }

user-selection-title = Welche Person bist du?
user-selection-hint = Wähle deinen Namen aus der Liste.
user-selection-required = Bitte wähle eine Person aus.

edit-project-deferred-new-members = das Hinzufügen neuer Beteiligter
edit-project-deferred-removals = das Entfernen von Beteiligten
edit-project-offline-deferred = Offline: { $items } wird übernommen, sobald du wieder verbunden bist.

export-failed = Export fehlgeschlagen: { $reason }

history-expense-added = Ausgabe hinzugefügt: { $name }
history-expense-edited = Ausgabe bearbeitet: { $name }
history-expense-deleted = Ausgabe gelöscht: { $name }
history-project-edited = Projekt bearbeitet: { $name }
history-name-changed = Name: „{ $from }“ → „{ $to }“
history-description-added = Beschreibung hinzugefügt: „{ $value }“
history-description-removed = Beschreibung entfernt: „{ $value }“
history-description-changed = Beschreibung: „{ $from }“ → „{ $to }“

### Sweep

field-amount = Betrag
expense-name-placeholder = Restaurant, Einkauf…
expense-actions = Aktionen für die Ausgabe
expense-your-share = Dein Anteil
expense-your-share-value = Dein Anteil: { $amount } { $currency }
expense-inconsistent-detail = Beträge passen nicht zusammen: { $paid } bezahlt, { $owed } geschuldet, bei einer Ausgabe von { $total }. Bearbeite die Ausgabe, um das zu korrigieren.
missing-access-key = Zugriffsschlüssel fehlt. Öffne dieses Projekt über seinen Freigabelink.
filter-all = Alle
filter-my-payments = Meine Zahlungen
filter-my-debts = Was ich schulde
participants-shares-for = Anteile von { $name }
participants-amount-for = Betrag für { $name }
reimbursement-add = Ausgleich hinzufügen
project-forget = Aus meiner Liste entfernen
project-history-title = Verlauf
history-kind-add = Hinzugefügt
history-kind-delete = Gelöscht
history-kind-edit = Bearbeitet
export = Exportieren
export-json = JSON exportieren
export-csv = CSV exportieren
share-link = Teilen
copy-link-failed = Der Link konnte nicht kopiert werden
open-in-app = In der App öffnen
not-found-title = Seite nicht gefunden
not-found-back = Zurück zu den Projekten

### Charts

charts-period = Zeitraum
period-all = Alle
period-month = Monat
period-3months = 3 Mon.
period-year = Jahr
period-custom = Eigen
charts-tab-categories = Kategorien
charts-tab-trends = Verlauf
charts-total-spent = Gesamt ausgegeben
charts-avg-per-person = Ø pro Person
charts-expense-count =
    { $count ->
        [one] { $count } Ausgabe
       *[other] { $count } Ausgaben
    }
charts-nothing-to-show = Nichts anzuzeigen
charts-my-share-note = Diese Beträge sind dein Anteil an jeder Ausgabe.
charts-my-share-skipped =
    { $count ->
        [one] 1 Projekt zählt nicht mit — keine Person ausgewählt, oder die Daten konnten nicht geladen werden.
       *[other] { $count } Projekte zählen nicht mit — keine Person ausgewählt, oder die Daten konnten nicht geladen werden.
    }

### Categories

category-food = Essen
category-transport = Transport
category-accommodation = Unterkunft
category-leisure = Freizeit
category-shopping = Shopping
category-services = Dienstleistungen
category-parties-gifts = Feiern & Geschenke
category-other = Sonstiges
history-empty = Keine Ereignisse
not-found-hint = Diese Seite gibt es nicht, oder sie ist umgezogen.
payers-title-paid-by = Bezahlt von
payers-title-sender = Absender
payers-title-contributors = Beitragende
debtors-title-debtors = Schuldet
debtors-title-recipients = Empfänger
debtors-title-beneficiaries = Begünstigte

### Welcome

welcome-title = Deine Abrechnung geht nur dich etwas an.
welcome-subtitle = Teile Ausgaben mit Freunden.
welcome-note = Kostenlos. Kein Konto nötig. Keine Werbung.
welcome-link-title = Ein Link, und alle machen mit.
welcome-link-body = Niemand muss ein Konto anlegen.
welcome-link-account = Ein Konto? Nie Pflicht. Damit findest du deine Projekte auf einem anderen Gerät wieder, lädst Freunde direkt aus der App ein und teilst deine Zahlungsdaten.
welcome-demo-project = Wochenende in Lyon
welcome-private-title = Niemand kann deine Abrechnungen lesen. Nicht einmal wir.
welcome-private-body = Namen, Beträge, Projekte: alles wird auf deinem Gerät verschlüsselt. Nur du hast den Schlüssel.
welcome-private-names = Namen
welcome-private-amounts = Beträge
welcome-private-projects = Projekte
welcome-scan-title = Fotografiere den Beleg.
welcome-scan-body = Betrag, Datum und Kategorie füllen sich von selbst aus. Alles passiert auf deinem Handy. Das Foto wird nicht gespeichert.
welcome-eu-title = 100 % europäisch
welcome-no-ads = Keine Werbung
welcome-no-trackers = Keine Tracker
welcome-step = Schritt { $current } von { $total }
welcome-next = Weiter
welcome-skip = Überspringen
welcome-start = Loslegen
welcome-how-it-works = Wie funktioniert das genau?

### Help

help-intro = Eine häufige Frage? Tippe darauf, um die Antwort aufzuklappen.
help-create-project-q = Wie lege ich ein Projekt an?
help-create-project-a = Tippe auf dem Startbildschirm unten auf +. Gib dem Projekt einen Namen, wähle die Währung, fertig.
help-add-participants-q = Wie füge ich Beteiligte hinzu?
help-add-participants-a = Öffne das Projekt und füge Beteiligte über die Mitgliederliste hinzu. Jede beteiligte Person kann bei einer Ausgabe bezahlen oder etwas schulden.
help-share-project-q = Wie teile ich ein Projekt?
help-share-project-a = Teile die Projekt-URL (die aus der Adressleiste). Wer den Link hat, kann das Projekt ansehen und bearbeiten.
help-add-expense-q = Wie füge ich eine Ausgabe hinzu?
help-add-expense-a = Tippe im Projekt auf +, gib den Betrag ein und lege fest, wer bezahlt hat und zwischen wem aufgeteilt wird. Du kannst auch ein anderes Datum als heute wählen.
help-types-q = Was ist der Unterschied zwischen Ausgabe, Überweisung und Einnahme?
help-types-expense = - ein Kauf, den eine Person tätigt und der zwischen mehreren aufgeteilt wird.
help-types-transfer = - eine Rückzahlung von einer Person an eine andere, ohne Aufteilung.
help-types-gain = - erhaltenes Geld (Erstattung, Geschenk), das zwischen mehreren aufgeteilt wird.
help-past-date-q = Kann ich eine Ausgabe auf ein früheres Datum setzen?
help-past-date-a = Ja, das Datumsfeld ist frei. Der Erstellungszeitpunkt des Eintrags wird separat gespeichert.
help-who-owes-q = Wie ermittelt Counted, wer was schuldet?
help-who-owes-a = Counted berechnet den Saldo jeder Person (Ausgelegtes minus Geschuldetes) und schlägt dann die kürzeste Reihe von Überweisungen vor, mit der alles ausgeglichen wird.
help-minimal-transfers-q = Warum ist die Zahl der vorgeschlagenen Überweisungen minimal?
help-minimal-transfers-a = Der Algorithmus paart zuerst Salden, die sich exakt aufheben, und arbeitet den Rest dann vom größten Gläubiger zum größten Schuldner ab. Ergebnis: weniger Überweisungen, um alles auszugleichen.
help-import-tricount-q = Wie importiere ich ein Projekt aus Tricount?
help-import-tricount-a = Tippe auf dem Startbildschirm unten auf „+“ und dann auf
help-import-tricount-b = Füge den Freigabelink des Tricounts ein, den du importieren willst.
help-encryption-q = Sind meine Daten verschlüsselt?
help-encryption-a = Ja. Counted kombiniert zwei Zusagen:
help-encryption-e2ee-term = Ende-zu-Ende-Verschlüsselung
help-encryption-e2ee-def = - alles zwischen dir und dem Server ist unterwegs verschlüsselt.
help-encryption-zero-term = Kein Zugriff
help-encryption-zero-def = - du verschlüsselst die Daten vor dem Senden, und der Server speichert nur Chiffrat. Wir können es nicht lesen.
help-encryption-see = Mehr dazu in der
help-forgot-password-q = Was passiert, wenn ich mein Passwort vergesse?
help-forgot-password-warning = Deine Daten sind dann endgültig verloren.
help-forgot-password-a = Der Schlüssel wird aus deinem Passwort abgeleitet, ein Zurücksetzen ist also nicht möglich: niemand - wir eingeschlossen - kann deine Projekte ohne dieses Passwort entschlüsseln. Bewahre es gut auf, am besten in einem Passwortmanager.
help-archive-delete-q = Wie archiviere oder lösche ich ein Projekt?
help-archive-delete-a = Öffne im Projekt das Menü und wähle
help-archive-delete-b = um es auszublenden, aber zu behalten. Endgültig gelöscht wird ein Projekt erst, wenn die letzte beteiligte Person es verlässt.
help-delete-account-q = Wie lösche ich mein Konto?
help-delete-account-a = Öffne die Einstellungen und nutze „Mein Konto löschen“. Das geschieht sofort und lässt sich nicht rückgängig machen.
help-contact = Noch eine Frage? Schreib uns an

# Receipt scanning (mobile only)
expense-scan = Beleg scannen
scan-in-progress = Beleg wird gelesen…
scan-error-capture = Das Foto konnte nicht aufgenommen werden. Versuche es erneut oder gib die Ausgabe von Hand ein.
scan-error-unreadable = Auf diesem Beleg ist nichts lesbar. Gib die Ausgabe von Hand ein.
scan-check-amount = Prüfe die Summe - sie war nicht klar gedruckt.
scan-take-photo = Foto aufnehmen
scan-choose-photo = Foto auswählen

update-required-title = Aktualisierung erforderlich
update-required-body = Diese Version von Counted ist zu alt, um mit dem Server zu kommunizieren. Aktualisieren Sie sie, um die App weiter zu nutzen.
update-required-button = Aktualisieren

### Common (Ergänzungen)

confirm = Bestätigen
copy = Kopieren
field-date = Datum
date-today = Heute
date-yesterday = Gestern

### Errors (Freunde)

error-identity-taken = Ein anderes Konto hat diesen Teilnehmer bereits übernommen.
error-self-friend-request = Du kannst dich nicht selbst als Freund hinzufügen.
error-not-a-friend = Du kannst nur Personen aus deiner Freundesliste einladen.
error-friend-has-no-key = Dieser Freund hat die neueste Version der App noch nicht geöffnet. Bitte ihn, sich einmal anzumelden, und versuche es dann erneut.
error-friend-request-not-found = Diese Freundschaftsanfrage existiert nicht mehr.
error-invitation-not-found = Diese Einladung existiert nicht mehr.
error-too-many-friend-requests = Zu viele Freundschaftsanfragen im Moment. Versuche es morgen erneut.
error-too-many-invitations = Zu viele ausstehende Einladungen.
error-invalid-kdf-salt = Die Verschlüsselungseinstellungen sind ungültig. Aktualisiere die App und versuche es erneut.
error-mixed-project-batch = Diese Beteiligten gehören nicht alle zum selben Projekt.
error-invalid-payload = Diese Version der App hat Daten gesendet, die der Server nicht akzeptiert. Aktualisiere sie und versuche es erneut.
error-invalid-public-key = Dein Verschlüsselungsschlüssel ist ungültig. Aktualisiere die App und versuche es erneut.
error-payment-methods-stale = Deine Zahlungsdaten wurden auf einem anderen Gerät geändert. Lade neu und versuche es erneut.

### Payment methods (Teilen)

settings-payment-methods-share-warning = Eine geteilte Zahlungsart ist für alle Mitglieder der Projekte sichtbar, in denen du deinen Namen gewählt hast - für jeden, der einen dieser Projektlinks besitzt.
payment-method-share = Mit meinen Projekten teilen
payment-method-share-hint = Wird neben deinem Namen angezeigt, wenn dir jemand Geld schuldet.
payment-method-copy = { $name } kopieren
payment-method-copied = Kopiert.
payment-method-copy-failed = Kopieren nicht möglich - markiere den Text und kopiere ihn von Hand.

### Expenses (Währung, Betrag)

expense-category-auto = Auto · { $emoji }
expense-currency = Währung des Betrags
amount-op-add = Plus
amount-op-subtract = Minus
amount-op-multiply = Mal
amount-op-divide = Geteilt
amount-op-equals = Gleich
amount-op-done = Fertig
expense-rate = Wechselkurs (optional)
expense-rate-hint = Leer lassen, um den Kurs der Europäischen Kommission (InforEuro) für { $month } zu verwenden: 1 { $from } = { $rate } { $to }.
expense-rate-invalid = Gib einen Wechselkurs größer als 0 ein.
expense-rate-unavailable = Kein automatischer Kurs verfügbar - gib ihn von Hand ein.
expenses-show-more = Mehr anzeigen ({ $count } weitere)
expense-converted-from = { $amount } { $from } bezahlt · 1 { $from } = { $rate } { $to }
project-currency = Währung
project-currency-hint = Alle Beträge werden in dieser Währung angezeigt. Sie kann später nicht geändert werden.
project-currency-locked = Die Währung wird beim Anlegen des Projekts festgelegt.
currency-search = Währung suchen
project-gone-title = Dieses Projekt existiert nicht mehr
participants-by-shares = Nach Anteilen
split-amounts = Beträge

### Reimbursements

reimbursements-empty-title = Alles beglichen!
reimbursement-record = Begleichen
reimbursement-pay-with = Bezahlen
reimbursement-pay-shared-by = Geteilt von { $name } - prüfe vor dem Senden den Empfängernamen, den deine App anzeigt.
reimbursement-pay-title = { $name } bezahlen
reimbursements-mine-title = Du schuldest
reimbursements-others-title = Andere Rückzahlungen

### Identity

identity-claimed = Mit einem Konto verknüpft
identity-claimed-by = Konto von { $name }
identity-taken-repick = Ein anderes Konto hat den Teilnehmer übernommen, den du benutzt hast. Bitte wähle einen anderen.
participant-gone-repick = Der Teilnehmer, den du benutzt hast, wurde aus diesem Projekt entfernt. Bitte wähle einen anderen.

### Edit project

edit-project-title = Projekt bearbeiten
edit-project-new-badge = neu

### Charts (Ergänzungen)

charts-project = Projekt
charts-all-projects = Alle Projekte
charts-date-from = Von
charts-date-to = Bis
charts-total = Gesamt
charts-tab-people = Personen
charts-tab-projects = Projekte
charts-scope = Wessen Ausgaben
charts-scope-group = Gruppe
charts-scope-me = Ich
charts-currency = Währung
charts-my-share = Mein Anteil
charts-share-of-total = { $pct } % von { $total }
charts-i-paid = Ich habe bezahlt
charts-paid-more = { $amount } mehr als dein Anteil
charts-paid-less = { $amount } weniger als dein Anteil
charts-paid-even = Genau dein Anteil
charts-part-title = Dein Anteil je Kategorie
charts-part-desc = Grau ist, was die Gruppe ausgegeben hat, farbig, was du verbraucht hast.
charts-breakdown-title = Aufteilung nach Kategorie
charts-breakdown-desc = Tippe auf ein Segment oder eine Zeile, um die Ausgaben zu sehen.
charts-of-total = { $amount } von { $total }
charts-show-all = Alle anzeigen ({ $count })
charts-show-less = Weniger anzeigen
charts-spend-title = Ausgaben im Zeitverlauf
charts-spend-desc = Kurze Zeiträume nach Tag, längere nach Woche oder Monat.
charts-group-by = Gruppieren nach
bucket-day = Tag
bucket-week = Woche
bucket-month = Monat
charts-avg = Ø
charts-cat-title-day = { $category }, Tag für Tag
charts-cat-title-week = { $category }, Woche für Woche
charts-cat-title-month = { $category }, Monat für Monat
charts-cat-desc = Wähle eine Kategorie, um sie im Zeitverlauf zu verfolgen.
charts-running-title = Laufende Summe
charts-running-desc = Seit { $date }.
charts-avg-per-day = { $amount } / Tag im Schnitt
charts-avg-per-week = { $amount } / Woche im Schnitt
charts-avg-per-month = { $amount } / Monat im Schnitt
charts-people-title = Wer die Gruppe getragen hat
charts-people-desc = Was jede Person bezahlt hat, neben dem, was sie verbraucht hat.
charts-paid = Bezahlt
charts-fair-share = Fairer Anteil
charts-you = (du)
charts-net-more = mehr bezahlt
charts-net-less = weniger bezahlt
charts-balance-title = Dein Saldo im Zeitverlauf
charts-balance-desc = Über der Linie schuldet dir die Gruppe etwas. Darunter schuldest du der Gruppe.
charts-owed = Du bekommst
charts-owe = Du schuldest
charts-projects-title = Dein Anteil pro Projekt
charts-projects-desc = Summen bleiben je Währung getrennt und werden nie addiert.
history-by = Von { $name }

### Notifications

notifications-label = Benachrichtigungen
notifications-title = Benachrichtigungen
notifications-empty = Nichts Neues
notifications-friend-request = Freundschaftsanfrage

### Friends

friends-title = Freunde
friends-anonymous-body = Freunde sind an dein Konto gebunden. Melde dich an, um Personen hinzuzufügen und sie ohne Link in deine Projekte einzuladen.
friends-add-title = Freund hinzufügen
friends-add-hint = Die Anfrage erscheint bei der nächsten Anmeldung. Keiner von euch erfährt, ob der andere ein Konto hat, bis die Anfrage angenommen ist.
friends-add-button = Hinzufügen
friends-add-from-project = Als Freund hinzufügen
friends-request-sent = Anfrage gesendet
friends-no-account-key = Melde dich erneut an, um deine Freunde auf diesem Gerät zu verwalten.
friends-incoming-title = Anfragen
friends-accept = Annehmen
friends-decline = Ablehnen
friends-list-title = Meine Freunde
friends-list-empty = Noch keine Freunde. Füge oben jemanden per E-Mail hinzu, oder aus einem gemeinsamen Projekt.
friends-remove = Entfernen
friends-remove-confirm-title = Freund entfernen
friends-remove-confirm-message = { $email } ist dann nicht mehr in deinen Freunden, und du nicht mehr in seinen. Jeder von euch kann später eine neue Anfrage senden.
friends-no-key = Noch nicht bereit
friends-fingerprint = Sicherheitscode
friends-fingerprint-hint = Zwei Freunde, die sich denselben Sicherheitscode vorlesen, wissen, dass niemand zwischen ihnen sitzt - nicht einmal unser Server.
friends-outgoing-title = Gesendet
friends-outgoing-hint = Wartet auf Antwort. Sobald sie annehmen, erscheinen sie in deinen Freunden.
friends-withdraw = Abbrechen
invite-friends-title = Freunde einladen
invite-friends-hint = Der Projektschlüssel wird auf diesem Gerät für jeden Freund verschlüsselt. Der Server sieht ihn nie.
invite-friends-empty = Noch keine Freunde zum Einladen.
invite-friends-button = Einladen
invite-sent = { $count ->
    [one] Einladung gesendet
   *[other] { $count } Einladungen gesendet
}
invitation-badge = Einladung
invitation-to = „{ $name }“ beitreten
invitation-to-unnamed = Einem Projekt beitreten
invitation-unreadable = Diese Einladung kann auf diesem Gerät nicht geöffnet werden
invitation-from = Von { $email }
invitation-accept = Beitreten
invitation-decline = Ablehnen

# Participants in the create and edit modals, and the "who are you?" picker - see
# docs/plans/friends.md §11.
participants-you-label = Dein Name in diesem Projekt
participants-you-badge = Du
participants-you-from-account = Aus deinem Kontonamen übernommen. Hier nur für dieses Projekt ändern.
participants-you-required = Pflichtfeld. So sehen dich die anderen.
participants-others = Weitere Teilnehmende
participants-empty = Noch niemand. Wähle unten einen Freund oder gib einen Namen ein.
participants-empty-signed-out = Noch niemand. Gib einen Namen ein, um jemanden hinzuzufügen.
participants-duplicate = „{ $name }“ ist bereits in der Liste.
participants-input-label = Freund hinzufügen oder Namen eingeben
participants-input-placeholder = Freund oder beliebiger Name
participants-suggest-friend = Freund · tritt als „{ $name }“ bei, erhält eine Einladung
participants-suggest-not-ready = Freund · noch nicht bereit
participants-suggest-guest = „{ $text }“ ohne Konto hinzufügen
participants-suggest-guest-sub = Kein Konto, nur ein Name
participants-friends = Deine Freunde
participants-all-friends = Alle Freunde
participants-login-hint = Melde dich an, um Leute direkt aus deiner Freundesliste hinzuzufügen.
participants-invite-badge = Einladen
participants-guest-badge = Ohne Konto
participants-guest-sub = Kein Konto, nur ein Name
participants-rename = { $name } umbenennen
participants-remove = { $name } entfernen
participants-rename-label = Neuer Name
participants-rename-save = Namen speichern
participants-rename-hint = Der Name, den alle in diesem Projekt sehen. Die Einladung geht weiterhin an { $email }.
participants-invited-badge = Eingeladen
participants-invited-sub = { $email } · noch nicht angenommen
participants-invited-pending = Einladung noch nicht angenommen
participants-unlinked = Mit keinem Konto verknüpft
add-project-create-invite = Erstellen und { $count } einladen
edit-project-save-invite = Speichern und { $count } einladen
edit-project-you-are = Auf diesem Gerät bist du { $name }
edit-project-no-identity = Du hast noch nicht gewählt, wer du bist
edit-project-switch = Wechseln
edit-project-choose = Wählen
invite-failed = Diese Einladungen konnten nicht gesendet werden: { $emails }
invite-again = Erneut einladen
friend-picker-title = Freunde hinzufügen
user-selection-invited-hint = { $email } hat dich zu „{ $project }“ eingeladen.
user-selection-suggested = Vorgeschlagen
user-selection-suggested-sub = { $email } hat dich unter diesem Namen hinzugefügt
user-selection-confirm-as = Ich bin { $name }
user-selection-missing = Dein Name fehlt? Bitte jemanden aus dem Projekt, dich in den Projekteinstellungen hinzuzufügen.

## Wiederkehrende Ausgaben

repeat-label = Wiederholen
repeat-none = Keine Wiederholung
repeat-weekly = Jede Woche
repeat-biweekly = Alle 2 Wochen
repeat-monthly = Jeden Monat
repeat-quarterly = Alle 3 Monate
repeat-yearly = Jedes Jahr
repeat-every-weeks = Alle { $count } Wochen
repeat-every-months = Alle { $count } Monate
repeat-every-years = Alle { $count } Jahre
repeat-custom = Benutzerdefiniert…
repeat-every = Alle
repeat-unit-weeks = Wochen
repeat-unit-months = Monate
repeat-unit-years = Jahre
repeat-on-weekday = am { $weekday }
repeat-on-day = am { $day }.
repeat-on-day-month = am { $day }. { $month }
repeat-month-end = In kürzeren Monaten fällt sie auf den letzten Tag.
repeat-ends = Endet
repeat-ends-never = Nie
repeat-ends-on = An einem Datum
repeat-ends-after = Nach
repeat-fewer = Weniger
repeat-more = Mehr
repeat-last-on = zuletzt am { $date }
repeat-variable = Betrag ändert sich jedes Mal
repeat-variable-hint = Jede wird mit dem letzten Betrag hinzugefügt und als „zu bestätigen“ markiert.
repeat-done = Fertig
repeat-no-end = Kein Ende
repeat-until = Bis { $date }
repeat-occurrences = Anzahl: { $count }
repeat-offline = Benötigt eine Verbindung. Die Ausgabe selbst kann trotzdem hinzugefügt werden.
repeat-foreign = Wiederholt sich mit { $amount } { $currency }, einmal zum heutigen Kurs umgerechnet. Du wirst gewarnt, wenn sich der Kurs um mehr als 5 % ändert.
repeat-backfill = Beginnt in der Vergangenheit. Jetzt hinzugefügte Ausgaben: { $count }.
add-and-repeat = Hinzufügen und wiederholen
weekday-1 = Montag
weekday-2 = Dienstag
weekday-3 = Mittwoch
weekday-4 = Donnerstag
weekday-5 = Freitag
weekday-6 = Samstag
weekday-7 = Sonntag
recurring-title = Wiederkehrende Ausgaben
recurring-strip = Wiederkehrende Ausgaben: { $count }
recurring-next = Nächste: { $name }, { $date }
recurring-to-confirm = Zu bestätigen: { $count }
recurring-per-month = Pro Monat, ca.
recurring-your-share = Dein Anteil
recurring-active = Aktiv
recurring-paused = Pausiert
recurring-finished = Beendet
recurring-paid-by = bezahlt von { $name }
recurring-next-on = Nächste am { $date }
recurring-progress = { $done } von { $total }
recurring-rate-badge = Kurs um { $percent } % verändert
recurring-empty = Noch wiederholt sich nichts. Wähle „Wiederholen“, wenn du eine Ausgabe hinzufügst: Miete, Abos, Rechnungen.
recurring-next-ones = Nächste Termine
recurring-added-so-far = Bisher hinzugefügt
recurring-set-up-by = Eingerichtet von
recurring-pause = Pausieren
recurring-resume = Fortsetzen
recurring-stop = Wiederholung beenden
recurring-stop-title = „{ $name }“ beenden?
recurring-stop-message = Sie wiederholt sich nicht mehr. Bereits hinzugefügte Ausgaben bleiben.
recurring-resume-title = „{ $name }“ fortsetzen?
recurring-resume-message = Nächste am { $date }. Während der Pause verpasste Termine werden nicht hinzugefügt.
recurring-edit-title = Wiederkehrende Ausgabe bearbeiten
recurring-edit-banner = Änderungen gelten ab { $date }. Bereits hinzugefügte Ausgaben bleiben unverändert.
recurring-next-on-label = Nächste am
recurring-next-too-early = Das nächste Datum muss nach der zuletzt hinzugefügten Ausgabe liegen.
recurring-use-stop = Zum Beenden „Wiederholung beenden“ in der wiederkehrenden Ausgabe verwenden.
recurring-drift = Der Kurs von { $currency } hat sich seit der Einrichtung um { $percent } % verändert. Jede wird weiterhin mit { $amount } { $project_currency } hinzugefügt (1 { $currency } = { $rate }). Zum heutigen Kurs wären es { $today_amount } { $project_currency }.
recurring-use-rate = Heutigen Kurs verwenden
recurring-keep = { $amount } { $currency } behalten
recurring-added = Wiederkehrende Ausgaben hinzugefügt: { $count }
recurring-blocks-removal = { $name } kann noch nicht entfernt werden: Teil von { $rules }. Nimm { $name } dort heraus oder beende sie, dann erneut speichern.
history-recurring-added = Automatisch hinzugefügt: { $name } ({ $date })
history-recurring-created = Wiederkehrende Ausgabe eingerichtet: { $name }
history-recurring-edited = Wiederkehrende Ausgabe bearbeitet: { $name }
history-recurring-paused = Wiederkehrende Ausgabe pausiert: { $name }
history-recurring-resumed = Wiederkehrende Ausgabe fortgesetzt: { $name }
history-recurring-stopped = Wiederkehrende Ausgabe beendet: { $name }
occurrence-recurring = Wiederkehrende Ausgabe
occurrence-auto = Automatisch aus einer wiederkehrenden Ausgabe hinzugefügt.
occurrence-auto-next = Automatisch aus einer wiederkehrenden Ausgabe hinzugefügt. Nächste am { $date }.
occurrence-auto-stopped = Automatisch aus einer inzwischen beendeten wiederkehrenden Ausgabe hinzugefügt.
occurrence-manage = Verwalten
estimate-badge = Zu bestätigen
estimate-title = Betrag zu bestätigen.
estimate-body = Mit dem vorherigen Betrag hinzugefügt. Gib den echten ein, sobald er feststeht.
estimate-confirm = Betrag bestätigen
apply-to = Anwenden auf
apply-this-only = Nur diese Ausgabe
apply-and-next = Diese und die nächsten
apply-and-next-hint = Die wiederkehrende Ausgabe ändert sich ab { $date }
apply-rule-failed = Die Ausgabe wurde gespeichert, die wiederkehrende Ausgabe aber nicht geändert.
occurrence-delete-message = „{ $name }“ vom { $date } wird endgültig gelöscht. Die Wiederholung läuft weiter, dieses Datum kommt nicht zurück.
occurrence-delete-one = Nur diese löschen
occurrence-delete-stop = Löschen und Wiederholung beenden
error-recurring-clock = Die Uhr dieses Geräts geht vor. Prüfe Datum und Uhrzeit.
error-recurring-not-found = Diese wiederkehrende Ausgabe existiert nicht mehr.
error-recurring-stale = Jemand hat diese wiederkehrende Ausgabe inzwischen geändert. Sie wurde neu geladen: prüfen und erneut speichern.
error-too-many-recurring = Dieses Projekt hat bereits 50 wiederkehrende Ausgaben. Beende eine nicht mehr benötigte, um eine neue hinzuzufügen.
error-user-in-recurring = Diese Person ist Teil einer wiederkehrenden Ausgabe. Zuerst dort entfernen oder die Ausgabe beenden.
