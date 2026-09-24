# Malti. Sħiħ ħlief għat-testi legali (legal-, terms-, privacy-), li jeżistu biss bl-Ingliż u bil-Franċiż
# u jaqgħu lura fuq en.ftl messaġġ messaġġ.

### Common

loading = Qed jitgħabba…
cancel = Ikkanċella
confirm = Ikkonferma
retry = Erġa' pprova
delete = Ħassar
back = Lura
language = Lingwa

### Navigation

nav-main = Navigazzjoni prinċipali
nav-projects = Proġetti
nav-charts = Statistika
nav-settings = Settings

### Connectivity

offline-banner = Offline
offline-pending =
    { $count ->
        [one] { $count } pendenti
        [few] { $count } pendenti
        [many] { $count } pendenti
       *[other] { $count } pendenti
    }

sync-conflict-edit = Kunflitt: l-editjar ta' “{ $name }” falla (element imħassar). Maqbuż.
sync-conflict-delete = Kunflitt: it-tħassir ta' “{ $name }” falla (element imħassar). Maqbuż.
sync-conflict-other = Kunflitt: l-operazzjoni fuq “{ $name }” falliet (element imħassar). Maqbuż.
sync-error = Żball ta' sinkronizzazzjoni: { $reason }

### Errors

error-network = Ma nistgħux nilħqu s-server. Iċċekkja l-konnessjoni tal-internet tiegħek.
error-generic = Xi ħaġa marret ħażin. Jekk jogħġbok erġa' pprova.

error-invalid-email = Dak l-indirizz tal-email mhux validu.
error-invalid-password = Dik il-password mhix valida.
error-password-too-short = Il-password trid tkun tal-anqas 8 karattri.
error-client-outdated = Din il-verżjoni tal-app skadiet. Aġġornaha biex tidħol.
error-invalid-link = Din il-link mhix valida.
error-batch-too-large = Wisq elementi f'daqqa.
error-payers-required = Agħżel tal-anqas persuna waħda li ħallset.
error-debtors-required = Agħżel tal-anqas persuna waħda li għandha tagħti.
error-duplicate-participant = Parteċipant jidher darbtejn fuq l-istess naħa.
error-participant-not-in-project = Dak il-parteċipant mhux parti minn dan il-proġett.
error-too-many-participants = Wisq parteċipanti għal spiża waħda.
error-invalid-credentials = Email jew password ħażina.
error-unauthenticated = Idħol biex tagħmel dan.
error-email-not-verified = L-indirizz tal-email tiegħek għadu mhux ivverifikat.
error-project-not-found = Dan il-proġett m'għadux jeżisti.
error-expense-not-found = Din l-ispiża m'għadhiex teżisti.
error-user-not-found = Dan il-parteċipant m'għadux jeżisti.
error-tricount-not-found = Tricount ma nstabx, jew l-API tiegħu ta żball.
error-too-many-members = Dan il-proġett laħaq il-limitu ta' membri.
error-identity-taken = Kont ieħor diġà ħa dan il-parteċipant.
error-claim-proof-invalid = Dan l-apparat m'għandux iċ-ċavetta tal-proġett, għalhekk ma jistax jieħu parteċipant. Erġa' iftaħ il-link tal-qsim.
error-user-has-payments = Dan il-parteċipant għandu spejjeż fil-proġett u ma jistax jitneħħa.
error-resend-cooldown = Stenna 60 sekonda qabel titlob email oħra.
error-self-friend-request = Ma tistax iżżid lilek innifsek bħala ħabib.
error-not-a-friend = Tista' tistieden biss nies mil-lista tal-ħbieb tiegħek.
error-friend-has-no-key = Dan il-ħabib għadu ma fetaħx l-aħħar verżjoni tal-app. Itolbu jidħol darba, imbagħad erġa' pprova.
error-friend-request-not-found = Din it-talba ta' ħbiberija m'għadhiex teżisti.
error-invitation-not-found = Din l-istedina m'għadhiex teżisti.
error-too-many-friend-requests = Wisq talbiet ta' ħbiberija għalissa. Erġa' pprova għada.
error-too-many-invitations = Wisq stediniet pendenti.

### Auth

field-email = Email
field-email-placeholder = int@ezempju.mt
field-password = Password
field-name = Isem
field-name-placeholder = Marija Borg

login-title = Idħol
login-submit = Idħol
login-submitting = Qed tidħol…
login-password-placeholder = Il-password tiegħek
login-no-account = M'għandekx kont għadu?
login-unverified = L-indirizz tal-email tiegħek għadu mhux ivverifikat. Iċċekkja l-inbox tiegħek, jew ibgħat il-link mill-ġdid.
login-resend = Ibgħat il-link ta' verifika mill-ġdid
login-resending = Qed jintbagħat…
login-resend-sent = Email mibgħuta - iċċekkja l-inbox tiegħek.

register-submit = Oħloq kont
register-submitting = Qed jinħoloq…
register-have-account = Diġà għandek kont?
register-password-placeholder = Tal-anqas 8 karattri
register-password-warning = Ikteb il-password tiegħek. Jekk tinsieha, il-kont tiegħek ma jistax jiġi rkuprat.
register-check-email-title = Iċċekkja l-email tiegħek
register-email-sent = Email mibgħuta
register-email-sent-hint = Ikklikkja l-link fl-inbox tiegħek biex tattiva l-kont.
register-not-received-prefix = Ma rċevejthiex? Iċċekkja l-folder tal-ispam, jew
register-sign-in-link = idħol
register-not-received-suffix = biex tibgħat il-link mill-ġdid.
register-terms-prefix = Billi toħloq kont taċċetta
register-terms-link = it-termini tal-użu
register-terms-and = u
register-privacy-link = l-politika tal-privatezza tagħna

settings-title = Settings
settings-preferences = Preferenzi
settings-preferences-local = Maħżuna fuq dan l-apparat.
settings-preferences-synced = Sinkronizzati mal-kont tiegħek, kriptati.
settings-about = Dwar
settings-anonymous-title = M'intix imdaħħal
settings-upsell-title = Il-proġetti tiegħek, fuq kull apparat
settings-upsell-free = B'xejn
settings-upsell-body = Counted jaħdem mingħajr kont. B'kont b'xejn, il-proġetti u l-preferenzi tiegħek isegwuk fuq il-mowbajl, il-laptop u l-web - xorta kriptati, xorta ma jinqrawx minna.
settings-locked-badge = Kont
settings-locked-friends = Oħloq kont biex iżżid ħbieb u tistedinhom fi proġett mill-app - mingħajr link x'tgħaddi.
settings-locked-payment-methods = Aħżen l-IBAN jew l-app tal-ħlas tiegħek darba u aqsamha mal-proġetti li tagħżel. Min għandu jagħtik jaraha ħdejn ismek.
settings-friends-hint = Żid ħbieb u stedinhom fil-proġetti tiegħek mingħajr ma taqsam link.

account-member-since = Membru minn
account-logout = Oħroġ
account-logging-out = Qed toħroġ…
account-delete-title = Ħassar il-kont tiegħi
account-delete-warning = Immedjat u permanenti, mingħajr recycle bin. L-ispejjeż li daħħalt fi proġett maqsum jibqgħu viżibbli għall-membri l-oħra - huma parti mill-kontijiet tagħhom.
account-delete-confirm-title = Ħassar il-kont
account-delete-confirm-message = Il-kont, is-sessjonijiet u l-lista tal-proġetti tiegħek se jitħassru b'mod permanenti. Mingħajr il-password tiegħek, id-data kriptata ta' proġett maqsum issir ma tinqarax għalik - dan ma jistax jitreġġa' lura.

settings-payment-methods = Dettalji tal-ħlas
settings-payment-methods-hint = Kif tixtieq titħallas lura. Kriptati mal-kont tiegħek.
payment-method-kind = Metodu
payment-method-kind-other = Ieħor
payment-method-label = Isem
payment-method-label-placeholder = Kont prinċipali
payment-method-value = Dettalji
payment-method-value-placeholder = IBAN, numru tat-telefon, username…
payment-method-add = Żid
payment-method-remove = Neħħi { $name }
payment-method-empty = Għadek ma żidt l-ebda dettalji tal-ħlas.
payment-method-deleted = Metodu tal-ħlas imħassar.
payment-method-value-required = Imla d-dettalji ta' kull metodu tal-ħlas, jew neħħih.
payment-method-label-required = Agħti isem lill-metodu personalizzat tiegħek.
payment-method-too-long = Dak twil wisq - qassru.
payment-method-invalid-characters = Neħħi kull line break jew karattri inviżibbli.
payment-method-limit = Tista' taħżen sa { $max } metodi tal-ħlas.
payment-methods-saved = Dettalji tal-ħlas maħżuna.
payment-methods-offline = Trid tkun online biex taħżen id-dettalji tal-ħlas tiegħek.
payment-methods-stale = Id-dettalji tal-ħlas tiegħek inbidlu fuq apparat ieħor. Reġgħu tgħabbew — jekk jogħġbok erġa' pprova.
payment-methods-key-missing = Erġa' idħol biex timmaniġġja d-dettalji tal-ħlas tiegħek.
settings-payment-methods-share-warning = Metodu maqsum huwa viżibbli għal kull membru tal-proġetti fejn għażilt ismek - kull min għandu waħda minn dawk il-links.
payment-method-share = Aqsam mal-proġetti tiegħi
payment-method-share-hint = Jidher ħdejn ismek meta xi ħadd għandu jagħtik flus.
payment-method-copy = Ikkopja { $name }
payment-method-copied = Ikkupjat.
payment-method-copy-failed = Ma setax jiġi kkupjat - agħżel it-test u kkupjah manwalment.

verify-email-checking = Qed jiġi vverifikat l-indirizz tal-email tiegħek…
verify-email-welcome = Email ivverifikata - merħba f'Counted!
verify-email-back-to-login = Lura għad-dħul

### Project status

project-close = Agħlaq
project-archive = Arkivja
project-reopen = Erġa' iftaħ
project-unarchive = Neħħi mill-arkivju

### Dates

date-long = { $day } ta' { $month } { $year }

month-1 = Jannar
month-2 = Frar
month-3 = Marzu
month-4 = April
month-5 = Mejju
month-6 = Ġunju
month-7 = Lulju
month-8 = Awwissu
month-9 = Settembru
month-10 = Ottubru
month-11 = Novembru
month-12 = Diċembru

month-short-1 = Jan
month-short-2 = Fra
month-short-3 = Mar
month-short-4 = Apr
month-short-5 = Mej
month-short-6 = Ġun
month-short-7 = Lul
month-short-8 = Aww
month-short-9 = Set
month-short-10 = Ott
month-short-11 = Nov
month-short-12 = Diċ

### Actions

add = Żid
create = Oħloq
creating = Qed jinħoloq…
edit = Editja
leave = Itlaq
close = Agħlaq
paste = Waħħal
join = Ingħaqad
import = Importa
importing = Qed jiġi importat…
field-description = Deskrizzjoni
field-date = Data
field-optional = Fakultattiv

### Projects

projects-filter-active = Attivi
projects-filter-all = Kollha
projects-count-label = Proġetti
projects-empty = L-ebda proġett
projects-empty-hint = Oħloq proġett bil-buttuna t'hawn taħt
projects-offline-banner = Data offline - erġa' ikkonnettja biex taġġorna.
projects-no-local-data = L-ebda data lokali
projects-no-local-data-hint = Idħol biex tgħabbi l-proġetti tiegħek għall-ewwel darba.
projects-add = Żid proġett
projects-create = Oħloq proġett
projects-join = Ingħaqad ma' proġett
projects-import-tricount = Importa minn Tricount
project-actions = Azzjonijiet tal-proġett

status-ongoing = Għaddej
status-closed = Magħluq
status-archived = Arkivjat

nav-help = Għajnuna
nav-privacy = Politika tal-privatezza
nav-terms = Termini tal-użu
nav-legal = Avviż legali

leave-project-title = Titlaq mill-proġett?
leave-project-message = Se titlef l-aċċess minn dan l-apparat. Jekk ma jibqa' l-ebda membru, il-proġett u l-ispejjeż kollha tiegħu jitħassru b'mod permanenti.

add-project-title = Proġett ġdid
add-project-name-label = Isem il-proġett
add-project-name-placeholder = Il-vjaġġ tiegħi, Flatmates 2024…
add-project-participants = Parteċipanti
add-project-participant-name = Isem il-parteċipant
add-project-participant-placeholder = Clark Kent
add-project-remove-participant = Neħħi l-parteċipant
add-project-me-badge = Jien
add-project-thats-me = Dak jien!
add-project-offline = Ma tistax toħloq proġett offline. Erġa' ikkonnettja u erġa' pprova.
add-project-name-required = Il-proġett jeħtieġ isem.
add-project-need-two-participants = Żid tal-anqas 2 parteċipanti.
add-project-pick-yourself = Għidilna liema parteċipant int.

join-link-label = Link tal-qsim
join-link-hint = Il-link iġġorr iċ-ċavetta tad-dekriptaġġ - ikkopjaha kollha.
join-invalid-link = Dik il-link mhix valida. Waħħal il-link tal-qsim kollha, inkluża l-parti wara #.
join-wrong-project = Dik il-link hija għal proġett ieħor.

import-tricount-link-label = Link jew ċavetta ta' Tricount
import-tricount-key-required = Daħħal link jew ċavetta ta' Tricount.
import-tricount-encryption-failed = Il-kriptaġġ falla.

### Expenses

save = Aħżen
saving = Qed jinħażen…
adding = Qed jiżdied…
link-copied = Link ikkupjata
missing-encryption-key = Iċ-ċavetta tal-kriptaġġ nieqsa.
missing-encryption-key-title = Iċ-ċavetta tal-kriptaġġ nieqsa
missing-encryption-key-hint = Il-link li użajt ma ġġorrx iċ-ċavetta meħtieġa biex tiddekripta dan il-proġett. Uża l-link sħiħa maqsuma minn min ħalqu.
project-locked-hint = Dan l-apparat m'għandux iċ-ċavetta ta' dan il-proġett. Iftaħ il-link tal-qsim tiegħu biex tiftħu.
project-unlock = Iftaħ
project-no-local-data-hint = Idħol biex tgħabbi d-data ta' dan il-proġett għall-ewwel darba.
project-gone-title = Dan il-proġett m'għadux jeżisti
project-gone-hint = Tħassar meta l-aħħar membru tiegħu telaq. Il-link tal-qsim m'għadhiex taħdem, anke jekk terġa' tiftaħha.

expense-add = Żid spiża
transfer-add = Żid trasferiment
expense-edit-title = Editja l-ispiża
expense-category = Kategorija
expense-category-auto = Awto · { $emoji }
expense-currency = Munita tal-ammont
amount-op-add = Żid
amount-op-subtract = Naqqas
amount-op-multiply = Immultiplika
amount-op-divide = Aqsam
amount-op-equals = Ugwali
expense-rate = Rata tal-kambju (fakultattiva)
expense-rate-hint = Ħalli vojt biex tuża r-rata tal-Kummissjoni Ewropea (InforEuro) għal { $month }: 1 { $from } = { $rate } { $to }.
expense-rate-invalid = Daħħal rata tal-kambju akbar minn 0.
expense-rate-unavailable = L-ebda rata awtomatika disponibbli - daħħal waħda manwalment.
expense-delete-title = Ħassar l-ispiża
expense-delete-message = “{ $name }” se titħassar b'mod permanenti. Dan ma jistax jitreġġa' lura.
expense-inconsistent-amounts = L-ammonti ma jaqblux
expenses-empty = L-ebda spiża
expenses-empty-hint = Ibda billi żżid spejjeż bil-buttuna t'hawn taħt
expenses-show-more = Uri aktar ({ $count } fadal)

expense-type-expense = Spiża
expense-type-transfer = Trasferiment
expense-type-gain = Dħul
expense-paid-by = imħallsa minn
expense-sent-by = mibgħut minn
expense-contributed-by = kontribwit minn

expense-name-required = Isem huwa meħtieġ.
expense-amount-not-positive = L-ammont irid ikun akbar minn 0.
expense-no-payer = Agħżel tal-anqas persuna waħda li ħallset.
expense-no-debtor = Agħżel tal-anqas persuna waħda li għandha tagħti.
expense-invalid-date = Dik id-data mhix valida.
expense-payers-mismatch = Dawk li ħallsu jammontaw għal { $sum }, li ma jaqbilx mal-ammont tal-ispiża ({ $total }).
expense-debtors-mismatch = Id-debituri jammontaw għal { $sum }, li ma jaqbilx mal-ammont tal-ispiża ({ $total }).

participants-none = Ħadd
participants-everyone = Kulħadd ({ $count })
participants-some = { $count } minn { $total }
participants-select-all = Agħżel kollha
participants-deselect-all = Neħħi l-għażla
participants-by-shares = Skont l-ishma
participants-remaining = { $amount } fadal
participants-over-by = { $amount } żejjed
participants-who-paid = Min ħallas?
participants-who-received = Min irċieva?
participants-who-transfers = Min qed jittrasferixxi?
participants-who-receives = Min jirċievi?
participants-for-whom = Għal min?

stats-total-expenses = Spejjeż totali
stats-my-expenses = L-ispejjeż tiegħi

tab-expenses = Spejjeż
tab-balance = Bilanċ
tab-reimbursements = Saldu
reimbursements-empty-title = Kollox saldat!
reimbursements-empty-hint = Suġġerimenti ta' saldu jidhru hawn meta l-kontijiet ma jibbilanċjawx
reimbursement-owes = { $debtor } għandu jagħti lil { $creditor }
reimbursement-record = Salda
reimbursement-pay-with = Ħallas
reimbursement-pay-shared-by = Maqsum minn { $name } - iċċekkja l-isem tar-riċevitur li turi l-app tiegħek qabel tibgħat.
reimbursement-pay-title = Ħallas lil { $name }
reimbursements-mine-title = Int għandek tagħti
reimbursements-others-title = Rimborżi oħra
copy = Ikkopja

user-selection-title = Liema parteċipant int?
user-selection-hint = Agħżel ismek mil-lista.
user-selection-required = Jekk jogħġbok agħżel parteċipant.
identity-claimed = Marbut ma' kont
identity-claimed-by = Il-kont ta' { $name }
identity-taken-repick = Kont ieħor ħa l-parteċipant li kont qed tuża. Jekk jogħġbok agħżel ieħor.
participant-gone-repick = Il-parteċipant li kont qed tuża tneħħa minn dan il-proġett. Jekk jogħġbok agħżel ieħor.

edit-project-title = Editja l-proġett
edit-project-new-badge = ġdid
edit-project-deferred-new-members = iż-żieda ta' membri ġodda
edit-project-deferred-removals = it-tneħħija ta' membri
edit-project-deferred-me = l-għażla “Dak jien”
edit-project-offline-deferred = Offline: { $items } se jiġi applikat meta terġa' tikkonnettja.

export-saved = Fajl maħżun:
    { $path }
export-failed = L-esportazzjoni falliet: { $reason }

history-expense-added = Spiża miżjuda: { $name }
history-expense-edited = Spiża editjata: { $name }
history-expense-deleted = Spiża mħassra: { $name }
history-project-edited = Proġett editjat: { $name }
history-name-changed = Isem: “{ $from }” → “{ $to }”
history-description-added = Deskrizzjoni miżjuda: “{ $value }”
history-description-removed = Deskrizzjoni mneħħija: “{ $value }”
history-description-changed = Deskrizzjoni: “{ $from }” → “{ $to }”

### Sweep

field-amount = Ammont
expense-name-placeholder = Restorant, groceries…
expense-actions = Azzjonijiet tal-ispiża
expense-your-share = Is-sehem tiegħek
expense-your-share-value = Is-sehem tiegħek: { $amount } { $currency }
expense-inconsistent-detail = L-ammonti ma jaqblux: { $paid } imħallas, { $owed } dovut, għal spiża ta' { $total }. Editja l-ispiża biex tikkoreġih.
missing-access-key = Iċ-ċavetta tal-aċċess nieqsa. Iftaħ dan il-proġett permezz tal-link tal-qsim tiegħu.
filter-all = Kollha
filter-my-payments = Il-ħlasijiet tiegħi
filter-my-debts = X'għandi nagħti
participants-shares-for = Ishma għal { $name }
participants-amount-for = Ammont għal { $name }
reimbursement-add = Żid saldu
project-forget = Neħħi mil-lista tiegħi
project-history-title = Storja
history-kind-add = Miżjud
history-kind-delete = Imħassar
history-kind-edit = Editjat
export = Esporta
export-json = Esporta JSON
export-csv = Esporta CSV
share-link = Aqsam
copy-link-failed = Il-link ma setgħetx tiġi kkupjata
open-in-app = Iftaħ fl-app
not-found-title = Paġna ma nstabitx
not-found-back = Lura għall-proġetti

### Charts

charts-period = Perjodu
period-all = Kollox
period-month = Xahar
period-3months = 3 xhur
period-year = Sena
period-custom = Personalizzat
charts-tab-categories = Kategoriji
charts-tab-per-person = Għal kull persuna
charts-tab-trends = Xejriet
charts-by-category = Tqassim skont il-kategorija
charts-per-person = Nefqa għal kull persuna
charts-categories-by-month = Kategoriji skont ix-xahar
charts-total-spent = Total minfuq
charts-avg-per-person = Medja għal kull persuna
charts-expense-count =
    { $count ->
        [one] { $count } spiża
        [few] { $count } spejjeż
        [many] { $count } spejjeż
       *[other] { $count } spejjeż
    }
charts-clear-category-filter = Neħħi l-filtru tal-kategorija
charts-no-expenses = L-ebda spiża.
charts-pick-a-project = Agħżel proġett biex tara n-nefqa għal kull persuna.
charts-nothing-to-show = Xejn x'juri
charts-my-share-note = Dawn iċ-ċifri huma s-sehem tiegħek minn kull spiża.
charts-my-share-skipped =
    { $count ->
        [one] { $count } proġett mhux magħdud — l-ebda parteċipant magħżul, jew id-data tiegħu ma tgħabbietx.
        [few] { $count } proġetti mhux magħduda — l-ebda parteċipant magħżul, jew id-data tagħhom ma tgħabbietx.
        [many] { $count } proġetti mhux magħduda — l-ebda parteċipant magħżul, jew id-data tagħhom ma tgħabbietx.
       *[other] { $count } proġetti mhux magħduda — l-ebda parteċipant magħżul, jew id-data tagħhom ma tgħabbietx.
    }

### Categories

category-food = Ikel
category-transport = Trasport
category-accommodation = Akkomodazzjoni
category-leisure = Divertiment
category-shopping = Xiri
category-services = Servizzi
category-parties-gifts = Festi u rigali
category-other = Oħrajn
charts-person = Persuna
charts-project = Proġett
charts-all-projects = Il-proġetti kollha
charts-whole-project = Il-proġett kollu
charts-date-from = Minn
charts-date-to = Sa
charts-total = Total
charts-payments-per-person-by-month = Ħlasijiet għal kull persuna skont ix-xahar
history-empty = L-ebda avveniment
history-by = Minn { $name }
not-found-hint = Din il-paġna ma teżistix, jew ġiet imċaqalqa.
payers-title-paid-by = Imħallsa minn
payers-title-sender = Mittent
payers-title-contributors = Kontributuri
debtors-title-debtors = Għandhom jagħtu
debtors-title-recipients = Riċevituri
debtors-title-beneficiaries = Benefiċjarji

### Welcome

welcome-title = Il-kontijiet tiegħek mhumiex affari ta' ħadd ieħor.
welcome-subtitle = Aqsam l-ispejjeż mal-ħbieb.
welcome-e2ee-title = Kollox kriptat
welcome-e2ee-body = Ismijiet, ammonti, proġetti: kollox jiġi kriptat fuq l-apparat tiegħek. Int biss għandek iċ-ċavetta. Ħadd ma jista' jaqra l-kontijiet tiegħek. Lanqas aħna.
welcome-e2ee-note = Ma jinqarax, anke minna (l-ebda aċċess mis-server)
welcome-eu-title = 100% Ewropew
welcome-eu-body = Servers fil-Ġermanja, emails mibgħuta minn Franza. Id-data tiegħek qatt ma toħroġ mill-Unjoni Ewropea.
welcome-noads-title = L-ebda reklam. L-ebda tracker.
welcome-noads-body = Ma niġbru xejn u ma nbigħux id-data tiegħek. Dak mhux il-mudell tagħna.
welcome-start = Ibda
welcome-how-it-works = Kif jaħdem, eżattament?

### Help

help-intro = Mistoqsija komuni? Mess biex tespandi t-tweġiba.
help-create-project-q = Kif noħloq proġett?
help-create-project-a = Mill-iskrin ewlieni, mess il-buttuna + fil-qiegħ. Agħti isem lill-proġett, agħżel il-munita tiegħu, u lest.
help-add-participants-q = Kif inżid parteċipanti?
help-add-participants-a = Iftaħ il-proġett, imbagħad żid parteċipanti mil-lista tal-membri. Kull parteċipant jista' jħallas jew ikollu jagħti fuq spiża.
help-share-project-q = Kif naqsam proġett?
help-share-project-a = Aqsam il-URL tal-proġett (dik fl-address bar tiegħek). Kull min għandu l-link jista' jara u jeditja l-proġett.
help-add-expense-q = Kif inżid spiża?
help-add-expense-a = Ġo proġett, mess +, daħħal l-ammont, għid min ħallas u bejn min taqsamha. Tista' wkoll tagħżel data differenti mil-lum.
help-types-q = X'inhi d-differenza bejn spiża, trasferiment u dħul?
help-types-expense = - xirja magħmula minn persuna waħda u maqsuma bejn diversi.
help-types-transfer = - ħlas lura minn persuna għal oħra, mingħajr qsim.
help-types-gain = - flus riċevuti (rifużjoni, rigal) biex jinqasmu bejn diversi persuni.
help-past-date-q = Nista' nagħti data fil-passat lil spiża?
help-past-date-a = Iva, il-kamp tad-data huwa liberu. Il-ħin tal-ħolqien tar-rekord jinżamm separatament.
help-who-owes-q = Kif Counted jaħdem min għandu jagħti x'hiex?
help-who-owes-a = Counted jikkalkula l-bilanċ nett ta' kull parteċipant (dak li ħallas minn qabel bit-tnaqqis ta' dak li għandu jagħti), imbagħad jipproponi l-iqsar serje ta' trasferimenti li ssalda lil kulħadd.
help-minimal-transfers-q = Għaliex in-numru ta' trasferimenti suġġeriti huwa minimu?
help-minimal-transfers-a = L-algoritmu l-ewwel iqabbel bilanċi li jikkanċellaw lil xulxin eżattament, imbagħad jaħdem fuq il-bqija mill-akbar kreditur għall-akbar debitur. Ir-riżultat: inqas trasferimenti biex kollox jiġi saldat.
help-import-tricount-q = Kif nimporta proġett minn Tricount?
help-import-tricount-a = Mill-iskrin ewlieni, mess il-buttuna “+” fil-qiegħ, imbagħad
help-import-tricount-b = Waħħal il-link tal-qsim tat-Tricount li trid timporta.
help-encryption-q = Id-data tiegħi kriptata?
help-encryption-a = Iva. Counted jgħaqqad żewġ garanziji:
help-encryption-e2ee-term = Kriptaġġ minn tarf sa tarf
help-encryption-e2ee-def = - kollox bejnek u s-server jivvjaġġa kriptat.
help-encryption-zero-term = L-ebda aċċess
help-encryption-zero-def = - int tikkripta d-data qabel tibgħatha, u s-server jaħżen biss test kriptat. M'għandna l-ebda mod kif naqrawh.
help-encryption-see = Għad-dettalji, ara
help-forgot-password-q = X'jiġri jekk ninsa l-password tiegħi?
help-forgot-password-warning = Id-data tiegħek tintilef b'mod permanenti.
help-forgot-password-a = Iċ-ċavetta tal-kriptaġġ hija derivata mill-password tiegħek, għalhekk l-ebda reset mhu possibbli: ħadd - lanqas aħna - ma jista' jiddekripta l-proġetti tiegħek mingħajrha. Żommha sigura, idealment f'password manager.
help-archive-delete-q = Kif narkivja jew inħassar proġett?
help-archive-delete-a = Mill-iskrin tal-proġett, iftaħ il-menu u agħżel
help-archive-delete-b = biex taħbih waqt li żżommu. Proġett jitħassar għal kollox ladarba l-aħħar membru tiegħu jitilqu.
help-delete-account-q = Kif inħassar il-kont tiegħi?
help-delete-account-a = Iftaħ Settings u uża “Ħassar il-kont tiegħi”. Huwa immedjat u ma jistax jitreġġa' lura.
help-contact = Mistoqsija oħra? Iktbilna fuq

# Receipt scanning (mobile only)
expense-scan = Skennja rċevuta
scan-in-progress = Qed tinqara l-irċevuta…
scan-error-capture = Ma stajniex nieħdu dik ir-ritratt. Erġa' pprova, jew daħħal l-ispiża manwalment.
scan-error-unreadable = Xejn ma jinqara fuq dik l-irċevuta. Daħħal l-ispiża manwalment.
scan-check-amount = Iċċekkja t-total - ma kienx stampat ċar.
scan-take-photo = Ħu ritratt
scan-choose-photo = Agħżel ritratt
expense-converted-from = Imħallas { $amount } { $from } · 1 { $from } = { $rate } { $to }
project-currency = Munita
project-currency-hint = Kull ammont jintwera f'din il-munita. Ma tistax tinbidel aktar tard.
project-currency-locked = Il-munita tiġi ffissata meta jinħoloq il-proġett.

update-required-title = Aġġornament meħtieġ
update-required-body = Din il-verżjoni ta' Counted hija qadima wisq biex titkellem mas-server. Aġġornaha biex tkompli tuża l-app.
update-required-body-testflight = Din il-verżjoni ta' Counted hija qadima wisq biex titkellem mas-server. Iftaħ TestFlight u installa l-aħħar verżjoni biex tkompli tuża l-app.
update-required-button = Aġġorna

notifications-label = Notifiki
notifications-title = Notifiki
notifications-empty = Xejn ġdid
notifications-friend-request = Talba ta' ħbiberija

friends-title = Ħbieb
friends-anonymous-body = Il-ħbieb jinżammu mal-kont tiegħek. Idħol biex iżżid nies u tistedinhom fil-proġetti tiegħek mingħajr ma taqsam link.
friends-add-title = Żid ħabib
friends-add-hint = Se jara t-talba tiegħek ladarba jidħol. Ħadd minnkom ma jkun jaf jekk l-ieħor għandux kont sakemm it-talba tiġi aċċettata.
friends-add-button = Żid
friends-add-from-project = Żid bħala ħabib
friends-request-sent = Talba mibgħuta
friends-no-account-key = Erġa' idħol biex timmaniġġja l-ħbieb tiegħek fuq dan l-apparat.
friends-incoming-title = Talbiet
friends-accept = Aċċetta
friends-decline = Irrifjuta
friends-list-title = Il-ħbieb tiegħi
friends-list-empty = L-ebda ħabib għadu. Żid lil xi ħadd bl-email hawn fuq, jew minn proġett li taqsmu.
friends-remove = Neħħi
friends-remove-confirm-title = Neħħi ħabib
friends-remove-confirm-message = { $email } ma jibqax fost il-ħbieb tiegħek, u int ma tibqax fost tiegħu. Kull wieħed minnkom jista' jibgħat talba ġdida aktar tard.
friends-no-key = Għadu mhux lest
friends-fingerprint = Kodiċi ta' sigurtà
friends-fingerprint-hint = Żewġ ħbieb li jaqraw lil xulxin l-istess kodiċi ta' sigurtà jafu li ħadd ma hemm bejniethom - lanqas is-server tagħna.
friends-outgoing-title = Mibgħuta
friends-outgoing-hint = Qed tistenna tweġiba. Se tarahom fost il-ħbieb tiegħek ladarba jaċċettaw.
friends-withdraw = Ikkanċella
invite-friends-title = Stieden ħbieb
invite-friends-hint = Iċ-ċavetta tal-proġett tiġi kriptata għal kull ħabib fuq dan l-apparat. Is-server qatt ma jaraha.
invite-friends-empty = L-ebda ħabib x'tistieden għadu.
invite-friends-button = Stieden
invite-sent = { $count ->
    [one] Stedina mibgħuta
    [few] { $count } stediniet mibgħuta
    [many] { $count } stediniet mibgħuta
   *[other] { $count } stediniet mibgħuta
}
invitation-badge = Stedina
invitation-to = Ingħaqad ma' “{ $name }”
invitation-to-unnamed = Ingħaqad ma' proġett
invitation-unreadable = Din l-istedina ma tistax tinfetaħ fuq dan l-apparat
invitation-from = Minn { $email }
invitation-accept = Ingħaqad
invitation-decline = Irrifjuta
