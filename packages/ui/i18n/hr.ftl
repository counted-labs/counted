# Hrvatski. Potpun osim pravnih tekstova (legal-, terms-, privacy-), koji postoje samo na engleskom
# i francuskom i za svaku poruku zasebno padaju na en.ftl.

### Common

loading = Učitavanje…
cancel = Odustani
discard-title = Odbaciti promjene?
discard-message = Uneseno će biti izgubljeno.
discard = Odbaci
confirm = Potvrdi
retry = Pokušaj ponovno
delete = Izbriši
back = Natrag
language = Jezik
theme = Tema
theme-light = Svijetla
theme-dark = Tamna
theme-system = Sustav
theme-hint-device = Spremljeno samo na ovom uređaju.
theme-hint-system = Prati izgled ovog uređaja.

### Navigation

nav-main = Glavna navigacija
nav-projects = Projekti
nav-charts = Statistika
nav-settings = Postavke

### Connectivity

offline-banner = Izvan mreže
offline-pending =
    { $count ->
        [one] { $count } na čekanju
        [few] { $count } na čekanju
       *[other] { $count } na čekanju
    }

sync-conflict-edit = Sukob: uređivanje „{ $name }” nije uspjelo (stavka izbrisana). Preskočeno.
sync-conflict-delete = Sukob: brisanje „{ $name }” nije uspjelo (stavka izbrisana). Preskočeno.
sync-conflict-other = Sukob: operacija na „{ $name }” nije uspjela (stavka izbrisana). Preskočeno.
sync-error = Greška sinkronizacije: { $reason }

### Errors

error-network = Poslužitelj nije dostupan. Provjeri internetsku vezu.
error-rate-limited = Poslužitelj je preopterećen. Pokušaj ponovno za trenutak.
error-generic = Nešto je pošlo po zlu. Pokušaj ponovno.

error-invalid-email = Ta e-mail adresa nije valjana.
error-invalid-password = Ta lozinka nije valjana.
error-password-too-short = Lozinka mora imati najmanje 12 znakova.
error-client-outdated = Ova verzija aplikacije je zastarjela. Ažuriraj je za prijavu.
error-invalid-link = Ova poveznica nije valjana.
error-batch-too-large = Previše stavki odjednom.
error-payers-required = Odaberi barem jednog platitelja.
error-debtors-required = Odaberi barem jednu osobu koja duguje.
error-duplicate-participant = Sudionik se pojavljuje dvaput na istoj strani.
error-participant-not-in-project = Taj sudionik nije dio ovog projekta.
error-too-many-participants = Previše sudionika za jedan trošak.
error-invalid-credentials = Netočan e-mail ili lozinka.
error-unauthenticated = Prijavi se za tu radnju.
error-email-not-verified = Tvoja e-mail adresa još nije potvrđena.
error-project-not-found = Ovaj projekt više ne postoji.
error-expense-not-found = Ovaj trošak više ne postoji.
error-storage-full = Pohrana je puna: ključ ovog projekta nije bilo moguće spremiti na ovaj uređaj. Sačuvaj poveznicu za dijeljenje.
error-user-not-found = Ovaj sudionik više ne postoji.
error-tricount-not-found = Tricount nije pronađen ili je njegov API vratio grešku.
error-too-many-members = Ovaj projekt je dosegao ograničenje broja članova.
error-identity-taken = Drugi račun već je preuzeo ovog sudionika.
error-claim-proof-invalid = Ovaj uređaj nema ključ projekta pa ne može preuzeti sudionika. Ponovno otvori poveznicu za dijeljenje.
error-user-has-payments = Ovaj sudionik ima troškove u projektu i ne može se ukloniti.
error-resend-cooldown = Pričekaj 60 sekundi prije traženja novog e-maila.
error-self-friend-request = Ne možeš dodati sebe kao prijatelja.
error-not-a-friend = Možeš pozvati samo osobe s popisa prijatelja.
error-friend-has-no-key = Ovaj prijatelj još nije otvorio najnoviju verziju aplikacije. Zamoli ga da se jednom prijavi, pa pokušaj ponovno.
error-friend-request-not-found = Ovaj zahtjev za prijateljstvo više ne postoji.
error-invitation-not-found = Ova pozivnica više ne postoji.
error-too-many-friend-requests = Zasad previše zahtjeva za prijateljstvo. Pokušaj sutra.
error-too-many-invitations = Previše pozivnica na čekanju.
error-invalid-kdf-salt = Postavke šifriranja nisu valjane. Ažuriraj aplikaciju i pokušaj ponovno.
error-mixed-project-batch = Ti sudionici nisu svi u istom projektu.
error-invalid-payload = Ova verzija aplikacije poslala je podatke koje poslužitelj ne prihvaća. Ažuriraj je i pokušaj ponovno.
error-invalid-public-key = Tvoj ključ za šifriranje nije valjan. Ažuriraj aplikaciju i pokušaj ponovno.
error-payment-methods-stale = Tvoji podaci za plaćanje promijenjeni su na drugom uređaju. Osvježi i pokušaj ponovno.

### Auth

field-email = E-mail
field-email-placeholder = ti@primjer.hr
field-password = Lozinka
field-name = Ime
field-name-placeholder = Ana Horvat

login-title = Prijava
login-submit = Prijavi se
login-submitting = Prijava…
login-password-placeholder = Tvoja lozinka
login-no-account = Još nemaš račun?
login-unverified = Tvoja e-mail adresa još nije potvrđena. Provjeri sandučić ili ponovno pošalji poveznicu.
login-maybe-unverified = Novi račun? Prvo potvrdi adresu poveznicom koju smo ti poslali ili je pošalji ponovno.
login-resend = Ponovno pošalji poveznicu za potvrdu
login-resending = Slanje…
login-resend-sent = E-mail poslan - provjeri sandučić.

register-submit = Stvori račun
register-submitting = Stvaranje…
register-have-account = Već imaš račun?
register-password-placeholder = Najmanje 12 znakova
register-password-warning = Zapiši svoju lozinku: mi je nikad ne vidimo i ne možemo ti je vratiti.
register-check-email-title = Provjeri e-mail
register-email-sent = E-mail poslan
register-email-sent-hint = Klikni poveznicu u sandučiću za aktivaciju računa.
register-not-received-prefix = Nisi ga dobio? Provjeri spam ili se
register-sign-in-link = prijavi
register-not-received-suffix = da ponovno pošalješ poveznicu.
register-terms-prefix = Stvaranjem računa prihvaćaš naše
register-terms-link = uvjete korištenja
register-terms-and = i našu
register-privacy-link = politiku privatnosti

settings-title = Postavke
settings-preferences = Postavke
settings-preferences-local = Spremljeno na ovom uređaju.
settings-preferences-synced = Sinkronizirano s tvojim računom, šifrirano.
settings-about = O aplikaciji
settings-anonymous-title = Nisi prijavljen
settings-upsell-title = Tvoji projekti na svakom uređaju
settings-upsell-free = Besplatno
settings-upsell-body = Counted radi i bez računa. S besplatnim računom tvoji projekti i postavke prate te na telefon, laptop i web - i dalje šifrirani, i dalje nečitljivi nama.
settings-locked-badge = Račun
settings-locked-friends = Stvori račun za dodavanje prijatelja i pozivanje u projekt iz aplikacije - bez dijeljenja poveznice.
settings-locked-payment-methods = Spremi svoj IBAN ili aplikaciju za plaćanje jednom i podijeli ih s projektima po izboru. Tko ti duguje, vidi ih pored tvog imena.
settings-friends-hint = Dodaj prijatelje i pozovi ih u svoje projekte bez dijeljenja poveznice.

account-member-since = Član od
account-logout = Odjava
account-logging-out = Odjava…
account-delete-title = Izbriši moj račun
account-delete-warning = Trenutno i trajno brisanje tvog računa i tvojih podataka.
account-delete-confirm-title = Izbriši račun
account-delete-confirm-message = Tvoj račun, sesije i popis projekata bit će trajno izbrisani. Bez tvoje lozinke šifrirani podaci dijeljenog projekta postaju ti nečitljivi - to se ne može poništiti.
erase-device-open = Odjavi se i izbriši ovaj uređaj
erase-device-title = Izbrisati ovaj uređaj?
erase-device-body = Projekti, njihovi ključevi i predmemorija na ovom uređaju bit će izbrisani, a zatim će sesija biti završena. Tvoji projekti ostaju na tvom računu: ponovno se prijavi da ih vratiš.
erase-device-checking = Provjeravam jesu li tvoji projekti na tvom računu…
erase-device-pending = Promjene koje čekaju slanje: { $count }. Ponovno se spoji na internet i pokušaj ponovno.
erase-device-unreachable = Nije bilo moguće provjeriti jesu li tvoji projekti na tvom računu. Provjeri vezu i pokušaj ponovno.
erase-device-at-risk = Ovi projekti nisu na tvom računu i bit će izgubljeni. Ako ih želiš zadržati, odustani i najprije kopiraj njihovu poveznicu:
erase-device-confirm = Izbriši i odjavi se
erase-device-confirm-anyway = Ipak izbriši
erase-device-erasing = Brisanje…

settings-payment-methods = Podaci za plaćanje
settings-payment-methods-hint = Kako želiš da ti se vrati novac. Šifrirano s tvojim računom.
payment-method-kind = Način
payment-method-kind-other = Ostalo
payment-method-label = Naziv
payment-method-label-placeholder = Glavni račun
payment-method-value = Podaci
payment-method-value-placeholder = IBAN, broj telefona, korisničko ime…
payment-method-add = Dodaj
payment-method-remove = Ukloni { $name }
payment-method-empty = Još nisi dodao podatke za plaćanje.
payment-method-deleted = Način plaćanja izbrisan.
payment-method-value-required = Ispuni podatke svakog načina plaćanja ili ga ukloni.
payment-method-label-required = Daj naziv svom prilagođenom načinu.
payment-method-too-long = To je predugo - skrati.
payment-method-invalid-characters = Ukloni prijelome redaka ili nevidljive znakove.
payment-method-limit = Možeš spremiti do { $max } načina plaćanja.
payment-methods-saved = Podaci za plaćanje spremljeni.
payment-methods-offline = Moraš biti na mreži da spremiš podatke za plaćanje.
payment-methods-stale = Tvoji podaci za plaćanje promijenjeni su na drugom uređaju. Ponovno su učitani - pokušaj ponovno.
payment-methods-key-missing = Prijavi se ponovno za upravljanje podacima za plaćanje.
settings-payment-methods-share-warning = Podijeljeni način vidljiv je svim članovima tvojih projekata.
payment-method-share = Podijeli s mojim projektima
payment-method-share-hint = Prikazuje se pored tvog imena kad ti netko duguje.
payment-method-copy = Kopiraj { $name }
payment-method-copied = Kopirano.
payment-method-copy-failed = Kopiranje nije uspjelo - označi tekst i kopiraj ga ručno.

verify-email-checking = Potvrđivanje e-mail adrese…
verify-email-welcome = E-mail potvrđen - dobro došao u Counted!
verify-email-back-to-login = Natrag na prijavu
verify-email-intro = Unesi svoju lozinku i potvrdi da je ova adresa tvoja.
verify-email-submit = Potvrdi moju adresu

### Project status

project-archive = Arhiviraj
project-sheet-invite = Pozovi
project-sheet-recurring = Ponavljajući
project-sheet-edit = Uredi projekt
project-sheet-close = Zatvori projekt
project-sheet-archive = Arhiviraj projekt
project-sheet-reopen = Ponovno otvori projekt
project-sheet-unarchive = Vrati projekt iz arhive
project-sheet-leave = Napusti projekt

### Dates

date-long = { $day }. { $month } { $year }.

month-1 = siječnja
month-2 = veljače
month-3 = ožujka
month-4 = travnja
month-5 = svibnja
month-6 = lipnja
month-7 = srpnja
month-8 = kolovoza
month-9 = rujna
month-10 = listopada
month-11 = studenoga
month-12 = prosinca

month-short-1 = sij
month-short-2 = velj
month-short-3 = ožu
month-short-4 = tra
month-short-5 = svi
month-short-6 = lip
month-short-7 = srp
month-short-8 = kol
month-short-9 = ruj
month-short-10 = lis
month-short-11 = stu
month-short-12 = pro

### Actions

add = Dodaj
create = Stvori
creating = Stvaranje…
edit = Uredi
leave = Napusti
close = Zatvori
paste = Zalijepi
join = Pridruži se
import = Uvezi
importing = Uvoz…
field-description = Opis
field-date = Datum
date-today = Danas
date-yesterday = Jučer
field-optional = Neobavezno

### Projects

projects-filter-active = Aktivni
projects-filter-all = Svi
projects-count-label = Projekti
projects-empty = Nema projekata
projects-offline-banner = Podaci izvan mreže - ponovno se spoji za osvježavanje.
demo-banner = Demo projekt - samo za čitanje.
demo-start-own = Pokreni vlastiti projekt
demo-badge = Demo
demo-bar = Tvoje promjene nestaju kada zatvoriš ovu karticu.
demo-leave = Započni stvarno
demo-preparing = Pripremamo tvoju demo verziju…
demo-failed = Demo verziju nije bilo moguće pripremiti.
demo-sign-in-blocked = Prijava je isključena u demo verziji. Napusti je kako bi se prijavio ili stvorio račun; tvoji projekti ostaju netaknuti.
projects-no-local-data = Nema lokalnih podataka
projects-no-local-data-hint = Prijavi se da prvi put učitaš svoje projekte.
projects-add = Dodaj projekt
projects-create = Stvori projekt
projects-personal-expense = Osobni trošak
personal-expense-banner = Dodano u Moje troškove
personal-create-title = Izradi osobni projekt
personal-create-hint = Prati što trošiš sam
group-create-title = Stvori zajednički projekt
group-create-hint = Dijeli troškove s prijateljima i obitelji
personal-create-action = Izradi
personal-create-signup = Izradi račun
personal-sheet-title = Tvoj osobni projekt
personal-sheet-intro = Za bilježenje onoga što trošiš sam. Prikazuje se s tvojim grupama u Statistici.
personal-sheet-point-alone = Jedan član: ti, s imenom tvog računa. Bez poveznice za dijeljenje.
personal-sheet-point-encrypted = Šifrirano na tvojim uređajima, kao i ostali projekti.
personal-sheet-point-stats = Tvoji udjeli u grupama pribrajaju se ovim troškovima u Statistici.
personal-default-name = Moji troškovi
my-spending-prev-month = Prethodni mjesec
my-spending-next-month = Sljedeći mjesec
my-spending-spent = Potrošio si
my-spending-groups = Grupe
my-spending-income = Prihodi
my-spending-balance = Saldo mjeseca
my-spending-unconverted = Udjeli u drugoj valuti ostaju po strani dok tečaj nije poznat.
my-spending-empty = Ništa ovog mjeseca
my-spending-empty-hint = Dodaj trošak pomoću +. Tvoji udjeli u grupama pojavit će se ovdje automatski.
my-spending-share-of = od { $amount }
my-spending-uncounted = ne računa se
payer-you = ti
expense-received-by = primio/la
counted-switch = Računaj u Moji troškovi
counted-switch-on = Uključeno: tvoj udio dodaje se u Moji troškovi
counted-switch-off = Isključeno: tvoj udio ne prikazuje se u Moji troškovi
counted-groups = Računate grupe
counted-groups-intro = Tvoj udio u ovim grupama dodaje se u Moji troškovi i u proračune.
counted-groups-converted = { $from }, preračunato u { $to }
counted-groups-encrypted = Ovaj je izbor šifriran tvojim računom i prati tvoje uređaje.
counted-groups-empty = Još nema grupe na ovom uređaju.
counted-groups-done = U redu
counted-save-failed = Ovaj izbor nije moguće spremiti. Pokušaj ponovno.
tab-budget = Proračun
budget-title = Proračuni
budget-manage = Upravljaj
budget-left = Preostalo { $amount }
budget-over = Prekoračeno za { $amount }
budget-empty = Nema proračuna
budget-empty-hint = Postavi gornju granicu po kategoriji. Tvoji udjeli u grupama također se računaju.
budget-define = Postavi proračun
budget-sheet-title = Mjesečni proračuni
budget-sheet-intro = Na tvoj udio: osobno i grupe. Ostavi prazno da ne pratiš kategoriju.
budget-average = Prosj. zadnja 3 mjeseca: { $amount }
budget-for = Proračun { $category }
budget-alert = Upozori me na 80 %
budget-alert-hint = Izračunato na ovom uređaju
budget-invalid = Neispravan iznos za { $category }.
budget-offline = Proračuni se spremaju samo uz vezu. Ponovno se poveži i pokušaj opet.
budget-alert-near = { $category }: { $percent } % mjesečnog proračuna
budget-alert-over = { $category }: mjesečni proračun prekoračen
personal-card-budget = { $month } · proračun { $amount }
my-spending-search-summary =
    { $count ->
        [one] { $count } rezultat
        [few] { $count } rezultata
       *[other] { $count } rezultata
    } · { $total } · osobno i grupe, svi mjeseci
projects-join = Pridruži se projektu
projects-import-tricount = Uvezi iz Tricounta
project-actions = Radnje projekta

status-ongoing = U tijeku
status-closed = Zatvoren
status-archived = Arhiviran

nav-help = Pomoć
nav-privacy = Politika privatnosti
nav-terms = Uvjeti korištenja
nav-legal = Pravne informacije

leave-project-title = Napustiti projekt?
leave-project-message = Izgubit ćeš pristup s ovog uređaja. Ako ne ostane nijedan član, projekt i svi njegovi troškovi trajno se brišu.

add-project-title = Novi projekt
add-project-name-label = Naziv projekta
add-project-name-placeholder = Moje putovanje, Cimeri 2024…
add-project-participants = Sudionici
add-project-participant-name = Ime sudionika
add-project-participant-placeholder = Clark Kent
add-project-offline = Ne možeš stvoriti projekt izvan mreže. Ponovno se spoji i pokušaj ponovno.
add-project-name-required = Projekt treba naziv.

join-link-label = Poveznica za dijeljenje
join-link-hint = Poveznica sadrži ključ za dešifriranje - kopiraj je cijelu.
join-invalid-link = Ta poveznica nije valjana. Zalijepi cijelu poveznicu za dijeljenje, uključujući dio nakon #.
join-wrong-project = Ta poveznica je za drugi projekt.

import-tricount-link-label = Tricount poveznica ili ključ
import-tricount-key-required = Unesi Tricount poveznicu ili ključ.
import-tricount-encryption-failed = Šifriranje nije uspjelo.
import-tricount-unimportable = Ništa nije uvezeno: ovaj Tricount ima članove s Tricount računom ili iznose koji se ne slažu (zahvaćeni unosi: { $count }).

### Expenses

save = Spremi
saving = Spremanje…
adding = Dodavanje…
link-copied = Poveznica kopirana
missing-encryption-key = Nedostaje ključ za šifriranje.
missing-encryption-key-title = Nedostaje ključ za šifriranje
missing-encryption-key-hint = Poveznica koju si koristio ne sadrži ključ potreban za dešifriranje ovog projekta. Koristi punu poveznicu koju je podijelio onaj tko ga je stvorio.
project-locked-hint = Ovaj uređaj nema ključ ovog projekta. Otvori njegovu poveznicu za dijeljenje da ga otključaš.
expenses-unreadable = Neki unosi ne mogu se pročitati na ovom uređaju i nisu uračunati u zbrojeve. Ažuriraj aplikaciju da ih vidiš.
project-unlock = Otključaj
project-no-local-data-hint = Prijavi se da prvi put učitaš podatke ovog projekta.
project-gone-title = Ovaj projekt više ne postoji
project-gone-hint = Izbrisan je kad ga je napustio posljednji član. Poveznica za dijeljenje više ne radi, čak i ako je ponovno otvoriš.

expense-add = Dodaj trošak
transfer-add = Dodaj prijenos
expense-edit-title = Uredi trošak
expense-category = Kategorija
expense-category-auto = Auto · { $emoji }
emoji-picker-label = Emoji
emoji-none = Bez emojija
expense-currency = Valuta iznosa
amount-op-add = Plus
amount-op-subtract = Minus
amount-op-multiply = Puta
amount-op-divide = Podijeljeno
amount-op-equals = Jednako
amount-op-done = Gotovo
expense-rate = Tečaj (neobavezno)
expense-rate-hint = Ostavi prazno za tečaj Europske komisije (InforEuro) za { $month }: 1 { $from } = { $rate } { $to }.
expense-rate-invalid = Unesi tečaj veći od 0.
expense-rate-unavailable = Automatski tečaj nije dostupan - unesi ga ručno.
expense-delete-title = Izbriši trošak
expense-delete-message = „{ $name }” bit će trajno izbrisan. To se ne može poništiti.
expense-inconsistent-amounts = Iznosi se ne slažu
expenses-empty = Nema troškova
expenses-empty-hint = Počni dodavanjem troškova gumbom ispod
expenses-show-more = Prikaži više (još { $count })

expense-type-expense = Trošak
expense-type-transfer = Prijenos
expense-type-gain = Primitak
expense-type-group = Vrsta
expense-paid-by = platio/la
expense-sent-by = poslao/la
expense-contributed-by = doprinio/la

expense-name-required = Naziv je obavezan.
expense-amount-not-positive = Iznos mora biti veći od 0.
expense-no-payer = Odaberi barem jednog platitelja.
expense-no-debtor = Odaberi barem jednu osobu koja duguje.
expense-invalid-date = Taj datum nije valjan.
expense-payers-mismatch = Zbroj platitelja je { $sum }, što ne odgovara iznosu troška ({ $total }).
expense-debtors-mismatch = Zbroj dužnika je { $sum }, što ne odgovara iznosu troška ({ $total }).

participants-none = Nitko
participants-everyone = Svi ({ $count })
participants-some = { $count } od { $total }
participants-select-all = Odaberi sve
participants-by-shares = Po udjelima
split-amounts = Iznosi
participants-remaining = Preostalo { $amount }
participants-over-by = { $amount } previše
participants-who-paid = Tko je platio?
participants-who-received = Tko je primio?
participants-who-transfers = Tko prenosi?
participants-who-receives = Tko prima?
participants-for-whom = Za koga?

stats-total-expenses = Ukupni troškovi
stats-my-expenses = Moji troškovi

tab-expenses = Troškovi
tab-balance = Stanje
tab-reimbursements = Podmirenje
balance-gets-back = Dobiva natrag
balance-owes = Duguje
balance-settled = Podmireno
reimbursements-empty-title = Sve je podmireno!
reimbursements-empty-hint = Prijedlozi za podmirenje pojavljuju se ovdje kad računi nisu u ravnoteži
reimbursement-owes = { $debtor } duguje { $creditor }
reimbursement-record = Podmiri
settlement-recorded = Podmirenje zabilježeno.
reimbursement-pay-with = Plati
reimbursement-pay-shared-by = Podijelio { $name } - provjeri ime primatelja koje prikazuje tvoja aplikacija prije slanja.
reimbursement-pay-title = Plati { $name }
reimbursements-mine-title = Ti duguješ
reimbursements-others-title = Ostali povrati
copy = Kopiraj

user-selection-title = Koji si sudionik?
user-selection-hint = Odaberi svoje ime s popisa.
user-selection-required = Odaberi sudionika.
identity-claimed = Povezano s računom
identity-claimed-by = Račun { $name }
identity-taken-repick = Drugi račun preuzeo je sudionika kojeg si koristio. Odaberi drugog.
participant-gone-repick = Sudionik kojeg si koristio uklonjen je iz ovog projekta. Odaberi drugog.

edit-project-title = Uredi projekt
edit-project-new-badge = novo
edit-project-deferred-new-members = dodavanje novih članova
edit-project-deferred-removals = uklanjanje članova
edit-project-offline-deferred = Izvan mreže: { $items } primijenit će se pri ponovnom spajanju.

export-failed = Izvoz nije uspio: { $reason }

history-expense-added = Trošak dodan: { $name }
history-expense-edited = Trošak uređen: { $name }
history-expense-deleted = Trošak izbrisan: { $name }
move-expense = Premjesti u projekt
move-expense-title = Premjesti u…
move-current = Trenutni
move-converted = { $currency }, preračunato po mjesečnom tečaju
move-note-personal = U sljedećem koraku biraš tko je platio i za koga. Trošak će napustiti Moje troškove.
move-note-group = Trošak će napustiti { $project } za sve članove, a saldo grupe bit će ponovno izračunat. U sljedećem koraku biraš tko je platio i za koga.
move-banner = Iz { $from } u { $to }
move-submit = Premjesti
move-done = Trošak premješten u { $project }
history-expense-moved = Trošak premješten u drugi projekt: { $name }
personal-badge = Osobno
move-members =
    { $count ->
        [one] { $count } član
        [few] { $count } člana
       *[other] { $count } članova
    }
history-project-edited = Projekt uređen: { $name }
history-name-changed = Naziv: „{ $from }” → „{ $to }”
history-description-added = Opis dodan: „{ $value }”
history-description-removed = Opis uklonjen: „{ $value }”
history-description-changed = Opis: „{ $from }” → „{ $to }”

### Sweep

field-amount = Iznos
expense-name-placeholder = Restoran, namirnice…
expense-actions = Radnje troška
expense-your-share = Tvoj udio
expense-your-share-value = Tvoj udio: { $amount } { $currency }
expense-share-percent = { $percent } %
expense-added-by = Dodano: { $name }, { $date }
expense-inconsistent-detail = Iznosi se ne slažu: { $paid } plaćeno, { $owed } dugovano, za trošak od { $total }. Uredi trošak da to ispraviš.
missing-access-key = Nedostaje pristupni ključ. Otvori ovaj projekt putem njegove poveznice za dijeljenje.
filter-all = Sve
filter-my-payments = Moja plaćanja
filter-my-debts = Što dugujem
expense-filter-group = Filtar
expenses-search = Pretraži troškove
expenses-no-match = Nijedan trošak ne odgovara.
expenses-search-open = Pretraži
expenses-search-clear = Očisti pretragu
expenses-search-summary =
    { $count ->
        [one] { $count } rezultat
        [few] { $count } rezultata
       *[other] { $count } rezultata
    } · { $total } · svi mjeseci
participants-shares-for = Udjeli za { $name }
participants-amount-for = Iznos za { $name }
reimbursement-add = Dodaj podmirenje
project-forget = Ukloni s mog popisa
project-history-title = Povijest
history-kind-add = Dodano
history-kind-delete = Izbrisano
history-kind-edit = Uređeno
export = Izvezi
export-json = Izvezi JSON
export-csv = Izvezi CSV
share-link = Podijeli
copy-link-failed = Poveznica se nije mogla kopirati
open-in-app = Otvori u aplikaciji
not-found-title = Stranica nije pronađena
not-found-back = Natrag na projekte

### Charts

charts-period = Razdoblje
period-all = Sve
period-month = Mjesec
period-3months = 3 mj.
period-year = Godina
period-custom = Prilagođeno
charts-tab-categories = Kategorije
charts-tab-trends = Trendovi
charts-total-spent = Ukupno potrošeno
charts-avg-per-person = Prosj. po osobi
charts-expense-count =
    { $count ->
        [one] { $count } trošak
        [few] { $count } troška
       *[other] { $count } troškova
    }
charts-nothing-to-show = Nema što prikazati
charts-my-share-skipped =
    { $count ->
        [one] 1 projekt nije uračunat - nije odabran sudionik ili se njegovi podaci nisu učitali.
        [few] { $count } projekta nisu uračunata - nije odabran sudionik ili se njihovi podaci nisu učitali.
       *[other] { $count } projekata nije uračunato - nije odabran sudionik ili se njihovi podaci nisu učitali.
    }

### Categories

category-food = Hrana
category-transport = Prijevoz
category-accommodation = Smještaj
category-leisure = Slobodno vrijeme
category-shopping = Kupovina
category-services = Usluge
category-parties-gifts = Zabave i pokloni
category-other = Ostalo
categories-manage = Upravljaj kategorijama
categories-title = Kategorije
categories-yours = Tvoje kategorije
categories-builtin = Zadane
categories-add = Dodaj kategoriju
categories-in-parent = u kategoriji { $parent }
categories-renamed-from = preimenovana · izvorno { $name }
categories-hidden-hint = skrivena · njezini troškovi idu u kategoriju { $other }
categories-always-shown = uvijek prikazana
categories-hidden = Skrivena
categories-new = Nova kategorija
categories-edit = Uredi kategoriju
categories-parent = Računa se u kategoriju
categories-parent-hint = U statistici svih tvojih projekata { $name } se računa u kategoriju { $parent }.
categories-create = Stvori
categories-hide = Sakrij ovu kategoriju
categories-hide-hint = Njezini troškovi svim se članovima prikazuju u kategoriji { $other }.
categories-delete-title = Izbrisati „{ $name }”?
categories-delete-body = Obuhvaćeni troškovi ({ $count }) vraćaju se u kategoriju { $parent }. Nijedan trošak se ne briše.
categories-delete-body-unknown = Njezini troškovi vraćaju se u kategoriju { $parent }. Nijedan trošak se ne briše.
categories-delete-warning = Kategorija nestaje za sve članove projekta.
categories-summary-created = Stvorene: { $count }
categories-summary-renamed = Preimenovane: { $count }
categories-summary-hidden = Skrivene: { $count }
categories-summary-default = Nema vlastitih kategorija
categories-offline = Kategorijama se može upravljati samo uz vezu. Ponovno se poveži i pokušaj opet.
categories-error-too-many = Projekt ima najviše 20 vlastitih kategorija.
categories-error-full = Ovaj projekt je dosegao ograničenje kategorija.
categories-error-name = Daj kategoriji naziv.
categories-error-missing = Ova kategorija se u međuvremenu promijenila. Zatvori i pokušaj ponovno.
history-category-created = Kategorija stvorena: { $name }
history-category-edited = Kategorija uređena: { $name }
history-category-deleted = Kategorija izbrisana: { $name }
charts-project = Projekt
charts-all-projects = Svi projekti
charts-date-from = Od
charts-date-to = Do
charts-total = Ukupno
charts-tab-people = Osobe
charts-tab-projects = Projekti
charts-scope = Čiji troškovi
charts-scope-group = Grupa
charts-scope-me = Ja
charts-currency = Valuta
charts-my-share = Moj udio
charts-share-of-total = { $pct } % od { $total }
charts-i-paid = Platio sam
charts-paid-more = { $amount } više od mog udjela
charts-paid-less = { $amount } manje od mog udjela
charts-paid-even = Točno moj udio
charts-part-title = Tvoj dio svake kategorije
charts-part-desc = Sivo je što je grupa potrošila, boja je što si ti potrošio.
charts-breakdown-title = Raspodjela po kategorijama
charts-breakdown-desc = Dodirni isječak ili redak za popis troškova.
charts-of-total = { $amount } od { $total }
charts-show-all = Prikaži sve ({ $count })
charts-show-less = Prikaži manje
charts-spend-title = Troškovi kroz vrijeme
charts-group-by = Grupiraj po
bucket-day = Dan
bucket-week = Tjedan
bucket-month = Mjesec
charts-avg = prosj.
charts-cat-title-day = { $category }, dan po dan
charts-cat-title-week = { $category }, tjedan po tjedan
charts-cat-title-month = { $category }, mjesec po mjesec
charts-cat-desc = Odaberi kategoriju i prati je kroz vrijeme.
charts-running-title = Ukupno do sada
charts-running-desc = Od { $date }.
charts-avg-per-day = { $amount } / dan u prosjeku
charts-avg-per-week = { $amount } / tjedan u prosjeku
charts-avg-per-month = { $amount } / mjesec u prosjeku
charts-people-title = Tko je nosio grupu
charts-people-desc = Što je tko platio, uz ono što je potrošio.
charts-paid = Plaćeno
charts-fair-share = Pošten udio
charts-you = (ti)
charts-net-more = platio više
charts-net-less = platio manje
charts-balance-title = Tvoje stanje kroz vrijeme
charts-balance-desc = Iznad crte grupa duguje tebi. Ispod nje ti duguješ grupi.
charts-owed = Duguju ti
charts-owe = Duguješ
charts-projects-title = Tvoj udio po projektu
charts-projects-desc = Zbrojevi se vode po valuti.
history-empty = Nema događaja
history-by = { $name }
not-found-hint = Ova stranica ne postoji ili je premještena.
payers-title-paid-by = Platio/la
payers-title-sender = Pošiljatelj
payers-title-contributors = Sudionici
debtors-title-debtors = Duguje
debtors-title-recipients = Primatelji
debtors-title-beneficiaries = Korisnici
debtors-title-for = Za
debtors-title-recipient = Primatelj
expense-split-between = podijeljeno na { $count }
expense-people =
    { $count ->
        [one] { $count } osoba
        [few] { $count } osobe
       *[other] { $count } osoba
    }

### Welcome

welcome-title = Tvoji računi nisu ničija briga.
welcome-subtitle = Dijeli troškove s prijateljima.
welcome-note = Besplatno. Bez računa. Bez oglasa.
welcome-link-title = Jedna poveznica i svi sudjeluju.
welcome-link-body = Nitko ne mora stvarati račun.
welcome-link-account = Račun? Nikad obavezan. Služi da pronađeš svoje projekte na drugom uređaju, pozoveš prijatelje iz aplikacije i podijeliš svoje podatke za plaćanje.
welcome-demo-project = Vikend u Lyonu
welcome-private-title = Nitko ne može čitati tvoje račune. Čak ni mi.
welcome-private-body = Imena, iznosi, projekti: sve se šifrira na tvom uređaju. Samo ti imaš ključ.
welcome-private-names = Imena
welcome-private-amounts = Iznosi
welcome-private-projects = Projekti
welcome-scan-title = Fotografiraj račun.
welcome-scan-body = Iznos, datum i kategorija ispune se sami. Sve se odvija na tvom telefonu. Fotografija se ne čuva.
welcome-eu-title = 100 % europski
welcome-no-ads = Bez oglasa
welcome-no-trackers = Bez praćenja
welcome-step = Korak { $current } od { $total }
welcome-next = Dalje
welcome-skip = Preskoči
welcome-start = Započni
welcome-how-it-works = Kako to točno radi?

### Help

help-intro = Često pitanje? Dodirni za prikaz odgovora.
help-create-project-q = Kako stvoriti projekt?
help-create-project-a = Na početnom zaslonu dodirni gumb + pri dnu. Daj projektu naziv, odaberi valutu i gotovo.
help-add-participants-q = Kako dodati sudionike?
help-add-participants-a = Otvori projekt i dodaj sudionike s popisa članova. Svaki sudionik može platiti ili dugovati za trošak.
help-share-project-q = Kako podijeliti projekt?
help-share-project-a = Podijeli URL projekta (onaj u adresnoj traci). Svatko s poveznicom može pregledavati i uređivati projekt.
help-add-expense-q = Kako dodati trošak?
help-add-expense-a = U projektu dodirni +, unesi iznos, navedi tko je platio i među kime podijeliti. Možeš odabrati i datum različit od današnjeg.
help-types-q = Koja je razlika između troška, prijenosa i primitka?
help-types-expense = - kupnja koju je obavila jedna osoba i podijelila među više njih.
help-types-transfer = - povrat od jedne osobe drugoj, bez podjele.
help-types-gain = - primljeni novac (povrat, poklon) za podjelu među više osoba.
help-past-date-q = Mogu li datirati trošak u prošlost?
help-past-date-a = Da, polje datuma je slobodno. Vrijeme stvaranja zapisa čuva se zasebno.
help-who-owes-q = Kako Counted izračunava tko što duguje?
help-who-owes-a = Counted računa neto stanje svakog sudionika (što je platio unaprijed minus što duguje), a zatim predlaže najkraći niz prijenosa koji sve podmiruje.
help-minimal-transfers-q = Zašto je broj predloženih prijenosa minimalan?
help-minimal-transfers-a = Algoritam prvo uparuje stanja koja se točno poništavaju, a zatim prolazi kroz ostatak od najvećeg vjerovnika do najvećeg dužnika. Rezultat: manje prijenosa za sve podmiriti.
help-import-tricount-q = Kako uvesti projekt iz Tricounta?
help-import-tricount-a = Na početnom zaslonu dodirni gumb „+” pri dnu, a zatim
help-import-tricount-b = Zalijepi poveznicu za dijeljenje Tricounta koji želiš uvesti.
help-encryption-q = Jesu li moji podaci šifrirani?
help-encryption-a = Da. Counted kombinira dva jamstva:
help-encryption-e2ee-term = Šifriranje s kraja na kraj
help-encryption-e2ee-def = - sve između tebe i poslužitelja putuje šifrirano.
help-encryption-zero-term = Nulti pristup
help-encryption-zero-def = - podatke šifriraš prije slanja, a poslužitelj pohranjuje samo šifrirani tekst. Nemamo ga kako pročitati.
help-encryption-see = Za detalje pogledaj
help-forgot-password-q = Što ako zaboravim lozinku?
help-forgot-password-warning = Nitko je ne može vratiti umjesto tebe.
help-forgot-password-a = Tvoja lozinka štiti tvoje ključeve za šifriranje i nikad nam se ne šalje: nitko - ni mi - ne može dešifrirati tvoje projekte umjesto tebe. Čuvaj je na sigurnom, idealno u upravitelju lozinki.
help-archive-delete-q = Kako arhivirati ili izbrisati projekt?
help-archive-delete-a = Na zaslonu projekta otvori izbornik i odaberi
help-archive-delete-b = da ga sakriješ, a zadržiš. Projekt se trajno briše kad ga napusti posljednji član.
help-delete-account-q = Kako izbrisati račun?
help-delete-account-a = Otvori Postavke i upotrijebi „Izbriši moj račun”. Trenutno je i ne može se poništiti.
help-contact = Još pitanja? Piši nam na

# Receipt scanning (mobile only)
expense-scan = Skeniraj račun
scan-in-progress = Čitanje računa…
scan-error-capture = Fotografija nije uspjela. Pokušaj ponovno ili unesi trošak ručno.
scan-error-unreadable = Ništa čitljivo na tom računu. Unesi trošak ručno.
scan-check-amount = Provjeri ukupan iznos - nije bio jasno otisnut.
scan-take-photo = Snimi fotografiju
scan-choose-photo = Odaberi fotografiju
expense-converted-from = Plaćeno { $amount } { $from } · 1 { $from } = { $rate } { $to }
project-currency = Valuta
project-currency-locked = Valuta se određuje pri stvaranju projekta.
currency-search = Pretraži valutu

update-required-title = Potrebno ažuriranje
update-required-body = Ova verzija Counteda prestara je za komunikaciju s poslužiteljem. Ažuriraj je da nastaviš koristiti aplikaciju.
update-required-button = Ažuriraj

notifications-label = Obavijesti
notifications-title = Obavijesti
notifications-empty = Ništa novo
notifications-friend-request = Zahtjev za prijateljstvo

friends-title = Prijatelji
friends-anonymous-body = Prijatelji su vezani uz tvoj račun. Prijavi se za dodavanje osoba i pozivanje u projekte bez dijeljenja poveznice.
friends-add-title = Dodaj prijatelja
friends-add-hint = Ako ta osoba ima račun, vidjet će tvoj zahtjev u aplikaciji. Ako nema, javi joj: pronaći će ga kad se registrira s ovom adresom.
friends-add-button = Dodaj
friends-add-from-project = Dodaj kao prijatelja
friends-request-sent = Zahtjev poslan
friends-no-account-key = Prijavi se ponovno za upravljanje prijateljima na ovom uređaju.
friends-incoming-title = Zahtjevi
friends-accept = Prihvati
friends-decline = Odbij
friends-list-title = Moji prijatelji
friends-list-empty = Još nema prijatelja. Dodaj nekoga e-mailom iznad ili iz projekta koji dijelite.
friends-remove = Ukloni
friends-remove-confirm-title = Ukloni prijatelja
friends-remove-confirm-message = { $email } više neće biti među tvojim prijateljima, a ni ti među njegovima. Bilo tko od vas može kasnije poslati novi zahtjev.
friends-no-key = Još nije spremno
friends-key-changed = Kod promijenjen
friends-key-changed-hint = Sigurnosni kôd prijatelja se promijenio. To se događa kad zaštiti svoj račun, ili ako je njegov ključ zamijenjen. Pozivnice s njim blokirane su dok jedno drugome ne pročitate novi kôd uživo ili telefonom i ne dodirneš „Kôd se podudara”.
friends-fingerprint = Sigurnosni kod
friends-fingerprint-hint = Dva prijatelja koja jedno drugom pročitaju isti sigurnosni kod znaju da nitko ne stoji između njih - čak ni naš poslužitelj.
friends-key-mismatch = Poslužitelj je za tvoj račun poslao javni ključ koji se ne podudara s tvojim privatnim ključem. Tvoj sigurnosni kod je skriven, a pozivnice su isključene na ovom uređaju: netko možda dira poslužitelj.
keypair-mismatch = Ključevi koje je poslužitelj poslao za tvoj račun ne odgovaraju onima na ovom uređaju. Pozivnice i promjene ključa ovdje su isključene dok se ne poklope: netko možda manipulira poslužiteljem.
friends-outgoing-title = Poslano
friends-outgoing-hint = Čeka se odgovor. Vidjet ćeš ih među prijateljima kad prihvate.
friends-withdraw = Odustani
friends-notify = Javi porukom
friends-notify-label = Javi: { $email }
friends-notify-message = Prelazim na Counted za naše zajedničke troškove. Besplatna registracija na https://counted.fr s adresom { $email }: tamo čeka moj zahtjev za prijateljstvo.
friends-notify-copied = Poruka kopirana
friends-notify-copy-failed = Poruku nije moguće kopirati
guests-hint-title =
    { $count ->
        [one] { $count } sudionik se još nije pridružio projektu
        [few] { $count } sudionika se još nisu pridružila projektu
       *[other] { $count } sudionika se još nije pridružilo projektu
    }
guests-hint-later = Kasnije
guests-hint-send = Pozovi ih
guests-send-title = Pošalji projekt sudionicima
guests-send-friend = Pozovi prijatelja s Counteda
guests-send-friend-badge = Bez poveznice
guests-send-friend-hint = Ako već ima račun, dobiva šifriranu pozivnicu; ništa ne ide preko tvoje aplikacije za poruke.
guests-send-link = Podijeli poveznicu projekta
guests-send-link-hint = Za one bez računa. Projekt otvaraju jednim dodirom.
guests-send-link-warning = Svatko tko ima ovu poveznicu može čitati i mijenjati projekt. Tvoja aplikacija za poruke je također vidi.
invite-friends-title = Pozovi prijatelje
invite-friends-hint = Ključ projekta šifrira se za svakog prijatelja na ovom uređaju. Poslužitelj ga nikad ne vidi.
invite-friends-empty = Još nema prijatelja za pozvati.
invite-friends-button = Pozovi
invite-sent = { $count ->
    [one] Pozivnica poslana
    [few] Poslane { $count } pozivnice
   *[other] Poslano { $count } pozivnica
}
invitation-badge = Pozivnica
invitation-to = Pridruži se „{ $name }”
invitation-to-unnamed = Pridruži se projektu
invitation-unreadable = Ova pozivnica ne može se otvoriti na ovom uređaju
invitation-from = Od { $email }
invitation-accept = Pridruži se
invitation-decline = Odbij

# Participants in the create and edit modals, and the "who are you?" picker - see
# docs/plans/friends.md §11.
participants-you-label = Tvoje ime u ovom projektu
participants-you-badge = Ti
participants-others = Ostali sudionici
participants-removed = Uklonjeni
participant-restore = Vrati
participants-empty = Dodaj sudionike ispod.
participants-duplicate = „{ $name }” je već na popisu.
participants-input-label = Dodaj prijatelja ili upiši ime
participants-input-placeholder = Prijatelj ili bilo koje ime
participants-suggest-friend = Prijatelj · pridružuje se kao „{ $name }”, dobiva pozivnicu
participants-suggest-not-ready = Prijatelj · još nije spreman
participants-suggest-guest = Dodaj „{ $text }”
participants-suggest-guest-sub = Gost
participants-friends = Tvoji prijatelji
participants-all-friends = Svi prijatelji
participants-login-hint = Prijavi se da dodaješ ljude izravno s popisa prijatelja.
participants-invite-badge = Pozovi
participants-guest-badge = Gost
participants-rename = Preimenuj: { $name }
participants-remove = Ukloni: { $name }
participants-rename-label = Novo ime
participants-rename-save = Spremi ime
participants-rename-hint = Ime koje svi vide u ovom projektu. Pozivnica i dalje ide na { $email }.
participants-invited-badge = Pozvan
participants-invited-sub = { $email } · još nije prihvaćeno
participants-invited-pending = Pozivnica još nije prihvaćena
participants-unlinked = Nije povezano s računom
add-project-create-invite = Stvori i pozovi: { $count }
edit-project-save-invite = Spremi i pozovi: { $count }
edit-project-you-are = Na ovom uređaju ti si { $name }
edit-project-no-identity = Još nisi odabrao/la tko si
edit-project-switch = Promijeni
edit-project-choose = Odaberi
invite-failed = Ove pozivnice nije bilo moguće poslati: { $emails }
invite-waiting = Ovi prijatelji moraju jednom otvoriti Counted prije nego što ih se može pozvati: { $emails }
key-offer-title = Drugi ključ za { $name }
key-offer-body = Ovaj uređaj ima drugi ključ za ovaj projekt, koji ga ovdje nikad nije otvorio.
key-offer-choices = Koristi ovaj ključ: projekt se otvara ključem iz ove poveznice ili pozivnice. Odustani: uređaj zadržava svoj trenutni ključ i ništa se ne mijenja.
key-offer-use = Koristi ovaj ključ
invite-again = Pozovi ponovno
friend-picker-title = Dodaj prijatelje
user-selection-invited-hint = { $email } te pozvao/la u „{ $project }”.
user-selection-suggested = Predloženo
user-selection-suggested-sub = { $email } te dodao/la pod ovim imenom
user-selection-confirm-as = Ja sam { $name }
user-selection-missing = Tvog imena nema? Zamoli sudionika da te doda u postavkama projekta.

## Ponavljajući troškovi

repeat-label = Ponavljaj
repeat-none = Ne ponavlja se
repeat-weekly = Svaki tjedan
repeat-biweekly = Svaka 2 tjedna
repeat-monthly = Svaki mjesec
repeat-quarterly = Svaka 3 mjeseca
repeat-yearly = Svake godine
repeat-every-weeks = Svakih { $count } tj.
repeat-every-months = Svakih { $count } mj.
repeat-every-years = Svakih { $count } god.
repeat-custom = Prilagođeno…
repeat-every = Svakih
repeat-unit-weeks = tj.
repeat-unit-months = mj.
repeat-unit-years = god.
repeat-on-weekday = dan: { $weekday }
repeat-on-day = { $day }. u mjesecu
repeat-on-day-month = { $day }. { $month }
repeat-month-end = U kraćim mjesecima pada na zadnji dan.
repeat-ends = Završava
repeat-ends-never = Nikad
repeat-ends-on = Na datum
repeat-ends-after = Nakon
repeat-fewer = Manje
repeat-more = Više
repeat-last-on = zadnji { $date }
repeat-variable = Iznos se svaki put mijenja
repeat-variable-hint = Svaki se dodaje s posljednjim iznosom i označava „za potvrdu”.
repeat-done = Gotovo
repeat-no-end = Bez kraja
repeat-until = Do { $date }
repeat-occurrences = Broj ponavljanja: { $count }
repeat-offline = Potrebna je veza. Sam trošak se ipak može dodati.
repeat-foreign = Ponavlja se kao { $amount } { $currency }, jednom preračunato po današnjem tečaju. Dobit ćeš upozorenje ako se tečaj promijeni za više od 5 %.
repeat-backfill = Počinje u prošlosti. Troškovi dodani sada: { $count }.
add-and-repeat = Dodaj i ponavljaj
weekday-1 = ponedjeljak
weekday-2 = utorak
weekday-3 = srijeda
weekday-4 = četvrtak
weekday-5 = petak
weekday-6 = subota
weekday-7 = nedjelja
recurring-title = Ponavljajući troškovi
recurring-strip = Ponavljajući troškovi: { $count }
recurring-next = Sljedeći: { $name }, { $date }
recurring-to-confirm = Za potvrdu: { $count }
recurring-per-month = Mjesečno, otprilike
recurring-your-share = Tvoj udio
recurring-active = Aktivni
recurring-paused = Pauzirani
recurring-finished = Završeni
recurring-paid-by = plaća { $name }
recurring-next-on = Sljedeći { $date }
recurring-progress = { $done } od { $total }
recurring-rate-badge = Tečaj promijenjen za { $percent } %
recurring-empty = Još se ništa ne ponavlja. Odaberi „Ponavljaj” kad dodaješ trošak: najam, pretplate, računi.
recurring-next-ones = Sljedeći
recurring-added-so-far = Dosad dodano
recurring-set-up-by = Postavio/la
recurring-pause = Pauziraj
recurring-resume = Nastavi
recurring-stop = Prekini ponavljanje
recurring-stop-title = Prekinuti „{ $name }”?
recurring-stop-message = Više se neće ponavljati. Već dodani troškovi ostaju.
recurring-resume-title = Nastaviti „{ $name }”?
recurring-resume-message = Sljedeći { $date }. Datumi propušteni tijekom pauze neće se dodati.
recurring-edit-title = Uredi ponavljajući trošak
recurring-edit-banner = Promjene vrijede od { $date }. Već dodani troškovi ostaju kakvi jesu.
recurring-next-on-label = Sljedeći
recurring-next-too-early = Sljedeći datum mora biti nakon zadnjeg već dodanog troška.
recurring-use-stop = Za završetak upotrijebi „Prekini ponavljanje” na ponavljajućem trošku.
recurring-drift = Tečaj { $currency } promijenio se za { $percent } % od postavljanja. Svaki se i dalje dodaje kao { $amount } { $project_currency } (1 { $currency } = { $rate }). Po današnjem tečaju bilo bi { $today_amount } { $project_currency }.
recurring-use-rate = Koristi današnji tečaj
recurring-keep = Zadrži { $amount } { $currency }
recurring-added = Dodani ponavljajući troškovi: { $count }
recurring-blocks-removal = { $name } se još ne može ukloniti: dio je { $rules }. Makni { $name } iz njih ili ih prekini pa ponovno spremi.
history-recurring-added = Automatski dodano: { $name } ({ $date })
history-recurring-created = Ponavljajući trošak postavljen: { $name }
history-recurring-edited = Ponavljajući trošak uređen: { $name }
history-recurring-paused = Ponavljajući trošak pauziran: { $name }
history-recurring-resumed = Ponavljajući trošak nastavljen: { $name }
history-recurring-stopped = Ponavljajući trošak zaustavljen: { $name }
occurrence-recurring = Ponavljajući trošak
occurrence-auto = Automatski dodano iz ponavljajućeg troška.
occurrence-auto-next = Automatski dodano iz ponavljajućeg troška. Sljedeći { $date }.
occurrence-auto-stopped = Automatski dodano iz ponavljajućeg troška koji je u međuvremenu prekinut.
occurrence-manage = Upravljaj
estimate-badge = Za potvrdu
expense-pending = Na čekanju
estimate-title = Iznos za potvrdu.
estimate-body = Dodano s prethodnim iznosom. Unesi pravi kad ga saznaš.
estimate-confirm = Potvrdi iznos
apply-to = Primijeni na
apply-this-only = Samo ovaj trošak
apply-and-next = Ovaj i sljedeće
apply-and-next-hint = Ponavljajući trošak mijenja se od { $date }
apply-rule-failed = Trošak je spremljen, ali ponavljajući trošak nije promijenjen.
occurrence-delete-message = „{ $name }” od { $date } bit će trajno izbrisan. Ponavljanje se nastavlja i ovaj se datum neće vratiti.
occurrence-delete-one = Izbriši samo ovaj
occurrence-delete-stop = Izbriši i prekini ponavljanje
error-recurring-clock = Sat ovog uređaja žuri. Provjeri datum i vrijeme.
error-recurring-not-found = Ovaj ponavljajući trošak više ne postoji.
error-recurring-stale = Netko je u međuvremenu promijenio ovaj ponavljajući trošak. Ponovno je učitan: provjeri ga i spremi ponovno.
error-too-many-recurring = Ovaj projekt već ima 50 ponavljajućih troškova. Prekini jedan koji ti više ne treba da bi dodao novi.
error-user-in-recurring = Ovaj sudionik dio je ponavljajućeg troška. Prvo ga makni iz njega ili prekini trošak.

home-owed-to-you = Duguju ti
home-you-owe = Duguješ
home-details-show = Prikaži pojedinosti
home-details-hide = Sakrij pojedinosti
home-all-settled = Sve je podmireno
home-pay = Plati
home-remind = Podsjeti
home-remind-message = Mali podsjetnik: u { $project } još ima nepodmirenih povrata.
home-row-owed = duguju ti
home-row-owe = duguješ
home-row-settled = Podmireno
home-row-quits = Kod tebe podmireno
home-row-pending =
    { $count ->
        [one] { $count } uplata na čekanju
        [few] { $count } uplate na čekanju
       *[other] { $count } uplata na čekanju
    }
home-totals-partial = Bez projekata bez tečaja
error-account-key-changed = Ključ tvog računa promijenjen je na drugom uređaju. Ponovno se prijavi za nastavak.
settings-security = Sigurnost
settings-change-password = Promijeni lozinku
settings-change-password-hint = Tvoji projekti i podaci ostaju kakvi jesu. Tvoji ostali uređaji bit će odjavljeni.
change-password-title = Promijeni svoju lozinku
change-password-current = Trenutna lozinka
change-password-new = Nova lozinka
change-password-confirm = Potvrdi novu lozinku
change-password-mismatch = Dvije nove lozinke se ne podudaraju.
change-password-submit = Promijeni lozinku
change-password-submitting = Mijenjam…
change-password-done = Lozinka promijenjena. Tvoji ostali uređaji su odjavljeni.
error-rotation-incomplete = Tvoj račun se u međuvremenu promijenio. Pokušaj ponovno.
error-key-lifecycle-unavailable = Još nije dostupno: najprije svi trebaju najnoviju verziju aplikacije.
settings-compromised = Netko možda zna moju lozinku
settings-compromised-hint = Nova lozinka, novi ključevi i odjava svih ostalih uređaja.
settings-compromised-open = Zaštiti moj račun
compromised-title = Zaštiti svoj račun
compromised-body = Tvoja lozinka i ključevi računa bit će zamijenjeni, a svi ostali uređaji odjavljeni. Tvoji prijatelji vidjet će da se tvoj sigurnosni kôd promijenio: ponovno ga usporedite.
compromised-keeps = Osoba s tvojom lozinkom zadržava sve što je već otvoreno: projekte koji su dosad bili čitljivi.
compromised-submit = Zamijeni lozinku i ključeve
compromised-done = Tvoj račun je zaštićen. Zamoli prijatelje da usporede tvoj novi sigurnosni kôd.
friends-new-fingerprint = Novi kôd:
friends-code-matches = Kôd se podudara
friends-code-trusted = Kôd potvrđen.
settings-recovery-key = Ključ za oporavak
settings-recovery-key-none = Bez njega te zaboravljena lozinka košta postavki, podataka za plaćanje i kodova prijatelja, a oporavak traje 3 dana.
settings-recovery-key-set = Stvoren. Ako zaboraviš lozinku, vraća ti račun sa svim sadržajem.
settings-recovery-key-pending = Novi ključ zamjenjuje ga { $when }. Nisi bio ti? Otkaži trenutnim ključem.
recovery-key-create = Stvori
recovery-key-replace = Zamijeni
recovery-key-cancel-replacement = Otkaži novi ključ
recovery-key-title = Tvoj ključ za oporavak
recovery-key-body = Zapiši ga ili ga spremi gdje samo ti možeš doći. Counted ga ne može ponovno prikazati ni vratiti umjesto tebe.
recovery-key-copy = Kopiraj
recovery-key-copied = Kopirano
recovery-key-save = Spremi kao datoteku
recovery-key-print = Ispiši
recovery-key-next = Nastavi
recovery-key-confirm-body = Za provjeru kopije upiši grupu { $n } ključa.
recovery-key-confirm-wrong = Ta se grupa ne podudara. Ponovno pogledaj kopiju.
recovery-key-password-body = Upiši lozinku da spremiš ključ na račun.
recovery-key-submit = Spremi ključ
recovery-key-done = Ključ za oporavak spremljen.
recovery-key-replacement-done = Novi ključ spremljen. Zamjenjuje trenutni za 72 sata; do tada trenutni još radi.
recovery-key-field = Ključ za oporavak
recovery-key-file-intro = Ključ za oporavak za Counted
recovery-key-cancel-done = Novi ključ je otkazan.
compromised-new-key = Spremi novi ključ za oporavak sada: zamjenjuje stari za 72 sata, a do tada stari još radi.
pending-reset-title = Zatraženo je ponovno postavljanje tvog računa
pending-reset-body = Stupa na snagu { $when }: tada se gube postavke, podaci za plaćanje i kodovi prijatelja, a uređaji se moraju ponovno prijaviti. Nisi bio ti? Otkaži trenutnom lozinkom.
pending-reset-cancel = Otkaži ponovno postavljanje
pending-reset-cancelled = Ponovno postavljanje je otkazano.
login-forgot = Zaboravljena lozinka?
forgot-title = Zaboravljena lozinka
forgot-body = Upiši svoju e-poštu. Ako ima račun, šaljemo poveznicu za odabir nove lozinke.
forgot-submit = Pošalji poveznicu
forgot-sent = Ako ova adresa ima račun, poveznica je na putu. Istječe za 24 sata.
reset-title = Vrati svoj račun
reset-with-key-body = Upiši ključ za oporavak i novu lozinku: sve se vraća kako je bilo.
reset-submit-key = Vrati moj račun
reset-no-key = Nemam ključ za oporavak
reset-without-key-title = Ponovno postavi bez ključa za oporavak
reset-without-key-body = Tvoj račun počinje ispočetka s novom lozinkom za 72 sata. Projekti se vraćaju s tvojih uređaja i poveznica za dijeljenje; postavke, podaci za plaćanje i kodovi prijatelja ne. Do tada trenutna lozinka radi i može otkazati ponovno postavljanje.
reset-name = Tvoje ime
reset-submit-request = Ponovno postavi moj račun
reset-requested = Gotovo. Nova lozinka radi od { $when }. Do tada trenutna lozinka radi i može to otkazati.
reset-pending-already = Ponovno postavljanje već čeka do { $when }. Novi zahtjev pokreće 72 sata ispočetka.
recovered-done = Račun je vraćen. Ostali uređaji su odjavljeni.
login-reset-undoable = Ovu je lozinku u posljednjih 30 dana zamijenilo ponovno postavljanje. Ako ga nisi tražio, poništi ga: račun se vraća kakav je bio.
login-undo-reset = Poništi ponovno postavljanje
reset-undone = Ponovno postavljanje je poništeno. Račun je kao prije.
error-reset-pending = Nova lozinka radi kad ponovno postavljanje stupi na snagu, 72 sata nakon zahtjeva.
error-account-disabled = Ovaj je račun izbrisan ubrzo nakon ponovnog postavljanja i čeka brisanje. Prethodna lozinka još može poništiti ponovno postavljanje.
error-recovery-key-invalid = Ovaj ključ za oporavak ne otvara ovaj račun.
error-project-key-required = Ovaj projekt treba svoj ključ. Ponovno ga otvori putem poveznice za dijeljenje.
names-or = { $first } ili { $last }
participants-admin-badge = Admin
edit-project-admins = Admini: { $count }
edit-project-succession = Ako u projektu ne ostane nijedan admin, ulogu preuzima osoba s računom koja je najduže u projektu.
member-sheet-account = Član s računom
member-actions = Radnje: { $name }
member-make-admin = Dodijeli ulogu admina
member-make-admin-hint = Dobiva mogućnost promjene ključa projekta i uklanjanja članova.
member-made-admin = { $name } sada ima ulogu admina.
group-security = Sigurnost projekta
group-security-admin = Sigurnost projekta · imaš ulogu admina
group-rotate = Promijeni ključ projekta
group-rotate-hint = Ako je poveznica projekta procurila: oni koji imaju staru poveznicu gube pristup projektu.
group-remove = Ukloni nekoga iz projekta
group-remove-hint = Oduzima pristup i istodobno mijenja ključ.
group-admins-only = Samo za admine.
group-admins-only-account = Samo za admine, koji imaju račun.
group-ask-admin = Pitaj admina projekta: { $names }.
group-ask-admins = Pitaj admine projekta: { $names }.
group-ask-an-admin = Pitaj nekog od admina projekta.
group-with-account = S računom
group-free = Besplatno
group-sign-in-hint = Admin projekta ({ $names }) ti zatim može dodijeliti ulogu.
group-sign-in-hint-any = Admin projekta ti zatim može dodijeliti ulogu.
group-no-admin = Ovaj projekt nema admina, pa se njegov ključ ne može promijeniti.
group-hide = Sakrij nekoga
group-hide-hint = Skriva osobu s popisa bez oduzimanja pristupa: poveznica i ključ i dalje rade.
group-recreate-hint = Ovaj projekt nema admina. Da nekoga ukloniš, stvori novi projekt.
group-recreate = Stvori novi projekt
group-no-admin-yet = Ovaj projekt još nema admina.
group-creator-hint = Samo ti možeš postati admin ovog projekta, ako stvoriš račun.
group-creator-elsewhere-hint = Samo osoba koja je stvorila projekt može postati njegov admin, ako stvori račun. Inače stvori novi projekt da nekoga ukloniš.
member-revoke-admin = Oduzmi ulogu admina
member-revoke-admin-hint = Više neće moći mijenjati ključ projekta ni uklanjati članove.
member-revoked-admin = { $name } više nema ulogu admina.
member-step-down = Prestani biti admin
member-step-down-hint = Više nećeš moći mijenjati ključ projekta ni uklanjati članove.
member-stepped-down = Više nemaš ulogu admina.
member-remove = Ukloni iz projekta
member-remove-hint = Mijenja ključ projekta: { $name } više neće moći čitati ništa novo.
member-hide = Sakrij s popisa
error-user-not-admin = Ovaj član nije admin.
error-creator-proof-invalid = Ovaj uređaj nije stvorio ovaj projekt.
error-project-has-admin = Ovaj projekt već ima admina.
error-not-project-admin = To mogu napraviti samo admini projekta.
error-user-not-claimed = Ova osoba nema račun, pa ne može biti admin.
error-project-key-rotated = Ključ projekta je promijenjen. Zatraži novu poveznicu od člana.
error-project-rotating = Ključ projekta se upravo mijenja. Pokušaj ponovno za trenutak.
error-rotation-expired = Promjena ključa trajala je predugo i otkazana je. Počni ispočetka.
error-rotation-rows-changed = Projekt se promijenio tijekom promjene ključa. Počni ispočetka.
project-rotated-title = Više nemaš pristup ovom projektu
project-rotated-body = Ključ ovog projekta je promijenjen. Zatraži novu poveznicu od admina projekta.
project-rotated-open-link = Otvori poveznicu
rotation-remove-title = Ukloniti { $name } iz projekta?
rotation-rotate-title = Promijeniti ključ projekta?
rotation-remove-body = Uklanjanje člana znači stvaranje novog ključa. Novu poveznicu trebat će podijeliti s osobama bez računa.
rotation-rotate-body = Tko ima staru poveznicu, gubi pristup projektu. Novu poveznicu trebat će podijeliti s osobama bez računa.
rotation-boxed =
    { $count ->
        [0] Nijedan drugi član neće automatski dobiti ključ
        [one] Ključ će automatski dobiti { $count } član
        [few] Ključ će automatski dobiti { $count } člana
       *[other] Ključ će automatski dobiti { $count } članova
    }
rotation-by-link-title = Osobe kojima ćeš morati poslati novu poveznicu
rotation-by-link-hint = Te osobe nemaju račun. Poslat ćeš im poveznicu odmah nakon toga.
rotation-names-and = { $first } i { $last }
rotation-names-and-others = { $names } i još { $count }
rotation-confirm-remove = Ukloni i promijeni ključ
rotation-confirm-rotate = Promijeni ključ
rotation-done-title = Novi ključ vrijedi
rotation-done-removed = { $name } više nije dio projekta.
rotation-done-rotated = Ključ projekta je promijenjen.
rotation-done-boxed =
    { $count ->
        [0] Ključ nije automatski poslan nijednom drugom članu.
        [one] Ključ je automatski poslan { $count } članu.
        [few] Ključ je automatski poslan { $count } članovima.
       *[other] Ključ je automatski poslan { $count } članovima.
    }
rotation-new-link =
    { $count ->
        [0] Nova poveznica na projekt
        [one] Nova poveznica za slanje · { $count } osoba bez računa
        [few] Nova poveznica za slanje · { $count } osobe bez računa
       *[other] Nova poveznica za slanje · { $count } osoba bez računa
    }
rotation-link-renewed = Poveznica je uspješno obnovljena.
rotation-done = Gotovo
history-key-changed = Ključ projekta promijenjen
history-member-removed = { $name } više nije dio projekta
project-rotated-row-hint = Ključ je promijenjen. Zatraži novu poveznicu.
