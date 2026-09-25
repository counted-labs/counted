# Nederlands. Volledig behalve de juridische teksten (legal-, terms-, privacy-), die alleen in het
# Engels en Frans bestaan en per bericht op en.ftl terugvallen.

### Common

loading = Laden…
cancel = Annuleren
retry = Opnieuw proberen
delete = Verwijderen
back = Terug
language = Taal

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
error-generic = Er is iets misgegaan. Probeer het opnieuw.

error-invalid-email = Dit e-mailadres is niet geldig.
error-invalid-password = Dit wachtwoord is niet geldig.
error-password-too-short = Het wachtwoord moet minstens 8 tekens bevatten.
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
login-resend = Verificatielink opnieuw sturen
login-resending = Bezig met verzenden…
login-resend-sent = E-mail verzonden - kijk in je inbox.

register-submit = Account aanmaken
register-submitting = Bezig met aanmaken…
register-have-account = Heb je al een account?
register-password-placeholder = Minstens 8 tekens
register-password-warning = Schrijf je wachtwoord op. Vergeet je het, dan kan je account niet worden hersteld.
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
settings-about = Over
settings-anonymous-title = Je bent niet ingelogd
settings-upsell-title = Je projecten, op al je apparaten
settings-upsell-free = Gratis
settings-upsell-body = Counted werkt zonder account. Met een gratis account volgen je projecten en voorkeuren je op je telefoon, je laptop en het web - nog steeds versleuteld, nog steeds onleesbaar voor ons.
settings-locked-badge = Account
settings-locked-friends = Maak een account om vrienden toe te voegen en ze vanuit de app in een project uit te nodigen - zonder link om door te sturen.
settings-locked-payment-methods = Sla je IBAN of betaalapp één keer op en deel ze met de projecten die je kiest. Wie je geld schuldig is, ziet ze naast je naam.
settings-friends-hint = Voeg vrienden toe en nodig ze uit in je projecten zonder een link te delen.

account-member-since = Lid sinds
account-logout = Uitloggen
account-logging-out = Bezig met uitloggen…
account-delete-title = Mijn account verwijderen
account-delete-warning = Direct en definitief, zonder prullenbak. Uitgaven die je in een gedeeld project hebt ingevoerd blijven zichtbaar voor de andere deelnemers - die horen bij hun administratie.
account-delete-confirm-title = Account verwijderen
account-delete-confirm-message = Je account, je sessies en je projectenlijst worden definitief verwijderd. Zonder je wachtwoord worden de versleutelde gegevens van een gedeeld project onleesbaar voor je - dit kan niet ongedaan worden gemaakt.

settings-payment-methods = Betaalgegevens
settings-payment-methods-hint = Hoe je terugbetaald wilt worden. Versleuteld met je account.
payment-method-kind = Methode
payment-method-kind-other = Anders
payment-method-label = Naam
payment-method-label-placeholder = Hoofdrekening
payment-method-value = Gegevens
payment-method-value-placeholder = IBAN, telefoonnummer, gebruikersnaam…
payment-method-add = Toevoegen
payment-method-remove = { $name } verwijderen
payment-method-empty = Je hebt nog geen betaalgegevens toegevoegd.
payment-method-deleted = Betaalmethode verwijderd.
payment-method-value-required = Vul de gegevens van elke betaalmethode in, of verwijder ze.
payment-method-label-required = Geef je eigen methode een naam.
payment-method-too-long = Dat is te lang - maak het korter.
payment-method-invalid-characters = Verwijder regeleinden of onzichtbare tekens.
payment-method-limit = Je kunt maximaal { $max } betaalmethodes opslaan.
payment-methods-saved = Betaalgegevens opgeslagen.
payment-methods-offline = Je moet online zijn om je betaalgegevens op te slaan.
payment-methods-stale = Je betaalgegevens zijn op een ander apparaat gewijzigd. Ze zijn opnieuw geladen — probeer het opnieuw.
payment-methods-key-missing = Log opnieuw in om je betaalgegevens te beheren.

verify-email-checking = Je e-mailadres wordt geverifieerd…
verify-email-welcome = E-mailadres geverifieerd - welkom bij Counted!
verify-email-back-to-login = Terug naar inloggen

### Project status

project-close = Afsluiten
project-archive = Archiveren
project-reopen = Heropenen
project-unarchive = Uit archief halen

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
projects-empty-hint = Maak een project aan met de knop hieronder
projects-offline-banner = Offlinegegevens - maak opnieuw verbinding om te vernieuwen.
projects-no-local-data = Geen lokale gegevens
projects-no-local-data-hint = Log in om je projecten voor het eerst te laden.
projects-add = Project toevoegen
projects-create = Project aanmaken
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
add-project-remove-participant = Deelnemer verwijderen
add-project-me-badge = Ik
add-project-thats-me = Dat ben ik!
add-project-offline = Offline kun je geen project aanmaken. Maak opnieuw verbinding en probeer het nog eens.
add-project-name-required = Het project heeft een naam nodig.
add-project-need-two-participants = Voeg minstens 2 deelnemers toe.
add-project-pick-yourself = Geef aan welke deelnemer jij bent.

join-link-label = Deellink
join-link-hint = De link bevat de ontsleutelsleutel - kopieer hem volledig.
join-invalid-link = Deze link is niet geldig. Plak de volledige deellink, inclusief het deel na de #.
join-wrong-project = Deze link hoort bij een ander project.

import-tricount-link-label = Tricount-link of -sleutel
import-tricount-key-required = Voer een Tricount-link of -sleutel in.
import-tricount-encryption-failed = Versleuteling mislukt.

### Expenses

save = Opslaan
saving = Bezig met opslaan…
adding = Bezig met toevoegen…
link-copied = Link gekopieerd
missing-encryption-key = Versleutelsleutel ontbreekt.
missing-encryption-key-title = Versleutelsleutel ontbreekt
missing-encryption-key-hint = De link die je gebruikte bevat niet de sleutel om dit project te ontsleutelen. Gebruik de volledige link van degene die het heeft aangemaakt.
project-locked-hint = Dit apparaat heeft de sleutel van dit project niet. Open de deellink om het te ontgrendelen.
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
participants-deselect-all = Selectie opheffen
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
reimbursements-empty-hint = Hier verschijnen afrekenvoorstellen zodra de rekeningen niet kloppen
reimbursement-owes = { $debtor } is { $creditor } verschuldigd

user-selection-title = Welke deelnemer ben jij?
user-selection-hint = Kies je naam uit de lijst.
user-selection-required = Selecteer een deelnemer.

edit-project-deferred-new-members = het toevoegen van nieuwe deelnemers
edit-project-deferred-removals = het verwijderen van deelnemers
edit-project-deferred-me = de keuze ‘Dat ben ik’
edit-project-offline-deferred = Offline: { $items } wordt toegepast zodra je weer verbinding hebt.

export-saved = Bestand opgeslagen:
    { $path }
export-failed = Exporteren mislukt: { $reason }

history-expense-added = Uitgave toegevoegd: { $name }
history-expense-edited = Uitgave bewerkt: { $name }
history-expense-deleted = Uitgave verwijderd: { $name }
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
expense-inconsistent-detail = Bedragen kloppen niet: { $paid } betaald, { $owed } verschuldigd, voor een uitgave van { $total }. Bewerk de uitgave om dit te corrigeren.
missing-access-key = Toegangssleutel ontbreekt. Open dit project via de deellink.
filter-all = Alles
filter-my-payments = Mijn betalingen
filter-my-debts = Wat ik verschuldigd ben
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
charts-tab-per-person = Per persoon
charts-tab-trends = Trends
charts-by-category = Verdeling per categorie
charts-per-person = Uitgaven per persoon
charts-categories-by-month = Categorieën per maand
charts-total-spent = Totaal uitgegeven
charts-avg-per-person = Gem. per persoon
charts-expense-count =
    { $count ->
        [one] { $count } uitgave
       *[other] { $count } uitgaven
    }
charts-clear-category-filter = Categoriefilter wissen
charts-no-expenses = Geen uitgaven.
charts-pick-a-project = Kies een project om de uitgaven per persoon te zien.
charts-nothing-to-show = Niets te tonen
charts-my-share-note = Deze bedragen zijn jouw aandeel in elke uitgave.
charts-my-share-skipped =
    { $count ->
        [one] 1 project telt niet mee — geen deelnemer gekozen, of de gegevens zijn niet geladen.
       *[other] { $count } projecten tellen niet mee — geen deelnemer gekozen, of de gegevens zijn niet geladen.
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
charts-person = Persoon
history-empty = Geen gebeurtenissen
not-found-hint = Deze pagina bestaat niet of is verplaatst.
payers-title-paid-by = Betaald door
payers-title-sender = Afzender
payers-title-contributors = Bijdragers
debtors-title-debtors = Verschuldigd
debtors-title-recipients = Ontvangers
debtors-title-beneficiaries = Begunstigden

### Welcome

welcome-title = Jouw afrekening gaat niemand anders aan.
welcome-subtitle = Deel uitgaven met vrienden.
welcome-e2ee-title = Alles versleuteld
welcome-e2ee-body = Namen, bedragen, projecten: alles wordt op je apparaat versleuteld. Alleen jij hebt de sleutel. Niemand kan je rekeningen lezen. Zelfs wij niet.
welcome-e2ee-note = Onleesbaar, ook voor ons (geen servertoegang)
welcome-eu-title = 100 % Europees
welcome-eu-body = Servers in Duitsland, e-mail verstuurd vanuit Frankrijk. Je gegevens verlaten de Europese Unie nooit.
welcome-noads-title = Geen advertenties. Geen trackers.
welcome-noads-body = We verzamelen niets en verkopen je gegevens niet. Dat is ons model niet.
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
help-forgot-password-warning = Je gegevens zijn dan definitief verloren.
help-forgot-password-a = De sleutel wordt afgeleid van je wachtwoord, dus resetten kan niet: niemand - wij ook niet - kan je projecten zonder dat wachtwoord ontsleutelen. Bewaar het goed, het liefst in een wachtwoordmanager.
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
update-required-body-testflight = Deze versie van Counted is te oud om met de server te communiceren. Open TestFlight en installeer de nieuwste versie om verder te gaan.
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

### Payment methods (delen)

settings-payment-methods-share-warning = Een gedeelde betaalmethode is zichtbaar voor alle leden van de projecten waarin je je naam hebt gekozen - iedereen die een van die projectlinks heeft.
payment-method-share = Delen met mijn projecten
payment-method-share-hint = Wordt naast je naam getoond wanneer iemand je geld schuldig is.
payment-method-copy = { $name } kopiëren
payment-method-copied = Gekopieerd.
payment-method-copy-failed = Kopiëren mislukt - selecteer de tekst en kopieer hem met de hand.

### Expenses (valuta, bedrag)

expense-category-auto = Auto · { $emoji }
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
project-currency-hint = Alle bedragen worden in deze valuta getoond. Dit kan later niet worden gewijzigd.
project-currency-locked = De valuta wordt vastgelegd bij het aanmaken van het project.
project-gone-title = Dit project bestaat niet meer
participants-by-shares = Op aandelen
split-amounts = Bedragen

### Reimbursements

reimbursements-empty-title = Alles is vereffend!
reimbursement-record = Vereffenen
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
charts-whole-project = Hele project
charts-date-from = Van
charts-date-to = Tot
charts-total = Totaal
charts-payments-per-person-by-month = Betalingen per persoon per maand
history-by = Door { $name }

### Notifications

notifications-label = Meldingen
notifications-title = Meldingen
notifications-empty = Niets nieuws
notifications-friend-request = Vriendschapsverzoek

### Friends

friends-title = Vrienden
friends-anonymous-body = Vrienden horen bij je account. Log in om mensen toe te voegen en ze zonder link in je projecten uit te nodigen.
friends-add-title = Een vriend toevoegen
friends-add-hint = Je verzoek verschijnt zodra die persoon inlogt. Geen van beiden hoort of de ander een account heeft totdat het verzoek is geaccepteerd.
friends-add-button = Toevoegen
friends-add-from-project = Als vriend toevoegen
friends-request-sent = Verzoek verstuurd
friends-no-account-key = Log opnieuw in om je vrienden op dit apparaat te beheren.
friends-incoming-title = Verzoeken
friends-accept = Accepteren
friends-decline = Weigeren
friends-list-title = Mijn vrienden
friends-list-empty = Nog geen vrienden. Voeg hierboven iemand toe via e-mail, of vanuit een project dat jullie delen.
friends-remove = Verwijderen
friends-remove-confirm-title = Vriend verwijderen
friends-remove-confirm-message = { $email } staat dan niet meer in je vrienden, en jij niet meer in die van hen. Elk van jullie kan later een nieuw verzoek sturen.
friends-no-key = Nog niet klaar
friends-fingerprint = Veiligheidscode
friends-fingerprint-hint = Twee vrienden die elkaar dezelfde veiligheidscode voorlezen weten dat er niemand tussen hen zit - zelfs onze server niet.
friends-outgoing-title = Verstuurd
friends-outgoing-hint = Wacht op antwoord. Ze verschijnen bij je vrienden zodra ze accepteren.
friends-withdraw = Annuleren
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
