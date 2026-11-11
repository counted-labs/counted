# Lietuvių. Pilnas, išskyrus teisinius tekstus (legal-, terms-, privacy-), kurie yra tik anglų ir
# prancūzų kalbomis ir kiekvienam pranešimui atskirai grįžta į en.ftl.

### Common

loading = Įkeliama…
cancel = Atšaukti
discard-title = Atmesti pakeitimus?
discard-message = Tai, ką įvedei, bus prarasta.
discard = Atmesti
confirm = Patvirtinti
retry = Bandyti dar kartą
delete = Ištrinti
back = Atgal
language = Kalba
theme = Tema
theme-light = Šviesi
theme-dark = Tamsi
theme-system = Sistemos
theme-hint-device = Išsaugota tik šiame įrenginyje.
theme-hint-system = Atitinka šio įrenginio išvaizdą.

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
error-rate-limited = Serveris užimtas. Pabandyk dar kartą po akimirkos.
error-generic = Kažkas nepavyko. Bandyk dar kartą.

error-invalid-email = Šis el. pašto adresas negalioja.
error-invalid-password = Šis slaptažodis negalioja.
error-password-too-short = Slaptažodį turi sudaryti bent 12 simbolių.
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
error-storage-full = Saugykla pilna: šio projekto rakto nepavyko išsaugoti šiame įrenginyje. Išsaugok bendrinimo nuorodą.
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
login-maybe-unverified = Ką tik užsiregistravai? Pirmiausia patvirtink adresą nuoroda, kurią tau išsiuntėme, arba išsiųsk ją dar kartą.
login-resend = Išsiųsti patvirtinimo nuorodą dar kartą
login-resending = Siunčiama…
login-resend-sent = Laiškas išsiųstas - patikrink pašto dėžutę.

register-submit = Sukurti paskyrą
register-submitting = Kuriama…
register-have-account = Jau turi paskyrą?
register-password-placeholder = Bent 12 simbolių
register-password-warning = Užsirašyk slaptažodį: mes jo niekada nematome ir negalime tau jo grąžinti.
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
settings-anonymous-title = Nesi prisijungęs
settings-upsell-title = Tavo projektai visuose įrenginiuose
settings-upsell-free = Nemokama

account-logout = Atsijungti
account-logging-out = Atsijungiama…
account-delete-title = Ištrinti mano paskyrą
account-delete-confirm-title = Ištrinti paskyrą
account-delete-confirm-message = Tavo paskyra, sesijos ir projektų sąrašas bus ištrinti visam laikui. Be slaptažodžio bendro projekto užšifruoti duomenys taps tau neįskaitomi - to atšaukti negalima.
erase-device-title = Išvalyti šį įrenginį?
erase-device-body = Šio įrenginio projektai, jų raktai ir talpykla bus ištrinti, tada sesija bus baigta. Tavo projektai lieka tavo paskyroje: vėl prisijunk, kad juos atgautum.
erase-device-checking = Tikrinama, ar tavo projektai yra tavo paskyroje…
erase-device-pending = Neišsiųsti pakeitimai: { $count }. Vėl prisijunk prie interneto ir bandyk dar kartą.
erase-device-unreachable = Nepavyko patikrinti, ar tavo projektai yra tavo paskyroje. Patikrink ryšį ir bandyk dar kartą.
erase-device-at-risk = Šių projektų nėra tavo paskyroje, ir jie bus prarasti. Jei nori juos išsaugoti, atšauk ir pirmiausia nukopijuok jų nuorodas:
erase-device-confirm = Išvalyti ir atsijungti
erase-device-confirm-anyway = Vis tiek išvalyti
erase-device-erasing = Valoma…

settings-payment-methods = Mokėjimo duomenys
payment-method-kind = Būdas
payment-method-kind-other = Kita
payment-method-label = Pavadinimas
payment-method-label-placeholder = Pagrindinė sąskaita
payment-method-value = Duomenys
payment-method-value-placeholder = IBAN, telefono numeris, naudotojo vardas…
payment-method-add = Pridėti
payment-method-deleted = Mokėjimo būdas ištrintas.
payment-method-value-required = Užpildyk kiekvieno mokėjimo būdo duomenis arba jį pašalink.
payment-method-label-required = Suteik savo mokėjimo būdui pavadinimą.
payment-method-too-long = Per ilgas - sutrumpink.
payment-method-invalid-characters = Pašalink eilučių lūžius ar nematomus simbolius.
payment-method-limit = Gali išsaugoti iki { $max } mokėjimo būdų.
payment-methods-saved = Mokėjimo duomenys išsaugoti.
payment-methods-offline = Norint išsaugoti mokėjimo duomenis, reikia interneto ryšio.
payment-methods-stale = Tavo mokėjimo duomenys buvo pakeisti kitame įrenginyje. Jie įkelti iš naujo - bandyk dar kartą.
payment-methods-key-missing = Prisijunk iš naujo, kad galėtum tvarkyti mokėjimo duomenis.
payment-method-copy = Kopijuoti { $name }
payment-method-copied = Nukopijuota.
payment-method-copy-failed = Nepavyko nukopijuoti - pažymėk tekstą ir nukopijuok rankiniu būdu.

verify-email-checking = Tikrinamas el. pašto adresas…
verify-email-welcome = El. paštas patvirtintas - sveiki atvykę į Counted!
verify-email-back-to-login = Grįžti į prisijungimą
verify-email-intro = Įvesk savo slaptažodį, kad patvirtintum, jog šis adresas yra tavo.
verify-email-submit = Patvirtinti mano adresą

### Project status

project-archive = Archyvuoti
project-sheet-invite = Pakviesti
project-sheet-recurring = Pasikartojančios
project-sheet-edit = Redaguoti projektą
project-sheet-close = Uždaryti projektą
project-sheet-archive = Archyvuoti projektą
project-sheet-reopen = Atidaryti projektą iš naujo
project-sheet-unarchive = Grąžinti projektą iš archyvo
project-sheet-leave = Išeiti iš projekto

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
projects-offline-banner = Duomenys neprisijungus - prisijunk iš naujo, kad atnaujintum.
demo-banner = Demonstracinis projektas - tik skaitymui.
demo-start-own = Sukurk savo projektą
demo-badge = Demo
demo-bar = Tavo pakeitimai išnyks uždarius šį skirtuką.
demo-leave = Pradėk iš tikrųjų
demo-preparing = Ruošiama tavo demonstracinė versija…
demo-failed = Nepavyko paruošti demonstracinės versijos.
demo-sign-in-blocked = Demonstracinėje versijoje prisijungimas išjungtas. Išeik iš jos, kad prisijungtum ar susikurtum paskyrą; tavo projektai lieka nepaliesti.
projects-no-local-data = Nėra vietinių duomenų
projects-no-local-data-hint = Prisijunk, kad pirmą kartą įkeltum savo projektus.
projects-add = Pridėti projektą
projects-create = Sukurti projektą
projects-personal-expense = Asmeninė išlaida
personal-expense-banner = Pridėta prie Mano išlaidų
personal-create-title = Susikurk asmeninį projektą
personal-create-hint = Sek, ką išleidi vienas
group-create-title = Sukurk bendrą projektą
group-create-hint = Dalinkis išlaidomis su draugais ir šeima
personal-create-action = Sukurti
personal-create-signup = Sukurti paskyrą
personal-sheet-title = Tavo asmeninis projektas
personal-sheet-intro = Kad užsirašytum, ką išleidi vienas. Rodomas kartu su tavo grupėmis Statistikoje.
personal-sheet-point-alone = Vienas narys: tu, su paskyros vardu. Be bendrinimo nuorodos.
personal-sheet-point-encrypted = Šifruojamas tavo įrenginiuose, kaip ir kiti projektai.
personal-sheet-point-stats = Tavo dalys grupėse Statistikoje pridedamos prie šių išlaidų.
personal-default-name = Mano išlaidos
my-spending-prev-month = Ankstesnis mėnuo
my-spending-next-month = Kitas mėnuo
my-spending-spent = Išleidai
my-spending-groups = Grupės
my-spending-income = Pajamos
my-spending-balance = Mėnesio balansas
my-spending-unconverted = Dalys kita valiuta neįtraukiamos, kol nežinomas kursas.
my-spending-empty = Šį mėnesį nieko
my-spending-empty-hint = Pridėk išlaidą mygtuku +. Tavo dalys grupėse čia atsiras automatiškai.
my-spending-share-of = iš { $amount }
my-spending-uncounted = neįskaičiuota
payer-you = tu
expense-received-by = gavo
counted-switch = Įskaičiuoti į Mano išlaidos
counted-switch-on = Įjungta: tavo dalis pridedama prie Mano išlaidos
counted-switch-off = Išjungta: tavo dalis nerodoma Mano išlaidos
counted-groups = Įskaičiuojamos grupės
counted-groups-intro = Tavo dalis šiose grupėse pridedama prie Mano išlaidos ir biudžetų.
counted-groups-converted = { $from }, perskaičiuota į { $to }
counted-groups-encrypted = Šis pasirinkimas užšifruotas tavo paskyra ir seka tave tavo įrenginiuose.
counted-groups-empty = Šiame įrenginyje dar nėra grupių.
counted-groups-done = Gerai
counted-save-failed = Nepavyko išsaugoti šio pasirinkimo. Bandyk dar kartą.
tab-budget = Biudžetas
budget-title = Biudžetai
budget-manage = Tvarkyti
budget-left = Liko { $amount }
budget-over = Viršyta { $amount }
budget-empty = Biudžeto nėra
budget-empty-hint = Nustatyk ribą kiekvienai kategorijai. Tavo dalys grupėse taip pat skaičiuojamos.
budget-define = Nustatyti biudžetą
budget-sheet-title = Mėnesio biudžetai
budget-sheet-intro = Tavo daliai: asmeninės ir grupės. Palik tuščią, kad nesektum kategorijos.
budget-average = Vid. per paskutinius 3 mėn.: { $amount }
budget-for = Biudžetas { $category }
budget-alert = Įspėti ties 80 %
budget-alert-hint = Skaičiuojama šiame įrenginyje
budget-invalid = Neteisinga suma: { $category }.
budget-offline = Biudžetus galima išsaugoti tik prisijungus. Prisijunk ir bandyk dar kartą.
budget-alert-near = { $category }: { $percent } % mėnesio biudžeto
budget-alert-over = { $category }: mėnesio biudžetas viršytas
my-spending-search-summary =
    { $count ->
        [one] { $count } rezultatas
        [few] { $count } rezultatai
        [many] { $count } rezultato
       *[other] { $count } rezultatų
    } · { $total } · asmeninės ir grupės, visi mėnesiai
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
add-project-offline = Neprisijungus projekto sukurti negalima. Prisijunk iš naujo ir bandyk dar kartą.
add-project-name-required = Projektui reikia pavadinimo.

join-link-label = Bendrinimo nuoroda
join-link-hint = Nuorodoje yra iššifravimo raktas - nukopijuok ją visą.
join-invalid-link = Ši nuoroda negalioja. Įklijuok visą bendrinimo nuorodą, įskaitant dalį po #.
join-wrong-project = Ši nuoroda skirta kitam projektui.

import-tricount-link-label = Tricount nuoroda arba raktas
import-tricount-key-required = Įvesk Tricount nuorodą arba raktą.
import-tricount-encryption-failed = Šifravimas nepavyko.
import-tricount-unimportable = Nieko neimportuota: šiame Tricount yra narių su Tricount paskyra arba sumų, kurios nesutampa (paveikti įrašai: { $count }).

### Expenses

save = Išsaugoti
saving = Saugoma…
adding = Pridedama…
link-copied = Nuoroda nukopijuota
missing-encryption-key = Trūksta šifravimo rakto.
missing-encryption-key-title = Trūksta šifravimo rakto
missing-encryption-key-hint = Naudota nuoroda neturi rakto, reikalingo šiam projektui iššifruoti. Naudok visą nuorodą, kurią pasidalijo projekto kūrėjas.
project-locked-hint = Šis įrenginys neturi šio projekto rakto. Atidaryk jo bendrinimo nuorodą, kad atrakintum.
expenses-unreadable = Kai kurių įrašų šiame įrenginyje nepavyksta perskaityti, todėl jie neįtraukti į sumas. Atnaujink programėlę, kad juos matytum.
project-unlock = Atrakinti
project-no-local-data-hint = Prisijunk, kad pirmą kartą įkeltum šio projekto duomenis.
project-gone-title = Šio projekto nebėra
project-gone-hint = Jis buvo ištrintas, kai išėjo paskutinis narys. Bendrinimo nuoroda nebeveikia, net jei ją atidarysi iš naujo.

expense-add = Pridėti išlaidą
transfer-add = Pridėti pervedimą
expense-edit-title = Redaguoti išlaidą
expense-category = Kategorija
expense-category-auto = Auto · { $emoji }
emoji-picker-label = Jaustukas
emoji-none = Be jaustuko
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
expense-type-group = Tipas
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
balance-gets-back = Atgauna
balance-owes = Skolingas
balance-settled = Atsiskaityta
reimbursements-empty-title = Viskas atsiskaityta!
reimbursements-empty-hint = Atsiskaitymo pasiūlymai rodomi čia, kai sąskaitos nesubalansuotos
reimbursement-owes = { $debtor } skolingas { $creditor }
reimbursement-record = Atsiskaityti
settlement-recorded = Atsiskaitymas įrašytas.
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
edit-project-offline-deferred = Neprisijungta: { $items } bus pritaikyta prisijungus.

export-failed = Eksportas nepavyko: { $reason }

history-expense-added = Išlaida pridėta: { $name }
history-expense-edited = Išlaida redaguota: { $name }
history-expense-deleted = Išlaida ištrinta: { $name }
move-expense = Perkelti į projektą
move-expense-title = Perkelti į…
move-current = Dabartinis
move-converted = { $currency }, perskaičiuota mėnesio kursu
move-note-personal = Kitame žingsnyje pasirinksi, kas mokėjo ir už ką. Išlaida paliks Mano išlaidas.
move-note-group = Išlaida paliks { $project } visiems nariams, o grupės balansas bus perskaičiuotas. Kitame žingsnyje pasirinksi, kas mokėjo ir už ką.
move-banner = Iš { $from } į { $to }
move-submit = Perkelti
move-done = Išlaida perkelta į { $project }
history-expense-moved = Išlaida perkelta į kitą projektą: { $name }
personal-badge = Asmeninė
move-members =
    { $count ->
        [one] { $count } narys
        [few] { $count } nariai
        [many] { $count } nario
       *[other] { $count } narių
    }
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
expense-share-percent = { $percent } %
expense-added-by = Pridėjo { $name }, { $date }
expense-inconsistent-detail = Sumos nesutampa: sumokėta { $paid }, skolinga { $owed }, o išlaida yra { $total }. Redaguok išlaidą, kad ištaisytum.
missing-access-key = Trūksta prieigos rakto. Atidaryk šį projektą per jo bendrinimo nuorodą.
filter-all = Visos
filter-my-payments = Mano mokėjimai
filter-my-debts = Ką esu skolingas
expense-filter-group = Filtras
expenses-search = Ieškoti išlaidų
expenses-no-match = Nėra atitinkančių išlaidų.
expenses-search-open = Ieškoti
expenses-search-clear = Išvalyti paiešką
expenses-search-summary =
    { $count ->
        [one] { $count } rezultatas
        [few] { $count } rezultatai
        [many] { $count } rezultato
       *[other] { $count } rezultatų
    } · { $total } · visi mėnesiai
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
charts-my-share-skipped =
    { $count ->
        [one] { $count } projektas neįskaičiuotas - nepasirinktas dalyvis arba jo duomenys neįkelti.
        [few] { $count } projektai neįskaičiuoti - nepasirinktas dalyvis arba jų duomenys neįkelti.
        [many] { $count } projekto neįskaičiuota - nepasirinktas dalyvis arba jų duomenys neįkelti.
       *[other] { $count } projektų neįskaičiuota - nepasirinktas dalyvis arba jų duomenys neįkelti.
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
categories-manage = Tvarkyti kategorijas
categories-title = Kategorijos
categories-yours = Tavo kategorijos
categories-builtin = Numatytosios
categories-add = Pridėti kategoriją
categories-in-parent = kategorijoje { $parent }
categories-renamed-from = pervadinta · iš pradžių { $name }
categories-hidden-hint = paslėpta · jos išlaidos patenka į kategoriją { $other }
categories-always-shown = visada rodoma
categories-hidden = Paslėpta
categories-new = Nauja kategorija
categories-edit = Redaguoti kategoriją
categories-parent = Skaičiuojama kategorijoje
categories-parent-hint = Visų tavo projektų statistikoje { $name } skaičiuojama kategorijoje { $parent }.
categories-create = Sukurti
categories-hide = Paslėpti šią kategoriją
categories-hide-hint = Jos išlaidos visiems nariams rodomos kategorijoje { $other }.
categories-delete-title = Ištrinti „{ $name }“?
categories-delete-body = Susijusios išlaidos ({ $count }) grįžta į kategoriją { $parent }. Jokios išlaidos neištrinamos.
categories-delete-body-unknown = Jos išlaidos grįžta į kategoriją { $parent }. Jokios išlaidos neištrinamos.
categories-delete-warning = Kategorija išnyksta visiems projekto nariams.
categories-summary-created = Sukurta: { $count }
categories-summary-renamed = Pervadinta: { $count }
categories-summary-hidden = Paslėpta: { $count }
categories-summary-default = Nėra savų kategorijų
categories-offline = Kategorijas galima tvarkyti tik prisijungus. Prisijunk ir bandyk dar kartą.
categories-error-too-many = Projektas gali turėti ne daugiau kaip 20 savų kategorijų.
categories-error-full = Šis projektas pasiekė kategorijų ribą.
categories-error-name = Suteik kategorijai pavadinimą.
categories-error-missing = Ši kategorija per tą laiką pasikeitė. Uždaryk ir bandyk dar kartą.
history-category-created = Sukurta kategorija: { $name }
history-category-edited = Redaguota kategorija: { $name }
history-category-deleted = Ištrinta kategorija: { $name }
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
charts-paid-more = { $amount } daugiau nei mano dalis
charts-paid-less = { $amount } mažiau nei mano dalis
charts-paid-even = Lygiai mano dalis
charts-part-title = Tavo dalis kiekvienoje kategorijoje
charts-part-desc = Pilka – ką išleido grupė, spalva – ką suvartojai tu.
charts-breakdown-title = Pasiskirstymas pagal kategorijas
charts-breakdown-desc = Paliesk sektorių ar eilutę, kad pamatytum išlaidas.
charts-of-total = { $amount } iš { $total }
charts-show-all = Rodyti viską ({ $count })
charts-show-less = Rodyti mažiau
charts-spend-title = Išlaidos laike
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
charts-projects-desc = Sumos laikomos pagal valiutą.
history-empty = Įvykių nėra
history-by = { $name }
not-found-hint = Šio puslapio nėra arba jis perkeltas.
payers-title-paid-by = Sumokėjo
payers-title-sender = Siuntėjas
payers-title-contributors = Prisidėję
debtors-title-debtors = Skolingi
debtors-title-recipients = Gavėjai
debtors-title-beneficiaries = Naudos gavėjai
debtors-title-for = Kam
debtors-title-recipient = Gavėjas
expense-split-between = padalyta tarp { $count }
expense-people =
    { $count ->
        [one] { $count } asmuo
        [few] { $count } asmenys
        [many] { $count } asmens
       *[other] { $count } asmenų
    }

### Welcome

welcome-title = Tavo apskaita - ne kitų reikalas.
welcome-subtitle = Dalinkis išlaidomis su draugais.
welcome-note = Nemokama. Paskyros nereikia. Jokių reklamų.
welcome-link-title = Viena nuoroda, ir visi dalyvauja.
welcome-link-body = Niekam nereikia kurti paskyros.
welcome-link-account = Paskyra? Niekada neprivaloma. Ji skirta rasti savo projektus kitame įrenginyje, kviesti draugus iš programėlės ir dalintis savo mokėjimo duomenimis.
welcome-demo-project = Savaitgalis Lione
welcome-private-title = Niekas negali skaityti tavo sąskaitų. Net ir mes.
welcome-private-body = Vardai, sumos, projektai: viskas užšifruojama tavo įrenginyje. Raktą turi tik tu.
welcome-private-names = Vardai
welcome-private-amounts = Sumos
welcome-private-projects = Projektai
welcome-scan-title = Nufotografuok kvitą.
welcome-scan-body = Suma, data ir kategorija užpildomos pačios. Viskas vyksta tavo telefone. Nuotrauka nesaugoma.
welcome-eu-title = 100 % europietiška
welcome-no-ads = Jokių reklamų
welcome-no-trackers = Jokių sekimo įrankių
welcome-step = { $current } žingsnis iš { $total }
welcome-next = Toliau
welcome-skip = Praleisti
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
help-forgot-password-warning = Niekas negali jo atkurti už tave.
help-forgot-password-a = Tavo slaptažodis saugo tavo šifravimo raktus ir niekada mums nesiunčiamas: niekas - net mes - negali iššifruoti tavo projektų už tave. Saugok jį saugiai, geriausia slaptažodžių tvarkyklėje.
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
project-currency-locked = Valiuta nustatoma kuriant projektą.
currency-search = Ieškoti valiutos

update-required-title = Reikalingas atnaujinimas
update-required-body = Ši Counted versija per sena, kad galėtų susisiekti su serveriu. Atnaujink ją, kad galėtum toliau naudotis programėle.
update-required-button = Atnaujinti

notifications-label = Pranešimai
notifications-title = Pranešimai
notifications-empty = Nieko naujo

friends-title = Draugai
friends-anonymous-body = Draugai saugomi su tavo paskyra. Prisijunk, kad galėtum pridėti žmonių ir kviesti juos į savo projektus nebendrindamas nuorodos.
friends-add-title = Pridėti draugą
friends-add-from-project = Pridėti kaip draugą
friends-request-sent = Užklausa išsiųsta
friends-no-account-key = Prisijunk iš naujo, kad galėtum tvarkyti draugus šiame įrenginyje.
friends-incoming-title = Užklausos
friends-accept = Priimti
friends-decline = Atmesti
friends-list-title = Mano draugai
friends-remove = Pašalinti
friends-remove-confirm-title = Pašalinti draugą
friends-remove-confirm-message = { $email } nebebus tarp tavo draugų, o tu – tarp jo. Bet kuris iš jūsų vėliau galės išsiųsti naują prašymą.
friends-no-key = Dar neparuošta
friends-key-mismatch = Serveris tavo paskyrai atsiuntė viešąjį raktą, kuris neatitinka tavo privataus rakto. Tavo saugos kodas paslėptas, o kvietimai šiame įrenginyje išjungti: galbūt kažkas klastoja serverį.
keypair-mismatch = Raktai, kuriuos serveris atsiuntė tavo paskyrai, nesutampa su esančiais šiame įrenginyje. Kvietimai ir rakto keitimai čia išjungti, kol jie nesutaps: kažkas gali manipuliuoti serveriu.
friends-outgoing-title = Išsiųstos
friends-notify = Pranešti žinute
friends-notify-message = Pereinu prie Counted mūsų bendroms išlaidoms. Nemokama registracija svetainėje https://counted.fr su el. pašto adresu { $email }: ten laukia mano draugystės užklausa.
friends-notify-copied = Žinutė nukopijuota
friends-notify-copy-failed = Nepavyko nukopijuoti žinutės
guests-hint-title =
    { $count ->
        [one] { $count } dalyvis dar neprisijungė prie projekto
        [few] { $count } dalyviai dar neprisijungė prie projekto
        [many] { $count } dalyvio dar neprisijungė prie projekto
       *[other] { $count } dalyvių dar neprisijungė prie projekto
    }
guests-hint-later = Vėliau
guests-hint-send = Pakviesk juos
guests-send-title = Siųsti projektą dalyviams
guests-send-friend = Pakviesti Counted draugą
guests-send-friend-badge = Be nuorodos
guests-send-friend-hint = Jei jau turi paskyrą, gauna šifruotą kvietimą; niekas neina per tavo žinučių programėlę.
guests-send-link = Bendrinti projekto nuorodą
guests-send-link-hint = Tiems, kas neturi paskyros. Projektą atidaro vienu palietimu.
guests-send-link-warning = Bet kas, turintis šią nuorodą, gali skaityti ir keisti projektą. Tavo žinučių programėlė ją irgi mato.
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

# Participants in the create and edit modals, and the "who are you?" picker - see
# docs/plans/friends.md §11.
participants-you-label = Tavo vardas šiame projekte
participants-you-badge = Tu
participants-others = Kiti dalyviai
participants-removed = Pašalinti
participant-restore = Atkurti
participants-empty = Pridėk dalyvius žemiau.
participants-duplicate = „{ $name }“ jau yra sąraše.
participants-input-label = Pridėti draugą arba įrašyti vardą
participants-input-placeholder = Draugas arba bet koks vardas
participants-suggest-friend = Draugas · prisijungs kaip „{ $name }“, gaus kvietimą
participants-suggest-not-ready = Draugas · dar nepasiruošęs
participants-suggest-guest = Pridėti „{ $text }“
participants-suggest-guest-sub = Svečias
participants-friends = Tavo draugai
participants-all-friends = Visi draugai
participants-login-hint = Prisijunk, kad galėtum pridėti žmones tiesiai iš draugų sąrašo.
participants-invite-badge = Pakviesti
participants-guest-badge = Svečias
participants-rename = Pervadinti: { $name }
participants-remove = Pašalinti: { $name }
participants-rename-label = Naujas vardas
participants-rename-save = Išsaugoti vardą
participants-rename-hint = Vardas, kurį šiame projekte mato visi. Kvietimas vis tiek bus siunčiamas { $email }.
participants-invited-badge = Pakviestas
participants-invited-sub = { $email } · dar nepriimtas
participants-invited-pending = Kvietimas dar nepriimtas
participants-unlinked = Nesusieta su paskyra
add-project-create-invite = Sukurti ir pakviesti: { $count }
edit-project-save-invite = Išsaugoti ir pakviesti: { $count }
edit-project-you-are = Šiame projekte tu esi { $name }
edit-project-no-identity = Dar nepasirinkai, kas esi
edit-project-switch = Keisti
edit-project-choose = Pasirinkti
invite-failed = Šių kvietimų nepavyko išsiųsti: { $emails }
invite-waiting = Šie draugai turi vieną kartą atidaryti Counted, kad juos būtų galima pakviesti: { $emails }
key-offer-title = Kitas raktas projektui { $name }
key-offer-body = Šis įrenginys turi kitą šio projekto raktą, kuris čia jo niekada neatidarė.
key-offer-choices = Naudoti šį raktą: projektas atidaromas šios nuorodos ar kvietimo raktu. Atšaukti: įrenginys pasilieka dabartinį raktą ir niekas nesikeičia.
key-offer-use = Naudoti šį raktą
invite-again = Pakviesti dar kartą
friend-picker-title = Pridėti draugų
user-selection-invited-hint = { $email } pakvietė tave į „{ $project }“.
user-selection-suggested = Siūloma
user-selection-suggested-sub = { $email } pridėjo tave šiuo vardu
user-selection-confirm-as = Aš esu { $name }
user-selection-missing = Tavo vardo čia nėra? Paprašyk dalyvio pridėti tave projekto nustatymuose.

## Pasikartojančios išlaidos

repeat-label = Kartoti
repeat-none = Nesikartoja
repeat-weekly = Kas savaitę
repeat-biweekly = Kas 2 savaites
repeat-monthly = Kas mėnesį
repeat-quarterly = Kas 3 mėnesius
repeat-yearly = Kas metus
repeat-every-weeks = Kas { $count } sav.
repeat-every-months = Kas { $count } mėn.
repeat-every-years = Kas { $count } m.
repeat-custom = Pasirinktinai…
repeat-every = Kas
repeat-unit-weeks = sav.
repeat-unit-months = mėn.
repeat-unit-years = m.
repeat-on-weekday = diena: { $weekday }
repeat-on-day = { $day } mėnesio dieną
repeat-on-day-month = { $month } { $day } d.
repeat-month-end = Trumpesniais mėnesiais tenka paskutinei dienai.
repeat-ends = Baigiasi
repeat-ends-never = Niekada
repeat-ends-on = Nurodytą dieną
repeat-ends-after = Po
repeat-fewer = Mažiau
repeat-more = Daugiau
repeat-last-on = paskutinė { $date }
repeat-variable = Suma kaskart keičiasi
repeat-variable-hint = Kiekviena pridedama su paskutine suma ir pažymima „patvirtinti“.
repeat-done = Atlikta
repeat-no-end = Be pabaigos
repeat-until = Iki { $date }
repeat-occurrences = Kartų skaičius: { $count }
repeat-offline = Reikia ryšio. Pačią išlaidą vis tiek galima pridėti.
repeat-foreign = Kartojasi kaip { $amount } { $currency }, vieną kartą perskaičiuota šiandienos kursu. Gausi įspėjimą, jei kursas pasikeis daugiau nei 5 %.
repeat-backfill = Prasideda praeityje. Dabar pridėtos išlaidos: { $count }.
add-and-repeat = Pridėti ir kartoti
weekday-1 = pirmadienis
weekday-2 = antradienis
weekday-3 = trečiadienis
weekday-4 = ketvirtadienis
weekday-5 = penktadienis
weekday-6 = šeštadienis
weekday-7 = sekmadienis
recurring-title = Pasikartojančios išlaidos
recurring-strip = Pasikartojančios išlaidos: { $count }
recurring-next = Kita: { $name }, { $date }
recurring-to-confirm = Patvirtinti: { $count }
recurring-per-month = Per mėnesį, apie
recurring-your-share = Tavo dalis
recurring-active = Aktyvios
recurring-paused = Pristabdytos
recurring-finished = Baigtos
recurring-paid-by = moka { $name }
recurring-next-on = Kita { $date }
recurring-progress = { $done } iš { $total }
recurring-rate-badge = Kursas pasikeitė { $percent } %
recurring-empty = Kol kas niekas nesikartoja. Pridėdamas išlaidą pasirink „Kartoti“: nuoma, prenumeratos, sąskaitos.
recurring-next-ones = Kitos
recurring-added-so-far = Iki šiol pridėta
recurring-set-up-by = Sukūrė
recurring-pause = Pristabdyti
recurring-resume = Tęsti
recurring-stop = Nebekartoti
recurring-stop-title = Nebekartoti „{ $name }“?
recurring-stop-message = Ji nebesikartos. Jau pridėtos išlaidos lieka.
recurring-resume-title = Tęsti „{ $name }“?
recurring-resume-message = Kita { $date }. Pristabdymo metu praleistos datos nepridedamos.
recurring-edit-title = Redaguoti pasikartojančią išlaidą
recurring-edit-banner = Pakeitimai galioja nuo { $date }. Jau pridėtos išlaidos lieka nepakitusios.
recurring-next-on-label = Kita
recurring-next-too-early = Kita data turi būti po paskutinės jau pridėtos išlaidos.
recurring-use-stop = Norėdamas ją užbaigti, pasikartojančioje išlaidoje naudok „Nebekartoti“.
recurring-drift = { $currency } kursas nuo sukūrimo pasikeitė { $percent } %. Kiekviena vis dar pridedama kaip { $amount } { $project_currency } (1 { $currency } = { $rate }). Šiandienos kursu būtų { $today_amount } { $project_currency }.
recurring-use-rate = Naudoti šiandienos kursą
recurring-keep = Palikti { $amount } { $currency }
recurring-added = Pridėtos pasikartojančios išlaidos: { $count }
recurring-blocks-removal = { $name } dar negalima pašalinti: dalyvauja { $rules }. Išimk { $name } iš jų arba jas sustabdyk ir išsaugok iš naujo.
history-recurring-added = Pridėta automatiškai: { $name } ({ $date })
history-recurring-created = Pasikartojanti išlaida sukurta: { $name }
history-recurring-edited = Pasikartojanti išlaida redaguota: { $name }
history-recurring-paused = Pasikartojanti išlaida pristabdyta: { $name }
history-recurring-resumed = Pasikartojanti išlaida atnaujinta: { $name }
history-recurring-stopped = Pasikartojanti išlaida nutraukta: { $name }
occurrence-recurring = Pasikartojanti išlaida
occurrence-auto = Pridėta automatiškai iš pasikartojančios išlaidos.
occurrence-auto-next = Pridėta automatiškai iš pasikartojančios išlaidos. Kita { $date }.
occurrence-auto-stopped = Pridėta automatiškai iš pasikartojančios išlaidos, kuri jau sustabdyta.
occurrence-manage = Tvarkyti
estimate-badge = Patvirtinti
expense-pending = Laukia
estimate-title = Sumą reikia patvirtinti.
estimate-body = Pridėta su ankstesne suma. Įvesk tikrąją, kai ją sužinosi.
estimate-confirm = Patvirtinti sumą
apply-to = Taikyti
apply-this-only = Tik šiai išlaidai
apply-and-next = Šiai ir kitoms
apply-and-next-hint = Pasikartojanti išlaida keičiasi nuo { $date }
apply-rule-failed = Išlaida išsaugota, bet pasikartojanti išlaida nepakeista.
occurrence-delete-message = „{ $name }“ ({ $date }) bus ištrinta visam laikui. Kartojimas tęsiasi, o ši data nebegrįš.
occurrence-delete-one = Ištrinti tik šią
occurrence-delete-stop = Ištrinti ir nebekartoti
error-recurring-clock = Šio įrenginio laikrodis skuba. Patikrink datą ir laiką.
error-recurring-not-found = Šios pasikartojančios išlaidos nebėra.
error-recurring-stale = Kažkas tuo metu pakeitė šią pasikartojančią išlaidą. Ji įkelta iš naujo: patikrink ir išsaugok dar kartą.
error-too-many-recurring = Šiame projekte jau yra 50 pasikartojančių išlaidų. Sustabdyk nereikalingą, kad pridėtum naują.
error-user-in-recurring = Šis dalyvis įtrauktas į pasikartojančią išlaidą. Pirmiau išimk jį iš jos arba ją sustabdyk.

home-owed-to-you = Tau skolingi
home-you-owe = Tu skolingas
home-details-show = Rodyti išsamiau
home-details-hide = Slėpti išsamiau
home-all-settled = Viskas atsiskaityta
home-pay = Mokėti
home-remind = Priminti
home-remind-message = Mažas priminimas: projekte { $project } dar liko neatsiskaitytų grąžinimų.
home-row-owed = tau skolingi
home-row-owe = tu skolingas
home-row-settled = Atsiskaityta
home-row-quits = Tavo dalis atsiskaityta
home-row-pending =
    { $count ->
        [one] { $count } laukiantis grąžinimas
        [few] { $count } laukiantys grąžinimai
        [many] { $count } laukiančio grąžinimo
       *[other] { $count } laukiančių grąžinimų
    }
home-totals-partial = Be projektų be valiutos kurso
error-account-key-changed = Tavo paskyros raktas pasikeitė kitame įrenginyje. Prisijunk iš naujo, kad tęstum.
settings-security = Saugumas
settings-change-password-hint = Tavo projektai ir duomenys lieka tokie patys. Kiti tavo įrenginiai atjungiami.
change-password-title = Pakeisk savo slaptažodį
change-password-current = Dabartinis slaptažodis
change-password-new = Naujas slaptažodis
change-password-confirm = Patvirtink naują slaptažodį
change-password-mismatch = Du nauji slaptažodžiai nesutampa.
change-password-submit = Keisti slaptažodį
change-password-submitting = Keičiama…
change-password-done = Slaptažodis pakeistas. Kiti tavo įrenginiai atjungti.
error-rotation-incomplete = Tavo paskyra tuo metu pasikeitė. Bandyk dar kartą.
error-key-lifecycle-unavailable = Dar negalima: pirmiausia visiems reikia naujausios programėlės versijos.
settings-compromised-open = Apsaugoti mano paskyrą
compromised-title = Apsaugok savo paskyrą
compromised-body = Tavo slaptažodis ir paskyros raktai pakeičiami, o visi kiti įrenginiai atjungiami. Tavo draugai pamatys, kad tavo saugos kodas pasikeitė: palygink jį su jais iš naujo.
compromised-keeps = Kas turėjo tavo slaptažodį, pasilieka tai, kas jau atidaryta: projektus, kuriuos iki šiol buvo galima skaityti.
compromised-submit = Pakeisti slaptažodį ir raktus
compromised-done = Tavo paskyra apsaugota. Paprašyk draugų palyginti tavo naują saugos kodą.
friends-code-matches = Kodas sutampa
friends-code-trusted = Kodas patvirtintas.
settings-recovery-key = Atkūrimo raktas
settings-recovery-key-pending = Naujas raktas jį pakeis { $when }. Ne tu? Atšauk dabartiniu raktu.
recovery-key-cancel-replacement = Atšaukti naują raktą
recovery-key-title = Tavo atkūrimo raktas
recovery-key-body = Užsirašyk jį arba išsaugok ten, kur gali pasiekti tik tu. Counted negali jo parodyti dar kartą ar atkurti už tave.
recovery-key-copy = Kopijuoti
recovery-key-copied = Nukopijuota
recovery-key-save = Išsaugoti kaip failą
recovery-key-print = Spausdinti
recovery-key-next = Tęsti
recovery-key-confirm-body = Norėdamas patikrinti kopiją, įvesk rakto grupę { $n }.
recovery-key-confirm-wrong = Ši grupė nesutampa. Dar kartą pažiūrėk į kopiją.
recovery-key-password-body = Įvesk slaptažodį, kad išsaugotum raktą paskyroje.
recovery-key-submit = Išsaugoti raktą
recovery-key-done = Atkūrimo raktas išsaugotas.
recovery-key-replacement-done = Naujas raktas išsaugotas. Jis pakeis dabartinį po 72 valandų; iki tol dabartinis vis dar veikia.
recovery-key-field = Atkūrimo raktas
recovery-key-file-intro = Counted atkūrimo raktas
recovery-key-cancel-done = Naujas raktas atšauktas.
compromised-new-key = Išsaugok naują atkūrimo raktą dabar: jis pakeis senąjį po 72 valandų, o iki tol senasis vis dar veikia.
pending-reset-title = Paprašyta atkurti paskyrą
pending-reset-body = Jis įsigalios { $when }. Ne tu? Atšauk savo slaptažodžiu.
pending-reset-cancel = Atšaukti atkūrimą
pending-reset-cancelled = Atkūrimas atšauktas.
login-forgot = Pamiršai slaptažodį?
forgot-title = Pamirštas slaptažodis
forgot-body = Įvesk el. paštą. Jei jis turi paskyrą, atsiųsime nuorodą naujam slaptažodžiui pasirinkti.
forgot-submit = Siųsti nuorodą
forgot-sent = Jei šis adresas turi paskyrą, nuoroda jau keliauja. Ji galioja 24 valandas.
reset-title = Atgauk savo paskyrą
reset-with-key-body = Įvesk atkūrimo raktą ir naują slaptažodį: viskas grįš taip, kaip buvo.
reset-submit-key = Atkurti mano paskyrą
reset-no-key = Neturiu atkūrimo rakto
reset-without-key-title = Atkurti be atkūrimo rakto
reset-without-key-body = Tavo paskyra po 72 valandų prasidės iš naujo su nauju slaptažodžiu. Projektai grįš iš tavo įrenginių ir bendrinimo nuorodų; nustatymai, mokėjimo duomenys ir draugų kodai ne. Iki tol dabartinis slaptažodis veikia ir gali atšaukti atkūrimą.
reset-name = Tavo vardas
reset-submit-request = Atkurti mano paskyrą iš naujo
reset-requested = Atlikta. Naujas slaptažodis veiks nuo { $when }. Iki tol dabartinis slaptažodis veikia ir gali tai atšaukti.
reset-pending-already = Atkūrimas jau laukia iki { $when }. Naujas prašymas pradeda 72 valandas iš naujo.
recovered-done = Tavo paskyra atgauta. Kiti įrenginiai atjungti.
login-reset-undoable = Šį slaptažodį per pastarąsias 30 dienų pakeitė atkūrimas. Jei to neprašei, atšauk: paskyra grįš tokia, kokia buvo.
login-undo-reset = Atšaukti atkūrimą
reset-undone = Atkūrimas atšauktas. Paskyra vėl tokia, kokia buvo.
error-reset-pending = Naujas slaptažodis veiks, kai atkūrimas įsigalios, praėjus 72 valandoms nuo prašymo.
error-account-disabled = Ši paskyra ištrinta netrukus po atkūrimo ir laukia sunaikinimo. Ankstesnis slaptažodis vis dar gali atšaukti atkūrimą.
error-recovery-key-invalid = Šis atkūrimo raktas neatidaro šios paskyros.
error-project-key-required = Šiam projektui reikia jo rakto. Atidaryk jį iš naujo per bendrinimo nuorodą.
names-or = { $first } arba { $last }
participants-admin-badge = Admin
edit-project-admins = Administratoriai: { $count }
edit-project-succession = Jei projekte nebelieka administratoriaus, šį vaidmenį perima ilgiausiai projekte esantis narys su paskyra.
member-sheet-account = Narys su paskyra
member-actions = Veiksmai: { $name }
member-make-admin = Suteikti administratoriaus vaidmenį
member-make-admin-hint = Galės pakeisti projekto raktą ir ką nors pašalinti.
member-made-admin = { $name } dabar turi administratoriaus vaidmenį.
group-security = Projekto saugumas
group-security-admin = Projekto saugumas · turi administratoriaus vaidmenį
group-rotate = Pakeisti projekto raktą
group-rotate-hint = Jei projekto nuoroda buvo nutekinta: senos nuorodos turėtojai praras prieigą prie projekto.
group-remove = Pašalinti ką nors iš projekto
group-remove-hint = Atima prieigą ir tuo pačiu pakeičia raktą.
group-admins-only = Tik administratoriams.
group-admins-only-account = Tik administratoriams, kurie turi paskyrą.
group-ask-admin = Paklausk projekto administratoriaus: { $names }.
group-ask-admins = Paklausk projekto administratorių: { $names }.
group-ask-an-admin = Paklausk kurio nors projekto administratoriaus.
group-with-account = Su paskyra
group-free = Nemokamai
group-sign-in-hint = Projekto administratorius ({ $names }) vėliau galės tau suteikti šį vaidmenį.
group-sign-in-hint-any = Projekto administratorius vėliau galės tau suteikti šį vaidmenį.
group-no-admin = Šis projektas neturi administratoriaus, todėl jo rakto pakeisti negalima.
group-hide = Paslėpti ką nors
group-hide-hint = Paslepia asmenį sąrašuose, bet prieigos neatima: nuoroda ir raktas vis dar veikia.
group-recreate-hint = Šis projektas neturi administratoriaus. Jei nori ką nors pašalinti, sukurk naują projektą.
group-recreate = Sukurti naują projektą
group-no-admin-yet = Šis projektas dar neturi administratoriaus.
group-creator-hint = Šio projekto administratoriaus vaidmenį gali gauti tik tu, jei susikursi paskyrą.
group-creator-elsewhere-hint = Administratoriaus vaidmenį gali gauti tik projektą sukūręs asmuo, jei susikurs paskyrą. Kitu atveju sukurk naują projektą, jei nori ką nors pašalinti.
member-revoke-admin = Atimti administratoriaus vaidmenį
member-revoke-admin-hint = Nebegalės keisti projekto rakto nei nieko šalinti.
member-revoked-admin = { $name } nebeturi administratoriaus vaidmens.
member-step-down = Atsisakyti administratoriaus vaidmens
member-step-down-hint = Nebegalėsi keisti projekto rakto nei nieko šalinti.
member-stepped-down = Nebeturi administratoriaus vaidmens.
member-remove = Pašalinti iš projekto
member-remove-hint = Pakeičia projekto raktą: { $name } nebegalės skaityti nieko naujo.
member-hide = Paslėpti sąrašuose
error-user-not-admin = Šis narys neturi administratoriaus vaidmens.
error-creator-proof-invalid = Šis įrenginys nesukūrė šio projekto.
error-project-has-admin = Šis projektas jau turi administratorių.
error-not-project-admin = Tai gali padaryti tik projekto administratoriai.
error-user-not-claimed = Šis asmuo neturi paskyros, todėl negali gauti administratoriaus vaidmens.
error-project-key-rotated = Projekto raktas pasikeitė. Paprašyk nario naujos nuorodos.
error-project-rotating = Projekto raktas keičiamas. Bandyk dar kartą po akimirkos.
error-rotation-expired = Rakto keitimas užtruko per ilgai ir buvo atšauktas. Pradėk iš naujo.
error-rotation-rows-changed = Projektas pasikeitė keičiant raktą. Pradėk iš naujo.
project-rotated-title = Nebeturi prieigos prie šio projekto
project-rotated-body = Administratorius pakeitė šio projekto raktą. Paprašyk administratoriaus naujos nuorodos.
project-rotated-open-link = Atidaryti nuorodą
rotation-remove-title = Pašalinti { $name } iš projekto?
rotation-rotate-title = Pakeisti projekto raktą?
rotation-remove-body = Pašalinus narį, sukuriamas naujas raktas. Naują nuorodą reikės pasidalyti su žmonėmis be paskyros.
rotation-rotate-body = Kas turi seną nuorodą, praras prieigą prie projekto. Naują nuorodą reikės pasidalyti su žmonėmis be paskyros.
rotation-boxed =
    { $count ->
        [0] Joks kitas narys negaus rakto automatiškai
        [one] { $count } narys gaus raktą automatiškai
        [few] { $count } nariai gaus raktą automatiškai
        [many] { $count } nario gaus raktą automatiškai
       *[other] { $count } narių gaus raktą automatiškai
    }
rotation-by-link-title = Žmonės, kuriems turėsi išsiųsti naują nuorodą
rotation-by-link-hint = Šie žmonės neturi paskyros. Nuorodą jiems išsiųsi iškart po to.
rotation-names-and = { $first } ir { $last }
rotation-names-and-others = { $names } ir dar { $count }
rotation-confirm-remove = Pašalinti ir pakeisti raktą
rotation-confirm-rotate = Pakeisti raktą
rotation-done-title = Naujas raktas galioja
rotation-done-removed = { $name } nebedalyvauja projekte.
rotation-done-rotated = Projekto raktas pakeistas.
rotation-done-boxed =
    { $count ->
        [0] Joks kitas narys negavo rakto automatiškai.
        [one] { $count } narys gavo raktą automatiškai.
        [few] { $count } nariai gavo raktą automatiškai.
        [many] { $count } nario gavo raktą automatiškai.
       *[other] { $count } narių gavo raktą automatiškai.
    }
rotation-new-link =
    { $count ->
        [0] Nauja projekto nuoroda
        [one] Nauja nuoroda išsiųsti · { $count } žmogus be paskyros
        [few] Nauja nuoroda išsiųsti · { $count } žmonės be paskyros
        [many] Nauja nuoroda išsiųsti · { $count } žmogaus be paskyros
       *[other] Nauja nuoroda išsiųsti · { $count } žmonių be paskyros
    }
rotation-link-renewed = Nuoroda sėkmingai atnaujinta.
rotation-done = Atlikta
history-key-changed = Projekto raktas pakeistas
history-member-removed = { $name } nebedalyvauja projekte
project-rotated-row-hint = Raktas pakeistas. Paprašyk naujos nuorodos.

## Settings and Friends redesign (docs/plans/friends-settings-redesign.md)
settings-group-sharing = Bendrinimas
settings-group-help = Pagalba ir informacija
settings-friends-changed = { $count ->
    [one] { $count } kodas pasikeitė
    [few] { $count } kodai pasikeitė
    [many] { $count } kodo pasikeitė
   *[other] { $count } kodų pasikeitė
}
settings-recovery-key-created = Sukurtas
settings-recovery-key-missing = Reikia sukurti
settings-recovery-key-replacing = Laukiama pakeitimo
settings-password = Slaptažodis
settings-compromised-short = Kažkas kitas žino tavo slaptažodį?
settings-recovery-key-footnote = Be atkūrimo rakto pamirštas slaptažodis ištrina tavo nuostatas, mokėjimo duomenis ir draugų kodus.
erase-device-row = Išvalyti šį įrenginį
erase-device-row-hint = Atjungia tave ir palieka tik kalbą.
account-delete-hint = Iš karto ir visam laikui.
settings-open-source = Atvirasis kodas, AGPL-3.0
settings-upsell-short = Be paskyros tavo projektai lieka šiame įrenginyje. Su paskyra jie keliauja su tavimi visur, užšifruoti.
settings-locked-group = Su paskyra
settings-locked-friends-short = Kviesk be nuorodos
settings-locked-payment-short = Mato tie, kurie tau skolingi
payment-methods-title = Mokėjimo būdai
payment-methods-row-hint = Bendrinami su draugais
payment-methods-intro = Bendrinami su draugais. Užšifruoti tavo paskyra.
payment-methods-footnote = Mokėjimo būdas bendrinamas (iki 20) su tavo projektų nariais skirtuke Atsiskaitymas.
payment-method-shared-pill = Bendrinamas
payment-method-share-groups = Bendrinti savo grupėse
payment-method-label-optional = Pavadinimas (neprivaloma)
payment-method-delete-confirm-title = Ištrinti šį mokėjimo būdą?
payment-method-delete-confirm-message = { $name } nebebus rodomas tavo projektuose.
payment-methods-empty-title = Nėra mokėjimo būdų
payment-methods-empty-body = Pridėk savo IBAN arba mokėjimo programėlę: tie, kurie tau skolingi, ją matys.
payment-method-masked = { $kind }, baigiasi { $last }
friends-to-check = Reikia patikrinti
friends-code-changed-row = Jo kodas pasikeitė
friends-own-code = Tavo saugos kodas
friends-own-code-hint = Tavo draugai mato šį kodą šalia tavo adreso. Jei jis sutampa su jų kodu, jie kviečia tave ir nieką kitą.
friends-received = Gautos
friends-received-on = Gauta { $date }
friends-pending = Laukiama
friends-from-project = Iš projekto
friends-accept-label = Priimti { $email } užklausą
friends-decline-label = Atmesti { $email } užklausą
friends-add-hint-short = Šis žmogus pamatys tavo užklausą, jei turi paskyrą. Jei ne, pranešk jam.
friends-send = Siųsti
friends-sent-hint = Dar neturi paskyros? Pranešk jam: žinutė kviečia užsiregistruoti šiuo adresu.
friends-done = Atlikta
friends-their-code = Jo saugos kodas
friends-code-compare-hint = Perskaitykite jį kartu. Jis turi matyti tą patį kodą savo puslapio Draugai viršuje.
friends-no-key-hint = Dar neparuošta: jo raktas bus sukurtas, kai jis kitą kartą atidarys Counted. Iki tol negali jo pakviesti.
friends-remove-row = Pašalinti iš mano draugų
friends-new-code = Naujas kodas
friends-key-changed-hint-short = Patikrinkite šį kodą kartu, ne per šią programėlę. Iki tol kvietimai su juo užblokuoti.
friends-kem-changed-title = Jo raktas buvo pakeistas
friends-kem-changed-body = Pasikeitė postkvantinė jo rakto dalis, bet ne jo kodas. Paskyros apsaugojimas keičia abu, todėl kažkas galėjo pakeisti šį raktą. Kvietimai su juo lieka užblokuoti.
friends-kem-changed-hint = Gali jį pašalinti, tada išsiųsti naują užklausą: jo raktas bus vėl prisegtas.
friends-pending-sheet = Užklausa išsiųsta, laukiama.
friends-withdraw-row = Atšaukti užklausą
friends-empty-title = Draugų dar nėra
friends-empty-body = Pridėk ką nors el. paštu, tada pakviesk į projektą nesidalydamas nuoroda.
friends-requests-empty-title = Užklausų nėra
friends-requests-empty-body = Čia rodomos gautos ir išsiųstos užklausos.
friends-code-hidden = Tavo kodas paslėptas.
friends-code-hidden-body = Tavo paskyrai pateiktas raktas neatitinka šio įrenginio. Kvietimai čia išjungti.
notifications-manage-friends = Tvarkyti mano draugus
notifications-friend-request-on = Draugystės užklausa · { $date }

### Link preview

share-preview-title = Mūsų išlaidos Counted
share-preview-description = Atidaryk nuorodą ir pamatysi išlaidas bei kas kam kiek skolingas. Be paskyros, šifruojama tavo įrenginyje.

### Currency names

currency-EUR = Euras
currency-AED = Jungtinių Arabų Emyratų dirhamas
currency-AFN = Afganistano afganis
currency-ALL = Albanijos lekas
currency-AMD = Armėnijos dramas
currency-AOA = Angolos kvanza
currency-ARS = Argentinos pesas
currency-AUD = Australijos doleris
currency-AWG = Arubos guldenas
currency-AZN = Azerbaidžano manatas
currency-BAM = Bosnijos ir Hercegovinos konvertuojamoji markė
currency-BBD = Barbadoso doleris
currency-BDT = Bangladešo taka
currency-BHD = Bahreino dinaras
currency-BIF = Burundžio frankas
currency-BMD = Bermudos doleris
currency-BND = Brunėjaus doleris
currency-BOB = Bolivijos bolivijanas
currency-BRL = Brazilijos realas
currency-BSD = Bahamų doleris
currency-BTN = Butano ngultrumas
currency-BWP = Botsvanos pula
currency-BYN = Baltarusijos rublis
currency-BZD = Belizo doleris
currency-CAD = Kanados doleris
currency-CDF = Kongo frankas
currency-CHF = Šveicarijos frankas
currency-CLP = Čilės pesas
currency-CNY = Kinijos ženminbi juanis
currency-COP = Kolumbijos pesas
currency-CRC = Kosta Rikos kolonas
currency-CUP = Kubos pesas
currency-CVE = Žaliojo Kyšulio eskudas
currency-CZK = Čekijos krona
currency-DJF = Džibučio frankas
currency-DKK = Danijos krona
currency-DOP = Dominikos pesas
currency-DZD = Alžyro dinaras
currency-EGP = Egipto svaras
currency-ERN = Eritrėjos nakfa
currency-ETB = Etiopijos biras
currency-FJD = Fidžio doleris
currency-FKP = Falklando salų svaras
currency-GBP = Didžiosios Britanijos svaras
currency-GEL = Gruzijos laris
currency-GHS = Ganos sedis
currency-GIP = Gibraltaro svaras
currency-GMD = Gambijos dalasis
currency-GNF = Gvinėjos frankas
currency-GTQ = Gvatemalos ketcalis
currency-GYD = Gajanos doleris
currency-HKD = Honkongo doleris
currency-HNL = Hondūro lempira
currency-HTG = Haičio gurdas
currency-HUF = Vengrijos forintas
currency-IDR = Indonezijos rupija
currency-ILS = Izraelio naujasis šekelis
currency-INR = Indijos rupija
currency-IQD = Irako dinaras
currency-IRR = Irano rialas
currency-ISK = Islandijos krona
currency-JMD = Jamaikos doleris
currency-JOD = Jordanijos dinaras
currency-JPY = Japonijos jena
currency-KES = Kenijos šilingas
currency-KGS = Kirgizijos somas
currency-KHR = Kambodžos rielis
currency-KMF = Komoro frankas
currency-KPW = Šiaurės Korėjos vonas
currency-KRW = Pietų Korėjos vonas
currency-KWD = Kuveito dinaras
currency-KYD = Kaimanų salų doleris
currency-KZT = Kazachstano tengė
currency-LAK = Laoso kipas
currency-LBP = Libano svaras
currency-LKR = Šri Lankos rupija
currency-LRD = Liberijos doleris
currency-LSL = Lesoto lotis
currency-LYD = Libijos dinaras
currency-MAD = Maroko dirhamas
currency-MDL = Moldovos lėja
currency-MGA = Madagaskaro ariaris
currency-MKD = Makedonijos denaras
currency-MMK = Mianmaro kijatas
currency-MNT = Mongolijos tugrikas
currency-MOP = Makao pataka
currency-MRU = Mauritanijos ugija
currency-MUR = Mauricijaus rupija
currency-MVR = Maldyvų rufija
currency-MWK = Malavio kvača
currency-MXN = Meksikos pesas
currency-MYR = Malaizijos ringitas
currency-MZN = Mozambiko metikalis
currency-NAD = Namibijos doleris
currency-NGN = Nigerijos naira
currency-NIO = Nikaragvos kordoba
currency-NOK = Norvegijos krona
currency-NPR = Nepalo rupija
currency-NZD = Naujosios Zelandijos doleris
currency-OMR = Omano rialas
currency-PAB = Panamos balboja
currency-PEN = Peru solis
currency-PGK = Papua Naujosios Gvinėjos kina
currency-PHP = Filipinų pesas
currency-PKR = Pakistano rupija
currency-PLN = Lenkijos zlotas
currency-PYG = Paragvajaus guaranis
currency-QAR = Kataro rialas
currency-RON = Rumunijos lėja
currency-RSD = Serbijos dinaras
currency-RUB = Rusijos rublis
currency-RWF = Ruandos frankas
currency-SAR = Saudo Arabijos rijalas
currency-SBD = Saliamono salų doleris
currency-SCR = Seišelių rupija
currency-SDG = Sudano svaras
currency-SEK = Švedijos krona
currency-SGD = Singapūro doleris
currency-SHP = Šv. Elenos salų svaras
currency-SLE = Siera Leonės leonė
currency-SOS = Somalio šilingas
currency-SRD = Surimano doleris
currency-SSP = Pietų Sudano svaras
currency-STN = San Tomės ir Principės dobra
currency-SVC = Salvadoro kolonas
currency-SYP = Sirijos svaras
currency-SZL = Svazilando lilangenis
currency-THB = Tailando batas
currency-TJS = Tadžikistano somonis
currency-TMT = Turkmėnistano manatas
currency-TND = Tuniso dinaras
currency-TOP = Tongo paanga
currency-TRY = Turkijos lira
currency-TTD = Trinidado ir Tobago doleris
currency-TWD = Taivano naujasis doleris
currency-TZS = Tanzanijos šilingas
currency-UAH = Ukrainos grivina
currency-UGX = Ugandos šilingas
currency-USD = JAV doleris
currency-UYU = Urugvajaus pesas
currency-UZS = Uzbekistano sumas
currency-VES = Venesuelos bolivaras
currency-VND = Vietnamo dongas
currency-VUV = Vanuatu vatas
currency-WST = Samoa tala
currency-XAF = CFA BEAC frankas
currency-XCD = Rytų Karibų doleris
currency-XCG = Caribbean Guilder
currency-XOF = CFA BCEAO frankas
currency-XPF = CFP frankas
currency-YER = Jemeno rialas
currency-ZAR = Pietų Afrikos Respublikos randas
currency-ZMW = Zambijos kvača
currency-ZWG = Zimbabwe Gold
