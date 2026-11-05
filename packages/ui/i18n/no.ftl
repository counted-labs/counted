# Norsk (bokmål). Komplett bortsett fra de juridiske tekstene (legal-, terms-, privacy-), som bare
# finnes på engelsk og fransk og faller tilbake på en.ftl melding for melding.

### Common

loading = Laster…
cancel = Avbryt
confirm = Bekreft
retry = Prøv igjen
delete = Slett
back = Tilbake
language = Språk
theme = Tema
theme-light = Lyst
theme-dark = Mørkt
theme-system = System
theme-hint-device = Lagret bare på denne enheten.
theme-hint-system = Følger utseendet til denne enheten.

### Navigation

nav-main = Hovednavigasjon
nav-projects = Prosjekter
nav-charts = Statistikk
nav-settings = Innstillinger

### Connectivity

offline-banner = Frakoblet
offline-pending =
    { $count ->
        [one] { $count } venter
       *[other] { $count } venter
    }

sync-conflict-edit = Konflikt: redigering av «{ $name }» mislyktes (elementet er slettet). Hoppet over.
sync-conflict-delete = Konflikt: sletting av «{ $name }» mislyktes (elementet er slettet). Hoppet over.
sync-conflict-other = Konflikt: handlingen på «{ $name }» mislyktes (elementet er slettet). Hoppet over.
sync-error = Synkroniseringsfeil: { $reason }

### Errors

error-network = Får ikke kontakt med serveren. Sjekk internettforbindelsen din.
error-rate-limited = Serveren er opptatt. Prøv igjen om litt.
error-generic = Noe gikk galt. Prøv igjen.

error-invalid-email = Den e-postadressen er ikke gyldig.
error-invalid-password = Det passordet er ikke gyldig.
error-password-too-short = Passordet må være minst 12 tegn.
error-client-outdated = Denne versjonen av appen er utdatert. Oppdater den for å logge inn.
error-invalid-link = Denne lenken er ikke gyldig.
error-batch-too-large = For mange elementer på én gang.
error-payers-required = Velg minst én betaler.
error-debtors-required = Velg minst én person som skylder.
error-duplicate-participant = En deltaker forekommer to ganger på samme side.
error-participant-not-in-project = Den deltakeren er ikke med i prosjektet.
error-too-many-participants = For mange deltakere for én utgift.
error-invalid-credentials = Feil e-post eller passord.
error-unauthenticated = Logg inn for å gjøre det.
error-email-not-verified = E-postadressen din er ikke bekreftet ennå.
error-project-not-found = Dette prosjektet finnes ikke lenger.
error-expense-not-found = Denne utgiften finnes ikke lenger.
error-storage-full = Lagringen er full: nøkkelen til dette prosjektet kunne ikke lagres på denne enheten. Ta vare på delingslenken.
error-user-not-found = Denne deltakeren finnes ikke lenger.
error-tricount-not-found = Tricount ble ikke funnet, eller API-et returnerte en feil.
error-too-many-members = Prosjektet har nådd grensen for medlemmer.
error-identity-taken = En annen konto har allerede gjort krav på denne deltakeren.
error-claim-proof-invalid = Denne enheten har ikke prosjektnøkkelen og kan derfor ikke gjøre krav på en deltaker. Åpne delingslenken på nytt.
error-user-has-payments = Denne deltakeren har utgifter i prosjektet og kan ikke fjernes.
error-resend-cooldown = Vent 60 sekunder før du ber om en ny e-post.
error-self-friend-request = Du kan ikke legge til deg selv som venn.
error-not-a-friend = Du kan bare invitere personer fra vennelisten din.
error-friend-has-no-key = Denne vennen har ikke åpnet den nyeste versjonen av appen ennå. Be vedkommende logge inn én gang, og prøv igjen.
error-friend-request-not-found = Denne venneforespørselen finnes ikke lenger.
error-invitation-not-found = Denne invitasjonen finnes ikke lenger.
error-too-many-friend-requests = For mange venneforespørsler akkurat nå. Prøv igjen i morgen.
error-too-many-invitations = For mange ventende invitasjoner.
error-invalid-kdf-salt = Krypteringsinnstillingene er ikke gyldige. Oppdater appen og prøv igjen.
error-mixed-project-batch = Disse deltakerne er ikke alle i samme prosjekt.
error-invalid-payload = Denne versjonen av appen sendte data som serveren ikke godtar. Oppdater den og prøv igjen.
error-invalid-public-key = Krypteringsnøkkelen din er ikke gyldig. Oppdater appen og prøv igjen.
error-payment-methods-stale = Betalingsopplysningene dine ble endret på en annen enhet. Last inn på nytt og prøv igjen.

### Auth

field-email = E-post
field-email-placeholder = du@eksempel.no
field-password = Passord
field-name = Navn
field-name-placeholder = Kari Nordmann

login-title = Logg inn
login-submit = Logg inn
login-submitting = Logger inn…
login-password-placeholder = Passordet ditt
login-no-account = Har du ikke konto ennå?
login-unverified = E-postadressen din er ikke bekreftet ennå. Sjekk innboksen, eller send lenken på nytt.
login-maybe-unverified = Har du nettopp registrert deg? Bekreft adressen din først med lenken vi sendte deg, eller send den på nytt.
login-resend = Send bekreftelseslenken på nytt
login-resending = Sender…
login-resend-sent = E-post sendt - sjekk innboksen din.

register-submit = Opprett en konto
register-submitting = Oppretter…
register-have-account = Har du allerede en konto?
register-password-placeholder = Minst 12 tegn
register-password-warning = Skriv ned passordet ditt: vi ser det aldri og kan ikke gi det tilbake til deg.
register-check-email-title = Sjekk e-posten din
register-email-sent = E-post sendt
register-email-sent-hint = Klikk på lenken i innboksen for å aktivere kontoen din.
register-not-received-prefix = Fikk du den ikke? Sjekk søppelposten, eller
register-sign-in-link = logg inn
register-not-received-suffix = for å sende lenken på nytt.
register-terms-prefix = Ved å opprette en konto godtar du våre
register-terms-link = bruksvilkår
register-terms-and = og vår
register-privacy-link = personvernerklæring

settings-title = Innstillinger
settings-preferences = Preferanser
settings-preferences-local = Lagret på denne enheten.
settings-preferences-synced = Synkronisert med kontoen din, kryptert.
settings-about = Om
settings-anonymous-title = Du er ikke logget inn
settings-upsell-title = Prosjektene dine, på alle enheter
settings-upsell-free = Gratis
settings-upsell-body = Counted fungerer uten konto. Med en gratis konto følger prosjektene og preferansene dine med til telefonen, datamaskinen og nettet - fortsatt kryptert, fortsatt uleselig for oss.
settings-locked-badge = Konto
settings-locked-friends = Opprett en konto for å legge til venner og invitere dem til et prosjekt fra appen - ingen lenke å sende rundt.
settings-locked-payment-methods = Lagre IBAN-en eller betalingsappen din én gang og del med prosjektene du velger. Den som skylder deg, ser det ved siden av navnet ditt.
settings-friends-hint = Legg til venner og inviter dem til prosjektene dine uten å dele en lenke.

account-member-since = Medlem siden
account-logout = Logg ut
account-logging-out = Logger ut…
account-delete-title = Slett kontoen min
account-delete-warning = Umiddelbar og permanent sletting av kontoen din og dataene dine.
account-delete-confirm-title = Slett konto
account-delete-confirm-message = Kontoen din, øktene dine og prosjektlisten din slettes permanent. Uten passordet ditt blir de krypterte dataene i et delt prosjekt uleselige for deg - dette kan ikke angres.
erase-device-open = Logg ut og slett denne enheten
erase-device-title = Slette denne enheten?
erase-device-body = Prosjektene, nøklene deres og hurtigbufferen på denne enheten slettes, deretter avsluttes økten din. Prosjektene dine blir værende i kontoen din: logg inn igjen for å få dem tilbake.
erase-device-checking = Sjekker at prosjektene dine er i kontoen din…
erase-device-pending = Endringer som venter på å bli sendt: { $count }. Koble til internett igjen, og prøv på nytt.
erase-device-unreachable = Kunne ikke sjekke at prosjektene dine er i kontoen din. Sjekk tilkoblingen, og prøv på nytt.
erase-device-at-risk = Disse prosjektene er ikke i kontoen din og går tapt. Vil du beholde dem, avbryt og kopier lenken deres først:
erase-device-confirm = Slett og logg ut
erase-device-confirm-anyway = Slett likevel
erase-device-erasing = Sletter…

settings-payment-methods = Betalingsopplysninger
settings-payment-methods-hint = Hvordan du vil få penger tilbake. Kryptert med kontoen din.
payment-method-kind = Metode
payment-method-kind-other = Annet
payment-method-label = Navn
payment-method-label-placeholder = Hovedkonto
payment-method-value = Opplysninger
payment-method-value-placeholder = IBAN, telefonnummer, brukernavn…
payment-method-add = Legg til
payment-method-remove = Fjern { $name }
payment-method-empty = Du har ikke lagt til betalingsopplysninger ennå.
payment-method-deleted = Betalingsmetode slettet.
payment-method-value-required = Fyll inn opplysningene for hver betalingsmetode, eller fjern den.
payment-method-label-required = Gi den egendefinerte metoden et navn.
payment-method-too-long = Det er for langt - kort det ned.
payment-method-invalid-characters = Fjern linjeskift eller usynlige tegn.
payment-method-limit = Du kan lagre opptil { $max } betalingsmetoder.
payment-methods-saved = Betalingsopplysninger lagret.
payment-methods-offline = Du må være tilkoblet for å lagre betalingsopplysningene dine.
payment-methods-stale = Betalingsopplysningene dine ble endret på en annen enhet. De er lastet inn på nytt - prøv igjen.
payment-methods-key-missing = Logg inn på nytt for å administrere betalingsopplysningene dine.
settings-payment-methods-share-warning = En delt metode er synlig for alle medlemmer av prosjektene dine.
payment-method-share = Del med prosjektene mine
payment-method-share-hint = Vises ved siden av navnet ditt når noen skylder deg penger.
payment-method-copy = Kopier { $name }
payment-method-copied = Kopiert.
payment-method-copy-failed = Kunne ikke kopiere - marker teksten og kopier den manuelt.

verify-email-checking = Bekrefter e-postadressen din…
verify-email-welcome = E-post bekreftet - velkommen til Counted!
verify-email-back-to-login = Tilbake til innlogging
verify-email-intro = Skriv inn passordet ditt for å bekrefte at adressen er din.
verify-email-submit = Bekreft adressen min

### Project status

project-archive = Arkiver
project-sheet-invite = Inviter
project-sheet-recurring = Gjentakende
project-sheet-edit = Rediger prosjekt
project-sheet-close = Lukk prosjekt
project-sheet-archive = Arkiver prosjekt
project-sheet-reopen = Gjenåpne prosjekt
project-sheet-unarchive = Gjenopprett prosjekt fra arkiv
project-sheet-leave = Forlat prosjekt

### Dates

date-long = { $day }. { $month } { $year }

month-1 = januar
month-2 = februar
month-3 = mars
month-4 = april
month-5 = mai
month-6 = juni
month-7 = juli
month-8 = august
month-9 = september
month-10 = oktober
month-11 = november
month-12 = desember

month-short-1 = jan
month-short-2 = feb
month-short-3 = mar
month-short-4 = apr
month-short-5 = mai
month-short-6 = jun
month-short-7 = jul
month-short-8 = aug
month-short-9 = sep
month-short-10 = okt
month-short-11 = nov
month-short-12 = des

### Actions

add = Legg til
create = Opprett
creating = Oppretter…
edit = Rediger
leave = Forlat
close = Lukk
paste = Lim inn
join = Bli med
import = Importer
importing = Importerer…
field-description = Beskrivelse
field-date = Dato
date-today = I dag
date-yesterday = I går
field-optional = Valgfritt

### Projects

projects-filter-active = Aktive
projects-filter-all = Alle
projects-count-label = Prosjekter
projects-empty = Ingen prosjekter
projects-offline-banner = Frakoblede data - koble til igjen for å oppdatere.
demo-banner = Demoprosjekt - skrivebeskyttet.
demo-start-own = Opprett ditt eget prosjekt
demo-badge = Demo
demo-bar = Endringene dine forsvinner når du lukker denne fanen.
demo-leave = Kom ordentlig i gang
demo-preparing = Forbereder demoen din…
demo-failed = Demoen kunne ikke forberedes.
demo-sign-in-blocked = Innlogging er slått av i demoen. Gå ut av den for å logge inn eller opprette en konto; dine egne prosjekter er urørt.
projects-no-local-data = Ingen lokale data
projects-no-local-data-hint = Logg inn for å laste prosjektene dine for første gang.
projects-add = Legg til et prosjekt
projects-create = Opprett et prosjekt
projects-personal-expense = Personlig utgift
personal-expense-banner = Lagt til i Mine utgifter
personal-create-title = Opprett ditt personlige prosjekt
personal-create-hint = Følg med på hva du bruker alene
group-create-title = Opprett et delt prosjekt
group-create-hint = Del utgifter med venner og familie
personal-create-action = Opprett
personal-create-signup = Opprett en konto
personal-sheet-title = Ditt personlige prosjekt
personal-sheet-intro = For det du bruker alene. Det vises sammen med gruppene dine i Statistikk.
personal-sheet-point-alone = Ett medlem: deg, med kontonavnet ditt. Ingen delingslenke.
personal-sheet-point-encrypted = Kryptert på enhetene dine, som de andre prosjektene dine.
personal-sheet-point-stats = Dine andeler i grupper legges til disse utgiftene i Statistikk.
personal-default-name = Mine utgifter
my-spending-prev-month = Forrige måned
my-spending-next-month = Neste måned
my-spending-spent = Du har brukt
my-spending-groups = Grupper
my-spending-income = Inntekter
my-spending-balance = Månedens saldo
my-spending-unconverted = Andeler i en annen valuta holdes utenfor til en kurs er kjent.
my-spending-empty = Ingenting denne måneden
my-spending-empty-hint = Legg til en utgift med +. Andelene dine i grupper vises her automatisk.
my-spending-share-of = av { $amount }
my-spending-uncounted = ikke talt med
payer-you = deg
expense-received-by = mottatt av
counted-switch = Tell med i Mine utgifter
counted-switch-on = På: andelen din legges til Mine utgifter
counted-switch-off = Av: andelen din vises ikke i Mine utgifter
counted-groups = Grupper som telles med
counted-groups-intro = Andelen din i disse gruppene legges til Mine utgifter og budsjettene.
counted-groups-converted = { $from }, omregnet til { $to }
counted-groups-encrypted = Dette valget er kryptert med kontoen din og følger enhetene dine.
counted-groups-empty = Ingen grupper på denne enheten ennå.
counted-groups-done = OK
counted-save-failed = Valget kunne ikke lagres. Prøv igjen.
tab-budget = Budsjett
budget-title = Budsjetter
budget-manage = Administrer
budget-left = { $amount } igjen
budget-over = Overskredet med { $amount }
budget-empty = Ingen budsjett
budget-empty-hint = Sett et tak per kategori. Andelene dine i grupper teller også.
budget-define = Sett et budsjett
budget-sheet-title = Månedlige budsjetter
budget-sheet-intro = På din andel: personlige og grupper. La stå tomt for ikke å følge en kategori.
budget-average = Snitt siste 3 måneder: { $amount }
budget-for = Budsjett { $category }
budget-alert = Varsle meg ved 80 %
budget-alert-hint = Beregnet på denne enheten
budget-invalid = Ugyldig beløp for { $category }.
budget-offline = Budsjetter kan bare lagres på nett. Koble til igjen og prøv på nytt.
budget-alert-near = { $category }: { $percent } % av månedens budsjett
budget-alert-over = { $category }: månedens budsjett er overskredet
personal-card-budget = { $month } · budsjett { $amount }
my-spending-search-summary =
    { $count ->
        [one] { $count } resultat
       *[other] { $count } resultater
    } · { $total } · personlige og grupper, alle måneder
projects-join = Bli med i et prosjekt
projects-import-tricount = Importer fra Tricount
project-actions = Prosjekthandlinger

status-ongoing = Pågående
status-closed = Lukket
status-archived = Arkivert

nav-help = Hjelp
nav-privacy = Personvernerklæring
nav-terms = Bruksvilkår
nav-legal = Juridisk informasjon

leave-project-title = Forlate prosjektet?
leave-project-message = Du mister tilgangen fra denne enheten. Er det ingen medlemmer igjen, slettes prosjektet og alle utgiftene permanent.

add-project-title = Nytt prosjekt
add-project-name-label = Prosjektnavn
add-project-name-placeholder = Reisen min, Kollektivet 2024…
add-project-participants = Deltakere
add-project-participant-name = Deltakerens navn
add-project-participant-placeholder = Clark Kent
add-project-offline = Du kan ikke opprette et prosjekt frakoblet. Koble til igjen og prøv på nytt.
add-project-name-required = Prosjektet trenger et navn.

join-link-label = Delingslenke
join-link-hint = Lenken inneholder dekrypteringsnøkkelen - kopier hele.
join-invalid-link = Den lenken er ikke gyldig. Lim inn hele delingslenken, inkludert delen etter #.
join-wrong-project = Den lenken gjelder et annet prosjekt.

import-tricount-link-label = Tricount-lenke eller -nøkkel
import-tricount-key-required = Skriv inn en Tricount-lenke eller -nøkkel.
import-tricount-encryption-failed = Kryptering mislyktes.
import-tricount-unimportable = Ingenting ble importert: denne Tricounten har medlemmer med Tricount-konto eller beløp som ikke går opp (berørte oppføringer: { $count }).

### Expenses

save = Lagre
saving = Lagrer…
adding = Legger til…
link-copied = Lenke kopiert
missing-encryption-key = Krypteringsnøkkel mangler.
missing-encryption-key-title = Krypteringsnøkkel mangler
missing-encryption-key-hint = Lenken du brukte, inneholder ikke nøkkelen som trengs for å dekryptere prosjektet. Bruk hele lenken som den som opprettet det, delte.
project-locked-hint = Denne enheten har ikke nøkkelen til dette prosjektet. Åpne delingslenken for å låse det opp.
expenses-unreadable = Noen oppføringer kan ikke leses på denne enheten og er ikke med i summene. Oppdater appen for å se dem.
project-unlock = Lås opp
project-no-local-data-hint = Logg inn for å laste prosjektets data for første gang.
project-gone-title = Dette prosjektet finnes ikke lenger
project-gone-hint = Det ble slettet da det siste medlemmet forlot det. Delingslenken virker ikke lenger, selv om du åpner den igjen.

expense-add = Legg til en utgift
transfer-add = Legg til en overføring
expense-edit-title = Rediger utgiften
expense-category = Kategori
expense-category-auto = Auto · { $emoji }
emoji-picker-label = Emoji
emoji-none = Ingen emoji
expense-currency = Beløpets valuta
amount-op-add = Pluss
amount-op-subtract = Minus
amount-op-multiply = Ganger
amount-op-divide = Delt på
amount-op-equals = Er lik
amount-op-done = Ferdig
expense-rate = Valutakurs (valgfritt)
expense-rate-hint = La stå tomt for å bruke Europakommisjonens (InforEuro) kurs for { $month }: 1 { $from } = { $rate } { $to }.
expense-rate-invalid = Skriv inn en valutakurs større enn 0.
expense-rate-unavailable = Ingen automatisk kurs tilgjengelig - skriv inn en manuelt.
expense-delete-title = Slett utgiften
expense-delete-message = «{ $name }» slettes permanent. Dette kan ikke angres.
expense-inconsistent-amounts = Beløpene stemmer ikke
expenses-empty = Ingen utgifter
expenses-empty-hint = Begynn med å legge til utgifter med knappen nedenfor
expenses-show-more = Vis flere ({ $count } igjen)

expense-type-expense = Utgift
expense-type-transfer = Overføring
expense-type-gain = Inntekt
expense-type-group = Type
expense-paid-by = betalt av
expense-sent-by = sendt av
expense-contributed-by = bidratt av

expense-name-required = Et navn er påkrevd.
expense-amount-not-positive = Beløpet må være større enn 0.
expense-no-payer = Velg minst én betaler.
expense-no-debtor = Velg minst én person som skylder.
expense-invalid-date = Den datoen er ikke gyldig.
expense-payers-mismatch = Betalerne summerer til { $sum }, som ikke stemmer med utgiftens beløp ({ $total }).
expense-debtors-mismatch = Skyldnerne summerer til { $sum }, som ikke stemmer med utgiftens beløp ({ $total }).

participants-none = Ingen
participants-everyone = Alle ({ $count })
participants-some = { $count } av { $total }
participants-select-all = Velg alle
participants-by-shares = Etter andeler
split-amounts = Beløp
participants-remaining = { $amount } igjen
participants-over-by = { $amount } for mye
participants-who-paid = Hvem betalte?
participants-who-received = Hvem mottok?
participants-who-transfers = Hvem overfører?
participants-who-receives = Hvem mottar?
participants-for-whom = For hvem?

stats-total-expenses = Totale utgifter
stats-my-expenses = Mine utgifter

tab-expenses = Utgifter
tab-balance = Saldo
tab-reimbursements = Gjør opp
balance-gets-back = Får tilbake
balance-owes = Skylder
balance-settled = Oppgjort
reimbursements-empty-title = Alt er gjort opp!
reimbursements-empty-hint = Forslag til oppgjør vises her når regnskapet ikke går opp
reimbursement-owes = { $debtor } skylder { $creditor }
reimbursement-record = Gjør opp
reimbursement-pay-with = Betal
reimbursement-pay-shared-by = Delt av { $name } - sjekk mottakernavnet appen din viser før du sender.
reimbursement-pay-title = Betal { $name }
reimbursements-mine-title = Du skylder
reimbursements-others-title = Andre tilbakebetalinger
copy = Kopier

user-selection-title = Hvilken deltaker er du?
user-selection-hint = Velg navnet ditt fra listen.
user-selection-required = Velg en deltaker.
identity-claimed = Knyttet til en konto
identity-claimed-by = { $name } sin konto
identity-taken-repick = En annen konto har gjort krav på deltakeren du brukte. Velg en annen.
participant-gone-repick = Deltakeren du brukte, er fjernet fra prosjektet. Velg en annen.

edit-project-title = Rediger prosjektet
edit-project-new-badge = ny
edit-project-deferred-new-members = tillegg av nye medlemmer
edit-project-deferred-removals = fjerning av medlemmer
edit-project-offline-deferred = Frakoblet: { $items } brukes når du kobler til igjen.

export-failed = Eksport mislyktes: { $reason }

history-expense-added = Utgift lagt til: { $name }
history-expense-edited = Utgift redigert: { $name }
history-expense-deleted = Utgift slettet: { $name }
move-expense = Flytt til et prosjekt
move-expense-title = Flytt til…
move-current = Nåværende
move-converted = { $currency }, omregnet til månedens kurs
move-note-personal = Du velger hvem som betalte og for hvem i neste steg. Utgiften forlater Mine utgifter.
move-note-group = Utgiften forlater { $project } for alle medlemmene, og gruppens saldo beregnes på nytt. Du velger hvem som betalte og for hvem i neste steg.
move-banner = Fra { $from } til { $to }
move-submit = Flytt
move-done = Utgift flyttet til { $project }
move-delete-failed = Utgiften ble lagt til i det andre prosjektet, men ligger fortsatt i { $project }: slett den der.
history-expense-moved = Utgift flyttet til et annet prosjekt: { $name }
personal-badge = Personlig
move-members =
    { $count ->
        [one] { $count } medlem
       *[other] { $count } medlemmer
    }
history-project-edited = Prosjekt redigert: { $name }
history-name-changed = Navn: «{ $from }» → «{ $to }»
history-description-added = Beskrivelse lagt til: «{ $value }»
history-description-removed = Beskrivelse fjernet: «{ $value }»
history-description-changed = Beskrivelse: «{ $from }» → «{ $to }»

### Sweep

field-amount = Beløp
expense-name-placeholder = Restaurant, dagligvarer…
expense-actions = Utgiftshandlinger
expense-your-share = Din andel
expense-your-share-value = Din andel: { $amount } { $currency }
expense-share-percent = { $percent } %
expense-added-by = Lagt til av { $name }, { $date }
expense-inconsistent-detail = Beløpene stemmer ikke: { $paid } betalt, { $owed } skyldig, for en utgift på { $total }. Rediger utgiften for å rette det.
missing-access-key = Tilgangsnøkkel mangler. Åpne prosjektet via delingslenken.
filter-all = Alle
filter-my-payments = Mine betalinger
filter-my-debts = Det jeg skylder
expense-filter-group = Filter
expenses-search = Søk i utgifter
expenses-no-match = Ingen utgifter samsvarer.
expenses-search-open = Søk
expenses-search-clear = Tøm søk
expenses-search-summary =
    { $count ->
        [one] { $count } resultat
       *[other] { $count } resultater
    } · { $total } · alle måneder
participants-shares-for = Andeler for { $name }
participants-amount-for = Beløp for { $name }
reimbursement-add = Legg til et oppgjør
project-forget = Fjern fra listen min
project-history-title = Historikk
history-kind-add = Lagt til
history-kind-delete = Slettet
history-kind-edit = Redigert
export = Eksporter
export-json = Eksporter JSON
export-csv = Eksporter CSV
share-link = Del
copy-link-failed = Kunne ikke kopiere lenken
open-in-app = Åpne i appen
not-found-title = Siden ble ikke funnet
not-found-back = Tilbake til prosjekter

### Charts

charts-period = Periode
period-all = Alt
period-month = Måned
period-3months = 3 mnd
period-year = År
period-custom = Egendefinert
charts-tab-categories = Kategorier
charts-tab-trends = Trender
charts-total-spent = Totalt brukt
charts-avg-per-person = Snitt per person
charts-expense-count =
    { $count ->
        [one] { $count } utgift
       *[other] { $count } utgifter
    }
charts-nothing-to-show = Ingenting å vise
charts-my-share-skipped =
    { $count ->
        [one] 1 prosjekt telles ikke med - ingen deltaker valgt, eller dataene ble ikke lastet.
       *[other] { $count } prosjekter telles ikke med - ingen deltaker valgt, eller dataene ble ikke lastet.
    }

### Categories

category-food = Mat
category-transport = Transport
category-accommodation = Overnatting
category-leisure = Fritid
category-shopping = Shopping
category-services = Tjenester
category-parties-gifts = Fester og gaver
category-other = Annet
categories-manage = Administrer kategorier
categories-title = Kategorier
categories-yours = Dine kategorier
categories-builtin = Standard
categories-add = Legg til en kategori
categories-in-parent = i { $parent }
categories-renamed-from = omdøpt · opprinnelig { $name }
categories-hidden-hint = skjult · utgiftene vises i { $other }
categories-always-shown = alltid vist
categories-hidden = Skjult
categories-new = Ny kategori
categories-edit = Rediger kategori
categories-parent = Telles i
categories-parent-hint = I statistikken for alle prosjektene dine telles { $name } i { $parent }.
categories-create = Opprett
categories-hide = Skjul denne kategorien
categories-hide-hint = Utgiftene vises i { $other } for alle medlemmer.
categories-delete-title = Slette «{ $name }»?
categories-delete-body = De berørte utgiftene ({ $count }) går tilbake til { $parent }. Ingen utgifter slettes.
categories-delete-body-unknown = Utgiftene går tilbake til { $parent }. Ingen utgifter slettes.
categories-delete-warning = Kategorien forsvinner for alle medlemmene i prosjektet.
categories-summary-created = Opprettet: { $count }
categories-summary-renamed = Omdøpt: { $count }
categories-summary-hidden = Skjult: { $count }
categories-summary-default = Ingen egne kategorier
categories-offline = Kategorier kan bare administreres på nett. Koble til igjen og prøv på nytt.
categories-error-too-many = Et prosjekt har maks 20 egne kategorier.
categories-error-full = Prosjektet har nådd grensen for kategorier.
categories-error-name = Gi kategorien et navn.
categories-error-missing = Kategorien er endret i mellomtiden. Lukk og prøv på nytt.
history-category-created = Kategori opprettet: { $name }
history-category-edited = Kategori redigert: { $name }
history-category-deleted = Kategori slettet: { $name }
charts-project = Prosjekt
charts-all-projects = Alle prosjekter
charts-date-from = Fra
charts-date-to = Til
charts-total = Totalt
charts-tab-people = Personer
charts-tab-projects = Prosjekter
charts-scope = Hvem sine utgifter
charts-scope-group = Gruppe
charts-scope-me = Meg
charts-currency = Valuta
charts-my-share = Min andel
charts-share-of-total = { $pct } % av { $total }
charts-i-paid = Jeg betalte
charts-paid-more = { $amount } mer enn min andel
charts-paid-less = { $amount } mindre enn min andel
charts-paid-even = Akkurat min andel
charts-part-title = Din del av hver kategori
charts-part-desc = Grått er det gruppen brukte, farge er det du brukte.
charts-breakdown-title = Fordeling etter kategori
charts-breakdown-desc = Trykk på et stykke eller en rad for å se utgiftene.
charts-of-total = { $amount } av { $total }
charts-show-all = Vis alle ({ $count })
charts-show-less = Vis færre
charts-spend-title = Forbruk over tid
charts-group-by = Grupper etter
bucket-day = Dag
bucket-week = Uke
bucket-month = Måned
charts-avg = snitt
charts-cat-title-day = { $category }, dag for dag
charts-cat-title-week = { $category }, uke for uke
charts-cat-title-month = { $category }, måned for måned
charts-cat-desc = Velg en kategori for å følge den over tid.
charts-running-title = Løpende sum
charts-running-desc = Siden { $date }.
charts-avg-per-day = { $amount } / dag i snitt
charts-avg-per-week = { $amount } / uke i snitt
charts-avg-per-month = { $amount } / måned i snitt
charts-people-title = Hvem bar gruppen
charts-people-desc = Hva hver person betalte, ved siden av hva de brukte.
charts-paid = Betalt
charts-fair-share = Rettferdig andel
charts-you = (deg)
charts-net-more = betalte mer
charts-net-less = betalte mindre
charts-balance-title = Saldoen din over tid
charts-balance-desc = Over linjen skylder gruppen deg. Under den skylder du gruppen.
charts-owed = Du har til gode
charts-owe = Du skylder
charts-projects-title = Din andel per prosjekt
charts-projects-desc = Summer holdes per valuta.
history-empty = Ingen hendelser
history-by = Av { $name }
not-found-hint = Denne siden finnes ikke, eller den er flyttet.
payers-title-paid-by = Betalt av
payers-title-sender = Avsender
payers-title-contributors = Bidragsytere
debtors-title-debtors = Skylder
debtors-title-recipients = Mottakere
debtors-title-beneficiaries = Begunstigede
debtors-title-for = For
debtors-title-recipient = Mottaker
expense-split-between = delt mellom { $count }
expense-people =
    { $count ->
        [one] { $count } person
       *[other] { $count } personer
    }

### Welcome

welcome-title = Regnskapet ditt angår ingen andre.
welcome-subtitle = Del utgifter med venner.
welcome-note = Gratis. Ingen konto nødvendig. Ingen reklame.
welcome-link-title = Én lenke, og alle er med.
welcome-link-body = Ingen trenger å opprette en konto.
welcome-link-account = En konto? Aldri påkrevd. Den brukes til å finne prosjektene dine på en annen enhet, invitere venner fra appen og dele betalingsopplysningene dine.
welcome-demo-project = Helg i Lyon
welcome-private-title = Ingen kan lese regnskapet ditt. Ikke engang vi.
welcome-private-body = Navn, beløp, prosjekter: alt krypteres på enheten din. Bare du har nøkkelen.
welcome-private-names = Navn
welcome-private-amounts = Beløp
welcome-private-projects = Prosjekter
welcome-scan-title = Ta bilde av kvitteringen.
welcome-scan-body = Beløp, dato og kategori fylles ut av seg selv. Alt skjer på telefonen din. Bildet lagres ikke.
welcome-eu-title = 100 % europeisk
welcome-no-ads = Ingen reklame
welcome-no-trackers = Ingen sporing
welcome-step = Steg { $current } av { $total }
welcome-next = Neste
welcome-skip = Hopp over
welcome-start = Kom i gang
welcome-how-it-works = Hvordan fungerer det, egentlig?

### Help

help-intro = Et vanlig spørsmål? Trykk for å vise svaret.
help-create-project-q = Hvordan oppretter jeg et prosjekt?
help-create-project-a = Fra startskjermen trykker du på +-knappen nederst. Gi prosjektet et navn, velg valuta, og du er i gang.
help-add-participants-q = Hvordan legger jeg til deltakere?
help-add-participants-a = Åpne prosjektet, og legg til deltakere fra medlemslisten. Hver deltaker kan betale for eller skylde for en utgift.
help-share-project-q = Hvordan deler jeg et prosjekt?
help-share-project-a = Del prosjektets URL (den i adressefeltet). Alle med lenken kan se og redigere prosjektet.
help-add-expense-q = Hvordan legger jeg til en utgift?
help-add-expense-a = I et prosjekt trykker du på +, skriver inn beløpet, hvem som betalte og hvem det skal deles mellom. Du kan også velge en annen dato enn i dag.
help-types-q = Hva er forskjellen på en utgift, en overføring og en inntekt?
help-types-expense = - et kjøp gjort av én person og delt mellom flere.
help-types-transfer = - en tilbakebetaling fra én person til en annen, uten deling.
help-types-gain = - penger mottatt (en refusjon, en gave) som skal deles mellom flere personer.
help-past-date-q = Kan jeg datere en utgift tilbake i tid?
help-past-date-a = Ja, datofeltet er fritt. Tidspunktet posten ble opprettet, lagres separat.
help-who-owes-q = Hvordan regner Counted ut hvem som skylder hva?
help-who-owes-a = Counted beregner hver deltakers nettosaldo (det de har lagt ut minus det de skylder), og foreslår så den korteste rekken overføringer som gjør opp for alle.
help-minimal-transfers-q = Hvorfor er antallet foreslåtte overføringer minimalt?
help-minimal-transfers-a = Algoritmen parer først saldoer som utligner hverandre nøyaktig, og går så gjennom resten fra største kreditor til største debitor. Resultatet: færre overføringer for å gjøre opp alt.
help-import-tricount-q = Hvordan importerer jeg et prosjekt fra Tricount?
help-import-tricount-a = Fra startskjermen trykker du på «+»-knappen nederst, og deretter
help-import-tricount-b = Lim inn delingslenken til Tricount-en du vil importere.
help-encryption-q = Er dataene mine kryptert?
help-encryption-a = Ja. Counted kombinerer to garantier:
help-encryption-e2ee-term = Ende-til-ende-kryptering
help-encryption-e2ee-def = - alt mellom deg og serveren sendes kryptert.
help-encryption-zero-term = Null tilgang
help-encryption-zero-def = - du krypterer dataene før de sendes, og serveren lagrer bare chiffertekst. Vi har ingen måte å lese den på.
help-encryption-see = For detaljer, se
help-forgot-password-q = Hva skjer hvis jeg glemmer passordet mitt?
help-forgot-password-warning = Ingen kan hente det fram for deg.
help-forgot-password-a = Passordet ditt beskytter krypteringsnøklene dine og sendes aldri til oss: ingen - heller ikke vi - kan dekryptere prosjektene dine for deg. Oppbevar det trygt, helst i en passordbehandler.
help-archive-delete-q = Hvordan arkiverer eller sletter jeg et prosjekt?
help-archive-delete-a = Fra prosjektskjermen åpner du menyen og velger
help-archive-delete-b = for å skjule det, men beholde det. Et prosjekt slettes for godt når det siste medlemmet forlater det.
help-delete-account-q = Hvordan sletter jeg kontoen min?
help-delete-account-a = Åpne Innstillinger og bruk «Slett kontoen min». Det skjer umiddelbart og kan ikke angres.
help-contact = Et annet spørsmål? Skriv til oss på

# Receipt scanning (mobile only)
expense-scan = Skann en kvittering
scan-in-progress = Leser kvitteringen…
scan-error-capture = Kunne ikke ta det bildet. Prøv igjen, eller skriv inn utgiften manuelt.
scan-error-unreadable = Ingenting lesbart på den kvitteringen. Skriv inn utgiften manuelt.
scan-check-amount = Sjekk totalen - den var ikke tydelig trykt.
scan-take-photo = Ta et bilde
scan-choose-photo = Velg et bilde
expense-converted-from = Betalt { $amount } { $from } · 1 { $from } = { $rate } { $to }
project-currency = Valuta
project-currency-locked = Valutaen låses når prosjektet opprettes.
currency-search = Søk etter valuta

update-required-title = Oppdatering kreves
update-required-body = Denne versjonen av Counted er for gammel til å snakke med serveren. Oppdater den for å fortsette å bruke appen.
update-required-button = Oppdater

notifications-label = Varsler
notifications-title = Varsler
notifications-empty = Ingenting nytt
notifications-friend-request = Venneforespørsel

friends-title = Venner
friends-anonymous-body = Venner lagres med kontoen din. Logg inn for å legge til personer og invitere dem til prosjektene dine uten å dele en lenke.
friends-add-title = Legg til en venn
friends-add-hint = Har personen en konto, ser vedkommende forespørselen din i appen. Hvis ikke, gi beskjed: vedkommende finner den ved å registrere seg med denne adressen.
friends-add-button = Legg til
friends-add-from-project = Legg til som venn
friends-request-sent = Forespørsel sendt
friends-no-account-key = Logg inn på nytt for å administrere vennene dine på denne enheten.
friends-incoming-title = Forespørsler
friends-accept = Godta
friends-decline = Avslå
friends-list-title = Vennene mine
friends-list-empty = Ingen venner ennå. Legg til noen via e-post ovenfor, eller fra et prosjekt dere deler.
friends-remove = Fjern
friends-remove-confirm-title = Fjern venn
friends-remove-confirm-message = { $email } vil ikke lenger være blant vennene dine, og du ikke blant deres. Hver av dere kan sende en ny forespørsel senere.
friends-no-key = Ikke klar ennå
friends-key-changed = Kode endret
friends-key-changed-hint = En venns sikkerhetskode er endret. Det skjer når vennen sikrer kontoen sin, eller hvis nøkkelen er byttet ut. Invitasjoner med vennen er blokkert til dere leser den nye koden for hverandre, personlig eller på telefon, og du trykker på «Koden stemmer».
friends-fingerprint = Sikkerhetskode
friends-fingerprint-hint = To venner som leser opp den samme sikkerhetskoden for hverandre, vet at ingen sitter imellom dem - ikke engang serveren vår.
friends-key-mismatch = Serveren sendte en offentlig nøkkel for kontoen din som ikke passer til den private nøkkelen din. Sikkerhetskoden din er skjult, og invitasjoner er slått av på denne enheten: noen tukler kanskje med serveren.
keypair-mismatch = Nøklene serveren sendte for kontoen din stemmer ikke med dem på denne enheten. Invitasjoner og nøkkelbytter er slått av her til de stemmer: noen tukler kanskje med serveren.
friends-outgoing-title = Sendt
friends-outgoing-hint = Venter på svar. Du ser dem blant vennene dine når de godtar.
friends-withdraw = Avbryt
friends-notify = Send en melding
friends-notify-label = Gi { $email } beskjed
friends-notify-message = Jeg bytter til Counted for de felles utgiftene våre. Gratis registrering på https://counted.fr med adressen { $email }: venneforespørselen min venter der.
friends-notify-copied = Melding kopiert
friends-notify-copy-failed = Kunne ikke kopiere meldingen
guests-hint-title =
    { $count ->
        [one] { $count } deltaker har ikke blitt med i prosjektet ennå
       *[other] { $count } deltakere har ikke blitt med i prosjektet ennå
    }
guests-hint-later = Senere
guests-hint-send = Inviter dem
guests-send-title = Send prosjektet til deltakerne
guests-send-friend = Inviter en Counted-venn
guests-send-friend-badge = Uten lenke
guests-send-friend-hint = Har vedkommende allerede en konto, får vedkommende en kryptert invitasjon; ingenting går via meldingsappen din.
guests-send-link = Del prosjektlenken
guests-send-link-hint = For dem uten konto. Ett trykk åpner prosjektet.
guests-send-link-warning = Alle med denne lenken kan lese og endre prosjektet. Meldingsappen din ser den også.
invite-friends-title = Inviter venner
invite-friends-hint = Prosjektnøkkelen krypteres for hver venn på denne enheten. Serveren ser den aldri.
invite-friends-empty = Ingen venner å invitere ennå.
invite-friends-button = Inviter
invite-sent = { $count ->
    [one] Invitasjon sendt
   *[other] { $count } invitasjoner sendt
}
invitation-badge = Invitasjon
invitation-to = Bli med i «{ $name }»
invitation-to-unnamed = Bli med i et prosjekt
invitation-unreadable = Denne invitasjonen kan ikke åpnes på denne enheten
invitation-from = Fra { $email }
invitation-accept = Bli med
invitation-decline = Avslå

# Participants in the create and edit modals, and the "who are you?" picker - see
# docs/plans/friends.md §11.
participants-you-label = Navnet ditt i dette prosjektet
participants-you-badge = Deg
participants-others = Andre deltakere
participants-removed = Fjernet
participant-restore = Gjenopprett
participants-empty = Legg til deltakere nedenfor.
participants-duplicate = «{ $name }» står allerede på listen.
participants-input-label = Legg til en venn eller skriv et navn
participants-input-placeholder = Venn eller et navn
participants-suggest-friend = Venn · blir med som «{ $name }», får en invitasjon
participants-suggest-not-ready = Venn · ikke klar ennå
participants-suggest-guest = Legg til «{ $text }»
participants-suggest-guest-sub = Gjest
participants-friends = Vennene dine
participants-all-friends = Alle venner
participants-login-hint = Logg inn for å legge til folk rett fra vennelisten din.
participants-invite-badge = Inviter
participants-guest-badge = Gjest
participants-rename = Gi { $name } nytt navn
participants-remove = Fjern { $name }
participants-rename-label = Nytt navn
participants-rename-save = Lagre navnet
participants-rename-hint = Navnet alle ser i dette prosjektet. Invitasjonen går fortsatt til { $email }.
participants-invited-badge = Invitert
participants-invited-sub = { $email } · ikke godtatt ennå
participants-invited-pending = Invitasjonen er ikke godtatt ennå
participants-unlinked = Ikke knyttet til en konto
add-project-create-invite = Opprett og inviter { $count }
edit-project-save-invite = Lagre og inviter { $count }
edit-project-you-are = På denne enheten er du { $name }
edit-project-no-identity = Du har ikke valgt hvem du er ennå
edit-project-switch = Bytt
edit-project-choose = Velg
invite-failed = Disse invitasjonene kunne ikke sendes: { $emails }
invite-again = Inviter på nytt
friend-picker-title = Legg til venner
user-selection-invited-hint = { $email } har invitert deg til «{ $project }».
user-selection-suggested = Foreslått
user-selection-suggested-sub = { $email } la deg til med dette navnet
user-selection-confirm-as = Jeg er { $name }
user-selection-missing = Står ikke navnet ditt her? Be en deltaker legge deg til i prosjektinnstillingene.

## Gjentakende utgifter

repeat-label = Gjenta
repeat-none = Gjentas ikke
repeat-weekly = Hver uke
repeat-biweekly = Annenhver uke
repeat-monthly = Hver måned
repeat-quarterly = Hver 3. måned
repeat-yearly = Hvert år
repeat-every-weeks = Hver { $count }. uke
repeat-every-months = Hver { $count }. måned
repeat-every-years = Hvert { $count }. år
repeat-custom = Tilpasset…
repeat-every = Hver
repeat-unit-weeks = uker
repeat-unit-months = måneder
repeat-unit-years = år
repeat-on-weekday = på { $weekday }
repeat-on-day = den { $day }.
repeat-on-day-month = den { $day }. { $month }
repeat-month-end = I kortere måneder havner den på siste dag.
repeat-ends = Slutter
repeat-ends-never = Aldri
repeat-ends-on = På en dato
repeat-ends-after = Etter
repeat-fewer = Færre
repeat-more = Flere
repeat-last-on = siste { $date }
repeat-variable = Beløpet endres hver gang
repeat-variable-hint = Hver legges til med det siste beløpet og merkes «må bekreftes».
repeat-done = Ferdig
repeat-no-end = Ingen slutt
repeat-until = Til { $date }
repeat-occurrences = Antall ganger: { $count }
repeat-offline = Krever tilkobling. Selve utgiften kan fortsatt legges til.
repeat-foreign = Gjentas som { $amount } { $currency }, omregnet én gang til dagens kurs. Du får varsel hvis kursen endrer seg mer enn 5 %.
repeat-backfill = Starter i fortiden. Utgifter lagt til nå: { $count }.
add-and-repeat = Legg til og gjenta
weekday-1 = mandag
weekday-2 = tirsdag
weekday-3 = onsdag
weekday-4 = torsdag
weekday-5 = fredag
weekday-6 = lørdag
weekday-7 = søndag
recurring-title = Gjentakende utgifter
recurring-strip = Gjentakende utgifter: { $count }
recurring-next = Neste: { $name }, { $date }
recurring-to-confirm = Må bekreftes: { $count }
recurring-per-month = Per måned, ca.
recurring-your-share = Din andel
recurring-active = Aktive
recurring-paused = På pause
recurring-finished = Avsluttet
recurring-paid-by = betalt av { $name }
recurring-next-on = Neste { $date }
recurring-progress = { $done } av { $total }
recurring-rate-badge = Kurs endret { $percent } %
recurring-empty = Ingenting gjentas ennå. Velg «Gjenta» når du legger til en utgift: husleie, abonnementer, regninger.
recurring-next-ones = Kommende
recurring-added-so-far = Lagt til hittil
recurring-set-up-by = Opprettet av
recurring-pause = Sett på pause
recurring-resume = Fortsett
recurring-stop = Slutt å gjenta
recurring-stop-title = Stoppe «{ $name }»?
recurring-stop-message = Den gjentas ikke lenger. Utgifter som alt er lagt til, blir værende.
recurring-resume-title = Fortsette «{ $name }»?
recurring-resume-message = Neste { $date }. Datoer som ble hoppet over under pausen, legges ikke til.
recurring-edit-title = Rediger gjentakende utgift
recurring-edit-banner = Endringene gjelder fra { $date }. Utgifter som alt er lagt til, forblir som de er.
recurring-next-on-label = Neste
recurring-next-too-early = Neste dato må komme etter den sist tillagte utgiften.
recurring-use-stop = Bruk «Slutt å gjenta» på den gjentakende utgiften for å avslutte den.
recurring-drift = Kursen for { $currency } har endret seg { $percent } % siden den ble opprettet. Hver legges fortsatt til som { $amount } { $project_currency } (1 { $currency } = { $rate }). Til dagens kurs ville det vært { $today_amount } { $project_currency }.
recurring-use-rate = Bruk dagens kurs
recurring-keep = Behold { $amount } { $currency }
recurring-added = Gjentakende utgifter lagt til: { $count }
recurring-blocks-removal = { $name } kan ikke fjernes ennå: inngår i { $rules }. Ta { $name } ut av dem eller stopp dem, og lagre på nytt.
history-recurring-added = Lagt til automatisk: { $name } ({ $date })
history-recurring-created = Gjentakende utgift opprettet: { $name }
history-recurring-edited = Gjentakende utgift redigert: { $name }
history-recurring-paused = Gjentakende utgift satt på pause: { $name }
history-recurring-resumed = Gjentakende utgift gjenopptatt: { $name }
history-recurring-stopped = Gjentakende utgift stoppet: { $name }
occurrence-recurring = Gjentakende utgift
occurrence-auto = Lagt til automatisk av en gjentakende utgift.
occurrence-auto-next = Lagt til automatisk av en gjentakende utgift. Neste { $date }.
occurrence-auto-stopped = Lagt til automatisk av en gjentakende utgift som siden er stoppet.
occurrence-manage = Administrer
estimate-badge = Må bekreftes
expense-pending = Venter
estimate-title = Beløpet må bekreftes.
estimate-body = Lagt til med forrige beløp. Skriv inn det riktige når det kommer.
estimate-confirm = Bekreft beløp
apply-to = Gjelder
apply-this-only = Bare denne utgiften
apply-and-next = Denne og de neste
apply-and-next-hint = Den gjentakende utgiften endres fra { $date }
apply-rule-failed = Utgiften ble lagret, men den gjentakende utgiften ble ikke endret.
occurrence-delete-message = «{ $name }» fra { $date } slettes for godt. Den fortsetter å gjentas, og denne datoen kommer ikke tilbake.
occurrence-delete-one = Slett bare denne
occurrence-delete-stop = Slett og slutt å gjenta
error-recurring-clock = Enhetens klokke går for fort. Sjekk dato og klokkeslett.
error-recurring-not-found = Denne gjentakende utgiften finnes ikke lenger.
error-recurring-stale = Noen har endret denne gjentakende utgiften i mellomtiden. Den er lastet inn på nytt: sjekk den og lagre igjen.
error-too-many-recurring = Prosjektet har allerede 50 gjentakende utgifter. Stopp en du ikke trenger lenger for å legge til en ny.
error-user-in-recurring = Deltakeren inngår i en gjentakende utgift. Fjern deltakeren derfra eller stopp utgiften først.

home-owed-to-you = Du har til gode
home-you-owe = Du skylder
home-details-show = Vis detaljer
home-details-hide = Skjul detaljer
home-all-settled = Alt er gjort opp
home-pay = Betal
home-remind = Påminn
home-remind-message = Liten påminnelse: det gjenstår tilbakebetalinger i { $project }.
home-row-owed = du har til gode
home-row-owe = du skylder
home-row-settled = Gjort opp
home-totals-partial = Uten prosjekter uten valutakurs
error-account-key-changed = Nøkkelen til kontoen din ble endret på en annen enhet. Logg inn igjen for å fortsette.
settings-security = Sikkerhet
settings-change-password = Endre passord
settings-change-password-hint = Prosjektene og dataene dine forblir som de er. De andre enhetene dine blir logget ut.
change-password-title = Endre passordet ditt
change-password-current = Nåværende passord
change-password-new = Nytt passord
change-password-confirm = Bekreft det nye passordet
change-password-mismatch = De to nye passordene er ikke like.
change-password-submit = Endre passord
change-password-submitting = Endrer…
change-password-done = Passordet er endret. De andre enhetene dine er logget ut.
error-rotation-incomplete = Kontoen din ble endret i mellomtiden. Prøv igjen.
error-key-lifecycle-unavailable = Ikke tilgjengelig ennå: først må alle ha den nyeste versjonen av appen.
settings-compromised = Noen kan kjenne passordet mitt
settings-compromised-hint = Nytt passord, nye nøkler, og alle andre enheter logges ut.
settings-compromised-open = Sikre kontoen min
compromised-title = Sikre kontoen din
compromised-body = Passordet ditt og kontoens nøkler byttes ut, og alle andre enheter logges ut. Vennene dine vil se at sikkerhetskoden din endres: sammenlign den med dem på nytt.
compromised-keeps = Den som hadde passordet ditt, beholder det som allerede er åpnet: prosjektene som kunne leses til nå.
compromised-submit = Bytt passord og nøkler
compromised-done = Kontoen din er sikret. Be vennene dine sammenligne den nye sikkerhetskoden din.
friends-new-fingerprint = Ny kode:
friends-code-matches = Koden stemmer
friends-code-trusted = Koden er bekreftet.
settings-recovery-key = Gjenopprettingsnøkkel
settings-recovery-key-none = Uten den koster et glemt passord innstillingene, betalingsopplysningene og vennekodene dine, og det tar 3 dager.
settings-recovery-key-set = Opprettet. Glemmer du passordet, får du kontoen tilbake med alt innhold.
settings-recovery-key-pending = En ny nøkkel erstatter den { $when }. Var det ikke deg? Avbryt med din nåværende nøkkel.
recovery-key-create = Opprett
recovery-key-replace = Erstatt
recovery-key-cancel-replacement = Avbryt den nye nøkkelen
recovery-key-title = Gjenopprettingsnøkkelen din
recovery-key-body = Skriv den ned eller lagre den der bare du har tilgang. Counted kan ikke vise den igjen eller finne den for deg.
recovery-key-copy = Kopier
recovery-key-copied = Kopiert
recovery-key-save = Lagre som fil
recovery-key-print = Skriv ut
recovery-key-next = Fortsett
recovery-key-confirm-body = Skriv gruppe { $n } av nøkkelen for å sjekke kopien din.
recovery-key-confirm-wrong = Gruppen stemmer ikke. Se på kopien din igjen.
recovery-key-password-body = Skriv inn passordet ditt for å lagre nøkkelen på kontoen.
recovery-key-submit = Lagre nøkkelen
recovery-key-done = Gjenopprettingsnøkkelen er lagret.
recovery-key-replacement-done = Ny nøkkel lagret. Den erstatter den nåværende om 72 timer; til da virker den nåværende fortsatt.
recovery-key-field = Gjenopprettingsnøkkel
recovery-key-file-intro = Gjenopprettingsnøkkel for Counted
recovery-key-cancel-done = Den nye nøkkelen er avbrutt.
compromised-new-key = Lagre den nye gjenopprettingsnøkkelen nå: den erstatter den gamle om 72 timer, og til da virker den gamle fortsatt.
pending-reset-title = Det er bedt om tilbakestilling av kontoen din
pending-reset-body = Den trer i kraft { $when }: da går innstillingene, betalingsopplysningene og vennekodene tapt, og enhetene dine må logge inn igjen. Var det ikke deg? Avbryt med ditt nåværende passord.
pending-reset-cancel = Avbryt tilbakestillingen
pending-reset-cancelled = Tilbakestillingen er avbrutt.
login-forgot = Glemt passordet?
forgot-title = Glemt passordet
forgot-body = Skriv inn e-posten din. Hvis den har en konto, sender vi en lenke for å velge nytt passord.
forgot-submit = Send lenken
forgot-sent = Hvis adressen har en konto, er en lenke på vei. Den utløper om 24 timer.
reset-title = Få tilbake kontoen din
reset-with-key-body = Skriv inn gjenopprettingsnøkkelen og et nytt passord: alt kommer tilbake slik det var.
reset-submit-key = Gjenopprett kontoen min
reset-no-key = Jeg har ikke gjenopprettingsnøkkelen min
reset-without-key-title = Tilbakestill uten gjenopprettingsnøkkelen
reset-without-key-body = Kontoen din starter på nytt med et nytt passord om 72 timer. Prosjektene kommer tilbake fra enhetene dine og delingslenkene; innstillingene, betalingsopplysningene og vennekodene gjør ikke det. Til da virker det nåværende passordet fortsatt og kan avbryte tilbakestillingen.
reset-name = Navnet ditt
reset-submit-request = Tilbakestill kontoen min
reset-requested = Ferdig. Det nye passordet virker fra { $when }. Til da virker det nåværende og kan avbryte det.
reset-pending-already = En tilbakestilling venter allerede til { $when }. En ny forespørsel starter de 72 timene på nytt.
recovered-done = Kontoen din er tilbake. De andre enhetene dine er logget ut.
login-reset-undoable = Dette passordet ble erstattet av en tilbakestilling de siste 30 dagene. Ba du ikke om det, angre det: kontoen kommer tilbake slik den var.
login-undo-reset = Angre tilbakestillingen
reset-undone = Tilbakestillingen er angret. Kontoen er som før.
error-reset-pending = Det nye passordet virker når tilbakestillingen trer i kraft, 72 timer etter forespørselen.
error-account-disabled = Kontoen ble slettet kort etter en tilbakestilling og venter på å bli fjernet. Det forrige passordet kan fortsatt angre tilbakestillingen.
error-recovery-key-invalid = Denne gjenopprettingsnøkkelen åpner ikke kontoen.
error-project-key-required = Prosjektet trenger nøkkelen sin. Åpne det igjen fra delingslenken.
names-or = { $first } eller { $last }
participants-admin-badge = Admin
edit-project-admins = Admins: { $count }
edit-project-succession = Hvis den siste adminen forlater prosjektet, blir medlemmet med konto som har vært med lengst, admin.
member-sheet-account = Medlem med konto
member-actions = Handlinger for { $name }
member-make-admin = Gjør til admin
member-make-admin-hint = Kan da bytte prosjektets nøkkel og fjerne noen.
member-made-admin = { $name } er nå admin.
group-security = Prosjektsikkerhet
group-security-admin = Prosjektsikkerhet · du er admin
group-rotate = Bytt prosjektets nøkkel
group-rotate-hint = Hvis prosjektlenken er kompromittert: de som har den gamle lenken, mister tilgangen til prosjektet.
group-remove = Fjern noen fra prosjektet
group-remove-hint = Fjerner tilgangen og bytter nøkkelen samtidig.
group-admins-only = Bare for admins.
group-admins-only-account = Bare for admins, som har en konto.
group-ask-admin = Spør { $names }, prosjektets admin.
group-ask-admins = Spør { $names }, prosjektets admins.
group-ask-an-admin = Spør en av prosjektets admins.
group-with-account = Med konto
group-free = Gratis
group-sign-in-hint = En admin i prosjektet, { $names }, kan deretter gi deg rollen.
group-sign-in-hint-any = En admin i prosjektet kan deretter gi deg rollen.
group-no-admin = Prosjektet har ingen admin, så nøkkelen kan ikke byttes.
group-hide = Skjul noen
group-hide-hint = Skjuler personen i listene uten å fjerne tilgangen: lenken og nøkkelen virker fortsatt.
group-recreate-hint = Prosjektet har ingen admin. Opprett et nytt prosjekt for å fjerne noen.
group-recreate = Opprett et nytt prosjekt
group-no-admin-yet = Prosjektet har ingen admin ennå.
group-creator-hint = Bare du kan bli admin for dette prosjektet, ved å opprette en konto.
group-creator-elsewhere-hint = Bare personen som opprettet prosjektet, kan bli admin for det, ved å opprette en konto. Ellers kan du opprette et nytt prosjekt for å fjerne noen.
member-revoke-admin = Fjern admin-rollen
member-revoke-admin-hint = Kan ikke lenger bytte prosjektets nøkkel eller fjerne noen.
member-revoked-admin = { $name } er ikke lenger admin.
member-step-down = Slutt å være admin
member-step-down-hint = Du kan ikke lenger bytte prosjektets nøkkel eller fjerne noen.
member-stepped-down = Du er ikke lenger admin.
member-remove = Fjern fra prosjektet
member-remove-hint = Bytter prosjektets nøkkel: { $name } kan ikke lenger lese noe nytt.
member-hide = Skjul i listene
error-user-not-admin = Medlemmet er ikke admin.
error-creator-proof-invalid = Denne enheten opprettet ikke prosjektet.
error-project-has-admin = Prosjektet har allerede en admin.
error-not-project-admin = Bare prosjektets admins kan gjøre dette.
error-user-not-claimed = Denne deltakeren har ingen konto og kan derfor ikke være admin.
error-project-key-rotated = Prosjektets nøkkel er byttet. Be et medlem om den nye lenken.
error-project-rotating = Prosjektets nøkkel byttes nå. Prøv igjen om et øyeblikk.
error-rotation-expired = Nøkkelbyttet tok for lang tid og ble avbrutt. Start på nytt.
error-rotation-rows-changed = Prosjektet endret seg under nøkkelbyttet. Start på nytt.
project-rotated-title = Du har ikke lenger tilgang til dette prosjektet
project-rotated-body = Nøkkelen til dette prosjektet er byttet av en admin. Be vedkommende om den nye lenken.
project-rotated-open-link = Åpne en lenke
rotation-remove-title = Fjerne { $name } fra prosjektet?
rotation-rotate-title = Bytte prosjektets nøkkel?
rotation-remove-body = Når du fjerner et medlem, lages det en ny nøkkel. Den nye lenken må deles med dem som ikke har en konto.
rotation-rotate-body = De som har den gamle lenken, mister tilgangen til prosjektet. Den nye lenken må deles med dem som ikke har en konto.
rotation-boxed =
    { $count ->
        [0] Ingen andre medlemmer får nøkkelen automatisk
        [one] { $count } medlem får nøkkelen automatisk
       *[other] { $count } medlemmer får nøkkelen automatisk
    }
rotation-by-link-title = Personer du må sende den nye lenken til
rotation-by-link-hint = Disse personene har ikke en konto. Du sender dem lenken rett etterpå.
rotation-names-and = { $first } og { $last }
rotation-names-and-others =
    { $names } og { $count ->
        [one] { $count } annen
       *[other] { $count } andre
    }
rotation-confirm-remove = Fjern og bytt nøkkelen
rotation-confirm-rotate = Bytt nøkkelen
rotation-done-title = Ny nøkkel på plass
rotation-done-removed = { $name } er ikke lenger med i prosjektet.
rotation-done-rotated = Prosjektets nøkkel er byttet.
rotation-done-boxed =
    { $count ->
        [0] Ingen andre medlemmer fikk nøkkelen automatisk.
        [one] { $count } medlem fikk nøkkelen automatisk.
       *[other] { $count } medlemmer fikk nøkkelen automatisk.
    }
rotation-new-link =
    { $count ->
        [0] Ny lenke til prosjektet
        [one] Ny lenke å sende · { $count } person uten konto
       *[other] Ny lenke å sende · { $count } personer uten konto
    }
rotation-link-renewed = Lenken er fornyet.
rotation-done = Ferdig
history-key-changed = Prosjektets nøkkel byttet
history-member-removed = { $name } er ikke lenger med i prosjektet
project-rotated-row-hint = Nøkkelen er byttet. Be om den nye lenken.
