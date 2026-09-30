# Shqip. I plotë, me përjashtim të teksteve ligjore (legal-, terms-, privacy-), që ekzistojnë vetëm
# në anglisht dhe frëngjisht dhe për çdo mesazh veç e veç kthehen te en.ftl.

### Common

loading = Duke u ngarkuar…
cancel = Anulo
confirm = Konfirmo
retry = Provo sërish
delete = Fshi
back = Prapa
language = Gjuha

### Navigation

nav-main = Navigimi kryesor
nav-projects = Projektet
nav-charts = Statistikat
nav-settings = Cilësimet

### Connectivity

offline-banner = Jashtë linje
offline-pending =
    { $count ->
        [one] { $count } në pritje
       *[other] { $count } në pritje
    }

sync-conflict-edit = Konflikt: ndryshimi i „{ $name }” dështoi (elementi u fshi). U anashkalua.
sync-conflict-delete = Konflikt: fshirja e „{ $name }” dështoi (elementi u fshi). U anashkalua.
sync-conflict-other = Konflikt: veprimi mbi „{ $name }” dështoi (elementi u fshi). U anashkalua.
sync-error = Gabim sinkronizimi: { $reason }

### Errors

error-network = Serveri nuk arrihet. Kontrollo lidhjen e internetit.
error-generic = Diçka shkoi keq. Provo sërish.

error-invalid-email = Kjo adresë emaili nuk është e vlefshme.
error-invalid-password = Ky fjalëkalim nuk është i vlefshëm.
error-password-too-short = Fjalëkalimi duhet të ketë të paktën 8 karaktere.
error-client-outdated = Ky version i aplikacionit është i vjetruar. Përditësoje për t'u identifikuar.
error-invalid-link = Kjo lidhje nuk është e vlefshme.
error-batch-too-large = Shumë elemente njëherësh.
error-payers-required = Zgjidh të paktën një pagues.
error-debtors-required = Zgjidh të paktën një person që ka borxh.
error-duplicate-participant = Një pjesëmarrës shfaqet dy herë në të njëjtën anë.
error-participant-not-in-project = Ky pjesëmarrës nuk është pjesë e këtij projekti.
error-too-many-participants = Shumë pjesëmarrës për një shpenzim.
error-invalid-credentials = Email ose fjalëkalim i pasaktë.
error-unauthenticated = Identifikohu për ta bërë këtë.
error-email-not-verified = Adresa jote e emailit nuk është verifikuar ende.
error-project-not-found = Ky projekt nuk ekziston më.
error-expense-not-found = Ky shpenzim nuk ekziston më.
error-storage-full = Hapësira e ruajtjes është plot: çelësi i këtij projekti nuk u ruajt dot në këtë pajisje. Ruaj lidhjen e ndarjes.
error-user-not-found = Ky pjesëmarrës nuk ekziston më.
error-tricount-not-found = Tricount nuk u gjet, ose API-ja e tij ktheu një gabim.
error-too-many-members = Ky projekt ka arritur kufirin e anëtarëve.
error-identity-taken = Një llogari tjetër e ka marrë tashmë këtë pjesëmarrës.
error-claim-proof-invalid = Kjo pajisje nuk e ka çelësin e projektit, kështu që nuk mund të marrë një pjesëmarrës. Hape sërish lidhjen e ndarjes.
error-user-has-payments = Ky pjesëmarrës ka shpenzime në projekt dhe nuk mund të hiqet.
error-resend-cooldown = Prit 60 sekonda para se të kërkosh një email tjetër.
error-self-friend-request = Nuk mund ta shtosh veten si mik.
error-not-a-friend = Mund të ftosh vetëm njerëz nga lista jote e miqve.
error-friend-has-no-key = Ky mik nuk e ka hapur ende versionin e fundit të aplikacionit. Kërkoji të identifikohet një herë, pastaj provo sërish.
error-friend-request-not-found = Kjo kërkesë miqësie nuk ekziston më.
error-invitation-not-found = Kjo ftesë nuk ekziston më.
error-too-many-friend-requests = Shumë kërkesa miqësie për momentin. Provo nesër.
error-too-many-invitations = Shumë ftesa në pritje.
error-invalid-kdf-salt = Cilësimet e enkriptimit nuk janë të vlefshme. Përditëso aplikacionin dhe provo sërish.
error-mixed-project-batch = Këta pjesëmarrës nuk janë të gjithë në të njëjtin projekt.
error-invalid-payload = Ky version i aplikacionit dërgoi të dhëna që serveri nuk i pranon. Përditësoje dhe provo sërish.
error-invalid-public-key = Çelësi yt i enkriptimit nuk është i vlefshëm. Përditëso aplikacionin dhe provo sërish.
error-payment-methods-stale = Të dhënat e tua të pagesës u ndryshuan në një pajisje tjetër. Ringarko dhe provo sërish.

### Auth

field-email = Email
field-email-placeholder = ti@shembull.al
field-password = Fjalëkalimi
field-name = Emri
field-name-placeholder = Ana Hoxha

login-title = Identifikohu
login-submit = Identifikohu
login-submitting = Duke u identifikuar…
login-password-placeholder = Fjalëkalimi yt
login-no-account = Nuk ke ende llogari?
login-unverified = Adresa jote e emailit nuk është verifikuar ende. Kontrollo kutinë postare ose dërgo lidhjen sërish.
login-resend = Dërgo sërish lidhjen e verifikimit
login-resending = Duke u dërguar…
login-resend-sent = Emaili u dërgua - kontrollo kutinë postare.

register-submit = Krijo një llogari
register-submitting = Duke u krijuar…
register-have-account = Ke tashmë një llogari?
register-password-placeholder = Të paktën 8 karaktere
register-password-warning = Shënoje fjalëkalimin. Nëse e harron, llogaria nuk mund të rikuperohet.
register-check-email-title = Kontrollo emailin
register-email-sent = Emaili u dërgua
register-email-sent-hint = Kliko lidhjen në kutinë postare për të aktivizuar llogarinë.
register-not-received-prefix = Nuk e more? Kontrollo dosjen e spamit, ose
register-sign-in-link = identifikohu
register-not-received-suffix = për ta dërguar lidhjen sërish.
register-terms-prefix = Duke krijuar një llogari pranon
register-terms-link = kushtet tona të përdorimit
register-terms-and = dhe
register-privacy-link = politikën tonë të privatësisë

settings-title = Cilësimet
settings-preferences = Preferencat
settings-preferences-local = Ruhen në këtë pajisje.
settings-preferences-synced = Sinkronizohen me llogarinë tënde, të enkriptuara.
settings-about = Rreth
settings-anonymous-title = Nuk je identifikuar
settings-upsell-title = Projektet e tua, në çdo pajisje
settings-upsell-free = Falas
settings-upsell-body = Counted punon pa llogari. Me një llogari falas, projektet dhe preferencat e tua të ndjekin në telefon, laptop dhe web - ende të enkriptuara, ende të palexueshme për ne.
settings-locked-badge = Llogari
settings-locked-friends = Krijo një llogari për të shtuar miq dhe për t'i ftuar në një projekt nga aplikacioni - pa asnjë lidhje për të shpërndarë.
settings-locked-payment-methods = Ruaje IBAN-in ose aplikacionin e pagesës një herë dhe ndaje me projektet që zgjedh. Kush të ka borxh e sheh pranë emrit tënd.
settings-friends-hint = Shto miq dhe ftoji në projektet e tua pa ndarë ndonjë lidhje.

account-member-since = Anëtar që nga
account-logout = Dil
account-logging-out = Duke dalë…
account-delete-title = Fshi llogarinë time
account-delete-warning = Menjëherë dhe përgjithmonë, pa kosh riciklimi. Shpenzimet që ke futur në një projekt të ndarë mbeten të dukshme për anëtarët e tjerë - janë pjesë e llogarive të tyre.
account-delete-confirm-title = Fshi llogarinë
account-delete-confirm-message = Llogaria jote, sesionet dhe lista e projekteve do të fshihen përgjithmonë. Pa fjalëkalimin tënd, të dhënat e enkriptuara të një projekti të ndarë bëhen të palexueshme për ty - kjo nuk mund të zhbëhet.

settings-payment-methods = Të dhënat e pagesës
settings-payment-methods-hint = Si dëshiron të të kthehen paratë. Të enkriptuara me llogarinë tënde.
payment-method-kind = Mënyra
payment-method-kind-other = Tjetër
payment-method-label = Emri
payment-method-label-placeholder = Llogaria kryesore
payment-method-value = Të dhënat
payment-method-value-placeholder = IBAN, numër telefoni, emër përdoruesi…
payment-method-add = Shto
payment-method-remove = Hiq { $name }
payment-method-empty = Nuk ke shtuar ende të dhëna pagese.
payment-method-deleted = Mënyra e pagesës u fshi.
payment-method-value-required = Plotëso të dhënat e çdo mënyre pagese, ose hiqe.
payment-method-label-required = Jepi një emër mënyrës tënde të personalizuar.
payment-method-too-long = Është shumë e gjatë - shkurtoje.
payment-method-invalid-characters = Hiq ndërprerjet e rreshtave ose karakteret e padukshme.
payment-method-limit = Mund të ruash deri në { $max } mënyra pagese.
payment-methods-saved = Të dhënat e pagesës u ruajtën.
payment-methods-offline = Duhet të jesh në linjë për të ruajtur të dhënat e pagesës.
payment-methods-stale = Të dhënat e tua të pagesës u ndryshuan në një pajisje tjetër. U ringarkuan — provo sërish.
payment-methods-key-missing = Identifikohu sërish për të menaxhuar të dhënat e pagesës.
settings-payment-methods-share-warning = Një mënyrë e ndarë është e dukshme për çdo anëtar të projekteve ku ke zgjedhur emrin tënd - për këdo që ka një nga ato lidhje.
payment-method-share = Ndaj me projektet e mia
payment-method-share-hint = Shfaqet pranë emrit tënd kur dikush të ka borxh.
payment-method-copy = Kopjo { $name }
payment-method-copied = U kopjua.
payment-method-copy-failed = Nuk u kopjua dot - zgjidh tekstin dhe kopjoje me dorë.

verify-email-checking = Duke verifikuar adresën e emailit…
verify-email-welcome = Emaili u verifikua - mirë se erdhe në Counted!
verify-email-back-to-login = Kthehu te identifikimi

### Project status

project-close = Mbyll
project-archive = Arkivo
project-reopen = Rihap
project-unarchive = Nxirr nga arkivi

### Dates

date-long = { $day } { $month } { $year }

month-1 = janar
month-2 = shkurt
month-3 = mars
month-4 = prill
month-5 = maj
month-6 = qershor
month-7 = korrik
month-8 = gusht
month-9 = shtator
month-10 = tetor
month-11 = nëntor
month-12 = dhjetor

month-short-1 = jan
month-short-2 = shk
month-short-3 = mar
month-short-4 = pri
month-short-5 = maj
month-short-6 = qer
month-short-7 = kor
month-short-8 = gus
month-short-9 = sht
month-short-10 = tet
month-short-11 = nën
month-short-12 = dhj

### Actions

add = Shto
create = Krijo
creating = Duke u krijuar…
edit = Ndrysho
leave = Largohu
close = Mbyll
paste = Ngjit
join = Bashkohu
import = Importo
importing = Duke importuar…
field-description = Përshkrimi
field-date = Data
date-today = Sot
date-yesterday = Dje
field-optional = Opsionale

### Projects

projects-filter-active = Aktive
projects-filter-all = Të gjitha
projects-count-label = Projektet
projects-empty = Asnjë projekt
projects-empty-hint = Krijo një projekt me butonin më poshtë
projects-offline-banner = Të dhëna jashtë linje - lidhu sërish për të rifreskuar.
projects-no-local-data = Asnjë e dhënë lokale
projects-no-local-data-hint = Identifikohu për të ngarkuar projektet për herë të parë.
projects-add = Shto një projekt
projects-create = Krijo një projekt
projects-join = Bashkohu në një projekt
projects-import-tricount = Importo nga Tricount
project-actions = Veprimet e projektit

status-ongoing = Në vazhdim
status-closed = I mbyllur
status-archived = I arkivuar

nav-help = Ndihmë
nav-privacy = Politika e privatësisë
nav-terms = Kushtet e përdorimit
nav-legal = Njoftim ligjor

leave-project-title = Të largohesh nga projekti?
leave-project-message = Do të humbasësh aksesin nga kjo pajisje. Nëse nuk mbetet asnjë anëtar, projekti dhe të gjitha shpenzimet e tij fshihen përgjithmonë.

add-project-title = Projekt i ri
add-project-name-label = Emri i projektit
add-project-name-placeholder = Udhëtimi im, Shokët e shtëpisë 2024…
add-project-participants = Pjesëmarrësit
add-project-participant-name = Emri i pjesëmarrësit
add-project-participant-placeholder = Clark Kent
add-project-offline = Nuk mund të krijosh një projekt jashtë linje. Lidhu sërish dhe provo përsëri.
add-project-name-required = Projekti ka nevojë për një emër.

join-link-label = Lidhja e ndarjes
join-link-hint = Lidhja mbart çelësin e dekriptimit - kopjoje të gjithën.
join-invalid-link = Kjo lidhje nuk është e vlefshme. Ngjit të gjithë lidhjen e ndarjes, përfshirë pjesën pas #.
join-wrong-project = Kjo lidhje është për një projekt tjetër.

import-tricount-link-label = Lidhja ose çelësi Tricount
import-tricount-key-required = Fut një lidhje ose çelës Tricount.
import-tricount-encryption-failed = Enkriptimi dështoi.
import-tricount-unimportable = Asgjë nuk u importua: ky Tricount ka anëtarë me llogari Tricount ose shuma që nuk përputhen (hyrje të prekura: { $count }).

### Expenses

save = Ruaj
saving = Duke u ruajtur…
adding = Duke u shtuar…
link-copied = Lidhja u kopjua
missing-encryption-key = Mungon çelësi i enkriptimit.
missing-encryption-key-title = Mungon çelësi i enkriptimit
missing-encryption-key-hint = Lidhja që përdore nuk mbart çelësin e nevojshëm për të dekriptuar këtë projekt. Përdor lidhjen e plotë të ndarë nga ai që e krijoi.
project-locked-hint = Kjo pajisje nuk e ka çelësin e këtij projekti. Hap lidhjen e ndarjes për ta zhbllokuar.
project-unlock = Zhblloko
project-no-local-data-hint = Identifikohu për të ngarkuar të dhënat e këtij projekti për herë të parë.
project-gone-title = Ky projekt nuk ekziston më
project-gone-hint = U fshi kur anëtari i fundit u largua. Lidhja e ndarjes nuk punon më, edhe nëse e rihap.

expense-add = Shto një shpenzim
transfer-add = Shto një transfertë
expense-edit-title = Ndrysho shpenzimin
expense-category = Kategoria
expense-category-auto = Auto · { $emoji }
expense-currency = Monedha e shumës
amount-op-add = Plus
amount-op-subtract = Minus
amount-op-multiply = Shumëzo
amount-op-divide = Pjesëto
amount-op-equals = Baraz
amount-op-done = U krye
expense-rate = Kursi i këmbimit (opsional)
expense-rate-hint = Lëre bosh për të përdorur kursin e Komisionit Evropian (InforEuro) për { $month }: 1 { $from } = { $rate } { $to }.
expense-rate-invalid = Fut një kurs këmbimi më të madh se 0.
expense-rate-unavailable = Nuk ka kurs automatik - fute me dorë.
expense-delete-title = Fshi shpenzimin
expense-delete-message = „{ $name }” do të fshihet përgjithmonë. Kjo nuk mund të zhbëhet.
expense-inconsistent-amounts = Shumat nuk përputhen
expenses-empty = Asnjë shpenzim
expenses-empty-hint = Fillo duke shtuar shpenzime me butonin më poshtë
expenses-show-more = Shfaq më shumë ({ $count } të tjera)

expense-type-expense = Shpenzim
expense-type-transfer = Transfertë
expense-type-gain = Fitim
expense-paid-by = paguar nga
expense-sent-by = dërguar nga
expense-contributed-by = kontribuar nga

expense-name-required = Emri është i detyrueshëm.
expense-amount-not-positive = Shuma duhet të jetë më e madhe se 0.
expense-no-payer = Zgjidh të paktën një pagues.
expense-no-debtor = Zgjidh të paktën një person që ka borxh.
expense-invalid-date = Kjo datë nuk është e vlefshme.
expense-payers-mismatch = Paguesit bëjnë gjithsej { $sum }, që nuk përputhet me shumën e shpenzimit ({ $total }).
expense-debtors-mismatch = Borxhlinjtë bëjnë gjithsej { $sum }, që nuk përputhet me shumën e shpenzimit ({ $total }).

participants-none = Askush
participants-everyone = Të gjithë ({ $count })
participants-some = { $count } nga { $total }
participants-select-all = Zgjidh të gjithë
participants-by-shares = Sipas pjesëve
split-amounts = Shumat
participants-remaining = Mbeten { $amount }
participants-over-by = { $amount } më shumë
participants-who-paid = Kush pagoi?
participants-who-received = Kush mori?
participants-who-transfers = Kush transferon?
participants-who-receives = Kush merr?
participants-for-whom = Për kë?

stats-total-expenses = Shpenzimet gjithsej
stats-my-expenses = Shpenzimet e mia

tab-expenses = Shpenzimet
tab-balance = Bilanci
tab-reimbursements = Shlyerja
reimbursements-empty-title = Gjithçka e shlyer!
reimbursements-empty-hint = Sugjerimet për shlyerje shfaqen këtu kur llogaritë nuk balancohen
reimbursement-owes = { $debtor } i ka borxh { $creditor }
reimbursement-record = Shlyej
reimbursement-pay-with = Paguaj
reimbursement-pay-shared-by = Ndarë nga { $name } - kontrollo emrin e marrësit që shfaq aplikacioni yt para se të dërgosh.
reimbursement-pay-title = Paguaj { $name }
reimbursements-mine-title = Ti ke borxh
reimbursements-others-title = Rimbursime të tjera
copy = Kopjo

user-selection-title = Cili pjesëmarrës je?
user-selection-hint = Zgjidh emrin tënd nga lista.
user-selection-required = Të lutem zgjidh një pjesëmarrës.
identity-claimed = Lidhur me një llogari
identity-claimed-by = Llogaria e { $name }
identity-taken-repick = Një llogari tjetër ka marrë pjesëmarrësin që po përdorje. Të lutem zgjidh një tjetër.
participant-gone-repick = Pjesëmarrësi që po përdorje u hoq nga ky projekt. Të lutem zgjidh një tjetër.

edit-project-title = Ndrysho projektin
edit-project-new-badge = i ri
edit-project-deferred-new-members = shtimi i anëtarëve të rinj
edit-project-deferred-removals = heqja e anëtarëve
edit-project-offline-deferred = Jashtë linje: { $items } do të zbatohet kur të lidhesh sërish.

export-failed = Eksportimi dështoi: { $reason }

history-expense-added = Shpenzim i shtuar: { $name }
history-expense-edited = Shpenzim i ndryshuar: { $name }
history-expense-deleted = Shpenzim i fshirë: { $name }
history-project-edited = Projekt i ndryshuar: { $name }
history-name-changed = Emri: „{ $from }” → „{ $to }”
history-description-added = Përshkrim i shtuar: „{ $value }”
history-description-removed = Përshkrim i hequr: „{ $value }”
history-description-changed = Përshkrimi: „{ $from }” → „{ $to }”

### Sweep

field-amount = Shuma
expense-name-placeholder = Restorant, ushqime…
expense-actions = Veprimet e shpenzimit
expense-your-share = Pjesa jote
expense-your-share-value = Pjesa jote: { $amount } { $currency }
expense-inconsistent-detail = Shumat nuk përputhen: { $paid } paguar, { $owed } borxh, për një shpenzim prej { $total }. Ndrysho shpenzimin për ta rregulluar.
missing-access-key = Mungon çelësi i aksesit. Hape këtë projekt përmes lidhjes së ndarjes.
filter-all = Të gjitha
filter-my-payments = Pagesat e mia
filter-my-debts = Borxhet e mia
participants-shares-for = Pjesët për { $name }
participants-amount-for = Shuma për { $name }
reimbursement-add = Shto një shlyerje
project-forget = Hiq nga lista ime
project-history-title = Historiku
history-kind-add = U shtua
history-kind-delete = U fshi
history-kind-edit = U ndryshua
export = Eksporto
export-json = Eksporto JSON
export-csv = Eksporto CSV
share-link = Ndaj
copy-link-failed = Lidhja nuk u kopjua dot
open-in-app = Hap në aplikacion
not-found-title = Faqja nuk u gjet
not-found-back = Kthehu te projektet

### Charts

charts-period = Periudha
period-all = Të gjitha
period-month = Muaj
period-3months = 3 muaj
period-year = Vit
period-custom = E personalizuar
charts-tab-categories = Kategoritë
charts-tab-trends = Tendencat
charts-total-spent = Gjithsej shpenzuar
charts-avg-per-person = Mes. për person
charts-expense-count =
    { $count ->
        [one] { $count } shpenzim
       *[other] { $count } shpenzime
    }
charts-nothing-to-show = Asgjë për të shfaqur
charts-my-share-note = Këto shifra janë pjesa jote në çdo shpenzim.
charts-my-share-skipped =
    { $count ->
        [one] 1 projekt nuk llogaritet — asnjë pjesëmarrës i zgjedhur, ose të dhënat e tij nuk u ngarkuan.
       *[other] { $count } projekte nuk llogariten — asnjë pjesëmarrës i zgjedhur, ose të dhënat e tyre nuk u ngarkuan.
    }

### Categories

category-food = Ushqim
category-transport = Transport
category-accommodation = Akomodim
category-leisure = Argëtim
category-shopping = Blerje
category-services = Shërbime
category-parties-gifts = Festa dhe dhurata
category-other = Tjetër
charts-project = Projekti
charts-all-projects = Të gjitha projektet
charts-date-from = Nga
charts-date-to = Deri
charts-total = Gjithsej
charts-tab-people = Njerëzit
charts-tab-projects = Projektet
charts-scope = Shpenzimet e kujt
charts-scope-group = Grupi
charts-scope-me = Unë
charts-currency = Monedha
charts-my-share = Pjesa ime
charts-share-of-total = { $pct }% e { $total }
charts-i-paid = Pagova
charts-paid-more = { $amount } më shumë se pjesa jote
charts-paid-less = { $amount } më pak se pjesa jote
charts-paid-even = Saktësisht pjesa jote
charts-part-title = Pjesa jote në çdo kategori
charts-part-desc = Gri është ç’shpenzoi grupi, ngjyrë është ç’konsumove ti.
charts-breakdown-title = Ndarja sipas kategorisë
charts-breakdown-desc = Prek një fetë ose rresht për të parë shpenzimet.
charts-of-total = { $amount } nga { $total }
charts-show-all = Shfaqi të gjitha ({ $count })
charts-show-less = Shfaq më pak
charts-spend-title = Shpenzimet në kohë
charts-spend-desc = Periudhat e shkurtra sipas ditës, të gjatat sipas javës ose muajit.
charts-group-by = Grupo sipas
bucket-day = Dita
bucket-week = Java
bucket-month = Muaji
charts-avg = mes.
charts-cat-title-day = { $category }, ditë pas dite
charts-cat-title-week = { $category }, javë pas jave
charts-cat-title-month = { $category }, muaj pas muaji
charts-cat-desc = Zgjidh një kategori për ta ndjekur në kohë.
charts-running-title = Totali kumulativ
charts-running-desc = Që nga { $date }.
charts-avg-per-day = { $amount } / ditë mesatarisht
charts-avg-per-week = { $amount } / javë mesatarisht
charts-avg-per-month = { $amount } / muaj mesatarisht
charts-people-title = Kush e mbajti grupin
charts-people-desc = Sa pagoi secili, pranë asaj që konsumoi.
charts-paid = Paguar
charts-fair-share = Pjesë e drejtë
charts-you = (ti)
charts-net-more = pagoi më shumë
charts-net-less = pagoi më pak
charts-balance-title = Bilanci yt në kohë
charts-balance-desc = Mbi vijë grupi të detyrohet ty. Poshtë saj ti i detyrohesh grupit.
charts-owed = Të detyrohen
charts-owe = Detyrohesh
charts-projects-title = Pjesa jote sipas projektit
charts-projects-desc = Totalet mbahen sipas monedhës dhe nuk mblidhen kurrë bashkë.
history-empty = Asnjë ngjarje
history-by = Nga { $name }
not-found-hint = Kjo faqe nuk ekziston, ose është zhvendosur.
payers-title-paid-by = Paguar nga
payers-title-sender = Dërguesi
payers-title-contributors = Kontribuesit
debtors-title-debtors = Kanë borxh
debtors-title-recipients = Marrësit
debtors-title-beneficiaries = Përfituesit

### Welcome

welcome-title = Llogaritë e tua nuk janë punë e askujt tjetër.
welcome-subtitle = Ndaj shpenzimet me miqtë.
welcome-note = Falas. Pa llogari. Pa reklama.
welcome-link-title = Një lidhje, dhe të gjithë marrin pjesë.
welcome-link-body = Askush nuk ka nevojë të krijojë llogari.
welcome-link-account = Llogari? Kurrë e detyrueshme. Shërben për të gjetur projektet e tua në një pajisje tjetër, për të ftuar miqtë nga aplikacioni dhe për të ndarë të dhënat e tua të pagesës.
welcome-demo-project = Fundjavë në Lion
welcome-private-title = Askush nuk mund t'i lexojë llogaritë e tua. As ne.
welcome-private-body = Emra, shuma, projekte: gjithçka enkriptohet në pajisjen tënde. Vetëm ti e ke çelësin.
welcome-private-names = Emra
welcome-private-amounts = Shuma
welcome-private-projects = Projekte
welcome-scan-title = Bëji një foto faturës.
welcome-scan-body = Shuma, data dhe kategoria plotësohen vetë. Gjithçka ndodh në telefonin tënd. Fotoja nuk ruhet.
welcome-eu-title = 100% evropiane
welcome-no-ads = Pa reklama
welcome-no-trackers = Pa gjurmues
welcome-step = Hapi { $current } nga { $total }
welcome-next = Tjetër
welcome-skip = Kapërce
welcome-start = Fillo
welcome-how-it-works = Si funksionon saktësisht?

### Help

help-intro = Një pyetje e zakonshme? Prek për të zgjeruar përgjigjen.
help-create-project-q = Si të krijoj një projekt?
help-create-project-a = Nga ekrani kryesor, prek butonin + poshtë. Jepi projektit një emër, zgjidh monedhën dhe je gati.
help-add-participants-q = Si të shtoj pjesëmarrës?
help-add-participants-a = Hap projektin, pastaj shto pjesëmarrës nga lista e anëtarëve. Çdo pjesëmarrës mund të paguajë ose të ketë borxh për një shpenzim.
help-share-project-q = Si të ndaj një projekt?
help-share-project-a = Ndaj URL-në e projektit (atë në shiritin e adresës). Kushdo me lidhjen mund ta shohë dhe ta ndryshojë projektin.
help-add-expense-q = Si të shtoj një shpenzim?
help-add-expense-a = Brenda një projekti, prek +, fut shumën, thuaj kush pagoi dhe mes kujt të ndahet. Mund të zgjedhësh edhe një datë tjetër nga sot.
help-types-q = Cili është ndryshimi mes shpenzimit, transfertës dhe fitimit?
help-types-expense = - një blerje e bërë nga një person dhe e ndarë mes disave.
help-types-transfer = - një rimbursim nga një person te tjetri, pa ndarje.
help-types-gain = - para të marra (një rimbursim, një dhuratë) për t'u ndarë mes disa personave.
help-past-date-q = A mund t'i vë një shpenzimi një datë të kaluar?
help-past-date-a = Po, fusha e datës është e lirë. Koha e krijimit të regjistrimit ruhet veçmas.
help-who-owes-q = Si e llogarit Counted kush ka borxh dhe sa?
help-who-owes-a = Counted llogarit bilancin neto të çdo pjesëmarrësi (sa ka paguar minus sa ka borxh), pastaj propozon serinë më të shkurtër të transfertave që shlyen të gjithë.
help-minimal-transfers-q = Pse numri i transfertave të sugjeruara është minimal?
help-minimal-transfers-a = Algoritmi fillimisht çifton bilancet që anulojnë njëri-tjetrin saktësisht, pastaj kalon nëpër të tjerat nga kreditori më i madh te borxhliu më i madh. Rezultati: më pak transferta për të shlyer gjithçka.
help-import-tricount-q = Si të importoj një projekt nga Tricount?
help-import-tricount-a = Nga ekrani kryesor, prek butonin „+” poshtë, pastaj
help-import-tricount-b = Ngjit lidhjen e ndarjes së Tricount-it që dëshiron të importosh.
help-encryption-q = A janë të dhënat e mia të enkriptuara?
help-encryption-a = Po. Counted kombinon dy garanci:
help-encryption-e2ee-term = Enkriptim nga skaji në skaj
help-encryption-e2ee-def = - gjithçka mes teje dhe serverit udhëton e enkriptuar.
help-encryption-zero-term = Zero akses
help-encryption-zero-def = - ti i enkripton të dhënat para se t'i dërgosh, dhe serveri ruan vetëm tekst të shifruar. Nuk kemi asnjë mënyrë për ta lexuar.
help-encryption-see = Për detajet, shih
help-forgot-password-q = Çfarë ndodh nëse harroj fjalëkalimin?
help-forgot-password-warning = Të dhënat e tua do të humbasin përgjithmonë.
help-forgot-password-a = Çelësi i enkriptimit rrjedh nga fjalëkalimi yt, kështu që nuk është i mundur asnjë rivendosje: askush - as ne - nuk mund të dekriptojë projektet e tua pa të. Ruaje mirë, idealisht në një menaxher fjalëkalimesh.
help-archive-delete-q = Si të arkivoj ose fshij një projekt?
help-archive-delete-a = Nga ekrani i projektit, hap menynë dhe zgjidh
help-archive-delete-b = për ta fshehur duke e mbajtur. Një projekt fshihet përgjithmonë kur anëtari i fundit largohet.
help-delete-account-q = Si të fshij llogarinë time?
help-delete-account-a = Hap Cilësimet dhe përdor „Fshi llogarinë time”. Është e menjëhershme dhe nuk mund të zhbëhet.
help-contact = Një pyetje tjetër? Na shkruaj te

# Receipt scanning (mobile only)
expense-scan = Skano një faturë
scan-in-progress = Duke lexuar faturën…
scan-error-capture = Fotoja nuk u bë dot. Provo sërish, ose fute shpenzimin me dorë.
scan-error-unreadable = Asgjë e lexueshme në atë faturë. Fute shpenzimin me dorë.
scan-check-amount = Kontrollo totalin - nuk ishte i printuar qartë.
scan-take-photo = Bëj një foto
scan-choose-photo = Zgjidh një foto
expense-converted-from = Paguar { $amount } { $from } · 1 { $from } = { $rate } { $to }
project-currency = Monedha
project-currency-hint = Çdo shumë shfaqet në këtë monedhë. Nuk mund të ndryshohet më vonë.
project-currency-locked = Monedha fiksohet kur krijohet projekti.

update-required-title = Kërkohet përditësim
update-required-body = Ky version i Counted është shumë i vjetër për të komunikuar me serverin. Përditësoje për të vazhduar përdorimin e aplikacionit.
update-required-button = Përditëso

notifications-label = Njoftimet
notifications-title = Njoftimet
notifications-empty = Asgjë e re
notifications-friend-request = Kërkesë miqësie

friends-title = Miqtë
friends-anonymous-body = Miqtë ruhen me llogarinë tënde. Identifikohu për të shtuar njerëz dhe për t'i ftuar në projektet e tua pa ndarë ndonjë lidhje.
friends-add-title = Shto një mik
friends-add-hint = Do ta shohë kërkesën tënde kur të identifikohet. Asnjëri nga ju nuk e mëson nëse tjetri ka llogari derisa kërkesa të pranohet.
friends-add-button = Shto
friends-add-from-project = Shto si mik
friends-request-sent = Kërkesa u dërgua
friends-no-account-key = Identifikohu sërish për të menaxhuar miqtë në këtë pajisje.
friends-incoming-title = Kërkesat
friends-accept = Prano
friends-decline = Refuzo
friends-list-title = Miqtë e mi
friends-list-empty = Asnjë mik ende. Shto dikë me email më lart, ose nga një projekt që ndani.
friends-remove = Hiq
friends-remove-confirm-title = Hiq mikun
friends-remove-confirm-message = { $email } nuk do të jetë më mes miqve tuaj, as ju mes të tijve. Secili nga ju mund të dërgojë një kërkesë të re më vonë.
friends-no-key = Ende jo gati
friends-fingerprint = Kodi i sigurisë
friends-fingerprint-hint = Dy miq që i lexojnë njëri-tjetrit të njëjtin kod sigurie e dinë se askush nuk qëndron mes tyre - as serveri ynë.
friends-outgoing-title = Të dërguara
friends-outgoing-hint = Në pritje të përgjigjes. Do t'i shohësh te miqtë sapo të pranojnë.
friends-withdraw = Anulo
invite-friends-title = Fto miq
invite-friends-hint = Çelësi i projektit enkriptohet për çdo mik në këtë pajisje. Serveri nuk e sheh kurrë.
invite-friends-empty = Asnjë mik për të ftuar ende.
invite-friends-button = Fto
invite-sent = { $count ->
    [one] Ftesa u dërgua
   *[other] { $count } ftesa u dërguan
}
invitation-badge = Ftesë
invitation-to = Bashkohu në „{ $name }”
invitation-to-unnamed = Bashkohu në një projekt
invitation-unreadable = Kjo ftesë nuk mund të hapet në këtë pajisje
invitation-from = Nga { $email }
invitation-accept = Bashkohu
invitation-decline = Refuzo

# Participants in the create and edit modals, and the "who are you?" picker - see
# docs/plans/friends.md §11.
participants-you-label = Emri yt në këtë projekt
participants-you-badge = Ti
participants-you-from-account = Marrë nga emri i llogarisë sate. Ndryshoje këtu vetëm për këtë projekt.
participants-you-required = I detyrueshëm. Kështu do të të shohin të tjerët.
participants-others = Pjesëmarrës të tjerë
participants-empty = Askush ende. Zgjidh një mik më poshtë ose shkruaj çfarëdo emri.
participants-empty-signed-out = Askush ende. Shkruaj një emër për të shtuar dikë.
participants-duplicate = „{ $name }” është tashmë në listë.
participants-input-label = Shto një mik ose shkruaj një emër
participants-input-placeholder = Mik ose çfarëdo emri
participants-suggest-friend = Mik · bashkohet si „{ $name }”, merr një ftesë
participants-suggest-not-ready = Mik · ende jo gati
participants-suggest-guest = Shto „{ $text }” pa llogari
participants-suggest-guest-sub = Pa llogari, vetëm një emër
participants-friends = Miqtë e tu
participants-all-friends = Të gjithë miqtë
participants-login-hint = Hyr për të shtuar njerëz drejtpërdrejt nga lista jote e miqve.
participants-invite-badge = Fto
participants-guest-badge = Pa llogari
participants-guest-sub = Pa llogari, vetëm një emër
participants-rename = Riemërto: { $name }
participants-remove = Hiq: { $name }
participants-rename-label = Emër i ri
participants-rename-save = Ruaj emrin
participants-rename-hint = Emri që shohin të gjithë në këtë projekt. Ftesa shkon përsëri te { $email }.
participants-invited-badge = I ftuar
participants-invited-sub = { $email } · ende e papranuar
participants-invited-pending = Ftesa ende e papranuar
participants-unlinked = I palidhur me llogari
add-project-create-invite = Krijo dhe fto: { $count }
edit-project-save-invite = Ruaj dhe fto: { $count }
edit-project-you-are = Në këtë pajisje ti je { $name }
edit-project-no-identity = Ende nuk ke zgjedhur kush je
edit-project-switch = Ndrysho
edit-project-choose = Zgjidh
invite-failed = Këto ftesa nuk u dërguan dot: { $emails }
invite-again = Fto sërish
friend-picker-title = Shto miq
user-selection-invited-hint = { $email } të ftoi në „{ $project }”.
user-selection-suggested = I sugjeruar
user-selection-suggested-sub = { $email } të shtoi me këtë emër
user-selection-confirm-as = Unë jam { $name }
user-selection-missing = Emri yt nuk është këtu? Kërkoji një pjesëmarrësi të të shtojë te cilësimet e projektit.
