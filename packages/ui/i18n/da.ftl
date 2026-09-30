# Dansk. Komplet bortset fra de juridiske tekster (legal-, terms-, privacy-), som kun findes på
# engelsk og fransk og falder tilbage på en.ftl besked for besked.

### Common

loading = Indlæser…
cancel = Annuller
confirm = Bekræft
retry = Prøv igen
delete = Slet
back = Tilbage
language = Sprog

### Navigation

nav-main = Hovednavigation
nav-projects = Projekter
nav-charts = Statistik
nav-settings = Indstillinger

### Connectivity

offline-banner = Offline
offline-pending =
    { $count ->
        [one] { $count } afventer
       *[other] { $count } afventer
    }

sync-conflict-edit = Konflikt: redigering af ”{ $name }” mislykkedes (elementet er slettet). Sprunget over.
sync-conflict-delete = Konflikt: sletning af ”{ $name }” mislykkedes (elementet er slettet). Sprunget over.
sync-conflict-other = Konflikt: handlingen på ”{ $name }” mislykkedes (elementet er slettet). Sprunget over.
sync-error = Synkroniseringsfejl: { $reason }

### Errors

error-network = Kan ikke nå serveren. Tjek din internetforbindelse.
error-generic = Noget gik galt. Prøv igen.

error-invalid-email = Den e-mailadresse er ikke gyldig.
error-invalid-password = Den adgangskode er ikke gyldig.
error-password-too-short = Adgangskoden skal være mindst 8 tegn.
error-client-outdated = Denne version af appen er forældet. Opdater den for at logge ind.
error-invalid-link = Dette link er ikke gyldigt.
error-batch-too-large = For mange elementer på én gang.
error-payers-required = Vælg mindst én betaler.
error-debtors-required = Vælg mindst én person, der skylder.
error-duplicate-participant = En deltager optræder to gange på samme side.
error-participant-not-in-project = Den deltager er ikke med i projektet.
error-too-many-participants = For mange deltagere til én udgift.
error-invalid-credentials = Forkert e-mail eller adgangskode.
error-unauthenticated = Log ind for at gøre det.
error-email-not-verified = Din e-mailadresse er ikke bekræftet endnu.
error-project-not-found = Dette projekt findes ikke længere.
error-expense-not-found = Denne udgift findes ikke længere.
error-storage-full = Lageret er fuldt: nøglen til dette projekt kunne ikke gemmes på denne enhed. Gem delingslinket.
error-user-not-found = Denne deltager findes ikke længere.
error-tricount-not-found = Tricount blev ikke fundet, eller dets API returnerede en fejl.
error-too-many-members = Projektet har nået sin grænse for medlemmer.
error-identity-taken = En anden konto har allerede gjort krav på denne deltager.
error-claim-proof-invalid = Denne enhed har ikke projektnøglen og kan derfor ikke gøre krav på en deltager. Åbn delelinket igen.
error-user-has-payments = Denne deltager har udgifter i projektet og kan ikke fjernes.
error-resend-cooldown = Vent 60 sekunder, før du beder om en ny e-mail.
error-self-friend-request = Du kan ikke tilføje dig selv som ven.
error-not-a-friend = Du kan kun invitere personer fra din venneliste.
error-friend-has-no-key = Denne ven har ikke åbnet den nyeste version af appen endnu. Bed vedkommende logge ind én gang, og prøv så igen.
error-friend-request-not-found = Denne venneanmodning findes ikke længere.
error-invitation-not-found = Denne invitation findes ikke længere.
error-too-many-friend-requests = For mange venneanmodninger lige nu. Prøv igen i morgen.
error-too-many-invitations = For mange afventende invitationer.
error-invalid-kdf-salt = Krypteringsindstillingerne er ikke gyldige. Opdater appen, og prøv igen.
error-mixed-project-batch = Deltagerne er ikke alle i samme projekt.
error-invalid-payload = Denne version af appen sendte data, som serveren ikke accepterer. Opdater den, og prøv igen.
error-invalid-public-key = Din krypteringsnøgle er ikke gyldig. Opdater appen, og prøv igen.
error-payment-methods-stale = Dine betalingsoplysninger blev ændret på en anden enhed. Genindlæs, og prøv igen.

### Auth

field-email = E-mail
field-email-placeholder = dig@eksempel.dk
field-password = Adgangskode
field-name = Navn
field-name-placeholder = Anna Jensen

login-title = Log ind
login-submit = Log ind
login-submitting = Logger ind…
login-password-placeholder = Din adgangskode
login-no-account = Har du ikke en konto endnu?
login-unverified = Din e-mailadresse er ikke bekræftet endnu. Tjek din indbakke, eller send linket igen.
login-resend = Send bekræftelseslinket igen
login-resending = Sender…
login-resend-sent = E-mail sendt - tjek din indbakke.

register-submit = Opret en konto
register-submitting = Opretter…
register-have-account = Har du allerede en konto?
register-password-placeholder = Mindst 8 tegn
register-password-warning = Skriv din adgangskode ned. Glemmer du den, kan kontoen ikke gendannes.
register-check-email-title = Tjek din e-mail
register-email-sent = E-mail sendt
register-email-sent-hint = Klik på linket i din indbakke for at aktivere din konto.
register-not-received-prefix = Fik du den ikke? Tjek din spam-mappe, eller
register-sign-in-link = log ind
register-not-received-suffix = for at sende linket igen.
register-terms-prefix = Ved at oprette en konto accepterer du vores
register-terms-link = brugsvilkår
register-terms-and = og vores
register-privacy-link = privatlivspolitik

settings-title = Indstillinger
settings-preferences = Præferencer
settings-preferences-local = Gemt på denne enhed.
settings-preferences-synced = Synkroniseret med din konto, krypteret.
settings-about = Om
settings-anonymous-title = Du er ikke logget ind
settings-upsell-title = Dine projekter, på alle enheder
settings-upsell-free = Gratis
settings-upsell-body = Counted virker uden en konto. Med en gratis konto følger dine projekter og præferencer med til din telefon, din computer og nettet - stadig krypteret, stadig ulæseligt for os.
settings-locked-badge = Konto
settings-locked-friends = Opret en konto for at tilføje venner og invitere dem til et projekt fra appen - intet link at sende rundt.
settings-locked-payment-methods = Gem dit IBAN eller din betalingsapp én gang og del det med de projekter, du vælger. Den, der skylder dig, ser det ved siden af dit navn.
settings-friends-hint = Tilføj venner og inviter dem til dine projekter uden at dele et link.

account-member-since = Medlem siden
account-logout = Log ud
account-logging-out = Logger ud…
account-delete-title = Slet min konto
account-delete-warning = Øjeblikkeligt og permanent, uden papirkurv. Udgifter, du har indtastet i et delt projekt, forbliver synlige for de andre medlemmer - de er en del af deres regnskab.
account-delete-confirm-title = Slet konto
account-delete-confirm-message = Din konto, dine sessioner og din projektliste slettes permanent. Uden din adgangskode bliver de krypterede data i et delt projekt ulæselige for dig - det kan ikke fortrydes.

settings-payment-methods = Betalingsoplysninger
settings-payment-methods-hint = Hvordan du gerne vil have penge tilbage. Krypteret med din konto.
payment-method-kind = Metode
payment-method-kind-other = Andet
payment-method-label = Navn
payment-method-label-placeholder = Hovedkonto
payment-method-value = Oplysninger
payment-method-value-placeholder = IBAN, telefonnummer, brugernavn…
payment-method-add = Tilføj
payment-method-remove = Fjern { $name }
payment-method-empty = Du har ikke tilføjet betalingsoplysninger endnu.
payment-method-deleted = Betalingsmetode slettet.
payment-method-value-required = Udfyld oplysningerne for hver betalingsmetode, eller fjern den.
payment-method-label-required = Giv din egen metode et navn.
payment-method-too-long = Det er for langt - gør det kortere.
payment-method-invalid-characters = Fjern linjeskift eller usynlige tegn.
payment-method-limit = Du kan gemme op til { $max } betalingsmetoder.
payment-methods-saved = Betalingsoplysninger gemt.
payment-methods-offline = Du skal være online for at gemme dine betalingsoplysninger.
payment-methods-stale = Dine betalingsoplysninger blev ændret på en anden enhed. De er genindlæst — prøv igen.
payment-methods-key-missing = Log ind igen for at administrere dine betalingsoplysninger.
settings-payment-methods-share-warning = En delt metode er synlig for alle medlemmer af de projekter, hvor du har valgt dit navn - alle, der har et af de projektlinks.
payment-method-share = Del med mine projekter
payment-method-share-hint = Vises ved siden af dit navn, når nogen skylder dig penge.
payment-method-copy = Kopiér { $name }
payment-method-copied = Kopieret.
payment-method-copy-failed = Kunne ikke kopiere - markér teksten, og kopiér den manuelt.

verify-email-checking = Bekræfter din e-mailadresse…
verify-email-welcome = E-mail bekræftet - velkommen til Counted!
verify-email-back-to-login = Tilbage til login

### Project status

project-close = Luk
project-archive = Arkivér
project-reopen = Genåbn
project-unarchive = Gendan fra arkiv

### Dates

date-long = { $day }. { $month } { $year }

month-1 = januar
month-2 = februar
month-3 = marts
month-4 = april
month-5 = maj
month-6 = juni
month-7 = juli
month-8 = august
month-9 = september
month-10 = oktober
month-11 = november
month-12 = december

month-short-1 = jan
month-short-2 = feb
month-short-3 = mar
month-short-4 = apr
month-short-5 = maj
month-short-6 = jun
month-short-7 = jul
month-short-8 = aug
month-short-9 = sep
month-short-10 = okt
month-short-11 = nov
month-short-12 = dec

### Actions

add = Tilføj
create = Opret
creating = Opretter…
edit = Rediger
leave = Forlad
close = Luk
paste = Indsæt
join = Deltag
import = Importér
importing = Importerer…
field-description = Beskrivelse
field-date = Dato
date-today = I dag
date-yesterday = I går
field-optional = Valgfrit

### Projects

projects-filter-active = Aktive
projects-filter-all = Alle
projects-count-label = Projekter
projects-empty = Ingen projekter
projects-empty-hint = Opret et projekt med knappen nedenfor
projects-offline-banner = Offlinedata - opret forbindelse igen for at opdatere.
projects-no-local-data = Ingen lokale data
projects-no-local-data-hint = Log ind for at indlæse dine projekter for første gang.
projects-add = Tilføj et projekt
projects-create = Opret et projekt
projects-join = Deltag i et projekt
projects-import-tricount = Importér fra Tricount
project-actions = Projekthandlinger

status-ongoing = Igangværende
status-closed = Lukket
status-archived = Arkiveret

nav-help = Hjælp
nav-privacy = Privatlivspolitik
nav-terms = Brugsvilkår
nav-legal = Juridisk information

leave-project-title = Forlad projektet?
leave-project-message = Du mister adgangen fra denne enhed. Er der ingen medlemmer tilbage, slettes projektet og alle dets udgifter permanent.

add-project-title = Nyt projekt
add-project-name-label = Projektnavn
add-project-name-placeholder = Min rejse, Bofællesskabet 2024…
add-project-participants = Deltagere
add-project-participant-name = Deltagerens navn
add-project-participant-placeholder = Clark Kent
add-project-offline = Du kan ikke oprette et projekt offline. Opret forbindelse igen, og prøv igen.
add-project-name-required = Projektet skal have et navn.

join-link-label = Delelink
join-link-hint = Linket indeholder dekrypteringsnøglen - kopiér det hele.
join-invalid-link = Det link er ikke gyldigt. Indsæt hele delelinket, inklusive delen efter #.
join-wrong-project = Det link er til et andet projekt.

import-tricount-link-label = Tricount-link eller -nøgle
import-tricount-key-required = Indtast et Tricount-link eller en nøgle.
import-tricount-encryption-failed = Kryptering mislykkedes.
import-tricount-unimportable = Intet blev importeret: denne Tricount har medlemmer med en Tricount-konto eller beløb, der ikke går op (berørte poster: { $count }).

### Expenses

save = Gem
saving = Gemmer…
adding = Tilføjer…
link-copied = Link kopieret
missing-encryption-key = Krypteringsnøgle mangler.
missing-encryption-key-title = Krypteringsnøgle mangler
missing-encryption-key-hint = Det link, du brugte, indeholder ikke den nøgle, der skal til for at dekryptere projektet. Brug hele linket, som den, der oprettede det, delte.
project-locked-hint = Denne enhed har ikke projektets nøgle. Åbn dets delelink for at låse det op.
project-unlock = Lås op
project-no-local-data-hint = Log ind for at indlæse projektets data for første gang.
project-gone-title = Dette projekt findes ikke længere
project-gone-hint = Det blev slettet, da det sidste medlem forlod det. Delelinket virker ikke længere, selv hvis du åbner det igen.

expense-add = Tilføj en udgift
transfer-add = Tilføj en overførsel
expense-edit-title = Rediger udgiften
expense-category = Kategori
expense-category-auto = Auto · { $emoji }
expense-currency = Beløbets valuta
amount-op-add = Plus
amount-op-subtract = Minus
amount-op-multiply = Gange
amount-op-divide = Divideret med
amount-op-equals = Lig med
amount-op-done = Færdig
expense-rate = Vekselkurs (valgfrit)
expense-rate-hint = Lad feltet stå tomt for at bruge Europa-Kommissionens (InforEuro) kurs for { $month }: 1 { $from } = { $rate } { $to }.
expense-rate-invalid = Indtast en vekselkurs større end 0.
expense-rate-unavailable = Ingen automatisk kurs tilgængelig - indtast en manuelt.
expense-delete-title = Slet udgiften
expense-delete-message = ”{ $name }” slettes permanent. Det kan ikke fortrydes.
expense-inconsistent-amounts = Beløbene stemmer ikke
expenses-empty = Ingen udgifter
expenses-empty-hint = Start med at tilføje udgifter med knappen nedenfor
expenses-show-more = Vis flere ({ $count } tilbage)

expense-type-expense = Udgift
expense-type-transfer = Overførsel
expense-type-gain = Indtægt
expense-paid-by = betalt af
expense-sent-by = sendt af
expense-contributed-by = bidraget af

expense-name-required = Et navn er påkrævet.
expense-amount-not-positive = Beløbet skal være større end 0.
expense-no-payer = Vælg mindst én betaler.
expense-no-debtor = Vælg mindst én person, der skylder.
expense-invalid-date = Den dato er ikke gyldig.
expense-payers-mismatch = Betalerne giver tilsammen { $sum }, hvilket ikke matcher udgiftens beløb ({ $total }).
expense-debtors-mismatch = Skyldnerne giver tilsammen { $sum }, hvilket ikke matcher udgiftens beløb ({ $total }).

participants-none = Ingen
participants-everyone = Alle ({ $count })
participants-some = { $count } af { $total }
participants-select-all = Vælg alle
participants-by-shares = Efter andele
split-amounts = Beløb
participants-remaining = { $amount } tilbage
participants-over-by = { $amount } for meget
participants-who-paid = Hvem betalte?
participants-who-received = Hvem modtog?
participants-who-transfers = Hvem overfører?
participants-who-receives = Hvem modtager?
participants-for-whom = Til hvem?

stats-total-expenses = Samlede udgifter
stats-my-expenses = Mine udgifter

tab-expenses = Udgifter
tab-balance = Saldo
tab-reimbursements = Afregn
reimbursements-empty-title = Alt er afregnet!
reimbursements-empty-hint = Forslag til afregning vises her, når regnskabet ikke går op
reimbursement-owes = { $debtor } skylder { $creditor }
reimbursement-record = Afregn
reimbursement-pay-with = Betal
reimbursement-pay-shared-by = Delt af { $name } - tjek modtagernavnet, som din app viser, før du sender.
reimbursement-pay-title = Betal { $name }
reimbursements-mine-title = Du skylder
reimbursements-others-title = Andre tilbagebetalinger
copy = Kopiér

user-selection-title = Hvilken deltager er du?
user-selection-hint = Vælg dit navn på listen.
user-selection-required = Vælg venligst en deltager.
identity-claimed = Knyttet til en konto
identity-claimed-by = { $name }s konto
identity-taken-repick = En anden konto har gjort krav på den deltager, du brugte. Vælg venligst en anden.
participant-gone-repick = Den deltager, du brugte, er fjernet fra projektet. Vælg venligst en anden.

edit-project-title = Rediger projektet
edit-project-new-badge = ny
edit-project-deferred-new-members = tilføjelse af nye medlemmer
edit-project-deferred-removals = fjernelse af medlemmer
edit-project-offline-deferred = Offline: { $items } anvendes, når du opretter forbindelse igen.

export-failed = Eksport mislykkedes: { $reason }

history-expense-added = Udgift tilføjet: { $name }
history-expense-edited = Udgift redigeret: { $name }
history-expense-deleted = Udgift slettet: { $name }
history-project-edited = Projekt redigeret: { $name }
history-name-changed = Navn: ”{ $from }” → ”{ $to }”
history-description-added = Beskrivelse tilføjet: ”{ $value }”
history-description-removed = Beskrivelse fjernet: ”{ $value }”
history-description-changed = Beskrivelse: ”{ $from }” → ”{ $to }”

### Sweep

field-amount = Beløb
expense-name-placeholder = Restaurant, dagligvarer…
expense-actions = Udgiftshandlinger
expense-your-share = Din andel
expense-your-share-value = Din andel: { $amount } { $currency }
expense-inconsistent-detail = Beløbene stemmer ikke: { $paid } betalt, { $owed } skyldigt, for en udgift på { $total }. Rediger udgiften for at rette det.
missing-access-key = Adgangsnøgle mangler. Åbn projektet via dets delelink.
filter-all = Alle
filter-my-payments = Mine betalinger
filter-my-debts = Hvad jeg skylder
participants-shares-for = Andele for { $name }
participants-amount-for = Beløb for { $name }
reimbursement-add = Tilføj en afregning
project-forget = Fjern fra min liste
project-history-title = Historik
history-kind-add = Tilføjet
history-kind-delete = Slettet
history-kind-edit = Redigeret
export = Eksportér
export-json = Eksportér JSON
export-csv = Eksportér CSV
share-link = Del
copy-link-failed = Kunne ikke kopiere linket
open-in-app = Åbn i appen
not-found-title = Siden blev ikke fundet
not-found-back = Tilbage til projekter

### Charts

charts-period = Periode
period-all = Alt
period-month = Måned
period-3months = 3 mdr.
period-year = År
period-custom = Tilpasset
charts-tab-categories = Kategorier
charts-tab-trends = Tendenser
charts-total-spent = Samlet forbrug
charts-avg-per-person = Gns. pr. person
charts-expense-count =
    { $count ->
        [one] { $count } udgift
       *[other] { $count } udgifter
    }
charts-nothing-to-show = Intet at vise
charts-my-share-note = Disse tal er din andel af hver udgift.
charts-my-share-skipped =
    { $count ->
        [one] 1 projekt tælles ikke med — ingen deltager valgt, eller dets data blev ikke indlæst.
       *[other] { $count } projekter tælles ikke med — ingen deltager valgt, eller deres data blev ikke indlæst.
    }

### Categories

category-food = Mad
category-transport = Transport
category-accommodation = Overnatning
category-leisure = Fritid
category-shopping = Shopping
category-services = Tjenester
category-parties-gifts = Fester & gaver
category-other = Andet
charts-project = Projekt
charts-all-projects = Alle projekter
charts-date-from = Fra
charts-date-to = Til
charts-total = I alt
charts-tab-people = Personer
charts-tab-projects = Projekter
charts-scope = Hvis udgifter
charts-scope-group = Gruppe
charts-scope-me = Mig
charts-currency = Valuta
charts-my-share = Min andel
charts-share-of-total = { $pct } % af { $total }
charts-i-paid = Jeg betalte
charts-paid-more = { $amount } mere end din andel
charts-paid-less = { $amount } mindre end din andel
charts-paid-even = Præcis din andel
charts-part-title = Din del af hver kategori
charts-part-desc = Gråt er, hvad gruppen brugte, farve er, hvad du forbrugte.
charts-breakdown-title = Fordeling efter kategori
charts-breakdown-desc = Tryk på et stykke eller en række for at se udgifterne.
charts-of-total = { $amount } af { $total }
charts-show-all = Vis alle ({ $count })
charts-show-less = Vis færre
charts-spend-title = Forbrug over tid
charts-spend-desc = Korte perioder vises pr. dag, længere pr. uge eller måned.
charts-group-by = Gruppér efter
bucket-day = Dag
bucket-week = Uge
bucket-month = Måned
charts-avg = gns.
charts-cat-title-day = { $category }, dag for dag
charts-cat-title-week = { $category }, uge for uge
charts-cat-title-month = { $category }, måned for måned
charts-cat-desc = Vælg en kategori for at følge den over tid.
charts-running-title = Løbende total
charts-running-desc = Siden { $date }.
charts-avg-per-day = { $amount } / dag i snit
charts-avg-per-week = { $amount } / uge i snit
charts-avg-per-month = { $amount } / måned i snit
charts-people-title = Hvem bar gruppen
charts-people-desc = Hvad hver person betalte, ved siden af hvad de forbrugte.
charts-paid = Betalt
charts-fair-share = Rimelig andel
charts-you = (dig)
charts-net-more = betalte mere
charts-net-less = betalte mindre
charts-balance-title = Din saldo over tid
charts-balance-desc = Over linjen skylder gruppen dig. Under den skylder du gruppen.
charts-owed = Du har til gode
charts-owe = Du skylder
charts-projects-title = Din andel pr. projekt
charts-projects-desc = Totaler holdes pr. valuta og lægges aldrig sammen.
history-empty = Ingen hændelser
history-by = Af { $name }
not-found-hint = Denne side findes ikke eller er flyttet.
payers-title-paid-by = Betalt af
payers-title-sender = Afsender
payers-title-contributors = Bidragydere
debtors-title-debtors = Skylder
debtors-title-recipients = Modtagere
debtors-title-beneficiaries = Begunstigede

### Welcome

welcome-title = Dit regnskab kommer ikke andre ved.
welcome-subtitle = Del udgifter med venner.
welcome-note = Gratis. Ingen konto nødvendig. Ingen reklamer.
welcome-link-title = Ét link, og alle er med.
welcome-link-body = Ingen behøver at oprette en konto.
welcome-link-account = En konto? Aldrig påkrævet. Den bruges til at finde dine projekter på en anden enhed, invitere venner fra appen og dele dine betalingsoplysninger.
welcome-demo-project = Weekend i Lyon
welcome-private-title = Ingen kan læse dine regnskaber. Ikke engang os.
welcome-private-body = Navne, beløb, projekter: alt krypteres på din enhed. Kun du har nøglen.
welcome-private-names = Navne
welcome-private-amounts = Beløb
welcome-private-projects = Projekter
welcome-scan-title = Tag et billede af kvitteringen.
welcome-scan-body = Beløb, dato og kategori udfyldes af sig selv. Det hele sker på din telefon. Billedet gemmes ikke.
welcome-eu-title = 100 % europæisk
welcome-no-ads = Ingen reklamer
welcome-no-trackers = Ingen trackere
welcome-step = Trin { $current } af { $total }
welcome-next = Næste
welcome-skip = Spring over
welcome-start = Kom i gang
welcome-how-it-works = Hvordan virker det helt præcist?

### Help

help-intro = Et almindeligt spørgsmål? Tryk for at folde svaret ud.
help-create-project-q = Hvordan opretter jeg et projekt?
help-create-project-a = Fra startskærmen trykker du på +-knappen nederst. Giv projektet et navn, vælg valuta, og du er klar.
help-add-participants-q = Hvordan tilføjer jeg deltagere?
help-add-participants-a = Åbn projektet, og tilføj deltagere fra medlemslisten. Hver deltager kan betale for eller skylde for en udgift.
help-share-project-q = Hvordan deler jeg et projekt?
help-share-project-a = Del projektets URL (den i adresselinjen). Alle med linket kan se og redigere projektet.
help-add-expense-q = Hvordan tilføjer jeg en udgift?
help-add-expense-a = I et projekt trykker du på +, indtaster beløbet, angiver hvem der betalte, og hvem det skal deles mellem. Du kan også vælge en anden dato end i dag.
help-types-q = Hvad er forskellen på en udgift, en overførsel og en indtægt?
help-types-expense = - et køb foretaget af én person og delt mellem flere.
help-types-transfer = - en tilbagebetaling fra én person til en anden, uden deling.
help-types-gain = - modtagne penge (en refusion, en gave) til deling mellem flere personer.
help-past-date-q = Kan jeg datere en udgift tilbage i tiden?
help-past-date-a = Ja, datofeltet er frit. Postens oprettelsestidspunkt gemmes separat.
help-who-owes-q = Hvordan regner Counted ud, hvem der skylder hvad?
help-who-owes-a = Counted beregner hver deltagers nettosaldo (hvad de har lagt ud minus hvad de skylder) og foreslår derefter den korteste række af overførsler, der afregner alle.
help-minimal-transfers-q = Hvorfor er antallet af foreslåede overførsler minimalt?
help-minimal-transfers-a = Algoritmen parrer først saldi, der udligner hinanden præcist, og gennemgår derefter resten fra største kreditor til største debitor. Resultatet: færre overførsler for at afregne alt.
help-import-tricount-q = Hvordan importerer jeg et projekt fra Tricount?
help-import-tricount-a = Fra startskærmen trykker du på ”+”-knappen nederst og derefter
help-import-tricount-b = Indsæt delelinket til det Tricount, du vil importere.
help-encryption-q = Er mine data krypteret?
help-encryption-a = Ja. Counted kombinerer to garantier:
help-encryption-e2ee-term = End-to-end-kryptering
help-encryption-e2ee-def = - alt mellem dig og serveren sendes krypteret.
help-encryption-zero-term = Nul adgang
help-encryption-zero-def = - du krypterer dataene, før de sendes, og serveren gemmer kun chiffertekst. Vi har ingen mulighed for at læse den.
help-encryption-see = For detaljer, se
help-forgot-password-q = Hvad sker der, hvis jeg glemmer min adgangskode?
help-forgot-password-warning = Dine data går permanent tabt.
help-forgot-password-a = Krypteringsnøglen udledes af din adgangskode, så ingen nulstilling er mulig: ingen - heller ikke os - kan dekryptere dine projekter uden den. Opbevar den sikkert, helst i en adgangskodemanager.
help-archive-delete-q = Hvordan arkiverer eller sletter jeg et projekt?
help-archive-delete-a = Fra projektskærmen åbner du menuen og vælger
help-archive-delete-b = for at skjule det, men beholde det. Et projekt slettes for altid, når dets sidste medlem forlader det.
help-delete-account-q = Hvordan sletter jeg min konto?
help-delete-account-a = Åbn Indstillinger, og brug ”Slet min konto”. Det sker øjeblikkeligt og kan ikke fortrydes.
help-contact = Et andet spørgsmål? Skriv til os på

# Receipt scanning (mobile only)
expense-scan = Scan en kvittering
scan-in-progress = Læser kvitteringen…
scan-error-capture = Kunne ikke tage det billede. Prøv igen, eller indtast udgiften manuelt.
scan-error-unreadable = Intet læsbart på den kvittering. Indtast udgiften manuelt.
scan-check-amount = Tjek totalen - den var ikke tydeligt trykt.
scan-take-photo = Tag et billede
scan-choose-photo = Vælg et billede
expense-converted-from = Betalt { $amount } { $from } · 1 { $from } = { $rate } { $to }
project-currency = Valuta
project-currency-hint = Alle beløb vises i denne valuta. Den kan ikke ændres senere.
project-currency-locked = Valutaen fastlægges, når projektet oprettes.

update-required-title = Opdatering påkrævet
update-required-body = Denne version af Counted er for gammel til at tale med serveren. Opdater den for at fortsætte med at bruge appen.
update-required-button = Opdater

notifications-label = Notifikationer
notifications-title = Notifikationer
notifications-empty = Intet nyt
notifications-friend-request = Venneanmodning

friends-title = Venner
friends-anonymous-body = Venner gemmes med din konto. Log ind for at tilføje personer og invitere dem til dine projekter uden at dele et link.
friends-add-title = Tilføj en ven
friends-add-hint = Vedkommende ser din anmodning ved login. Ingen af jer får at vide, om den anden har en konto, før anmodningen er accepteret.
friends-add-button = Tilføj
friends-add-from-project = Tilføj som ven
friends-request-sent = Anmodning sendt
friends-no-account-key = Log ind igen for at administrere dine venner på denne enhed.
friends-incoming-title = Anmodninger
friends-accept = Acceptér
friends-decline = Afvis
friends-list-title = Mine venner
friends-list-empty = Ingen venner endnu. Tilføj nogen via e-mail ovenfor eller fra et projekt, I deler.
friends-remove = Fjern
friends-remove-confirm-title = Fjern ven
friends-remove-confirm-message = { $email } vil ikke længere være blandt dine venner, og du ikke blandt deres. Hver af jer kan sende en ny anmodning senere.
friends-no-key = Ikke klar endnu
friends-fingerprint = Sikkerhedskode
friends-fingerprint-hint = To venner, der læser den samme sikkerhedskode op for hinanden, ved, at ingen sidder imellem dem - ikke engang vores server.
friends-outgoing-title = Sendt
friends-outgoing-hint = Venter på svar. Du ser dem blandt dine venner, når de accepterer.
friends-withdraw = Annuller
invite-friends-title = Inviter venner
invite-friends-hint = Projektnøglen krypteres for hver ven på denne enhed. Serveren ser den aldrig.
invite-friends-empty = Ingen venner at invitere endnu.
invite-friends-button = Inviter
invite-sent = { $count ->
    [one] Invitation sendt
   *[other] { $count } invitationer sendt
}
invitation-badge = Invitation
invitation-to = Deltag i ”{ $name }”
invitation-to-unnamed = Deltag i et projekt
invitation-unreadable = Denne invitation kan ikke åbnes på denne enhed
invitation-from = Fra { $email }
invitation-accept = Deltag
invitation-decline = Afvis

# Participants in the create and edit modals, and the "who are you?" picker - see
# docs/plans/friends.md §11.
participants-you-label = Dit navn i dette projekt
participants-you-badge = Dig
participants-you-from-account = Hentet fra dit kontonavn. Ret det her kun for dette projekt.
participants-you-required = Påkrævet. Sådan ser de andre dig.
participants-others = Andre deltagere
participants-empty = Ingen endnu. Vælg en ven nedenfor, eller skriv et navn.
participants-empty-signed-out = Ingen endnu. Skriv et navn for at tilføje nogen.
participants-duplicate = ”{ $name }” er allerede på listen.
participants-input-label = Tilføj en ven, eller skriv et navn
participants-input-placeholder = Ven eller et navn
participants-suggest-friend = Ven · deltager som ”{ $name }”, får en invitation
participants-suggest-not-ready = Ven · ikke klar endnu
participants-suggest-guest = Tilføj ”{ $text }” uden konto
participants-suggest-guest-sub = Ingen konto, kun et navn
participants-friends = Dine venner
participants-all-friends = Alle venner
participants-login-hint = Log ind for at tilføje folk direkte fra din venneliste.
participants-invite-badge = Inviter
participants-guest-badge = Uden konto
participants-guest-sub = Ingen konto, kun et navn
participants-rename = Omdøb { $name }
participants-remove = Fjern { $name }
participants-rename-label = Nyt navn
participants-rename-save = Gem navnet
participants-rename-hint = Navnet, alle ser i dette projekt. Invitationen går stadig til { $email }.
participants-invited-badge = Inviteret
participants-invited-sub = { $email } · ikke accepteret endnu
participants-invited-pending = Invitationen er ikke accepteret endnu
participants-unlinked = Ikke knyttet til en konto
add-project-create-invite = Opret og inviter { $count }
edit-project-save-invite = Gem og inviter { $count }
edit-project-you-are = På denne enhed er du { $name }
edit-project-no-identity = Du har ikke valgt, hvem du er, endnu
edit-project-switch = Skift
edit-project-choose = Vælg
invite-failed = Disse invitationer kunne ikke sendes: { $emails }
invite-again = Inviter igen
friend-picker-title = Tilføj venner
user-selection-invited-hint = { $email } har inviteret dig til ”{ $project }”.
user-selection-suggested = Foreslået
user-selection-suggested-sub = { $email } tilføjede dig under dette navn
user-selection-confirm-as = Jeg er { $name }
user-selection-missing = Er dit navn her ikke? Bed en deltager om at tilføje dig i projektets indstillinger.
