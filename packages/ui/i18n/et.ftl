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
error-generic = Midagi läks valesti. Proovi uuesti.

error-invalid-email = See e-posti aadress ei kehti.
error-invalid-password = See parool ei kehti.
error-password-too-short = Parool peab olema vähemalt 8 tähemärki pikk.
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
login-resend = Saada kinnituslink uuesti
login-resending = Saatmine…
login-resend-sent = Kiri saadetud - vaata postkasti.

register-submit = Loo konto
register-submitting = Loomine…
register-have-account = Konto juba olemas?
register-password-placeholder = Vähemalt 8 tähemärki
register-password-warning = Kirjuta parool üles. Kui selle unustad, ei saa kontot taastada.
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
account-delete-warning = Kohe ja jäädavalt, prügikastita. Jagatud projekti sisestatud kulud jäävad teistele liikmetele nähtavaks - need on osa nende arvepidamisest.
account-delete-confirm-title = Konto kustutamine
account-delete-confirm-message = Sinu konto, seansid ja projektide loend kustutatakse jäädavalt. Ilma paroolita muutuvad jagatud projekti krüpteeritud andmed sulle loetamatuks - seda ei saa tagasi võtta.

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
payment-methods-stale = Sinu makseandmeid muudeti teises seadmes. Need laaditi uuesti — proovi uuesti.
payment-methods-key-missing = Makseandmete haldamiseks logi uuesti sisse.
settings-payment-methods-share-warning = Jagatud makseviis on nähtav kõigile nende projektide liikmetele, kus oled oma nime valinud - kõigile, kel on üks neist linkidest.
payment-method-share = Jaga minu projektidega
payment-method-share-hint = Kuvatakse sinu nime kõrval, kui keegi on sulle võlgu.
payment-method-copy = Kopeeri { $name }
payment-method-copied = Kopeeritud.
payment-method-copy-failed = Kopeerimine ebaõnnestus - vali tekst ja kopeeri käsitsi.

verify-email-checking = E-posti aadressi kinnitamine…
verify-email-welcome = E-post kinnitatud - tere tulemast Countedisse!
verify-email-back-to-login = Tagasi sisselogimisse

### Project status

project-close = Sulge
project-archive = Arhiveeri
project-reopen = Ava uuesti
project-unarchive = Taasta arhiivist

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
field-optional = Valikuline

### Projects

projects-filter-active = Aktiivsed
projects-filter-all = Kõik
projects-count-label = Projektid
projects-empty = Projekte pole
projects-empty-hint = Loo projekt allolevast nupust
projects-offline-banner = Võrguühenduseta andmed - värskendamiseks loo ühendus uuesti.
projects-no-local-data = Kohalikke andmeid pole
projects-no-local-data-hint = Projektide esmakordseks laadimiseks logi sisse.
projects-add = Lisa projekt
projects-create = Loo projekt
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
add-project-remove-participant = Eemalda osaleja
add-project-me-badge = Mina
add-project-thats-me = See olen mina!
add-project-offline = Võrguühenduseta ei saa projekti luua. Loo ühendus uuesti ja proovi uuesti.
add-project-name-required = Projektil peab olema nimi.
add-project-need-two-participants = Lisa vähemalt 2 osalejat.
add-project-pick-yourself = Ütle, milline osaleja sina oled.

join-link-label = Jagamislink
join-link-hint = Link sisaldab dekrüpteerimisvõtit - kopeeri see tervikuna.
join-invalid-link = See link ei kehti. Kleebi kogu jagamislink, kaasa arvatud osa pärast #-märki.
join-wrong-project = See link kuulub teisele projektile.

import-tricount-link-label = Tricounti link või võti
import-tricount-key-required = Sisesta Tricounti link või võti.
import-tricount-encryption-failed = Krüpteerimine ebaõnnestus.

### Expenses

save = Salvesta
saving = Salvestamine…
adding = Lisamine…
link-copied = Link kopeeritud
missing-encryption-key = Krüpteerimisvõti puudub.
missing-encryption-key-title = Krüpteerimisvõti puudub
missing-encryption-key-hint = Kasutatud link ei sisalda selle projekti dekrüpteerimiseks vajalikku võtit. Kasuta täielikku linki, mille projekti looja jagas.
project-locked-hint = Sellel seadmel pole selle projekti võtit. Avamiseks ava selle jagamislink.
project-unlock = Ava lukust
project-no-local-data-hint = Selle projekti andmete esmakordseks laadimiseks logi sisse.
project-gone-title = Seda projekti enam pole
project-gone-hint = See kustutati, kui viimane liige lahkus. Jagamislink enam ei tööta, isegi kui selle uuesti avad.

expense-add = Lisa kulu
transfer-add = Lisa ülekanne
expense-edit-title = Muuda kulu
expense-category = Kategooria
expense-category-auto = Auto · { $emoji }
expense-currency = Summa valuuta
amount-op-add = Pluss
amount-op-subtract = Miinus
amount-op-multiply = Korruta
amount-op-divide = Jaga
amount-op-equals = Võrdub
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
participants-deselect-all = Tühista valik
participants-by-shares = Osade kaupa
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
edit-project-deferred-me = „See olen mina“ valik
edit-project-offline-deferred = Võrguühenduseta: { $items } rakendatakse ühenduse taastumisel.

export-saved = Fail salvestatud:
    { $path }
export-failed = Eksport ebaõnnestus: { $reason }

history-expense-added = Kulu lisatud: { $name }
history-expense-edited = Kulu muudetud: { $name }
history-expense-deleted = Kulu kustutatud: { $name }
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
expense-inconsistent-detail = Summad ei klapi: makstud { $paid }, võlgu { $owed }, kulu suurus { $total }. Parandamiseks muuda kulu.
missing-access-key = Juurdepääsuvõti puudub. Ava see projekt selle jagamislingi kaudu.
filter-all = Kõik
filter-my-payments = Minu maksed
filter-my-debts = Minu võlad
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
charts-tab-per-person = Inimese kohta
charts-tab-trends = Trendid
charts-by-category = Jaotus kategooriate kaupa
charts-per-person = Kulud inimese kohta
charts-categories-by-month = Kategooriad kuude kaupa
charts-total-spent = Kokku kulutatud
charts-avg-per-person = Keskm. inimese kohta
charts-expense-count =
    { $count ->
        [one] { $count } kulu
       *[other] { $count } kulu
    }
charts-clear-category-filter = Tühista kategooriafilter
charts-no-expenses = Kulusid pole.
charts-pick-a-project = Vali projekt, et näha kulusid inimese kohta.
charts-nothing-to-show = Pole midagi näidata
charts-my-share-note = Need summad on sinu osa igast kulust.
charts-my-share-skipped =
    { $count ->
        [one] 1 projekti ei arvestata — osalejat pole valitud või selle andmed ei laadinud.
       *[other] { $count } projekti ei arvestata — osalejat pole valitud või nende andmed ei laadinud.
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
charts-person = Inimene
charts-project = Projekt
charts-all-projects = Kõik projektid
charts-whole-project = Kogu projekt
charts-date-from = Alates
charts-date-to = Kuni
charts-total = Kokku
charts-payments-per-person-by-month = Maksed inimese kohta kuude kaupa
history-empty = Sündmusi pole
history-by = { $name }
not-found-hint = Seda lehte pole olemas või see on teisaldatud.
payers-title-paid-by = Maksis
payers-title-sender = Saatja
payers-title-contributors = Panustajad
debtors-title-debtors = Võlgneb
debtors-title-recipients = Saajad
debtors-title-beneficiaries = Kasusaajad

### Welcome

welcome-title = Sinu arvepidamine ei puutu kellessegi teise.
welcome-subtitle = Jaga kulusid sõpradega.
welcome-e2ee-title = Kõik krüpteeritud
welcome-e2ee-body = Nimed, summad, projektid: kõik krüpteeritakse sinu seadmes. Võti on ainult sinul. Keegi ei saa sinu arveid lugeda. Isegi mitte meie.
welcome-e2ee-note = Loetamatu isegi meile (serveril puudub juurdepääs)
welcome-eu-title = 100% Euroopa
welcome-eu-body = Serverid Saksamaal, kirjad saadetakse Prantsusmaalt. Sinu andmed ei lahku kunagi Euroopa Liidust.
welcome-noads-title = Ei reklaame. Ei jälgijaid.
welcome-noads-body = Me ei kogu midagi ega müü sinu andmeid. See pole meie mudel.
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
help-forgot-password-warning = Sinu andmed lähevad jäädavalt kaotsi.
help-forgot-password-a = Krüpteerimisvõti tuletatakse sinu paroolist, seega lähtestamine pole võimalik: keegi - ka mitte meie - ei saa ilma selleta sinu projekte dekrüpteerida. Hoia seda kindlas kohas, ideaalis paroolihalduris.
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
project-currency-hint = Kõik summad kuvatakse selles valuutas. Seda ei saa hiljem muuta.
project-currency-locked = Valuuta määratakse projekti loomisel.

update-required-title = Vajalik on uuendus
update-required-body = See Countedi versioon on serveriga suhtlemiseks liiga vana. Rakenduse kasutamise jätkamiseks uuenda seda.
update-required-body-testflight = See Countedi versioon on serveriga suhtlemiseks liiga vana. Rakenduse kasutamise jätkamiseks ava TestFlight ja paigalda uusim versioon.
update-required-button = Uuenda

notifications-label = Teavitused
notifications-title = Teavitused
notifications-empty = Midagi uut pole
notifications-friend-request = Sõbrakutse

friends-title = Sõbrad
friends-anonymous-body = Sõbrad on seotud sinu kontoga. Logi sisse, et lisada inimesi ja kutsuda neid oma projektidesse ilma linki jagamata.
friends-add-title = Lisa sõber
friends-add-hint = Ta näeb sinu kutset sisselogimisel. Kumbki teist ei saa teada, kas teisel on konto, enne kui kutse on vastu võetud.
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
friends-fingerprint = Turvakood
friends-fingerprint-hint = Kaks sõpra, kes loevad teineteisele sama turvakoodi, teavad, et keegi ei ole nende vahel - isegi mitte meie server.
friends-outgoing-title = Saadetud
friends-outgoing-hint = Ootab vastust. Näed neid oma sõprade seas, kui nad on kutse vastu võtnud.
friends-withdraw = Tühista
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
