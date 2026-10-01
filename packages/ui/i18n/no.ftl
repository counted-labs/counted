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
error-generic = Noe gikk galt. Prøv igjen.

error-invalid-email = Den e-postadressen er ikke gyldig.
error-invalid-password = Det passordet er ikke gyldig.
error-password-too-short = Passordet må være minst 8 tegn.
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
login-resend = Send bekreftelseslenken på nytt
login-resending = Sender…
login-resend-sent = E-post sendt - sjekk innboksen din.

register-submit = Opprett en konto
register-submitting = Oppretter…
register-have-account = Har du allerede en konto?
register-password-placeholder = Minst 8 tegn
register-password-warning = Skriv ned passordet ditt. Glemmer du det, kan ikke kontoen gjenopprettes.
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
account-delete-warning = Umiddelbart og permanent, uten papirkurv. Utgifter du har lagt inn i et delt prosjekt, forblir synlige for de andre medlemmene - de er en del av regnskapet deres.
account-delete-confirm-title = Slett konto
account-delete-confirm-message = Kontoen din, øktene dine og prosjektlisten din slettes permanent. Uten passordet ditt blir de krypterte dataene i et delt prosjekt uleselige for deg - dette kan ikke angres.

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
payment-methods-stale = Betalingsopplysningene dine ble endret på en annen enhet. De er lastet inn på nytt — prøv igjen.
payment-methods-key-missing = Logg inn på nytt for å administrere betalingsopplysningene dine.
settings-payment-methods-share-warning = En delt metode er synlig for alle medlemmer av prosjektene der du har valgt navnet ditt - alle som har en av de prosjektlenkene.
payment-method-share = Del med prosjektene mine
payment-method-share-hint = Vises ved siden av navnet ditt når noen skylder deg penger.
payment-method-copy = Kopier { $name }
payment-method-copied = Kopiert.
payment-method-copy-failed = Kunne ikke kopiere - marker teksten og kopier den manuelt.

verify-email-checking = Bekrefter e-postadressen din…
verify-email-welcome = E-post bekreftet - velkommen til Counted!
verify-email-back-to-login = Tilbake til innlogging

### Project status

project-close = Lukk
project-archive = Arkiver
project-reopen = Gjenåpne
project-unarchive = Gjenopprett fra arkiv

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
projects-empty-hint = Opprett et prosjekt med knappen nedenfor
projects-offline-banner = Frakoblede data - koble til igjen for å oppdatere.
projects-no-local-data = Ingen lokale data
projects-no-local-data-hint = Logg inn for å laste prosjektene dine for første gang.
projects-add = Legg til et prosjekt
projects-create = Opprett et prosjekt
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
project-unlock = Lås opp
project-no-local-data-hint = Logg inn for å laste prosjektets data for første gang.
project-gone-title = Dette prosjektet finnes ikke lenger
project-gone-hint = Det ble slettet da det siste medlemmet forlot det. Delingslenken virker ikke lenger, selv om du åpner den igjen.

expense-add = Legg til en utgift
transfer-add = Legg til en overføring
expense-edit-title = Rediger utgiften
expense-category = Kategori
expense-category-auto = Auto · { $emoji }
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
expense-inconsistent-detail = Beløpene stemmer ikke: { $paid } betalt, { $owed } skyldig, for en utgift på { $total }. Rediger utgiften for å rette det.
missing-access-key = Tilgangsnøkkel mangler. Åpne prosjektet via delingslenken.
filter-all = Alle
filter-my-payments = Mine betalinger
filter-my-debts = Det jeg skylder
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
charts-my-share-note = Disse tallene er din andel av hver utgift.
charts-my-share-skipped =
    { $count ->
        [one] 1 prosjekt telles ikke med — ingen deltaker valgt, eller dataene ble ikke lastet.
       *[other] { $count } prosjekter telles ikke med — ingen deltaker valgt, eller dataene ble ikke lastet.
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
charts-paid-more = { $amount } mer enn din andel
charts-paid-less = { $amount } mindre enn din andel
charts-paid-even = Akkurat din andel
charts-part-title = Din del av hver kategori
charts-part-desc = Grått er det gruppen brukte, farge er det du brukte.
charts-breakdown-title = Fordeling etter kategori
charts-breakdown-desc = Trykk på et stykke eller en rad for å se utgiftene.
charts-of-total = { $amount } av { $total }
charts-show-all = Vis alle ({ $count })
charts-show-less = Vis færre
charts-spend-title = Forbruk over tid
charts-spend-desc = Korte perioder vises per dag, lengre per uke eller måned.
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
charts-projects-desc = Summer holdes per valuta og legges aldri sammen.
history-empty = Ingen hendelser
history-by = Av { $name }
not-found-hint = Denne siden finnes ikke, eller den er flyttet.
payers-title-paid-by = Betalt av
payers-title-sender = Avsender
payers-title-contributors = Bidragsytere
debtors-title-debtors = Skylder
debtors-title-recipients = Mottakere
debtors-title-beneficiaries = Begunstigede

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
help-forgot-password-warning = Dataene dine går permanent tapt.
help-forgot-password-a = Krypteringsnøkkelen utledes fra passordet ditt, så ingen tilbakestilling er mulig: ingen - heller ikke vi - kan dekryptere prosjektene dine uten det. Oppbevar det trygt, helst i en passordbehandler.
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
project-currency-hint = Alle beløp vises i denne valutaen. Den kan ikke endres senere.
project-currency-locked = Valutaen låses når prosjektet opprettes.

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
friends-add-hint = Vedkommende ser forespørselen din ved innlogging. Ingen av dere får vite om den andre har en konto før forespørselen er godtatt.
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
friends-fingerprint = Sikkerhetskode
friends-fingerprint-hint = To venner som leser opp den samme sikkerhetskoden for hverandre, vet at ingen sitter imellom dem - ikke engang serveren vår.
friends-outgoing-title = Sendt
friends-outgoing-hint = Venter på svar. Du ser dem blant vennene dine når de godtar.
friends-withdraw = Avbryt
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
participants-you-from-account = Hentet fra kontonavnet ditt. Endre det her bare for dette prosjektet.
participants-you-required = Påkrevd. Slik ser de andre deg.
participants-others = Andre deltakere
participants-empty = Ingen ennå. Velg en venn nedenfor eller skriv et navn.
participants-empty-signed-out = Ingen ennå. Skriv et navn for å legge til noen.
participants-duplicate = «{ $name }» står allerede på listen.
participants-input-label = Legg til en venn eller skriv et navn
participants-input-placeholder = Venn eller et navn
participants-suggest-friend = Venn · blir med som «{ $name }», får en invitasjon
participants-suggest-not-ready = Venn · ikke klar ennå
participants-suggest-guest = Legg til «{ $text }» uten konto
participants-suggest-guest-sub = Ingen konto, bare et navn
participants-friends = Vennene dine
participants-all-friends = Alle venner
participants-login-hint = Logg inn for å legge til folk rett fra vennelisten din.
participants-invite-badge = Inviter
participants-guest-badge = Uten konto
participants-guest-sub = Ingen konto, bare et navn
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
