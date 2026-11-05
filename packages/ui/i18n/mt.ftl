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
theme = Tema
theme-light = Ċar
theme-dark = Skur
theme-system = Sistema
theme-hint-device = Issejvjat fuq dan l-apparat biss.
theme-hint-system = Isegwi d-dehra ta' dan l-apparat.

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
error-rate-limited = Is-server huwa okkupat. Erġa' pprova fi ftit.
error-generic = Xi ħaġa marret ħażin. Jekk jogħġbok erġa' pprova.

error-invalid-email = Dak l-indirizz tal-email mhux validu.
error-invalid-password = Dik il-password mhix valida.
error-password-too-short = Il-password trid tkun tal-anqas 12-il karattru.
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
error-storage-full = Il-ħażna hija mimlija: iċ-ċavetta ta' dan il-proġett ma setgħetx tiġi salvata fuq dan it-tagħmir. Żomm il-link tal-qsim.
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
error-invalid-kdf-salt = Is-settings tal-encryption mhumiex validi. Aġġorna l-app u erġa' pprova.
error-mixed-project-batch = Dawn il-parteċipanti mhumiex kollha fl-istess proġett.
error-invalid-payload = Din il-verżjoni tal-app bagħtet data li s-server ma jaċċettax. Aġġornaha u erġa' pprova.
error-invalid-public-key = Iċ-ċavetta tal-encryption tiegħek mhijiex valida. Aġġorna l-app u erġa' pprova.
error-payment-methods-stale = Id-dettalji tal-ħlas tiegħek inbidlu fuq apparat ieħor. Erġa' llowdja u pprova mill-ġdid.

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
login-maybe-unverified = Għadek kif irreġistrajt? L-ewwel ivverifika l-indirizz tiegħek bil-link li bgħatnielek, jew erġa' ibagħtu.
login-resend = Ibgħat il-link ta' verifika mill-ġdid
login-resending = Qed jintbagħat…
login-resend-sent = Email mibgħuta - iċċekkja l-inbox tiegħek.

register-submit = Oħloq kont
register-submitting = Qed jinħoloq…
register-have-account = Diġà għandek kont?
register-password-placeholder = Tal-anqas 12-il karattru
register-password-warning = Ikteb il-password tiegħek: aħna qatt ma narawha u ma nistgħux nagħtuhielek lura.
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
account-delete-warning = Tħassir immedjat u permanenti tal-kont u d-data tiegħek.
account-delete-confirm-title = Ħassar il-kont
account-delete-confirm-message = Il-kont, is-sessjonijiet u l-lista tal-proġetti tiegħek se jitħassru b'mod permanenti. Mingħajr il-password tiegħek, id-data kriptata ta' proġett maqsum issir ma tinqarax għalik - dan ma jistax jitreġġa' lura.
erase-device-open = Oħroġ u ħassar dan l-apparat
erase-device-title = Tħassar dan l-apparat?
erase-device-body = Il-proġetti, iċ-ċwievet tagħhom u l-cache fuq dan l-apparat se jitħassru, imbagħad is-sessjoni se tingħalaq. Il-proġetti tiegħek jibqgħu fil-kont tiegħek: idħol mill-ġdid biex terġa' tiksibhom.
erase-device-checking = Qed niċċekkja li l-proġetti tiegħek jinsabu fil-kont tiegħek…
erase-device-pending = Bidliet li qed jistennew jintbagħtu: { $count }. Erġa' kkonnettja mal-internet u erġa' pprova.
erase-device-unreachable = Ma stajtx niċċekkja li l-proġetti tiegħek jinsabu fil-kont tiegħek. Iċċekkja l-konnessjoni u erġa' pprova.
erase-device-at-risk = Dawn il-proġetti mhumiex fil-kont tiegħek u se jintilfu. Biex iżżommhom, ikkanċella u l-ewwel ikkopja l-link tagħhom:
erase-device-confirm = Ħassar u oħroġ
erase-device-confirm-anyway = Ħassar xorta waħda
erase-device-erasing = Qed jitħassar…

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
payment-methods-stale = Id-dettalji tal-ħlas tiegħek inbidlu fuq apparat ieħor. Reġgħu tgħabbew - jekk jogħġbok erġa' pprova.
payment-methods-key-missing = Erġa' idħol biex timmaniġġja d-dettalji tal-ħlas tiegħek.
settings-payment-methods-share-warning = Metodu maqsum huwa viżibbli għall-membri kollha tal-proġetti tiegħek.
payment-method-share = Aqsam mal-proġetti tiegħi
payment-method-share-hint = Jidher ħdejn ismek meta xi ħadd għandu jagħtik flus.
payment-method-copy = Ikkopja { $name }
payment-method-copied = Ikkupjat.
payment-method-copy-failed = Ma setax jiġi kkupjat - agħżel it-test u kkupjah manwalment.

verify-email-checking = Qed jiġi vverifikat l-indirizz tal-email tiegħek…
verify-email-welcome = Email ivverifikata - merħba f'Counted!
verify-email-back-to-login = Lura għad-dħul
verify-email-intro = Daħħal il-password tiegħek biex tikkonferma li dan l-indirizz huwa tiegħek.
verify-email-submit = Ivverifika l-indirizz tiegħi

### Project status

project-archive = Arkivja
project-sheet-invite = Stieden
project-sheet-recurring = Rikorrenti
project-sheet-edit = Editja l-proġett
project-sheet-close = Agħlaq il-proġett
project-sheet-archive = Arkivja l-proġett
project-sheet-reopen = Erġa' iftaħ il-proġett
project-sheet-unarchive = Neħħi l-proġett mill-arkivju
project-sheet-leave = Itlaq mill-proġett

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
date-today = Illum
date-yesterday = Ilbieraħ
field-optional = Fakultattiv

### Projects

projects-filter-active = Attivi
projects-filter-all = Kollha
projects-count-label = Proġetti
projects-empty = L-ebda proġett
projects-offline-banner = Data offline - erġa' ikkonnettja biex taġġorna.
demo-banner = Proġett ta' dimostrazzjoni - qari biss.
demo-start-own = Oħloq il-proġett tiegħek
demo-badge = Demo
demo-bar = Il-bidliet tiegħek jisparixxu meta tagħlaq din it-tab.
demo-leave = Ibda sew
demo-preparing = Qed nippreparaw id-demo tiegħek…
demo-failed = Id-demo ma setgħetx tiġi ppreparata.
demo-sign-in-blocked = Id-dħul huwa mitfi fid-demo. Oħroġ minnha biex tidħol jew toħloq kont; il-proġetti tiegħek jibqgħu kif inhuma.
projects-no-local-data = L-ebda data lokali
projects-no-local-data-hint = Idħol biex tgħabbi l-proġetti tiegħek għall-ewwel darba.
projects-add = Żid proġett
projects-create = Oħloq proġett
projects-personal-expense = Nefqa personali
personal-expense-banner = Miżjuda ma' L-infiq tiegħi
personal-create-title = Oħloq il-proġett personali tiegħek
personal-create-hint = Segwi x’tonfoq waħdek
group-create-title = Oħloq proġett komuni
group-create-hint = Aqsam l-ispejjeż mal-ħbieb u l-familja
personal-create-action = Oħloq
personal-create-signup = Oħloq kont
personal-sheet-title = Il-proġett personali tiegħek
personal-sheet-intro = Biex tniżżel x’tonfoq waħdek. Jidher mal-gruppi tiegħek fl-Istatistika.
personal-sheet-point-alone = Membru wieħed: int, bl-isem tal-kont tiegħek. L-ebda link biex taqsam.
personal-sheet-point-encrypted = Ikkriptat fuq it-tagħmir tiegħek, bħall-proġetti l-oħra.
personal-sheet-point-stats = Is-sehem tiegħek fil-gruppi jiżdied ma’ dan l-infiq fl-Istatistika.
personal-default-name = L-infiq tiegħi
my-spending-prev-month = Ix-xahar ta’ qabel
my-spending-next-month = Ix-xahar li jmiss
my-spending-spent = Nfaqt
my-spending-groups = Gruppi
my-spending-income = Dħul
my-spending-balance = Bilanċ tax-xahar
my-spending-unconverted = Is-sehem f’munita oħra jibqa’ barra sakemm ikun magħruf rata.
my-spending-empty = Xejn dan ix-xahar
my-spending-empty-hint = Żid nefqa b’+. Is-sehem tiegħek fil-gruppi jidher hawn awtomatikament.
my-spending-share-of = minn { $amount }
my-spending-uncounted = mhux magħdud
payer-you = int
expense-received-by = riċevut minn
counted-switch = Għodd f’L-infiq tiegħi
counted-switch-on = Mixgħul: is-sehem tiegħek jiżdied ma’ L-infiq tiegħi
counted-switch-off = Mitfi: is-sehem tiegħek ma jidhirx f’L-infiq tiegħi
counted-groups = Gruppi magħduda
counted-groups-intro = Is-sehem tiegħek f’dawn il-gruppi jiżdied ma’ L-infiq tiegħi u mal-baġits.
counted-groups-converted = { $from }, maqlub għal { $to }
counted-groups-encrypted = Din l-għażla hija kriptata bil-kont tiegħek u ssegwik fuq it-tagħmir tiegħek.
counted-groups-empty = Għad m’hemm l-ebda grupp fuq dan it-tagħmir.
counted-groups-done = OK
counted-save-failed = Din l-għażla ma setgħetx tiġi ssejvjata. Erġa’ pprova.
tab-budget = Baġit
budget-title = Baġits
budget-manage = Immaniġġja
budget-left = Fadal { $amount }
budget-over = Maqbuż b’{ $amount }
budget-empty = L-ebda baġit
budget-empty-hint = Issettja limitu għal kull kategorija. Is-sehem tiegħek fil-gruppi jgħodd ukoll.
budget-define = Issettja baġit
budget-sheet-title = Baġits ta’ kull xahar
budget-sheet-intro = Fuq is-sehem tiegħek: personali u gruppi. Ħalli vojt biex ma ssegwix kategorija.
budget-average = Medja tal-aħħar 3 xhur: { $amount }
budget-for = Baġit { $category }
budget-alert = Avżani meta nilħaq 80 %
budget-alert-hint = Ikkalkulat fuq dan it-tagħmir
budget-invalid = Ammont mhux validu għal { $category }.
budget-offline = Il-baġits jiġu ssejvjati biss online. Erġa’ qabbad u pprova mill-ġdid.
budget-alert-near = { $category }: { $percent } % tal-baġit tax-xahar
budget-alert-over = { $category }: il-baġit tax-xahar inqabeż
personal-card-budget = { $month } · baġit { $amount }
my-spending-search-summary =
    { $count ->
        [one] { $count } riżultat
        [few] { $count } riżultati
        [many] { $count } riżultati
       *[other] { $count } riżultati
    } · { $total } · personali u gruppi, ix-xhur kollha
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
add-project-offline = Ma tistax toħloq proġett offline. Erġa' ikkonnettja u erġa' pprova.
add-project-name-required = Il-proġett jeħtieġ isem.

join-link-label = Link tal-qsim
join-link-hint = Il-link iġġorr iċ-ċavetta tad-dekriptaġġ - ikkopjaha kollha.
join-invalid-link = Dik il-link mhix valida. Waħħal il-link tal-qsim kollha, inkluża l-parti wara #.
join-wrong-project = Dik il-link hija għal proġett ieħor.

import-tricount-link-label = Link jew ċavetta ta' Tricount
import-tricount-key-required = Daħħal link jew ċavetta ta' Tricount.
import-tricount-encryption-failed = Il-kriptaġġ falla.
import-tricount-unimportable = Xejn ma ġie importat: dan it-Tricount għandu membri b'kont Tricount jew ammonti li ma jaqblux (entrati affettwati: { $count }).

### Expenses

save = Aħżen
saving = Qed jinħażen…
adding = Qed jiżdied…
link-copied = Link ikkupjata
missing-encryption-key = Iċ-ċavetta tal-kriptaġġ nieqsa.
missing-encryption-key-title = Iċ-ċavetta tal-kriptaġġ nieqsa
missing-encryption-key-hint = Il-link li użajt ma ġġorrx iċ-ċavetta meħtieġa biex tiddekripta dan il-proġett. Uża l-link sħiħa maqsuma minn min ħalqu.
project-locked-hint = Dan l-apparat m'għandux iċ-ċavetta ta' dan il-proġett. Iftaħ il-link tal-qsim tiegħu biex tiftħu.
expenses-unreadable = Xi entrati ma jistgħux jinqraw fuq dan l-apparat u mhumiex inklużi fit-totali. Aġġorna l-app biex tarahom.
project-unlock = Iftaħ
project-no-local-data-hint = Idħol biex tgħabbi d-data ta' dan il-proġett għall-ewwel darba.
project-gone-title = Dan il-proġett m'għadux jeżisti
project-gone-hint = Tħassar meta l-aħħar membru tiegħu telaq. Il-link tal-qsim m'għadhiex taħdem, anke jekk terġa' tiftaħha.

expense-add = Żid spiża
transfer-add = Żid trasferiment
expense-edit-title = Editja l-ispiża
expense-category = Kategorija
expense-category-auto = Awto · { $emoji }
emoji-picker-label = Emoji
emoji-none = Mingħajr emoji
expense-currency = Munita tal-ammont
amount-op-add = Żid
amount-op-subtract = Naqqas
amount-op-multiply = Immultiplika
amount-op-divide = Aqsam
amount-op-equals = Ugwali
amount-op-done = Lest
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
expense-type-group = Tip
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
participants-by-shares = Skont l-ishma
split-amounts = Ammonti
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
balance-gets-back = Jieħu lura
balance-owes = Għandu jħallas
balance-settled = Imħallsin
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
edit-project-offline-deferred = Offline: { $items } se jiġi applikat meta terġa' tikkonnettja.

export-failed = L-esportazzjoni falliet: { $reason }

history-expense-added = Spiża miżjuda: { $name }
history-expense-edited = Spiża editjata: { $name }
history-expense-deleted = Spiża mħassra: { $name }
move-expense = Mexxi għal proġett
move-expense-title = Mexxi għal…
move-current = Attwali
move-converted = { $currency }, ikkonvertit bir-rata tax-xahar
move-note-personal = Fil-pass li jmiss tagħżel min ħallas u għal min. In-nefqa toħroġ minn L-infiq tiegħi.
move-note-group = In-nefqa toħroġ minn { $project } għall-membri kollha, u l-bilanċ tal-grupp jerġa’ jinħadem. Fil-pass li jmiss tagħżel min ħallas u għal min.
move-banner = Minn { $from } għal { $to }
move-submit = Mexxi
move-done = In-nefqa tmexxiet għal { $project }
move-delete-failed = In-nefqa ġiet miżjuda mal-proġett l-ieħor iżda għadha f’{ $project }: ħassarha hemm.
history-expense-moved = In-nefqa tmexxiet għal proġett ieħor: { $name }
personal-badge = Personali
move-members =
    { $count ->
        [one] { $count } membru
        [few] { $count } membri
        [many] { $count } membri
       *[other] { $count } membri
    }
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
expense-share-percent = { $percent }%
expense-added-by = Miżjud minn { $name }, { $date }
expense-inconsistent-detail = L-ammonti ma jaqblux: { $paid } imħallas, { $owed } dovut, għal spiża ta' { $total }. Editja l-ispiża biex tikkoreġih.
missing-access-key = Iċ-ċavetta tal-aċċess nieqsa. Iftaħ dan il-proġett permezz tal-link tal-qsim tiegħu.
filter-all = Kollha
filter-my-payments = Il-ħlasijiet tiegħi
filter-my-debts = X'għandi nagħti
expense-filter-group = Filtru
expenses-search = Fittex l-infiq
expenses-no-match = L-ebda nfiq ma jaqbel.
expenses-search-open = Fittex
expenses-search-clear = Neħħi t-tfittxija
expenses-search-summary =
    { $count ->
        [one] { $count } riżultat
        [few] { $count } riżultati
        [many] { $count } riżultati
       *[other] { $count } riżultati
    } · { $total } · ix-xhur kollha
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
charts-tab-trends = Xejriet
charts-total-spent = Total minfuq
charts-avg-per-person = Medja għal kull persuna
charts-expense-count =
    { $count ->
        [one] { $count } spiża
        [few] { $count } spejjeż
        [many] { $count } spejjeż
       *[other] { $count } spejjeż
    }
charts-nothing-to-show = Xejn x'juri
charts-my-share-skipped =
    { $count ->
        [one] { $count } proġett mhux magħdud - l-ebda parteċipant magħżul, jew id-data tiegħu ma tgħabbietx.
        [few] { $count } proġetti mhux magħduda - l-ebda parteċipant magħżul, jew id-data tagħhom ma tgħabbietx.
        [many] { $count } proġetti mhux magħduda - l-ebda parteċipant magħżul, jew id-data tagħhom ma tgħabbietx.
       *[other] { $count } proġetti mhux magħduda - l-ebda parteċipant magħżul, jew id-data tagħhom ma tgħabbietx.
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
categories-manage = Immaniġġja l-kategoriji
categories-title = Kategoriji
categories-yours = Il-kategoriji tiegħek
categories-builtin = Awtomatiċi
categories-add = Żid kategorija
categories-in-parent = fil-kategorija { $parent }
categories-renamed-from = b’isem ġdid · oriġinarjament { $name }
categories-hidden-hint = moħbija · l-infiq tagħha jmur fil-kategorija { $other }
categories-always-shown = dejjem murija
categories-hidden = Moħbija
categories-new = Kategorija ġdida
categories-edit = Editja l-kategorija
categories-parent = Tingħadd fil-kategorija
categories-parent-hint = Fl-istatistika tal-proġetti kollha tiegħek, { $name } tingħadd fil-kategorija { $parent }.
categories-create = Oħloq
categories-hide = Aħbi din il-kategorija
categories-hide-hint = L-infiq tagħha jidher fil-kategorija { $other } għall-membri kollha.
categories-delete-title = Tħassar “{ $name }”?
categories-delete-body = L-infiq ikkonċernat ({ $count }) jerġa’ lura fil-kategorija { $parent }. L-ebda nefqa ma titħassar.
categories-delete-body-unknown = L-infiq tagħha jerġa’ lura fil-kategorija { $parent }. L-ebda nefqa ma titħassar.
categories-delete-warning = Il-kategorija tisparixxi għall-membri kollha tal-proġett.
categories-summary-created = Maħluqa: { $count }
categories-summary-renamed = B’isem ġdid: { $count }
categories-summary-hidden = Moħbija: { $count }
categories-summary-default = L-ebda kategorija personalizzata
categories-offline = Il-kategoriji jiġu mmaniġġjati biss online. Erġa’ qabbad u pprova mill-ġdid.
categories-error-too-many = Proġett għandu massimu ta’ 20 kategorija tiegħu.
categories-error-full = Dan il-proġett laħaq il-limitu tal-kategoriji.
categories-error-name = Agħti isem lill-kategorija.
categories-error-missing = Din il-kategorija nbidlet sadanittant. Agħlaq u erġa’ pprova.
history-category-created = Kategorija maħluqa: { $name }
history-category-edited = Kategorija editjata: { $name }
history-category-deleted = Kategorija mħassra: { $name }
charts-project = Proġett
charts-all-projects = Il-proġetti kollha
charts-date-from = Minn
charts-date-to = Sa
charts-total = Total
charts-tab-people = Persuni
charts-tab-projects = Proġetti
charts-scope = In-nefqa ta’ min
charts-scope-group = Grupp
charts-scope-me = Jien
charts-currency = Munita
charts-my-share = Is-sehem tiegħi
charts-share-of-total = { $pct }% ta’ { $total }
charts-i-paid = Ħallast
charts-paid-more = { $amount } aktar mis-sehem tiegħi
charts-paid-less = { $amount } inqas mis-sehem tiegħi
charts-paid-even = Eżattament is-sehem tiegħi
charts-part-title = Is-sehem tiegħek f’kull kategorija
charts-part-desc = Griż hu dak li nefaq il-grupp, bil-kulur dak li kkunsmajt int.
charts-breakdown-title = Tqassim skont il-kategorija
charts-breakdown-desc = Agħfas porzjon jew ringiela biex tara n-nefqiet.
charts-of-total = { $amount } minn { $total }
charts-show-all = Uri kollox ({ $count })
charts-show-less = Uri inqas
charts-spend-title = Nefqa matul iż-żmien
charts-group-by = Iġbor skont
bucket-day = Jum
bucket-week = Ġimgħa
bucket-month = Xahar
charts-avg = medja
charts-cat-title-day = { $category }, jum b’jum
charts-cat-title-week = { $category }, ġimgħa b’ġimgħa
charts-cat-title-month = { $category }, xahar b’xahar
charts-cat-desc = Agħżel kategorija biex issegwiha matul iż-żmien.
charts-running-title = Total kumulattiv
charts-running-desc = Minn { $date }.
charts-avg-per-day = { $amount } / jum bħala medja
charts-avg-per-week = { $amount } / ġimgħa bħala medja
charts-avg-per-month = { $amount } / xahar bħala medja
charts-people-title = Min ġarr il-grupp
charts-people-desc = Kemm ħallas kulħadd, ħdejn kemm ikkonsma.
charts-paid = Imħallas
charts-fair-share = Sehem ġust
charts-you = (int)
charts-net-more = ħallas aktar
charts-net-less = ħallas inqas
charts-balance-title = Il-bilanċ tiegħek matul iż-żmien
charts-balance-desc = Fuq il-linja l-grupp jonqoslok. Taħtha int tonqos lill-grupp.
charts-owed = Għandek tieħu
charts-owe = Għandek tagħti
charts-projects-title = Is-sehem tiegħek, skont il-proġett
charts-projects-desc = It-totali jinżammu skont il-munita.
history-empty = L-ebda avveniment
history-by = Minn { $name }
not-found-hint = Din il-paġna ma teżistix, jew ġiet imċaqalqa.
payers-title-paid-by = Imħallsa minn
payers-title-sender = Mittent
payers-title-contributors = Kontributuri
debtors-title-debtors = Għandhom jagħtu
debtors-title-recipients = Riċevituri
debtors-title-beneficiaries = Benefiċjarji
debtors-title-for = Għal
debtors-title-recipient = Riċevitur
expense-split-between = maqsum bejn { $count }
expense-people =
    { $count ->
        [one] { $count } persuna
        [few] { $count } persuni
        [many] { $count } persuna
       *[other] { $count } persuni
    }

### Welcome

welcome-title = Il-kontijiet tiegħek mhumiex affari ta' ħadd ieħor.
welcome-subtitle = Aqsam l-ispejjeż mal-ħbieb.
welcome-note = B'xejn. L-ebda kont meħtieġ. L-ebda reklam.
welcome-link-title = Link wieħed, u kulħadd jieħu sehem.
welcome-link-body = Ħadd m'għandu għalfejn joħloq kont.
welcome-link-account = Kont? Qatt obbligatorju. Iservi biex issib il-proġetti tiegħek fuq apparat ieħor, tistieden ħbieb mill-app u taqsam id-dettalji tal-ħlas tiegħek.
welcome-demo-project = Weekend f'Lyon
welcome-private-title = Ħadd ma jista' jaqra l-kontijiet tiegħek. Lanqas aħna.
welcome-private-body = Ismijiet, ammonti, proġetti: kollox jiġi kriptat fuq l-apparat tiegħek. Int biss għandek iċ-ċavetta.
welcome-private-names = Ismijiet
welcome-private-amounts = Ammonti
welcome-private-projects = Proġetti
welcome-scan-title = Ħu ritratt tal-irċevuta.
welcome-scan-body = L-ammont, id-data u l-kategorija jimtlew waħedhom. Kollox isir fuq it-telefon tiegħek. Ir-ritratt ma jinżammx.
welcome-eu-title = 100% Ewropew
welcome-no-ads = L-ebda reklam
welcome-no-trackers = L-ebda tracker
welcome-step = Pass { $current } minn { $total }
welcome-next = Li jmiss
welcome-skip = Aqbeż
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
help-forgot-password-warning = Ħadd ma jista' jirkupraha għalik.
help-forgot-password-a = Il-password tiegħek tipproteġi ċ-ċwievet tal-kriptaġġ tiegħek u qatt ma tintbagħtilna: ħadd - lanqas aħna - ma jista' jiddekripta l-proġetti tiegħek minflokok. Żommha sigura, idealment f'password manager.
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
project-currency-locked = Il-munita tiġi ffissata meta jinħoloq il-proġett.
currency-search = Fittex munita

update-required-title = Aġġornament meħtieġ
update-required-body = Din il-verżjoni ta' Counted hija qadima wisq biex titkellem mas-server. Aġġornaha biex tkompli tuża l-app.
update-required-button = Aġġorna

notifications-label = Notifiki
notifications-title = Notifiki
notifications-empty = Xejn ġdid
notifications-friend-request = Talba ta' ħbiberija

friends-title = Ħbieb
friends-anonymous-body = Il-ħbieb jinżammu mal-kont tiegħek. Idħol biex iżżid nies u tistedinhom fil-proġetti tiegħek mingħajr ma taqsam link.
friends-add-title = Żid ħabib
friends-add-hint = Jekk din il-persuna għandha kont, se tara t-talba tiegħek fl-app. Jekk le, għarrafha: issibha meta tirreġistra b'dan l-indirizz.
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
friends-key-changed = Il-kodiċi nbidel
friends-key-changed-hint = Il-kodiċi tas-sigurtà ta' ħabib inbidel. Dan jiġri meta jipproteġi l-kont tiegħu, jew jekk iċ-ċavetta tiegħu ġiet sostitwita. L-istediniet miegħu huma mblukkati sakemm taqraw il-kodiċi l-ġdid lil xulxin, wiċċ imb wiċċ jew bit-telefon, u tagħfas "Il-kodiċi jaqbel".
friends-fingerprint = Kodiċi ta' sigurtà
friends-fingerprint-hint = Żewġ ħbieb li jaqraw lil xulxin l-istess kodiċi ta' sigurtà jafu li ħadd ma hemm bejniethom - lanqas is-server tagħna.
friends-key-mismatch = Is-server bagħat ċavetta pubblika għall-kont tiegħek li ma taqbilx maċ-ċavetta privata tiegħek. Il-kodiċi tas-sigurtà tiegħek huwa moħbi u l-istediniet huma mitfija fuq dan l-apparat: forsi xi ħadd qed jbagħbas is-server.
keypair-mismatch = Iċ-ċwievet li s-server bagħat għall-kont tiegħek ma jaqblux ma' dawk fuq dan l-apparat. L-istediniet u l-bidliet taċ-ċavetta huma mitfija hawn sakemm jaqblu: xi ħadd jista' jkun qed ibagħbas is-server.
friends-outgoing-title = Mibgħuta
friends-outgoing-hint = Qed tistenna tweġiba. Se tarahom fost il-ħbieb tiegħek ladarba jaċċettaw.
friends-withdraw = Ikkanċella
friends-notify = Għarraf b'messaġġ
friends-notify-label = Għarraf lil { $email }
friends-notify-message = Qed naqleb għal Counted għall-ispejjeż maqsuma tagħna. Reġistrazzjoni b'xejn fuq https://counted.fr bl-indirizz { $email }: it-talba ta' ħbiberija tiegħi qed tistenna hemm.
friends-notify-copied = Messaġġ ikkupjat
friends-notify-copy-failed = Ma setax jiġi kkupjat il-messaġġ
guests-hint-title =
    { $count ->
        [one] { $count } parteċipant għadu ma ngħaqadx mal-proġett
        [few] { $count } parteċipanti għadhom ma ngħaqdux mal-proġett
        [many] { $count } parteċipanti għadhom ma ngħaqdux mal-proġett
       *[other] { $count } parteċipanti għadhom ma ngħaqdux mal-proġett
    }
guests-hint-later = Aktar tard
guests-hint-send = Stedinhom
guests-send-title = Ibgħat il-proġett lill-parteċipanti
guests-send-friend = Stieden ħabib ta' Counted
guests-send-friend-badge = Mingħajr link
guests-send-friend-hint = Jekk diġà għandu kont, jirċievi stedina kriptata; xejn ma jgħaddi mill-app tal-messaġġi tiegħek.
guests-send-link = Aqsam il-link tal-proġett
guests-send-link-hint = Għal min m'għandux kont. Jiftħu l-proġett b'tektika waħda.
guests-send-link-warning = Kull min għandu dan il-link jista' jaqra u jbiddel il-proġett. L-app tal-messaġġi tiegħek tarah ukoll.
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

# Participants in the create and edit modals, and the "who are you?" picker - see
# docs/plans/friends.md §11.
participants-you-label = Ismek f'dan il-proġett
participants-you-badge = Int
participants-others = Parteċipanti oħra
participants-removed = Imneħħija
participant-restore = Irrestawra
participants-empty = Żid parteċipanti hawn taħt.
participants-duplicate = “{ $name }” diġà fil-lista.
participants-input-label = Żid ħabib jew ikteb isem
participants-input-placeholder = Ħabib jew kwalunkwe isem
participants-suggest-friend = Ħabib · jingħaqad bħala “{ $name }”, jirċievi stedina
participants-suggest-not-ready = Ħabib · għadu mhux lest
participants-suggest-guest = Żid “{ $text }”
participants-suggest-guest-sub = Mistieden
participants-friends = Il-ħbieb tiegħek
participants-all-friends = Il-ħbieb kollha
participants-login-hint = Idħol biex iżżid nies dritt mil-lista tal-ħbieb tiegħek.
participants-invite-badge = Stieden
participants-guest-badge = Mistieden
participants-rename = Ibdel l-isem ta' { $name }
participants-remove = Neħħi lil { $name }
participants-rename-label = Isem ġdid
participants-rename-save = Issejvja l-isem
participants-rename-hint = L-isem li jara kulħadd f'dan il-proġett. L-istedina xorta tmur għand { $email }.
participants-invited-badge = Mistieden
participants-invited-sub = { $email } · għadha mhix aċċettata
participants-invited-pending = L-istedina għadha mhix aċċettata
participants-unlinked = Mhux marbut ma' kont
add-project-create-invite = Oħloq u stieden { $count }
edit-project-save-invite = Issejvja u stieden { $count }
edit-project-you-are = F'dan it-tagħmir int { $name }
edit-project-no-identity = Għadek ma għażiltx min int
edit-project-switch = Ibdel
edit-project-choose = Agħżel
invite-failed = Dawn l-istediniet ma setgħux jintbagħtu: { $emails }
invite-again = Erġa' stieden
friend-picker-title = Żid ħbieb
user-selection-invited-hint = { $email } stiednek f'“{ $project }”.
user-selection-suggested = Issuġġerit
user-selection-suggested-sub = { $email } żiedek b'dan l-isem
user-selection-confirm-as = Jien { $name }
user-selection-missing = Ismek mhux hawn? Itlob lil parteċipant biex iżżidek fis-settings tal-proġett.

## Spejjeż rikorrenti

repeat-label = Irrepeti
repeat-none = Ma tirrepetix
repeat-weekly = Kull ġimgħa
repeat-biweekly = Kull ġimagħtejn
repeat-monthly = Kull xahar
repeat-quarterly = Kull 3 xhur
repeat-yearly = Kull sena
repeat-every-weeks = Kull { $count } ġimgħat
repeat-every-months = Kull { $count } xhur
repeat-every-years = Kull { $count } snin
repeat-custom = Personalizzat…
repeat-every = Kull
repeat-unit-weeks = ġimgħat
repeat-unit-months = xhur
repeat-unit-years = snin
repeat-on-weekday = jum: { $weekday }
repeat-on-day = fil-{ $day } tax-xahar
repeat-on-day-month = fil-{ $day } ta’ { $month }
repeat-month-end = Fix-xhur iqsar taqa’ fl-aħħar jum.
repeat-ends = Tispiċċa
repeat-ends-never = Qatt
repeat-ends-on = F’data
repeat-ends-after = Wara
repeat-fewer = Inqas
repeat-more = Aktar
repeat-last-on = l-aħħar waħda fil-{ $date }
repeat-variable = L-ammont jinbidel kull darba
repeat-variable-hint = Kull waħda tiżdied bl-aħħar ammont u tiġi mmarkata “biex tiġi kkonfermata”.
repeat-done = Lest
repeat-no-end = Bla tmiem
repeat-until = Sal-{ $date }
repeat-occurrences = Għadd ta’ drabi: { $count }
repeat-offline = Teħtieġ konnessjoni. L-ispiża nnifisha xorta tista’ tiżdied.
repeat-foreign = Tirrepeti bħala { $amount } { $currency }, maqluba darba bir-rata tal-lum. Tiġi avżat jekk ir-rata tinbidel b’aktar minn 5 %.
repeat-backfill = Tibda fil-passat. Spejjeż miżjuda issa: { $count }.
add-and-repeat = Żid u rrepeti
weekday-1 = It-Tnejn
weekday-2 = It-Tlieta
weekday-3 = L-Erbgħa
weekday-4 = Il-Ħamis
weekday-5 = Il-Ġimgħa
weekday-6 = Is-Sibt
weekday-7 = Il-Ħadd
recurring-title = Spejjeż rikorrenti
recurring-strip = Spejjeż rikorrenti: { $count }
recurring-next = Li jmiss: { $name }, { $date }
recurring-to-confirm = Biex jiġu kkonfermati: { $count }
recurring-per-month = Fix-xahar, bejn wieħed u ieħor
recurring-your-share = Is-sehem tiegħek
recurring-active = Attivi
recurring-paused = Fuq pawża
recurring-finished = Mitmuma
recurring-paid-by = imħallsa minn { $name }
recurring-next-on = Li jmiss fil-{ $date }
recurring-progress = { $done } minn { $total }
recurring-rate-badge = Ir-rata nbidlet { $percent } %
recurring-empty = Għadu xejn ma jirrepeti. Agħżel “Irrepeti” meta żżid spiża: kera, abbonamenti, kontijiet.
recurring-next-ones = Li jmiss
recurring-added-so-far = Miżjuda s’issa
recurring-set-up-by = Maħluqa minn
recurring-pause = Poġġi fuq pawża
recurring-resume = Kompli
recurring-stop = Waqqaf ir-repetizzjoni
recurring-stop-title = Twaqqaf “{ $name }”?
recurring-stop-message = Mhux se tirrepeti aktar. L-ispejjeż diġà miżjuda jibqgħu.
recurring-resume-title = Tkompli “{ $name }”?
recurring-resume-message = Li jmiss fil-{ $date }. Id-dati mitlufa waqt il-pawża ma jiżdidux.
recurring-edit-title = Editja spiża rikorrenti
recurring-edit-banner = Il-bidliet japplikaw mill-{ $date }. L-ispejjeż diġà miżjuda jibqgħu kif inhuma.
recurring-next-on-label = Li jmiss fil-
recurring-next-too-early = Id-data li jmiss trid tkun wara l-aħħar spiża diġà miżjuda.
recurring-use-stop = Biex ittemmha, uża “Waqqaf ir-repetizzjoni” fuq l-ispiża rikorrenti.
recurring-drift = Ir-rata tal-{ $currency } inbidlet { $percent } % minn meta nħolqot. Kull waħda għadha tiżdied bħala { $amount } { $project_currency } (1 { $currency } = { $rate }). Bir-rata tal-lum tkun { $today_amount } { $project_currency }.
recurring-use-rate = Uża r-rata tal-lum
recurring-keep = Żomm { $amount } { $currency }
recurring-added = Spejjeż rikorrenti miżjuda: { $count }
recurring-blocks-removal = { $name } għadu ma jistax jitneħħa: parti minn { $rules }. Neħħi lil { $name } minnhom jew waqqafhom, imbagħad erġa’ ssejvja.
history-recurring-added = Miżjuda awtomatikament: { $name } ({ $date })
history-recurring-created = Spiża rikorrenti stabbilita: { $name }
history-recurring-edited = Spiża rikorrenti editjata: { $name }
history-recurring-paused = Spiża rikorrenti fuq pawża: { $name }
history-recurring-resumed = Spiża rikorrenti kompliet: { $name }
history-recurring-stopped = Spiża rikorrenti mwaqqfa: { $name }
occurrence-recurring = Spiża rikorrenti
occurrence-auto = Miżjuda awtomatikament minn spiża rikorrenti.
occurrence-auto-next = Miżjuda awtomatikament minn spiża rikorrenti. Li jmiss fil-{ $date }.
occurrence-auto-stopped = Miżjuda awtomatikament minn spiża rikorrenti li minn dak iż-żmien twaqqfet.
occurrence-manage = Immaniġġja
estimate-badge = Biex tiġi kkonfermata
expense-pending = Pendenti
estimate-title = Ammont biex jiġi kkonfermat.
estimate-body = Miżjuda bl-ammont ta’ qabel. Daħħal l-ammont veru meta jasal.
estimate-confirm = Ikkonferma l-ammont
apply-to = Applika għal
apply-this-only = Din l-ispiża biss
apply-and-next = Din u dawk ta’ wara
apply-and-next-hint = L-ispiża rikorrenti tinbidel mill-{ $date }
apply-rule-failed = L-ispiża ġiet issejvjata, iżda l-ispiża rikorrenti ma nbidlitx.
occurrence-delete-message = “{ $name }” tal-{ $date } se titħassar għal kollox. Ir-repetizzjoni tkompli u din id-data mhux se terġa’ lura.
occurrence-delete-one = Ħassar din biss
occurrence-delete-stop = Ħassar u waqqaf ir-repetizzjoni
error-recurring-clock = L-arloġġ ta’ dan l-apparat qiegħed ’il quddiem. Iċċekkja d-data u l-ħin.
error-recurring-not-found = Din l-ispiża rikorrenti m’għadhiex teżisti.
error-recurring-stale = Xi ħadd biddel din l-ispiża rikorrenti sadanittant. Ġiet mgħobbija mill-ġdid: iċċekkjaha u erġa’ ssejvja.
error-too-many-recurring = Dan il-proġett diġà għandu 50 spiża rikorrenti. Waqqaf waħda li m’għadekx teħtieġ biex iżżid oħra.
error-user-in-recurring = Dan il-parteċipant huwa parti minn spiża rikorrenti. L-ewwel neħħih minnha jew waqqafha.

home-owed-to-you = Għandek tieħu
home-you-owe = Għandek tagħti
home-details-show = Uri d-dettalji
home-details-hide = Aħbi d-dettalji
home-all-settled = Kollox imħallas
home-pay = Ħallas
home-remind = Fakkar
home-remind-message = Tfakkira żgħira: għad hemm rimborżi x’jitħallsu f’{ $project }.
home-row-owed = għandek tieħu
home-row-owe = għandek tagħti
home-row-settled = Imħallas
home-totals-partial = Mingħajr proġetti mingħajr rata tal-kambju
error-account-key-changed = Iċ-ċavetta tal-kont tiegħek inbidlet fuq apparat ieħor. Erġa' idħol biex tkompli.
settings-security = Sigurtà
settings-change-password = Biddel il-password
settings-change-password-hint = Il-proġetti u d-data tiegħek jibqgħu kif inhuma. L-apparati l-oħra tiegħek joħorġu mill-kont.
change-password-title = Biddel il-password tiegħek
change-password-current = Password attwali
change-password-new = Password ġdida
change-password-confirm = Ikkonferma l-password il-ġdida
change-password-mismatch = Iż-żewġ passwords il-ġodda ma jaqblux.
change-password-submit = Biddel il-password
change-password-submitting = Qed tinbidel…
change-password-done = Il-password inbidlet. L-apparati l-oħra tiegħek ħarġu mill-kont.
error-rotation-incomplete = Il-kont tiegħek inbidel sadanittant. Erġa' pprova.
error-key-lifecycle-unavailable = Għadu mhux disponibbli: l-ewwel kulħadd jeħtieġ l-aħħar verżjoni tal-app.
settings-compromised = Xi ħadd jista' jkun jaf il-password tiegħi
settings-compromised-hint = Password ġdida, ċwievet ġodda, u l-apparati l-oħra kollha joħorġu mill-kont.
settings-compromised-open = Ipproteġi l-kont tiegħi
compromised-title = Ipproteġi l-kont tiegħek
compromised-body = Il-password u ċ-ċwievet tal-kont tiegħek jinbidlu, u l-apparati l-oħra kollha joħorġu mill-kont. Il-ħbieb tiegħek se jaraw il-kodiċi tas-sigurtà tiegħek jinbidel: erġgħu qabbluh.
compromised-keeps = Dak li diġà nfetaħ bil-password tiegħek ma jistax jittieħed lura: il-proġetti li setgħu jinqraw s'issa.
compromised-submit = Ibdel il-password u ċ-ċwievet
compromised-done = Il-kont tiegħek huwa protett. Itlob lill-ħbieb tiegħek iqabblu l-kodiċi l-ġdid tas-sigurtà tiegħek.
friends-new-fingerprint = Kodiċi ġdid:
friends-code-matches = Il-kodiċi jaqbel
friends-code-trusted = Il-kodiċi kkonfermat.
settings-recovery-key = Ċavetta ta' rkupru
settings-recovery-key-none = Mingħajrha, password minsija tiswielek il-preferenzi, id-dettalji tal-ħlas u l-kodiċijiet tal-ħbieb, u tieħu 3 ijiem.
settings-recovery-key-set = Maħluqa. Jekk tinsa l-password, terġa' tagħtik il-kont b'kollox fih.
settings-recovery-key-pending = Ċavetta ġdida tieħu postha fi { $when }. Ma kontx int? Ikkanċellaha biċ-ċavetta attwali.
recovery-key-create = Oħloq
recovery-key-replace = Ibdel
recovery-key-cancel-replacement = Ikkanċella ċ-ċavetta l-ġdida
recovery-key-title = Iċ-ċavetta ta' rkupru tiegħek
recovery-key-body = Iktibha jew issejvjaha f'post fejn tista' tilħaqha int biss. Counted ma tistax turiha mill-ġdid jew tirkuprahielek.
recovery-key-copy = Ikkopja
recovery-key-copied = Ikkopjata
recovery-key-save = Issejvja bħala fajl
recovery-key-print = Ipprintja
recovery-key-next = Kompli
recovery-key-confirm-body = Biex tiċċekkja l-kopja, ikteb il-grupp { $n } taċ-ċavetta.
recovery-key-confirm-wrong = Dak il-grupp ma jaqbilx. Erġa' ħares lejn il-kopja.
recovery-key-password-body = Daħħal il-password biex tissejvja ċ-ċavetta fil-kont.
recovery-key-submit = Issejvja ċ-ċavetta
recovery-key-done = Iċ-ċavetta ta' rkupru ġiet issejvjata.
recovery-key-replacement-done = Iċ-ċavetta l-ġdida ġiet issejvjata. Tieħu post l-attwali fi 72 siegħa; sa dakinhar l-attwali għadha taħdem.
recovery-key-field = Ċavetta ta' rkupru
recovery-key-file-intro = Ċavetta ta' rkupru ta' Counted
recovery-key-cancel-done = Iċ-ċavetta l-ġdida ġiet ikkanċellata.
compromised-new-key = Issejvja ċ-ċavetta l-ġdida ta' rkupru issa: tieħu post il-qadima fi 72 siegħa, u sa dakinhar il-qadima għadha taħdem.
pending-reset-title = Intalab reset tal-kont tiegħek
pending-reset-body = Jidħol fis-seħħ fi { $when }: imbagħad jintilfu l-preferenzi, id-dettalji tal-ħlas u l-kodiċijiet tal-ħbieb, u l-apparati jridu jerġgħu jidħlu. Ma kontx int? Ikkanċellah bil-password attwali.
pending-reset-cancel = Ikkanċella r-reset
pending-reset-cancelled = Ir-reset ġie kkanċellat.
login-forgot = Insejt il-password?
forgot-title = Insejt il-password
forgot-body = Daħħal l-email tiegħek. Jekk għandha kont, nibagħtu link biex tagħżel password ġdida.
forgot-submit = Ibgħat il-link
forgot-sent = Jekk dan l-indirizz għandu kont, link qiegħed fi triqtu. Jiskadi fi 24 siegħa.
reset-title = Ikseb lura l-kont tiegħek
reset-with-key-body = Daħħal iċ-ċavetta ta' rkupru u password ġdida: kollox jerġa' lura kif kien.
reset-submit-key = Irkupra l-kont tiegħi
reset-no-key = M'għandix iċ-ċavetta ta' rkupru
reset-without-key-title = Irrisettja mingħajr iċ-ċavetta ta' rkupru
reset-without-key-body = Il-kont tiegħek jerġa' jibda b'password ġdida fi 72 siegħa. Il-proġetti jiġu lura mill-apparati tiegħek u mil-links tal-qsim; il-preferenzi, id-dettalji tal-ħlas u l-kodiċijiet tal-ħbieb le. Sa dakinhar il-password attwali għadha taħdem u tista' tikkanċella r-reset.
reset-name = Ismek
reset-submit-request = Irrisettja l-kont tiegħi
reset-requested = Lest. Il-password il-ġdida taħdem minn { $when }. Sa dakinhar il-password attwali għadha taħdem u tista' tikkanċellah.
reset-pending-already = Diġà hemm reset pendenti sa { $when }. Talba ġdida terġa' tibda t-72 siegħa.
recovered-done = Il-kont tiegħek reġa' lura. L-apparati l-oħra ħarġu mill-kont.
login-reset-undoable = Din il-password ġiet mibdula b'reset fl-aħħar 30 jum. Jekk ma tlabtux, ħassru: il-kont jerġa' lura kif kien.
login-undo-reset = Ħassar ir-reset
reset-undone = Ir-reset tħassar. Il-kont huwa kif kien.
error-reset-pending = Din il-password il-ġdida taħdem meta r-reset jidħol fis-seħħ, 72 siegħa wara t-talba.
error-account-disabled = Dan il-kont tħassar ftit wara reset u qed jistenna li jitħassar għal kollox. Il-password ta' qabel għadha tista' tħassar ir-reset.
error-recovery-key-invalid = Din iċ-ċavetta ta' rkupru ma tiftaħx dan il-kont.
error-project-key-required = Dan il-proġett jeħtieġ iċ-ċavetta tiegħu. Erġa' iftħu mil-link tal-qsim.
names-or = { $first } jew { $last }
participants-admin-badge = Admin
edit-project-admins = Admins: { $count }
edit-project-succession = Jekk il-proġett jibqa' mingħajr admin, ir-rwol jgħaddi lill-membru b'kont li ilu l-aktar fil-proġett.
member-sheet-account = Membru b'kont
member-actions = Azzjonijiet: { $name }
member-make-admin = Agħti r-rwol ta' admin
member-make-admin-hint = Ir-rwol jagħti s-setgħa li tinbidel iċ-ċavetta tal-proġett u li jitneħħa xi ħadd.
member-made-admin = Ir-rwol ta' admin issa huwa ta' { $name }.
group-security = Sigurtà tal-proġett
group-security-admin = Sigurtà tal-proġett · għandek ir-rwol ta' admin
group-rotate = Ibdel iċ-ċavetta tal-proġett
group-rotate-hint = Jekk il-link tal-proġett inkixef: min għandu l-link il-qadim jitlef l-aċċess għall-proġett.
group-remove = Neħħi lil xi ħadd mill-proġett
group-remove-hint = Ineħħi l-aċċess u jibdel iċ-ċavetta fl-istess ħin.
group-admins-only = Għall-admins biss.
group-admins-only-account = Għall-admins biss, li għandhom kont.
group-ask-admin = Staqsi lil { $names }, l-admin tal-proġett.
group-ask-admins = Staqsi lil { $names }, l-admins tal-proġett.
group-ask-an-admin = Staqsi lil wieħed mill-admins tal-proġett.
group-with-account = B'kont
group-free = B'xejn
group-sign-in-hint = Imbagħad ir-rwol jista' jingħatalek minn admin tal-proġett, { $names }.
group-sign-in-hint-any = Imbagħad ir-rwol jista' jingħatalek minn admin tal-proġett.
group-no-admin = Dan il-proġett m'għandux admin, għalhekk iċ-ċavetta tiegħu ma tistax tinbidel.
group-hide = Aħbi lil xi ħadd
group-hide-hint = Jaħbi lill-persuna mil-listi mingħajr ma jneħħi l-aċċess: il-link u ċ-ċavetta għadhom jaħdmu.
group-recreate-hint = Dan il-proġett m'għandux admin. Biex tneħħi lil xi ħadd, oħloq proġett ġdid.
group-recreate = Oħloq proġett ġdid
group-no-admin-yet = Dan il-proġett għad m'għandux admin.
group-creator-hint = Int biss tista' ssir admin ta' dan il-proġett, billi toħloq kont.
group-creator-elsewhere-hint = Il-persuna li ħolqot il-proġett biss tista' ssir l-admin tiegħu, billi toħloq kont. Inkella, oħloq proġett ġdid biex tneħħi lil xi ħadd.
member-revoke-admin = Neħħi r-rwol ta' admin
member-revoke-admin-hint = It-tibdil taċ-ċavetta tal-proġett u t-tneħħija ta' xi ħadd ma jibqgħux possibbli.
member-revoked-admin = Ir-rwol ta' admin m'għadux ta' { $name }.
member-step-down = Ieqaf milli tkun admin
member-step-down-hint = Ma tibqax tista' tibdel iċ-ċavetta tal-proġett jew tneħħi lil xi ħadd.
member-stepped-down = M'għadx għandek ir-rwol ta' admin.
member-remove = Neħħi mill-proġett
member-remove-hint = Jibdel iċ-ċavetta tal-proġett: xejn ġdid ma jibqa' jinqara minn { $name }.
member-hide = Aħbi mil-listi
error-user-not-admin = Dan il-membru mhuwiex admin.
error-creator-proof-invalid = Dan l-apparat ma ħoloqx dan il-proġett.
error-project-has-admin = Dan il-proġett diġà għandu admin.
error-not-project-admin = L-admins tal-proġett biss jistgħu jagħmlu dan.
error-user-not-claimed = Din il-persuna m'għandhiex kont, għalhekk ma tistax tkun admin.
error-project-key-rotated = Iċ-ċavetta tal-proġett inbidlet. Itlob il-link il-ġdid lil membru.
error-project-rotating = Iċ-ċavetta tal-proġett qed tinbidel. Erġa' pprova fi ftit.
error-rotation-expired = Dan it-tibdil taċ-ċavetta dam wisq u ġie kkanċellat. Erġa' ibda.
error-rotation-rows-changed = Il-proġett inbidel waqt it-tibdil taċ-ċavetta. Erġa' ibda.
project-rotated-title = M'għadx għandek aċċess għal dan il-proġett
project-rotated-body = Iċ-ċavetta ta' dan il-proġett inbidlet mill-admin. Itlob il-link il-ġdid lill-admin.
project-rotated-open-link = Iftaħ link
rotation-remove-title = Tneħħi lil { $name } mill-proġett?
rotation-rotate-title = Tibdel iċ-ċavetta tal-proġett?
rotation-remove-body = Li tneħħi membru jfisser li tinħoloq ċavetta ġdida. Il-link il-ġdid ikollok taqsmu mal-persuni li m’għandhomx kont.
rotation-rotate-body = Min għandu l-link il-qadim jitlef l-aċċess għall-proġett. Il-link il-ġdid ikollok taqsmu mal-persuni li m’għandhomx kont.
rotation-boxed =
    { $count ->
        [0] L-ebda membru ieħor mhu se jirċievi ċ-ċavetta awtomatikament
        [one] { $count } membru se jirċievi ċ-ċavetta awtomatikament
        [few] { $count } membri se jirċievu ċ-ċavetta awtomatikament
        [many] { $count } membru se jirċievu ċ-ċavetta awtomatikament
       *[other] { $count } membru se jirċievu ċ-ċavetta awtomatikament
    }
rotation-by-link-title = Persuni li jkollok tibgħatilhom il-link il-ġdid
rotation-by-link-hint = Dawn il-persuni m’għandhomx kont. Tibgħatilhom il-link eżatt wara.
rotation-names-and = { $first } u { $last }
rotation-names-and-others = { $names } u { $count } oħra
rotation-confirm-remove = Neħħi u ibdel iċ-ċavetta
rotation-confirm-rotate = Ibdel iċ-ċavetta
rotation-done-title = Iċ-ċavetta l-ġdida hija attiva
rotation-done-removed = { $name } mhux aktar parti mill-proġett.
rotation-done-rotated = Iċ-ċavetta tal-proġett inbidlet.
rotation-done-boxed =
    { $count ->
        [0] L-ebda membru ieħor ma rċieva ċ-ċavetta awtomatikament.
        [one] { $count } membru rċieva ċ-ċavetta awtomatikament.
        [few] { $count } membri rċivew ċ-ċavetta awtomatikament.
        [many] { $count } membru rċivew ċ-ċavetta awtomatikament.
       *[other] { $count } membru rċivew ċ-ċavetta awtomatikament.
    }
rotation-new-link =
    { $count ->
        [0] Link ġdid tal-proġett
        [one] Link ġdid biex tibgħat · { $count } persuna mingħajr kont
        [few] Link ġdid biex tibgħat · { $count } persuni mingħajr kont
        [many] Link ġdid biex tibgħat · { $count } persuna mingħajr kont
       *[other] Link ġdid biex tibgħat · { $count } persuna mingħajr kont
    }
rotation-link-renewed = Il-link ġie mġedded b’suċċess.
rotation-done = Lest
history-key-changed = Iċ-ċavetta tal-proġett inbidlet
history-member-removed = { $name } mhux aktar parti mill-proġett
project-rotated-row-hint = Iċ-ċavetta inbidlet. Itlob il-link il-ġdid.
