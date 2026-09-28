# Lietuvių. Pilnas, išskyrus teisinius tekstus (legal-, terms-, privacy-), kurie yra tik anglų ir
# prancūzų kalbomis ir kiekvienam pranešimui atskirai grįžta į en.ftl.

### Common

loading = Įkeliama…
cancel = Atšaukti
confirm = Patvirtinti
retry = Bandyti dar kartą
delete = Ištrinti
back = Atgal
language = Kalba

### Navigation

nav-main = Pagrindinė navigacija
nav-projects = Projektai
nav-charts = Statistika
nav-settings = Nustatymai

### Connectivity

offline-banner = Neprisijungta
offline-pending =
    { $count ->
        [one] { $count } laukiantis
        [few] { $count } laukiantys
        [many] { $count } laukiančio
       *[other] { $count } laukiančių
    }

sync-conflict-edit = Konfliktas: nepavyko redaguoti „{ $name }“ (elementas ištrintas). Praleista.
sync-conflict-delete = Konfliktas: nepavyko ištrinti „{ $name }“ (elementas ištrintas). Praleista.
sync-conflict-other = Konfliktas: operacija su „{ $name }“ nepavyko (elementas ištrintas). Praleista.
sync-error = Sinchronizavimo klaida: { $reason }

### Errors

error-network = Nepavyksta pasiekti serverio. Patikrink interneto ryšį.
error-generic = Kažkas nepavyko. Bandyk dar kartą.

error-invalid-email = Šis el. pašto adresas negalioja.
error-invalid-password = Šis slaptažodis negalioja.
error-password-too-short = Slaptažodį turi sudaryti bent 8 simboliai.
error-client-outdated = Ši programėlės versija pasenusi. Atnaujink ją, kad galėtum prisijungti.
error-invalid-link = Ši nuoroda negalioja.
error-batch-too-large = Per daug elementų vienu metu.
error-payers-required = Pasirink bent vieną mokėtoją.
error-debtors-required = Pasirink bent vieną skolininką.
error-duplicate-participant = Dalyvis toje pačioje pusėje pasikartoja du kartus.
error-participant-not-in-project = Šis dalyvis nepriklauso šiam projektui.
error-too-many-participants = Per daug dalyvių vienai išlaidai.
error-invalid-credentials = Neteisingas el. paštas arba slaptažodis.
error-unauthenticated = Prisijunk, kad galėtum tai padaryti.
error-email-not-verified = Tavo el. pašto adresas dar nepatvirtintas.
error-project-not-found = Šio projekto nebėra.
error-expense-not-found = Šios išlaidos nebėra.
error-user-not-found = Šio dalyvio nebėra.
error-tricount-not-found = Tricount nerastas arba jo API grąžino klaidą.
error-too-many-members = Šis projektas pasiekė narių limitą.
error-identity-taken = Kita paskyra jau pasisavino šį dalyvį.
error-claim-proof-invalid = Šis įrenginys neturi projekto rakto, todėl negali pasisavinti dalyvio. Dar kartą atidaryk bendrinimo nuorodą.
error-user-has-payments = Šis dalyvis turi išlaidų projekte ir negali būti pašalintas.
error-resend-cooldown = Palauk 60 sekundžių, prieš prašydamas kito laiško.
error-self-friend-request = Negali pridėti savęs kaip draugo.
error-not-a-friend = Gali kviesti tik žmones iš savo draugų sąrašo.
error-friend-has-no-key = Šis draugas dar neatidarė naujausios programėlės versijos. Paprašyk jo kartą prisijungti ir bandyk dar kartą.
error-friend-request-not-found = Šios draugystės užklausos nebėra.
error-invitation-not-found = Šio kvietimo nebėra.
error-too-many-friend-requests = Kol kas per daug draugystės užklausų. Bandyk rytoj.
error-too-many-invitations = Per daug laukiančių kvietimų.
error-invalid-kdf-salt = Šifravimo nustatymai netinkami. Atnaujink programėlę ir bandyk dar kartą.
error-mixed-project-batch = Šie dalyviai ne visi priklauso tam pačiam projektui.
error-invalid-payload = Ši programėlės versija išsiuntė duomenis, kurių serveris nepriima. Atnaujink ją ir bandyk dar kartą.
error-invalid-public-key = Tavo šifravimo raktas netinkamas. Atnaujink programėlę ir bandyk dar kartą.
error-payment-methods-stale = Tavo mokėjimo duomenys pakeisti kitame įrenginyje. Įkelk iš naujo ir bandyk dar kartą.

### Auth

field-email = El. paštas
field-email-placeholder = tu@pavyzdys.lt
field-password = Slaptažodis
field-name = Vardas
field-name-placeholder = Ona Kazlauskienė

login-title = Prisijungti
login-submit = Prisijungti
login-submitting = Jungiamasi…
login-password-placeholder = Tavo slaptažodis
login-no-account = Dar neturi paskyros?
login-unverified = Tavo el. pašto adresas dar nepatvirtintas. Patikrink pašto dėžutę arba išsiųsk nuorodą dar kartą.
login-resend = Išsiųsti patvirtinimo nuorodą dar kartą
login-resending = Siunčiama…
login-resend-sent = Laiškas išsiųstas - patikrink pašto dėžutę.

register-submit = Sukurti paskyrą
register-submitting = Kuriama…
register-have-account = Jau turi paskyrą?
register-password-placeholder = Bent 8 simboliai
register-password-warning = Užsirašyk slaptažodį. Jei jį pamirši, paskyros atkurti nepavyks.
register-check-email-title = Patikrink el. paštą
register-email-sent = Laiškas išsiųstas
register-email-sent-hint = Spustelėk nuorodą pašto dėžutėje, kad aktyvuotum paskyrą.
register-not-received-prefix = Negavai? Patikrink šlamšto aplanką arba
register-sign-in-link = prisijunk
register-not-received-suffix = kad nuoroda būtų išsiųsta dar kartą.
register-terms-prefix = Kurdamas paskyrą sutinki su mūsų
register-terms-link = naudojimo sąlygomis
register-terms-and = ir mūsų
register-privacy-link = privatumo politika

settings-title = Nustatymai
settings-preferences = Nuostatos
settings-preferences-local = Saugoma šiame įrenginyje.
settings-preferences-synced = Sinchronizuota su tavo paskyra, užšifruota.
settings-about = Apie
settings-anonymous-title = Nesi prisijungęs
settings-upsell-title = Tavo projektai visuose įrenginiuose
settings-upsell-free = Nemokama
settings-upsell-body = Counted veikia ir be paskyros. Su nemokama paskyra tavo projektai ir nuostatos keliauja kartu į telefoną, nešiojamąjį kompiuterį ir naršyklę - vis dar užšifruoti, vis dar neįskaitomi mums.
settings-locked-badge = Paskyra
settings-locked-friends = Susikurk paskyrą, kad galėtum pridėti draugų ir kviesti juos į projektą tiesiai iš programėlės - be jokios nuorodos.
settings-locked-payment-methods = Vieną kartą išsaugok savo IBAN arba mokėjimo programėlę ir bendrink su pasirinktais projektais. Kas tau skolingas, matys tai šalia tavo vardo.
settings-friends-hint = Pridėk draugų ir kviesk juos į savo projektus nebendrindamas nuorodos.

account-member-since = Narys nuo
account-logout = Atsijungti
account-logging-out = Atsijungiama…
account-delete-title = Ištrinti mano paskyrą
account-delete-warning = Nedelsiant ir visam laikui, be šiukšlinės. Išlaidos, kurias įvedei bendrame projekte, lieka matomos kitiems nariams - jos yra jų apskaitos dalis.
account-delete-confirm-title = Ištrinti paskyrą
account-delete-confirm-message = Tavo paskyra, sesijos ir projektų sąrašas bus ištrinti visam laikui. Be slaptažodžio bendro projekto užšifruoti duomenys taps tau neįskaitomi - to atšaukti negalima.

settings-payment-methods = Mokėjimo duomenys
settings-payment-methods-hint = Kaip norėtum gauti pinigus atgal. Užšifruota su tavo paskyra.
payment-method-kind = Būdas
payment-method-kind-other = Kita
payment-method-label = Pavadinimas
payment-method-label-placeholder = Pagrindinė sąskaita
payment-method-value = Duomenys
payment-method-value-placeholder = IBAN, telefono numeris, naudotojo vardas…
payment-method-add = Pridėti
payment-method-remove = Pašalinti { $name }
payment-method-empty = Dar nepridėjai mokėjimo duomenų.
payment-method-deleted = Mokėjimo būdas ištrintas.
payment-method-value-required = Užpildyk kiekvieno mokėjimo būdo duomenis arba jį pašalink.
payment-method-label-required = Suteik savo mokėjimo būdui pavadinimą.
payment-method-too-long = Per ilgas - sutrumpink.
payment-method-invalid-characters = Pašalink eilučių lūžius ar nematomus simbolius.
payment-method-limit = Gali išsaugoti iki { $max } mokėjimo būdų.
payment-methods-saved = Mokėjimo duomenys išsaugoti.
payment-methods-offline = Norint išsaugoti mokėjimo duomenis, reikia interneto ryšio.
payment-methods-stale = Tavo mokėjimo duomenys buvo pakeisti kitame įrenginyje. Jie įkelti iš naujo — bandyk dar kartą.
payment-methods-key-missing = Prisijunk iš naujo, kad galėtum tvarkyti mokėjimo duomenis.
settings-payment-methods-share-warning = Bendrinamą būdą mato visi projektų, kuriuose pasirinkai savo vardą, nariai - kiekvienas, turintis vieną iš tų nuorodų.
payment-method-share = Bendrinti su mano projektais
payment-method-share-hint = Rodoma šalia tavo vardo, kai kas nors tau skolingas.
payment-method-copy = Kopijuoti { $name }
payment-method-copied = Nukopijuota.
payment-method-copy-failed = Nepavyko nukopijuoti - pažymėk tekstą ir nukopijuok rankiniu būdu.

verify-email-checking = Tikrinamas el. pašto adresas…
verify-email-welcome = El. paštas patvirtintas - sveiki atvykę į Counted!
verify-email-back-to-login = Grįžti į prisijungimą

### Project status

project-close = Uždaryti
project-archive = Archyvuoti
project-reopen = Atidaryti iš naujo
project-unarchive = Grąžinti iš archyvo

### Dates

date-long = { $year } m. { $month } { $day } d.

month-1 = sausio
month-2 = vasario
month-3 = kovo
month-4 = balandžio
month-5 = gegužės
month-6 = birželio
month-7 = liepos
month-8 = rugpjūčio
month-9 = rugsėjo
month-10 = spalio
month-11 = lapkričio
month-12 = gruodžio

month-short-1 = sau
month-short-2 = vas
month-short-3 = kov
month-short-4 = bal
month-short-5 = geg
month-short-6 = bir
month-short-7 = lie
month-short-8 = rgp
month-short-9 = rgs
month-short-10 = spa
month-short-11 = lap
month-short-12 = grd

### Actions

add = Pridėti
create = Sukurti
creating = Kuriama…
edit = Redaguoti
leave = Išeiti
close = Uždaryti
paste = Įklijuoti
join = Prisijungti
import = Importuoti
importing = Importuojama…
field-description = Aprašymas
field-date = Data
date-today = Šiandien
date-yesterday = Vakar
field-optional = Neprivaloma

### Projects

projects-filter-active = Aktyvūs
projects-filter-all = Visi
projects-count-label = Projektai
projects-empty = Projektų nėra
projects-empty-hint = Sukurk projektą mygtuku žemiau
projects-offline-banner = Duomenys neprisijungus - prisijunk iš naujo, kad atnaujintum.
projects-no-local-data = Nėra vietinių duomenų
projects-no-local-data-hint = Prisijunk, kad pirmą kartą įkeltum savo projektus.
projects-add = Pridėti projektą
projects-create = Sukurti projektą
projects-join = Prisijungti prie projekto
projects-import-tricount = Importuoti iš Tricount
project-actions = Projekto veiksmai

status-ongoing = Vykdomas
status-closed = Uždarytas
status-archived = Archyvuotas

nav-help = Pagalba
nav-privacy = Privatumo politika
nav-terms = Naudojimo sąlygos
nav-legal = Teisinė informacija

leave-project-title = Išeiti iš projekto?
leave-project-message = Prarasi prieigą iš šio įrenginio. Jei neliks nė vieno nario, projektas ir visos jo išlaidos bus ištrinti visam laikui.

add-project-title = Naujas projektas
add-project-name-label = Projekto pavadinimas
add-project-name-placeholder = Mano kelionė, Bendrabutis 2024…
add-project-participants = Dalyviai
add-project-participant-name = Dalyvio vardas
add-project-participant-placeholder = Clark Kent
add-project-remove-participant = Pašalinti dalyvį
add-project-me-badge = Aš
add-project-thats-me = Tai aš!
add-project-offline = Neprisijungus projekto sukurti negalima. Prisijunk iš naujo ir bandyk dar kartą.
add-project-name-required = Projektui reikia pavadinimo.
add-project-need-two-participants = Pridėk bent 2 dalyvius.
add-project-pick-yourself = Nurodyk, kuris dalyvis esi tu.

join-link-label = Bendrinimo nuoroda
join-link-hint = Nuorodoje yra iššifravimo raktas - nukopijuok ją visą.
join-invalid-link = Ši nuoroda negalioja. Įklijuok visą bendrinimo nuorodą, įskaitant dalį po #.
join-wrong-project = Ši nuoroda skirta kitam projektui.

import-tricount-link-label = Tricount nuoroda arba raktas
import-tricount-key-required = Įvesk Tricount nuorodą arba raktą.
import-tricount-encryption-failed = Šifravimas nepavyko.

### Expenses

save = Išsaugoti
saving = Saugoma…
adding = Pridedama…
link-copied = Nuoroda nukopijuota
missing-encryption-key = Trūksta šifravimo rakto.
missing-encryption-key-title = Trūksta šifravimo rakto
missing-encryption-key-hint = Naudota nuoroda neturi rakto, reikalingo šiam projektui iššifruoti. Naudok visą nuorodą, kurią pasidalijo projekto kūrėjas.
project-locked-hint = Šis įrenginys neturi šio projekto rakto. Atidaryk jo bendrinimo nuorodą, kad atrakintum.
project-unlock = Atrakinti
project-no-local-data-hint = Prisijunk, kad pirmą kartą įkeltum šio projekto duomenis.
project-gone-title = Šio projekto nebėra
project-gone-hint = Jis buvo ištrintas, kai išėjo paskutinis narys. Bendrinimo nuoroda nebeveikia, net jei ją atidarysi iš naujo.

expense-add = Pridėti išlaidą
transfer-add = Pridėti pervedimą
expense-edit-title = Redaguoti išlaidą
expense-category = Kategorija
expense-category-auto = Auto · { $emoji }
expense-currency = Sumos valiuta
amount-op-add = Plius
amount-op-subtract = Minus
amount-op-multiply = Dauginti
amount-op-divide = Dalyti
amount-op-equals = Lygu
amount-op-done = Atlikta
expense-rate = Valiutos kursas (neprivaloma)
expense-rate-hint = Palik tuščią, kad būtų naudojamas Europos Komisijos (InforEuro) { $month } kursas: 1 { $from } = { $rate } { $to }.
expense-rate-invalid = Įvesk kursą, didesnį nei 0.
expense-rate-unavailable = Automatinis kursas nepasiekiamas - įvesk jį rankiniu būdu.
expense-delete-title = Ištrinti išlaidą
expense-delete-message = „{ $name }“ bus ištrinta visam laikui. To atšaukti negalima.
expense-inconsistent-amounts = Sumos nesutampa
expenses-empty = Išlaidų nėra
expenses-empty-hint = Pradėk pridėdamas išlaidas mygtuku žemiau
expenses-show-more = Rodyti daugiau (liko { $count })

expense-type-expense = Išlaida
expense-type-transfer = Pervedimas
expense-type-gain = Pajamos
expense-paid-by = sumokėjo
expense-sent-by = išsiuntė
expense-contributed-by = prisidėjo

expense-name-required = Pavadinimas privalomas.
expense-amount-not-positive = Suma turi būti didesnė nei 0.
expense-no-payer = Pasirink bent vieną mokėtoją.
expense-no-debtor = Pasirink bent vieną skolininką.
expense-invalid-date = Ši data negalioja.
expense-payers-mismatch = Mokėtojų suma yra { $sum }, o tai neatitinka išlaidos sumos ({ $total }).
expense-debtors-mismatch = Skolininkų suma yra { $sum }, o tai neatitinka išlaidos sumos ({ $total }).

participants-none = Niekas
participants-everyone = Visi ({ $count })
participants-some = { $count } iš { $total }
participants-select-all = Pažymėti visus
participants-by-shares = Pagal dalis
split-amounts = Sumos
participants-remaining = Liko { $amount }
participants-over-by = { $amount } per daug
participants-who-paid = Kas sumokėjo?
participants-who-received = Kas gavo?
participants-who-transfers = Kas perveda?
participants-who-receives = Kas gauna?
participants-for-whom = Kam?

stats-total-expenses = Iš viso išlaidų
stats-my-expenses = Mano išlaidos

tab-expenses = Išlaidos
tab-balance = Balansas
tab-reimbursements = Atsiskaitymas
reimbursements-empty-title = Viskas atsiskaityta!
reimbursements-empty-hint = Atsiskaitymo pasiūlymai rodomi čia, kai sąskaitos nesubalansuotos
reimbursement-owes = { $debtor } skolingas { $creditor }
reimbursement-record = Atsiskaityti
reimbursement-pay-with = Mokėti
reimbursement-pay-shared-by = Pasidalijo { $name } - prieš siųsdamas patikrink gavėjo vardą, kurį rodo tavo programėlė.
reimbursement-pay-title = Mokėti { $name }
reimbursements-mine-title = Tu skolingas
reimbursements-others-title = Kiti grąžinimai
copy = Kopijuoti

user-selection-title = Kuris dalyvis esi tu?
user-selection-hint = Pasirink savo vardą iš sąrašo.
user-selection-required = Pasirink dalyvį.
identity-claimed = Susieta su paskyra
identity-claimed-by = { $name } paskyra
identity-taken-repick = Kita paskyra pasisavino dalyvį, kurį naudojai. Pasirink kitą.
participant-gone-repick = Dalyvis, kurį naudojai, buvo pašalintas iš šio projekto. Pasirink kitą.

edit-project-title = Redaguoti projektą
edit-project-new-badge = naujas
edit-project-deferred-new-members = naujų narių pridėjimas
edit-project-deferred-removals = narių šalinimas
edit-project-deferred-me = pasirinkimas „Tai aš“
edit-project-offline-deferred = Neprisijungta: { $items } bus pritaikyta prisijungus.

export-saved = Failas išsaugotas:
    { $path }
export-failed = Eksportas nepavyko: { $reason }

history-expense-added = Išlaida pridėta: { $name }
history-expense-edited = Išlaida redaguota: { $name }
history-expense-deleted = Išlaida ištrinta: { $name }
history-project-edited = Projektas redaguotas: { $name }
history-name-changed = Pavadinimas: „{ $from }“ → „{ $to }“
history-description-added = Aprašymas pridėtas: „{ $value }“
history-description-removed = Aprašymas pašalintas: „{ $value }“
history-description-changed = Aprašymas: „{ $from }“ → „{ $to }“

### Sweep

field-amount = Suma
expense-name-placeholder = Restoranas, maisto prekės…
expense-actions = Išlaidos veiksmai
expense-your-share = Tavo dalis
expense-your-share-value = Tavo dalis: { $amount } { $currency }
expense-inconsistent-detail = Sumos nesutampa: sumokėta { $paid }, skolinga { $owed }, o išlaida yra { $total }. Redaguok išlaidą, kad ištaisytum.
missing-access-key = Trūksta prieigos rakto. Atidaryk šį projektą per jo bendrinimo nuorodą.
filter-all = Visos
filter-my-payments = Mano mokėjimai
filter-my-debts = Ką esu skolingas
participants-shares-for = { $name } dalys
participants-amount-for = { $name } suma
reimbursement-add = Pridėti atsiskaitymą
project-forget = Pašalinti iš mano sąrašo
project-history-title = Istorija
history-kind-add = Pridėta
history-kind-delete = Ištrinta
history-kind-edit = Redaguota
export = Eksportuoti
export-json = Eksportuoti JSON
export-csv = Eksportuoti CSV
share-link = Bendrinti
copy-link-failed = Nepavyko nukopijuoti nuorodos
open-in-app = Atidaryti programėlėje
not-found-title = Puslapis nerastas
not-found-back = Grįžti į projektus

### Charts

charts-period = Laikotarpis
period-all = Viskas
period-month = Mėnuo
period-3months = 3 mėn.
period-year = Metai
period-custom = Pasirinktinis
charts-tab-categories = Kategorijos
charts-tab-trends = Tendencijos
charts-total-spent = Iš viso išleista
charts-avg-per-person = Vid. asmeniui
charts-expense-count =
    { $count ->
        [one] { $count } išlaida
        [few] { $count } išlaidos
        [many] { $count } išlaidos
       *[other] { $count } išlaidų
    }
charts-nothing-to-show = Nėra ką rodyti
charts-my-share-note = Šios sumos yra tavo dalis kiekvienoje išlaidoje.
charts-my-share-skipped =
    { $count ->
        [one] { $count } projektas neįskaičiuotas — nepasirinktas dalyvis arba jo duomenys neįkelti.
        [few] { $count } projektai neįskaičiuoti — nepasirinktas dalyvis arba jų duomenys neįkelti.
        [many] { $count } projekto neįskaičiuota — nepasirinktas dalyvis arba jų duomenys neįkelti.
       *[other] { $count } projektų neįskaičiuota — nepasirinktas dalyvis arba jų duomenys neįkelti.
    }

### Categories

category-food = Maistas
category-transport = Transportas
category-accommodation = Apgyvendinimas
category-leisure = Laisvalaikis
category-shopping = Pirkiniai
category-services = Paslaugos
category-parties-gifts = Vakarėliai ir dovanos
category-other = Kita
charts-project = Projektas
charts-all-projects = Visi projektai
charts-date-from = Nuo
charts-date-to = Iki
charts-total = Iš viso
charts-tab-people = Žmonės
charts-tab-projects = Projektai
charts-scope = Kieno išlaidos
charts-scope-group = Grupė
charts-scope-me = Aš
charts-currency = Valiuta
charts-my-share = Mano dalis
charts-share-of-total = { $pct } % iš { $total }
charts-i-paid = Sumokėjau
charts-paid-more = { $amount } daugiau nei tavo dalis
charts-paid-less = { $amount } mažiau nei tavo dalis
charts-paid-even = Lygiai tavo dalis
charts-part-title = Tavo dalis kiekvienoje kategorijoje
charts-part-desc = Pilka – ką išleido grupė, spalva – ką suvartojai tu.
charts-breakdown-title = Pasiskirstymas pagal kategorijas
charts-breakdown-desc = Paliesk sektorių ar eilutę, kad pamatytum išlaidas.
charts-of-total = { $amount } iš { $total }
charts-show-all = Rodyti viską ({ $count })
charts-show-less = Rodyti mažiau
charts-spend-title = Išlaidos laike
charts-spend-desc = Trumpi laikotarpiai rodomi pagal dienas, ilgesni – pagal savaites ar mėnesius.
charts-group-by = Grupuoti pagal
bucket-day = Diena
bucket-week = Savaitė
bucket-month = Mėnuo
charts-avg = vid.
charts-cat-title-day = { $category }, diena po dienos
charts-cat-title-week = { $category }, savaitė po savaitės
charts-cat-title-month = { $category }, mėnuo po mėnesio
charts-cat-desc = Pasirink kategoriją, kad sektum ją laike.
charts-running-title = Suma iš viso
charts-running-desc = Nuo { $date }.
charts-avg-per-day = { $amount } / dienai vidutiniškai
charts-avg-per-week = { $amount } / savaitei vidutiniškai
charts-avg-per-month = { $amount } / mėnesiui vidutiniškai
charts-people-title = Kas nešė grupę
charts-people-desc = Ką kiekvienas sumokėjo, šalia to, ką suvartojo.
charts-paid = Sumokėta
charts-fair-share = Teisinga dalis
charts-you = (tu)
charts-net-more = sumokėjo daugiau
charts-net-less = sumokėjo mažiau
charts-balance-title = Tavo balansas laike
charts-balance-desc = Virš linijos grupė skolinga tau. Žemiau – tu skolingas grupei.
charts-owed = Tau skolingi
charts-owe = Tu skolingas
charts-projects-title = Tavo dalis pagal projektą
charts-projects-desc = Sumos laikomos pagal valiutą ir niekada nesudedamos.
history-empty = Įvykių nėra
history-by = { $name }
not-found-hint = Šio puslapio nėra arba jis perkeltas.
payers-title-paid-by = Sumokėjo
payers-title-sender = Siuntėjas
payers-title-contributors = Prisidėję
debtors-title-debtors = Skolingi
debtors-title-recipients = Gavėjai
debtors-title-beneficiaries = Naudos gavėjai

### Welcome

welcome-title = Tavo apskaita - ne kitų reikalas.
welcome-subtitle = Dalinkis išlaidomis su draugais.
welcome-e2ee-title = Viskas užšifruota
welcome-e2ee-body = Vardai, sumos, projektai: viskas užšifruojama tavo įrenginyje. Raktą turi tik tu. Niekas negali skaityti tavo sąskaitų. Net ir mes.
welcome-e2ee-note = Neįskaitoma net mums (serveris neturi prieigos)
welcome-eu-title = 100 % europietiška
welcome-eu-body = Serveriai Vokietijoje, laiškai siunčiami iš Prancūzijos. Tavo duomenys niekada nepalieka Europos Sąjungos.
welcome-noads-title = Jokių reklamų. Jokių sekimo įrankių.
welcome-noads-body = Nieko nerenkame ir neparduodame tavo duomenų. Tai ne mūsų modelis.
welcome-start = Pradėti
welcome-how-it-works = Kaip tiksliai tai veikia?

### Help

help-intro = Dažnas klausimas? Palies, kad išskleistum atsakymą.
help-create-project-q = Kaip sukurti projektą?
help-create-project-a = Pradiniame ekrane paliesk mygtuką + apačioje. Suteik projektui pavadinimą, pasirink valiutą ir viskas.
help-add-participants-q = Kaip pridėti dalyvių?
help-add-participants-a = Atidaryk projektą ir pridėk dalyvius iš narių sąrašo. Kiekvienas dalyvis gali mokėti už išlaidą arba būti už ją skolingas.
help-share-project-q = Kaip bendrinti projektą?
help-share-project-a = Pasidalink projekto URL (tuo, kuris adreso juostoje). Kiekvienas, turintis nuorodą, gali peržiūrėti ir redaguoti projektą.
help-add-expense-q = Kaip pridėti išlaidą?
help-add-expense-a = Projekte paliesk +, įvesk sumą, nurodyk, kas sumokėjo ir tarp ko padalinti. Taip pat gali pasirinkti kitą datą nei šiandien.
help-types-q = Kuo skiriasi išlaida, pervedimas ir pajamos?
help-types-expense = - vieno asmens pirkinys, padalintas keliems.
help-types-transfer = - grąžinimas iš vieno asmens kitam, be dalijimo.
help-types-gain = - gauti pinigai (grąžinimas, dovana), dalijami keliems asmenims.
help-past-date-q = Ar galiu nurodyti praėjusią išlaidos datą?
help-past-date-a = Taip, datos laukas laisvas. Įrašo sukūrimo laikas saugomas atskirai.
help-who-owes-q = Kaip Counted apskaičiuoja, kas kam skolingas?
help-who-owes-a = Counted apskaičiuoja kiekvieno dalyvio grynąjį balansą (kiek sumokėjo minus kiek skolingas), tada pasiūlo trumpiausią pervedimų seką, kuri atsiskaito su visais.
help-minimal-transfers-q = Kodėl siūlomų pervedimų skaičius minimalus?
help-minimal-transfers-a = Algoritmas pirmiausia suporuoja balansus, kurie tiksliai panaikina vienas kitą, tada pereina per likusius nuo didžiausio kreditoriaus iki didžiausio skolininko. Rezultatas: mažiau pervedimų viskam atsiskaityti.
help-import-tricount-q = Kaip importuoti projektą iš Tricount?
help-import-tricount-a = Pradiniame ekrane paliesk mygtuką „+“ apačioje, tada
help-import-tricount-b = Įklijuok Tricount, kurį nori importuoti, bendrinimo nuorodą.
help-encryption-q = Ar mano duomenys užšifruoti?
help-encryption-a = Taip. Counted derina dvi garantijas:
help-encryption-e2ee-term = Ištisinis šifravimas
help-encryption-e2ee-def = - viskas tarp tavęs ir serverio keliauja užšifruota.
help-encryption-zero-term = Nulinė prieiga
help-encryption-zero-def = - duomenis užšifruoji prieš išsiųsdamas, o serveris saugo tik šifruotą tekstą. Neturime jokios galimybės jo perskaityti.
help-encryption-see = Išsamiau žr.
help-forgot-password-q = Kas nutiks, jei pamiršiu slaptažodį?
help-forgot-password-warning = Tavo duomenys bus prarasti visam laikui.
help-forgot-password-a = Šifravimo raktas išvedamas iš tavo slaptažodžio, todėl atstatyti neįmanoma: niekas - net mes - negali iššifruoti tavo projektų be jo. Saugok jį saugiai, geriausia slaptažodžių tvarkyklėje.
help-archive-delete-q = Kaip archyvuoti arba ištrinti projektą?
help-archive-delete-a = Projekto ekrane atidaryk meniu ir pasirink
help-archive-delete-b = kad paslėptum jį išsaugodamas. Projektas ištrinamas visam laikui, kai iš jo išeina paskutinis narys.
help-delete-account-q = Kaip ištrinti paskyrą?
help-delete-account-a = Atidaryk Nustatymus ir naudok „Ištrinti mano paskyrą“. Tai įvyksta iš karto ir atšaukti negalima.
help-contact = Kitas klausimas? Rašyk mums adresu

# Receipt scanning (mobile only)
expense-scan = Nuskaityti kvitą
scan-in-progress = Skaitomas kvitas…
scan-error-capture = Nepavyko nufotografuoti. Bandyk dar kartą arba įvesk išlaidą rankiniu būdu.
scan-error-unreadable = Šiame kvite nieko įskaitomo. Įvesk išlaidą rankiniu būdu.
scan-check-amount = Patikrink bendrą sumą - ji buvo neaiškiai išspausdinta.
scan-take-photo = Nufotografuoti
scan-choose-photo = Pasirinkti nuotrauką
expense-converted-from = Sumokėta { $amount } { $from } · 1 { $from } = { $rate } { $to }
project-currency = Valiuta
project-currency-hint = Visos sumos rodomos šia valiuta. Vėliau jos pakeisti negalima.
project-currency-locked = Valiuta nustatoma kuriant projektą.

update-required-title = Reikalingas atnaujinimas
update-required-body = Ši Counted versija per sena, kad galėtų susisiekti su serveriu. Atnaujink ją, kad galėtum toliau naudotis programėle.
update-required-button = Atnaujinti

notifications-label = Pranešimai
notifications-title = Pranešimai
notifications-empty = Nieko naujo
notifications-friend-request = Draugystės užklausa

friends-title = Draugai
friends-anonymous-body = Draugai saugomi su tavo paskyra. Prisijunk, kad galėtum pridėti žmonių ir kviesti juos į savo projektus nebendrindamas nuorodos.
friends-add-title = Pridėti draugą
friends-add-hint = Jis pamatys tavo užklausą prisijungęs. Nė vienas iš jūsų nesužinos, ar kitas turi paskyrą, kol užklausa nebus priimta.
friends-add-button = Pridėti
friends-add-from-project = Pridėti kaip draugą
friends-request-sent = Užklausa išsiųsta
friends-no-account-key = Prisijunk iš naujo, kad galėtum tvarkyti draugus šiame įrenginyje.
friends-incoming-title = Užklausos
friends-accept = Priimti
friends-decline = Atmesti
friends-list-title = Mano draugai
friends-list-empty = Draugų dar nėra. Pridėk ką nors el. paštu aukščiau arba iš bendro projekto.
friends-remove = Pašalinti
friends-remove-confirm-title = Pašalinti draugą
friends-remove-confirm-message = { $email } nebebus tarp jūsų draugų, o jūs – tarp jo. Bet kuris iš jūsų vėliau galės išsiųsti naują prašymą.
friends-no-key = Dar neparuošta
friends-fingerprint = Saugos kodas
friends-fingerprint-hint = Du draugai, perskaitę vienas kitam tą patį saugos kodą, žino, kad tarp jų nėra nieko - net mūsų serverio.
friends-outgoing-title = Išsiųstos
friends-outgoing-hint = Laukiama atsakymo. Pamatysi juos tarp draugų, kai jie priims.
friends-withdraw = Atšaukti
invite-friends-title = Pakviesti draugus
invite-friends-hint = Projekto raktas užšifruojamas kiekvienam draugui šiame įrenginyje. Serveris jo niekada nemato.
invite-friends-empty = Dar nėra draugų, kuriuos galėtum pakviesti.
invite-friends-button = Pakviesti
invite-sent = { $count ->
    [one] Kvietimas išsiųstas
    [few] Išsiųsti { $count } kvietimai
    [many] Išsiųsta { $count } kvietimo
   *[other] Išsiųsta { $count } kvietimų
}
invitation-badge = Kvietimas
invitation-to = Prisijungti prie „{ $name }“
invitation-to-unnamed = Prisijungti prie projekto
invitation-unreadable = Šio kvietimo negalima atidaryti šiame įrenginyje
invitation-from = Nuo { $email }
invitation-accept = Prisijungti
invitation-decline = Atmesti
