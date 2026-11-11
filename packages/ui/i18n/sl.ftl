# Slovenščina. Popolna razen pravnih besedil (legal-, terms-, privacy-), ki obstajajo le v angleščini
# in francoščini in se za vsako sporočilo posebej vrnejo na en.ftl.

### Common

loading = Nalaganje…
cancel = Prekliči
discard-title = Želiš zavreči spremembe?
discard-message = Vneseno bo izgubljeno.
discard = Zavrzi
confirm = Potrdi
retry = Poskusi znova
delete = Izbriši
back = Nazaj
language = Jezik
theme = Tema
theme-light = Svetla
theme-dark = Temna
theme-system = Sistemska
theme-hint-device = Shranjeno samo v tej napravi.
theme-hint-system = Sledi videzu te naprave.

### Navigation

nav-main = Glavna navigacija
nav-projects = Projekti
nav-charts = Statistika
nav-settings = Nastavitve

### Connectivity

offline-banner = Brez povezave
offline-pending =
    { $count ->
        [one] { $count } na čakanju
        [two] { $count } na čakanju
        [few] { $count } na čakanju
       *[other] { $count } na čakanju
    }

sync-conflict-edit = Spor: urejanje »{ $name }« ni uspelo (element izbrisan). Preskočeno.
sync-conflict-delete = Spor: brisanje »{ $name }« ni uspelo (element izbrisan). Preskočeno.
sync-conflict-other = Spor: operacija na »{ $name }« ni uspela (element izbrisan). Preskočeno.
sync-error = Napaka sinhronizacije: { $reason }

### Errors

error-network = Strežnik ni dosegljiv. Preveri internetno povezavo.
error-rate-limited = Strežnik je preobremenjen. Poskusi znova čez trenutek.
error-generic = Nekaj je šlo narobe. Poskusi znova.

error-invalid-email = Ta e-poštni naslov ni veljaven.
error-invalid-password = To geslo ni veljavno.
error-password-too-short = Geslo mora imeti vsaj 12 znakov.
error-client-outdated = Ta različica aplikacije je zastarela. Posodobi jo za prijavo.
error-invalid-link = Ta povezava ni veljavna.
error-batch-too-large = Preveč elementov naenkrat.
error-payers-required = Izberi vsaj enega plačnika.
error-debtors-required = Izberi vsaj eno osebo, ki dolguje.
error-duplicate-participant = Udeleženec se na isti strani pojavi dvakrat.
error-participant-not-in-project = Ta udeleženec ni del tega projekta.
error-too-many-participants = Preveč udeležencev za en strošek.
error-invalid-credentials = Napačen e-poštni naslov ali geslo.
error-unauthenticated = Za to dejanje se prijavi.
error-email-not-verified = Tvoj e-poštni naslov še ni potrjen.
error-project-not-found = Ta projekt ne obstaja več.
error-expense-not-found = Ta strošek ne obstaja več.
error-storage-full = Shramba je polna: ključa tega projekta ni bilo mogoče shraniti v to napravo. Shrani povezavo za deljenje.
error-user-not-found = Ta udeleženec ne obstaja več.
error-tricount-not-found = Tricount ni bil najden ali je njegov API vrnil napako.
error-too-many-members = Ta projekt je dosegel omejitev števila članov.
error-identity-taken = Tega udeleženca je že prevzel drug račun.
error-claim-proof-invalid = Ta naprava nima ključa projekta, zato ne more prevzeti udeleženca. Znova odpri povezavo za deljenje.
error-user-has-payments = Ta udeleženec ima stroške v projektu in ga ni mogoče odstraniti.
error-resend-cooldown = Pred zahtevo za novo e-pošto počakaj 60 sekund.
error-self-friend-request = Sebe ne moreš dodati kot prijatelja.
error-not-a-friend = Povabiš lahko le osebe s seznama prijateljev.
error-friend-has-no-key = Ta prijatelj še ni odprl najnovejše različice aplikacije. Prosi ga, naj se enkrat prijavi, nato poskusi znova.
error-friend-request-not-found = Ta prošnja za prijateljstvo ne obstaja več.
error-invitation-not-found = To povabilo ne obstaja več.
error-too-many-friend-requests = Zaenkrat preveč prošenj za prijateljstvo. Poskusi jutri.
error-too-many-invitations = Preveč čakajočih povabil.
error-invalid-kdf-salt = Nastavitve šifriranja niso veljavne. Posodobi aplikacijo in poskusi znova.
error-mixed-project-batch = Ti udeleženci niso vsi v istem projektu.
error-invalid-payload = Ta različica aplikacije je poslala podatke, ki jih strežnik ne sprejme. Posodobi jo in poskusi znova.
error-invalid-public-key = Tvoj ključ za šifriranje ni veljaven. Posodobi aplikacijo in poskusi znova.
error-payment-methods-stale = Tvoji plačilni podatki so bili spremenjeni v drugi napravi. Osveži in poskusi znova.

### Auth

field-email = E-pošta
field-email-placeholder = ti@primer.si
field-password = Geslo
field-name = Ime
field-name-placeholder = Ana Novak

login-title = Prijava
login-submit = Prijavi se
login-submitting = Prijavljanje…
login-password-placeholder = Tvoje geslo
login-no-account = Še nimaš računa?
login-unverified = Tvoj e-poštni naslov še ni potrjen. Preveri nabiralnik ali znova pošlji povezavo.
login-maybe-unverified = Nov račun? Najprej potrdi naslov s povezavo, ki smo ti jo poslali, ali jo pošlji znova.
login-resend = Znova pošlji povezavo za potrditev
login-resending = Pošiljanje…
login-resend-sent = E-pošta poslana - preveri nabiralnik.

register-submit = Ustvari račun
register-submitting = Ustvarjanje…
register-have-account = Že imaš račun?
register-password-placeholder = Vsaj 12 znakov
register-password-warning = Zapiši si geslo: nikoli ga ne vidimo in ti ga ne moremo vrniti.
register-check-email-title = Preveri e-pošto
register-email-sent = E-pošta poslana
register-email-sent-hint = Klikni povezavo v nabiralniku, da aktiviraš račun.
register-not-received-prefix = Je nisi prejel? Preveri mapo z neželeno pošto ali se
register-sign-in-link = prijavi
register-not-received-suffix = za ponovno pošiljanje povezave.
register-terms-prefix = Z ustvarjanjem računa sprejemaš naše
register-terms-link = pogoje uporabe
register-terms-and = in našo
register-privacy-link = politiko zasebnosti

settings-title = Nastavitve
settings-preferences = Nastavitve
settings-preferences-local = Shranjeno na tej napravi.
settings-preferences-synced = Sinhronizirano s tvojim računom, šifrirano.
settings-anonymous-title = Nisi prijavljen
settings-upsell-title = Tvoji projekti na vsaki napravi
settings-upsell-free = Brezplačno

account-logout = Odjava
account-logging-out = Odjavljanje…
account-delete-title = Izbriši moj račun
account-delete-confirm-title = Izbriši račun
account-delete-confirm-message = Tvoj račun, seje in seznam projektov bodo trajno izbrisani. Brez gesla postanejo šifrirani podatki deljenega projekta zate neberljivi - tega ni mogoče razveljaviti.
erase-device-title = Izbrisati to napravo?
erase-device-body = Projekti, njihovi ključi in predpomnilnik na tej napravi bodo izbrisani, nato bo seja končana. Tvoji projekti ostanejo v tvojem računu: znova se prijavi, da jih dobiš nazaj.
erase-device-checking = Preverjam, ali so tvoji projekti v tvojem računu…
erase-device-pending = Spremembe, ki čakajo na pošiljanje: { $count }. Znova se poveži z internetom in poskusi znova.
erase-device-unreachable = Ni bilo mogoče preveriti, ali so tvoji projekti v tvojem računu. Preveri povezavo z internetom in poskusi znova.
erase-device-at-risk = Ti projekti niso v tvojem računu in bodo izgubljeni. Če jih želiš obdržati, prekliči in najprej kopiraj njihovo povezavo:
erase-device-confirm = Izbriši in se odjavi
erase-device-confirm-anyway = Vseeno izbriši
erase-device-erasing = Brisanje…

settings-payment-methods = Podatki za plačilo
payment-method-kind = Način
payment-method-kind-other = Drugo
payment-method-label = Ime
payment-method-label-placeholder = Glavni račun
payment-method-value = Podatki
payment-method-value-placeholder = IBAN, telefonska številka, uporabniško ime…
payment-method-add = Dodaj
payment-method-deleted = Plačilni način izbrisan.
payment-method-value-required = Izpolni podatke vsakega plačilnega načina ali ga odstrani.
payment-method-label-required = Poimenuj svoj način po meri.
payment-method-too-long = To je predolgo - skrajšaj.
payment-method-invalid-characters = Odstrani prelome vrstic ali nevidne znake.
payment-method-limit = Shraniš lahko do { $max } plačilnih načinov.
payment-methods-saved = Podatki za plačilo shranjeni.
payment-methods-offline = Za shranjevanje podatkov za plačilo moraš biti povezan.
payment-methods-stale = Tvoji podatki za plačilo so bili spremenjeni na drugi napravi. Znova so naloženi - poskusi znova.
payment-methods-key-missing = Za upravljanje podatkov za plačilo se znova prijavi.
payment-method-copy = Kopiraj { $name }
payment-method-copied = Kopirano.
payment-method-copy-failed = Kopiranje ni uspelo - označi besedilo in ga kopiraj ročno.

verify-email-checking = Preverjanje e-poštnega naslova…
verify-email-welcome = E-pošta potrjena - dobrodošel v Counted!
verify-email-back-to-login = Nazaj na prijavo
verify-email-intro = Vnesi svoje geslo in potrdi, da je ta naslov tvoj.
verify-email-submit = Potrdi moj naslov

### Project status

project-archive = Arhiviraj
project-sheet-invite = Povabi
project-sheet-recurring = Ponavljajoči
project-sheet-edit = Uredi projekt
project-sheet-close = Zapri projekt
project-sheet-archive = Arhiviraj projekt
project-sheet-reopen = Znova odpri projekt
project-sheet-unarchive = Vrni projekt iz arhiva
project-sheet-leave = Zapusti projekt

### Dates

date-long = { $day }. { $month } { $year }

month-1 = januar
month-2 = februar
month-3 = marec
month-4 = april
month-5 = maj
month-6 = junij
month-7 = julij
month-8 = avgust
month-9 = september
month-10 = oktober
month-11 = november
month-12 = december

month-short-1 = jan
month-short-2 = feb
month-short-3 = mar
month-short-4 = apr
month-short-5 = maj
month-short-6 = jun
month-short-7 = jul
month-short-8 = avg
month-short-9 = sep
month-short-10 = okt
month-short-11 = nov
month-short-12 = dec

### Actions

add = Dodaj
create = Ustvari
creating = Ustvarjanje…
edit = Uredi
leave = Zapusti
close = Zapri
paste = Prilepi
join = Pridruži se
import = Uvozi
importing = Uvažanje…
field-description = Opis
field-date = Datum
date-today = Danes
date-yesterday = Včeraj
field-optional = Neobvezno

### Projects

projects-filter-active = Aktivni
projects-filter-all = Vsi
projects-count-label = Projekti
projects-empty = Ni projektov
projects-offline-banner = Podatki brez povezave - za osvežitev se znova poveži.
demo-banner = Predstavitveni projekt - samo za branje.
demo-start-own = Ustvari svoj projekt
demo-badge = Demo
demo-bar = Tvoje spremembe izginejo, ko zapreš ta zavihek.
demo-leave = Začni zares
demo-preparing = Pripravljamo tvoj demo…
demo-failed = Demota ni bilo mogoče pripraviti.
demo-sign-in-blocked = Prijava je v demu izklopljena. Zapusti ga, da se prijaviš ali ustvariš račun; tvoji projekti ostanejo nedotaknjeni.
projects-no-local-data = Ni lokalnih podatkov
projects-no-local-data-hint = Prijavi se, da prvič naložiš svoje projekte.
projects-add = Dodaj projekt
projects-create = Ustvari projekt
projects-personal-expense = Osebni strošek
personal-expense-banner = Dodano v Moje stroške
personal-create-title = Ustvari osebni projekt
personal-create-hint = Spremljaj, kaj zapraviš sam
group-create-title = Ustvari skupni projekt
group-create-hint = Deli stroške s prijatelji in družino
personal-create-action = Ustvari
personal-create-signup = Ustvari račun
personal-sheet-title = Tvoj osebni projekt
personal-sheet-intro = Za beleženje tega, kar zapraviš sam. Prikaže se s tvojimi skupinami v Statistiki.
personal-sheet-point-alone = En sam član: ti, z imenom tvojega računa. Brez povezave za deljenje.
personal-sheet-point-encrypted = Šifriran na tvojih napravah, kot drugi projekti.
personal-sheet-point-stats = Tvoji deleži v skupinah se v Statistiki prištejejo tem stroškom.
personal-default-name = Moji stroški
my-spending-prev-month = Prejšnji mesec
my-spending-next-month = Naslednji mesec
my-spending-spent = Porabil si
my-spending-groups = Skupine
my-spending-income = Prihodki
my-spending-balance = Saldo meseca
my-spending-unconverted = Deleži v drugi valuti ostanejo ob strani, dokler tečaj ni znan.
my-spending-empty = Ta mesec nič
my-spending-empty-hint = Dodaj strošek s +. Tvoji deleži v skupinah se bodo tu prikazali samodejno.
my-spending-share-of = od { $amount }
my-spending-uncounted = ni upoštevano
payer-you = ti
expense-received-by = prejel(a)
counted-switch = Upoštevaj v Moji stroški
counted-switch-on = Vklopljeno: tvoj delež se prišteje k Moji stroški
counted-switch-off = Izklopljeno: tvoj delež se v Moji stroški ne prikaže
counted-groups = Upoštevane skupine
counted-groups-intro = Tvoj delež v teh skupinah se prišteje k Moji stroški in k proračunom.
counted-groups-converted = { $from }, pretvorjeno v { $to }
counted-groups-encrypted = Ta izbira je šifrirana s tvojim računom in te spremlja na tvojih napravah.
counted-groups-empty = Na tej napravi še ni skupine.
counted-groups-done = V redu
counted-save-failed = Te izbire ni bilo mogoče shraniti. Poskusi znova.
tab-budget = Proračun
budget-title = Proračuni
budget-manage = Upravljaj
budget-left = Ostane { $amount }
budget-over = Preseženo za { $amount }
budget-empty = Ni proračuna
budget-empty-hint = Določi zgornjo mejo za vsako kategorijo. Tvoji deleži v skupinah se tudi štejejo.
budget-define = Določi proračun
budget-sheet-title = Mesečni proračuni
budget-sheet-intro = Na tvoj delež: osebno in skupine. Pusti prazno, da kategorije ne spremljaš.
budget-average = Povpr. zadnji 3 meseci: { $amount }
budget-for = Proračun { $category }
budget-alert = Opozori me pri 80 %
budget-alert-hint = Izračunano na tej napravi
budget-invalid = Neveljaven znesek za { $category }.
budget-offline = Proračune je mogoče shraniti le s povezavo. Znova se poveži in poskusi še enkrat.
budget-alert-near = { $category }: { $percent } % mesečnega proračuna
budget-alert-over = { $category }: mesečni proračun presežen
my-spending-search-summary =
    { $count ->
        [one] { $count } zadetek
        [two] { $count } zadetka
        [few] { $count } zadetki
       *[other] { $count } zadetkov
    } · { $total } · osebno in skupine, vsi meseci
projects-join = Pridruži se projektu
projects-import-tricount = Uvozi iz Tricounta
project-actions = Dejanja projekta

status-ongoing = V teku
status-closed = Zaključen
status-archived = Arhiviran

nav-help = Pomoč
nav-privacy = Politika zasebnosti
nav-terms = Pogoji uporabe
nav-legal = Pravno obvestilo

leave-project-title = Zapustiti projekt?
leave-project-message = Izgubil boš dostop s te naprave. Če ne ostane noben član, se projekt in vsi njegovi stroški trajno izbrišejo.

add-project-title = Nov projekt
add-project-name-label = Ime projekta
add-project-name-placeholder = Moje potovanje, Sostanovalci 2024…
add-project-participants = Udeleženci
add-project-participant-name = Ime udeleženca
add-project-participant-placeholder = Clark Kent
add-project-offline = Brez povezave ni mogoče ustvariti projekta. Znova se poveži in poskusi znova.
add-project-name-required = Projekt potrebuje ime.

join-link-label = Povezava za deljenje
join-link-hint = Povezava vsebuje ključ za dešifriranje - kopiraj jo v celoti.
join-invalid-link = Ta povezava ni veljavna. Prilepi celotno povezavo za deljenje, vključno z delom za #.
join-wrong-project = Ta povezava je za drug projekt.

import-tricount-link-label = Povezava ali ključ Tricount
import-tricount-key-required = Vnesi povezavo ali ključ Tricount.
import-tricount-encryption-failed = Šifriranje ni uspelo.
import-tricount-unimportable = Nič ni bilo uvoženo: ta Tricount ima člane z računom Tricount ali zneske, ki se ne ujemajo (prizadeti vnosi: { $count }).

### Expenses

save = Shrani
saving = Shranjevanje…
adding = Dodajanje…
link-copied = Povezava kopirana
missing-encryption-key = Manjka šifrirni ključ.
missing-encryption-key-title = Manjka šifrirni ključ
missing-encryption-key-hint = Uporabljena povezava ne vsebuje ključa za dešifriranje tega projekta. Uporabi celotno povezavo, ki jo je delil ustvarjalec.
project-locked-hint = Ta naprava nima ključa tega projekta. Odpri njegovo povezavo za deljenje, da ga odkleneš.
expenses-unreadable = Nekaterih vnosov na tej napravi ni mogoče prebrati in niso všteti v vsote. Posodobi aplikacijo, da jih vidiš.
project-unlock = Odkleni
project-no-local-data-hint = Prijavi se, da prvič naložiš podatke tega projekta.
project-gone-title = Ta projekt ne obstaja več
project-gone-hint = Izbrisan je bil, ko ga je zapustil zadnji član. Povezava za deljenje ne deluje več, tudi če jo znova odpreš.

expense-add = Dodaj strošek
transfer-add = Dodaj nakazilo
expense-edit-title = Uredi strošek
expense-category = Kategorija
expense-category-auto = Samodejno · { $emoji }
emoji-picker-label = Emoji
emoji-none = Brez emojija
expense-currency = Valuta zneska
amount-op-add = Plus
amount-op-subtract = Minus
amount-op-multiply = Krat
amount-op-divide = Deljeno
amount-op-equals = Enako
amount-op-done = Končano
expense-rate = Menjalni tečaj (neobvezno)
expense-rate-hint = Pusti prazno za tečaj Evropske komisije (InforEuro) za { $month }: 1 { $from } = { $rate } { $to }.
expense-rate-invalid = Vnesi menjalni tečaj, večji od 0.
expense-rate-unavailable = Samodejni tečaj ni na voljo - vnesi ga ročno.
expense-delete-title = Izbriši strošek
expense-delete-message = »{ $name }« bo trajno izbrisan. Tega ni mogoče razveljaviti.
expense-inconsistent-amounts = Zneski se ne ujemajo
expenses-empty = Ni stroškov
expenses-empty-hint = Začni z dodajanjem stroškov z gumbom spodaj
expenses-show-more = Prikaži več (še { $count })

expense-type-expense = Strošek
expense-type-transfer = Nakazilo
expense-type-gain = Prejemek
expense-type-group = Vrsta
expense-paid-by = plačal(a)
expense-sent-by = poslal(a)
expense-contributed-by = prispeval(a)

expense-name-required = Ime je obvezno.
expense-amount-not-positive = Znesek mora biti večji od 0.
expense-no-payer = Izberi vsaj enega plačnika.
expense-no-debtor = Izberi vsaj eno osebo, ki dolguje.
expense-invalid-date = Ta datum ni veljaven.
expense-payers-mismatch = Vsota plačnikov je { $sum }, kar se ne ujema z zneskom stroška ({ $total }).
expense-debtors-mismatch = Vsota dolžnikov je { $sum }, kar se ne ujema z zneskom stroška ({ $total }).

participants-none = Nihče
participants-everyone = Vsi ({ $count })
participants-some = { $count } od { $total }
participants-select-all = Izberi vse
participants-by-shares = Po deležih
split-amounts = Zneski
participants-remaining = Ostane { $amount }
participants-over-by = { $amount } preveč
participants-who-paid = Kdo je plačal?
participants-who-received = Kdo je prejel?
participants-who-transfers = Kdo nakazuje?
participants-who-receives = Kdo prejme?
participants-for-whom = Za koga?

stats-total-expenses = Skupni stroški
stats-my-expenses = Moji stroški

tab-expenses = Stroški
tab-balance = Stanje
tab-reimbursements = Poravnava
balance-gets-back = Dobi nazaj
balance-owes = Dolguje
balance-settled = Poravnano
reimbursements-empty-title = Vse je poravnano!
reimbursements-empty-hint = Predlogi za poravnavo se prikažejo tukaj, ko računi niso uravnoteženi
reimbursement-owes = { $debtor } dolguje { $creditor }
reimbursement-record = Poravnaj
settlement-recorded = Poravnava zabeležena.
reimbursement-pay-with = Plačaj
reimbursement-pay-shared-by = Delil { $name } - pred pošiljanjem preveri ime prejemnika, ki ga prikaže tvoja aplikacija.
reimbursement-pay-title = Plačaj { $name }
reimbursements-mine-title = Dolguješ
reimbursements-others-title = Druga povračila
copy = Kopiraj

user-selection-title = Kateri udeleženec si?
user-selection-hint = Izberi svoje ime s seznama.
user-selection-required = Izberi udeleženca.
identity-claimed = Povezano z računom
identity-claimed-by = Račun { $name }
identity-taken-repick = Drug račun je prevzel udeleženca, ki si ga uporabljal. Izberi drugega.
participant-gone-repick = Udeleženec, ki si ga uporabljal, je bil odstranjen iz tega projekta. Izberi drugega.

edit-project-title = Uredi projekt
edit-project-new-badge = novo
edit-project-deferred-new-members = dodajanje novih članov
edit-project-deferred-removals = odstranjevanje članov
edit-project-offline-deferred = Brez povezave: { $items } bo uveljavljeno ob ponovni povezavi.

export-failed = Izvoz ni uspel: { $reason }

history-expense-added = Strošek dodan: { $name }
history-expense-edited = Strošek urejen: { $name }
history-expense-deleted = Strošek izbrisan: { $name }
move-expense = Premakni v projekt
move-expense-title = Premakni v…
move-current = Trenutni
move-converted = { $currency }, preračunano po mesečnem tečaju
move-note-personal = V naslednjem koraku boš izbral, kdo je plačal in za koga. Strošek bo zapustil Moje stroške.
move-note-group = Strošek bo zapustil { $project } za vse člane, stanje skupine pa bo preračunano. V naslednjem koraku boš izbral, kdo je plačal in za koga.
move-banner = Iz { $from } v { $to }
move-submit = Premakni
move-done = Strošek premaknjen v { $project }
history-expense-moved = Strošek premaknjen v drug projekt: { $name }
personal-badge = Osebno
move-members =
    { $count ->
        [one] { $count } član
        [two] { $count } člana
        [few] { $count } člani
       *[other] { $count } članov
    }
history-project-edited = Projekt urejen: { $name }
history-name-changed = Ime: »{ $from }« → »{ $to }«
history-description-added = Opis dodan: »{ $value }«
history-description-removed = Opis odstranjen: »{ $value }«
history-description-changed = Opis: »{ $from }« → »{ $to }«

### Sweep

field-amount = Znesek
expense-name-placeholder = Restavracija, nakupi…
expense-actions = Dejanja stroška
expense-your-share = Tvoj delež
expense-your-share-value = Tvoj delež: { $amount } { $currency }
expense-share-percent = { $percent } %
expense-added-by = Dodano: { $name }, { $date }
expense-inconsistent-detail = Zneski se ne ujemajo: { $paid } plačano, { $owed } dolgovano, za strošek { $total }. Uredi strošek, da to popraviš.
missing-access-key = Manjka dostopni ključ. Odpri ta projekt prek njegove povezave za deljenje.
filter-all = Vse
filter-my-payments = Moja plačila
filter-my-debts = Kaj dolgujem
expense-filter-group = Filter
expenses-search = Iskanje stroškov
expenses-no-match = Noben strošek se ne ujema.
expenses-search-open = Iskanje
expenses-search-clear = Počisti iskanje
expenses-search-summary =
    { $count ->
        [one] { $count } zadetek
        [two] { $count } zadetka
        [few] { $count } zadetki
       *[other] { $count } zadetkov
    } · { $total } · vsi meseci
participants-shares-for = Deleži za { $name }
participants-amount-for = Znesek za { $name }
reimbursement-add = Dodaj poravnavo
project-forget = Odstrani z mojega seznama
project-history-title = Zgodovina
history-kind-add = Dodano
history-kind-delete = Izbrisano
history-kind-edit = Urejeno
export = Izvozi
export-json = Izvozi JSON
export-csv = Izvozi CSV
share-link = Deli
copy-link-failed = Povezave ni bilo mogoče kopirati
open-in-app = Odpri v aplikaciji
not-found-title = Stran ni najdena
not-found-back = Nazaj na projekte

### Charts

charts-period = Obdobje
period-all = Vse
period-month = Mesec
period-3months = 3 mes.
period-year = Leto
period-custom = Po meri
charts-tab-categories = Kategorije
charts-tab-trends = Trendi
charts-total-spent = Skupaj porabljeno
charts-avg-per-person = Povp. na osebo
charts-expense-count =
    { $count ->
        [one] { $count } strošek
        [two] { $count } stroška
        [few] { $count } stroški
       *[other] { $count } stroškov
    }
charts-nothing-to-show = Ničesar za prikaz
charts-my-share-skipped =
    { $count ->
        [one] 1 projekt ni upoštevan - udeleženec ni izbran ali se njegovi podatki niso naložili.
        [two] { $count } projekta nista upoštevana - udeleženec ni izbran ali se njuni podatki niso naložili.
        [few] { $count } projekti niso upoštevani - udeleženec ni izbran ali se njihovi podatki niso naložili.
       *[other] { $count } projektov ni upoštevanih - udeleženec ni izbran ali se njihovi podatki niso naložili.
    }

### Categories

category-food = Hrana
category-transport = Prevoz
category-accommodation = Nastanitev
category-leisure = Prosti čas
category-shopping = Nakupovanje
category-services = Storitve
category-parties-gifts = Zabave in darila
category-other = Drugo
categories-manage = Upravljaj kategorije
categories-title = Kategorije
categories-yours = Tvoje kategorije
categories-builtin = Privzete
categories-add = Dodaj kategorijo
categories-in-parent = v kategoriji { $parent }
categories-renamed-from = preimenovana · prvotno { $name }
categories-hidden-hint = skrita · njeni stroški gredo v kategorijo { $other }
categories-always-shown = vedno prikazana
categories-hidden = Skrita
categories-new = Nova kategorija
categories-edit = Uredi kategorijo
categories-parent = Šteje se v kategorijo
categories-parent-hint = V statistiki vseh tvojih projektov se { $name } šteje v kategorijo { $parent }.
categories-create = Ustvari
categories-hide = Skrij to kategorijo
categories-hide-hint = Njeni stroški se vsem članom prikažejo v kategoriji { $other }.
categories-delete-title = Izbrišem „{ $name }“?
categories-delete-body = Zadevni stroški ({ $count }) se vrnejo v kategorijo { $parent }. Noben strošek ni izbrisan.
categories-delete-body-unknown = Njeni stroški se vrnejo v kategorijo { $parent }. Noben strošek ni izbrisan.
categories-delete-warning = Kategorija izgine za vse člane projekta.
categories-summary-created = Ustvarjene: { $count }
categories-summary-renamed = Preimenovane: { $count }
categories-summary-hidden = Skrite: { $count }
categories-summary-default = Ni lastnih kategorij
categories-offline = Kategorije je mogoče upravljati le s povezavo. Znova se poveži in poskusi še enkrat.
categories-error-too-many = Projekt ima največ 20 lastnih kategorij.
categories-error-full = Ta projekt je dosegel omejitev kategorij.
categories-error-name = Poimenuj kategorijo.
categories-error-missing = Ta kategorija se je medtem spremenila. Zapri in poskusi znova.
history-category-created = Kategorija ustvarjena: { $name }
history-category-edited = Kategorija urejena: { $name }
history-category-deleted = Kategorija izbrisana: { $name }
charts-project = Projekt
charts-all-projects = Vsi projekti
charts-date-from = Od
charts-date-to = Do
charts-total = Skupaj
charts-tab-people = Osebe
charts-tab-projects = Projekti
charts-scope = Čigavi stroški
charts-scope-group = Skupina
charts-scope-me = Jaz
charts-currency = Valuta
charts-my-share = Moj delež
charts-share-of-total = { $pct } % od { $total }
charts-i-paid = Plačal sem
charts-paid-more = { $amount } več od mojega deleža
charts-paid-less = { $amount } manj od mojega deleža
charts-paid-even = Točno moj delež
charts-part-title = Tvoj del v vsaki kategoriji
charts-part-desc = Sivo je, kar je porabila skupina, barvno, kar si porabil ti.
charts-breakdown-title = Razčlenitev po kategorijah
charts-breakdown-desc = Tapni izsek ali vrstico za prikaz stroškov.
charts-of-total = { $amount } od { $total }
charts-show-all = Pokaži vse ({ $count })
charts-show-less = Pokaži manj
charts-spend-title = Stroški skozi čas
charts-group-by = Združi po
bucket-day = Dan
bucket-week = Teden
bucket-month = Mesec
charts-avg = povp.
charts-cat-title-day = { $category }, dan za dnem
charts-cat-title-week = { $category }, teden za tednom
charts-cat-title-month = { $category }, mesec za mesecem
charts-cat-desc = Izberi kategorijo in jo spremljaj skozi čas.
charts-running-title = Skupna vsota
charts-running-desc = Od { $date }.
charts-avg-per-day = { $amount } / dan v povprečju
charts-avg-per-week = { $amount } / teden v povprečju
charts-avg-per-month = { $amount } / mesec v povprečju
charts-people-title = Kdo je nosil skupino
charts-people-desc = Kaj je kdo plačal, poleg tega, kar je porabil.
charts-paid = Plačano
charts-fair-share = Pošten delež
charts-you = (ti)
charts-net-more = plačal več
charts-net-less = plačal manj
charts-balance-title = Tvoje stanje skozi čas
charts-balance-desc = Nad črto ti skupina dolguje. Pod njo ti dolguješ skupini.
charts-owed = Dolgujejo ti
charts-owe = Dolguješ
charts-projects-title = Tvoj delež po projektih
charts-projects-desc = Vsote se vodijo po valutah.
history-empty = Ni dogodkov
history-by = { $name }
not-found-hint = Ta stran ne obstaja ali je bila premaknjena.
payers-title-paid-by = Plačal(a)
payers-title-sender = Pošiljatelj
payers-title-contributors = Prispevali
debtors-title-debtors = Dolguje
debtors-title-recipients = Prejemniki
debtors-title-beneficiaries = Upravičenci
debtors-title-for = Za
debtors-title-recipient = Prejemnik
expense-split-between = razdeljeno med { $count }
expense-people =
    { $count ->
        [one] { $count } oseba
        [two] { $count } osebi
        [few] { $count } osebe
       *[other] { $count } oseb
    }

### Welcome

welcome-title = Tvoji računi niso nikogaršnja stvar.
welcome-subtitle = Deli stroške s prijatelji.
welcome-note = Brezplačno. Brez računa. Brez oglasov.
welcome-link-title = Ena povezava in vsi sodelujejo.
welcome-link-body = Nikomur ni treba ustvariti računa.
welcome-link-account = Račun? Nikoli obvezen. Služi temu, da svoje projekte najdeš na drugi napravi, povabiš prijatelje iz aplikacije in deliš svoje podatke za plačilo.
welcome-demo-project = Vikend v Lyonu
welcome-private-title = Nihče ne more brati tvojih računov. Niti mi.
welcome-private-body = Imena, zneski, projekti: vse se šifrira na tvoji napravi. Ključ imaš samo ti.
welcome-private-names = Imena
welcome-private-amounts = Zneski
welcome-private-projects = Projekti
welcome-scan-title = Fotografiraj račun.
welcome-scan-body = Znesek, datum in kategorija se izpolnijo sami. Vse se dogaja na tvojem telefonu. Fotografija se ne shrani.
welcome-eu-title = 100 % evropsko
welcome-no-ads = Brez oglasov
welcome-no-trackers = Brez sledilnikov
welcome-step = Korak { $current } od { $total }
welcome-next = Naprej
welcome-skip = Preskoči
welcome-start = Začni
welcome-how-it-works = Kako točno deluje?

### Help

help-intro = Pogosto vprašanje? Tapni za prikaz odgovora.
help-create-project-q = Kako ustvarim projekt?
help-create-project-a = Na začetnem zaslonu tapni gumb + spodaj. Poimenuj projekt, izberi valuto in končano.
help-add-participants-q = Kako dodam udeležence?
help-add-participants-a = Odpri projekt in dodaj udeležence s seznama članov. Vsak udeleženec lahko strošek plača ali ga dolguje.
help-share-project-q = Kako delim projekt?
help-share-project-a = Deli URL projekta (tistega v naslovni vrstici). Vsak s povezavo si lahko projekt ogleda in ga ureja.
help-add-expense-q = Kako dodam strošek?
help-add-expense-a = V projektu tapni +, vnesi znesek, povej, kdo je plačal in med koga razdeliti. Izbereš lahko tudi datum, ki ni današnji.
help-types-q = Kakšna je razlika med stroškom, nakazilom in prejemkom?
help-types-expense = - nakup ene osebe, razdeljen med več oseb.
help-types-transfer = - povračilo ene osebe drugi, brez delitve.
help-types-gain = - prejeti denar (povračilo, darilo) za delitev med več oseb.
help-past-date-q = Lahko strošek datiram v preteklost?
help-past-date-a = Da, polje datuma je prosto. Čas ustvarjanja zapisa se hrani ločeno.
help-who-owes-q = Kako Counted ugotovi, kdo komu dolguje?
help-who-owes-a = Counted izračuna neto stanje vsakega udeleženca (kar je založil minus kar dolguje), nato predlaga najkrajše zaporedje nakazil, ki vse poravna.
help-minimal-transfers-q = Zakaj je število predlaganih nakazil minimalno?
help-minimal-transfers-a = Algoritem najprej poveže stanja, ki se natančno izničijo, nato obdela preostanek od največjega upnika do največjega dolžnika. Rezultat: manj nakazil za poravnavo vsega.
help-import-tricount-q = Kako uvozim projekt iz Tricounta?
help-import-tricount-a = Na začetnem zaslonu tapni gumb »+« spodaj, nato
help-import-tricount-b = Prilepi povezavo za deljenje Tricounta, ki ga želiš uvoziti.
help-encryption-q = So moji podatki šifrirani?
help-encryption-a = Da. Counted združuje dve zagotovili:
help-encryption-e2ee-term = Šifriranje od konca do konca
help-encryption-e2ee-def = - vse med tabo in strežnikom potuje šifrirano.
help-encryption-zero-term = Ničelni dostop
help-encryption-zero-def = - podatke šifriraš pred pošiljanjem, strežnik pa hrani le šifrirano besedilo. Nimamo možnosti, da bi ga prebrali.
help-encryption-see = Za podrobnosti glej
help-forgot-password-q = Kaj se zgodi, če pozabim geslo?
help-forgot-password-warning = Nihče ga ne more obnoviti namesto tebe.
help-forgot-password-a = Tvoje geslo varuje tvoje šifrirne ključe in nam ni nikoli poslano: nihče - niti mi - ne more dešifrirati tvojih projektov namesto tebe. Hrani ga na varnem, najbolje v upravitelju gesel.
help-archive-delete-q = Kako arhiviram ali izbrišem projekt?
help-archive-delete-a = Na zaslonu projekta odpri meni in izberi
help-archive-delete-b = da ga skriješ, a ohraniš. Projekt je dokončno izbrisan, ko ga zapusti zadnji član.
help-delete-account-q = Kako izbrišem svoj račun?
help-delete-account-a = Odpri Nastavitve in uporabi »Izbriši moj račun«. Takojšnje je in ga ni mogoče razveljaviti.
help-contact = Še kakšno vprašanje? Piši nam na

# Receipt scanning (mobile only)
expense-scan = Skeniraj račun
scan-in-progress = Branje računa…
scan-error-capture = Fotografije ni bilo mogoče posneti. Poskusi znova ali vnesi strošek ročno.
scan-error-unreadable = Na tem računu ni nič berljivega. Vnesi strošek ročno.
scan-check-amount = Preveri skupni znesek - ni bil jasno natisnjen.
scan-take-photo = Posnemi fotografijo
scan-choose-photo = Izberi fotografijo
expense-converted-from = Plačano { $amount } { $from } · 1 { $from } = { $rate } { $to }
project-currency = Valuta
project-currency-locked = Valuta se določi ob ustvarjanju projekta.
currency-search = Poišči valuto

update-required-title = Potrebna je posodobitev
update-required-body = Ta različica Counteda je prestara za komunikacijo s strežnikom. Posodobi jo, da nadaljuješ z uporabo aplikacije.
update-required-button = Posodobi

notifications-label = Obvestila
notifications-title = Obvestila
notifications-empty = Nič novega

friends-title = Prijatelji
friends-anonymous-body = Prijatelji so vezani na tvoj račun. Prijavi se, da dodaš osebe in jih povabiš v svoje projekte brez deljenja povezave.
friends-add-title = Dodaj prijatelja
friends-add-from-project = Dodaj kot prijatelja
friends-request-sent = Prošnja poslana
friends-no-account-key = Za upravljanje prijateljev na tej napravi se znova prijavi.
friends-incoming-title = Prošnje
friends-accept = Sprejmi
friends-decline = Zavrni
friends-list-title = Moji prijatelji
friends-remove = Odstrani
friends-remove-confirm-title = Odstrani prijatelja
friends-remove-confirm-message = { $email } ne bo več med tvojimi prijatelji, ti pa ne več med njegovimi. Kdorkoli od vaju lahko kasneje pošlje novo prošnjo.
friends-no-key = Še ni pripravljeno
friends-key-mismatch = Strežnik je za tvoj račun poslal javni ključ, ki se ne ujema s tvojim zasebnim ključem. Tvoja varnostna koda je skrita, povabila pa so v tej napravi izklopljena: morda nekdo posega v strežnik.
keypair-mismatch = Ključi, ki jih je strežnik poslal za tvoj račun, se ne ujemajo s tistimi na tej napravi. Povabila in menjave ključa so tu izklopljeni, dokler se ne ujamejo: nekdo morda posega v strežnik.
friends-outgoing-title = Poslano
friends-notify = Obvesti s sporočilom
friends-notify-message = Prehajam na Counted za naše skupne stroške. Brezplačna registracija na https://counted.fr z naslovom { $email }: tam čaka moja prošnja za prijateljstvo.
friends-notify-copied = Sporočilo kopirano
friends-notify-copy-failed = Sporočila ni bilo mogoče kopirati
guests-hint-title =
    { $count ->
        [one] { $count } udeleženec se še ni pridružil projektu
        [two] { $count } udeleženca se še nista pridružila projektu
        [few] { $count } udeleženci se še niso pridružili projektu
       *[other] { $count } udeležencev se še ni pridružilo projektu
    }
guests-hint-later = Pozneje
guests-hint-send = Povabi jih
guests-send-title = Pošlji projekt udeležencem
guests-send-friend = Povabi prijatelja iz Counteda
guests-send-friend-badge = Brez povezave
guests-send-friend-hint = Če že ima račun, dobi šifrirano povabilo; nič ne gre prek tvoje aplikacije za sporočila.
guests-send-link = Deli povezavo do projekta
guests-send-link-hint = Za tiste brez računa. Projekt odprejo z enim dotikom.
guests-send-link-warning = Kdorkoli ima to povezavo, lahko bere in ureja projekt. Tvoja aplikacija za sporočila jo vidi tudi.
invite-friends-title = Povabi prijatelje
invite-friends-hint = Ključ projekta je na tej napravi šifriran za vsakega prijatelja. Strežnik ga nikoli ne vidi.
invite-friends-empty = Še ni prijateljev za povabilo.
invite-friends-button = Povabi
invite-sent = { $count ->
    [one] Povabilo poslano
    [two] Poslani { $count } povabili
    [few] Poslana { $count } povabila
   *[other] Poslanih { $count } povabil
}
invitation-badge = Povabilo
invitation-to = Pridruži se »{ $name }«
invitation-to-unnamed = Pridruži se projektu
invitation-unreadable = Tega povabila na tej napravi ni mogoče odpreti
invitation-from = Od { $email }
invitation-accept = Pridruži se
invitation-decline = Zavrni

# Participants in the create and edit modals, and the "who are you?" picker - see
# docs/plans/friends.md §11.
participants-you-label = Tvoje ime v tem projektu
participants-you-badge = Ti
participants-others = Drugi udeleženci
participants-removed = Odstranjeni
participant-restore = Obnovi
participants-empty = Spodaj dodaj udeležence.
participants-duplicate = »{ $name }« je že na seznamu.
participants-input-label = Dodaj prijatelja ali vpiši ime
participants-input-placeholder = Prijatelj ali katero koli ime
participants-suggest-friend = Prijatelj · pridruži se kot »{ $name }«, prejme povabilo
participants-suggest-not-ready = Prijatelj · še ni pripravljen
participants-suggest-guest = Dodaj »{ $text }«
participants-suggest-guest-sub = Gost
participants-friends = Tvoji prijatelji
participants-all-friends = Vsi prijatelji
participants-login-hint = Prijavi se, da dodajaš ljudi neposredno s seznama prijateljev.
participants-invite-badge = Povabi
participants-guest-badge = Gost
participants-rename = Preimenuj: { $name }
participants-remove = Odstrani: { $name }
participants-rename-label = Novo ime
participants-rename-save = Shrani ime
participants-rename-hint = Ime, ki ga v tem projektu vidijo vsi. Povabilo gre še vedno na { $email }.
participants-invited-badge = Povabljen
participants-invited-sub = { $email } · še ni sprejeto
participants-invited-pending = Povabilo še ni sprejeto
participants-unlinked = Ni povezano z računom
add-project-create-invite = Ustvari in povabi: { $count }
edit-project-save-invite = Shrani in povabi: { $count }
edit-project-you-are = V tem projektu si { $name }
edit-project-no-identity = Še nisi izbral(a), kdo si
edit-project-switch = Zamenjaj
edit-project-choose = Izberi
invite-failed = Teh povabil ni bilo mogoče poslati: { $emails }
invite-waiting = Ti prijatelji morajo enkrat odpreti Counted, preden jih je mogoče povabiti: { $emails }
key-offer-title = Drug ključ za { $name }
key-offer-body = Ta naprava ima za ta projekt drug ključ, ki ga tukaj še nikoli ni odprl.
key-offer-choices = Uporabi ta ključ: projekt se odpre s ključem iz te povezave ali povabila. Prekliči: naprava obdrži trenutni ključ in nič se ne spremeni.
key-offer-use = Uporabi ta ključ
invite-again = Povabi znova
friend-picker-title = Dodaj prijatelje
user-selection-invited-hint = { $email } te je povabil(a) v »{ $project }«.
user-selection-suggested = Predlagano
user-selection-suggested-sub = { $email } te je dodal(a) pod tem imenom
user-selection-confirm-as = Sem { $name }
user-selection-missing = Tvojega imena ni tukaj? Prosi udeleženca, naj te doda v nastavitvah projekta.

## Ponavljajoči se stroški

repeat-label = Ponavljaj
repeat-none = Se ne ponavlja
repeat-weekly = Vsak teden
repeat-biweekly = Vsaka 2 tedna
repeat-monthly = Vsak mesec
repeat-quarterly = Vsake 3 mesece
repeat-yearly = Vsako leto
repeat-every-weeks = Vsakih { $count } tednov
repeat-every-months = Vsakih { $count } mesecev
repeat-every-years = Vsakih { $count } let
repeat-custom = Po meri…
repeat-every = Vsakih
repeat-unit-weeks = tednov
repeat-unit-months = mesecev
repeat-unit-years = let
repeat-on-weekday = na dan: { $weekday }
repeat-on-day = { $day }. v mesecu
repeat-on-day-month = { $day }. { $month }
repeat-month-end = V krajših mesecih pade na zadnji dan.
repeat-ends = Konec
repeat-ends-never = Nikoli
repeat-ends-on = Na datum
repeat-ends-after = Po
repeat-fewer = Manj
repeat-more = Več
repeat-last-on = zadnji { $date }
repeat-variable = Znesek se vsakič spremeni
repeat-variable-hint = Vsak se doda z zadnjim zneskom in označi »za potrditev«.
repeat-done = Končano
repeat-no-end = Brez konca
repeat-until = Do { $date }
repeat-occurrences = Število ponovitev: { $count }
repeat-offline = Potrebna je povezava. Sam strošek lahko vseeno dodaš.
repeat-foreign = Ponavlja se kot { $amount } { $currency }, enkrat preračunano po današnjem tečaju. Opozorjen boš, če se tečaj spremeni za več kot 5 %.
repeat-backfill = Začne se v preteklosti. Stroški, dodani zdaj: { $count }.
add-and-repeat = Dodaj in ponavljaj
weekday-1 = ponedeljek
weekday-2 = torek
weekday-3 = sreda
weekday-4 = četrtek
weekday-5 = petek
weekday-6 = sobota
weekday-7 = nedelja
recurring-title = Ponavljajoči se stroški
recurring-strip = Ponavljajoči se stroški: { $count }
recurring-next = Naslednji: { $name }, { $date }
recurring-to-confirm = Za potrditev: { $count }
recurring-per-month = Na mesec, pribl.
recurring-your-share = Tvoj delež
recurring-active = Aktivni
recurring-paused = Zaustavljeni
recurring-finished = Končani
recurring-paid-by = plača { $name }
recurring-next-on = Naslednji { $date }
recurring-progress = { $done } od { $total }
recurring-rate-badge = Tečaj spremenjen za { $percent } %
recurring-empty = Nič se še ne ponavlja. Ko dodajaš strošek, izberi »Ponavljaj«: najemnina, naročnine, računi.
recurring-next-ones = Naslednji
recurring-added-so-far = Doslej dodano
recurring-set-up-by = Ustvaril(a)
recurring-pause = Zaustavi
recurring-resume = Nadaljuj
recurring-stop = Ustavi ponavljanje
recurring-stop-title = Ustaviti »{ $name }«?
recurring-stop-message = Ne bo se več ponavljal. Že dodani stroški ostanejo.
recurring-resume-title = Nadaljevati »{ $name }«?
recurring-resume-message = Naslednji { $date }. Datumi, zamujeni med zaustavitvijo, se ne dodajo.
recurring-edit-title = Uredi ponavljajoči se strošek
recurring-edit-banner = Spremembe veljajo od { $date }. Že dodani stroški ostanejo, kot so.
recurring-next-on-label = Naslednji
recurring-next-too-early = Naslednji datum mora biti po zadnjem že dodanem strošku.
recurring-use-stop = Za konec uporabi »Ustavi ponavljanje« pri ponavljajočem se strošku.
recurring-drift = Tečaj { $currency } se je od nastanka spremenil za { $percent } %. Vsak se še vedno doda kot { $amount } { $project_currency } (1 { $currency } = { $rate }). Po današnjem tečaju bi bilo { $today_amount } { $project_currency }.
recurring-use-rate = Uporabi današnji tečaj
recurring-keep = Obdrži { $amount } { $currency }
recurring-added = Dodani ponavljajoči se stroški: { $count }
recurring-blocks-removal = { $name } še ni mogoče odstraniti: del { $rules }. Odstrani { $name } iz njih ali jih ustavi, nato znova shrani.
history-recurring-added = Samodejno dodano: { $name } ({ $date })
history-recurring-created = Ponavljajoči se strošek ustvarjen: { $name }
history-recurring-edited = Ponavljajoči se strošek urejen: { $name }
history-recurring-paused = Ponavljajoči se strošek zaustavljen: { $name }
history-recurring-resumed = Ponavljajoči se strošek nadaljevan: { $name }
history-recurring-stopped = Ponavljajoči se strošek ustavljen: { $name }
occurrence-recurring = Ponavljajoči se strošek
occurrence-auto = Samodejno dodano iz ponavljajočega se stroška.
occurrence-auto-next = Samodejno dodano iz ponavljajočega se stroška. Naslednji { $date }.
occurrence-auto-stopped = Samodejno dodano iz ponavljajočega se stroška, ki je bil medtem ustavljen.
occurrence-manage = Upravljaj
estimate-badge = Za potrditev
expense-pending = V čakanju
estimate-title = Znesek za potrditev.
estimate-body = Dodano s prejšnjim zneskom. Vnesi pravega, ko ga izveš.
estimate-confirm = Potrdi znesek
apply-to = Uporabi za
apply-this-only = Samo ta strošek
apply-and-next = Tega in naslednje
apply-and-next-hint = Ponavljajoči se strošek se spremeni od { $date }
apply-rule-failed = Strošek je shranjen, ponavljajoči se strošek pa ni bil spremenjen.
occurrence-delete-message = »{ $name }« z dne { $date } bo trajno izbrisan. Ponavljanje se nadaljuje in ta datum se ne vrne.
occurrence-delete-one = Izbriši samo tega
occurrence-delete-stop = Izbriši in ustavi ponavljanje
error-recurring-clock = Ura te naprave prehiteva. Preveri datum in čas.
error-recurring-not-found = Ta ponavljajoči se strošek ne obstaja več.
error-recurring-stale = Nekdo je medtem spremenil ta ponavljajoči se strošek. Znova je naložen: preveri ga in znova shrani.
error-too-many-recurring = Projekt že ima 50 ponavljajočih se stroškov. Ustavi enega, ki ga ne potrebuješ več, da dodaš novega.
error-user-in-recurring = Ta udeleženec je del ponavljajočega se stroška. Najprej ga odstrani iz njega ali strošek ustavi.

home-owed-to-you = Dolgujejo ti
home-you-owe = Dolguješ
home-details-show = Pokaži podrobnosti
home-details-hide = Skrij podrobnosti
home-all-settled = Vse je poravnano
home-pay = Plačaj
home-remind = Opomni
home-remind-message = Majhen opomnik: v { $project } so še neporavnana vračila.
home-row-owed = dolgujejo ti
home-row-owe = dolguješ
home-row-settled = Poravnano
home-row-quits = Pri tebi poravnano
home-row-pending =
    { $count ->
        [one] { $count } čakajoče vračilo
        [two] { $count } čakajoči vračili
        [few] { $count } čakajoča vračila
       *[other] { $count } čakajočih vračil
    }
home-totals-partial = Brez projektov brez menjalnega tečaja
error-account-key-changed = Ključ tvojega računa se je spremenil v drugi napravi. Znova se prijavi za nadaljevanje.
settings-security = Varnost
settings-change-password-hint = Tvoji projekti in podatki ostanejo nespremenjeni. Tvoje druge naprave bodo odjavljene.
change-password-title = Spremeni svoje geslo
change-password-current = Trenutno geslo
change-password-new = Novo geslo
change-password-confirm = Potrdi novo geslo
change-password-mismatch = Novi gesli se ne ujemata.
change-password-submit = Spremeni geslo
change-password-submitting = Spreminjam…
change-password-done = Geslo spremenjeno. Tvoje druge naprave so bile odjavljene.
error-rotation-incomplete = Tvoj račun se je medtem spremenil. Poskusi znova.
error-key-lifecycle-unavailable = Še ni na voljo: najprej morajo vsi imeti najnovejšo različico aplikacije.
settings-compromised-open = Zavaruj moj račun
compromised-title = Zavaruj svoj račun
compromised-body = Tvoje geslo in ključi računa se zamenjajo, vse druge naprave pa se odjavijo. Tvoji prijatelji bodo videli, da se je tvoja varnostna koda spremenila: ponovno jo primerjajte.
compromised-keeps = Oseba s tvojim geslom obdrži vse, kar je že bilo odprto: projekte, ki so bili do zdaj berljivi.
compromised-submit = Zamenjaj geslo in ključe
compromised-done = Tvoj račun je zavarovan. Prosi prijatelje, naj primerjajo tvojo novo varnostno kodo.
friends-code-matches = Koda se ujema
friends-code-trusted = Koda potrjena.
settings-recovery-key = Obnovitveni ključ
settings-recovery-key-pending = Nov ključ ga zamenja { $when }. Nisi bil ti? Prekliči s trenutnim ključem.
recovery-key-cancel-replacement = Prekliči nov ključ
recovery-key-title = Tvoj obnovitveni ključ
recovery-key-body = Zapiši si ga ali ga shrani tja, kamor lahko dostopaš le ti. Counted ga ne more znova pokazati ali obnoviti namesto tebe.
recovery-key-copy = Kopiraj
recovery-key-copied = Kopirano
recovery-key-save = Shrani kot datoteko
recovery-key-print = Natisni
recovery-key-next = Nadaljuj
recovery-key-confirm-body = Za preverjanje kopije vpiši skupino { $n } ključa.
recovery-key-confirm-wrong = Ta skupina se ne ujema. Še enkrat poglej kopijo.
recovery-key-password-body = Vpiši geslo, da ključ shraniš v račun.
recovery-key-submit = Shrani ključ
recovery-key-done = Obnovitveni ključ shranjen.
recovery-key-replacement-done = Nov ključ shranjen. Trenutnega zamenja čez 72 ur; do takrat trenutni še deluje.
recovery-key-field = Obnovitveni ključ
recovery-key-file-intro = Obnovitveni ključ Counted
recovery-key-cancel-done = Nov ključ je preklican.
compromised-new-key = Zdaj shrani nov obnovitveni ključ: starega zamenja čez 72 ur, do takrat stari še deluje.
pending-reset-title = Zahtevana ponastavitev
pending-reset-body = Začne veljati { $when }. Nisi bil ti? Prekliči s svojim geslom.
pending-reset-cancel = Prekliči ponastavitev
pending-reset-cancelled = Ponastavitev je preklicana.
login-forgot = Pozabljeno geslo?
forgot-title = Pozabljeno geslo
forgot-body = Vpiši e-poštni naslov. Če ima račun, pošljemo povezavo za izbiro novega gesla.
forgot-submit = Pošlji povezavo
forgot-sent = Če ima ta naslov račun, je povezava na poti. Poteče čez 24 ur.
reset-title = Pridobi svoj račun nazaj
reset-with-key-body = Vpiši obnovitveni ključ in novo geslo: vse se vrne, kot je bilo.
reset-submit-key = Obnovi moj račun
reset-no-key = Nimam obnovitvenega ključa
reset-without-key-title = Ponastavi brez obnovitvenega ključa
reset-without-key-body = Tvoj račun čez 72 ur začne znova z novim geslom. Projekti se vrnejo iz tvojih naprav in povezav za deljenje; nastavitve, plačilni podatki in kode prijateljev ne. Do takrat trenutno geslo deluje in lahko ponastavitev prekliče.
reset-name = Tvoje ime
reset-submit-request = Ponastavi moj račun
reset-requested = Končano. Novo geslo deluje od { $when }. Do takrat trenutno geslo deluje in lahko to prekliče.
reset-pending-already = Ponastavitev že čaka do { $when }. Nova zahteva začne 72 ur znova.
recovered-done = Račun je spet tvoj. Druge naprave so bile odjavljene.
login-reset-undoable = To geslo je v zadnjih 30 dneh zamenjala ponastavitev. Če je nisi zahteval, jo razveljavi: račun se vrne, kot je bil.
login-undo-reset = Razveljavi ponastavitev
reset-undone = Ponastavitev je razveljavljena. Račun je kot prej.
error-reset-pending = Novo geslo deluje, ko ponastavitev začne veljati, 72 ur po zahtevi.
error-account-disabled = Ta račun je bil izbrisan kmalu po ponastavitvi in čaka na izbris. Prejšnje geslo lahko ponastavitev še razveljavi.
error-recovery-key-invalid = Ta obnovitveni ključ ne odpre tega računa.
error-project-key-required = Ta projekt potrebuje svoj ključ. Znova ga odpri prek povezave za deljenje.
names-or = { $first } ali { $last }
participants-admin-badge = Skrbnik
edit-project-admins = Skrbniki: { $count }
edit-project-succession = Če v projektu ne ostane noben skrbnik, vlogo prevzame oseba z računom, ki je v projektu najdlje.
member-sheet-account = Član z računom
member-actions = Dejanja: { $name }
member-make-admin = Dodeli vlogo skrbnika
member-make-admin-hint = Dobi možnost spreminjanja ključa projekta in odstranjevanja članov.
member-made-admin = { $name } ima zdaj vlogo skrbnika.
group-security = Varnost projekta
group-security-admin = Varnost projekta · imaš vlogo skrbnika
group-rotate = Spremeni ključ projekta
group-rotate-hint = Če je povezava do projekta ušla v napačne roke: imetniki stare povezave izgubijo dostop do projekta.
group-remove = Odstrani koga iz projekta
group-remove-hint = Odvzame dostop in hkrati spremeni ključ.
group-admins-only = Samo za skrbnike.
group-admins-only-account = Samo za skrbnike, ki imajo račun.
group-ask-admin = Vprašaj skrbnika projekta: { $names }.
group-ask-admins = Vprašaj skrbnike projekta: { $names }.
group-ask-an-admin = Vprašaj enega od skrbnikov projekta.
group-with-account = Z računom
group-free = Brezplačno
group-sign-in-hint = Skrbnik projekta ({ $names }) ti nato lahko dodeli vlogo.
group-sign-in-hint-any = Skrbnik projekta ti nato lahko dodeli vlogo.
group-no-admin = Ta projekt nima skrbnika, zato njegovega ključa ni mogoče spremeniti.
group-hide = Skrij koga
group-hide-hint = Osebo skrije s seznamov, dostop pa ji ostane: povezava in ključ še vedno delujeta.
group-recreate-hint = Ta projekt nima skrbnika. Če želiš koga odstraniti, ustvari nov projekt.
group-recreate = Ustvari nov projekt
group-no-admin-yet = Ta projekt še nima skrbnika.
group-creator-hint = Samo ti lahko postaneš skrbnik tega projekta, če ustvariš račun.
group-creator-elsewhere-hint = Skrbnik projekta lahko postane le oseba, ki ga je ustvarila, če ustvari račun. Sicer ustvari nov projekt, če želiš koga odstraniti.
member-revoke-admin = Odvzemi vlogo skrbnika
member-revoke-admin-hint = Spreminjanje ključa projekta in odstranjevanje članov ne bosta več na voljo.
member-revoked-admin = { $name } nima več vloge skrbnika.
member-step-down = Odpovej se vlogi skrbnika
member-step-down-hint = Spreminjanje ključa projekta in odstranjevanje članov ti ne bosta več na voljo.
member-stepped-down = Nimaš več vloge skrbnika.
member-remove = Odstrani iz projekta
member-remove-hint = Spremeni ključ projekta: za { $name } nič novega ne bo več berljivo.
member-hide = Skrij s seznamov
error-user-not-admin = Ta član ni skrbnik.
error-creator-proof-invalid = Ta naprava ni ustvarila tega projekta.
error-project-has-admin = Ta projekt že ima skrbnika.
error-not-project-admin = To lahko storijo samo skrbniki projekta.
error-user-not-claimed = Ta oseba nima računa, zato ne more biti skrbnik.
error-project-key-rotated = Ključ projekta se je spremenil. Prosi člana za novo povezavo.
error-project-rotating = Ključ projekta se spreminja. Poskusi znova čez trenutek.
error-rotation-expired = Sprememba ključa je trajala predolgo in je bila preklicana. Začni znova.
error-rotation-rows-changed = Projekt se je med spremembo ključa spremenil. Začni znova.
project-rotated-title = Do tega projekta nimaš več dostopa
project-rotated-body = Ključ tega projekta je bil spremenjen. Za novo povezavo prosi skrbnika projekta.
project-rotated-open-link = Odpri povezavo
rotation-remove-title = Odstraniti { $name } iz projekta?
rotation-rotate-title = Spremeniti ključ projekta?
rotation-remove-body = Odstranitev člana pomeni ustvarjanje novega ključa. Novo povezavo bo treba deliti z ljudmi brez računa.
rotation-rotate-body = Kdor ima staro povezavo, izgubi dostop do projekta. Novo povezavo bo treba deliti z ljudmi brez računa.
rotation-boxed =
    { $count ->
        [0] Ključ ne bo samodejno poslan nobenemu drugemu članu
        [one] Ključ bo samodejno poslan { $count } članu
        [two] Ključ bo samodejno poslan { $count } članoma
        [few] Ključ bo samodejno poslan { $count } članom
       *[other] Ključ bo samodejno poslan { $count } članom
    }
rotation-by-link-title = Ljudje, ki jim je treba poslati novo povezavo
rotation-by-link-hint = Ti ljudje nimajo računa. Povezavo jim pošlješ takoj zatem.
rotation-names-and = { $first } in { $last }
rotation-names-and-others = { $names } in še { $count }
rotation-confirm-remove = Odstrani in spremeni ključ
rotation-confirm-rotate = Spremeni ključ
rotation-done-title = Nov ključ velja
rotation-done-removed = { $name } ni več del projekta.
rotation-done-rotated = Ključ projekta se je spremenil.
rotation-done-boxed =
    { $count ->
        [0] Ključ ni bil samodejno poslan nobenemu drugemu članu.
        [one] Ključ je bil samodejno poslan { $count } članu.
        [two] Ključ je bil samodejno poslan { $count } članoma.
        [few] Ključ je bil samodejno poslan { $count } članom.
       *[other] Ključ je bil samodejno poslan { $count } članom.
    }
rotation-new-link =
    { $count ->
        [0] Nova povezava do projekta
        [one] Nova povezava za pošiljanje · { $count } oseba brez računa
        [two] Nova povezava za pošiljanje · { $count } osebi brez računa
        [few] Nova povezava za pošiljanje · { $count } osebe brez računa
       *[other] Nova povezava za pošiljanje · { $count } oseb brez računa
    }
rotation-link-renewed = Povezava je bila uspešno obnovljena.
rotation-done = Končano
history-key-changed = Ključ projekta spremenjen
history-member-removed = { $name } ni več del projekta
project-rotated-row-hint = Ključ je bil spremenjen. Prosi za novo povezavo.

## Settings and Friends redesign (docs/plans/friends-settings-redesign.md)
settings-group-sharing = Deljenje
settings-group-help = Pomoč in informacije
settings-friends-changed = { $count ->
    [one] { $count } koda spremenjena
    [two] { $count } kodi spremenjeni
    [few] { $count } kode spremenjene
   *[other] { $count } kod spremenjenih
}
settings-recovery-key-created = Ustvarjen
settings-recovery-key-missing = Še ni ustvarjen
settings-recovery-key-replacing = Zamenjava v teku
settings-password = Geslo
settings-compromised-short = Tvoje geslo pozna še kdo?
settings-recovery-key-footnote = Brez obnovitvenega ključa pozabljeno geslo izbriše tvoje nastavitve, plačilne podatke in kode prijateljev.
erase-device-row = Izbriši to napravo
erase-device-row-hint = Odjavi te in ohrani samo jezik.
account-delete-hint = Takoj in trajno.
settings-open-source = Odprta koda, AGPL-3.0
settings-upsell-short = Brez računa tvoji projekti ostanejo na tej napravi. Z računom ti sledijo povsod, šifrirani.
settings-locked-group = Z računom
settings-locked-friends-short = Povabi brez povezave
settings-locked-payment-short = Vidijo ga tisti, ki ti dolgujejo
payment-methods-title = Plačilni načini
payment-methods-row-hint = Deljeno s prijatelji
payment-methods-intro = Deljeno s prijatelji. Šifrirano s tvojim računom.
payment-methods-footnote = Plačilni način se deli (največ 20) s člani tvojih projektov na zavihku Poravnava.
payment-method-shared-pill = Deljeno
payment-method-share-groups = Deli v svojih skupinah
payment-method-label-optional = Ime (neobvezno)
payment-method-delete-confirm-title = Izbrisati ta plačilni način?
payment-method-delete-confirm-message = { $name } ne bo več prikazan v tvojih projektih.
payment-methods-empty-title = Ni plačilnega načina
payment-methods-empty-body = Dodaj svoj IBAN ali plačilno aplikacijo: videli ga bodo tisti, ki ti dolgujejo.
payment-method-masked = { $kind }, ki se konča na { $last }
friends-to-check = Za preverjanje
friends-code-changed-row = Njegova koda se je spremenila
friends-own-code = Tvoja varnostna koda
friends-own-code-hint = Tvoji prijatelji vidijo to kodo ob tvojem naslovu. Če se ujema z njihovo, vabijo tebe in nikogar drugega.
friends-received = Prejete
friends-received-on = Prejeto { $date }
friends-pending = V čakanju
friends-from-project = Iz projekta
friends-accept-label = Sprejmi prošnjo od { $email }
friends-decline-label = Zavrni prošnjo od { $email }
friends-add-hint-short = Ta oseba bo videla tvojo prošnjo, če ima račun. Če ne, ji sporoči.
friends-send = Pošlji
friends-sent-hint = Še nima računa? Sporoči ji: sporočilo jo povabi, da se registrira s tem naslovom.
friends-done = Končano
friends-their-code = Njegova varnostna koda
friends-code-compare-hint = Preberita jo skupaj. Isto kodo mora videti na vrhu svoje strani Prijatelji.
friends-no-key-hint = Še ni pripravljeno: njegov ključ se ustvari, ko naslednjič odpre Counted. Do takrat ga ne moreš povabiti.
friends-remove-row = Odstrani iz mojih prijateljev
friends-new-code = Nova koda
friends-key-changed-hint-short = Preverita to kodo skupaj, zunaj te aplikacije. Do takrat so povabila z njim blokirana.
friends-kem-changed-title = Njegov ključ je bil zamenjan
friends-kem-changed-body = Postkvantni del njegovega ključa se je spremenil, njegova koda pa ne. Zavarovanje računa spremeni oboje, zato je morda kdo podtaknil ta ključ. Povabila z njim ostanejo blokirana.
friends-kem-changed-hint = Lahko ga odstraniš in nato pošlješ novo prošnjo: njegov ključ bo znova pripet.
friends-pending-sheet = Prošnja poslana, v čakanju.
friends-withdraw-row = Prekliči prošnjo
friends-empty-title = Še ni prijateljev
friends-empty-body = Dodaj nekoga po e-pošti, nato ga povabi v projekt, ne da bi delil povezavo.
friends-requests-empty-title = Ni prošenj
friends-requests-empty-body = Tu se prikažejo prošnje, ki jih prejmeš in pošlješ.
friends-code-hidden = Tvoja koda je skrita.
friends-code-hidden-body = Ključ, poslan za tvoj račun, se ne ujema s to napravo. Povabila so tu izklopljena.
notifications-manage-friends = Upravljaj moje prijatelje
notifications-friend-request-on = Prošnja za prijateljstvo · { $date }

### Link preview

share-preview-title = Naši stroški na Countedu
share-preview-description = Odpri povezavo in poglej stroške ter kdo komu koliko dolguje. Brez računa, šifrirano na tvoji napravi.

### Currency names

currency-EUR = Evro
currency-AED = Dirham Združenih arabskih emiratov
currency-AFN = Afgani
currency-ALL = Albanski lek
currency-AMD = Armenski dram
currency-AOA = Angolska kvanza
currency-ARS = Argentinski peso
currency-AUD = Avstralski dolar
currency-AWG = Arubski florin
currency-AZN = Azerbajdžanski manat
currency-BAM = Bosansko-hercegovska konvertibilna marka
currency-BBD = Barbadoški dolar
currency-BDT = Bangladeška taka
currency-BHD = Bahrajnski dinar
currency-BIF = Burundski frank
currency-BMD = Bermudski dolar
currency-BND = Brunejski dolar
currency-BOB = Bolivijski boliviano
currency-BRL = Brazilski real
currency-BSD = Bahamski dolar
currency-BTN = Butanski ngultrum
currency-BWP = Bocvanska pula
currency-BYN = Beloruski rubelj
currency-BZD = Belizejski dolar
currency-CAD = Kanadski dolar
currency-CDF = Kongovski frank
currency-CHF = Švicarski frank
currency-CLP = Čilski peso
currency-CNY = Kitajski juan
currency-COP = Kolumbijski peso
currency-CRC = Kostariški kolon
currency-CUP = Kubanski peso
currency-CVE = Zelenortski eskudo
currency-CZK = Češka krona
currency-DJF = Džibutski frank
currency-DKK = Danska krona
currency-DOP = Dominikanski peso
currency-DZD = Alžirski dinar
currency-EGP = Egiptovski funt
currency-ERN = Eritrejska nakfa
currency-ETB = Etiopski bir
currency-FJD = Fidžijski dolar
currency-FKP = Falklandski funt
currency-GBP = Britanski funt
currency-GEL = Gruzijski lari
currency-GHS = Ganski cedi
currency-GIP = Gibraltarski funt
currency-GMD = Gambijski dalasi
currency-GNF = Gvinejski frank
currency-GTQ = Gvatemalski kecal
currency-GYD = Gvajanski dolar
currency-HKD = Hongkonški dolar
currency-HNL = Honduraška lempira
currency-HTG = Haitijski gurd
currency-HUF = Madžarski forint
currency-IDR = Indonezijska rupija
currency-ILS = Izraelski šekel
currency-INR = Indijska rupija
currency-IQD = Iraški dinar
currency-IRR = Iranski rial
currency-ISK = Islandska krona
currency-JMD = Jamajški dolar
currency-JOD = Jordanski dinar
currency-JPY = Japonski jen
currency-KES = Kenijski šiling
currency-KGS = Kirgiški som
currency-KHR = Kamboški riel
currency-KMF = Komorski frank
currency-KPW = Severnokorejski von
currency-KRW = Južnokorejski von
currency-KWD = Kuvajtski dinar
currency-KYD = Kajmanski dolar
currency-KZT = Kazahstanski tenge
currency-LAK = Laoški kip
currency-LBP = Libanonski funt
currency-LKR = Šrilanška rupija
currency-LRD = Liberijski dolar
currency-LSL = Lesotski loti
currency-LYD = Libijski dinar
currency-MAD = Maroški dirham
currency-MDL = Moldavski lev
currency-MGA = Madagaskarski ariari
currency-MKD = Makedonski denar
currency-MMK = Mjanmarski kjat
currency-MNT = Mongolski tugrik
currency-MOP = Macajska pataka
currency-MRU = Mavretanska uguija
currency-MUR = Mavricijska rupija
currency-MVR = Maldivska rufija
currency-MWK = Malavijska kvača
currency-MXN = Mehiški peso
currency-MYR = Malezijski ringit
currency-MZN = Mozambiški metikal
currency-NAD = Namibijski dolar
currency-NGN = Nigerijska naira
currency-NIO = Nikaragovska kordova
currency-NOK = Norveška krona
currency-NPR = Nepalska rupija
currency-NZD = Novozelandski dolar
currency-OMR = Omanski rial
currency-PAB = Panamska balboa
currency-PEN = Perujski sol
currency-PGK = Kina Papue Nove Gvineje
currency-PHP = Filipinski peso
currency-PKR = Pakistanska rupija
currency-PLN = Poljski zlot
currency-PYG = Paragvajski gvarani
currency-QAR = Katarski rial
currency-RON = Romunski lev
currency-RSD = Srbski dinar
currency-RUB = Ruski rubelj
currency-RWF = Ruandski frank
currency-SAR = Saudski rial
currency-SBD = Solomonski dolar
currency-SCR = Sejšelska rupija
currency-SDG = Sudanski funt
currency-SEK = Švedska krona
currency-SGD = Singapurski dolar
currency-SHP = Funt Sv. Helene
currency-SLE = Sierraleonski leone
currency-SOS = Somalski šiling
currency-SRD = Surinamski dolar
currency-SSP = Južnosudanski funt
currency-STN = Dobra Svetega Tomaža in Princa
currency-SVC = Salvadorski kolon
currency-SYP = Sirijski funt
currency-SZL = Esvatinski lilangeni
currency-THB = Tajski baht
currency-TJS = Tadžikistanski somoni
currency-TMT = Turkmenistanski manat
currency-TND = Tunizijski dinar
currency-TOP = Tongovska paanga
currency-TRY = Turška lira
currency-TTD = Dolar Trinidada in Tobaga
currency-TWD = Novi tajvanski dolar
currency-TZS = Tanzanijski šiling
currency-UAH = Ukrajinska grivna
currency-UGX = Ugandski šiling
currency-USD = Ameriški dolar
currency-UYU = Urugvajski peso
currency-UZS = Uzbeški sum
currency-VES = Venezuelski bolivar
currency-VND = Vientnamski dong
currency-VUV = Vanuatujski vatu
currency-WST = Samoanska tala
currency-XAF = Srednjeafriški frank CFA
currency-XCD = Vzhodnokaribski dolar
currency-XCG = Caribbean Guilder
currency-XOF = Zahodnoafriški frank CFA
currency-XPF = Frank CFP
currency-YER = Jemenski rial
currency-ZAR = Južnoafriški rand
currency-ZMW = Zambijska kvača
currency-ZWG = Zimbabwe Gold
