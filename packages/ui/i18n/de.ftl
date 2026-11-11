# Deutsch. Vollständig bis auf die Rechtstexte (legal-, terms-, privacy-), die nur auf Englisch und
# Französisch vorliegen und pro Nachricht auf en.ftl zurückfallen.

### Common

loading = Wird geladen…
cancel = Abbrechen
discard-title = Änderungen verwerfen?
discard-message = Deine Eingaben gehen verloren.
discard = Verwerfen
retry = Erneut versuchen
delete = Löschen
back = Zurück
language = Sprache
theme = Design
theme-light = Hell
theme-dark = Dunkel
theme-system = System
theme-hint-device = Nur auf diesem Gerät gespeichert.
theme-hint-system = Folgt dem Erscheinungsbild dieses Geräts.

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
error-rate-limited = Der Server ist gerade ausgelastet. Versuch es gleich noch mal.
error-generic = Etwas ist schiefgelaufen. Bitte versuche es erneut.

error-invalid-email = Diese E-Mail-Adresse ist ungültig.
error-invalid-password = Dieses Passwort ist ungültig.
error-password-too-short = Das Passwort muss mindestens 12 Zeichen lang sein.
error-client-outdated = Diese Version der App ist veraltet. Bitte aktualisiere sie, um dich anzumelden.
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
error-claim-proof-invalid = Dieses Gerät besitzt den Projektschlüssel nicht und kann daher keinen Teilnehmer beanspruchen. Öffne den Freigabelink erneut.
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
login-maybe-unverified = Gerade registriert? Bestätige zuerst deine Adresse mit dem Link aus unserer E-Mail oder lass ihn dir erneut senden.
login-resend = Bestätigungslink erneut senden
login-resending = Wird gesendet…
login-resend-sent = E-Mail gesendet – sieh in deinem Postfach nach.

register-submit = Konto erstellen
register-submitting = Wird erstellt…
register-have-account = Schon ein Konto?
register-password-placeholder = Mindestens 12 Zeichen
register-password-warning = Notiere dir dein Passwort: wir sehen es nie und können es dir nicht zurückgeben.
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
settings-preferences-synced = Verschlüsselt mit deinem Konto synchronisiert.
settings-anonymous-title = Du bist nicht angemeldet
settings-upsell-title = Deine Projekte, auf jedem Gerät
settings-upsell-free = Kostenlos

account-logout = Abmelden
account-logging-out = Abmeldung läuft…
account-delete-title = Mein Konto löschen
account-delete-confirm-title = Konto löschen
account-delete-confirm-message = Dein Konto, deine Sitzungen und deine Projektliste werden endgültig gelöscht. Ohne dein Passwort sind die verschlüsselten Daten eines geteilten Projekts für dich nicht mehr lesbar - das lässt sich nicht rückgängig machen.
erase-device-title = Dieses Gerät löschen?
erase-device-body = Die Projekte, ihre Schlüssel und der Cache auf diesem Gerät werden gelöscht, danach endet deine Sitzung. Deine Projekte bleiben in deinem Konto: Melde dich wieder an, um sie zurückzubekommen.
erase-device-checking = Prüfe, ob deine Projekte in deinem Konto sind…
erase-device-pending = Noch nicht gesendete Änderungen: { $count }. Geh wieder online und versuch es dann erneut.
erase-device-unreachable = Konnte nicht prüfen, ob deine Projekte in deinem Konto sind. Prüf deine Verbindung und versuch es dann erneut.
erase-device-at-risk = Diese Projekte sind nicht in deinem Konto und gehen verloren. Um sie zu behalten, brich ab und kopiere zuerst ihren Link:
erase-device-confirm = Löschen und abmelden
erase-device-confirm-anyway = Trotzdem löschen
erase-device-erasing = Wird gelöscht…

settings-payment-methods = Zahlungsdaten
payment-method-kind = Methode
payment-method-kind-other = Andere
payment-method-label = Name
payment-method-label-placeholder = Hauptkonto
payment-method-value = Daten
payment-method-value-placeholder = IBAN, Telefonnummer, Benutzername…
payment-method-add = Hinzufügen
payment-method-deleted = Zahlungsmethode gelöscht.
payment-method-value-required = Fülle die Daten jeder Zahlungsmethode aus oder entferne sie.
payment-method-label-required = Gib deiner eigenen Methode einen Namen.
payment-method-too-long = Das ist zu lang - kürze es.
payment-method-invalid-characters = Entferne Zeilenumbrüche oder unsichtbare Zeichen.
payment-method-limit = Du kannst bis zu { $max } Zahlungsmethoden speichern.
payment-methods-saved = Zahlungsdaten gespeichert.
payment-methods-offline = Du musst online sein, um deine Zahlungsdaten zu speichern.
payment-methods-stale = Deine Zahlungsdaten wurden auf einem anderen Gerät geändert. Sie wurden neu geladen - bitte versuche es erneut.
payment-methods-key-missing = Melde dich erneut an, um deine Zahlungsdaten zu verwalten.

verify-email-checking = E-Mail-Adresse wird bestätigt…
verify-email-welcome = E-Mail bestätigt - willkommen bei Counted!
verify-email-back-to-login = Zurück zur Anmeldung
verify-email-intro = Gib dein Passwort ein, um zu bestätigen, dass diese Adresse dir gehört.
verify-email-submit = Meine Adresse bestätigen

### Project status

project-archive = Archivieren
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
projects-offline-banner = Offline-Daten - verbinde dich neu, um zu aktualisieren.
demo-banner = Demo-Projekt - nur lesen.
demo-start-own = Eigenes Projekt starten
demo-badge = Demo
demo-bar = Deine Änderungen verschwinden, wenn du diesen Tab schließt.
demo-leave = Richtig loslegen
demo-preparing = Deine Demo wird vorbereitet…
demo-failed = Die Demo konnte nicht vorbereitet werden.
demo-sign-in-blocked = In der Demo ist die Anmeldung deaktiviert. Verlasse sie, um dich anzumelden oder ein Konto zu erstellen; deine eigenen Projekte bleiben unverändert.
projects-no-local-data = Keine lokalen Daten
projects-no-local-data-hint = Melde dich an, um deine Projekte zum ersten Mal zu laden.
projects-add = Projekt hinzufügen
projects-create = Projekt erstellen
projects-personal-expense = Eigene Ausgabe
personal-expense-banner = Zu Meine Ausgaben hinzugefügt
personal-create-title = Dein eigenes Projekt anlegen
personal-create-hint = Behalte im Blick, was du allein ausgibst
group-create-title = Gemeinsames Projekt erstellen
group-create-hint = Teile Ausgaben mit Freunden und Familie
personal-create-action = Anlegen
personal-create-signup = Konto erstellen
personal-sheet-title = Dein eigenes Projekt
personal-sheet-intro = Für das, was du allein ausgibst. Es erscheint mit deinen Gruppen in der Statistik.
personal-sheet-point-alone = Ein Mitglied: du, mit deinem Kontonamen. Kein Teilen-Link.
personal-sheet-point-encrypted = Auf deinen Geräten verschlüsselt, wie deine anderen Projekte.
personal-sheet-point-stats = Deine Anteile in Gruppen kommen in der Statistik zu diesen Ausgaben hinzu.
personal-default-name = Meine Ausgaben
my-spending-prev-month = Vorheriger Monat
my-spending-next-month = Nächster Monat
my-spending-spent = Du hast ausgegeben
my-spending-groups = Gruppen
my-spending-income = Einnahmen
my-spending-balance = Monatssaldo
my-spending-unconverted = Anteile in einer anderen Währung bleiben außen vor, bis ein Kurs bekannt ist.
my-spending-empty = Nichts in diesem Monat
my-spending-empty-hint = Füge mit + eine Ausgabe hinzu. Deine Anteile in Gruppen erscheinen hier automatisch.
my-spending-share-of = von { $amount }
my-spending-uncounted = nicht gezählt
payer-you = dir
expense-received-by = erhalten von
counted-switch = In Meine Ausgaben zählen
counted-switch-on = An: dein Anteil zählt zu Meine Ausgaben
counted-switch-off = Aus: dein Anteil erscheint nicht in Meine Ausgaben
counted-groups = Gezählte Gruppen
counted-groups-intro = Dein Anteil in diesen Gruppen zählt zu Meine Ausgaben und zu den Budgets.
counted-groups-converted = { $from }, umgerechnet in { $to }
counted-groups-encrypted = Diese Wahl ist mit deinem Konto verschlüsselt und folgt deinen Geräten.
counted-groups-empty = Noch keine Gruppe auf diesem Gerät.
counted-groups-done = OK
counted-save-failed = Diese Wahl konnte nicht gespeichert werden. Versuche es erneut.
tab-budget = Budget
budget-title = Budgets
budget-manage = Verwalten
budget-left = Noch { $amount }
budget-over = Überschritten um { $amount }
budget-empty = Kein Budget
budget-empty-hint = Lege eine Obergrenze pro Kategorie fest. Deine Anteile in Gruppen zählen mit.
budget-define = Budget festlegen
budget-sheet-title = Monatsbudgets
budget-sheet-intro = Auf deinen Anteil: eigene und Gruppen. Leer lassen, um eine Kategorie nicht zu verfolgen.
budget-average = Ø letzte 3 Monate: { $amount }
budget-for = Budget { $category }
budget-alert = Bei 80 % warnen
budget-alert-hint = Auf diesem Gerät berechnet
budget-invalid = Kein gültiger Betrag für { $category }.
budget-offline = Budgets lassen sich nur online speichern. Verbinde dich und versuche es erneut.
budget-alert-near = { $category }: { $percent } % des Monatsbudgets
budget-alert-over = { $category }: Monatsbudget überschritten
my-spending-search-summary =
    { $count ->
        [one] { $count } Ergebnis
       *[other] { $count } Ergebnisse
    } · { $total } · eigene und Gruppen, alle Monate
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
expenses-unreadable = Einige Einträge können auf diesem Gerät nicht gelesen werden und fehlen in den Summen. Aktualisiere die App, um sie zu sehen.
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
expense-type-group = Typ
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
move-expense = In ein Projekt verschieben
move-expense-title = Verschieben nach…
move-current = Aktuell
move-converted = { $currency }, umgerechnet zum Monatskurs
move-note-personal = Wer bezahlt hat und für wen, wählst du im nächsten Schritt. Die Ausgabe verlässt Meine Ausgaben.
move-note-group = Die Ausgabe verlässt { $project } für alle Mitglieder, und der Saldo der Gruppe wird neu berechnet. Wer bezahlt hat und für wen, wählst du im nächsten Schritt.
move-banner = Von { $from } nach { $to }
move-submit = Verschieben
move-done = Ausgabe nach { $project } verschoben
history-expense-moved = Ausgabe in ein anderes Projekt verschoben: { $name }
personal-badge = Privat
move-members =
    { $count ->
        [one] { $count } Mitglied
       *[other] { $count } Mitglieder
    }
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
expense-share-percent = { $percent } %
expense-added-by = Hinzugefügt von { $name } am { $date }
expense-inconsistent-detail = Beträge passen nicht zusammen: { $paid } bezahlt, { $owed } geschuldet, bei einer Ausgabe von { $total }. Bearbeite die Ausgabe, um das zu korrigieren.
missing-access-key = Zugriffsschlüssel fehlt. Öffne dieses Projekt über seinen Freigabelink.
filter-all = Alle
filter-my-payments = Meine Zahlungen
filter-my-debts = Was ich schulde
expense-filter-group = Filter
expenses-search = Ausgaben suchen
expenses-no-match = Keine passende Ausgabe.
expenses-search-open = Suchen
expenses-search-clear = Suche löschen
expenses-search-summary =
    { $count ->
        [one] { $count } Ergebnis
       *[other] { $count } Ergebnisse
    } · { $total } · alle Monate
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
charts-my-share-skipped =
    { $count ->
        [one] 1 Projekt zählt nicht mit - keine Person ausgewählt, oder die Daten konnten nicht geladen werden.
       *[other] { $count } Projekte zählen nicht mit - keine Person ausgewählt, oder die Daten konnten nicht geladen werden.
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
categories-manage = Kategorien verwalten
categories-title = Kategorien
categories-yours = Deine Kategorien
categories-builtin = Standard
categories-add = Kategorie hinzufügen
categories-in-parent = in { $parent }
categories-renamed-from = umbenannt · ursprünglich { $name }
categories-hidden-hint = ausgeblendet · Ausgaben landen in { $other }
categories-always-shown = immer sichtbar
categories-hidden = Ausgeblendet
categories-new = Neue Kategorie
categories-edit = Kategorie bearbeiten
categories-parent = Gezählt in
categories-parent-hint = In der Statistik all deiner Projekte zählt { $name } zu { $parent }.
categories-create = Erstellen
categories-hide = Diese Kategorie ausblenden
categories-hide-hint = Ihre Ausgaben erscheinen für alle Mitglieder in { $other }.
categories-delete-title = „{ $name }“ löschen?
categories-delete-body = Die betroffenen Ausgaben ({ $count }) kommen zurück zu { $parent }. Keine Ausgabe wird gelöscht.
categories-delete-body-unknown = Ihre Ausgaben kommen zurück zu { $parent }. Keine Ausgabe wird gelöscht.
categories-delete-warning = Die Kategorie verschwindet für alle Mitglieder des Projekts.
categories-summary-created = Erstellt: { $count }
categories-summary-renamed = Umbenannt: { $count }
categories-summary-hidden = Ausgeblendet: { $count }
categories-summary-default = Keine eigenen Kategorien
categories-offline = Kategorien lassen sich nur online verwalten. Verbinde dich und versuche es erneut.
categories-error-too-many = Ein Projekt hat höchstens 20 eigene Kategorien.
categories-error-full = Dieses Projekt hat sein Kategorienlimit erreicht.
categories-error-name = Gib der Kategorie einen Namen.
categories-error-missing = Diese Kategorie wurde inzwischen geändert. Schließe und versuche es erneut.
history-category-created = Kategorie erstellt: { $name }
history-category-edited = Kategorie bearbeitet: { $name }
history-category-deleted = Kategorie gelöscht: { $name }
history-empty = Keine Ereignisse
not-found-hint = Diese Seite gibt es nicht, oder sie ist umgezogen.
payers-title-paid-by = Bezahlt von
payers-title-sender = Absender
payers-title-contributors = Beitragende
debtors-title-debtors = Schuldet
debtors-title-recipients = Empfänger
debtors-title-beneficiaries = Begünstigte
debtors-title-for = Für
debtors-title-recipient = Empfänger
expense-split-between = aufgeteilt auf { $count }
expense-people =
    { $count ->
        [one] { $count } Person
       *[other] { $count } Personen
    }

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
help-forgot-password-warning = Niemand kann es für dich wiederherstellen.
help-forgot-password-a = Dein Passwort schützt deine Schlüssel und wird nie an uns gesendet: niemand - wir eingeschlossen - kann deine Projekte an deiner Stelle entschlüsseln. Bewahre es gut auf, am besten in einem Passwortmanager.
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
update-required-body = Diese Version von Counted ist zu alt, um mit dem Server zu kommunizieren. Aktualisiere sie, um die App weiter zu nutzen.
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

payment-method-copy = { $name } kopieren
payment-method-copied = Kopiert.
payment-method-copy-failed = Kopieren nicht möglich - markiere den Text und kopiere ihn von Hand.

### Expenses (Währung, Betrag)

expense-category-auto = Auto · { $emoji }
emoji-picker-label = Emoji
emoji-none = Kein Emoji
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
project-currency-locked = Die Währung wird beim Anlegen des Projekts festgelegt.
currency-search = Währung suchen
project-gone-title = Dieses Projekt existiert nicht mehr
participants-by-shares = Nach Anteilen
split-amounts = Beträge

### Reimbursements

reimbursements-empty-title = Alles beglichen!
reimbursement-record = Begleichen
settlement-recorded = Ausgleich gespeichert.
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
charts-paid-more = { $amount } mehr als mein Anteil
charts-paid-less = { $amount } weniger als mein Anteil
charts-paid-even = Genau mein Anteil
charts-part-title = Dein Anteil je Kategorie
charts-part-desc = Grau ist, was die Gruppe ausgegeben hat, farbig, was du verbraucht hast.
charts-breakdown-title = Aufteilung nach Kategorie
charts-breakdown-desc = Tippe auf ein Segment oder eine Zeile, um die Ausgaben zu sehen.
charts-of-total = { $amount } von { $total }
charts-show-all = Alle anzeigen ({ $count })
charts-show-less = Weniger anzeigen
charts-spend-title = Ausgaben im Zeitverlauf
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
charts-projects-desc = Summen bleiben je Währung getrennt.
history-by = Von { $name }

### Notifications

notifications-label = Benachrichtigungen
notifications-title = Benachrichtigungen
notifications-empty = Nichts Neues

### Friends

friends-title = Freunde
friends-anonymous-body = Freunde sind an dein Konto gebunden. Melde dich an, um Personen hinzuzufügen und sie ohne Link in deine Projekte einzuladen.
friends-add-title = Freund hinzufügen
friends-add-from-project = Als Freund hinzufügen
friends-request-sent = Anfrage gesendet
friends-no-account-key = Melde dich erneut an, um deine Freunde auf diesem Gerät zu verwalten.
friends-incoming-title = Anfragen
friends-accept = Annehmen
friends-decline = Ablehnen
friends-list-title = Meine Freunde
friends-remove = Entfernen
friends-remove-confirm-title = Freund entfernen
friends-remove-confirm-message = { $email } ist dann nicht mehr in deinen Freunden, und du nicht mehr in seinen. Jeder von euch kann später eine neue Anfrage senden.
friends-no-key = Noch nicht bereit
friends-key-mismatch = Der Server hat für dein Konto einen öffentlichen Schlüssel geschickt, der nicht zu deinem privaten Schlüssel passt. Dein Sicherheitscode ist ausgeblendet und Einladungen sind auf diesem Gerät aus: Vielleicht manipuliert jemand den Server.
keypair-mismatch = Die Schlüssel, die der Server für dein Konto geschickt hat, passen nicht zu denen auf diesem Gerät. Einladungen und Schlüsselwechsel sind hier aus, bis sie passen: Vielleicht manipuliert jemand den Server.
friends-outgoing-title = Gesendet
friends-notify = Per Nachricht Bescheid sagen
friends-notify-message = Ich nutze jetzt Counted für unsere gemeinsamen Ausgaben. Kostenlose Registrierung auf https://counted.fr mit der Adresse { $email }: Meine Freundschaftsanfrage wartet dort.
friends-notify-copied = Nachricht kopiert
friends-notify-copy-failed = Die Nachricht konnte nicht kopiert werden
guests-hint-title =
    { $count ->
        [one] { $count } Teilnehmer ist dem Projekt noch nicht beigetreten
       *[other] { $count } Teilnehmer sind dem Projekt noch nicht beigetreten
    }
guests-hint-later = Später
guests-hint-send = Lade sie ein
guests-send-title = Projekt an die Teilnehmer senden
guests-send-friend = Counted-Freund einladen
guests-send-friend-badge = Ohne Link
guests-send-friend-hint = Hat die Person schon ein Konto, bekommt sie eine verschlüsselte Einladung; nichts läuft über deinen Messenger.
guests-send-link = Projektlink teilen
guests-send-link-hint = Für alle ohne Konto: ein Tippen öffnet das Projekt.
guests-send-link-warning = Wer diesen Link hat, kann das Projekt lesen und bearbeiten. Dein Messenger sieht ihn auch.
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
participants-others = Weitere Teilnehmende
participants-removed = Entfernt
participant-restore = Wiederherstellen
participants-empty = Füge unten Teilnehmer hinzu.
participants-duplicate = „{ $name }“ ist bereits in der Liste.
participants-input-label = Freund hinzufügen oder Namen eingeben
participants-input-placeholder = Freund oder beliebiger Name
participants-suggest-friend = Freund · tritt als „{ $name }“ bei, erhält eine Einladung
participants-suggest-not-ready = Freund · noch nicht bereit
participants-suggest-guest = „{ $text }“ hinzufügen
participants-suggest-guest-sub = Gast
participants-friends = Deine Freunde
participants-all-friends = Alle Freunde
participants-login-hint = Melde dich an, um Leute direkt aus deiner Freundesliste hinzuzufügen.
participants-invite-badge = Einladen
participants-guest-badge = Gast
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
edit-project-you-are = In diesem Projekt bist du { $name }
edit-project-no-identity = Du hast noch nicht gewählt, wer du bist
edit-project-switch = Wechseln
edit-project-choose = Wählen
invite-failed = Diese Einladungen konnten nicht gesendet werden: { $emails }
invite-waiting = Diese Freunde müssen Counted einmal öffnen, bevor sie eingeladen werden können: { $emails }
key-offer-title = Ein anderer Schlüssel für { $name }
key-offer-body = Dieses Gerät hat einen anderen Schlüssel für dieses Projekt, der es hier noch nie geöffnet hat.
key-offer-choices = Diesen Schlüssel verwenden: Das Projekt öffnet sich mit dem Schlüssel aus diesem Link oder dieser Einladung. Abbrechen: Das Gerät behält seinen aktuellen Schlüssel, nichts ändert sich.
key-offer-use = Diesen Schlüssel verwenden
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
expense-pending = Ausstehend
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

home-owed-to-you = Dir geschuldet
home-you-owe = Du schuldest
home-details-show = Details anzeigen
home-details-hide = Details ausblenden
home-all-settled = Alles ausgeglichen
home-pay = Bezahlen
home-remind = Erinnern
home-remind-message = Kleine Erinnerung: In { $project } sind noch Rückzahlungen offen.
home-row-owed = dir geschuldet
home-row-owe = du schuldest
home-row-settled = Ausgeglichen
home-row-quits = Du bist quitt
home-row-pending =
    { $count ->
        [one] { $count } Rückzahlung offen
       *[other] { $count } Rückzahlungen offen
    }
home-totals-partial = Ohne Projekte ohne Wechselkurs
error-account-key-changed = Der Schlüssel deines Kontos wurde auf einem anderen Gerät geändert. Melde dich erneut an, um weiterzumachen.
settings-security = Sicherheit
settings-change-password-hint = Deine Projekte und Daten bleiben, wie sie sind. Deine anderen Geräte werden abgemeldet.
change-password-title = Passwort ändern
change-password-current = Aktuelles Passwort
change-password-new = Neues Passwort
change-password-confirm = Neues Passwort bestätigen
change-password-mismatch = Die beiden neuen Passwörter stimmen nicht überein.
change-password-submit = Passwort ändern
change-password-submitting = Wird geändert…
change-password-done = Passwort geändert. Deine anderen Geräte wurden abgemeldet.
error-rotation-incomplete = Dein Konto hat sich währenddessen geändert. Versuche es erneut.
error-key-lifecycle-unavailable = Noch nicht verfügbar: Zuerst brauchen alle die neueste Version der App.
settings-compromised-open = Konto absichern
compromised-title = Konto absichern
compromised-body = Dein Passwort und die Schlüssel deines Kontos werden ersetzt, und alle anderen Geräte werden abgemeldet. Deine Freunde sehen, dass sich dein Sicherheitscode ändert: Vergleicht ihn erneut.
compromised-keeps = Wer dein Passwort hatte, behält alles, was schon geöffnet wurde: die Projekte, die bis jetzt lesbar waren.
compromised-submit = Passwort und Schlüssel ersetzen
compromised-done = Dein Konto ist abgesichert. Bitte deine Freunde, deinen neuen Sicherheitscode zu vergleichen.
friends-code-matches = Der Code stimmt
friends-code-trusted = Code bestätigt.
settings-recovery-key = Wiederherstellungsschlüssel
settings-recovery-key-pending = Ein neuer Schlüssel ersetzt ihn am { $when }. Warst du das nicht? Brich ihn mit deinem aktuellen Schlüssel ab.
recovery-key-cancel-replacement = Neuen Schlüssel abbrechen
recovery-key-title = Dein Wiederherstellungsschlüssel
recovery-key-body = Schreib ihn auf oder speichere ihn dort, wo nur du hinkommst. Counted kann ihn weder erneut zeigen noch für dich wiederfinden.
recovery-key-copy = Kopieren
recovery-key-copied = Kopiert
recovery-key-save = Als Datei speichern
recovery-key-print = Drucken
recovery-key-next = Weiter
recovery-key-confirm-body = Gib zur Kontrolle Gruppe { $n } des Schlüssels ein.
recovery-key-confirm-wrong = Diese Gruppe stimmt nicht. Schau noch einmal auf deine Kopie.
recovery-key-password-body = Gib dein Passwort ein, um den Schlüssel in deinem Konto zu speichern.
recovery-key-submit = Schlüssel speichern
recovery-key-done = Wiederherstellungsschlüssel gespeichert.
recovery-key-replacement-done = Neuer Schlüssel gespeichert. Er ersetzt den aktuellen in 72 Stunden; bis dahin funktioniert der aktuelle weiter.
recovery-key-field = Wiederherstellungsschlüssel
recovery-key-file-intro = Counted-Wiederherstellungsschlüssel
recovery-key-cancel-done = Der neue Schlüssel ist abgebrochen.
compromised-new-key = Speichere jetzt deinen neuen Wiederherstellungsschlüssel: Er ersetzt den alten in 72 Stunden, bis dahin funktioniert der alte noch.
pending-reset-title = Zurücksetzen angefordert
pending-reset-body = Es wird am { $when } wirksam. Warst du das nicht? Brich es mit deinem Passwort ab.
pending-reset-cancel = Zurücksetzen abbrechen
pending-reset-cancelled = Das Zurücksetzen ist abgebrochen.
login-forgot = Passwort vergessen?
forgot-title = Passwort vergessen
forgot-body = Gib deine E-Mail-Adresse ein. Wenn es dazu ein Konto gibt, schicken wir einen Link für ein neues Passwort.
forgot-submit = Link senden
forgot-sent = Wenn es zu dieser Adresse ein Konto gibt, ist ein Link unterwegs. Er läuft in 24 Stunden ab.
reset-title = Konto wiederherstellen
reset-with-key-body = Gib deinen Wiederherstellungsschlüssel und ein neues Passwort ein: Alles kommt so zurück, wie es war.
reset-submit-key = Konto wiederherstellen
reset-no-key = Ich habe meinen Wiederherstellungsschlüssel nicht
reset-without-key-title = Ohne Wiederherstellungsschlüssel zurücksetzen
reset-without-key-body = Dein Konto startet in 72 Stunden neu, mit einem neuen Passwort. Deine Projekte kommen von deinen Geräten und über Freigabelinks zurück, deine Einstellungen, Zahlungsdaten und Freundescodes nicht. Bis dahin funktioniert dein aktuelles Passwort weiter und kann den Neustart abbrechen.
reset-name = Dein Name
reset-submit-request = Konto zurücksetzen
reset-requested = Erledigt. Dein neues Passwort funktioniert ab { $when }. Bis dahin funktioniert dein aktuelles Passwort und kann es abbrechen.
reset-pending-already = Ein Zurücksetzen wartet bereits bis { $when }. Eine neue Anfrage startet die 72 Stunden neu.
recovered-done = Dein Konto ist zurück. Deine anderen Geräte wurden abgemeldet.
login-reset-undoable = Dieses Passwort wurde in den letzten 30 Tagen durch ein Zurücksetzen ersetzt. Wenn du das nicht wolltest, mach es rückgängig: Dein Konto kommt zurück, wie es war.
login-undo-reset = Zurücksetzen rückgängig machen
reset-undone = Das Zurücksetzen ist rückgängig gemacht. Dein Konto ist wieder wie vorher.
error-reset-pending = Dieses neue Passwort funktioniert, sobald das Zurücksetzen wirksam wird, 72 Stunden nach der Anfrage.
error-account-disabled = Dieses Konto wurde kurz nach einem Zurücksetzen gelöscht und wartet auf die Löschung. Das vorherige Passwort kann das Zurücksetzen noch rückgängig machen.
error-recovery-key-invalid = Dieser Wiederherstellungsschlüssel öffnet dieses Konto nicht.
error-project-key-required = Dieses Projekt braucht seinen Schlüssel. Öffne es erneut über seinen Freigabelink.
names-or = { $first } oder { $last }
participants-admin-badge = Admin
edit-project-admins = Admins: { $count }
edit-project-succession = Verlässt der letzte Admin das Projekt, wird das Mitglied mit Konto Admin, das am längsten dabei ist.
member-sheet-account = Mitglied mit Konto
member-actions = Aktionen für { $name }
member-make-admin = Zum Admin machen
member-make-admin-hint = Kann dann den Projektschlüssel ändern und jemanden entfernen.
member-made-admin = { $name } ist jetzt Admin.
group-security = Projektsicherheit
group-security-admin = Projektsicherheit · du bist Admin
group-rotate = Projektschlüssel ändern
group-rotate-hint = Falls der Projektlink in falsche Hände geraten ist: Wer den alten Link hat, verliert den Zugang zum Projekt.
group-remove = Jemanden aus dem Projekt entfernen
group-remove-hint = Entzieht den Zugang und ändert gleichzeitig den Schlüssel.
group-admins-only = Nur für Admins.
group-admins-only-account = Nur für Admins, und die haben ein Konto.
group-ask-admin = Frag { $names }, Admin des Projekts.
group-ask-admins = Frag { $names }, die Admins des Projekts.
group-ask-an-admin = Frag einen Admin des Projekts.
group-with-account = Mit Konto
group-free = Kostenlos
group-sign-in-hint = Ein Admin des Projekts, { $names }, kann dir dann die Rolle geben.
group-sign-in-hint-any = Ein Admin des Projekts kann dir dann die Rolle geben.
group-no-admin = Dieses Projekt hat keinen Admin, daher kann sein Schlüssel nicht geändert werden.
group-hide = Jemanden ausblenden
group-hide-hint = Blendet die Person in den Listen aus, ohne ihr den Zugang zu nehmen: Link und Schlüssel bleiben gültig.
group-recreate-hint = Dieses Projekt hat keinen Admin. Um jemanden zu entfernen, erstelle ein neues Projekt.
group-recreate = Neues Projekt erstellen
group-no-admin-yet = Dieses Projekt hat noch keinen Admin.
group-creator-hint = Nur du kannst Admin dieses Projekts werden, indem du ein Konto erstellst.
group-creator-elsewhere-hint = Nur die Person, die das Projekt erstellt hat, kann sein Admin werden, indem sie ein Konto erstellt. Sonst erstelle ein neues Projekt, um jemanden zu entfernen.
member-revoke-admin = Admin-Rolle entziehen
member-revoke-admin-hint = Kann den Projektschlüssel dann nicht mehr ändern und niemanden mehr entfernen.
member-revoked-admin = { $name } ist kein Admin mehr.
member-step-down = Kein Admin mehr sein
member-step-down-hint = Du kannst den Projektschlüssel dann nicht mehr ändern und niemanden mehr entfernen.
member-stepped-down = Du bist kein Admin mehr.
member-remove = Aus dem Projekt entfernen
member-remove-hint = Ändert den Projektschlüssel: { $name } kann nichts Neues mehr lesen.
member-hide = In den Listen ausblenden
error-user-not-admin = Dieses Mitglied ist kein Admin.
error-creator-proof-invalid = Dieses Gerät hat das Projekt nicht erstellt.
error-project-has-admin = Dieses Projekt hat schon einen Admin.
error-not-project-admin = Nur die Admins des Projekts können das.
error-user-not-claimed = Diese Person hat kein Konto und kann daher kein Admin sein.
error-project-key-rotated = Der Projektschlüssel hat sich geändert. Frag ein Mitglied nach dem neuen Link.
error-project-rotating = Der Projektschlüssel wird gerade geändert. Versuch es gleich noch einmal.
error-rotation-expired = Diese Schlüsseländerung hat zu lange gedauert und wurde abgebrochen. Fang neu an.
error-rotation-rows-changed = Das Projekt hat sich während der Schlüsseländerung geändert. Fang neu an.
project-rotated-title = Du hast keinen Zugang mehr zu diesem Projekt
project-rotated-body = Der Admin hat den Schlüssel dieses Projekts geändert. Frag nach dem neuen Link.
project-rotated-open-link = Link öffnen
rotation-remove-title = { $name } aus dem Projekt entfernen?
rotation-rotate-title = Projektschlüssel ändern?
rotation-remove-body = Ein Mitglied zu entfernen erzeugt einen neuen Schlüssel. Den neuen Link musst du mit den Personen ohne Konto teilen.
rotation-rotate-body = Wer den alten Link hat, verliert den Zugang zum Projekt. Den neuen Link musst du mit den Personen ohne Konto teilen.
rotation-boxed =
    { $count ->
        [0] Kein anderes Mitglied erhält den Schlüssel automatisch
        [one] { $count } Mitglied erhält den Schlüssel automatisch
       *[other] { $count } Mitglieder erhalten den Schlüssel automatisch
    }
rotation-by-link-title = Personen, denen du den neuen Link schicken musst
rotation-by-link-hint = Diese Personen haben kein Konto. Du schickst ihnen den Link gleich danach.
rotation-names-and = { $first } und { $last }
rotation-names-and-others =
    { $names } und { $count ->
        [one] { $count } weitere Person
       *[other] { $count } weitere Personen
    }
rotation-confirm-remove = Entfernen und Schlüssel ändern
rotation-confirm-rotate = Schlüssel ändern
rotation-done-title = Neuer Schlüssel aktiv
rotation-done-removed = { $name } gehört nicht mehr zum Projekt.
rotation-done-rotated = Der Projektschlüssel wurde geändert.
rotation-done-boxed =
    { $count ->
        [0] Kein anderes Mitglied hat den Schlüssel automatisch erhalten.
        [one] { $count } Mitglied hat den Schlüssel automatisch erhalten.
       *[other] { $count } Mitglieder haben den Schlüssel automatisch erhalten.
    }
rotation-new-link =
    { $count ->
        [0] Neuer Projektlink
        [one] Neuer Link zum Senden · { $count } Person ohne Konto
       *[other] Neuer Link zum Senden · { $count } Personen ohne Konto
    }
rotation-link-renewed = Der Link wurde erfolgreich erneuert.
rotation-done = Fertig
history-key-changed = Projektschlüssel geändert
history-member-removed = { $name } gehört nicht mehr zum Projekt
project-rotated-row-hint = Der Schlüssel wurde geändert. Frag nach dem neuen Link.

## Settings and Friends redesign (docs/plans/friends-settings-redesign.md)
settings-group-sharing = Teilen
settings-group-help = Hilfe und Infos
settings-friends-changed = { $count ->
    [one] 1 Code geändert
   *[other] { $count } Codes geändert
}
settings-recovery-key-created = Erstellt
settings-recovery-key-missing = Zu erstellen
settings-recovery-key-replacing = Ersetzung ausstehend
settings-password = Passwort
settings-compromised-short = Kennt jemand dein Passwort?
settings-recovery-key-footnote = Ohne Wiederherstellungsschlüssel löscht ein vergessenes Passwort deine Einstellungen, Zahlungsdaten und Freundescodes.
erase-device-row = Dieses Gerät löschen
erase-device-row-hint = Meldet dich ab und behält nur die Sprache.
account-delete-hint = Sofort und endgültig.
settings-open-source = Open Source, AGPL-3.0
settings-upsell-short = Ohne Konto bleiben deine Projekte auf diesem Gerät. Mit Konto folgen sie dir überallhin, verschlüsselt.
settings-locked-group = Mit Konto
settings-locked-friends-short = Ohne Link einladen
settings-locked-payment-short = Sichtbar für alle, die dir Geld schulden
payment-methods-title = Zahlungsmethoden
payment-methods-row-hint = Mit deinen Freunden geteilt
payment-methods-intro = Mit deinen Freunden geteilt. Mit deinem Konto verschlüsselt.
payment-methods-footnote = Eine Zahlungsmethode wird (max. 20) mit den Mitgliedern deiner Projekte im Tab Ausgleichen geteilt.
payment-method-shared-pill = Geteilt
payment-method-share-groups = In deinen Gruppen teilen
payment-method-label-optional = Name (optional)
payment-method-delete-confirm-title = Diese Zahlungsmethode löschen?
payment-method-delete-confirm-message = { $name } wird in deinen Projekten nicht mehr angezeigt.
payment-methods-empty-title = Keine Zahlungsmethode
payment-methods-empty-body = Füge deine IBAN oder Bezahl-App hinzu: Wer dir Geld schuldet, sieht sie.
payment-method-masked = { $kind } endet auf { $last }
friends-to-check = Zu prüfen
friends-code-changed-row = Sein Code hat sich geändert
friends-own-code = Dein Sicherheitscode
friends-own-code-hint = Deine Freunde sehen diesen Code neben deiner Adresse. Stimmt er mit ihrem überein, laden sie dich ein und niemand anderen.
friends-received = Erhalten
friends-received-on = Erhalten am { $date }
friends-pending = Ausstehend
friends-from-project = Aus einem Projekt
friends-accept-label = Anfrage von { $email } annehmen
friends-decline-label = Anfrage von { $email } ablehnen
friends-add-hint-short = Hat die Person ein Konto, sieht sie deine Anfrage. Sonst sag ihr Bescheid.
friends-send = Senden
friends-sent-hint = Noch kein Konto? Sag Bescheid: Die Nachricht lädt die Person ein, sich mit dieser Adresse zu registrieren.
friends-done = Fertig
friends-their-code = Sein Sicherheitscode
friends-code-compare-hint = Lest ihn zusammen. Oben auf seiner Freunde-Seite muss derselbe Code stehen.
friends-no-key-hint = Noch nicht bereit: Sein Schlüssel wird beim nächsten Öffnen von Counted erstellt. Bis dahin kannst du ihn nicht einladen.
friends-remove-row = Aus meinen Freunden entfernen
friends-new-code = Neuer Code
friends-key-changed-hint-short = Prüft diesen Code zusammen, außerhalb dieser App. Bis dahin sind Einladungen mit ihm gesperrt.
friends-kem-changed-title = Sein Schlüssel wurde ersetzt
friends-kem-changed-body = Der Post-Quanten-Teil seines Schlüssels hat sich geändert, aber nicht sein Code. Ein Konto abzusichern ändert beides, also hat vielleicht jemand diesen Schlüssel ausgetauscht. Einladungen mit ihm bleiben gesperrt.
friends-kem-changed-hint = Du kannst ihn entfernen und dann eine neue Anfrage senden: Sein Schlüssel wird neu fixiert.
friends-pending-sheet = Anfrage gesendet, ausstehend.
friends-withdraw-row = Anfrage zurückziehen
friends-empty-title = Noch keine Freunde
friends-empty-body = Füge jemanden per E-Mail hinzu und lade ihn dann ohne Link in ein Projekt ein.
friends-requests-empty-title = Keine Anfragen
friends-requests-empty-body = Empfangene und gesendete Anfragen erscheinen hier.
friends-code-hidden = Dein Code ist ausgeblendet.
friends-code-hidden-body = Ein für dein Konto gelieferter Schlüssel passt nicht zu diesem Gerät. Einladungen sind hier aus.
notifications-manage-friends = Meine Freunde verwalten
notifications-friend-request-on = Freundschaftsanfrage · { $date }

### Link preview

share-preview-title = Unsere Ausgaben auf Counted
share-preview-description = Öffne den Link, um die Ausgaben zu sehen und wer wem was schuldet. Ohne Konto, verschlüsselt auf deinem Gerät.

### Currency names

currency-EUR = Euro
currency-AED = VAE-Dirham
currency-AFN = Afghanischer Afghani
currency-ALL = Albanischer Lek
currency-AMD = Armenischer Dram
currency-AOA = Angolanischer Kwanza
currency-ARS = Argentinischer Peso
currency-AUD = Australischer Dollar
currency-AWG = Aruba-Florin
currency-AZN = Aserbaidschan-Manat
currency-BAM = Konvertible Mark Bosnien und Herzegowina
currency-BBD = Barbados-Dollar
currency-BDT = Bangladesch-Taka
currency-BHD = Bahrain-Dinar
currency-BIF = Burundi-Franc
currency-BMD = Bermuda-Dollar
currency-BND = Brunei-Dollar
currency-BOB = Bolivianischer Boliviano
currency-BRL = Brasilianischer Real
currency-BSD = Bahamas-Dollar
currency-BTN = Bhutan-Ngultrum
currency-BWP = Botswanischer Pula
currency-BYN = Weißrussischer Rubel
currency-BZD = Belize-Dollar
currency-CAD = Kanadischer Dollar
currency-CDF = Kongo-Franc
currency-CHF = Schweizer Franken
currency-CLP = Chilenischer Peso
currency-CNY = Renminbi Yuan
currency-COP = Kolumbianischer Peso
currency-CRC = Costa-Rica-Colón
currency-CUP = Kubanischer Peso
currency-CVE = Cabo-Verde-Escudo
currency-CZK = Tschechische Krone
currency-DJF = Dschibuti-Franc
currency-DKK = Dänische Krone
currency-DOP = Dominikanischer Peso
currency-DZD = Algerischer Dinar
currency-EGP = Ägyptisches Pfund
currency-ERN = Eritreischer Nakfa
currency-ETB = Äthiopischer Birr
currency-FJD = Fidschi-Dollar
currency-FKP = Falkland-Pfund
currency-GBP = Britisches Pfund
currency-GEL = Georgischer Lari
currency-GHS = Ghanaischer Cedi
currency-GIP = Gibraltar-Pfund
currency-GMD = Gambia-Dalasi
currency-GNF = Guinea-Franc
currency-GTQ = Guatemaltekischer Quetzal
currency-GYD = Guyana-Dollar
currency-HKD = Hongkong-Dollar
currency-HNL = Honduras-Lempira
currency-HTG = Haitianische Gourde
currency-HUF = Ungarischer Forint
currency-IDR = Indonesische Rupiah
currency-ILS = Israelischer Neuer Schekel
currency-INR = Indische Rupie
currency-IQD = Irakischer Dinar
currency-IRR = Iranischer Rial
currency-ISK = Isländische Krone
currency-JMD = Jamaika-Dollar
currency-JOD = Jordanischer Dinar
currency-JPY = Japanischer Yen
currency-KES = Kenia-Schilling
currency-KGS = Kirgisischer Som
currency-KHR = Kambodschanischer Riel
currency-KMF = Komoren-Franc
currency-KPW = Nordkoreanischer Won
currency-KRW = Südkoreanischer Won
currency-KWD = Kuwait-Dinar
currency-KYD = Kaiman-Dollar
currency-KZT = Kasachischer Tenge
currency-LAK = Laotischer Kip
currency-LBP = Libanesisches Pfund
currency-LKR = Sri-Lanka-Rupie
currency-LRD = Liberianischer Dollar
currency-LSL = Loti
currency-LYD = Libyscher Dinar
currency-MAD = Marokkanischer Dirham
currency-MDL = Moldau-Leu
currency-MGA = Madagaskar-Ariary
currency-MKD = Mazedonischer Denar
currency-MMK = Myanmarischer Kyat
currency-MNT = Mongolischer Tögrög
currency-MOP = Macao-Pataca
currency-MRU = Mauretanischer Ouguiya
currency-MUR = Mauritius-Rupie
currency-MVR = Malediven-Rufiyaa
currency-MWK = Malawi-Kwacha
currency-MXN = Mexikanischer Peso
currency-MYR = Malaysischer Ringgit
currency-MZN = Mosambikanischer Metical
currency-NAD = Namibia-Dollar
currency-NGN = Nigerianischer Naira
currency-NIO = Nicaragua-Córdoba
currency-NOK = Norwegische Krone
currency-NPR = Nepalesische Rupie
currency-NZD = Neuseeland-Dollar
currency-OMR = Omanischer Rial
currency-PAB = Panamaischer Balboa
currency-PEN = Peruanischer Sol
currency-PGK = Papua-neuguineischer Kina
currency-PHP = Philippinischer Peso
currency-PKR = Pakistanische Rupie
currency-PLN = Polnischer Złoty
currency-PYG = Paraguayischer Guaraní
currency-QAR = Katar-Riyal
currency-RON = Rumänischer Leu
currency-RSD = Serbischer Dinar
currency-RUB = Russischer Rubel
currency-RWF = Ruanda-Franc
currency-SAR = Saudi-Rial
currency-SBD = Salomonen-Dollar
currency-SCR = Seychellen-Rupie
currency-SDG = Sudanesisches Pfund
currency-SEK = Schwedische Krone
currency-SGD = Singapur-Dollar
currency-SHP = St.-Helena-Pfund
currency-SLE = Sierra-leonischer Leone
currency-SOS = Somalia-Schilling
currency-SRD = Suriname-Dollar
currency-SSP = Südsudanesisches Pfund
currency-STN = São-toméischer Dobra
currency-SVC = El Salvador Colon
currency-SYP = Syrisches Pfund
currency-SZL = Swasiländischer Lilangeni
currency-THB = Thailändischer Baht
currency-TJS = Tadschikistan-Somoni
currency-TMT = Turkmenistan-Manat
currency-TND = Tunesischer Dinar
currency-TOP = Tongaischer Paʻanga
currency-TRY = Türkische Lira
currency-TTD = Trinidad-und-Tobago-Dollar
currency-TWD = Neuer Taiwan-Dollar
currency-TZS = Tansania-Schilling
currency-UAH = Ukrainische Hrywnja
currency-UGX = Uganda-Schilling
currency-USD = US-Dollar
currency-UYU = Uruguayischer Peso
currency-UZS = Usbekistan-Sum
currency-VES = Venezolanischer Bolívar
currency-VND = Vietnamesischer Dong
currency-VUV = Vanuatu-Vatu
currency-WST = Samoanischer Tala
currency-XAF = CFA-Franc (BEAC)
currency-XCD = Ostkaribischer Dollar
currency-XCG = Caribbean Guilder
currency-XOF = CFA-Franc (BCEAO)
currency-XPF = CFP-Franc
currency-YER = Jemen-Rial
currency-ZAR = Südafrikanischer Rand
currency-ZMW = Kwacha
currency-ZWG = Zimbabwe Gold
