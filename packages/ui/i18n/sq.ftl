# Shqip. I plotë, me përjashtim të teksteve ligjore (legal-, terms-, privacy-), që ekzistojnë vetëm
# në anglisht dhe frëngjisht dhe për çdo mesazh veç e veç kthehen te en.ftl.

### Common

loading = Duke u ngarkuar…
cancel = Anulo
discard-title = T'i hedhësh poshtë ndryshimet?
discard-message = Ato që fute do të humbasin.
discard = Hidhi poshtë
confirm = Konfirmo
retry = Provo sërish
delete = Fshi
back = Prapa
language = Gjuha
theme = Tema
theme-light = E çelët
theme-dark = E errët
theme-system = Sistemi
theme-hint-device = Ruhet vetëm në këtë pajisje.
theme-hint-system = Ndjek pamjen e kësaj pajisjeje.

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
error-rate-limited = Serveri është i zënë. Provo përsëri pas pak.
error-generic = Diçka shkoi keq. Provo sërish.

error-invalid-email = Kjo adresë emaili nuk është e vlefshme.
error-invalid-password = Ky fjalëkalim nuk është i vlefshëm.
error-password-too-short = Fjalëkalimi duhet të ketë të paktën 12 karaktere.
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
login-maybe-unverified = Sapo u regjistrove? Fillimisht verifiko adresën me lidhjen që të dërguam, ose dërgoje përsëri.
login-resend = Dërgo sërish lidhjen e verifikimit
login-resending = Duke u dërguar…
login-resend-sent = Emaili u dërgua - kontrollo kutinë postare.

register-submit = Krijo një llogari
register-submitting = Duke u krijuar…
register-have-account = Ke tashmë një llogari?
register-password-placeholder = Të paktën 12 karaktere
register-password-warning = Shënoje fjalëkalimin: ne nuk e shohim kurrë dhe nuk mund të ta kthejmë.
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
account-delete-warning = Fshirje e menjëhershme dhe përgjithmonë e llogarisë dhe të dhënave të tua.
account-delete-confirm-title = Fshi llogarinë
account-delete-confirm-message = Llogaria jote, sesionet dhe lista e projekteve do të fshihen përgjithmonë. Pa fjalëkalimin tënd, të dhënat e enkriptuara të një projekti të ndarë bëhen të palexueshme për ty - kjo nuk mund të zhbëhet.
erase-device-open = Dil dhe fshi këtë pajisje
erase-device-title = Të fshihet kjo pajisje?
erase-device-body = Projektet, çelësat e tyre dhe memoria e përkohshme në këtë pajisje do të fshihen, pastaj sesioni do të mbyllet. Projektet e tua mbeten në llogarinë tënde: hyr përsëri për t'i rikthyer.
erase-device-checking = Po kontrolloj nëse projektet e tua janë në llogarinë tënde…
erase-device-pending = Ndryshime që presin të dërgohen: { $count }. Lidhu përsëri me internetin dhe provo sërish.
erase-device-unreachable = Nuk u arrit të kontrollohej nëse projektet e tua janë në llogarinë tënde. Kontrollo lidhjen dhe provo sërish.
erase-device-at-risk = Këto projekte nuk janë në llogarinë tënde dhe do të humbasin. Për t'i mbajtur, anulo dhe kopjo fillimisht lidhjen e tyre:
erase-device-confirm = Fshi dhe dil
erase-device-confirm-anyway = Fshi gjithsesi
erase-device-erasing = Po fshihet…

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
payment-methods-stale = Të dhënat e tua të pagesës u ndryshuan në një pajisje tjetër. U ringarkuan - provo sërish.
payment-methods-key-missing = Identifikohu sërish për të menaxhuar të dhënat e pagesës.
settings-payment-methods-share-warning = Një mënyrë e ndarë është e dukshme për të gjithë anëtarët e projekteve të tua.
payment-method-share = Ndaj me projektet e mia
payment-method-share-hint = Shfaqet pranë emrit tënd kur dikush të ka borxh.
payment-method-copy = Kopjo { $name }
payment-method-copied = U kopjua.
payment-method-copy-failed = Nuk u kopjua dot - zgjidh tekstin dhe kopjoje me dorë.

verify-email-checking = Duke verifikuar adresën e emailit…
verify-email-welcome = Emaili u verifikua - mirë se erdhe në Counted!
verify-email-back-to-login = Kthehu te identifikimi
verify-email-intro = Fut fjalëkalimin tënd për të konfirmuar që kjo adresë është e jotja.
verify-email-submit = Verifiko adresën time

### Project status

project-archive = Arkivo
project-sheet-invite = Fto
project-sheet-recurring = Të përsëritura
project-sheet-edit = Ndrysho projektin
project-sheet-close = Mbyll projektin
project-sheet-archive = Arkivo projektin
project-sheet-reopen = Rihap projektin
project-sheet-unarchive = Nxirr projektin nga arkivi
project-sheet-leave = Largohu nga projekti

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
projects-offline-banner = Të dhëna jashtë linje - lidhu sërish për të rifreskuar.
demo-banner = Projekt demonstrues - vetëm për lexim.
demo-start-own = Krijo projektin tënd
demo-badge = Demo
demo-bar = Ndryshimet e tua zhduken kur mbyll këtë skedë.
demo-leave = Fillo vërtet
demo-preparing = Po përgatitet demoja jote…
demo-failed = Demoja nuk mund të përgatitej.
demo-sign-in-blocked = Hyrja është e çaktivizuar në demo. Dil prej saj për t’u identifikuar ose për të krijuar një llogari; projektet e tua mbeten të paprekura.
projects-no-local-data = Asnjë e dhënë lokale
projects-no-local-data-hint = Identifikohu për të ngarkuar projektet për herë të parë.
projects-add = Shto një projekt
projects-create = Krijo një projekt
projects-personal-expense = Shpenzim personal
personal-expense-banner = U shtua te Shpenzimet e mia
personal-create-title = Krijo projektin tënd personal
personal-create-hint = Ndiq çfarë shpenzon vetëm
group-create-title = Krijo një projekt të përbashkët
group-create-hint = Ndaj shpenzimet me miqtë dhe familjen
personal-create-action = Krijo
personal-create-signup = Krijo një llogari
personal-sheet-title = Projekti yt personal
personal-sheet-intro = Për të shënuar çfarë shpenzon vetëm. Shfaqet me grupet e tua te Statistikat.
personal-sheet-point-alone = Një anëtar: ti, me emrin e llogarisë sate. Pa lidhje ndarjeje.
personal-sheet-point-encrypted = I enkriptuar në pajisjet e tua, si projektet e tjera.
personal-sheet-point-stats = Pjesët e tua në grupe u shtohen këtyre shpenzimeve te Statistikat.
personal-default-name = Shpenzimet e mia
my-spending-prev-month = Muaji i mëparshëm
my-spending-next-month = Muaji tjetër
my-spending-spent = Shpenzove
my-spending-groups = Grupet
my-spending-income = Të ardhurat
my-spending-balance = Bilanci i muajit
my-spending-unconverted = Pjesët në një monedhë tjetër mbeten jashtë derisa të njihet një kurs.
my-spending-empty = Asgjë këtë muaj
my-spending-empty-hint = Shto një shpenzim me +. Pjesët e tua në grupe do të shfaqen këtu automatikisht.
my-spending-share-of = nga { $amount }
my-spending-uncounted = nuk llogaritet
payer-you = ti
expense-received-by = marrë nga
counted-switch = Llogarite te Shpenzimet e mia
counted-switch-on = Aktiv: pjesa jote i shtohet Shpenzimet e mia
counted-switch-off = Joaktiv: pjesa jote nuk shfaqet te Shpenzimet e mia
counted-groups = Grupet e llogaritura
counted-groups-intro = Pjesa jote në këto grupe i shtohet Shpenzimet e mia dhe buxheteve.
counted-groups-converted = { $from }, konvertuar në { $to }
counted-groups-encrypted = Kjo zgjedhje është e enkriptuar me llogarinë tënde dhe të ndjek në pajisjet e tua.
counted-groups-empty = Ende asnjë grup në këtë pajisje.
counted-groups-done = OK
counted-save-failed = Kjo zgjedhje nuk mund të ruhej. Provo përsëri.
tab-budget = Buxhet
budget-title = Buxhetet
budget-manage = Menaxho
budget-left = Mbeten { $amount }
budget-over = Tejkaluar me { $amount }
budget-empty = Asnjë buxhet
budget-empty-hint = Vendos një tavan për çdo kategori. Pjesët e tua në grupe llogariten gjithashtu.
budget-define = Vendos një buxhet
budget-sheet-title = Buxhetet mujore
budget-sheet-intro = Mbi pjesën tënde: personale dhe grupe. Lëre bosh për të mos ndjekur një kategori.
budget-average = Mesatarja 3 muajt e fundit: { $amount }
budget-for = Buxheti { $category }
budget-alert = Më njofto në 80 %
budget-alert-hint = Llogaritet në këtë pajisje
budget-invalid = Shumë e pavlefshme për { $category }.
budget-offline = Buxhetet ruhen vetëm në linjë. Rilidhu dhe provo përsëri.
budget-alert-near = { $category }: { $percent } % e buxhetit të muajit
budget-alert-over = { $category }: buxheti i muajit u tejkalua
my-spending-search-summary =
    { $count ->
        [one] { $count } rezultat
       *[other] { $count } rezultate
    } · { $total } · personale dhe grupe, të gjithë muajt
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
expenses-unreadable = Disa regjistrime nuk mund të lexohen në këtë pajisje dhe nuk përfshihen në totale. Përditëso aplikacionin për t’i parë.
project-unlock = Zhblloko
project-no-local-data-hint = Identifikohu për të ngarkuar të dhënat e këtij projekti për herë të parë.
project-gone-title = Ky projekt nuk ekziston më
project-gone-hint = U fshi kur anëtari i fundit u largua. Lidhja e ndarjes nuk punon më, edhe nëse e rihap.

expense-add = Shto një shpenzim
transfer-add = Shto një transfertë
expense-edit-title = Ndrysho shpenzimin
expense-category = Kategoria
expense-category-auto = Auto · { $emoji }
emoji-picker-label = Emoji
emoji-none = Pa emoji
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
expense-type-group = Lloji
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
balance-gets-back = Merr mbrapsht
balance-owes = Ka borxh
balance-settled = Të shlyer
reimbursements-empty-title = Gjithçka e shlyer!
reimbursements-empty-hint = Sugjerimet për shlyerje shfaqen këtu kur llogaritë nuk balancohen
reimbursement-owes = { $debtor } i ka borxh { $creditor }
reimbursement-record = Shlyej
settlement-recorded = Shlyerja u regjistrua.
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
move-expense = Zhvendos në një projekt
move-expense-title = Zhvendos te…
move-current = Aktual
move-converted = { $currency }, i konvertuar me kursin e muajit
move-note-personal = Në hapin tjetër do të zgjedhësh kush pagoi dhe për kë. Shpenzimi do të largohet nga Shpenzimet e mia.
move-note-group = Shpenzimi do të largohet nga { $project } për të gjithë anëtarët dhe bilanci i grupit do të rillogaritet. Në hapin tjetër do të zgjedhësh kush pagoi dhe për kë.
move-banner = Nga { $from } te { $to }
move-submit = Zhvendos
move-done = Shpenzimi u zhvendos te { $project }
history-expense-moved = Shpenzimi u zhvendos te një projekt tjetër: { $name }
personal-badge = Personal
move-members =
    { $count ->
        [one] { $count } anëtar
       *[other] { $count } anëtarë
    }
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
expense-share-percent = { $percent }%
expense-added-by = Shtuar nga { $name } më { $date }
expense-inconsistent-detail = Shumat nuk përputhen: { $paid } paguar, { $owed } borxh, për një shpenzim prej { $total }. Ndrysho shpenzimin për ta rregulluar.
missing-access-key = Mungon çelësi i aksesit. Hape këtë projekt përmes lidhjes së ndarjes.
filter-all = Të gjitha
filter-my-payments = Pagesat e mia
filter-my-debts = Borxhet e mia
expense-filter-group = Filtri
expenses-search = Kërko shpenzime
expenses-no-match = Asnjë shpenzim nuk përputhet.
expenses-search-open = Kërko
expenses-search-clear = Pastro kërkimin
expenses-search-summary =
    { $count ->
        [one] { $count } rezultat
       *[other] { $count } rezultate
    } · { $total } · të gjithë muajt
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
charts-my-share-skipped =
    { $count ->
        [one] 1 projekt nuk llogaritet - asnjë pjesëmarrës i zgjedhur, ose të dhënat e tij nuk u ngarkuan.
       *[other] { $count } projekte nuk llogariten - asnjë pjesëmarrës i zgjedhur, ose të dhënat e tyre nuk u ngarkuan.
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
categories-manage = Menaxho kategoritë
categories-title = Kategoritë
categories-yours = Kategoritë e tua
categories-builtin = Të paracaktuara
categories-add = Shto një kategori
categories-in-parent = në kategorinë { $parent }
categories-renamed-from = riemërtuar · fillimisht { $name }
categories-hidden-hint = e fshehur · shpenzimet e saj shkojnë te { $other }
categories-always-shown = shfaqet gjithmonë
categories-hidden = E fshehur
categories-new = Kategori e re
categories-edit = Ndrysho kategorinë
categories-parent = Llogaritet në
categories-parent-hint = Në statistikat e të gjitha projekteve të tua, { $name } llogaritet në kategorinë { $parent }.
categories-create = Krijo
categories-hide = Fshih këtë kategori
categories-hide-hint = Shpenzimet e saj shfaqen te { $other } për të gjithë anëtarët.
categories-delete-title = Të fshihet “{ $name }”?
categories-delete-body = Shpenzimet përkatëse ({ $count }) kthehen te { $parent }. Asnjë shpenzim nuk fshihet.
categories-delete-body-unknown = Shpenzimet e saj kthehen te { $parent }. Asnjë shpenzim nuk fshihet.
categories-delete-warning = Kategoria zhduket për të gjithë anëtarët e projektit.
categories-summary-created = Të krijuara: { $count }
categories-summary-renamed = Të riemërtuara: { $count }
categories-summary-hidden = Të fshehura: { $count }
categories-summary-default = Asnjë kategori e jotja
categories-offline = Kategoritë menaxhohen vetëm në linjë. Rilidhu dhe provo përsëri.
categories-error-too-many = Një projekt ka të shumtën 20 kategori të vetat.
categories-error-full = Ky projekt ka arritur kufirin e kategorive.
categories-error-name = Jepi një emër kategorisë.
categories-error-missing = Kjo kategori ndryshoi ndërkohë. Mbylle dhe provo përsëri.
history-category-created = Kategori e krijuar: { $name }
history-category-edited = Kategori e ndryshuar: { $name }
history-category-deleted = Kategori e fshirë: { $name }
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
charts-paid-more = { $amount } më shumë se pjesa ime
charts-paid-less = { $amount } më pak se pjesa ime
charts-paid-even = Saktësisht pjesa ime
charts-part-title = Pjesa jote në çdo kategori
charts-part-desc = Gri është ç’shpenzoi grupi, ngjyrë është ç’konsumove ti.
charts-breakdown-title = Ndarja sipas kategorisë
charts-breakdown-desc = Prek një fetë ose rresht për të parë shpenzimet.
charts-of-total = { $amount } nga { $total }
charts-show-all = Shfaqi të gjitha ({ $count })
charts-show-less = Shfaq më pak
charts-spend-title = Shpenzimet në kohë
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
charts-projects-desc = Totalet mbahen sipas monedhës.
history-empty = Asnjë ngjarje
history-by = Nga { $name }
not-found-hint = Kjo faqe nuk ekziston, ose është zhvendosur.
payers-title-paid-by = Paguar nga
payers-title-sender = Dërguesi
payers-title-contributors = Kontribuesit
debtors-title-debtors = Kanë borxh
debtors-title-recipients = Marrësit
debtors-title-beneficiaries = Përfituesit
debtors-title-for = Për
debtors-title-recipient = Marrësi
expense-split-between = ndarë mes { $count }
expense-people =
    { $count ->
        [one] { $count } person
       *[other] { $count } persona
    }

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
help-forgot-password-warning = Askush nuk mund ta rikuperojë për ty.
help-forgot-password-a = Fjalëkalimi yt mbron çelësat e tu të enkriptimit dhe nuk na dërgohet kurrë: askush - as ne - nuk mund t'i dekriptojë projektet e tua në vendin tënd. Ruaje mirë, idealisht në një menaxher fjalëkalimesh.
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
project-currency-locked = Monedha fiksohet kur krijohet projekti.
currency-search = Kërko një monedhë

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
friends-add-hint = Nëse ky person ka llogari, do ta shohë kërkesën tënde në aplikacion. Nëse jo, njoftoje: do ta gjejë kur të regjistrohet me këtë adresë.
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
friends-remove-confirm-message = { $email } nuk do të jetë më mes miqve të tu, as ti mes të tijve. Secili nga ju mund të dërgojë një kërkesë të re më vonë.
friends-no-key = Ende jo gati
friends-key-changed = Kodi ndryshoi
friends-key-changed-hint = Kodi i sigurisë i një miku ndryshoi. Kjo ndodh kur ai siguron llogarinë, ose nëse çelësi i tij u zëvendësua. Ftesat me të janë të bllokuara derisa t’ia lexoni njëri-tjetrit kodin e ri, ballë për ballë ose me telefon, dhe të prekësh “Kodi përputhet”.
friends-fingerprint = Kodi i sigurisë
friends-fingerprint-hint = Dy miq që i lexojnë njëri-tjetrit të njëjtin kod sigurie e dinë se askush nuk qëndron mes tyre - as serveri ynë.
friends-key-mismatch = Serveri dërgoi për llogarinë tënde një çelës publik që nuk përputhet me çelësin tënd privat. Kodi yt i sigurisë është fshehur dhe ftesat janë çaktivizuar në këtë pajisje: dikush mund të jetë duke manipuluar serverin.
keypair-mismatch = Çelësat që serveri dërgoi për llogarinë tënde nuk përputhen me ata të kësaj pajisjeje. Ftesat dhe ndryshimet e çelësit janë të çaktivizuara këtu derisa të përputhen: dikush mund të jetë duke ndërhyrë në server.
friends-outgoing-title = Të dërguara
friends-outgoing-hint = Në pritje të përgjigjes. Do t'i shohësh te miqtë sapo të pranojnë.
friends-withdraw = Anulo
friends-notify = Njofto me mesazh
friends-notify-label = Njofto { $email }
friends-notify-message = Po kaloj te Counted për shpenzimet tona të përbashkëta. Regjistrim falas në https://counted.fr me adresën { $email }: kërkesa ime për miqësi pret atje.
friends-notify-copied = Mesazhi u kopjua
friends-notify-copy-failed = Mesazhi nuk mund të kopjohej
guests-hint-title =
    { $count ->
        [one] { $count } pjesëmarrës nuk i është bashkuar ende projektit
       *[other] { $count } pjesëmarrës nuk i janë bashkuar ende projektit
    }
guests-hint-later = Më vonë
guests-hint-send = Ftoji
guests-send-title = Dërgo projektin te pjesëmarrësit
guests-send-friend = Fto një mik në Counted
guests-send-friend-badge = Pa lidhje
guests-send-friend-hint = Nëse ka tashmë llogari, merr një ftesë të enkriptuar; asgjë nuk kalon nga aplikacioni yt i mesazheve.
guests-send-link = Ndaj lidhjen e projektit
guests-send-link-hint = Për ata pa llogari. E hapin projektin me një prekje.
guests-send-link-warning = Kushdo që ka këtë lidhje mund ta lexojë dhe ndryshojë projektin. Edhe aplikacioni yt i mesazheve e sheh.
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
participants-others = Pjesëmarrës të tjerë
participants-removed = Të hequr
participant-restore = Rikthe
participants-empty = Shto pjesëmarrës më poshtë.
participants-duplicate = „{ $name }” është tashmë në listë.
participants-input-label = Shto një mik ose shkruaj një emër
participants-input-placeholder = Mik ose çfarëdo emri
participants-suggest-friend = Mik · bashkohet si „{ $name }”, merr një ftesë
participants-suggest-not-ready = Mik · ende jo gati
participants-suggest-guest = Shto „{ $text }”
participants-suggest-guest-sub = I ftuar
participants-friends = Miqtë e tu
participants-all-friends = Të gjithë miqtë
participants-login-hint = Hyr për të shtuar njerëz drejtpërdrejt nga lista jote e miqve.
participants-invite-badge = Fto
participants-guest-badge = I ftuar
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
edit-project-you-are = Në këtë projekt ti je { $name }
edit-project-no-identity = Ende nuk ke zgjedhur kush je
edit-project-switch = Ndrysho
edit-project-choose = Zgjidh
invite-failed = Këto ftesa nuk u dërguan dot: { $emails }
invite-waiting = Këta miq duhet ta hapin Counted një herë para se të mund të ftohen: { $emails }
key-offer-title = Një çelës tjetër për { $name }
key-offer-body = Kjo pajisje ka një çelës tjetër për këtë projekt, që nuk e ka hapur kurrë këtu.
key-offer-choices = Përdor këtë çelës: projekti hapet me çelësin nga kjo lidhje ose ftesë. Anulo: pajisja mban çelësin e saj aktual dhe asgjë nuk ndryshon.
key-offer-use = Përdor këtë çelës
invite-again = Fto sërish
friend-picker-title = Shto miq
user-selection-invited-hint = { $email } të ftoi në „{ $project }”.
user-selection-suggested = I sugjeruar
user-selection-suggested-sub = { $email } të shtoi me këtë emër
user-selection-confirm-as = Unë jam { $name }
user-selection-missing = Emri yt nuk është këtu? Kërkoji një pjesëmarrësi të të shtojë te cilësimet e projektit.

## Shpenzime të përsëritura

repeat-label = Përsërit
repeat-none = Nuk përsëritet
repeat-weekly = Çdo javë
repeat-biweekly = Çdo 2 javë
repeat-monthly = Çdo muaj
repeat-quarterly = Çdo 3 muaj
repeat-yearly = Çdo vit
repeat-every-weeks = Çdo { $count } javë
repeat-every-months = Çdo { $count } muaj
repeat-every-years = Çdo { $count } vjet
repeat-custom = E personalizuar…
repeat-every = Çdo
repeat-unit-weeks = javë
repeat-unit-months = muaj
repeat-unit-years = vjet
repeat-on-weekday = ditën: { $weekday }
repeat-on-day = më { $day } të muajit
repeat-on-day-month = më { $day } { $month }
repeat-month-end = Në muajt më të shkurtër bie në ditën e fundit.
repeat-ends = Mbaron
repeat-ends-never = Kurrë
repeat-ends-on = Në një datë
repeat-ends-after = Pas
repeat-fewer = Më pak
repeat-more = Më shumë
repeat-last-on = e fundit më { $date }
repeat-variable = Shuma ndryshon çdo herë
repeat-variable-hint = Secila shtohet me shumën e fundit dhe shënohet “për t’u konfirmuar”.
repeat-done = U krye
repeat-no-end = Pa fund
repeat-until = Deri më { $date }
repeat-occurrences = Numri i herëve: { $count }
repeat-offline = Kërkon lidhje. Vetë shpenzimi mund të shtohet gjithsesi.
repeat-foreign = Përsëritet si { $amount } { $currency }, i konvertuar një herë me kursin e sotëm. Do të paralajmërohesh nëse kursi ndryshon më shumë se 5 %.
repeat-backfill = Fillon në të kaluarën. Shpenzime të shtuara tani: { $count }.
add-and-repeat = Shto dhe përsërit
weekday-1 = e hënë
weekday-2 = e martë
weekday-3 = e mërkurë
weekday-4 = e enjte
weekday-5 = e premte
weekday-6 = e shtunë
weekday-7 = e diel
recurring-title = Shpenzime të përsëritura
recurring-strip = Shpenzime të përsëritura: { $count }
recurring-next = Tjetri: { $name }, { $date }
recurring-to-confirm = Për t’u konfirmuar: { $count }
recurring-per-month = Në muaj, afërsisht
recurring-your-share = Pjesa jote
recurring-active = Aktive
recurring-paused = Në pauzë
recurring-finished = Të përfunduara
recurring-paid-by = paguan { $name }
recurring-next-on = Tjetri më { $date }
recurring-progress = { $done } nga { $total }
recurring-rate-badge = Kursi ndryshoi { $percent } %
recurring-empty = Ende asgjë nuk përsëritet. Zgjidh “Përsërit” kur shton një shpenzim: qira, abonime, fatura.
recurring-next-ones = Të ardhshmet
recurring-added-so-far = Të shtuara deri tani
recurring-set-up-by = Krijuar nga
recurring-pause = Vendos në pauzë
recurring-resume = Vazhdo
recurring-stop = Ndalo përsëritjen
recurring-stop-title = Të ndalohet “{ $name }”?
recurring-stop-message = Nuk do të përsëritet më. Shpenzimet e shtuara tashmë mbeten.
recurring-resume-title = Të vazhdojë “{ $name }”?
recurring-resume-message = Tjetri më { $date }. Datat e humbura gjatë pauzës nuk shtohen.
recurring-edit-title = Ndrysho shpenzimin e përsëritur
recurring-edit-banner = Ndryshimet vlejnë nga { $date }. Shpenzimet e shtuara tashmë mbeten siç janë.
recurring-next-on-label = Tjetri më
recurring-next-too-early = Data tjetër duhet të jetë pas shpenzimit të fundit të shtuar.
recurring-use-stop = Për ta përfunduar, përdor “Ndalo përsëritjen” te shpenzimi i përsëritur.
recurring-drift = Kursi i { $currency } ka ndryshuar { $percent } % që nga krijimi. Secili shtohet ende si { $amount } { $project_currency } (1 { $currency } = { $rate }). Me kursin e sotëm do të ishte { $today_amount } { $project_currency }.
recurring-use-rate = Përdor kursin e sotëm
recurring-keep = Mbaj { $amount } { $currency }
recurring-added = Shpenzime të përsëritura të shtuara: { $count }
recurring-blocks-removal = { $name } nuk mund të hiqet ende: pjesë e { $rules }. Hiqe { $name } prej tyre ose ndaloji, pastaj ruaj përsëri.
history-recurring-added = Shtuar automatikisht: { $name } ({ $date })
history-recurring-created = Shpenzim periodik i krijuar: { $name }
history-recurring-edited = Shpenzim periodik i ndryshuar: { $name }
history-recurring-paused = Shpenzim periodik në pauzë: { $name }
history-recurring-resumed = Shpenzim periodik i rifilluar: { $name }
history-recurring-stopped = Shpenzim periodik i ndalur: { $name }
occurrence-recurring = Shpenzim i përsëritur
occurrence-auto = Shtuar automatikisht nga një shpenzim i përsëritur.
occurrence-auto-next = Shtuar automatikisht nga një shpenzim i përsëritur. Tjetri më { $date }.
occurrence-auto-stopped = Shtuar automatikisht nga një shpenzim i përsëritur që është ndalur që atëherë.
occurrence-manage = Menaxho
estimate-badge = Për t’u konfirmuar
expense-pending = Në pritje
estimate-title = Shuma për t’u konfirmuar.
estimate-body = Shtuar me shumën e mëparshme. Fut shumën e vërtetë kur ta mësosh.
estimate-confirm = Konfirmo shumën
apply-to = Zbato për
apply-this-only = Vetëm këtë shpenzim
apply-and-next = Këtë dhe të ardhshmet
apply-and-next-hint = Shpenzimi i përsëritur ndryshon nga { $date }
apply-rule-failed = Shpenzimi u ruajt, por shpenzimi i përsëritur nuk u ndryshua.
occurrence-delete-message = “{ $name }” i { $date } do të fshihet përgjithmonë. Përsëritja vazhdon dhe kjo datë nuk do të kthehet.
occurrence-delete-one = Fshi vetëm këtë
occurrence-delete-stop = Fshi dhe ndalo përsëritjen
error-recurring-clock = Ora e kësaj pajisjeje shkon përpara. Kontrollo datën dhe orën.
error-recurring-not-found = Ky shpenzim i përsëritur nuk ekziston më.
error-recurring-stale = Dikush e ndryshoi ndërkohë këtë shpenzim të përsëritur. U ringarkua: kontrolloje dhe ruaj përsëri.
error-too-many-recurring = Ky projekt ka tashmë 50 shpenzime të përsëritura. Ndalo një që nuk të duhet më për të shtuar një tjetër.
error-user-in-recurring = Ky pjesëmarrës është pjesë e një shpenzimi të përsëritur. Hiqe fillimisht prej tij ose ndalo shpenzimin.

home-owed-to-you = Të detyrohen
home-you-owe = Detyrohesh
home-details-show = Shiko detajet
home-details-hide = Fshih detajet
home-all-settled = Gjithçka është shlyer
home-pay = Paguaj
home-remind = Kujto
home-remind-message = Kujtesë e vogël: ka ende rimbursime për t’u shlyer te { $project }.
home-row-owed = të detyrohen
home-row-owe = detyrohesh
home-row-settled = E shlyer
home-row-quits = Ti je në rregull
home-row-pending =
    { $count ->
        [one] { $count } rimbursim në pritje
       *[other] { $count } rimbursime në pritje
    }
home-totals-partial = Pa projektet pa kurs këmbimi
error-account-key-changed = Çelësi i llogarisë sate ndryshoi në një pajisje tjetër. Hyr përsëri për të vazhduar.
settings-security = Siguria
settings-change-password = Ndrysho fjalëkalimin
settings-change-password-hint = Projektet dhe të dhënat e tua mbeten siç janë. Pajisjet e tua të tjera dalin nga llogaria.
change-password-title = Ndrysho fjalëkalimin tënd
change-password-current = Fjalëkalimi aktual
change-password-new = Fjalëkalimi i ri
change-password-confirm = Konfirmo fjalëkalimin e ri
change-password-mismatch = Dy fjalëkalimet e reja nuk përputhen.
change-password-submit = Ndrysho fjalëkalimin
change-password-submitting = Po ndryshohet…
change-password-done = Fjalëkalimi u ndryshua. Pajisjet e tua të tjera dolën nga llogaria.
error-rotation-incomplete = Llogaria jote ndryshoi ndërkohë. Provo sërish.
error-key-lifecycle-unavailable = Ende nuk është i disponueshëm: fillimisht të gjithëve u duhet versioni më i ri i aplikacionit.
settings-compromised = Dikush mund ta dijë fjalëkalimin tim
settings-compromised-hint = Fjalëkalim i ri, çelësa të rinj dhe dalje nga të gjitha pajisjet e tjera.
settings-compromised-open = Siguro llogarinë time
compromised-title = Siguro llogarinë tënde
compromised-body = Fjalëkalimi dhe çelësat e llogarisë sate zëvendësohen, dhe të gjitha pajisjet e tjera dalin nga llogaria. Miqtë e tu do të shohin që kodi yt i sigurisë ndryshon: krahasojeni sërish.
compromised-keeps = Kush e kishte fjalëkalimin tënd mban atë që ka hapur tashmë: projektet që mund të lexonte deri tani.
compromised-submit = Zëvendëso fjalëkalimin dhe çelësat
compromised-done = Llogaria jote është e siguruar. Kërkoju miqve të krahasojnë kodin tënd të ri të sigurisë.
friends-new-fingerprint = Kodi i ri:
friends-code-matches = Kodi përputhet
friends-code-trusted = Kodi u konfirmua.
settings-recovery-key = Çelës rikuperimi
settings-recovery-key-none = Pa të, një fjalëkalim i harruar të kushton preferencat, të dhënat e pagesës dhe kodet e miqve, dhe zgjat 3 ditë.
settings-recovery-key-set = U krijua. Nëse harron fjalëkalimin, ta kthen llogarinë me gjithçka.
settings-recovery-key-pending = Një çelës i ri e zëvendëson më { $when }. Nuk ishe ti? Anuloje me çelësin aktual.
recovery-key-create = Krijo
recovery-key-replace = Zëvendëso
recovery-key-cancel-replacement = Anulo çelësin e ri
recovery-key-title = Çelësi yt i rikuperimit
recovery-key-body = Shkruaje ose ruaje diku ku mund të arrish vetëm ti. Counted nuk mund ta shfaqë sërish ose ta rikuperojë për ty.
recovery-key-copy = Kopjo
recovery-key-copied = U kopjua
recovery-key-save = Ruaje si skedar
recovery-key-print = Printo
recovery-key-next = Vazhdo
recovery-key-confirm-body = Për të kontrolluar kopjen, shkruaj grupin { $n } të çelësit.
recovery-key-confirm-wrong = Ky grup nuk përputhet. Shiko sërish kopjen.
recovery-key-password-body = Shkruaj fjalëkalimin për ta ruajtur çelësin në llogari.
recovery-key-submit = Ruaj çelësin
recovery-key-done = Çelësi i rikuperimit u ruajt.
recovery-key-replacement-done = Çelësi i ri u ruajt. E zëvendëson aktualin pas 72 orësh; deri atëherë aktuali funksionon ende.
recovery-key-field = Çelës rikuperimi
recovery-key-file-intro = Çelësi i rikuperimit të Counted
recovery-key-cancel-done = Çelësi i ri u anulua.
compromised-new-key = Ruaje tani çelësin e ri të rikuperimit: e zëvendëson të vjetrin pas 72 orësh, dhe deri atëherë i vjetri funksionon ende.
pending-reset-title = U kërkua rivendosja e llogarisë sate
pending-reset-body = Hyn në fuqi më { $when }: atëherë humbasin preferencat, të dhënat e pagesës dhe kodet e miqve, dhe pajisjet duhet të hyjnë sërish. Nuk ishe ti? Anuloje me fjalëkalimin aktual.
pending-reset-cancel = Anulo rivendosjen
pending-reset-cancelled = Rivendosja u anulua.
login-forgot = Harrove fjalëkalimin?
forgot-title = Harrove fjalëkalimin
forgot-body = Shkruaj email-in. Nëse ka llogari, dërgojmë një lidhje për të zgjedhur fjalëkalim të ri.
forgot-submit = Dërgo lidhjen
forgot-sent = Nëse kjo adresë ka llogari, një lidhje është rrugës. Skadon pas 24 orësh.
reset-title = Rikupero llogarinë tënde
reset-with-key-body = Shkruaj çelësin e rikuperimit dhe një fjalëkalim të ri: gjithçka kthehet siç ishte.
reset-submit-key = Rikupero llogarinë time
reset-no-key = Nuk e kam çelësin e rikuperimit
reset-without-key-title = Rivendos pa çelësin e rikuperimit
reset-without-key-body = Llogaria jote nis nga e para me fjalëkalim të ri pas 72 orësh. Projektet kthehen nga pajisjet e tua dhe lidhjet e ndarjes; preferencat, të dhënat e pagesës dhe kodet e miqve jo. Deri atëherë fjalëkalimi aktual funksionon dhe mund ta anulojë rivendosjen.
reset-name = Emri yt
reset-submit-request = Rivendos llogarinë time
reset-requested = U krye. Fjalëkalimi i ri funksionon nga { $when }. Deri atëherë fjalëkalimi aktual funksionon dhe mund ta anulojë.
reset-pending-already = Një rivendosje është tashmë në pritje deri më { $when }. Një kërkesë e re i nis 72 orët nga e para.
recovered-done = Llogaria jote u rikthye. Pajisjet e tjera dolën.
login-reset-undoable = Ky fjalëkalim u zëvendësua nga një rivendosje në 30 ditët e fundit. Nëse nuk e kërkove, zhbëje: llogaria kthehet siç ishte.
login-undo-reset = Zhbëje rivendosjen
reset-undone = Rivendosja u zhbë. Llogaria është si më parë.
error-reset-pending = Ky fjalëkalim i ri funksionon kur rivendosja hyn në fuqi, 72 orë pas kërkesës.
error-account-disabled = Kjo llogari u fshi pak pas një rivendosjeje dhe pret të fshihet. Fjalëkalimi i mëparshëm mund ta zhbëjë ende rivendosjen.
error-recovery-key-invalid = Ky çelës rikuperimi nuk e hap këtë llogari.
error-project-key-required = Ky projekt ka nevojë për çelësin e tij. Hape sërish nga lidhja e ndarjes.
names-or = { $first } ose { $last }
participants-admin-badge = Admin
edit-project-admins = Adminë: { $count }
edit-project-succession = Nëse projekti mbetet pa admin, rolin e merr anëtari me llogari që është në projekt prej më shumë kohe.
member-sheet-account = Anëtar me llogari
member-actions = Veprime: { $name }
member-make-admin = Jep rolin e adminit
member-make-admin-hint = Do të mund të ndryshojë çelësin e projektit dhe të heqë dikë.
member-made-admin = { $name } tani ka rolin e adminit.
group-security = Siguria e projektit
group-security-admin = Siguria e projektit · ke rolin e adminit
group-rotate = Ndrysho çelësin e projektit
group-rotate-hint = Nëse lidhja e projektit është komprometuar: ata që kanë lidhjen e vjetër humbasin aksesin në projekt.
group-remove = Hiq dikë nga projekti
group-remove-hint = I heq aksesin dhe ndryshon çelësin njëkohësisht.
group-admins-only = Vetëm për adminët.
group-admins-only-account = Vetëm për adminët, që kanë llogari.
group-ask-admin = Pyet adminin e projektit: { $names }.
group-ask-admins = Pyet adminët e projektit: { $names }.
group-ask-an-admin = Pyet një nga adminët e projektit.
group-with-account = Me llogari
group-free = Falas
group-sign-in-hint = Një admin i projektit ({ $names }) mund të ta japë më pas rolin.
group-sign-in-hint-any = Një admin i projektit mund të ta japë më pas rolin.
group-no-admin = Ky projekt nuk ka admin, prandaj çelësi i tij nuk mund të ndryshojë.
group-hide = Fshih dikë
group-hide-hint = E fsheh nga listat pa i hequr aksesin: lidhja dhe çelësi vazhdojnë të funksionojnë.
group-recreate-hint = Ky projekt nuk ka admin. Për të hequr dikë, krijo një projekt të ri.
group-recreate = Krijo një projekt të ri
group-no-admin-yet = Ky projekt ende nuk ka admin.
group-creator-hint = Vetëm ti mund të bëhesh admin i këtij projekti, duke krijuar një llogari.
group-creator-elsewhere-hint = Vetëm personi që krijoi projektin mund të bëhet admin i tij, duke krijuar një llogari. Përndryshe, krijo një projekt të ri për të hequr dikë.
member-revoke-admin = Hiq rolin e adminit
member-revoke-admin-hint = Nuk do të mund të ndryshojë më çelësin e projektit, as të heqë askënd.
member-revoked-admin = { $name } nuk ka më rolin e adminit.
member-step-down = Mos ji më admin
member-step-down-hint = Nuk do të mund të ndryshosh më çelësin e projektit, as të heqësh askënd.
member-stepped-down = Nuk ke më rolin e adminit.
member-remove = Hiq nga projekti
member-remove-hint = Ndryshon çelësin e projektit: { $name } nuk do të mund të lexojë asgjë të re.
member-hide = Fshih nga listat
error-user-not-admin = Ky anëtar nuk është admin.
error-creator-proof-invalid = Kjo pajisje nuk e krijoi këtë projekt.
error-project-has-admin = Ky projekt ka tashmë admin.
error-not-project-admin = Këtë mund ta bëjnë vetëm adminët e projektit.
error-user-not-claimed = Ky person nuk ka llogari, prandaj nuk mund të jetë admin.
error-project-key-rotated = Çelësi i projektit ndryshoi. Kërkoji një anëtari lidhjen e re.
error-project-rotating = Çelësi i projektit po ndryshohet. Provo sërish pas pak.
error-rotation-expired = Ndryshimi i çelësit zgjati shumë dhe u anulua. Fillo nga e para.
error-rotation-rows-changed = Projekti ndryshoi gjatë ndryshimit të çelësit. Fillo nga e para.
project-rotated-title = Nuk ke më akses në këtë projekt
project-rotated-body = Çelësi i këtij projekti u ndryshua nga admini. Kërkoji adminit lidhjen e re.
project-rotated-open-link = Hap një lidhje
rotation-remove-title = Të hiqet { $name } nga projekti?
rotation-rotate-title = Të ndryshohet çelësi i projektit?
rotation-remove-body = Heqja e një anëtari do të thotë krijim i një çelësi të ri. Lidhjen e re do të duhet ta ndash me personat pa llogari.
rotation-rotate-body = Kush ka lidhjen e vjetër, do ta humbasë aksesin në projekt. Lidhjen e re do të duhet ta ndash me personat pa llogari.
rotation-boxed =
    { $count ->
        [0] Asnjë anëtar tjetër nuk do ta marrë çelësin automatikisht
        [one] { $count } anëtar do ta marrë çelësin automatikisht
       *[other] { $count } anëtarë do ta marrin çelësin automatikisht
    }
rotation-by-link-title = Personat të cilëve do të duhet t’u dërgosh lidhjen e re
rotation-by-link-hint = Këta persona nuk kanë llogari. Do t’ua dërgosh lidhjen menjëherë pas kësaj.
rotation-names-and = { $first } dhe { $last }
rotation-names-and-others = { $names } dhe { $count } të tjerë
rotation-confirm-remove = Hiq dhe ndrysho çelësin
rotation-confirm-rotate = Ndrysho çelësin
rotation-done-title = Çelësi i ri është në fuqi
rotation-done-removed = { $name } nuk është më pjesë e projektit.
rotation-done-rotated = Çelësi i projektit ndryshoi.
rotation-done-boxed =
    { $count ->
        [0] Asnjë anëtar tjetër nuk e mori çelësin automatikisht.
        [one] { $count } anëtar e mori çelësin automatikisht.
       *[other] { $count } anëtarë e morën çelësin automatikisht.
    }
rotation-new-link =
    { $count ->
        [0] Lidhje e re e projektit
        [one] Lidhje e re për t’u dërguar · { $count } person pa llogari
       *[other] Lidhje e re për t’u dërguar · { $count } persona pa llogari
    }
rotation-link-renewed = Lidhja u rinovua me sukses.
rotation-done = U krye
history-key-changed = Çelësi i projektit u ndryshua
history-member-removed = { $name } nuk është më pjesë e projektit
project-rotated-row-hint = Çelësi ndryshoi. Kërko lidhjen e re.
