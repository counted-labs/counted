# Latviešu. Pilns, izņemot juridiskos tekstus (legal-, terms-, privacy-), kas pastāv tikai angļu un
# franču valodā un katram ziņojumam atsevišķi atgriežas pie en.ftl.

### Common

loading = Ielādē…
cancel = Atcelt
confirm = Apstiprināt
retry = Mēģināt vēlreiz
delete = Dzēst
back = Atpakaļ
language = Valoda

### Navigation

nav-main = Galvenā navigācija
nav-projects = Projekti
nav-charts = Statistika
nav-settings = Iestatījumi

### Connectivity

offline-banner = Bezsaistē
offline-pending =
    { $count ->
        [zero] { $count } gaida
        [one] { $count } gaida
       *[other] { $count } gaida
    }

sync-conflict-edit = Konflikts: neizdevās rediģēt „{ $name }” (vienums dzēsts). Izlaists.
sync-conflict-delete = Konflikts: neizdevās dzēst „{ $name }” (vienums dzēsts). Izlaists.
sync-conflict-other = Konflikts: darbība ar „{ $name }” neizdevās (vienums dzēsts). Izlaists.
sync-error = Sinhronizācijas kļūda: { $reason }

### Errors

error-network = Nevar sasniegt serveri. Pārbaudi interneta savienojumu.
error-generic = Kaut kas nogāja greizi. Mēģini vēlreiz.

error-invalid-email = Šī e-pasta adrese nav derīga.
error-invalid-password = Šī parole nav derīga.
error-password-too-short = Parolei jābūt vismaz 8 rakstzīmes garai.
error-client-outdated = Šī lietotnes versija ir novecojusi. Atjaunini to, lai pieteiktos.
error-invalid-link = Šī saite nav derīga.
error-batch-too-large = Pārāk daudz vienumu vienlaikus.
error-payers-required = Izvēlies vismaz vienu maksātāju.
error-debtors-required = Izvēlies vismaz vienu personu, kas ir parādā.
error-duplicate-participant = Dalībnieks vienā pusē parādās divreiz.
error-participant-not-in-project = Šis dalībnieks nav šī projekta daļa.
error-too-many-participants = Pārāk daudz dalībnieku vienam izdevumam.
error-invalid-credentials = Nepareizs e-pasts vai parole.
error-unauthenticated = Piesakies, lai to izdarītu.
error-email-not-verified = Tava e-pasta adrese vēl nav apstiprināta.
error-project-not-found = Šis projekts vairs nepastāv.
error-expense-not-found = Šis izdevums vairs nepastāv.
error-user-not-found = Šis dalībnieks vairs nepastāv.
error-tricount-not-found = Tricount nav atrasts, vai tā API atgrieza kļūdu.
error-too-many-members = Šis projekts ir sasniedzis dalībnieku limitu.
error-identity-taken = Cits konts jau ir piesavinājies šo dalībnieku.
error-claim-proof-invalid = Šai ierīcei nav projekta atslēgas, tāpēc tā nevar piesavināties dalībnieku. Atver kopīgošanas saiti vēlreiz.
error-user-has-payments = Šim dalībniekam projektā ir izdevumi, un viņu nevar noņemt.
error-resend-cooldown = Pagaidi 60 sekundes, pirms pieprasi vēl vienu e-pastu.
error-self-friend-request = Nevari pievienot sevi kā draugu.
error-not-a-friend = Vari uzaicināt tikai cilvēkus no sava draugu saraksta.
error-friend-has-no-key = Šis draugs vēl nav atvēris jaunāko lietotnes versiju. Palūdz viņam vienreiz pieteikties un mēģini vēlreiz.
error-friend-request-not-found = Šis draudzības pieprasījums vairs nepastāv.
error-invitation-not-found = Šis uzaicinājums vairs nepastāv.
error-too-many-friend-requests = Pagaidām pārāk daudz draudzības pieprasījumu. Mēģini rīt.
error-too-many-invitations = Pārāk daudz neapstiprinātu uzaicinājumu.
error-invalid-kdf-salt = Šifrēšanas iestatījumi nav derīgi. Atjaunini lietotni un mēģini vēlreiz.
error-mixed-project-batch = Šie dalībnieki nav visi vienā projektā.
error-invalid-payload = Šī lietotnes versija nosūtīja datus, kurus serveris nepieņem. Atjaunini to un mēģini vēlreiz.
error-invalid-public-key = Tava šifrēšanas atslēga nav derīga. Atjaunini lietotni un mēģini vēlreiz.
error-payment-methods-stale = Tavi maksājumu dati tika mainīti citā ierīcē. Pārlādē un mēģini vēlreiz.

### Auth

field-email = E-pasts
field-email-placeholder = tu@piemers.lv
field-password = Parole
field-name = Vārds
field-name-placeholder = Anna Bērziņa

login-title = Pieteikties
login-submit = Pieteikties
login-submitting = Piesakās…
login-password-placeholder = Tava parole
login-no-account = Vēl nav konta?
login-unverified = Tava e-pasta adrese vēl nav apstiprināta. Pārbaudi pastkasti vai nosūti saiti vēlreiz.
login-resend = Nosūtīt apstiprinājuma saiti vēlreiz
login-resending = Sūta…
login-resend-sent = E-pasts nosūtīts - pārbaudi pastkasti.

register-submit = Izveidot kontu
register-submitting = Izveido…
register-have-account = Jau ir konts?
register-password-placeholder = Vismaz 8 rakstzīmes
register-password-warning = Pieraksti savu paroli. Ja to aizmirsīsi, kontu nevarēs atjaunot.
register-check-email-title = Pārbaudi e-pastu
register-email-sent = E-pasts nosūtīts
register-email-sent-hint = Noklikšķini uz saites pastkastē, lai aktivizētu kontu.
register-not-received-prefix = Nesaņēmi? Pārbaudi mēstuļu mapi vai
register-sign-in-link = piesakies
register-not-received-suffix = lai nosūtītu saiti vēlreiz.
register-terms-prefix = Izveidojot kontu, tu piekrīti mūsu
register-terms-link = lietošanas noteikumiem
register-terms-and = un mūsu
register-privacy-link = privātuma politikai

settings-title = Iestatījumi
settings-preferences = Preferences
settings-preferences-local = Saglabātas šajā ierīcē.
settings-preferences-synced = Sinhronizētas ar tavu kontu, šifrētas.
settings-about = Par
settings-anonymous-title = Tu neesi pieteicies
settings-upsell-title = Tavi projekti katrā ierīcē
settings-upsell-free = Bez maksas
settings-upsell-body = Counted darbojas bez konta. Ar bezmaksas kontu tavi projekti un preferences seko tev uz tālruni, klēpjdatoru un tīmekli - joprojām šifrēti, joprojām mums nelasāmi.
settings-locked-badge = Konts
settings-locked-friends = Izveido kontu, lai pievienotu draugus un uzaicinātu viņus projektā no lietotnes - bez saites pārsūtīšanas.
settings-locked-payment-methods = Saglabā savu IBAN vai maksājumu lietotni vienreiz un kopīgo to ar izvēlētajiem projektiem. Kas tev ir parādā, to redz blakus tavam vārdam.
settings-friends-hint = Pievieno draugus un uzaicini viņus savos projektos, nekopīgojot saiti.

account-member-since = Dalībnieks kopš
account-logout = Izrakstīties
account-logging-out = Izrakstās…
account-delete-title = Dzēst manu kontu
account-delete-warning = Tūlītēji un neatgriezeniski, bez atkritnes. Izdevumi, ko ievadīji kopīgā projektā, paliek redzami pārējiem dalībniekiem - tie ir daļa no viņu uzskaites.
account-delete-confirm-title = Dzēst kontu
account-delete-confirm-message = Tavs konts, sesijas un projektu saraksts tiks neatgriezeniski dzēsti. Bez paroles kopīgā projekta šifrētie dati tev kļūs nelasāmi - to nevar atsaukt.

settings-payment-methods = Maksājumu dati
settings-payment-methods-hint = Kā vēlies saņemt naudu atpakaļ. Šifrēti ar tavu kontu.
payment-method-kind = Veids
payment-method-kind-other = Cits
payment-method-label = Nosaukums
payment-method-label-placeholder = Galvenais konts
payment-method-value = Dati
payment-method-value-placeholder = IBAN, tālruņa numurs, lietotājvārds…
payment-method-add = Pievienot
payment-method-remove = Noņemt { $name }
payment-method-empty = Vēl neesi pievienojis maksājumu datus.
payment-method-deleted = Maksājuma veids dzēsts.
payment-method-value-required = Aizpildi katra maksājuma veida datus vai noņem to.
payment-method-label-required = Dod savam pielāgotajam veidam nosaukumu.
payment-method-too-long = Tas ir par garu - saīsini.
payment-method-invalid-characters = Noņem rindu pārtraukumus vai neredzamas rakstzīmes.
payment-method-limit = Vari saglabāt līdz { $max } maksājumu veidiem.
payment-methods-saved = Maksājumu dati saglabāti.
payment-methods-offline = Lai saglabātu maksājumu datus, jābūt tiešsaistē.
payment-methods-stale = Tavi maksājumu dati tika mainīti citā ierīcē. Tie ir pārlādēti — mēģini vēlreiz.
payment-methods-key-missing = Piesakies vēlreiz, lai pārvaldītu maksājumu datus.
settings-payment-methods-share-warning = Kopīgots veids ir redzams katram to projektu dalībniekam, kuros esi izvēlējies savu vārdu - ikvienam, kam ir kāda no šīm saitēm.
payment-method-share = Kopīgot ar maniem projektiem
payment-method-share-hint = Parādās blakus tavam vārdam, kad kāds tev ir parādā.
payment-method-copy = Kopēt { $name }
payment-method-copied = Nokopēts.
payment-method-copy-failed = Neizdevās nokopēt - iezīmē tekstu un nokopē to manuāli.

verify-email-checking = Apstiprina e-pasta adresi…
verify-email-welcome = E-pasts apstiprināts - laipni lūgts Counted!
verify-email-back-to-login = Atpakaļ uz pieteikšanos

### Project status

project-close = Slēgt
project-archive = Arhivēt
project-reopen = Atvērt no jauna
project-unarchive = Atjaunot no arhīva

### Dates

date-long = { $year }. gada { $day }. { $month }

month-1 = janvāris
month-2 = februāris
month-3 = marts
month-4 = aprīlis
month-5 = maijs
month-6 = jūnijs
month-7 = jūlijs
month-8 = augusts
month-9 = septembris
month-10 = oktobris
month-11 = novembris
month-12 = decembris

month-short-1 = janv.
month-short-2 = febr.
month-short-3 = marts
month-short-4 = apr.
month-short-5 = maijs
month-short-6 = jūn.
month-short-7 = jūl.
month-short-8 = aug.
month-short-9 = sept.
month-short-10 = okt.
month-short-11 = nov.
month-short-12 = dec.

### Actions

add = Pievienot
create = Izveidot
creating = Izveido…
edit = Rediģēt
leave = Pamest
close = Aizvērt
paste = Ielīmēt
join = Pievienoties
import = Importēt
importing = Importē…
field-description = Apraksts
field-date = Datums
date-today = Šodien
date-yesterday = Vakar
field-optional = Neobligāti

### Projects

projects-filter-active = Aktīvie
projects-filter-all = Visi
projects-count-label = Projekti
projects-empty = Nav projektu
projects-empty-hint = Izveido projektu ar pogu zemāk
projects-offline-banner = Bezsaistes dati - pieslēdzies vēlreiz, lai atjauninātu.
projects-no-local-data = Nav lokālu datu
projects-no-local-data-hint = Piesakies, lai pirmo reizi ielādētu savus projektus.
projects-add = Pievienot projektu
projects-create = Izveidot projektu
projects-join = Pievienoties projektam
projects-import-tricount = Importēt no Tricount
project-actions = Projekta darbības

status-ongoing = Notiek
status-closed = Slēgts
status-archived = Arhivēts

nav-help = Palīdzība
nav-privacy = Privātuma politika
nav-terms = Lietošanas noteikumi
nav-legal = Juridiskā informācija

leave-project-title = Pamest projektu?
leave-project-message = Tu zaudēsi piekļuvi no šīs ierīces. Ja nepaliks neviens dalībnieks, projekts un visi tā izdevumi tiks neatgriezeniski dzēsti.

add-project-title = Jauns projekts
add-project-name-label = Projekta nosaukums
add-project-name-placeholder = Mans ceļojums, Dzīvoklis 2024…
add-project-participants = Dalībnieki
add-project-participant-name = Dalībnieka vārds
add-project-participant-placeholder = Clark Kent
add-project-remove-participant = Noņemt dalībnieku
add-project-me-badge = Es
add-project-thats-me = Tas esmu es!
add-project-offline = Bezsaistē nevar izveidot projektu. Pieslēdzies vēlreiz un mēģini atkal.
add-project-name-required = Projektam nepieciešams nosaukums.
add-project-need-two-participants = Pievieno vismaz 2 dalībniekus.
add-project-pick-yourself = Norādi, kurš dalībnieks esi tu.

join-link-label = Kopīgošanas saite
join-link-hint = Saite satur atšifrēšanas atslēgu - nokopē to visu.
join-invalid-link = Šī saite nav derīga. Ielīmē visu kopīgošanas saiti, ieskaitot daļu aiz #.
join-wrong-project = Šī saite ir citam projektam.

import-tricount-link-label = Tricount saite vai atslēga
import-tricount-key-required = Ievadi Tricount saiti vai atslēgu.
import-tricount-encryption-failed = Šifrēšana neizdevās.

### Expenses

save = Saglabāt
saving = Saglabā…
adding = Pievieno…
link-copied = Saite nokopēta
missing-encryption-key = Trūkst šifrēšanas atslēgas.
missing-encryption-key-title = Trūkst šifrēšanas atslēgas
missing-encryption-key-hint = Izmantotajā saitē nav atslēgas, kas nepieciešama šī projekta atšifrēšanai. Izmanto pilno saiti, ko kopīgoja projekta veidotājs.
project-locked-hint = Šai ierīcei nav šī projekta atslēgas. Atver tā kopīgošanas saiti, lai to atbloķētu.
project-unlock = Atbloķēt
project-no-local-data-hint = Piesakies, lai pirmo reizi ielādētu šī projekta datus.
project-gone-title = Šis projekts vairs nepastāv
project-gone-hint = Tas tika dzēsts, kad pēdējais dalībnieks to pameta. Kopīgošanas saite vairs nedarbojas, pat ja to atver vēlreiz.

expense-add = Pievienot izdevumu
transfer-add = Pievienot pārskaitījumu
expense-edit-title = Rediģēt izdevumu
expense-category = Kategorija
expense-category-auto = Auto · { $emoji }
expense-currency = Summas valūta
amount-op-add = Plus
amount-op-subtract = Mīnus
amount-op-multiply = Reizināt
amount-op-divide = Dalīt
amount-op-equals = Vienāds ar
amount-op-done = Gatavs
expense-rate = Valūtas kurss (neobligāti)
expense-rate-hint = Atstāj tukšu, lai izmantotu Eiropas Komisijas (InforEuro) kursu par { $month }: 1 { $from } = { $rate } { $to }.
expense-rate-invalid = Ievadi valūtas kursu, kas lielāks par 0.
expense-rate-unavailable = Automātiskais kurss nav pieejams - ievadi to manuāli.
expense-delete-title = Dzēst izdevumu
expense-delete-message = „{ $name }” tiks neatgriezeniski dzēsts. To nevar atsaukt.
expense-inconsistent-amounts = Summas nesakrīt
expenses-empty = Nav izdevumu
expenses-empty-hint = Sāc, pievienojot izdevumus ar pogu zemāk
expenses-show-more = Rādīt vairāk (atlikuši { $count })

expense-type-expense = Izdevums
expense-type-transfer = Pārskaitījums
expense-type-gain = Ieņēmums
expense-paid-by = maksāja
expense-sent-by = nosūtīja
expense-contributed-by = ieguldīja

expense-name-required = Nosaukums ir obligāts.
expense-amount-not-positive = Summai jābūt lielākai par 0.
expense-no-payer = Izvēlies vismaz vienu maksātāju.
expense-no-debtor = Izvēlies vismaz vienu personu, kas ir parādā.
expense-invalid-date = Šis datums nav derīgs.
expense-payers-mismatch = Maksātāju summa ir { $sum }, kas nesakrīt ar izdevuma summu ({ $total }).
expense-debtors-mismatch = Parādnieku summa ir { $sum }, kas nesakrīt ar izdevuma summu ({ $total }).

participants-none = Neviens
participants-everyone = Visi ({ $count })
participants-some = { $count } no { $total }
participants-select-all = Atlasīt visus
participants-by-shares = Pēc daļām
split-amounts = Summas
participants-remaining = Atlicis { $amount }
participants-over-by = Par { $amount } vairāk
participants-who-paid = Kas maksāja?
participants-who-received = Kas saņēma?
participants-who-transfers = Kas pārskaita?
participants-who-receives = Kas saņem?
participants-for-whom = Kam?

stats-total-expenses = Kopējie izdevumi
stats-my-expenses = Mani izdevumi

tab-expenses = Izdevumi
tab-balance = Bilance
tab-reimbursements = Norēķini
reimbursements-empty-title = Viss norēķināts!
reimbursements-empty-hint = Norēķinu ieteikumi parādās šeit, kad konti nav līdzsvarā
reimbursement-owes = { $debtor } ir parādā { $creditor }
reimbursement-record = Norēķināties
reimbursement-pay-with = Maksāt
reimbursement-pay-shared-by = Kopīgoja { $name } - pirms sūtīšanas pārbaudi saņēmēja vārdu, ko rāda tava lietotne.
reimbursement-pay-title = Maksāt { $name }
reimbursements-mine-title = Tu esi parādā
reimbursements-others-title = Citas atmaksas
copy = Kopēt

user-selection-title = Kurš dalībnieks esi tu?
user-selection-hint = Izvēlies savu vārdu no saraksta.
user-selection-required = Lūdzu, izvēlies dalībnieku.
identity-claimed = Saistīts ar kontu
identity-claimed-by = { $name } konts
identity-taken-repick = Cits konts ir piesavinājies dalībnieku, kuru tu izmantoji. Lūdzu, izvēlies citu.
participant-gone-repick = Dalībnieks, kuru tu izmantoji, ir noņemts no šī projekta. Lūdzu, izvēlies citu.

edit-project-title = Rediģēt projektu
edit-project-new-badge = jauns
edit-project-deferred-new-members = jaunu dalībnieku pievienošana
edit-project-deferred-removals = dalībnieku noņemšana
edit-project-deferred-me = izvēle „Tas esmu es”
edit-project-offline-deferred = Bezsaistē: { $items } tiks piemērots pēc pieslēgšanās.

export-saved = Fails saglabāts:
    { $path }
export-failed = Eksports neizdevās: { $reason }

history-expense-added = Izdevums pievienots: { $name }
history-expense-edited = Izdevums rediģēts: { $name }
history-expense-deleted = Izdevums dzēsts: { $name }
history-project-edited = Projekts rediģēts: { $name }
history-name-changed = Nosaukums: „{ $from }” → „{ $to }”
history-description-added = Apraksts pievienots: „{ $value }”
history-description-removed = Apraksts noņemts: „{ $value }”
history-description-changed = Apraksts: „{ $from }” → „{ $to }”

### Sweep

field-amount = Summa
expense-name-placeholder = Restorāns, pārtika…
expense-actions = Izdevuma darbības
expense-your-share = Tava daļa
expense-your-share-value = Tava daļa: { $amount } { $currency }
expense-inconsistent-detail = Summas nesakrīt: samaksāts { $paid }, parādā { $owed }, izdevumam { $total }. Rediģē izdevumu, lai to labotu.
missing-access-key = Trūkst piekļuves atslēgas. Atver šo projektu caur tā kopīgošanas saiti.
filter-all = Visi
filter-my-payments = Mani maksājumi
filter-my-debts = Ko esmu parādā
participants-shares-for = { $name } daļas
participants-amount-for = { $name } summa
reimbursement-add = Pievienot norēķinu
project-forget = Noņemt no mana saraksta
project-history-title = Vēsture
history-kind-add = Pievienots
history-kind-delete = Dzēsts
history-kind-edit = Rediģēts
export = Eksportēt
export-json = Eksportēt JSON
export-csv = Eksportēt CSV
share-link = Kopīgot
copy-link-failed = Neizdevās nokopēt saiti
open-in-app = Atvērt lietotnē
not-found-title = Lapa nav atrasta
not-found-back = Atpakaļ uz projektiem

### Charts

charts-period = Periods
period-all = Viss
period-month = Mēnesis
period-3months = 3 mēn.
period-year = Gads
period-custom = Pielāgots
charts-tab-categories = Kategorijas
charts-tab-per-person = Uz personu
charts-tab-trends = Tendences
charts-by-category = Sadalījums pa kategorijām
charts-per-person = Izdevumi uz personu
charts-categories-by-month = Kategorijas pa mēnešiem
charts-total-spent = Kopā iztērēts
charts-avg-per-person = Vid. uz personu
charts-expense-count =
    { $count ->
        [zero] { $count } izdevumu
        [one] { $count } izdevums
       *[other] { $count } izdevumi
    }
charts-clear-category-filter = Notīrīt kategorijas filtru
charts-no-expenses = Nav izdevumu.
charts-pick-a-project = Izvēlies projektu, lai redzētu izdevumus uz personu.
charts-nothing-to-show = Nav ko rādīt
charts-my-share-note = Šīs summas ir tava daļa katrā izdevumā.
charts-my-share-skipped =
    { $count ->
        [zero] { $count } projektu nav ieskaitīti — nav izvēlēts dalībnieks vai to dati neielādējās.
        [one] { $count } projekts nav ieskaitīts — nav izvēlēts dalībnieks vai tā dati neielādējās.
       *[other] { $count } projekti nav ieskaitīti — nav izvēlēts dalībnieks vai to dati neielādējās.
    }

### Categories

category-food = Ēdiens
category-transport = Transports
category-accommodation = Naktsmītne
category-leisure = Atpūta
category-shopping = Iepirkšanās
category-services = Pakalpojumi
category-parties-gifts = Ballītes un dāvanas
category-other = Cits
charts-person = Persona
charts-project = Projekts
charts-all-projects = Visi projekti
charts-whole-project = Viss projekts
charts-date-from = No
charts-date-to = Līdz
charts-total = Kopā
charts-payments-per-person-by-month = Maksājumi uz personu pa mēnešiem
history-empty = Nav notikumu
history-by = { $name }
not-found-hint = Šī lapa nepastāv vai ir pārvietota.
payers-title-paid-by = Maksāja
payers-title-sender = Sūtītājs
payers-title-contributors = Ieguldītāji
debtors-title-debtors = Parādā
debtors-title-recipients = Saņēmēji
debtors-title-beneficiaries = Labuma guvēji

### Welcome

welcome-title = Tava uzskaite nav neviena cita darīšana.
welcome-subtitle = Dali izdevumus ar draugiem.
welcome-e2ee-title = Viss šifrēts
welcome-e2ee-body = Vārdi, summas, projekti: viss tiek šifrēts tavā ierīcē. Atslēga ir tikai tev. Neviens nevar lasīt tavus rēķinus. Pat ne mēs.
welcome-e2ee-note = Nelasāms pat mums (serverim nav piekļuves)
welcome-eu-title = 100 % eiropeisks
welcome-eu-body = Serveri Vācijā, e-pasti sūtīti no Francijas. Tavi dati nekad nepamet Eiropas Savienību.
welcome-noads-title = Bez reklāmām. Bez izsekotājiem.
welcome-noads-body = Mēs neko nevācam un nepārdodam tavus datus. Tas nav mūsu modelis.
welcome-start = Sākt
welcome-how-it-works = Kā tieši tas darbojas?

### Help

help-intro = Bieži uzdots jautājums? Pieskaries, lai izvērstu atbildi.
help-create-project-q = Kā izveidot projektu?
help-create-project-a = Sākuma ekrānā pieskaries pogai + apakšā. Dod projektam nosaukumu, izvēlies valūtu, un gatavs.
help-add-participants-q = Kā pievienot dalībniekus?
help-add-participants-a = Atver projektu un pievieno dalībniekus no dalībnieku saraksta. Katrs dalībnieks var maksāt par izdevumu vai būt par to parādā.
help-share-project-q = Kā kopīgot projektu?
help-share-project-a = Kopīgo projekta URL (to, kas adreses joslā). Ikviens ar saiti var skatīt un rediģēt projektu.
help-add-expense-q = Kā pievienot izdevumu?
help-add-expense-a = Projektā pieskaries +, ievadi summu, norādi, kas maksāja un starp ko sadalīt. Vari arī izvēlēties citu datumu, nevis šodienu.
help-types-q = Kāda ir atšķirība starp izdevumu, pārskaitījumu un ieņēmumu?
help-types-expense = - pirkums, ko veica viena persona un kas sadalīts starp vairākām.
help-types-transfer = - atmaksa no vienas personas otrai, bez sadalīšanas.
help-types-gain = - saņemta nauda (atmaksa, dāvana), ko sadala starp vairākām personām.
help-past-date-q = Vai varu datēt izdevumu pagātnē?
help-past-date-a = Jā, datuma lauks ir brīvs. Ieraksta izveides laiks tiek glabāts atsevišķi.
help-who-owes-q = Kā Counted aprēķina, kurš kam ir parādā?
help-who-owes-a = Counted aprēķina katra dalībnieka neto bilanci (cik samaksāja mīnus cik ir parādā), tad piedāvā īsāko pārskaitījumu virkni, kas norēķina visus.
help-minimal-transfers-q = Kāpēc ieteikto pārskaitījumu skaits ir minimāls?
help-minimal-transfers-a = Algoritms vispirms savieno bilances, kas precīzi izlīdzinās, tad apstrādā pārējās no lielākā kreditora līdz lielākajam parādniekam. Rezultāts: mazāk pārskaitījumu, lai visu norēķinātu.
help-import-tricount-q = Kā importēt projektu no Tricount?
help-import-tricount-a = Sākuma ekrānā pieskaries pogai „+” apakšā un tad
help-import-tricount-b = Ielīmē importējamā Tricount kopīgošanas saiti.
help-encryption-q = Vai mani dati ir šifrēti?
help-encryption-a = Jā. Counted apvieno divas garantijas:
help-encryption-e2ee-term = Pilnīga šifrēšana
help-encryption-e2ee-def = - viss starp tevi un serveri ceļo šifrēts.
help-encryption-zero-term = Nulles piekļuve
help-encryption-zero-def = - tu šifrē datus pirms nosūtīšanas, un serveris glabā tikai šifrētu tekstu. Mums nav iespējas to izlasīt.
help-encryption-see = Sīkāk skati
help-forgot-password-q = Kas notiks, ja aizmirsīšu paroli?
help-forgot-password-warning = Tavi dati tiks neatgriezeniski zaudēti.
help-forgot-password-a = Šifrēšanas atslēga tiek atvasināta no tavas paroles, tāpēc atiestatīšana nav iespējama: neviens - arī mēs ne - nevar atšifrēt tavus projektus bez tās. Glabā to drošībā, ideāli paroļu pārvaldniekā.
help-archive-delete-q = Kā arhivēt vai dzēst projektu?
help-archive-delete-a = Projekta ekrānā atver izvēlni un izvēlies
help-archive-delete-b = lai to paslēptu, bet saglabātu. Projekts tiek neatgriezeniski dzēsts, kad to pamet pēdējais dalībnieks.
help-delete-account-q = Kā dzēst savu kontu?
help-delete-account-a = Atver Iestatījumus un izmanto „Dzēst manu kontu”. Tas notiek tūlītēji un nav atsaucams.
help-contact = Cits jautājums? Raksti mums uz

# Receipt scanning (mobile only)
expense-scan = Skenēt čeku
scan-in-progress = Nolasa čeku…
scan-error-capture = Neizdevās uzņemt fotoattēlu. Mēģini vēlreiz vai ievadi izdevumu manuāli.
scan-error-unreadable = Šajā čekā nav nekā salasāma. Ievadi izdevumu manuāli.
scan-check-amount = Pārbaudi kopsummu - tā nebija skaidri nodrukāta.
scan-take-photo = Uzņemt fotoattēlu
scan-choose-photo = Izvēlēties fotoattēlu
expense-converted-from = Samaksāts { $amount } { $from } · 1 { $from } = { $rate } { $to }
project-currency = Valūta
project-currency-hint = Visas summas tiek rādītas šajā valūtā. To vēlāk nevar mainīt.
project-currency-locked = Valūta tiek noteikta, izveidojot projektu.

update-required-title = Nepieciešams atjauninājums
update-required-body = Šī Counted versija ir pārāk veca, lai sazinātos ar serveri. Atjaunini to, lai turpinātu lietot lietotni.
update-required-button = Atjaunināt

notifications-label = Paziņojumi
notifications-title = Paziņojumi
notifications-empty = Nekā jauna
notifications-friend-request = Draudzības pieprasījums

friends-title = Draugi
friends-anonymous-body = Draugi tiek glabāti kopā ar tavu kontu. Piesakies, lai pievienotu cilvēkus un uzaicinātu viņus savos projektos, nekopīgojot saiti.
friends-add-title = Pievienot draugu
friends-add-hint = Viņš redzēs tavu pieprasījumu, kad pieteiksies. Neviens no jums neuzzina, vai otram ir konts, kamēr pieprasījums nav pieņemts.
friends-add-button = Pievienot
friends-add-from-project = Pievienot kā draugu
friends-request-sent = Pieprasījums nosūtīts
friends-no-account-key = Piesakies vēlreiz, lai pārvaldītu draugus šajā ierīcē.
friends-incoming-title = Pieprasījumi
friends-accept = Pieņemt
friends-decline = Noraidīt
friends-list-title = Mani draugi
friends-list-empty = Vēl nav draugu. Pievieno kādu pa e-pastu augstāk vai no kopīga projekta.
friends-remove = Noņemt
friends-remove-confirm-title = Noņemt draugu
friends-remove-confirm-message = { $email } vairs nebūs starp jūsu draugiem, un jūs – starp viņa. Jebkurš no jums vēlāk var nosūtīt jaunu pieprasījumu.
friends-no-key = Vēl nav gatavs
friends-fingerprint = Drošības kods
friends-fingerprint-hint = Divi draugi, kas viens otram nolasa vienādu drošības kodu, zina, ka starp viņiem neviena nav - pat ne mūsu servera.
friends-outgoing-title = Nosūtītie
friends-outgoing-hint = Gaida atbildi. Redzēsi viņus starp draugiem, kad viņi pieņems.
friends-withdraw = Atcelt
invite-friends-title = Uzaicināt draugus
invite-friends-hint = Projekta atslēga tiek šifrēta katram draugam šajā ierīcē. Serveris to nekad neredz.
invite-friends-empty = Vēl nav draugu, ko uzaicināt.
invite-friends-button = Uzaicināt
invite-sent = { $count ->
    [zero] Nosūtīti { $count } uzaicinājumu
    [one] Uzaicinājums nosūtīts
   *[other] Nosūtīti { $count } uzaicinājumi
}
invitation-badge = Uzaicinājums
invitation-to = Pievienoties „{ $name }”
invitation-to-unnamed = Pievienoties projektam
invitation-unreadable = Šo uzaicinājumu nevar atvērt šajā ierīcē
invitation-from = No { $email }
invitation-accept = Pievienoties
invitation-decline = Noraidīt
