# Eesti. Täielik, välja arvatud juriidilised tekstid (legal-, terms-, privacy-), mis on olemas ainult
# inglise ja prantsuse keeles ning langevad sõnumi kaupa tagasi en.ftl-ile.

### Common

loading = Laadimine…
cancel = Loobu
confirm = Kinnita
retry = Proovi uuesti
delete = Kustuta
back = Tagasi
language = Keel
theme = Teema
theme-light = Hele
theme-dark = Tume
theme-system = Süsteem
theme-hint-device = Salvestatud ainult selles seadmes.
theme-hint-system = Järgib selle seadme välimust.

### Navigation

nav-main = Peanavigatsioon
nav-projects = Projektid
nav-charts = Statistika
nav-settings = Seaded

### Connectivity

offline-banner = Võrguühenduseta
offline-pending =
    { $count ->
        [one] { $count } ootel
       *[other] { $count } ootel
    }

sync-conflict-edit = Konflikt: „{ $name }“ muutmine ebaõnnestus (kirje kustutatud). Vahele jäetud.
sync-conflict-delete = Konflikt: „{ $name }“ kustutamine ebaõnnestus (kirje kustutatud). Vahele jäetud.
sync-conflict-other = Konflikt: toiming kirjel „{ $name }“ ebaõnnestus (kirje kustutatud). Vahele jäetud.
sync-error = Sünkroonimise viga: { $reason }

### Errors

error-network = Serveriga ei saa ühendust. Kontrolli oma internetiühendust.
error-rate-limited = Server on hõivatud. Proovi hetke pärast uuesti.
error-generic = Midagi läks valesti. Proovi uuesti.

error-invalid-email = See e-posti aadress ei kehti.
error-invalid-password = See parool ei kehti.
error-password-too-short = Parool peab olema vähemalt 12 tähemärki pikk.
error-client-outdated = See rakenduse versioon on aegunud. Sisselogimiseks uuenda seda.
error-invalid-link = See link ei kehti.
error-batch-too-large = Liiga palju kirjeid korraga.
error-payers-required = Vali vähemalt üks maksja.
error-debtors-required = Vali vähemalt üks võlgnik.
error-duplicate-participant = Osaleja esineb samal poolel kaks korda.
error-participant-not-in-project = See osaleja ei kuulu sellesse projekti.
error-too-many-participants = Ühe kulu jaoks liiga palju osalejaid.
error-invalid-credentials = Vale e-post või parool.
error-unauthenticated = Selle tegemiseks logi sisse.
error-email-not-verified = Sinu e-posti aadress pole veel kinnitatud.
error-project-not-found = Seda projekti enam pole.
error-expense-not-found = Seda kulu enam pole.
error-storage-full = Salvestusruum on täis: selle projekti võtit ei õnnestunud selles seadmes salvestada. Hoia jagamislink alles.
error-user-not-found = Seda osalejat enam pole.
error-tricount-not-found = Tricounti ei leitud või selle API tagastas vea.
error-too-many-members = See projekt on jõudnud liikmete piirini.
error-identity-taken = Teine konto on selle osaleja juba endale võtnud.
error-claim-proof-invalid = Sellel seadmel pole projekti võtit, seega ei saa see osalejat endale võtta. Ava jagamislink uuesti.
error-user-has-payments = Sellel osalejal on projektis kulusid ja teda ei saa eemaldada.
error-resend-cooldown = Oota 60 sekundit, enne kui uut kirja küsid.
error-self-friend-request = Sa ei saa ennast sõbraks lisada.
error-not-a-friend = Kutsuda saab ainult oma sõbralisti inimesi.
error-friend-has-no-key = See sõber pole veel rakenduse uusimat versiooni avanud. Palu tal korra sisse logida ja proovi siis uuesti.
error-friend-request-not-found = Seda sõbrakutset enam pole.
error-invitation-not-found = Seda kutset enam pole.
error-too-many-friend-requests = Praegu liiga palju sõbrakutseid. Proovi homme.
error-too-many-invitations = Liiga palju ootel kutseid.
error-invalid-kdf-salt = Krüpteerimisseaded ei ole kehtivad. Uuenda rakendust ja proovi uuesti.
error-mixed-project-batch = Need osalejad ei ole kõik samas projektis.
error-invalid-payload = See rakenduse versioon saatis andmeid, mida server ei aktsepteeri. Uuenda seda ja proovi uuesti.
error-invalid-public-key = Sinu krüpteerimisvõti ei ole kehtiv. Uuenda rakendust ja proovi uuesti.
error-payment-methods-stale = Sinu makseandmeid muudeti teises seadmes. Laadi uuesti ja proovi uuesti.

### Auth

field-email = E-post
field-email-placeholder = sina@naide.ee
field-password = Parool
field-name = Nimi
field-name-placeholder = Mari Maasikas

login-title = Logi sisse
login-submit = Logi sisse
login-submitting = Sisselogimine…
login-password-placeholder = Sinu parool
login-no-account = Pole veel kontot?
login-unverified = Sinu e-posti aadress pole veel kinnitatud. Vaata postkasti või saada link uuesti.
login-maybe-unverified = Registreerusid just? Kinnita esmalt oma aadress lingiga, mille sulle saatsime, või saada see uuesti.
login-resend = Saada kinnituslink uuesti
login-resending = Saatmine…
login-resend-sent = Kiri saadetud - vaata postkasti.

register-submit = Loo konto
register-submitting = Loomine…
register-have-account = Konto juba olemas?
register-password-placeholder = Vähemalt 12 tähemärki
register-password-warning = Kirjuta parool üles: me ei näe seda kunagi ega saa seda sulle tagasi anda.
register-check-email-title = Vaata oma e-posti
register-email-sent = Kiri saadetud
register-email-sent-hint = Konto aktiveerimiseks klõpsa postkastis olevat linki.
register-not-received-prefix = Ei jõudnud kohale? Vaata rämpsposti või
register-sign-in-link = logi sisse
register-not-received-suffix = et link uuesti saata.
register-terms-prefix = Konto loomisega nõustud meie
register-terms-link = kasutustingimustega
register-terms-and = ja meie
register-privacy-link = privaatsuspoliitikaga

settings-title = Seaded
settings-preferences = Eelistused
settings-preferences-local = Salvestatud sellesse seadmesse.
settings-preferences-synced = Sünkroonitud sinu kontoga, krüpteeritud.
settings-about = Teave
settings-anonymous-title = Sa pole sisse logitud
settings-upsell-title = Sinu projektid igas seadmes
settings-upsell-free = Tasuta
settings-upsell-body = Counted töötab ilma kontota. Tasuta kontoga järgnevad sinu projektid ja eelistused sulle telefoni, sülearvutisse ja veebi - endiselt krüpteeritud, endiselt meile loetamatud.
settings-locked-badge = Konto
settings-locked-friends = Loo konto, et lisada sõpru ja kutsuda neid projekti otse rakendusest - ilma linki edastamata.
settings-locked-payment-methods = Salvesta oma IBAN või makserakendus üks kord ja jaga neid valitud projektidega. Kes sulle võlgu on, näeb neid sinu nime kõrval.
settings-friends-hint = Lisa sõpru ja kutsu nad oma projektidesse ilma linki jagamata.

account-member-since = Liige alates
account-logout = Logi välja
account-logging-out = Väljalogimine…
account-delete-title = Kustuta minu konto
account-delete-warning = Sinu konto ja andmete kohene ja jäädav kustutamine.
account-delete-confirm-title = Konto kustutamine
account-delete-confirm-message = Sinu konto, seansid ja projektide loend kustutatakse jäädavalt. Ilma paroolita muutuvad jagatud projekti krüpteeritud andmed sulle loetamatuks - seda ei saa tagasi võtta.
erase-device-open = Logi välja ja tühjenda see seade
erase-device-title = Kas tühjendada see seade?
erase-device-body = Selle seadme projektid, nende võtmed ja vahemälu kustutatakse, seejärel sinu seanss lõpeb. Sinu projektid jäävad sinu kontole: logi uuesti sisse, et need tagasi saada.
erase-device-checking = Kontrollin, kas sinu projektid on sinu kontol…
erase-device-pending = Saatmata muudatusi: { $count }. Mine uuesti võrku ja proovi siis uuesti.
erase-device-unreachable = Ei õnnestunud kontrollida, kas sinu projektid on sinu kontol. Kontrolli ühendust ja proovi siis uuesti.
erase-device-at-risk = Need projektid ei ole sinu kontol ja lähevad kaotsi. Kui tahad need alles jätta, loobu ja kopeeri enne nende link:
erase-device-confirm = Tühjenda ja logi välja
erase-device-confirm-anyway = Tühjenda ikkagi
erase-device-erasing = Tühjendan…

settings-payment-methods = Makseandmed
settings-payment-methods-hint = Kuidas soovid raha tagasi saada. Krüpteeritud sinu kontoga.
payment-method-kind = Viis
payment-method-kind-other = Muu
payment-method-label = Nimi
payment-method-label-placeholder = Põhikonto
payment-method-value = Andmed
payment-method-value-placeholder = IBAN, telefoninumber, kasutajanimi…
payment-method-add = Lisa
payment-method-remove = Eemalda { $name }
payment-method-empty = Sa pole veel makseandmeid lisanud.
payment-method-deleted = Makseviis kustutatud.
payment-method-value-required = Täida iga makseviisi andmed või eemalda see.
payment-method-label-required = Anna oma kohandatud makseviisile nimi.
payment-method-too-long = See on liiga pikk - lühenda.
payment-method-invalid-characters = Eemalda reavahetused või nähtamatud märgid.
payment-method-limit = Salvestada saab kuni { $max } makseviisi.
payment-methods-saved = Makseandmed salvestatud.
payment-methods-offline = Makseandmete salvestamiseks pead olema võrgus.
payment-methods-stale = Sinu makseandmeid muudeti teises seadmes. Need laaditi uuesti - proovi uuesti.
payment-methods-key-missing = Makseandmete haldamiseks logi uuesti sisse.
settings-payment-methods-share-warning = Jagatud makseviis on nähtav kõigile sinu projektide liikmetele.
payment-method-share = Jaga minu projektidega
payment-method-share-hint = Kuvatakse sinu nime kõrval, kui keegi on sulle võlgu.
payment-method-copy = Kopeeri { $name }
payment-method-copied = Kopeeritud.
payment-method-copy-failed = Kopeerimine ebaõnnestus - vali tekst ja kopeeri käsitsi.

verify-email-checking = E-posti aadressi kinnitamine…
verify-email-welcome = E-post kinnitatud - tere tulemast Countedisse!
verify-email-back-to-login = Tagasi sisselogimisse
verify-email-intro = Sisesta oma parool, et kinnitada, et see aadress on sinu oma.
verify-email-submit = Kinnita minu aadress

### Project status

project-archive = Arhiveeri
project-sheet-invite = Kutsu
project-sheet-recurring = Korduvad
project-sheet-edit = Muuda projekti
project-sheet-close = Sulge projekt
project-sheet-archive = Arhiveeri projekt
project-sheet-reopen = Ava projekt uuesti
project-sheet-unarchive = Taasta projekt arhiivist
project-sheet-leave = Lahku projektist

### Dates

date-long = { $day }. { $month } { $year }

month-1 = jaanuar
month-2 = veebruar
month-3 = märts
month-4 = aprill
month-5 = mai
month-6 = juuni
month-7 = juuli
month-8 = august
month-9 = september
month-10 = oktoober
month-11 = november
month-12 = detsember

month-short-1 = jaan
month-short-2 = veebr
month-short-3 = märts
month-short-4 = apr
month-short-5 = mai
month-short-6 = juuni
month-short-7 = juuli
month-short-8 = aug
month-short-9 = sept
month-short-10 = okt
month-short-11 = nov
month-short-12 = dets

### Actions

add = Lisa
create = Loo
creating = Loomine…
edit = Muuda
leave = Lahku
close = Sulge
paste = Kleebi
join = Liitu
import = Impordi
importing = Importimine…
field-description = Kirjeldus
field-date = Kuupäev
date-today = Täna
date-yesterday = Eile
field-optional = Valikuline

### Projects

projects-filter-active = Aktiivsed
projects-filter-all = Kõik
projects-count-label = Projektid
projects-empty = Projekte pole
projects-offline-banner = Võrguühenduseta andmed - värskendamiseks loo ühendus uuesti.
demo-banner = Näidisprojekt - ainult lugemiseks.
demo-start-own = Loo oma projekt
demo-badge = Demo
demo-bar = Sinu muudatused kaovad selle vahekaardi sulgemisel.
demo-leave = Alusta päriselt
demo-preparing = Demo valmistamine…
demo-failed = Demo ettevalmistamine ebaõnnestus.
demo-sign-in-blocked = Demos on sisselogimine välja lülitatud. Lahku demost, et sisse logida või konto luua; sinu enda projektid jäävad puutumata.
projects-no-local-data = Kohalikke andmeid pole
projects-no-local-data-hint = Projektide esmakordseks laadimiseks logi sisse.
projects-add = Lisa projekt
projects-create = Loo projekt
projects-personal-expense = Isiklik kulu
personal-expense-banner = Lisatud jaotisse Minu kulud
personal-create-title = Loo oma isiklik projekt
personal-create-hint = Jälgi, mida üksi kulutad
group-create-title = Loo jagatud projekt
group-create-hint = Jaga kulusid sõprade ja perega
personal-create-action = Loo
personal-create-signup = Loo konto
personal-sheet-title = Sinu isiklik projekt
personal-sheet-intro = Selle kirja panemiseks, mida üksi kulutad. See kuvatakse koos gruppidega Statistikas.
personal-sheet-point-alone = Üks liige: sina, oma konto nimega. Jagamislinki pole.
personal-sheet-point-encrypted = Krüpteeritud sinu seadmetes, nagu teised projektid.
personal-sheet-point-stats = Sinu osad gruppides liituvad Statistikas nende kuludega.
personal-default-name = Minu kulud
my-spending-prev-month = Eelmine kuu
my-spending-next-month = Järgmine kuu
my-spending-spent = Kulutasid
my-spending-groups = Grupid
my-spending-income = Tulud
my-spending-balance = Kuu saldo
my-spending-unconverted = Teise valuuta osad jäetakse kõrvale, kuni kurss on teada.
my-spending-empty = Sel kuul ei midagi
my-spending-empty-hint = Lisa kulu nupuga +. Sinu osad gruppides ilmuvad siia automaatselt.
my-spending-share-of = summast { $amount }
my-spending-uncounted = ei arvestata
payer-you = sina
expense-received-by = sai
counted-switch = Arvesta: Minu kulud
counted-switch-on = Sees: sinu osa liidetakse kohta Minu kulud
counted-switch-off = Väljas: sinu osa ei ilmu kohta Minu kulud
counted-groups = Arvestatavad grupid
counted-groups-intro = Sinu osa nendes gruppides liidetakse kohta Minu kulud ja eelarvetesse.
counted-groups-converted = { $from }, teisendatud valuutasse { $to }
counted-groups-encrypted = See valik on krüpteeritud sinu kontoga ja järgib sind sinu seadmetes.
counted-groups-empty = Selles seadmes pole veel ühtegi gruppi.
counted-groups-done = OK
counted-save-failed = Valikut ei õnnestunud salvestada. Proovi uuesti.
tab-budget = Eelarve
budget-title = Eelarved
budget-manage = Halda
budget-left = Jääk { $amount }
budget-over = Ületatud { $amount } võrra
budget-empty = Eelarvet pole
budget-empty-hint = Määra igale kategooriale ülempiir. Sinu osad gruppides lähevad ka arvesse.
budget-define = Määra eelarve
budget-sheet-title = Kuueelarved
budget-sheet-intro = Sinu osale: isiklikud ja grupid. Jäta tühjaks, et kategooriat mitte jälgida.
budget-average = Kesk. viimased 3 kuud: { $amount }
budget-for = Eelarve { $category }
budget-alert = Hoiata mind 80 % juures
budget-alert-hint = Arvutatakse selles seadmes
budget-invalid = Vigane summa: { $category }.
budget-offline = Eelarveid saab salvestada ainult võrgus. Ühenda uuesti ja proovi uuesti.
budget-alert-near = { $category }: { $percent } % kuu eelarvest
budget-alert-over = { $category }: kuu eelarve ületatud
personal-card-budget = { $month } · eelarve { $amount }
my-spending-search-summary =
    { $count ->
        [one] { $count } tulemus
       *[other] { $count } tulemust
    } · { $total } · isiklikud ja grupid, kõik kuud
projects-join = Liitu projektiga
projects-import-tricount = Impordi Tricountist
project-actions = Projekti toimingud

status-ongoing = Käimas
status-closed = Suletud
status-archived = Arhiveeritud

nav-help = Abi
nav-privacy = Privaatsuspoliitika
nav-terms = Kasutustingimused
nav-legal = Juriidiline teave

leave-project-title = Kas lahkud projektist?
leave-project-message = Kaotad sellest seadmest juurdepääsu. Kui ühtegi liiget ei jää, kustutatakse projekt ja kõik selle kulud jäädavalt.

add-project-title = Uus projekt
add-project-name-label = Projekti nimi
add-project-name-placeholder = Minu reis, Korterikaaslased 2024…
add-project-participants = Osalejad
add-project-participant-name = Osaleja nimi
add-project-participant-placeholder = Clark Kent
add-project-offline = Võrguühenduseta ei saa projekti luua. Loo ühendus uuesti ja proovi uuesti.
add-project-name-required = Projektil peab olema nimi.

join-link-label = Jagamislink
join-link-hint = Link sisaldab dekrüpteerimisvõtit - kopeeri see tervikuna.
join-invalid-link = See link ei kehti. Kleebi kogu jagamislink, kaasa arvatud osa pärast #-märki.
join-wrong-project = See link kuulub teisele projektile.

import-tricount-link-label = Tricounti link või võti
import-tricount-key-required = Sisesta Tricounti link või võti.
import-tricount-encryption-failed = Krüpteerimine ebaõnnestus.
import-tricount-unimportable = Midagi ei imporditud: selles Tricountis on Tricounti kontoga liikmeid või summasid, mis ei klapi (mõjutatud kirjeid: { $count }).

### Expenses

save = Salvesta
saving = Salvestamine…
adding = Lisamine…
link-copied = Link kopeeritud
missing-encryption-key = Krüpteerimisvõti puudub.
missing-encryption-key-title = Krüpteerimisvõti puudub
missing-encryption-key-hint = Kasutatud link ei sisalda selle projekti dekrüpteerimiseks vajalikku võtit. Kasuta täielikku linki, mille projekti looja jagas.
project-locked-hint = Sellel seadmel pole selle projekti võtit. Avamiseks ava selle jagamislink.
expenses-unreadable = Mõnda kirjet ei saa sellel seadmel lugeda ja need pole summadesse arvestatud. Nende nägemiseks uuenda rakendust.
project-unlock = Ava lukust
project-no-local-data-hint = Selle projekti andmete esmakordseks laadimiseks logi sisse.
project-gone-title = Seda projekti enam pole
project-gone-hint = See kustutati, kui viimane liige lahkus. Jagamislink enam ei tööta, isegi kui selle uuesti avad.

expense-add = Lisa kulu
transfer-add = Lisa ülekanne
expense-edit-title = Muuda kulu
expense-category = Kategooria
expense-category-auto = Auto · { $emoji }
emoji-picker-label = Emoji
emoji-none = Ilma emojita
expense-currency = Summa valuuta
amount-op-add = Pluss
amount-op-subtract = Miinus
amount-op-multiply = Korruta
amount-op-divide = Jaga
amount-op-equals = Võrdub
amount-op-done = Valmis
expense-rate = Vahetuskurss (valikuline)
expense-rate-hint = Jäta tühjaks, et kasutada Euroopa Komisjoni (InforEuro) kurssi kuule { $month }: 1 { $from } = { $rate } { $to }.
expense-rate-invalid = Sisesta vahetuskurss, mis on suurem kui 0.
expense-rate-unavailable = Automaatset kurssi pole - sisesta see käsitsi.
expense-delete-title = Kustuta kulu
expense-delete-message = „{ $name }“ kustutatakse jäädavalt. Seda ei saa tagasi võtta.
expense-inconsistent-amounts = Summad ei klapi
expenses-empty = Kulusid pole
expenses-empty-hint = Alusta kulude lisamisega allolevast nupust
expenses-show-more = Näita rohkem ({ $count } veel)

expense-type-expense = Kulu
expense-type-transfer = Ülekanne
expense-type-gain = Tulu
expense-type-group = Tüüp
expense-paid-by = maksis
expense-sent-by = saatis
expense-contributed-by = panustas

expense-name-required = Nimi on kohustuslik.
expense-amount-not-positive = Summa peab olema suurem kui 0.
expense-no-payer = Vali vähemalt üks maksja.
expense-no-debtor = Vali vähemalt üks võlgnik.
expense-invalid-date = See kuupäev ei kehti.
expense-payers-mismatch = Maksjate summa on { $sum }, mis ei klapi kulu summaga ({ $total }).
expense-debtors-mismatch = Võlgnike summa on { $sum }, mis ei klapi kulu summaga ({ $total }).

participants-none = Mitte keegi
participants-everyone = Kõik ({ $count })
participants-some = { $count } / { $total }
participants-select-all = Vali kõik
participants-by-shares = Osade kaupa
split-amounts = Summad
participants-remaining = { $amount } jäänud
participants-over-by = { $amount } üle
participants-who-paid = Kes maksis?
participants-who-received = Kes sai?
participants-who-transfers = Kes kannab üle?
participants-who-receives = Kes saab?
participants-for-whom = Kellele?

stats-total-expenses = Kulud kokku
stats-my-expenses = Minu kulud

tab-expenses = Kulud
tab-balance = Saldo
tab-reimbursements = Tasaarveldus
balance-gets-back = Saab tagasi
balance-owes = Võlgneb
balance-settled = Tasaarveldatud
reimbursements-empty-title = Kõik tasaarveldatud!
reimbursements-empty-hint = Tasaarvelduse soovitused ilmuvad siia, kui arved ei klapi
reimbursement-owes = { $debtor } võlgneb { $creditor }
reimbursement-record = Tasaarvelda
reimbursement-pay-with = Maksa
reimbursement-pay-shared-by = Jaganud { $name } - enne saatmist kontrolli saaja nime, mida su rakendus näitab.
reimbursement-pay-title = Maksa { $name }
reimbursements-mine-title = Sina võlgned
reimbursements-others-title = Muud tagasimaksed
copy = Kopeeri

user-selection-title = Milline osaleja sina oled?
user-selection-hint = Vali loendist oma nimi.
user-selection-required = Palun vali osaleja.
identity-claimed = Seotud kontoga
identity-claimed-by = { $name } konto
identity-taken-repick = Teine konto on võtnud endale osaleja, keda sa kasutasid. Palun vali teine.
participant-gone-repick = Osaleja, keda sa kasutasid, on projektist eemaldatud. Palun vali teine.

edit-project-title = Muuda projekti
edit-project-new-badge = uus
edit-project-deferred-new-members = uute liikmete lisamine
edit-project-deferred-removals = liikmete eemaldamine
edit-project-offline-deferred = Võrguühenduseta: { $items } rakendatakse ühenduse taastumisel.

export-failed = Eksport ebaõnnestus: { $reason }

history-expense-added = Kulu lisatud: { $name }
history-expense-edited = Kulu muudetud: { $name }
history-expense-deleted = Kulu kustutatud: { $name }
move-expense = Teisalda projekti
move-expense-title = Teisalda…
move-current = Praegune
move-converted = { $currency }, kuu kursiga arvestatud
move-note-personal = Järgmises etapis valid, kes maksis ja kelle eest. Kulu lahkub jaotisest Minu kulud.
move-note-group = Kulu lahkub projektist { $project } kõigi liikmete jaoks ja grupi saldo arvutatakse ümber. Järgmises etapis valid, kes maksis ja kelle eest.
move-banner = { $from } → { $to }
move-submit = Teisalda
move-done = Kulu teisaldati projekti { $project }
move-delete-failed = Kulu lisati teise projekti, kuid on endiselt projektis { $project }: kustuta see seal.
history-expense-moved = Kulu teisaldati teise projekti: { $name }
personal-badge = Isiklik
move-members =
    { $count ->
        [one] { $count } liige
       *[other] { $count } liiget
    }
history-project-edited = Projekt muudetud: { $name }
history-name-changed = Nimi: „{ $from }“ → „{ $to }“
history-description-added = Kirjeldus lisatud: „{ $value }“
history-description-removed = Kirjeldus eemaldatud: „{ $value }“
history-description-changed = Kirjeldus: „{ $from }“ → „{ $to }“

### Sweep

field-amount = Summa
expense-name-placeholder = Restoran, toidupood…
expense-actions = Kulu toimingud
expense-your-share = Sinu osa
expense-your-share-value = Sinu osa: { $amount } { $currency }
expense-share-percent = { $percent }%
expense-added-by = Lisas { $name }, { $date }
expense-inconsistent-detail = Summad ei klapi: makstud { $paid }, võlgu { $owed }, kulu suurus { $total }. Parandamiseks muuda kulu.
missing-access-key = Juurdepääsuvõti puudub. Ava see projekt selle jagamislingi kaudu.
filter-all = Kõik
filter-my-payments = Minu maksed
filter-my-debts = Minu võlad
expense-filter-group = Filter
expenses-search = Otsi kulusid
expenses-no-match = Ükski kulu ei sobi.
expenses-search-open = Otsi
expenses-search-clear = Tühjenda otsing
expenses-search-summary =
    { $count ->
        [one] { $count } tulemus
       *[other] { $count } tulemust
    } · { $total } · kõik kuud
participants-shares-for = { $name } osad
participants-amount-for = { $name } summa
reimbursement-add = Lisa tasaarveldus
project-forget = Eemalda minu loendist
project-history-title = Ajalugu
history-kind-add = Lisatud
history-kind-delete = Kustutatud
history-kind-edit = Muudetud
export = Ekspordi
export-json = Ekspordi JSON
export-csv = Ekspordi CSV
share-link = Jaga
copy-link-failed = Linki ei õnnestunud kopeerida
open-in-app = Ava rakenduses
not-found-title = Lehte ei leitud
not-found-back = Tagasi projektide juurde

### Charts

charts-period = Periood
period-all = Kõik
period-month = Kuu
period-3months = 3 kuud
period-year = Aasta
period-custom = Kohandatud
charts-tab-categories = Kategooriad
charts-tab-trends = Trendid
charts-total-spent = Kokku kulutatud
charts-avg-per-person = Keskm. inimese kohta
charts-expense-count =
    { $count ->
        [one] { $count } kulu
       *[other] { $count } kulu
    }
charts-nothing-to-show = Pole midagi näidata
charts-my-share-skipped =
    { $count ->
        [one] 1 projekti ei arvestata - osalejat pole valitud või selle andmed ei laadinud.
       *[other] { $count } projekti ei arvestata - osalejat pole valitud või nende andmed ei laadinud.
    }

### Categories

category-food = Toit
category-transport = Transport
category-accommodation = Majutus
category-leisure = Vaba aeg
category-shopping = Ostud
category-services = Teenused
category-parties-gifts = Peod ja kingitused
category-other = Muu
categories-manage = Halda kategooriaid
categories-title = Kategooriad
categories-yours = Sinu kategooriad
categories-builtin = Vaikimisi
categories-add = Lisa kategooria
categories-in-parent = kategoorias { $parent }
categories-renamed-from = ümber nimetatud · algselt { $name }
categories-hidden-hint = peidetud · kulud lähevad kategooriasse { $other }
categories-always-shown = alati nähtav
categories-hidden = Peidetud
categories-new = Uus kategooria
categories-edit = Muuda kategooriat
categories-parent = Arvestatakse kategoorias
categories-parent-hint = Kõigi sinu projektide statistikas arvestatakse { $name } kategoorias { $parent }.
categories-create = Loo
categories-hide = Peida see kategooria
categories-hide-hint = Selle kulud kuvatakse kõigile liikmetele kategoorias { $other }.
categories-delete-title = Kas kustutada „{ $name }”?
categories-delete-body = Asjaomased kulud ({ $count }) lähevad tagasi kategooriasse { $parent }. Ühtegi kulu ei kustutata.
categories-delete-body-unknown = Selle kulud lähevad tagasi kategooriasse { $parent }. Ühtegi kulu ei kustutata.
categories-delete-warning = Kategooria kaob kõigi projekti liikmete jaoks.
categories-summary-created = Loodud: { $count }
categories-summary-renamed = Ümber nimetatud: { $count }
categories-summary-hidden = Peidetud: { $count }
categories-summary-default = Omi kategooriaid pole
categories-offline = Kategooriaid saab hallata ainult võrgus. Ühenda uuesti ja proovi uuesti.
categories-error-too-many = Projektis saab olla kuni 20 oma kategooriat.
categories-error-full = Projekt on jõudnud kategooriate piirini.
categories-error-name = Anna kategooriale nimi.
categories-error-missing = Seda kategooriat on vahepeal muudetud. Sulge ja proovi uuesti.
history-category-created = Kategooria loodud: { $name }
history-category-edited = Kategooria muudetud: { $name }
history-category-deleted = Kategooria kustutatud: { $name }
charts-project = Projekt
charts-all-projects = Kõik projektid
charts-date-from = Alates
charts-date-to = Kuni
charts-total = Kokku
charts-tab-people = Inimesed
charts-tab-projects = Projektid
charts-scope = Kelle kulud
charts-scope-group = Grupp
charts-scope-me = Mina
charts-currency = Valuuta
charts-my-share = Minu osa
charts-share-of-total = { $pct }% summast { $total }
charts-i-paid = Maksin
charts-paid-more = { $amount } rohkem kui minu osa
charts-paid-less = { $amount } vähem kui minu osa
charts-paid-even = Täpselt minu osa
charts-part-title = Sinu osa igas kategoorias
charts-part-desc = Hall on grupi kulu, värv sinu tarbitu.
charts-breakdown-title = Jaotus kategooriate kaupa
charts-breakdown-desc = Puuduta sektorit või rida, et näha kulusid.
charts-of-total = { $amount } / { $total }
charts-show-all = Näita kõiki ({ $count })
charts-show-less = Näita vähem
charts-spend-title = Kulud ajas
charts-group-by = Rühmita
bucket-day = Päev
bucket-week = Nädal
bucket-month = Kuu
charts-avg = kesk.
charts-cat-title-day = { $category }, päev päeva järel
charts-cat-title-week = { $category }, nädal nädala järel
charts-cat-title-month = { $category }, kuu kuu järel
charts-cat-desc = Vali kategooria, et seda ajas jälgida.
charts-running-title = Kogusumma
charts-running-desc = Alates { $date }.
charts-avg-per-day = { $amount } / päevas keskmiselt
charts-avg-per-week = { $amount } / nädalas keskmiselt
charts-avg-per-month = { $amount } / kuus keskmiselt
charts-people-title = Kes grupi kinni maksis
charts-people-desc = Mida igaüks maksis, kõrval see, mida ta tarbis.
charts-paid = Makstud
charts-fair-share = Õiglane osa
charts-you = (sina)
charts-net-more = maksis rohkem
charts-net-less = maksis vähem
charts-balance-title = Sinu saldo ajas
charts-balance-desc = Joonest kõrgemal võlgneb grupp sulle. Allpool võlgned sina grupile.
charts-owed = Sulle võlgnetakse
charts-owe = Sa võlgned
charts-projects-title = Sinu osa projektide kaupa
charts-projects-desc = Summad on valuuta kaupa.
history-empty = Sündmusi pole
history-by = { $name }
not-found-hint = Seda lehte pole olemas või see on teisaldatud.
payers-title-paid-by = Maksis
payers-title-sender = Saatja
payers-title-contributors = Panustajad
debtors-title-debtors = Võlgneb
debtors-title-recipients = Saajad
debtors-title-beneficiaries = Kasusaajad
debtors-title-for = Kellele
debtors-title-recipient = Saaja
expense-split-between = jagatud { $count } vahel
expense-people =
    { $count ->
        [one] { $count } inimene
       *[other] { $count } inimest
    }

### Welcome

welcome-title = Sinu arvepidamine ei puutu kellessegi teise.
welcome-subtitle = Jaga kulusid sõpradega.
welcome-note = Tasuta. Kontot pole vaja. Reklaame pole.
welcome-link-title = Üks link ja kõik saavad osaleda.
welcome-link-body = Kellelgi pole vaja kontot luua.
welcome-link-account = Konto? Pole kunagi kohustuslik. Sellega leiad oma projektid teisest seadmest, kutsud sõpru rakendusest ja jagad oma makseandmeid.
welcome-demo-project = Nädalavahetus Lyonis
welcome-private-title = Keegi ei saa sinu arveid lugeda. Isegi mitte meie.
welcome-private-body = Nimed, summad, projektid: kõik krüpteeritakse sinu seadmes. Võti on ainult sinul.
welcome-private-names = Nimed
welcome-private-amounts = Summad
welcome-private-projects = Projektid
welcome-scan-title = Pildista tšekki.
welcome-scan-body = Summa, kuupäev ja kategooria täituvad ise. Kõik toimub sinu telefonis. Fotot ei säilitata.
welcome-eu-title = 100% Euroopa
welcome-no-ads = Ei reklaame
welcome-no-trackers = Ei jälgijaid
welcome-step = Samm { $current }/{ $total }
welcome-next = Edasi
welcome-skip = Jäta vahele
welcome-start = Alusta
welcome-how-it-works = Kuidas see täpselt töötab?

### Help

help-intro = Levinud küsimus? Vastuse avamiseks puuduta.
help-create-project-q = Kuidas projekti luua?
help-create-project-a = Puuduta avakuval all olevat nuppu +. Anna projektile nimi, vali valuuta ja ongi valmis.
help-add-participants-q = Kuidas osalejaid lisada?
help-add-participants-a = Ava projekt ja lisa osalejad liikmete loendist. Iga osaleja võib kulu eest maksta või selle eest võlgu olla.
help-share-project-q = Kuidas projekti jagada?
help-share-project-a = Jaga projekti URL-i (aadressiribal olevat). Igaüks, kel on link, saab projekti vaadata ja muuta.
help-add-expense-q = Kuidas kulu lisada?
help-add-expense-a = Puuduta projektis +, sisesta summa, määra, kes maksis ja kelle vahel jagada. Võid valida ka tänasest erineva kuupäeva.
help-types-q = Mis vahe on kulul, ülekandel ja tulul?
help-types-expense = - ühe inimese tehtud ost, mis jagatakse mitme vahel.
help-types-transfer = - tagasimakse ühelt inimeselt teisele, ilma jagamiseta.
help-types-gain = - saadud raha (tagastus, kingitus), mis jagatakse mitme inimese vahel.
help-past-date-q = Kas kulu saab dateerida minevikku?
help-past-date-a = Jah, kuupäevaväli on vaba. Kirje loomise aeg salvestatakse eraldi.
help-who-owes-q = Kuidas Counted arvutab, kes kellele võlgneb?
help-who-owes-a = Counted arvutab iga osaleja netosaldo (mida ta ette maksis miinus mida võlgneb) ja pakub siis lühima ülekannete jada, mis kõik tasaarveldab.
help-minimal-transfers-q = Miks on soovitatud ülekannete arv minimaalne?
help-minimal-transfers-a = Algoritm seob esmalt saldod, mis täpselt tasakaalustuvad, ja käib siis ülejäänud läbi suurimast võlausaldajast suurima võlgnikuni. Tulemus: vähem ülekandeid kõige tasaarveldamiseks.
help-import-tricount-q = Kuidas projekti Tricountist importida?
help-import-tricount-a = Puuduta avakuval all olevat nuppu „+“ ja seejärel
help-import-tricount-b = Kleebi imporditava Tricounti jagamislink.
help-encryption-q = Kas minu andmed on krüpteeritud?
help-encryption-a = Jah. Counted ühendab kaks tagatist:
help-encryption-e2ee-term = Otsast otsani krüpteerimine
help-encryption-e2ee-def = - kõik sinu ja serveri vahel liigub krüpteeritult.
help-encryption-zero-term = Nulljuurdepääs
help-encryption-zero-def = - sina krüpteerid andmed enne saatmist ja server salvestab ainult šifreeritud teksti. Meil pole võimalust seda lugeda.
help-encryption-see = Üksikasju vaata
help-forgot-password-q = Mis juhtub, kui parooli unustan?
help-forgot-password-warning = Keegi ei saa seda sinu eest taastada.
help-forgot-password-a = Sinu parool kaitseb sinu krüpteerimisvõtmeid ja seda ei saadeta meile kunagi: keegi - ka mitte meie - ei saa sinu projekte sinu eest dekrüpteerida. Hoia seda kindlas kohas, ideaalis paroolihalduris.
help-archive-delete-q = Kuidas projekti arhiveerida või kustutada?
help-archive-delete-a = Ava projekti kuval menüü ja vali
help-archive-delete-b = et see peita, kuid alles hoida. Projekt kustutatakse lõplikult, kui viimane liige sellest lahkub.
help-delete-account-q = Kuidas oma kontot kustutada?
help-delete-account-a = Ava Seaded ja kasuta „Kustuta minu konto“. See toimub kohe ja seda ei saa tagasi võtta.
help-contact = Veel küsimusi? Kirjuta meile aadressil

# Receipt scanning (mobile only)
expense-scan = Skanni tšekk
scan-in-progress = Tšeki lugemine…
scan-error-capture = Fotot ei õnnestunud teha. Proovi uuesti või sisesta kulu käsitsi.
scan-error-unreadable = Sellel tšekil pole midagi loetavat. Sisesta kulu käsitsi.
scan-check-amount = Kontrolli kogusummat - see ei olnud selgelt trükitud.
scan-take-photo = Tee foto
scan-choose-photo = Vali foto
expense-converted-from = Makstud { $amount } { $from } · 1 { $from } = { $rate } { $to }
project-currency = Valuuta
project-currency-locked = Valuuta määratakse projekti loomisel.
currency-search = Otsi valuutat

update-required-title = Vajalik on uuendus
update-required-body = See Countedi versioon on serveriga suhtlemiseks liiga vana. Rakenduse kasutamise jätkamiseks uuenda seda.
update-required-button = Uuenda

notifications-label = Teavitused
notifications-title = Teavitused
notifications-empty = Midagi uut pole
notifications-friend-request = Sõbrakutse

friends-title = Sõbrad
friends-anonymous-body = Sõbrad on seotud sinu kontoga. Logi sisse, et lisada inimesi ja kutsuda neid oma projektidesse ilma linki jagamata.
friends-add-title = Lisa sõber
friends-add-hint = Kui tal on konto, näeb ta sinu kutset rakenduses. Kui ei, anna talle teada: ta leiab selle, kui registreerub selle aadressiga.
friends-add-button = Lisa
friends-add-from-project = Lisa sõbraks
friends-request-sent = Kutse saadetud
friends-no-account-key = Sõprade haldamiseks selles seadmes logi uuesti sisse.
friends-incoming-title = Kutsed
friends-accept = Võta vastu
friends-decline = Keeldu
friends-list-title = Minu sõbrad
friends-list-empty = Sõpru veel pole. Lisa keegi ülal e-posti kaudu või ühisest projektist.
friends-remove = Eemalda
friends-remove-confirm-title = Eemalda sõber
friends-remove-confirm-message = { $email } ei ole enam sinu sõprade seas ja sina tema omade seas. Kumbki teist saab hiljem uue taotluse saata.
friends-no-key = Pole veel valmis
friends-key-changed = Kood muutus
friends-key-changed-hint = Sõbra turvakood muutus. Nii juhtub, kui ta turvab oma konto, või kui tema võti vahetati välja. Kutsed temaga on blokeeritud, kuni loete teineteisele uue koodi ette, silmast silma või telefonis, ja sa puudutad „Kood klapib“.
friends-fingerprint = Turvakood
friends-fingerprint-hint = Kaks sõpra, kes loevad teineteisele sama turvakoodi, teavad, et keegi ei ole nende vahel - isegi mitte meie server.
friends-key-mismatch = Server saatis sinu kontole avaliku võtme, mis ei sobi sinu privaatvõtmega. Sinu turvakood on peidetud ja kutsed on selles seadmes välja lülitatud: keegi võib serveriga manipuleerida.
keypair-mismatch = Võtmed, mille server sinu konto jaoks saatis, ei klapi selle seadme omadega. Kutsed ja võtmevahetused on siin välja lülitatud, kuni need klapivad: keegi võib serverit manipuleerida.
friends-outgoing-title = Saadetud
friends-outgoing-hint = Ootab vastust. Näed neid oma sõprade seas, kui nad on kutse vastu võtnud.
friends-withdraw = Tühista
friends-notify = Anna sõnumiga teada
friends-notify-label = Anna teada: { $email }
friends-notify-message = Võtan Countedi kasutusele meie ühiste kulude jaoks. Tasuta registreerumine lehel https://counted.fr aadressiga { $email }: minu sõbrakutse ootab seal.
friends-notify-copied = Sõnum kopeeritud
friends-notify-copy-failed = Sõnumit ei õnnestunud kopeerida
guests-hint-title =
    { $count ->
        [one] { $count } osaleja pole veel projektiga liitunud
       *[other] { $count } osalejat pole veel projektiga liitunud
    }
guests-hint-later = Hiljem
guests-hint-send = Kutsu nad
guests-send-title = Saada projekt osalejatele
guests-send-friend = Kutsu Countedi sõber
guests-send-friend-badge = Ilma lingita
guests-send-friend-hint = Kui tal on juba konto, saab ta krüpteeritud kutse; midagi ei liigu läbi sinu sõnumirakenduse.
guests-send-link = Jaga projekti linki
guests-send-link-hint = Neile, kellel pole kontot. Projekt avaneb ühe puudutusega.
guests-send-link-warning = Igaüks, kellel on see link, saab projekti lugeda ja muuta. Sinu sõnumirakendus näeb seda ka.
invite-friends-title = Kutsu sõpru
invite-friends-hint = Projekti võti krüpteeritakse iga sõbra jaoks selles seadmes. Server ei näe seda kunagi.
invite-friends-empty = Kutsutavaid sõpru veel pole.
invite-friends-button = Kutsu
invite-sent = { $count ->
    [one] Kutse saadetud
   *[other] { $count } kutset saadetud
}
invitation-badge = Kutse
invitation-to = Liitu projektiga „{ $name }“
invitation-to-unnamed = Liitu projektiga
invitation-unreadable = Seda kutset ei saa selles seadmes avada
invitation-from = Saatja: { $email }
invitation-accept = Liitu
invitation-decline = Keeldu

# Participants in the create and edit modals, and the "who are you?" picker - see
# docs/plans/friends.md §11.
participants-you-label = Sinu nimi selles projektis
participants-you-badge = Sina
participants-others = Teised osalejad
participants-removed = Eemaldatud
participant-restore = Taasta
participants-empty = Lisa osalejad allpool.
participants-duplicate = „{ $name }“ on juba loendis.
participants-input-label = Lisa sõber või kirjuta nimi
participants-input-placeholder = Sõber või mis tahes nimi
participants-suggest-friend = Sõber · liitub nimega „{ $name }“, saab kutse
participants-suggest-not-ready = Sõber · pole veel valmis
participants-suggest-guest = Lisa „{ $text }“
participants-suggest-guest-sub = Külaline
participants-friends = Sinu sõbrad
participants-all-friends = Kõik sõbrad
participants-login-hint = Logi sisse, et lisada inimesi otse oma sõbraloendist.
participants-invite-badge = Kutsu
participants-guest-badge = Külaline
participants-rename = Nimeta ümber: { $name }
participants-remove = Eemalda: { $name }
participants-rename-label = Uus nimi
participants-rename-save = Salvesta nimi
participants-rename-hint = Nimi, mida kõik selles projektis näevad. Kutse läheb endiselt aadressile { $email }.
participants-invited-badge = Kutsutud
participants-invited-sub = { $email } · pole veel vastu võetud
participants-invited-pending = Kutset pole veel vastu võetud
participants-unlinked = Pole kontoga seotud
add-project-create-invite = Loo ja kutsu: { $count }
edit-project-save-invite = Salvesta ja kutsu: { $count }
edit-project-you-are = Selles seadmes oled sa { $name }
edit-project-no-identity = Sa pole veel valinud, kes sa oled
edit-project-switch = Vaheta
edit-project-choose = Vali
invite-failed = Neid kutseid ei õnnestunud saata: { $emails }
invite-again = Kutsu uuesti
friend-picker-title = Lisa sõpru
user-selection-invited-hint = { $email } kutsus sind projekti „{ $project }“.
user-selection-suggested = Soovitatud
user-selection-suggested-sub = { $email } lisas sind selle nimega
user-selection-confirm-as = Mina olen { $name }
user-selection-missing = Sinu nime pole siin? Palu osalejal sind projekti seadetes lisada.

## Korduvad kulud

repeat-label = Korda
repeat-none = Ei kordu
repeat-weekly = Iga nädal
repeat-biweekly = Iga 2 nädala tagant
repeat-monthly = Iga kuu
repeat-quarterly = Iga 3 kuu tagant
repeat-yearly = Iga aasta
repeat-every-weeks = Iga { $count } nädala tagant
repeat-every-months = Iga { $count } kuu tagant
repeat-every-years = Iga { $count } aasta tagant
repeat-custom = Kohandatud…
repeat-every = Intervall:
repeat-unit-weeks = nädalat
repeat-unit-months = kuud
repeat-unit-years = aastat
repeat-on-weekday = päeval: { $weekday }
repeat-on-day = kuu { $day }. päeval
repeat-on-day-month = { $day }. { $month }
repeat-month-end = Lühematel kuudel langeb see viimasele päevale.
repeat-ends = Lõpeb
repeat-ends-never = Mitte kunagi
repeat-ends-on = Kuupäeval
repeat-ends-after = Pärast
repeat-fewer = Vähem
repeat-more = Rohkem
repeat-last-on = viimane { $date }
repeat-variable = Summa muutub iga kord
repeat-variable-hint = Iga kord lisatakse viimase summaga ja märgitakse „kinnitamiseks”.
repeat-done = Valmis
repeat-no-end = Lõputa
repeat-until = Kuni { $date }
repeat-occurrences = Kordusi: { $count }
repeat-offline = Vajab ühendust. Kulu ennast saab siiski lisada.
repeat-foreign = Kordub summas { $amount } { $currency }, arvestatud korra tänase kursiga. Saad hoiatuse, kui kurss muutub üle 5 %.
repeat-backfill = Algab minevikus. Praegu lisatud kulud: { $count }.
add-and-repeat = Lisa ja korda
weekday-1 = esmaspäev
weekday-2 = teisipäev
weekday-3 = kolmapäev
weekday-4 = neljapäev
weekday-5 = reede
weekday-6 = laupäev
weekday-7 = pühapäev
recurring-title = Korduvad kulud
recurring-strip = Korduvad kulud: { $count }
recurring-next = Järgmine: { $name }, { $date }
recurring-to-confirm = Kinnitada: { $count }
recurring-per-month = Kuus umbes
recurring-your-share = Sinu osa
recurring-active = Aktiivsed
recurring-paused = Peatatud
recurring-finished = Lõppenud
recurring-paid-by = maksab { $name }
recurring-next-on = Järgmine { $date }
recurring-progress = { $done } / { $total }
recurring-rate-badge = Kurss muutus { $percent } %
recurring-empty = Midagi ei kordu veel. Vali kulu lisamisel „Korda”: üür, tellimused, arved.
recurring-next-ones = Järgmised
recurring-added-so-far = Seni lisatud
recurring-set-up-by = Lõi
recurring-pause = Peata
recurring-resume = Jätka
recurring-stop = Lõpeta kordamine
recurring-stop-title = Lõpetada „{ $name }”?
recurring-stop-message = See ei kordu enam. Juba lisatud kulud jäävad alles.
recurring-resume-title = Jätkata „{ $name }”?
recurring-resume-message = Järgmine { $date }. Pausi ajal vahele jäänud kuupäevi ei lisata.
recurring-edit-title = Muuda korduvat kulu
recurring-edit-banner = Muudatused kehtivad alates { $date }. Juba lisatud kulud jäävad samaks.
recurring-next-on-label = Järgmine
recurring-next-too-early = Järgmine kuupäev peab olema pärast viimast juba lisatud kulu.
recurring-use-stop = Selle lõpetamiseks kasuta korduva kulu juures „Lõpeta kordamine”.
recurring-drift = { $currency } kurss on loomisest saadik muutunud { $percent } %. Iga kord lisatakse endiselt summas { $amount } { $project_currency } (1 { $currency } = { $rate }). Tänase kursiga oleks see { $today_amount } { $project_currency }.
recurring-use-rate = Kasuta tänast kurssi
recurring-keep = Jäta { $amount } { $currency }
recurring-added = Lisatud korduvad kulud: { $count }
recurring-blocks-removal = Osalejat { $name } ei saa veel eemaldada: kuulub kuludesse { $rules }. Eemalda { $name } neist või lõpeta need ja salvesta uuesti.
history-recurring-added = Lisatud automaatselt: { $name } ({ $date })
history-recurring-created = Korduv kulu loodud: { $name }
history-recurring-edited = Korduv kulu muudetud: { $name }
history-recurring-paused = Korduv kulu peatatud: { $name }
history-recurring-resumed = Korduv kulu jätkub: { $name }
history-recurring-stopped = Korduv kulu lõpetatud: { $name }
occurrence-recurring = Korduv kulu
occurrence-auto = Lisatud automaatselt korduvast kulust.
occurrence-auto-next = Lisatud automaatselt korduvast kulust. Järgmine { $date }.
occurrence-auto-stopped = Lisatud automaatselt korduvast kulust, mis on vahepeal lõpetatud.
occurrence-manage = Halda
estimate-badge = Kinnitada
expense-pending = Ootel
estimate-title = Summa vajab kinnitamist.
estimate-body = Lisatud eelmise summaga. Sisesta õige, kui see selgub.
estimate-confirm = Kinnita summa
apply-to = Rakenda
apply-this-only = Ainult sellele kulule
apply-and-next = Sellele ja järgmistele
apply-and-next-hint = Korduv kulu muutub alates { $date }
apply-rule-failed = Kulu salvestati, kuid korduvat kulu ei muudetud.
occurrence-delete-message = „{ $name }” ({ $date }) kustutatakse jäädavalt. Kordamine jätkub ja see kuupäev ei tule tagasi.
occurrence-delete-one = Kustuta ainult see
occurrence-delete-stop = Kustuta ja lõpeta kordamine
error-recurring-clock = Selle seadme kell käib ette. Kontrolli kuupäeva ja kellaaega.
error-recurring-not-found = Seda korduvat kulu enam ei ole.
error-recurring-stale = Keegi muutis vahepeal seda korduvat kulu. See laaditi uuesti: kontrolli ja salvesta uuesti.
error-too-many-recurring = Projektil on juba 50 korduvat kulu. Lõpeta üks mittevajalik, et uus lisada.
error-user-in-recurring = See osaleja kuulub korduvasse kulusse. Eemalda ta sealt või lõpeta kulu kõigepealt.

home-owed-to-you = Sulle võlgu
home-you-owe = Sina oled võlgu
home-details-show = Näita üksikasju
home-details-hide = Peida üksikasjad
home-all-settled = Kõik on tasutud
home-pay = Maksa
home-remind = Meenuta
home-remind-message = Väike meeldetuletus: projektis { $project } on veel tagasimakseid tegemata.
home-row-owed = sulle võlgu
home-row-owe = oled võlgu
home-row-settled = Tasutud
home-totals-partial = Ilma vahetuskursita projektideta
error-account-key-changed = Sinu konto võti muutus teises seadmes. Logi uuesti sisse, et jätkata.
settings-security = Turvalisus
settings-change-password = Muuda parooli
settings-change-password-hint = Sinu projektid ja andmed jäävad samaks. Sinu teised seadmed logitakse välja.
change-password-title = Muuda oma parooli
change-password-current = Praegune parool
change-password-new = Uus parool
change-password-confirm = Kinnita uus parool
change-password-mismatch = Kaks uut parooli ei ühti.
change-password-submit = Muuda parooli
change-password-submitting = Muudan…
change-password-done = Parool muudetud. Sinu teised seadmed logiti välja.
error-rotation-incomplete = Sinu konto muutus vahepeal. Proovi uuesti.
error-key-lifecycle-unavailable = Pole veel saadaval: kõigepealt vajavad kõik rakenduse uusimat versiooni.
settings-compromised = Keegi võib teada minu parooli
settings-compromised-hint = Uus parool, uued võtmed ja kõik teised seadmed logitakse välja.
settings-compromised-open = Turva mu konto
compromised-title = Turva oma konto
compromised-body = Sinu parool ja konto võtmed asendatakse ning kõik teised seadmed logitakse välja. Sinu sõbrad näevad, et su turvakood muutub: võrrelge seda uuesti.
compromised-keeps = See, kellel oli sinu parool, jätab endale selle, mille ta juba avas: projektid, mida ta seni lugeda sai.
compromised-submit = Asenda parool ja võtmed
compromised-done = Sinu konto on turvatud. Palu sõpradel võrrelda sinu uut turvakoodi.
friends-new-fingerprint = Uus kood:
friends-code-matches = Kood klapib
friends-code-trusted = Kood kinnitatud.
settings-recovery-key = Taastevõti
settings-recovery-key-none = Ilma selleta läheb unustatud parooliga kaotsi seaded, makseandmed ja sõprade koodid ning taastamine võtab 3 päeva.
settings-recovery-key-set = Loodud. Kui unustad parooli, saad sellega konto koos kõigega tagasi.
settings-recovery-key-pending = Uus võti asendab selle { $when }. Polnud sina? Tühista see praeguse võtmega.
recovery-key-create = Loo
recovery-key-replace = Asenda
recovery-key-cancel-replacement = Tühista uus võti
recovery-key-title = Sinu taastevõti
recovery-key-body = Kirjuta see üles või salvesta kohta, kuhu ainult sina ligi pääsed. Counted ei saa seda uuesti näidata ega sinu eest taastada.
recovery-key-copy = Kopeeri
recovery-key-copied = Kopeeritud
recovery-key-save = Salvesta failina
recovery-key-print = Prindi
recovery-key-next = Jätka
recovery-key-confirm-body = Koopia kontrollimiseks sisesta võtme rühm { $n }.
recovery-key-confirm-wrong = See rühm ei klapi. Vaata oma koopiat uuesti.
recovery-key-password-body = Võtme kontole salvestamiseks sisesta oma parool.
recovery-key-submit = Salvesta võti
recovery-key-done = Taastevõti salvestatud.
recovery-key-replacement-done = Uus võti salvestatud. See asendab praeguse 72 tunni pärast; seni töötab praegune.
recovery-key-field = Taastevõti
recovery-key-file-intro = Countedi taastevõti
recovery-key-cancel-done = Uus võti on tühistatud.
compromised-new-key = Salvesta uus taastevõti kohe: see asendab vana 72 tunni pärast ja seni töötab vana.
pending-reset-title = Sinu konto lähtestamist on palutud
pending-reset-body = See jõustub { $when }: siis lähevad kaotsi seaded, makseandmed ja sõprade koodid ning seadmed peavad uuesti sisse logima. Polnud sina? Tühista see praeguse parooliga.
pending-reset-cancel = Tühista lähtestamine
pending-reset-cancelled = Lähtestamine on tühistatud.
login-forgot = Unustasid parooli?
forgot-title = Unustatud parool
forgot-body = Sisesta oma e-post. Kui sellel on konto, saadame lingi uue parooli valimiseks.
forgot-submit = Saada link
forgot-sent = Kui sellel aadressil on konto, on link teel. See aegub 24 tunni pärast.
reset-title = Saa oma konto tagasi
reset-with-key-body = Sisesta taastevõti ja uus parool: kõik tuleb tagasi nii, nagu oli.
reset-submit-key = Taasta mu konto
reset-no-key = Mul pole taastevõtit
reset-without-key-title = Lähtesta ilma taastevõtmeta
reset-without-key-body = Sinu konto alustab 72 tunni pärast uue parooliga uuesti. Projektid tulevad tagasi sinu seadmetest ja jagamislinkidest, seaded, makseandmed ja sõprade koodid mitte. Seni töötab praegune parool ja saab lähtestamise tühistada.
reset-name = Sinu nimi
reset-submit-request = Lähtesta mu konto
reset-requested = Valmis. Uus parool töötab alates { $when }. Seni töötab praegune parool ja saab selle tühistada.
reset-pending-already = Lähtestamine juba ootab kuni { $when }. Uus taotlus alustab 72 tundi uuesti.
recovered-done = Sinu konto on tagasi. Teised seadmed logiti välja.
login-reset-undoable = See parool asendati viimase 30 päeva jooksul lähtestamisega. Kui sa seda ei palunud, võta see tagasi: konto tuleb tagasi nii, nagu oli.
login-undo-reset = Võta lähtestamine tagasi
reset-undone = Lähtestamine on tagasi võetud. Sinu konto on nagu enne.
error-reset-pending = Uus parool hakkab tööle, kui lähtestamine jõustub, 72 tundi pärast taotlust.
error-account-disabled = See konto kustutati veidi pärast lähtestamist ja ootab hävitamist. Eelmine parool saab lähtestamise veel tagasi võtta.
error-recovery-key-invalid = See taastevõti ei ava seda kontot.
error-project-key-required = See projekt vajab oma võtit. Ava see uuesti jagamislingist.
names-or = { $first } või { $last }
participants-admin-badge = Admin
edit-project-admins = Adminid: { $count }
edit-project-succession = Kui viimane admin projektist lahkub, saab adminiks kõige kauem projektis olnud kontoga liige.
member-sheet-account = Kontoga liige
member-actions = Toimingud: { $name }
member-make-admin = Tee adminiks
member-make-admin-hint = Saab siis projekti võtit muuta ja kedagi eemaldada.
member-made-admin = { $name } on nüüd admin.
group-security = Projekti turvalisus
group-security-admin = Projekti turvalisus · sa oled admin
group-rotate = Muuda projekti võtit
group-rotate-hint = Kui projekti link on lekkinud: vana lingi omanikud kaotavad ligipääsu projektile.
group-remove = Eemalda keegi projektist
group-remove-hint = Võtab ligipääsu ära ja muudab samal ajal võtit.
group-admins-only = Ainult adminitele.
group-admins-only-account = Ainult adminitele, kellel on konto.
group-ask-admin = Küsi projekti adminilt: { $names }.
group-ask-admins = Küsi projekti adminitelt: { $names }.
group-ask-an-admin = Küsi mõnelt projekti adminilt.
group-with-account = Kontoga
group-free = Tasuta
group-sign-in-hint = Projekti admin ({ $names }) saab sulle siis rolli anda.
group-sign-in-hint-any = Projekti admin saab sulle siis rolli anda.
group-no-admin = Sellel projektil pole adminit, seega selle võtit ei saa muuta.
group-hide = Peida keegi
group-hide-hint = Peidab ta loenditest ilma ligipääsu ära võtmata: link ja võti kehtivad edasi.
group-recreate-hint = Sellel projektil pole adminit. Kellegi eemaldamiseks loo uus projekt.
group-recreate = Loo uus projekt
group-no-admin-yet = Sellel projektil pole veel adminit.
group-creator-hint = Ainult sina saad selle projekti adminiks, kui lood konto.
group-creator-elsewhere-hint = Projekti adminiks saab ainult projekti looja, kui ta loob konto. Muidu loo kellegi eemaldamiseks uus projekt.
member-revoke-admin = Võta admini roll ära
member-revoke-admin-hint = Ei saa enam projekti võtit muuta ega kedagi eemaldada.
member-revoked-admin = { $name } pole enam admin.
member-step-down = Loobu admini rollist
member-step-down-hint = Sa ei saa enam projekti võtit muuta ega kedagi eemaldada.
member-stepped-down = Sa pole enam admin.
member-remove = Eemalda projektist
member-remove-hint = Muudab projekti võtit: { $name } ei saa enam midagi uut lugeda.
member-hide = Peida loenditest
error-user-not-admin = See liige pole admin.
error-creator-proof-invalid = See seade ei loonud seda projekti.
error-project-has-admin = Sellel projektil on juba admin.
error-not-project-admin = Seda saavad teha ainult projekti adminid.
error-user-not-claimed = Sellel osalejal pole kontot, seega ei saa ta olla admin.
error-project-key-rotated = Projekti võti muutus. Küsi mõnelt liikmelt uut linki.
error-project-rotating = Projekti võtit muudetakse. Proovi hetke pärast uuesti.
error-rotation-expired = Võtme muutmine võttis liiga kaua aega ja tühistati. Alusta uuesti.
error-rotation-rows-changed = Projekt muutus võtme muutmise ajal. Alusta uuesti.
project-rotated-title = Sul pole enam ligipääsu sellele projektile
project-rotated-body = Admin muutis selle projekti võtit. Küsi temalt uut linki.
project-rotated-open-link = Ava link
rotation-remove-title = Kas eemaldada { $name } projektist?
rotation-rotate-title = Kas muuta projekti võtit?
rotation-remove-body = Liikme eemaldamine tähendab uue võtme loomist. Uut linki tuleb jagada nendega, kellel pole kontot.
rotation-rotate-body = Vana lingi omanikud kaotavad ligipääsu projektile. Uut linki tuleb jagada nendega, kellel pole kontot.
rotation-boxed =
    { $count ->
        [0] Ükski teine liige ei saa võtit automaatselt
        [one] { $count } liige saab võtme automaatselt
       *[other] { $count } liiget saavad võtme automaatselt
    }
rotation-by-link-title = Inimesed, kellele pead uue lingi saatma
rotation-by-link-hint = Neil inimestel pole kontot. Saadad neile lingi kohe pärast seda.
rotation-names-and = { $first } ja { $last }
rotation-names-and-others = { $names } ja veel { $count }
rotation-confirm-remove = Eemalda ja muuda võtit
rotation-confirm-rotate = Muuda võtit
rotation-done-title = Uus võti on kasutusel
rotation-done-removed = { $name } ei ole enam projektis.
rotation-done-rotated = Projekti võti muutus.
rotation-done-boxed =
    { $count ->
        [0] Ükski teine liige ei saanud võtit automaatselt.
        [one] { $count } liige sai võtme automaatselt.
       *[other] { $count } liiget said võtme automaatselt.
    }
rotation-new-link =
    { $count ->
        [0] Projekti uus link
        [one] Uus link saatmiseks · { $count } inimene ilma kontota
       *[other] Uus link saatmiseks · { $count } inimest ilma kontota
    }
rotation-link-renewed = Link uuendati.
rotation-done = Valmis
history-key-changed = Projekti võti muudetud
history-member-removed = { $name } ei ole enam projektis
project-rotated-row-hint = Võti muutus. Küsi uut linki.
