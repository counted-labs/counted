# Nederlands. Volledig behalve de juridische teksten (legal-, terms-, privacy-), die alleen in het
# Engels en Frans bestaan en per bericht op en.ftl terugvallen.

### Common

loading = Laden…
cancel = Annuleren
discard-title = Wijzigingen weggooien?
discard-message = Wat je hebt ingevuld, gaat verloren.
discard = Weggooien
retry = Opnieuw proberen
delete = Verwijderen
back = Terug
language = Taal
theme = Thema
theme-light = Licht
theme-dark = Donker
theme-system = Systeem
theme-hint-device = Alleen op dit apparaat opgeslagen.
theme-hint-system = Volgt de weergave van dit apparaat.

### Navigation

nav-main = Hoofdnavigatie
nav-projects = Projecten
nav-charts = Statistieken
nav-settings = Instellingen

### Connectivity

offline-banner = Offline
offline-pending =
    { $count ->
        [one] { $count } in wachtrij
       *[other] { $count } in wachtrij
    }

sync-conflict-edit = Conflict: bewerken van ‘{ $name }’ mislukt (item verwijderd). Overgeslagen.
sync-conflict-delete = Conflict: verwijderen van ‘{ $name }’ mislukt (item verwijderd). Overgeslagen.
sync-conflict-other = Conflict: bewerking op ‘{ $name }’ mislukt (item verwijderd). Overgeslagen.
sync-error = Synchronisatiefout: { $reason }

### Errors

error-network = Kan de server niet bereiken. Controleer je internetverbinding.
error-rate-limited = De server heeft het druk. Probeer het zo opnieuw.
error-generic = Er is iets misgegaan. Probeer het opnieuw.

error-invalid-email = Dit e-mailadres is niet geldig.
error-invalid-password = Dit wachtwoord is niet geldig.
error-password-too-short = Het wachtwoord moet minstens 12 tekens bevatten.
error-client-outdated = Deze versie van de app is verouderd. Werk hem bij om in te loggen.
error-invalid-link = Deze link is niet geldig.
error-batch-too-large = Te veel items tegelijk.
error-payers-required = Selecteer minstens één betaler.
error-debtors-required = Selecteer minstens één persoon die iets verschuldigd is.
error-duplicate-participant = Een deelnemer staat twee keer aan dezelfde kant.
error-participant-not-in-project = Deze deelnemer hoort niet bij dit project.
error-too-many-participants = Te veel deelnemers voor één uitgave.
error-invalid-credentials = E-mailadres of wachtwoord klopt niet.
error-unauthenticated = Meld je aan om dat te doen.
error-email-not-verified = Je e-mailadres is nog niet geverifieerd.
error-claim-proof-invalid = Dit apparaat heeft de projectsleutel niet en kan dus geen deelnemer claimen. Open de deellink opnieuw.
error-project-not-found = Dit project bestaat niet meer.
error-expense-not-found = Deze uitgave bestaat niet meer.
error-storage-full = De opslag is vol: de sleutel van dit project kon niet op dit apparaat worden opgeslagen. Bewaar de deellink.
error-user-not-found = Deze deelnemer bestaat niet meer.
error-tricount-not-found = Tricount niet gevonden, of de API gaf een fout.
error-too-many-members = Dit project heeft de limiet aan deelnemers bereikt.
error-user-has-payments = Deze deelnemer heeft uitgaven in het project en kan niet worden verwijderd.
error-resend-cooldown = Wacht 60 seconden voordat je een nieuwe e-mail aanvraagt.

### Auth

field-email = E-mailadres
field-email-placeholder = jij@voorbeeld.nl
field-password = Wachtwoord
field-name = Naam
field-name-placeholder = Jan Jansen

login-title = Inloggen
login-submit = Inloggen
login-submitting = Bezig met inloggen…
login-password-placeholder = Je wachtwoord
login-no-account = Nog geen account?
login-unverified = Je e-mailadres is nog niet geverifieerd. Kijk in je inbox of laat de link opnieuw sturen.
login-maybe-unverified = Net geregistreerd? Bevestig eerst je adres met de link die we je mailden, of stuur hem opnieuw.
login-resend = Verificatielink opnieuw sturen
login-resending = Bezig met verzenden…
login-resend-sent = E-mail verzonden - kijk in je inbox.

register-submit = Account aanmaken
register-submitting = Bezig met aanmaken…
register-have-account = Heb je al een account?
register-password-placeholder = Minstens 12 tekens
register-password-warning = Schrijf je wachtwoord op: wij zien het nooit en kunnen het je niet teruggeven.
register-check-email-title = Kijk in je inbox
register-email-sent = E-mail verzonden
register-email-sent-hint = Klik op de link in je inbox om je account te activeren.
register-not-received-prefix = Niets ontvangen? Kijk in je spam, of
register-sign-in-link = log in
register-not-received-suffix = om de link opnieuw te sturen.
register-terms-prefix = Door een account aan te maken ga je akkoord met onze
register-terms-link = gebruiksvoorwaarden
register-terms-and = en ons
register-privacy-link = privacybeleid

settings-title = Instellingen
settings-preferences = Voorkeuren
settings-preferences-local = Opgeslagen op dit apparaat.
settings-preferences-synced = Versleuteld gesynchroniseerd met je account.
settings-anonymous-title = Je bent niet ingelogd
settings-upsell-title = Je projecten, op al je apparaten
settings-upsell-free = Gratis

account-logout = Uitloggen
account-logging-out = Bezig met uitloggen…
account-delete-title = Mijn account verwijderen
account-delete-confirm-title = Account verwijderen
account-delete-confirm-message = Je account, je sessies en je projectenlijst worden definitief verwijderd. Zonder je wachtwoord worden de versleutelde gegevens van een gedeeld project onleesbaar voor je - dit kan niet ongedaan worden gemaakt.
erase-device-title = Dit apparaat wissen?
erase-device-body = De projecten, hun sleutels en de cache op dit apparaat worden gewist, daarna eindigt je sessie. Je projecten blijven in je account: log opnieuw in om ze terug te krijgen.
erase-device-checking = Controleren of je projecten in je account staan…
erase-device-pending = Wijzigingen die nog verstuurd moeten worden: { $count }. Ga weer online en probeer het dan opnieuw.
erase-device-unreachable = Kon niet controleren of je projecten in je account staan. Controleer je verbinding en probeer het dan opnieuw.
erase-device-at-risk = Deze projecten staan niet in je account en gaan verloren. Wil je ze houden, annuleer dan en kopieer eerst hun link:
erase-device-confirm = Wissen en uitloggen
erase-device-confirm-anyway = Toch wissen
erase-device-erasing = Wissen…

settings-payment-methods = Betaalgegevens
payment-method-kind = Methode
payment-method-kind-other = Anders
payment-method-label = Naam
payment-method-label-placeholder = Hoofdrekening
payment-method-value = Gegevens
payment-method-value-placeholder = IBAN, telefoonnummer, gebruikersnaam…
payment-method-add = Toevoegen
payment-method-deleted = Betaalmethode verwijderd.
payment-method-value-required = Vul de gegevens van elke betaalmethode in, of verwijder ze.
payment-method-label-required = Geef je eigen methode een naam.
payment-method-too-long = Dat is te lang - maak het korter.
payment-method-invalid-characters = Verwijder regeleinden of onzichtbare tekens.
payment-method-limit = Je kunt maximaal { $max } betaalmethodes opslaan.
payment-methods-saved = Betaalgegevens opgeslagen.
payment-methods-offline = Je moet online zijn om je betaalgegevens op te slaan.
payment-methods-stale = Je betaalgegevens zijn op een ander apparaat gewijzigd. Ze zijn opnieuw geladen - probeer het opnieuw.
payment-methods-key-missing = Log opnieuw in om je betaalgegevens te beheren.

verify-email-checking = Je e-mailadres wordt geverifieerd…
verify-email-welcome = E-mailadres geverifieerd - welkom bij Counted!
verify-email-back-to-login = Terug naar inloggen
verify-email-intro = Voer je wachtwoord in om te bevestigen dat dit adres van jou is.
verify-email-submit = Mijn adres bevestigen

### Project status

project-archive = Archiveren
project-sheet-invite = Uitnodigen
project-sheet-recurring = Terugkerend
project-sheet-edit = Project bewerken
project-sheet-close = Project afsluiten
project-sheet-archive = Project archiveren
project-sheet-reopen = Project heropenen
project-sheet-unarchive = Project uit archief halen
project-sheet-leave = Project verlaten

### Dates

date-long = { $day } { $month } { $year }

month-1 = januari
month-2 = februari
month-3 = maart
month-4 = april
month-5 = mei
month-6 = juni
month-7 = juli
month-8 = augustus
month-9 = september
month-10 = oktober
month-11 = november
month-12 = december

month-short-1 = jan
month-short-2 = feb
month-short-3 = mrt
month-short-4 = apr
month-short-5 = mei
month-short-6 = jun
month-short-7 = jul
month-short-8 = aug
month-short-9 = sep
month-short-10 = okt
month-short-11 = nov
month-short-12 = dec

### Actions

add = Toevoegen
create = Aanmaken
creating = Bezig met aanmaken…
edit = Bewerken
leave = Verlaten
close = Sluiten
paste = Plakken
join = Deelnemen
import = Importeren
importing = Bezig met importeren…
field-description = Omschrijving
field-optional = Optioneel

### Projects

projects-filter-active = Actief
projects-filter-all = Alle
projects-count-label = Projecten
projects-empty = Geen projecten
projects-offline-banner = Offlinegegevens - maak opnieuw verbinding om te vernieuwen.
demo-banner = Demoproject - alleen-lezen.
demo-start-own = Start je eigen project
demo-badge = Demo
demo-bar = Je wijzigingen verdwijnen als je dit tabblad sluit.
demo-leave = Echt beginnen
demo-preparing = Je demo wordt voorbereid…
demo-failed = De demo kon niet worden voorbereid.
demo-sign-in-blocked = Inloggen is uitgeschakeld in de demo. Verlaat de demo om in te loggen of een account aan te maken; je eigen projecten blijven onaangeroerd.
projects-no-local-data = Geen lokale gegevens
projects-no-local-data-hint = Log in om je projecten voor het eerst te laden.
projects-add = Project toevoegen
projects-create = Project aanmaken
projects-personal-expense = Eigen uitgave
personal-expense-banner = Toegevoegd aan Mijn uitgaven
personal-create-title = Maak je eigen project
personal-create-hint = Houd bij wat je alleen uitgeeft
group-create-title = Maak een gedeeld project
group-create-hint = Deel uitgaven met vrienden en familie
personal-create-action = Maken
personal-create-signup = Account maken
personal-sheet-title = Je eigen project
personal-sheet-intro = Om bij te houden wat je alleen uitgeeft. Het staat bij je groepen in Statistieken.
personal-sheet-point-alone = Eén lid: jij, met je accountnaam. Geen deellink.
personal-sheet-point-encrypted = Versleuteld op je apparaten, net als je andere projecten.
personal-sheet-point-stats = Je aandelen in groepen tellen in Statistieken op bij deze uitgaven.
personal-default-name = Mijn uitgaven
my-spending-prev-month = Vorige maand
my-spending-next-month = Volgende maand
my-spending-spent = Je hebt uitgegeven
my-spending-groups = Groepen
my-spending-income = Inkomsten
my-spending-balance = Maandsaldo
my-spending-unconverted = Aandelen in een andere valuta blijven buiten beschouwing tot er een koers bekend is.
my-spending-empty = Niets deze maand
my-spending-empty-hint = Voeg een uitgave toe met +. Je aandelen in groepen verschijnen hier automatisch.
my-spending-share-of = van { $amount }
my-spending-uncounted = niet meegeteld
payer-you = jou
expense-received-by = ontvangen door
counted-switch = Meetellen in Mijn uitgaven
counted-switch-on = Aan: je aandeel telt mee in Mijn uitgaven
counted-switch-off = Uit: je aandeel verschijnt niet in Mijn uitgaven
counted-groups = Meegetelde groepen
counted-groups-intro = Je aandeel in deze groepen telt mee in Mijn uitgaven en in de budgetten.
counted-groups-converted = { $from }, omgerekend naar { $to }
counted-groups-encrypted = Deze keuze is versleuteld met je account en volgt je apparaten.
counted-groups-empty = Nog geen groep op dit apparaat.
counted-groups-done = OK
counted-save-failed = Deze keuze kon niet worden opgeslagen. Probeer het opnieuw.
tab-budget = Budget
budget-title = Budgetten
budget-manage = Beheren
budget-left = Nog { $amount }
budget-over = Overschreden met { $amount }
budget-empty = Geen budget
budget-empty-hint = Stel per categorie een plafond in. Je aandelen in groepen tellen ook mee.
budget-define = Budget instellen
budget-sheet-title = Maandbudgetten
budget-sheet-intro = Op je aandeel: eigen en groepen. Laat leeg om een categorie niet te volgen.
budget-average = Gem. laatste 3 maanden: { $amount }
budget-for = Budget { $category }
budget-alert = Waarschuw me bij 80 %
budget-alert-hint = Berekend op dit apparaat
budget-invalid = Geen geldig bedrag voor { $category }.
budget-offline = Budgetten worden alleen online opgeslagen. Maak verbinding en probeer het opnieuw.
budget-alert-near = { $category }: { $percent } % van het maandbudget
budget-alert-over = { $category }: maandbudget overschreden
my-spending-search-summary =
    { $count ->
        [one] { $count } resultaat
       *[other] { $count } resultaten
    } · { $total } · eigen en groepen, alle maanden
projects-join = Deelnemen aan een project
projects-import-tricount = Importeren uit Tricount
project-actions = Projectacties

status-ongoing = Loopt
status-closed = Afgesloten
status-archived = Gearchiveerd

nav-help = Help
nav-privacy = Privacybeleid
nav-terms = Gebruiksvoorwaarden
nav-legal = Colofon

leave-project-title = Project verlaten?
leave-project-message = Je verliest de toegang vanaf dit apparaat. Blijft er niemand over, dan worden het project en al zijn uitgaven definitief verwijderd.

add-project-title = Nieuw project
add-project-name-label = Projectnaam
add-project-name-placeholder = Mijn reis, Huis 2024…
add-project-participants = Deelnemers
add-project-participant-name = Naam van de deelnemer
add-project-participant-placeholder = Clark Kent
add-project-offline = Offline kun je geen project aanmaken. Maak opnieuw verbinding en probeer het nog eens.
add-project-name-required = Het project heeft een naam nodig.

join-link-label = Deellink
join-link-hint = De link bevat de ontsleutelsleutel - kopieer hem volledig.
join-invalid-link = Deze link is niet geldig. Plak de volledige deellink, inclusief het deel na de #.
join-wrong-project = Deze link hoort bij een ander project.

import-tricount-link-label = Tricount-link of -sleutel
import-tricount-key-required = Voer een Tricount-link of -sleutel in.
import-tricount-encryption-failed = Versleuteling mislukt.
import-tricount-unimportable = Er is niets geïmporteerd: deze Tricount heeft leden met een Tricount-account of bedragen die niet kloppen (betrokken posten: { $count }).

### Expenses

save = Opslaan
saving = Bezig met opslaan…
adding = Bezig met toevoegen…
link-copied = Link gekopieerd
missing-encryption-key = Versleutelsleutel ontbreekt.
missing-encryption-key-title = Versleutelsleutel ontbreekt
missing-encryption-key-hint = De link die je gebruikte bevat niet de sleutel om dit project te ontsleutelen. Gebruik de volledige link van degene die het heeft aangemaakt.
project-locked-hint = Dit apparaat heeft de sleutel van dit project niet. Open de deellink om het te ontgrendelen.
expenses-unreadable = Sommige items kunnen op dit apparaat niet worden gelezen en tellen niet mee in de totalen. Werk de app bij om ze te zien.
project-unlock = Ontgrendelen
project-no-local-data-hint = Log in om de gegevens van dit project voor het eerst te laden.
project-gone-hint = Het is verwijderd toen de laatste deelnemer het verliet. De deellink werkt niet meer, ook niet als je hem opnieuw opent.

expense-add = Uitgave toevoegen
transfer-add = Overboeking toevoegen
expense-edit-title = Uitgave bewerken
expense-category = Categorie
expense-delete-title = Uitgave verwijderen
expense-delete-message = ‘{ $name }’ wordt definitief verwijderd. Dit kan niet ongedaan worden gemaakt.
expense-inconsistent-amounts = Bedragen kloppen niet
expenses-empty = Geen uitgaven
expenses-empty-hint = Voeg je eerste uitgave toe met de knop hieronder

expense-type-expense = Uitgave
expense-type-transfer = Overboeking
expense-type-gain = Inkomsten
expense-type-group = Type
expense-paid-by = betaald door
expense-sent-by = verstuurd door
expense-contributed-by = bijgedragen door

expense-name-required = Een naam is verplicht.
expense-amount-not-positive = Het bedrag moet groter zijn dan 0.
expense-no-payer = Selecteer minstens één betaler.
expense-no-debtor = Selecteer minstens één persoon die iets verschuldigd is.
expense-invalid-date = Deze datum is niet geldig.
expense-payers-mismatch = De betalers komen uit op { $sum }, dat komt niet overeen met het bedrag van de uitgave ({ $total }).
expense-debtors-mismatch = De schuldenaren komen uit op { $sum }, dat komt niet overeen met het bedrag van de uitgave ({ $total }).

participants-none = Niemand
participants-everyone = Iedereen ({ $count })
participants-some = { $count } van { $total }
participants-select-all = Alles selecteren
participants-remaining = { $amount } te weinig
participants-over-by = { $amount } te veel
participants-who-paid = Wie heeft betaald?
participants-who-received = Wie heeft ontvangen?
participants-who-transfers = Wie boekt over?
participants-who-receives = Wie ontvangt?
participants-for-whom = Voor wie?

stats-total-expenses = Totaal uitgaven
stats-my-expenses = Mijn uitgaven

tab-expenses = Uitgaven
tab-balance = Balans
tab-reimbursements = Afrekenen
balance-gets-back = Krijgt terug
balance-owes = Is schuldig
balance-settled = Verrekend
reimbursements-empty-hint = Hier verschijnen afrekenvoorstellen zodra de rekeningen niet kloppen
reimbursement-owes = { $debtor } is { $creditor } verschuldigd

user-selection-title = Welke deelnemer ben jij?
user-selection-hint = Kies je naam uit de lijst.
user-selection-required = Selecteer een deelnemer.

edit-project-deferred-new-members = het toevoegen van nieuwe deelnemers
edit-project-deferred-removals = het verwijderen van deelnemers
edit-project-offline-deferred = Offline: { $items } wordt toegepast zodra je weer verbinding hebt.

export-failed = Exporteren mislukt: { $reason }

history-expense-added = Uitgave toegevoegd: { $name }
history-expense-edited = Uitgave bewerkt: { $name }
history-expense-deleted = Uitgave verwijderd: { $name }
move-expense = Naar een project verplaatsen
move-expense-title = Verplaatsen naar…
move-current = Huidig
move-converted = { $currency }, omgerekend tegen de maandkoers
move-note-personal = Je kiest in de volgende stap wie betaalde en voor wie. De uitgave verlaat Mijn uitgaven.
move-note-group = De uitgave verlaat { $project } voor alle leden, en het saldo van de groep wordt opnieuw berekend. Je kiest in de volgende stap wie betaalde en voor wie.
move-banner = Van { $from } naar { $to }
move-submit = Verplaatsen
move-done = Uitgave verplaatst naar { $project }
history-expense-moved = Uitgave verplaatst naar een ander project: { $name }
personal-badge = Eigen
move-members =
    { $count ->
        [one] { $count } lid
       *[other] { $count } leden
    }
history-project-edited = Project bewerkt: { $name }
history-name-changed = Naam: ‘{ $from }’ → ‘{ $to }’
history-description-added = Omschrijving toegevoegd: ‘{ $value }’
history-description-removed = Omschrijving verwijderd: ‘{ $value }’
history-description-changed = Omschrijving: ‘{ $from }’ → ‘{ $to }’

### Sweep

field-amount = Bedrag
expense-name-placeholder = Restaurant, boodschappen…
expense-actions = Acties voor de uitgave
expense-your-share = Jouw deel
expense-your-share-value = Jouw deel: { $amount } { $currency }
expense-share-percent = { $percent }%
expense-added-by = Toegevoegd door { $name } op { $date }
expense-inconsistent-detail = Bedragen kloppen niet: { $paid } betaald, { $owed } verschuldigd, voor een uitgave van { $total }. Bewerk de uitgave om dit te corrigeren.
missing-access-key = Toegangssleutel ontbreekt. Open dit project via de deellink.
filter-all = Alles
filter-my-payments = Mijn betalingen
filter-my-debts = Wat ik verschuldigd ben
expense-filter-group = Filter
expenses-search = Uitgaven zoeken
expenses-no-match = Geen uitgave gevonden.
expenses-search-open = Zoeken
expenses-search-clear = Zoekopdracht wissen
expenses-search-summary =
    { $count ->
        [one] { $count } resultaat
       *[other] { $count } resultaten
    } · { $total } · alle maanden
participants-shares-for = Delen van { $name }
participants-amount-for = Bedrag voor { $name }
reimbursement-add = Afrekening toevoegen
project-forget = Uit mijn lijst verwijderen
project-history-title = Geschiedenis
history-kind-add = Toegevoegd
history-kind-delete = Verwijderd
history-kind-edit = Bewerkt
export = Exporteren
export-json = JSON exporteren
export-csv = CSV exporteren
share-link = Delen
copy-link-failed = De link kon niet worden gekopieerd
open-in-app = Openen in de app
not-found-title = Pagina niet gevonden
not-found-back = Terug naar projecten

### Charts

charts-period = Periode
period-all = Alles
period-month = Maand
period-3months = 3 mnd
period-year = Jaar
period-custom = Eigen
charts-tab-categories = Categorieën
charts-tab-trends = Trends
charts-total-spent = Totaal uitgegeven
charts-avg-per-person = Gem. per persoon
charts-expense-count =
    { $count ->
        [one] { $count } uitgave
       *[other] { $count } uitgaven
    }
charts-nothing-to-show = Niets te tonen
charts-my-share-skipped =
    { $count ->
        [one] 1 project telt niet mee - geen deelnemer gekozen, of de gegevens zijn niet geladen.
       *[other] { $count } projecten tellen niet mee - geen deelnemer gekozen, of de gegevens zijn niet geladen.
    }

### Categories

category-food = Eten
category-transport = Vervoer
category-accommodation = Verblijf
category-leisure = Vrije tijd
category-shopping = Shoppen
category-services = Diensten
category-parties-gifts = Feesten & cadeaus
category-other = Overig
categories-manage = Categorieën beheren
categories-title = Categorieën
categories-yours = Jouw categorieën
categories-builtin = Standaard
categories-add = Categorie toevoegen
categories-in-parent = in { $parent }
categories-renamed-from = hernoemd · oorspronkelijk { $name }
categories-hidden-hint = verborgen · uitgaven gaan naar { $other }
categories-always-shown = altijd zichtbaar
categories-hidden = Verborgen
categories-new = Nieuwe categorie
categories-edit = Categorie bewerken
categories-parent = Telt mee in
categories-parent-hint = In de statistieken van al je projecten telt { $name } mee in { $parent }.
categories-create = Aanmaken
categories-hide = Deze categorie verbergen
categories-hide-hint = De uitgaven staan voor alle leden in { $other }.
categories-delete-title = “{ $name }” verwijderen?
categories-delete-body = De betrokken uitgaven ({ $count }) gaan terug naar { $parent }. Er wordt geen uitgave verwijderd.
categories-delete-body-unknown = De uitgaven gaan terug naar { $parent }. Er wordt geen uitgave verwijderd.
categories-delete-warning = De categorie verdwijnt voor alle leden van het project.
categories-summary-created = Aangemaakt: { $count }
categories-summary-renamed = Hernoemd: { $count }
categories-summary-hidden = Verborgen: { $count }
categories-summary-default = Geen eigen categorieën
categories-offline = Categorieën beheren kan alleen online. Maak verbinding en probeer het opnieuw.
categories-error-too-many = Een project heeft hoogstens 20 eigen categorieën.
categories-error-full = Dit project heeft de limiet voor categorieën bereikt.
categories-error-name = Geef de categorie een naam.
categories-error-missing = Deze categorie is intussen gewijzigd. Sluit en probeer het opnieuw.
history-category-created = Categorie aangemaakt: { $name }
history-category-edited = Categorie bewerkt: { $name }
history-category-deleted = Categorie verwijderd: { $name }
history-empty = Geen gebeurtenissen
not-found-hint = Deze pagina bestaat niet of is verplaatst.
payers-title-paid-by = Betaald door
payers-title-sender = Afzender
payers-title-contributors = Bijdragers
debtors-title-debtors = Verschuldigd
debtors-title-recipients = Ontvangers
debtors-title-beneficiaries = Begunstigden
debtors-title-for = Voor
debtors-title-recipient = Ontvanger
expense-split-between = verdeeld over { $count }
expense-people =
    { $count ->
        [one] { $count } persoon
       *[other] { $count } personen
    }

### Welcome

welcome-title = Jouw afrekening gaat niemand anders aan.
welcome-subtitle = Deel uitgaven met vrienden.
welcome-note = Gratis. Geen account nodig. Geen advertenties.
welcome-link-title = Eén link, en iedereen doet mee.
welcome-link-body = Niemand hoeft een account aan te maken.
welcome-link-account = Een account? Nooit verplicht. Het dient om je projecten op een ander apparaat terug te vinden, vrienden vanuit de app uit te nodigen en je betaalgegevens te delen.
welcome-demo-project = Weekendje Lyon
welcome-private-title = Niemand kan je rekeningen lezen. Zelfs wij niet.
welcome-private-body = Namen, bedragen, projecten: alles wordt op je apparaat versleuteld. Alleen jij hebt de sleutel.
welcome-private-names = Namen
welcome-private-amounts = Bedragen
welcome-private-projects = Projecten
welcome-scan-title = Maak een foto van de bon.
welcome-scan-body = Bedrag, datum en categorie vullen zich vanzelf in. Alles gebeurt op je telefoon. De foto wordt niet bewaard.
welcome-eu-title = 100 % Europees
welcome-no-ads = Geen advertenties
welcome-no-trackers = Geen trackers
welcome-step = Stap { $current } van { $total }
welcome-next = Volgende
welcome-skip = Overslaan
welcome-start = Aan de slag
welcome-how-it-works = Hoe werkt het precies?

### Help

help-intro = Een veelgestelde vraag? Tik om het antwoord uit te klappen.
help-create-project-q = Hoe maak ik een project aan?
help-create-project-a = Tik op het startscherm onderaan op +. Geef het project een naam, kies de valuta en je kunt beginnen.
help-add-participants-q = Hoe voeg ik deelnemers toe?
help-add-participants-a = Open het project en voeg deelnemers toe via de ledenlijst. Elke deelnemer kan bij een uitgave betalen of iets verschuldigd zijn.
help-share-project-q = Hoe deel ik een project?
help-share-project-a = Deel de URL van het project (die uit je adresbalk). Iedereen met de link kan het project bekijken en bewerken.
help-add-expense-q = Hoe voeg ik een uitgave toe?
help-add-expense-a = Tik binnen een project op +, vul het bedrag in en geef aan wie betaald heeft en tussen wie het wordt verdeeld. Je kunt ook een andere datum dan vandaag kiezen.
help-types-q = Wat is het verschil tussen uitgave, overboeking en inkomsten?
help-types-expense = - een aankoop door één persoon, verdeeld over meerdere.
help-types-transfer = - een terugbetaling van de ene persoon aan de andere, zonder verdeling.
help-types-gain = - ontvangen geld (een terugbetaling, een cadeau) om over meerdere personen te verdelen.
help-past-date-q = Kan ik een uitgave in het verleden dateren?
help-past-date-a = Ja, het datumveld is vrij. Het aanmaakmoment van het item wordt apart bewaard.
help-who-owes-q = Hoe berekent Counted wie wat verschuldigd is?
help-who-owes-a = Counted berekent het nettosaldo van elke deelnemer (wat je voorgeschoten hebt min wat je verschuldigd bent) en stelt dan de kortste reeks overboekingen voor om alles te vereffenen.
help-minimal-transfers-q = Waarom is het aantal voorgestelde overboekingen minimaal?
help-minimal-transfers-a = Het algoritme koppelt eerst saldi die elkaar precies opheffen en werkt de rest daarna af van de grootste schuldeiser naar de grootste schuldenaar. Resultaat: minder overboekingen om alles te vereffenen.
help-import-tricount-q = Hoe importeer ik een project uit Tricount?
help-import-tricount-a = Tik op het startscherm onderaan op ‘+’ en daarna op
help-import-tricount-b = Plak de deellink van de Tricount die je wilt importeren.
help-encryption-q = Zijn mijn gegevens versleuteld?
help-encryption-a = Ja. Counted combineert twee garanties:
help-encryption-e2ee-term = End-to-end-versleuteling
help-encryption-e2ee-def = - alles tussen jou en de server gaat versleuteld over de lijn.
help-encryption-zero-term = Geen toegang
help-encryption-zero-def = - jij versleutelt de gegevens voordat je ze verstuurt, en de server bewaart alleen versleutelde tekst. Wij kunnen die niet lezen.
help-encryption-see = Zie voor de details het
help-forgot-password-q = Wat gebeurt er als ik mijn wachtwoord vergeet?
help-forgot-password-warning = Niemand kan het voor je terughalen.
help-forgot-password-a = Je wachtwoord beschermt je sleutels en wordt nooit naar ons gestuurd: niemand - wij ook niet - kan je projecten in jouw plaats ontsleutelen. Bewaar het goed, het liefst in een wachtwoordmanager.
help-archive-delete-q = Hoe archiveer of verwijder ik een project?
help-archive-delete-a = Open in het project het menu en kies
help-archive-delete-b = om het te verbergen maar te bewaren. Een project wordt pas definitief verwijderd als de laatste deelnemer het verlaat.
help-delete-account-q = Hoe verwijder ik mijn account?
help-delete-account-a = Open de Instellingen en gebruik ‘Mijn account verwijderen’. Dat gebeurt direct en kan niet ongedaan worden gemaakt.
help-contact = Nog een vraag? Schrijf ons op

# Receipt scanning (mobile only)
expense-scan = Bon scannen
scan-in-progress = Bon wordt gelezen…
scan-error-capture = Die foto kon niet worden gemaakt. Probeer het opnieuw, of voer de uitgave handmatig in.
scan-error-unreadable = Niets leesbaars op deze bon. Voer de uitgave handmatig in.
scan-check-amount = Controleer het totaal - het was niet duidelijk afgedrukt.
scan-take-photo = Foto maken
scan-choose-photo = Foto kiezen

update-required-title = Update vereist
update-required-body = Deze versie van Counted is te oud om met de server te communiceren. Werk hem bij om de app te blijven gebruiken.
update-required-button = Bijwerken

### Common (aanvullingen)

confirm = Bevestigen
copy = Kopiëren
field-date = Datum
date-today = Vandaag
date-yesterday = Gisteren

### Errors (vrienden)

error-identity-taken = Een ander account heeft deze deelnemer al geclaimd.
error-self-friend-request = Je kunt jezelf niet als vriend toevoegen.
error-not-a-friend = Je kunt alleen mensen uit je vriendenlijst uitnodigen.
error-friend-has-no-key = Deze vriend heeft de nieuwste versie van de app nog niet geopend. Vraag hem of haar om één keer in te loggen en probeer het dan opnieuw.
error-friend-request-not-found = Dit vriendschapsverzoek bestaat niet meer.
error-invitation-not-found = Deze uitnodiging bestaat niet meer.
error-too-many-friend-requests = Te veel vriendschapsverzoeken voor nu. Probeer het morgen opnieuw.
error-too-many-invitations = Te veel openstaande uitnodigingen.
error-invalid-kdf-salt = De versleutelingsinstellingen zijn niet geldig. Werk de app bij en probeer het opnieuw.
error-mixed-project-batch = Deze deelnemers zitten niet allemaal in hetzelfde project.
error-invalid-payload = Deze versie van de app stuurde gegevens die de server niet accepteert. Werk hem bij en probeer het opnieuw.
error-invalid-public-key = Je versleutelingssleutel is niet geldig. Werk de app bij en probeer het opnieuw.
error-payment-methods-stale = Je betaalgegevens zijn op een ander apparaat gewijzigd. Laad opnieuw en probeer het opnieuw.

### Payment methods (delen)

payment-method-copy = { $name } kopiëren
payment-method-copied = Gekopieerd.
payment-method-copy-failed = Kopiëren mislukt - selecteer de tekst en kopieer hem met de hand.

### Expenses (valuta, bedrag)

expense-category-auto = Auto · { $emoji }
emoji-picker-label = Emoji
emoji-none = Geen emoji
expense-currency = Valuta van het bedrag
amount-op-add = Plus
amount-op-subtract = Min
amount-op-multiply = Keer
amount-op-divide = Gedeeld door
amount-op-equals = Is gelijk aan
amount-op-done = Klaar
expense-rate = Wisselkoers (optioneel)
expense-rate-hint = Laat leeg om de koers van de Europese Commissie (InforEuro) voor { $month } te gebruiken: 1 { $from } = { $rate } { $to }.
expense-rate-invalid = Voer een wisselkoers groter dan 0 in.
expense-rate-unavailable = Geen automatische koers beschikbaar - voer er zelf een in.
expenses-show-more = Meer tonen ({ $count } resterend)
expense-converted-from = { $amount } { $from } betaald · 1 { $from } = { $rate } { $to }
project-currency = Valuta
project-currency-locked = De valuta wordt vastgelegd bij het aanmaken van het project.
currency-search = Valuta zoeken
project-gone-title = Dit project bestaat niet meer
participants-by-shares = Op aandelen
split-amounts = Bedragen

### Reimbursements

reimbursements-empty-title = Alles is vereffend!
reimbursement-record = Vereffenen
settlement-recorded = Vereffening opgeslagen.
reimbursement-pay-with = Betalen
reimbursement-pay-shared-by = Gedeeld door { $name } - controleer de naam van de ontvanger die je app toont voordat je verstuurt.
reimbursement-pay-title = { $name } betalen
reimbursements-mine-title = Jij bent schuldig
reimbursements-others-title = Andere terugbetalingen

### Identity

identity-claimed = Gekoppeld aan een account
identity-claimed-by = Account van { $name }
identity-taken-repick = Een ander account heeft de deelnemer geclaimd die jij gebruikte. Kies een andere.
participant-gone-repick = De deelnemer die jij gebruikte is uit dit project verwijderd. Kies een andere.

### Edit project

edit-project-title = Project bewerken
edit-project-new-badge = nieuw

### Charts (aanvullingen)

charts-project = Project
charts-all-projects = Alle projecten
charts-date-from = Van
charts-date-to = Tot
charts-total = Totaal
charts-tab-people = Personen
charts-tab-projects = Projecten
charts-scope = Wiens uitgaven
charts-scope-group = Groep
charts-scope-me = Ik
charts-currency = Valuta
charts-my-share = Mijn deel
charts-share-of-total = { $pct }% van { $total }
charts-i-paid = Ik betaalde
charts-paid-more = { $amount } meer dan mijn deel
charts-paid-less = { $amount } minder dan mijn deel
charts-paid-even = Precies mijn deel
charts-part-title = Jouw deel per categorie
charts-part-desc = Grijs is wat de groep uitgaf, kleur is wat jij verbruikte.
charts-breakdown-title = Verdeling per categorie
charts-breakdown-desc = Tik op een segment of rij om de uitgaven te zien.
charts-of-total = { $amount } van { $total }
charts-show-all = Alles tonen ({ $count })
charts-show-less = Minder tonen
charts-spend-title = Uitgaven door de tijd
charts-group-by = Groeperen per
bucket-day = Dag
bucket-week = Week
bucket-month = Maand
charts-avg = gem.
charts-cat-title-day = { $category }, dag per dag
charts-cat-title-week = { $category }, week per week
charts-cat-title-month = { $category }, maand per maand
charts-cat-desc = Kies een categorie om die door de tijd te volgen.
charts-running-title = Lopend totaal
charts-running-desc = Sinds { $date }.
charts-avg-per-day = { $amount } / dag gemiddeld
charts-avg-per-week = { $amount } / week gemiddeld
charts-avg-per-month = { $amount } / maand gemiddeld
charts-people-title = Wie de groep droeg
charts-people-desc = Wat ieder betaalde, naast wat hij of zij verbruikte.
charts-paid = Betaald
charts-fair-share = Eerlijk deel
charts-you = (jij)
charts-net-more = betaalde meer
charts-net-less = betaalde minder
charts-balance-title = Jouw saldo door de tijd
charts-balance-desc = Boven de lijn is de groep jou iets schuldig. Eronder ben jij de groep iets schuldig.
charts-owed = Jij krijgt
charts-owe = Jij bent schuldig
charts-projects-title = Jouw deel, per project
charts-projects-desc = Totalen blijven per valuta.
history-by = Door { $name }

### Notifications

notifications-label = Meldingen
notifications-title = Meldingen
notifications-empty = Niets nieuws

### Friends

friends-title = Vrienden
friends-anonymous-body = Vrienden horen bij je account. Log in om mensen toe te voegen en ze zonder link in je projecten uit te nodigen.
friends-add-title = Een vriend toevoegen
friends-add-from-project = Als vriend toevoegen
friends-request-sent = Verzoek verstuurd
friends-no-account-key = Log opnieuw in om je vrienden op dit apparaat te beheren.
friends-incoming-title = Verzoeken
friends-accept = Accepteren
friends-decline = Weigeren
friends-list-title = Mijn vrienden
friends-remove = Verwijderen
friends-remove-confirm-title = Vriend verwijderen
friends-remove-confirm-message = { $email } staat dan niet meer in je vrienden, en jij niet meer in die van hen. Elk van jullie kan later een nieuw verzoek sturen.
friends-no-key = Nog niet klaar
friends-key-mismatch = De server stuurde voor je account een publieke sleutel die niet bij je privésleutel past. Je veiligheidscode is verborgen en uitnodigingen staan uit op dit apparaat: misschien knoeit iemand met de server.
keypair-mismatch = De sleutels die de server voor je account stuurde, komen niet overeen met die op dit apparaat. Uitnodigingen en sleutelwissels staan hier uit tot ze overeenkomen: misschien knoeit iemand met de server.
friends-outgoing-title = Verstuurd
friends-notify = Laten weten via een bericht
friends-notify-message = Ik stap over op Counted voor onze gedeelde uitgaven. Gratis aanmelden op https://counted.fr met het adres { $email }: mijn vriendschapsverzoek staat daar klaar.
friends-notify-copied = Bericht gekopieerd
friends-notify-copy-failed = Kan het bericht niet kopiëren
guests-hint-title =
    { $count ->
        [one] { $count } deelnemer is nog niet bij het project gekomen
       *[other] { $count } deelnemers zijn nog niet bij het project gekomen
    }
guests-hint-later = Later
guests-hint-send = Nodig ze uit
guests-send-title = Project naar de deelnemers sturen
guests-send-friend = Een Counted-vriend uitnodigen
guests-send-friend-badge = Zonder link
guests-send-friend-hint = Heeft die al een account, dan krijgt die een versleutelde uitnodiging; niets gaat via je berichtenapp.
guests-send-link = Projectlink delen
guests-send-link-hint = Voor wie geen account heeft. Eén tik opent het project.
guests-send-link-warning = Iedereen met deze link kan het project lezen en bewerken. Je berichtenapp ziet hem ook.
invite-friends-title = Vrienden uitnodigen
invite-friends-hint = De projectsleutel wordt op dit apparaat voor elke vriend versleuteld. De server ziet hem nooit.
invite-friends-empty = Nog geen vrienden om uit te nodigen.
invite-friends-button = Uitnodigen
invite-sent = { $count ->
    [one] Uitnodiging verstuurd
   *[other] { $count } uitnodigingen verstuurd
}
invitation-badge = Uitnodiging
invitation-to = Deelnemen aan “{ $name }”
invitation-to-unnamed = Deelnemen aan een project
invitation-unreadable = Deze uitnodiging kan niet op dit apparaat worden geopend
invitation-from = Van { $email }
invitation-accept = Deelnemen
invitation-decline = Weigeren

# Participants in the create and edit modals, and the "who are you?" picker - see
# docs/plans/friends.md §11.
participants-you-label = Je naam in dit project
participants-you-badge = Jij
participants-others = Andere deelnemers
participants-removed = Verwijderd
participant-restore = Herstellen
participants-empty = Voeg hieronder deelnemers toe.
participants-duplicate = “{ $name }” staat al in de lijst.
participants-input-label = Een vriend toevoegen of een naam typen
participants-input-placeholder = Vriend of een naam
participants-suggest-friend = Vriend · doet mee als “{ $name }”, krijgt een uitnodiging
participants-suggest-not-ready = Vriend · nog niet klaar
participants-suggest-guest = “{ $text }” toevoegen
participants-suggest-guest-sub = Gast
participants-friends = Je vrienden
participants-all-friends = Alle vrienden
participants-login-hint = Log in om mensen direct uit je vriendenlijst toe te voegen.
participants-invite-badge = Uitnodigen
participants-guest-badge = Gast
participants-rename = { $name } hernoemen
participants-remove = { $name } verwijderen
participants-rename-label = Nieuwe naam
participants-rename-save = Naam opslaan
participants-rename-hint = De naam die iedereen in dit project ziet. De uitnodiging gaat nog steeds naar { $email }.
participants-invited-badge = Uitgenodigd
participants-invited-sub = { $email } · nog niet geaccepteerd
participants-invited-pending = Uitnodiging nog niet geaccepteerd
participants-unlinked = Niet gekoppeld aan een account
add-project-create-invite = Aanmaken en { $count } uitnodigen
edit-project-save-invite = Opslaan en { $count } uitnodigen
edit-project-you-are = In dit project ben je { $name }
edit-project-no-identity = Je hebt nog niet gekozen wie je bent
edit-project-switch = Wisselen
edit-project-choose = Kiezen
invite-failed = Deze uitnodigingen konden niet worden verstuurd: { $emails }
invite-waiting = Deze vrienden moeten Counted één keer openen voordat ze kunnen worden uitgenodigd: { $emails }
key-offer-title = Een andere sleutel voor { $name }
key-offer-body = Dit apparaat heeft een andere sleutel voor dit project, die het hier nog nooit heeft geopend.
key-offer-choices = Deze sleutel gebruiken: het project opent met de sleutel uit deze link of uitnodiging. Annuleren: het apparaat houdt zijn huidige sleutel en er verandert niets.
key-offer-use = Deze sleutel gebruiken
invite-again = Opnieuw uitnodigen
friend-picker-title = Vrienden toevoegen
user-selection-invited-hint = { $email } heeft je uitgenodigd voor “{ $project }”.
user-selection-suggested = Voorgesteld
user-selection-suggested-sub = { $email } heeft je onder deze naam toegevoegd
user-selection-confirm-as = Ik ben { $name }
user-selection-missing = Staat je naam er niet bij? Vraag een deelnemer om je toe te voegen in de projectinstellingen.

## Terugkerende uitgaven

repeat-label = Herhalen
repeat-none = Herhaalt niet
repeat-weekly = Elke week
repeat-biweekly = Elke 2 weken
repeat-monthly = Elke maand
repeat-quarterly = Elke 3 maanden
repeat-yearly = Elk jaar
repeat-every-weeks = Elke { $count } weken
repeat-every-months = Elke { $count } maanden
repeat-every-years = Elke { $count } jaar
repeat-custom = Aangepast…
repeat-every = Elke
repeat-unit-weeks = weken
repeat-unit-months = maanden
repeat-unit-years = jaar
repeat-on-weekday = op { $weekday }
repeat-on-day = op dag { $day }
repeat-on-day-month = op { $day } { $month }
repeat-month-end = In kortere maanden valt hij op de laatste dag.
repeat-ends = Eindigt
repeat-ends-never = Nooit
repeat-ends-on = Op een datum
repeat-ends-after = Na
repeat-fewer = Minder
repeat-more = Meer
repeat-last-on = laatste op { $date }
repeat-variable = Bedrag verandert elke keer
repeat-variable-hint = Elke wordt toegevoegd met het laatste bedrag en gemarkeerd als ‘te bevestigen’.
repeat-done = Klaar
repeat-no-end = Geen einde
repeat-until = Tot { $date }
repeat-occurrences = Aantal keer: { $count }
repeat-offline = Vereist een verbinding. De uitgave zelf kan wel worden toegevoegd.
repeat-foreign = Herhaalt als { $amount } { $currency }, eenmalig omgerekend tegen de koers van vandaag. Je krijgt een waarschuwing als de koers meer dan 5% verandert.
repeat-backfill = Begint in het verleden. Nu toegevoegde uitgaven: { $count }.
add-and-repeat = Toevoegen en herhalen
weekday-1 = maandag
weekday-2 = dinsdag
weekday-3 = woensdag
weekday-4 = donderdag
weekday-5 = vrijdag
weekday-6 = zaterdag
weekday-7 = zondag
recurring-title = Terugkerende uitgaven
recurring-strip = Terugkerende uitgaven: { $count }
recurring-next = Volgende: { $name }, { $date }
recurring-to-confirm = Te bevestigen: { $count }
recurring-per-month = Per maand, ong.
recurring-your-share = Jouw deel
recurring-active = Actief
recurring-paused = Gepauzeerd
recurring-finished = Afgelopen
recurring-paid-by = betaald door { $name }
recurring-next-on = Volgende op { $date }
recurring-progress = { $done } van { $total }
recurring-rate-badge = Koers { $percent }% veranderd
recurring-empty = Er herhaalt nog niets. Kies ‘Herhalen’ bij het toevoegen van een uitgave: huur, abonnementen, rekeningen.
recurring-next-ones = Volgende
recurring-added-so-far = Tot nu toe toegevoegd
recurring-set-up-by = Ingesteld door
recurring-pause = Pauzeren
recurring-resume = Hervatten
recurring-stop = Stoppen met herhalen
recurring-stop-title = ‘{ $name }’ stoppen?
recurring-stop-message = Hij herhaalt niet meer. Al toegevoegde uitgaven blijven.
recurring-resume-title = ‘{ $name }’ hervatten?
recurring-resume-message = Volgende op { $date }. Data die tijdens de pauze zijn gemist, worden niet toegevoegd.
recurring-edit-title = Terugkerende uitgave bewerken
recurring-edit-banner = Wijzigingen gelden vanaf { $date }. Al toegevoegde uitgaven blijven zoals ze zijn.
recurring-next-on-label = Volgende op
recurring-next-too-early = De volgende datum moet na de laatst toegevoegde uitgave liggen.
recurring-use-stop = Gebruik ‘Stoppen met herhalen’ op de terugkerende uitgave om hem te beëindigen.
recurring-drift = De koers van { $currency } is sinds het instellen { $percent }% veranderd. Elke wordt nog toegevoegd als { $amount } { $project_currency } (1 { $currency } = { $rate }). Tegen de koers van vandaag zou het { $today_amount } { $project_currency } zijn.
recurring-use-rate = Koers van vandaag gebruiken
recurring-keep = { $amount } { $currency } behouden
recurring-added = Terugkerende uitgaven toegevoegd: { $count }
recurring-blocks-removal = { $name } kan nog niet worden verwijderd: onderdeel van { $rules }. Haal { $name } daar uit of stop ze, en sla dan opnieuw op.
history-recurring-added = Automatisch toegevoegd: { $name } ({ $date })
history-recurring-created = Terugkerende uitgave ingesteld: { $name }
history-recurring-edited = Terugkerende uitgave bewerkt: { $name }
history-recurring-paused = Terugkerende uitgave gepauzeerd: { $name }
history-recurring-resumed = Terugkerende uitgave hervat: { $name }
history-recurring-stopped = Terugkerende uitgave gestopt: { $name }
occurrence-recurring = Terugkerende uitgave
occurrence-auto = Automatisch toegevoegd door een terugkerende uitgave.
occurrence-auto-next = Automatisch toegevoegd door een terugkerende uitgave. Volgende op { $date }.
occurrence-auto-stopped = Automatisch toegevoegd door een terugkerende uitgave die inmiddels gestopt is.
occurrence-manage = Beheren
estimate-badge = Te bevestigen
expense-pending = In afwachting
estimate-title = Bedrag te bevestigen.
estimate-body = Toegevoegd met het vorige bedrag. Vul het echte bedrag in zodra je het weet.
estimate-confirm = Bedrag bevestigen
apply-to = Toepassen op
apply-this-only = Alleen deze uitgave
apply-and-next = Deze en de volgende
apply-and-next-hint = De terugkerende uitgave verandert vanaf { $date }
apply-rule-failed = De uitgave is opgeslagen, maar de terugkerende uitgave is niet gewijzigd.
occurrence-delete-message = ‘{ $name }’ van { $date } wordt definitief verwijderd. Hij blijft herhalen en deze datum komt niet terug.
occurrence-delete-one = Alleen deze verwijderen
occurrence-delete-stop = Verwijderen en stoppen met herhalen
error-recurring-clock = De klok van dit apparaat loopt voor. Controleer datum en tijd.
error-recurring-not-found = Deze terugkerende uitgave bestaat niet meer.
error-recurring-stale = Iemand heeft deze terugkerende uitgave intussen gewijzigd. Hij is opnieuw geladen: controleer hem en sla opnieuw op.
error-too-many-recurring = Dit project heeft al 50 terugkerende uitgaven. Stop er een die je niet meer nodig hebt om een nieuwe toe te voegen.
error-user-in-recurring = Deze deelnemer maakt deel uit van een terugkerende uitgave. Haal de deelnemer daar eerst uit of stop de uitgave.

home-owed-to-you = Jij krijgt nog
home-you-owe = Jij moet nog
home-details-show = Details tonen
home-details-hide = Details verbergen
home-all-settled = Alles verrekend
home-pay = Betalen
home-remind = Herinneren
home-remind-message = Kleine herinnering: er staan nog terugbetalingen open in { $project }.
home-row-owed = jij krijgt nog
home-row-owe = jij moet nog
home-row-settled = Verrekend
home-row-quits = Jij staat quitte
home-row-pending =
    { $count ->
        [one] { $count } terugbetaling open
       *[other] { $count } terugbetalingen open
    }
home-totals-partial = Zonder projecten zonder wisselkoers
error-account-key-changed = De sleutel van je account is op een ander apparaat gewijzigd. Log opnieuw in om verder te gaan.
settings-security = Beveiliging
settings-change-password-hint = Je projecten en gegevens blijven zoals ze zijn. Je andere apparaten worden uitgelogd.
change-password-title = Wijzig je wachtwoord
change-password-current = Huidig wachtwoord
change-password-new = Nieuw wachtwoord
change-password-confirm = Bevestig het nieuwe wachtwoord
change-password-mismatch = De twee nieuwe wachtwoorden komen niet overeen.
change-password-submit = Wachtwoord wijzigen
change-password-submitting = Bezig met wijzigen…
change-password-done = Wachtwoord gewijzigd. Je andere apparaten zijn uitgelogd.
error-rotation-incomplete = Je account is intussen gewijzigd. Probeer het opnieuw.
error-key-lifecycle-unavailable = Nog niet beschikbaar: eerst heeft iedereen de nieuwste versie van de app nodig.
settings-compromised-open = Mijn account beveiligen
compromised-title = Beveilig je account
compromised-body = Je wachtwoord en de sleutels van je account worden vervangen, en alle andere apparaten worden uitgelogd. Je vrienden zien je veiligheidscode veranderen: vergelijk hem opnieuw met hen.
compromised-keeps = Wie je wachtwoord had, houdt wat die al heeft geopend: de projecten die tot nu toe leesbaar waren.
compromised-submit = Wachtwoord en sleutels vervangen
compromised-done = Je account is beveiligd. Vraag je vrienden je nieuwe veiligheidscode te vergelijken.
friends-code-matches = De code klopt
friends-code-trusted = Code bevestigd.
settings-recovery-key = Herstelsleutel
settings-recovery-key-pending = Een nieuwe sleutel vervangt hem op { $when }. Was jij dit niet? Annuleer met je huidige sleutel.
recovery-key-cancel-replacement = Nieuwe sleutel annuleren
recovery-key-title = Je herstelsleutel
recovery-key-body = Schrijf hem op of bewaar hem waar alleen jij bij kunt. Counted kan hem niet opnieuw tonen of voor je terughalen.
recovery-key-copy = Kopiëren
recovery-key-copied = Gekopieerd
recovery-key-save = Opslaan als bestand
recovery-key-print = Afdrukken
recovery-key-next = Doorgaan
recovery-key-confirm-body = Typ ter controle groep { $n } van de sleutel.
recovery-key-confirm-wrong = Die groep klopt niet. Kijk nog eens naar je kopie.
recovery-key-password-body = Voer je wachtwoord in om de sleutel aan je account toe te voegen.
recovery-key-submit = Sleutel opslaan
recovery-key-done = Herstelsleutel opgeslagen.
recovery-key-replacement-done = Nieuwe sleutel opgeslagen. Hij vervangt de huidige over 72 uur; tot dan werkt de huidige nog.
recovery-key-field = Herstelsleutel
recovery-key-file-intro = Counted-herstelsleutel
recovery-key-cancel-done = De nieuwe sleutel is geannuleerd.
compromised-new-key = Bewaar nu je nieuwe herstelsleutel: hij vervangt de oude over 72 uur, en tot dan werkt de oude nog.
pending-reset-title = Opnieuw instellen aangevraagd
pending-reset-body = Dit gaat in op { $when }. Was jij dit niet? Annuleer met je wachtwoord.
pending-reset-cancel = Opnieuw instellen annuleren
pending-reset-cancelled = Het opnieuw instellen is geannuleerd.
login-forgot = Wachtwoord vergeten?
forgot-title = Wachtwoord vergeten
forgot-body = Voer je e-mailadres in. Als er een account bij hoort, sturen we een link om een nieuw wachtwoord te kiezen.
forgot-submit = Link versturen
forgot-sent = Als dit adres een account heeft, is er een link onderweg. Die verloopt over 24 uur.
reset-title = Je account terugkrijgen
reset-with-key-body = Voer je herstelsleutel en een nieuw wachtwoord in: alles komt terug zoals het was.
reset-submit-key = Mijn account herstellen
reset-no-key = Ik heb mijn herstelsleutel niet
reset-without-key-title = Opnieuw instellen zonder herstelsleutel
reset-without-key-body = Je account begint over 72 uur opnieuw met een nieuw wachtwoord. Je projecten komen terug van je apparaten en deellinks; je voorkeuren, betaalgegevens en vriendcodes niet. Tot dan werkt je huidige wachtwoord nog en kun je er de reset mee annuleren.
reset-name = Je naam
reset-submit-request = Mijn account opnieuw instellen
reset-requested = Klaar. Je nieuwe wachtwoord werkt vanaf { $when }. Tot dan werkt je huidige wachtwoord nog en kan het dit annuleren.
reset-pending-already = Er staat al een herinstelling klaar tot { $when }. Een nieuw verzoek start de 72 uur opnieuw.
recovered-done = Je account is terug. Je andere apparaten zijn uitgelogd.
login-reset-undoable = Dit wachtwoord is de afgelopen 30 dagen vervangen door een herinstelling. Heb je daar niet om gevraagd, maak het dan ongedaan: je account komt terug zoals het was.
login-undo-reset = Herinstelling ongedaan maken
reset-undone = De herinstelling is ongedaan gemaakt. Je account is weer zoals het was.
error-reset-pending = Dit nieuwe wachtwoord werkt zodra de herinstelling ingaat, 72 uur na het verzoek.
error-account-disabled = Dit account is kort na een herinstelling verwijderd en wacht op wissing. Het vorige wachtwoord kan de herinstelling nog ongedaan maken.
error-recovery-key-invalid = Deze herstelsleutel opent dit account niet.
error-project-key-required = Dit project heeft zijn sleutel nodig. Open het opnieuw via de deellink.
names-or = { $first } of { $last }
participants-admin-badge = Beheerder
edit-project-admins = Beheerders: { $count }
edit-project-succession = Als de laatste beheerder het project verlaat, wordt het lid met een account dat het langst in het project zit beheerder.
member-sheet-account = Lid met een account
member-actions = Acties voor { $name }
member-make-admin = Beheerder maken
member-make-admin-hint = Kan dan de projectsleutel wijzigen en iemand verwijderen.
member-made-admin = { $name } is nu beheerder.
group-security = Projectbeveiliging
group-security-admin = Projectbeveiliging · je bent beheerder
group-rotate = Projectsleutel wijzigen
group-rotate-hint = Als de projectlink is uitgelekt: wie de oude link heeft, verliest de toegang tot het project.
group-remove = Iemand uit het project verwijderen
group-remove-hint = Neemt de toegang af en wijzigt tegelijk de sleutel.
group-admins-only = Alleen voor beheerders.
group-admins-only-account = Alleen voor beheerders, die een account hebben.
group-ask-admin = Vraag het { $names }, beheerder van het project.
group-ask-admins = Vraag het { $names }, de beheerders van het project.
group-ask-an-admin = Vraag het een beheerder van het project.
group-with-account = Met een account
group-free = Gratis
group-sign-in-hint = Een beheerder van het project, { $names }, kan je daarna de rol geven.
group-sign-in-hint-any = Een beheerder van het project kan je daarna de rol geven.
group-no-admin = Dit project heeft geen beheerder, dus de sleutel kan niet veranderen.
group-hide = Iemand verbergen
group-hide-hint = Verbergt diegene in de lijsten zonder de toegang af te nemen: de link en de sleutel blijven geldig.
group-recreate-hint = Dit project heeft geen beheerder. Maak een nieuw project om iemand te verwijderen.
group-recreate = Nieuw project maken
group-no-admin-yet = Dit project heeft nog geen beheerder.
group-creator-hint = Alleen jij kunt beheerder van dit project worden, door een account aan te maken.
group-creator-elsewhere-hint = Alleen de persoon die het project heeft gemaakt, kan de beheerder worden, door een account aan te maken. Maak anders een nieuw project om iemand te verwijderen.
member-revoke-admin = Beheerdersrol afnemen
member-revoke-admin-hint = Kan de projectsleutel dan niet meer wijzigen en niemand meer verwijderen.
member-revoked-admin = { $name } is geen beheerder meer.
member-step-down = Geen beheerder meer zijn
member-step-down-hint = Je kunt de projectsleutel dan niet meer wijzigen en niemand meer verwijderen.
member-stepped-down = Je bent geen beheerder meer.
member-remove = Uit het project verwijderen
member-remove-hint = Wijzigt de projectsleutel: { $name } kan niets nieuws meer lezen.
member-hide = Verbergen in de lijsten
error-user-not-admin = Dit lid is geen beheerder.
error-creator-proof-invalid = Dit apparaat heeft dit project niet gemaakt.
error-project-has-admin = Dit project heeft al een beheerder.
error-not-project-admin = Alleen de beheerders van het project kunnen dit.
error-user-not-claimed = Deze deelnemer heeft geen account en kan dus geen beheerder zijn.
error-project-key-rotated = De projectsleutel is gewijzigd. Vraag een lid om de nieuwe link.
error-project-rotating = De projectsleutel wordt gewijzigd. Probeer het zo opnieuw.
error-rotation-expired = Deze sleutelwijziging duurde te lang en is geannuleerd. Begin opnieuw.
error-rotation-rows-changed = Het project is tijdens de sleutelwijziging veranderd. Begin opnieuw.
project-rotated-title = Je hebt geen toegang meer tot dit project
project-rotated-body = De sleutel van dit project is door de beheerder gewijzigd. Vraag de beheerder om de nieuwe link.
project-rotated-open-link = Een link openen
rotation-remove-title = { $name } uit het project verwijderen?
rotation-rotate-title = Projectsleutel wijzigen?
rotation-remove-body = Een lid verwijderen betekent een nieuwe sleutel maken. De nieuwe link moet je delen met de mensen zonder account.
rotation-rotate-body = Wie de oude link heeft, verliest de toegang tot het project. De nieuwe link moet je delen met de mensen zonder account.
rotation-boxed =
    { $count ->
        [0] Geen ander lid krijgt de sleutel automatisch
        [one] { $count } lid krijgt de sleutel automatisch
       *[other] { $count } leden krijgen de sleutel automatisch
    }
rotation-by-link-title = Mensen naar wie je de nieuwe link moet sturen
rotation-by-link-hint = Deze mensen hebben geen account. Je stuurt ze de link meteen daarna.
rotation-names-and = { $first } en { $last }
rotation-names-and-others =
    { $names } en { $count ->
        [one] { $count } ander
       *[other] { $count } anderen
    }
rotation-confirm-remove = Verwijderen en sleutel wijzigen
rotation-confirm-rotate = Sleutel wijzigen
rotation-done-title = Nieuwe sleutel actief
rotation-done-removed = { $name } hoort niet meer bij het project.
rotation-done-rotated = De projectsleutel is gewijzigd.
rotation-done-boxed =
    { $count ->
        [0] Geen ander lid heeft de sleutel automatisch gekregen.
        [one] { $count } lid heeft de sleutel automatisch gekregen.
       *[other] { $count } leden hebben de sleutel automatisch gekregen.
    }
rotation-new-link =
    { $count ->
        [0] Nieuwe projectlink
        [one] Nieuwe link om te sturen · { $count } persoon zonder account
       *[other] Nieuwe link om te sturen · { $count } mensen zonder account
    }
rotation-link-renewed = De link is vernieuwd.
rotation-done = Klaar
history-key-changed = Projectsleutel gewijzigd
history-member-removed = { $name } hoort niet meer bij het project
project-rotated-row-hint = De sleutel is gewijzigd. Vraag om de nieuwe link.

## Settings and Friends redesign (docs/plans/friends-settings-redesign.md)
settings-group-sharing = Delen
settings-group-help = Hulp en informatie
settings-friends-changed = { $count ->
    [one] 1 code gewijzigd
   *[other] { $count } codes gewijzigd
}
settings-recovery-key-created = Aangemaakt
settings-recovery-key-missing = Nog aanmaken
settings-recovery-key-replacing = Vervanging gepland
settings-password = Wachtwoord
settings-compromised-short = Kent iemand anders je wachtwoord?
settings-recovery-key-footnote = Zonder herstelsleutel wist een vergeten wachtwoord je voorkeuren, betaalgegevens en vriendcodes.
erase-device-row = Dit apparaat wissen
erase-device-row-hint = Meldt je af en bewaart alleen de taal.
account-delete-hint = Direct en definitief.
settings-open-source = Open source, AGPL-3.0
settings-upsell-short = Zonder account blijven je projecten op dit apparaat. Met een account volgen ze je overal, versleuteld.
settings-locked-group = Met een account
settings-locked-friends-short = Uitnodigen zonder link
settings-locked-payment-short = Zichtbaar voor wie je geld schuldig is
payment-methods-title = Betaalmethoden
payment-methods-row-hint = Gedeeld met je vrienden
payment-methods-intro = Gedeeld met je vrienden. Versleuteld met je account.
payment-methods-footnote = Een betaalmethode wordt (max. 20) gedeeld met de leden van je projecten in het tabblad Afrekenen.
payment-method-shared-pill = Gedeeld
payment-method-share-groups = Delen in je groepen
payment-method-label-optional = Naam (optioneel)
payment-method-delete-confirm-title = Deze betaalmethode verwijderen?
payment-method-delete-confirm-message = { $name } wordt niet meer getoond in je projecten.
payment-methods-empty-title = Geen betaalmethode
payment-methods-empty-body = Voeg je IBAN of betaalapp toe: wie je geld schuldig is, ziet het.
payment-method-masked = { $kind } eindigend op { $last }
friends-to-check = Te controleren
friends-code-changed-row = Zijn code is gewijzigd
friends-own-code = Jouw veiligheidscode
friends-own-code-hint = Je vrienden zien deze code naast je adres. Komt die overeen met de hunne, dan nodigen ze jou uit en niemand anders.
friends-received = Ontvangen
friends-received-on = Ontvangen op { $date }
friends-pending = In afwachting
friends-from-project = Uit een project
friends-accept-label = Verzoek van { $email } accepteren
friends-decline-label = Verzoek van { $email } weigeren
friends-add-hint-short = Deze persoon ziet je verzoek als die een account heeft. Zo niet, laat het weten.
friends-send = Versturen
friends-sent-hint = Nog geen account? Laat het weten: het bericht nodigt die persoon uit zich met dit adres aan te melden.
friends-done = Klaar
friends-their-code = Zijn veiligheidscode
friends-code-compare-hint = Lees hem samen. Die vriend moet dezelfde code bovenaan zijn Vrienden-pagina zien.
friends-no-key-hint = Nog niet klaar: de sleutel wordt aangemaakt als die Counted weer opent. Tot dan kun je die vriend niet uitnodigen.
friends-remove-row = Uit mijn vrienden verwijderen
friends-new-code = Nieuwe code
friends-key-changed-hint-short = Controleer deze code samen, buiten deze app. Tot dan zijn uitnodigingen met die vriend geblokkeerd.
friends-kem-changed-title = Zijn sleutel is vervangen
friends-kem-changed-body = Het post-kwantumdeel van zijn sleutel is veranderd, maar zijn code niet. Een account beveiligen verandert beide, dus misschien heeft iemand deze sleutel vervangen. Uitnodigingen met die vriend blijven geblokkeerd.
friends-kem-changed-hint = Je kunt die vriend verwijderen en dan een nieuw verzoek sturen: de sleutel wordt opnieuw vastgelegd.
friends-pending-sheet = Verzoek verstuurd, in afwachting.
friends-withdraw-row = Verzoek annuleren
friends-empty-title = Nog geen vrienden
friends-empty-body = Voeg iemand toe via e-mail en nodig die dan zonder link uit in een project.
friends-requests-empty-title = Geen verzoeken
friends-requests-empty-body = Verzoeken die je ontvangt en verstuurt, verschijnen hier.
friends-code-hidden = Je code is verborgen.
friends-code-hidden-body = Een sleutel die voor je account is geleverd, past niet bij dit apparaat. Uitnodigingen staan hier uit.
notifications-manage-friends = Mijn vrienden beheren
notifications-friend-request-on = Vriendschapsverzoek · { $date }

### Link preview

share-preview-title = Onze uitgaven op Counted
share-preview-description = Open de link om de uitgaven te zien en wie wat verschuldigd is. Zonder account, versleuteld op je apparaat.

### Currency names

currency-EUR = Euro
currency-AED = Verenigde Arabische Emiraten-dirham
currency-AFN = Afghaanse afghani
currency-ALL = Albanese lek
currency-AMD = Armeense dram
currency-AOA = Angolese kwanza
currency-ARS = Argentijnse peso
currency-AUD = Australische dollar
currency-AWG = Arubaanse gulden
currency-AZN = Azerbeidzjaanse manat
currency-BAM = Bosnische convertibele mark
currency-BBD = Barbadaanse dollar
currency-BDT = Bengalese taka
currency-BHD = Bahreinse dinar
currency-BIF = Burundese frank
currency-BMD = Bermuda-dollar
currency-BND = Bruneise dollar
currency-BOB = Boliviaanse boliviano
currency-BRL = Braziliaanse real
currency-BSD = Bahamaanse dollar
currency-BTN = Bhutaanse ngultrum
currency-BWP = Botswaanse pula
currency-BYN = Belarussische roebel
currency-BZD = Belizaanse dollar
currency-CAD = Canadese dollar
currency-CDF = Congolese frank
currency-CHF = Zwitserse frank
currency-CLP = Chileense peso
currency-CNY = Chinese yuan
currency-COP = Colombiaanse peso
currency-CRC = Costa Ricaanse colon
currency-CUP = Cubaanse peso
currency-CVE = Kaapverdische escudo
currency-CZK = Tsjechische kroon
currency-DJF = Djiboutiaanse frank
currency-DKK = Deense kroon
currency-DOP = Dominicaanse peso
currency-DZD = Algerijnse dinar
currency-EGP = Egyptisch pond
currency-ERN = Eritrese nakfa
currency-ETB = Ethiopische birr
currency-FJD = Fiji-dollar
currency-FKP = Falklandeilands pond
currency-GBP = Britse pond
currency-GEL = Georgische lari
currency-GHS = Ghanese cedi
currency-GIP = Gibraltarees pond
currency-GMD = Gambiaanse dalasi
currency-GNF = Guinese frank
currency-GTQ = Guatemalteekse quetzal
currency-GYD = Guyaanse dollar
currency-HKD = Hongkongse dollar
currency-HNL = Hondurese lempira
currency-HTG = Haïtiaanse gourde
currency-HUF = Hongaarse forint
currency-IDR = Indonesische roepia
currency-ILS = Israëlische nieuwe shekel
currency-INR = Indiase roepie
currency-IQD = Iraakse dinar
currency-IRR = Iraanse rial
currency-ISK = IJslandse kroon
currency-JMD = Jamaicaanse dollar
currency-JOD = Jordaanse dinar
currency-JPY = Japanse yen
currency-KES = Keniaanse shilling
currency-KGS = Kirgizische som
currency-KHR = Cambodjaanse riel
currency-KMF = Comorese frank
currency-KPW = Noord-Koreaanse won
currency-KRW = Zuid-Koreaanse won
currency-KWD = Koeweitse dinar
currency-KYD = Kaaimaneilandse dollar
currency-KZT = Kazachse tenge
currency-LAK = Laotiaanse kip
currency-LBP = Libanees pond
currency-LKR = Sri Lankaanse roepie
currency-LRD = Liberiaanse dollar
currency-LSL = Lesothaanse loti
currency-LYD = Libische dinar
currency-MAD = Marokkaanse dirham
currency-MDL = Moldavische leu
currency-MGA = Malagassische ariary
currency-MKD = Macedonische denar
currency-MMK = Myanmarese kyat
currency-MNT = Mongoolse tugrik
currency-MOP = Macause pataca
currency-MRU = Mauritaanse ouguiya
currency-MUR = Mauritiaanse roepie
currency-MVR = Maldivische rufiyaa
currency-MWK = Malawische kwacha
currency-MXN = Mexicaanse peso
currency-MYR = Maleisische ringgit
currency-MZN = Mozambikaanse metical
currency-NAD = Namibische dollar
currency-NGN = Nigeriaanse naira
currency-NIO = Nicaraguaanse córdoba
currency-NOK = Noorse kroon
currency-NPR = Nepalese roepie
currency-NZD = Nieuw-Zeelandse dollar
currency-OMR = Omaanse rial
currency-PAB = Panamese balboa
currency-PEN = Peruaanse sol
currency-PGK = Papoea-Nieuw-Guinese kina
currency-PHP = Filipijnse peso
currency-PKR = Pakistaanse roepie
currency-PLN = Poolse zloty
currency-PYG = Paraguayaanse guarani
currency-QAR = Qatarese rial
currency-RON = Roemeense leu
currency-RSD = Servische dinar
currency-RUB = Russische roebel
currency-RWF = Rwandese frank
currency-SAR = Saoedi-Arabische riyal
currency-SBD = Salomon-dollar
currency-SCR = Seychelse roepie
currency-SDG = Soedanees pond
currency-SEK = Zweedse kroon
currency-SGD = Singaporese dollar
currency-SHP = Sint-Heleens pond
currency-SLE = Sierra Leoonse leone
currency-SOS = Somalische shilling
currency-SRD = Surinaamse dollar
currency-SSP = Zuid-Soedanees pond
currency-STN = Santomese dobra
currency-SVC = Salvadoraanse colón
currency-SYP = Syrisch pond
currency-SZL = Swazische lilangeni
currency-THB = Thaise baht
currency-TJS = Tadzjiekse somoni
currency-TMT = Turkmeense manat
currency-TND = Tunesische dinar
currency-TOP = Tongaanse paʻanga
currency-TRY = Turkse lira
currency-TTD = Trinidad en Tobago-dollar
currency-TWD = Nieuwe Taiwanese dollar
currency-TZS = Tanzaniaanse shilling
currency-UAH = Oekraïense hryvnia
currency-UGX = Oegandese shilling
currency-USD = Amerikaanse dollar
currency-UYU = Uruguayaanse peso
currency-UZS = Oezbeekse sum
currency-VES = Venezolaanse bolivar
currency-VND = Vietnamese dong
currency-VUV = Vanuatuaanse vatu
currency-WST = Samoaanse tala
currency-XAF = CFA-frank
currency-XCD = Oost-Caribische dollar
currency-XCG = Caribische gulden
currency-XOF = CFA-franc BCEAO
currency-XPF = CFP-frank
currency-YER = Jemenitische rial
currency-ZAR = Zuid-Afrikaanse rand
currency-ZMW = Zambiaanse kwacha
currency-ZWG = Zimbabwe Gold
