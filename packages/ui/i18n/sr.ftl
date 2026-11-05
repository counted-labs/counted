# Srpski (latinica). Potpun osim pravnih tekstova (legal-, terms-, privacy-), koji postoje samo na
# engleskom i francuskom i za svaku poruku zasebno padaju na en.ftl.

### Common

loading = Učitavanje…
cancel = Otkaži
confirm = Potvrdi
retry = Pokušaj ponovo
delete = Obriši
back = Nazad
language = Jezik
theme = Tema
theme-light = Svetla
theme-dark = Tamna
theme-system = Sistemska
theme-hint-device = Sačuvano samo na ovom uređaju.
theme-hint-system = Prati izgled ovog uređaja.

### Navigation

nav-main = Glavna navigacija
nav-projects = Projekti
nav-charts = Statistika
nav-settings = Podešavanja

### Connectivity

offline-banner = Van mreže
offline-pending =
    { $count ->
        [one] { $count } na čekanju
        [few] { $count } na čekanju
       *[other] { $count } na čekanju
    }

sync-conflict-edit = Sukob: izmena „{ $name }” nije uspela (stavka obrisana). Preskočeno.
sync-conflict-delete = Sukob: brisanje „{ $name }” nije uspelo (stavka obrisana). Preskočeno.
sync-conflict-other = Sukob: operacija na „{ $name }” nije uspela (stavka obrisana). Preskočeno.
sync-error = Greška sinhronizacije: { $reason }

### Errors

error-network = Server nije dostupan. Proveri internet vezu.
error-rate-limited = Server je preopterećen. Pokušaj ponovo za trenutak.
error-generic = Nešto je pošlo naopako. Pokušaj ponovo.

error-invalid-email = Ta imejl adresa nije važeća.
error-invalid-password = Ta lozinka nije važeća.
error-password-too-short = Lozinka mora imati najmanje 12 znakova.
error-client-outdated = Ova verzija aplikacije je zastarela. Ažuriraj je da bi se prijavio.
error-invalid-link = Ovaj link nije važeći.
error-batch-too-large = Previše stavki odjednom.
error-payers-required = Izaberi bar jednog platioca.
error-debtors-required = Izaberi bar jednu osobu koja duguje.
error-duplicate-participant = Učesnik se pojavljuje dvaput na istoj strani.
error-participant-not-in-project = Taj učesnik nije deo ovog projekta.
error-too-many-participants = Previše učesnika za jedan trošak.
error-invalid-credentials = Netačan imejl ili lozinka.
error-unauthenticated = Prijavi se za tu radnju.
error-email-not-verified = Tvoja imejl adresa još nije potvrđena.
error-project-not-found = Ovaj projekat više ne postoji.
error-expense-not-found = Ovaj trošak više ne postoji.
error-storage-full = Skladište je puno: ključ ovog projekta nije mogao da se sačuva na ovom uređaju. Sačuvaj link za deljenje.
error-user-not-found = Ovaj učesnik više ne postoji.
error-tricount-not-found = Tricount nije pronađen ili je njegov API vratio grešku.
error-too-many-members = Ovaj projekat je dostigao ograničenje broja članova.
error-identity-taken = Drugi nalog je već preuzeo ovog učesnika.
error-claim-proof-invalid = Ovaj uređaj nema ključ projekta pa ne može da preuzme učesnika. Ponovo otvori link za deljenje.
error-user-has-payments = Ovaj učesnik ima troškove u projektu i ne može se ukloniti.
error-resend-cooldown = Sačekaj 60 sekundi pre traženja novog imejla.
error-self-friend-request = Ne možeš da dodaš sebe kao prijatelja.
error-not-a-friend = Možeš da pozoveš samo osobe sa liste prijatelja.
error-friend-has-no-key = Ovaj prijatelj još nije otvorio najnoviju verziju aplikacije. Zamoli ga da se jednom prijavi, pa pokušaj ponovo.
error-friend-request-not-found = Ovaj zahtev za prijateljstvo više ne postoji.
error-invitation-not-found = Ova pozivnica više ne postoji.
error-too-many-friend-requests = Zasad previše zahteva za prijateljstvo. Pokušaj sutra.
error-too-many-invitations = Previše pozivnica na čekanju.
error-invalid-kdf-salt = Podešavanja šifrovanja nisu važeća. Ažuriraj aplikaciju i pokušaj ponovo.
error-mixed-project-batch = Ovi učesnici nisu svi u istom projektu.
error-invalid-payload = Ova verzija aplikacije je poslala podatke koje server ne prihvata. Ažuriraj je i pokušaj ponovo.
error-invalid-public-key = Tvoj ključ za šifrovanje nije važeći. Ažuriraj aplikaciju i pokušaj ponovo.
error-payment-methods-stale = Tvoji podaci za plaćanje su promenjeni na drugom uređaju. Osveži i pokušaj ponovo.

### Auth

field-email = Imejl
field-email-placeholder = ti@primer.rs
field-password = Lozinka
field-name = Ime
field-name-placeholder = Ana Petrović

login-title = Prijava
login-submit = Prijavi se
login-submitting = Prijava…
login-password-placeholder = Tvoja lozinka
login-no-account = Još nemaš nalog?
login-unverified = Tvoja imejl adresa još nije potvrđena. Proveri sanduče ili ponovo pošalji link.
login-maybe-unverified = Nov nalog? Prvo potvrdi adresu linkom koji smo ti poslali ili ga pošalji ponovo.
login-resend = Ponovo pošalji link za potvrdu
login-resending = Slanje…
login-resend-sent = Imejl poslat - proveri sanduče.

register-submit = Napravi nalog
register-submitting = Pravljenje…
register-have-account = Već imaš nalog?
register-password-placeholder = Najmanje 12 znakova
register-password-warning = Zapiši svoju lozinku: mi je nikad ne vidimo i ne možemo da ti je vratimo.
register-check-email-title = Proveri imejl
register-email-sent = Imejl poslat
register-email-sent-hint = Klikni na link u sandučetu da aktiviraš nalog.
register-not-received-prefix = Nisi ga dobio? Proveri spam ili se
register-sign-in-link = prijavi
register-not-received-suffix = da ponovo pošalješ link.
register-terms-prefix = Pravljenjem naloga prihvataš naše
register-terms-link = uslove korišćenja
register-terms-and = i našu
register-privacy-link = politiku privatnosti

settings-title = Podešavanja
settings-preferences = Podešavanja
settings-preferences-local = Sačuvano na ovom uređaju.
settings-preferences-synced = Sinhronizovano s tvojim nalogom, šifrovano.
settings-about = O aplikaciji
settings-anonymous-title = Nisi prijavljen
settings-upsell-title = Tvoji projekti na svakom uređaju
settings-upsell-free = Besplatno
settings-upsell-body = Counted radi i bez naloga. Sa besplatnim nalogom tvoji projekti i podešavanja prate te na telefon, laptop i veb - i dalje šifrovani, i dalje nečitljivi nama.
settings-locked-badge = Nalog
settings-locked-friends = Napravi nalog da dodaješ prijatelje i pozivaš ih u projekat iz aplikacije - bez deljenja linka.
settings-locked-payment-methods = Sačuvaj svoj IBAN ili aplikaciju za plaćanje jednom i podeli ih s projektima po izboru. Ko ti duguje, vidi ih pored tvog imena.
settings-friends-hint = Dodaj prijatelje i pozovi ih u svoje projekte bez deljenja linka.

account-member-since = Član od
account-logout = Odjava
account-logging-out = Odjava…
account-delete-title = Obriši moj nalog
account-delete-warning = Trenutno i trajno brisanje tvog naloga i tvojih podataka.
account-delete-confirm-title = Obriši nalog
account-delete-confirm-message = Tvoj nalog, sesije i lista projekata biće trajno obrisani. Bez tvoje lozinke šifrovani podaci deljenog projekta postaju ti nečitljivi - to ne može da se poništi.
erase-device-open = Odjavi se i obriši ovaj uređaj
erase-device-title = Obrisati ovaj uređaj?
erase-device-body = Projekti, njihovi ključevi i keš na ovom uređaju biće obrisani, a zatim će sesija biti završena. Tvoji projekti ostaju na tvom nalogu: ponovo se prijavi da ih vratiš.
erase-device-checking = Proveravam da li su tvoji projekti na tvom nalogu…
erase-device-pending = Izmene koje čekaju slanje: { $count }. Ponovo se poveži na internet i pokušaj ponovo.
erase-device-unreachable = Nije moguće proveriti da li su tvoji projekti na tvom nalogu. Proveri vezu i pokušaj ponovo.
erase-device-at-risk = Ovi projekti nisu na tvom nalogu i biće izgubljeni. Ako želiš da ih zadržiš, otkaži i prvo kopiraj njihov link:
erase-device-confirm = Obriši i odjavi se
erase-device-confirm-anyway = Ipak obriši
erase-device-erasing = Brisanje…

settings-payment-methods = Podaci za plaćanje
settings-payment-methods-hint = Kako želiš da ti se vrati novac. Šifrovano s tvojim nalogom.
payment-method-kind = Način
payment-method-kind-other = Ostalo
payment-method-label = Naziv
payment-method-label-placeholder = Glavni račun
payment-method-value = Podaci
payment-method-value-placeholder = IBAN, broj telefona, korisničko ime…
payment-method-add = Dodaj
payment-method-remove = Ukloni { $name }
payment-method-empty = Još nisi dodao podatke za plaćanje.
payment-method-deleted = Način plaćanja obrisan.
payment-method-value-required = Popuni podatke svakog načina plaćanja ili ga ukloni.
payment-method-label-required = Daj naziv svom prilagođenom načinu.
payment-method-too-long = To je predugačko - skrati.
payment-method-invalid-characters = Ukloni prelome redova ili nevidljive znakove.
payment-method-limit = Možeš da sačuvaš do { $max } načina plaćanja.
payment-methods-saved = Podaci za plaćanje sačuvani.
payment-methods-offline = Moraš biti na mreži da sačuvaš podatke za plaćanje.
payment-methods-stale = Tvoji podaci za plaćanje promenjeni su na drugom uređaju. Ponovo su učitani - pokušaj ponovo.
payment-methods-key-missing = Prijavi se ponovo da upravljaš podacima za plaćanje.
settings-payment-methods-share-warning = Podeljeni način vidljiv je svim članovima tvojih projekata.
payment-method-share = Podeli s mojim projektima
payment-method-share-hint = Prikazuje se pored tvog imena kad ti neko duguje.
payment-method-copy = Kopiraj { $name }
payment-method-copied = Kopirano.
payment-method-copy-failed = Kopiranje nije uspelo - označi tekst i kopiraj ga ručno.

verify-email-checking = Potvrđivanje imejl adrese…
verify-email-welcome = Imejl potvrđen - dobro došao u Counted!
verify-email-back-to-login = Nazad na prijavu
verify-email-intro = Unesi svoju lozinku da potvrdiš da je ova adresa tvoja.
verify-email-submit = Potvrdi moju adresu

### Project status

project-archive = Arhiviraj
project-sheet-invite = Pozovi
project-sheet-recurring = Ponavljajući
project-sheet-edit = Izmeni projekat
project-sheet-close = Zatvori projekat
project-sheet-archive = Arhiviraj projekat
project-sheet-reopen = Ponovo otvori projekat
project-sheet-unarchive = Vrati projekat iz arhive
project-sheet-leave = Napusti projekat

### Dates

date-long = { $day }. { $month } { $year }.

month-1 = januara
month-2 = februara
month-3 = marta
month-4 = aprila
month-5 = maja
month-6 = juna
month-7 = jula
month-8 = avgusta
month-9 = septembra
month-10 = oktobra
month-11 = novembra
month-12 = decembra

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
create = Napravi
creating = Pravljenje…
edit = Izmeni
leave = Napusti
close = Zatvori
paste = Nalepi
join = Pridruži se
import = Uvezi
importing = Uvoz…
field-description = Opis
field-date = Datum
date-today = Danas
date-yesterday = Juče
field-optional = Opciono

### Projects

projects-filter-active = Aktivni
projects-filter-all = Svi
projects-count-label = Projekti
projects-empty = Nema projekata
projects-offline-banner = Podaci van mreže - ponovo se poveži za osvežavanje.
demo-banner = Demo projekat - samo za čitanje.
demo-start-own = Pokreni svoj projekat
demo-badge = Demo
demo-bar = Tvoje izmene nestaju kada zatvoriš ovu karticu.
demo-leave = Počni stvarno
demo-preparing = Priprema tvog demoa…
demo-failed = Demo nije mogao da se pripremi.
demo-sign-in-blocked = Prijava je isključena u demou. Izađi iz njega da bi se prijavio ili napravio nalog; tvoji projekti ostaju netaknuti.
projects-no-local-data = Nema lokalnih podataka
projects-no-local-data-hint = Prijavi se da prvi put učitaš svoje projekte.
projects-add = Dodaj projekat
projects-create = Napravi projekat
projects-personal-expense = Lični trošak
personal-expense-banner = Dodato u Moje troškove
personal-create-title = Napravi lični projekat
personal-create-hint = Prati šta trošiš sam
group-create-title = Napravi zajednički projekat
group-create-hint = Deli troškove sa prijateljima i porodicom
personal-create-action = Napravi
personal-create-signup = Napravi nalog
personal-sheet-title = Tvoj lični projekat
personal-sheet-intro = Za beleženje onoga što trošiš sam. Prikazuje se sa tvojim grupama u Statistici.
personal-sheet-point-alone = Jedan član: ti, sa imenom tvog naloga. Bez linka za deljenje.
personal-sheet-point-encrypted = Šifrovano na tvojim uređajima, kao i ostali projekti.
personal-sheet-point-stats = Tvoji udeli u grupama dodaju se ovim troškovima u Statistici.
personal-default-name = Moji troškovi
my-spending-prev-month = Prethodni mesec
my-spending-next-month = Sledeći mesec
my-spending-spent = Potrošio si
my-spending-groups = Grupe
my-spending-income = Prihodi
my-spending-balance = Saldo meseca
my-spending-unconverted = Udeli u drugoj valuti ostaju po strani dok kurs nije poznat.
my-spending-empty = Ništa ovog meseca
my-spending-empty-hint = Dodaj trošak pomoću +. Tvoji udeli u grupama pojaviće se ovde automatski.
my-spending-share-of = od { $amount }
my-spending-uncounted = ne računa se
payer-you = ti
expense-received-by = primio/la
counted-switch = Računaj u Moji troškovi
counted-switch-on = Uključeno: tvoj udeo se dodaje u Moji troškovi
counted-switch-off = Isključeno: tvoj udeo se ne prikazuje u Moji troškovi
counted-groups = Računate grupe
counted-groups-intro = Tvoj udeo u ovim grupama dodaje se u Moji troškovi i u budžete.
counted-groups-converted = { $from }, preračunato u { $to }
counted-groups-encrypted = Ovaj izbor je šifrovan tvojim nalogom i prati tvoje uređaje.
counted-groups-empty = Još nema grupe na ovom uređaju.
counted-groups-done = U redu
counted-save-failed = Ovaj izbor nije moguće sačuvati. Pokušaj ponovo.
tab-budget = Budžet
budget-title = Budžeti
budget-manage = Upravljaj
budget-left = Preostalo { $amount }
budget-over = Prekoračeno za { $amount }
budget-empty = Nema budžeta
budget-empty-hint = Postavi gornju granicu po kategoriji. Tvoji udeli u grupama se takođe računaju.
budget-define = Postavi budžet
budget-sheet-title = Mesečni budžeti
budget-sheet-intro = Na tvoj udeo: lično i grupe. Ostavi prazno da ne pratiš kategoriju.
budget-average = Prosek poslednja 3 meseca: { $amount }
budget-for = Budžet { $category }
budget-alert = Upozori me na 80 %
budget-alert-hint = Izračunato na ovom uređaju
budget-invalid = Neispravan iznos za { $category }.
budget-offline = Budžeti se čuvaju samo uz vezu. Ponovo se poveži i pokušaj opet.
budget-alert-near = { $category }: { $percent } % mesečnog budžeta
budget-alert-over = { $category }: mesečni budžet prekoračen
personal-card-budget = { $month } · budžet { $amount }
my-spending-search-summary =
    { $count ->
        [one] { $count } rezultat
        [few] { $count } rezultata
       *[other] { $count } rezultata
    } · { $total } · lično i grupe, svi meseci
projects-join = Pridruži se projektu
projects-import-tricount = Uvezi iz Tricounta
project-actions = Radnje projekta

status-ongoing = U toku
status-closed = Zatvoren
status-archived = Arhiviran

nav-help = Pomoć
nav-privacy = Politika privatnosti
nav-terms = Uslovi korišćenja
nav-legal = Pravne informacije

leave-project-title = Napustiti projekat?
leave-project-message = Izgubićeš pristup sa ovog uređaja. Ako ne ostane nijedan član, projekat i svi njegovi troškovi trajno se brišu.

add-project-title = Novi projekat
add-project-name-label = Naziv projekta
add-project-name-placeholder = Moje putovanje, Cimeri 2024…
add-project-participants = Učesnici
add-project-participant-name = Ime učesnika
add-project-participant-placeholder = Clark Kent
add-project-offline = Ne možeš da napraviš projekat van mreže. Ponovo se poveži i pokušaj ponovo.
add-project-name-required = Projekat treba naziv.

join-link-label = Link za deljenje
join-link-hint = Link sadrži ključ za dešifrovanje - kopiraj ga celog.
join-invalid-link = Taj link nije važeći. Nalepi ceo link za deljenje, uključujući deo posle #.
join-wrong-project = Taj link je za drugi projekat.

import-tricount-link-label = Tricount link ili ključ
import-tricount-key-required = Unesi Tricount link ili ključ.
import-tricount-encryption-failed = Šifrovanje nije uspelo.
import-tricount-unimportable = Ništa nije uvezeno: ovaj Tricount ima članove sa Tricount nalogom ili iznose koji se ne slažu (pogođeni unosi: { $count }).

### Expenses

save = Sačuvaj
saving = Čuvanje…
adding = Dodavanje…
link-copied = Link kopiran
missing-encryption-key = Nedostaje ključ za šifrovanje.
missing-encryption-key-title = Nedostaje ključ za šifrovanje
missing-encryption-key-hint = Link koji si koristio ne sadrži ključ potreban za dešifrovanje ovog projekta. Koristi pun link koji je podelio onaj ko ga je napravio.
project-locked-hint = Ovaj uređaj nema ključ ovog projekta. Otvori njegov link za deljenje da ga otključaš.
expenses-unreadable = Neki unosi ne mogu da se pročitaju na ovom uređaju i nisu uračunati u zbirove. Ažuriraj aplikaciju da ih vidiš.
project-unlock = Otključaj
project-no-local-data-hint = Prijavi se da prvi put učitaš podatke ovog projekta.
project-gone-title = Ovaj projekat više ne postoji
project-gone-hint = Obrisan je kad ga je napustio poslednji član. Link za deljenje više ne radi, čak i ako ga ponovo otvoriš.

expense-add = Dodaj trošak
transfer-add = Dodaj prenos
expense-edit-title = Izmeni trošak
expense-category = Kategorija
expense-category-auto = Auto · { $emoji }
emoji-picker-label = Emoji
emoji-none = Bez emodžija
expense-currency = Valuta iznosa
amount-op-add = Plus
amount-op-subtract = Minus
amount-op-multiply = Puta
amount-op-divide = Podeljeno
amount-op-equals = Jednako
amount-op-done = Gotovo
expense-rate = Kurs (opciono)
expense-rate-hint = Ostavi prazno za kurs Evropske komisije (InforEuro) za { $month }: 1 { $from } = { $rate } { $to }.
expense-rate-invalid = Unesi kurs veći od 0.
expense-rate-unavailable = Automatski kurs nije dostupan - unesi ga ručno.
expense-delete-title = Obriši trošak
expense-delete-message = „{ $name }” biće trajno obrisan. To ne može da se poništi.
expense-inconsistent-amounts = Iznosi se ne slažu
expenses-empty = Nema troškova
expenses-empty-hint = Počni dodavanjem troškova dugmetom ispod
expenses-show-more = Prikaži više (još { $count })

expense-type-expense = Trošak
expense-type-transfer = Prenos
expense-type-gain = Prihod
expense-type-group = Vrsta
expense-paid-by = platio/la
expense-sent-by = poslao/la
expense-contributed-by = doprineo/la

expense-name-required = Naziv je obavezan.
expense-amount-not-positive = Iznos mora biti veći od 0.
expense-no-payer = Izaberi bar jednog platioca.
expense-no-debtor = Izaberi bar jednu osobu koja duguje.
expense-invalid-date = Taj datum nije važeći.
expense-payers-mismatch = Zbir platilaca je { $sum }, što ne odgovara iznosu troška ({ $total }).
expense-debtors-mismatch = Zbir dužnika je { $sum }, što ne odgovara iznosu troška ({ $total }).

participants-none = Niko
participants-everyone = Svi ({ $count })
participants-some = { $count } od { $total }
participants-select-all = Izaberi sve
participants-by-shares = Po udelima
split-amounts = Iznosi
participants-remaining = Preostalo { $amount }
participants-over-by = { $amount } previše
participants-who-paid = Ko je platio?
participants-who-received = Ko je primio?
participants-who-transfers = Ko prenosi?
participants-who-receives = Ko prima?
participants-for-whom = Za koga?

stats-total-expenses = Ukupni troškovi
stats-my-expenses = Moji troškovi

tab-expenses = Troškovi
tab-balance = Stanje
tab-reimbursements = Izmirenje
balance-gets-back = Dobija nazad
balance-owes = Duguje
balance-settled = Izmireno
reimbursements-empty-title = Sve je izmireno!
reimbursements-empty-hint = Predlozi za izmirenje pojavljuju se ovde kad računi nisu u ravnoteži
reimbursement-owes = { $debtor } duguje { $creditor }
reimbursement-record = Izmiri
reimbursement-pay-with = Plati
reimbursement-pay-shared-by = Podelio { $name } - proveri ime primaoca koje prikazuje tvoja aplikacija pre slanja.
reimbursement-pay-title = Plati { $name }
reimbursements-mine-title = Ti duguješ
reimbursements-others-title = Ostali povraćaji
copy = Kopiraj

user-selection-title = Koji si učesnik?
user-selection-hint = Izaberi svoje ime sa liste.
user-selection-required = Izaberi učesnika.
identity-claimed = Povezano s nalogom
identity-claimed-by = Nalog { $name }
identity-taken-repick = Drugi nalog je preuzeo učesnika kojeg si koristio. Izaberi drugog.
participant-gone-repick = Učesnik kojeg si koristio uklonjen je iz ovog projekta. Izaberi drugog.

edit-project-title = Izmeni projekat
edit-project-new-badge = novo
edit-project-deferred-new-members = dodavanje novih članova
edit-project-deferred-removals = uklanjanje članova
edit-project-offline-deferred = Van mreže: { $items } primeniće se pri ponovnom povezivanju.

export-failed = Izvoz nije uspeo: { $reason }

history-expense-added = Trošak dodat: { $name }
history-expense-edited = Trošak izmenjen: { $name }
history-expense-deleted = Trošak obrisan: { $name }
move-expense = Premesti u projekat
move-expense-title = Premesti u…
move-current = Trenutni
move-converted = { $currency }, preračunato po mesečnom kursu
move-note-personal = U sledećem koraku biraš ko je platio i za koga. Trošak će napustiti Moje troškove.
move-note-group = Trošak će napustiti { $project } za sve članove, a saldo grupe biće ponovo izračunat. U sledećem koraku biraš ko je platio i za koga.
move-banner = Iz { $from } u { $to }
move-submit = Premesti
move-done = Trošak premešten u { $project }
move-delete-failed = Trošak je dodat u drugi projekat, ali je još u { $project }: obriši ga tamo.
history-expense-moved = Trošak premešten u drugi projekat: { $name }
personal-badge = Lično
move-members =
    { $count ->
        [one] { $count } član
        [few] { $count } člana
       *[other] { $count } članova
    }
history-project-edited = Projekat izmenjen: { $name }
history-name-changed = Naziv: „{ $from }” → „{ $to }”
history-description-added = Opis dodat: „{ $value }”
history-description-removed = Opis uklonjen: „{ $value }”
history-description-changed = Opis: „{ $from }” → „{ $to }”

### Sweep

field-amount = Iznos
expense-name-placeholder = Restoran, namirnice…
expense-actions = Radnje troška
expense-your-share = Tvoj udeo
expense-your-share-value = Tvoj udeo: { $amount } { $currency }
expense-share-percent = { $percent }%
expense-added-by = Dodato: { $name }, { $date }
expense-inconsistent-detail = Iznosi se ne slažu: { $paid } plaćeno, { $owed } dugovano, za trošak od { $total }. Izmeni trošak da to ispraviš.
missing-access-key = Nedostaje pristupni ključ. Otvori ovaj projekat preko njegovog linka za deljenje.
filter-all = Sve
filter-my-payments = Moja plaćanja
filter-my-debts = Šta dugujem
expense-filter-group = Filter
expenses-search = Pretraži troškove
expenses-no-match = Nijedan trošak ne odgovara.
expenses-search-open = Pretraži
expenses-search-clear = Obriši pretragu
expenses-search-summary =
    { $count ->
        [one] { $count } rezultat
        [few] { $count } rezultata
       *[other] { $count } rezultata
    } · { $total } · svi meseci
participants-shares-for = Udeli za { $name }
participants-amount-for = Iznos za { $name }
reimbursement-add = Dodaj izmirenje
project-forget = Ukloni sa moje liste
project-history-title = Istorija
history-kind-add = Dodato
history-kind-delete = Obrisano
history-kind-edit = Izmenjeno
export = Izvezi
export-json = Izvezi JSON
export-csv = Izvezi CSV
share-link = Podeli
copy-link-failed = Link nije mogao da se kopira
open-in-app = Otvori u aplikaciji
not-found-title = Stranica nije pronađena
not-found-back = Nazad na projekte

### Charts

charts-period = Period
period-all = Sve
period-month = Mesec
period-3months = 3 mes.
period-year = Godina
period-custom = Prilagođeno
charts-tab-categories = Kategorije
charts-tab-trends = Trendovi
charts-total-spent = Ukupno potrošeno
charts-avg-per-person = Pros. po osobi
charts-expense-count =
    { $count ->
        [one] { $count } trošak
        [few] { $count } troška
       *[other] { $count } troškova
    }
charts-nothing-to-show = Nema šta da se prikaže
charts-my-share-skipped =
    { $count ->
        [one] 1 projekat nije uračunat - nije izabran učesnik ili se njegovi podaci nisu učitali.
        [few] { $count } projekta nisu uračunata - nije izabran učesnik ili se njihovi podaci nisu učitali.
       *[other] { $count } projekata nije uračunato - nije izabran učesnik ili se njihovi podaci nisu učitali.
    }

### Categories

category-food = Hrana
category-transport = Prevoz
category-accommodation = Smeštaj
category-leisure = Slobodno vreme
category-shopping = Kupovina
category-services = Usluge
category-parties-gifts = Zabave i pokloni
category-other = Ostalo
categories-manage = Upravljaj kategorijama
categories-title = Kategorije
categories-yours = Tvoje kategorije
categories-builtin = Podrazumevane
categories-add = Dodaj kategoriju
categories-in-parent = u kategoriji { $parent }
categories-renamed-from = preimenovana · prvobitno { $name }
categories-hidden-hint = skrivena · njeni troškovi idu u kategoriju { $other }
categories-always-shown = uvek prikazana
categories-hidden = Skrivena
categories-new = Nova kategorija
categories-edit = Izmeni kategoriju
categories-parent = Računa se u kategoriju
categories-parent-hint = U statistici svih tvojih projekata { $name } se računa u kategoriju { $parent }.
categories-create = Napravi
categories-hide = Sakrij ovu kategoriju
categories-hide-hint = Njeni troškovi se svim članovima prikazuju u kategoriji { $other }.
categories-delete-title = Obrisati „{ $name }”?
categories-delete-body = Obuhvaćeni troškovi ({ $count }) vraćaju se u kategoriju { $parent }. Nijedan trošak se ne briše.
categories-delete-body-unknown = Njeni troškovi vraćaju se u kategoriju { $parent }. Nijedan trošak se ne briše.
categories-delete-warning = Kategorija nestaje za sve članove projekta.
categories-summary-created = Napravljene: { $count }
categories-summary-renamed = Preimenovane: { $count }
categories-summary-hidden = Skrivene: { $count }
categories-summary-default = Nema sopstvenih kategorija
categories-offline = Kategorijama se može upravljati samo uz vezu. Ponovo se poveži i pokušaj opet.
categories-error-too-many = Projekat ima najviše 20 sopstvenih kategorija.
categories-error-full = Ovaj projekat je dostigao ograničenje kategorija.
categories-error-name = Daj kategoriji naziv.
categories-error-missing = Ova kategorija se u međuvremenu promenila. Zatvori i pokušaj ponovo.
history-category-created = Kategorija napravljena: { $name }
history-category-edited = Kategorija izmenjena: { $name }
history-category-deleted = Kategorija obrisana: { $name }
charts-project = Projekat
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
charts-my-share = Moj udeo
charts-share-of-total = { $pct } % od { $total }
charts-i-paid = Platio sam
charts-paid-more = { $amount } više od mog udela
charts-paid-less = { $amount } manje od mog udela
charts-paid-even = Tačno moj udeo
charts-part-title = Tvoj deo svake kategorije
charts-part-desc = Sivo je šta je grupa potrošila, boja je šta si ti potrošio.
charts-breakdown-title = Raspodela po kategorijama
charts-breakdown-desc = Dodirni isečak ili red za spisak troškova.
charts-of-total = { $amount } od { $total }
charts-show-all = Prikaži sve ({ $count })
charts-show-less = Prikaži manje
charts-spend-title = Troškovi kroz vreme
charts-group-by = Grupiši po
bucket-day = Dan
bucket-week = Nedelja
bucket-month = Mesec
charts-avg = pros.
charts-cat-title-day = { $category }, dan po dan
charts-cat-title-week = { $category }, nedelju po nedelju
charts-cat-title-month = { $category }, mesec po mesec
charts-cat-desc = Izaberi kategoriju i prati je kroz vreme.
charts-running-title = Ukupno do sada
charts-running-desc = Od { $date }.
charts-avg-per-day = { $amount } / dan u proseku
charts-avg-per-week = { $amount } / nedelja u proseku
charts-avg-per-month = { $amount } / mesec u proseku
charts-people-title = Ko je nosio grupu
charts-people-desc = Šta je ko platio, pored onoga što je potrošio.
charts-paid = Plaćeno
charts-fair-share = Pošten udeo
charts-you = (ti)
charts-net-more = platio više
charts-net-less = platio manje
charts-balance-title = Tvoje stanje kroz vreme
charts-balance-desc = Iznad linije grupa duguje tebi. Ispod nje ti duguješ grupi.
charts-owed = Duguju ti
charts-owe = Duguješ
charts-projects-title = Tvoj udeo po projektu
charts-projects-desc = Zbirovi se vode po valuti.
history-empty = Nema događaja
history-by = { $name }
not-found-hint = Ova stranica ne postoji ili je premeštena.
payers-title-paid-by = Platio/la
payers-title-sender = Pošiljalac
payers-title-contributors = Učesnici
debtors-title-debtors = Duguje
debtors-title-recipients = Primaoci
debtors-title-beneficiaries = Korisnici
debtors-title-for = Za
debtors-title-recipient = Primalac
expense-split-between = podeljeno na { $count }
expense-people =
    { $count ->
        [one] { $count } osoba
        [few] { $count } osobe
       *[other] { $count } osoba
    }

### Welcome

welcome-title = Tvoji računi nisu ničija briga.
welcome-subtitle = Deli troškove s prijateljima.
welcome-note = Besplatno. Bez naloga. Bez reklama.
welcome-link-title = Jedan link i svi učestvuju.
welcome-link-body = Niko ne mora da pravi nalog.
welcome-link-account = Nalog? Nikad obavezan. Služi da pronađeš svoje projekte na drugom uređaju, pozoveš prijatelje iz aplikacije i podeliš svoje podatke za plaćanje.
welcome-demo-project = Vikend u Lionu
welcome-private-title = Niko ne može da čita tvoje račune. Čak ni mi.
welcome-private-body = Imena, iznosi, projekti: sve se šifruje na tvom uređaju. Samo ti imaš ključ.
welcome-private-names = Imena
welcome-private-amounts = Iznosi
welcome-private-projects = Projekti
welcome-scan-title = Fotografiši račun.
welcome-scan-body = Iznos, datum i kategorija se popune sami. Sve se dešava na tvom telefonu. Fotografija se ne čuva.
welcome-eu-title = 100 % evropski
welcome-no-ads = Bez reklama
welcome-no-trackers = Bez praćenja
welcome-step = Korak { $current } od { $total }
welcome-next = Dalje
welcome-skip = Preskoči
welcome-start = Započni
welcome-how-it-works = Kako to tačno radi?

### Help

help-intro = Često pitanje? Dodirni za prikaz odgovora.
help-create-project-q = Kako da napravim projekat?
help-create-project-a = Na početnom ekranu dodirni dugme + pri dnu. Daj projektu naziv, izaberi valutu i gotovo.
help-add-participants-q = Kako da dodam učesnike?
help-add-participants-a = Otvori projekat i dodaj učesnike sa liste članova. Svaki učesnik može da plati ili duguje za trošak.
help-share-project-q = Kako da podelim projekat?
help-share-project-a = Podeli URL projekta (onaj u adresnoj traci). Svako s linkom može da pregleda i izmeni projekat.
help-add-expense-q = Kako da dodam trošak?
help-add-expense-a = U projektu dodirni +, unesi iznos, navedi ko je platio i među kime podeliti. Možeš da izabereš i datum različit od današnjeg.
help-types-q = Koja je razlika između troška, prenosa i prihoda?
help-types-expense = - kupovina koju je obavila jedna osoba i podelila među više njih.
help-types-transfer = - povraćaj od jedne osobe drugoj, bez podele.
help-types-gain = - primljeni novac (povraćaj, poklon) za podelu među više osoba.
help-past-date-q = Mogu li da datiram trošak u prošlost?
help-past-date-a = Da, polje datuma je slobodno. Vreme pravljenja zapisa čuva se zasebno.
help-who-owes-q = Kako Counted računa ko šta duguje?
help-who-owes-a = Counted računa neto stanje svakog učesnika (šta je platio unapred minus šta duguje), a zatim predlaže najkraći niz prenosa koji sve izmiruje.
help-minimal-transfers-q = Zašto je broj predloženih prenosa minimalan?
help-minimal-transfers-a = Algoritam prvo uparuje stanja koja se tačno poništavaju, a zatim prolazi kroz ostatak od najvećeg poverioca do najvećeg dužnika. Rezultat: manje prenosa da se sve izmiri.
help-import-tricount-q = Kako da uvezem projekat iz Tricounta?
help-import-tricount-a = Na početnom ekranu dodirni dugme „+” pri dnu, a zatim
help-import-tricount-b = Nalepi link za deljenje Tricounta koji želiš da uvezeš.
help-encryption-q = Da li su moji podaci šifrovani?
help-encryption-a = Da. Counted kombinuje dve garancije:
help-encryption-e2ee-term = Šifrovanje s kraja na kraj
help-encryption-e2ee-def = - sve između tebe i servera putuje šifrovano.
help-encryption-zero-term = Nulti pristup
help-encryption-zero-def = - podatke šifruješ pre slanja, a server čuva samo šifrovani tekst. Nemamo kako da ga pročitamo.
help-encryption-see = Za detalje pogledaj
help-forgot-password-q = Šta ako zaboravim lozinku?
help-forgot-password-warning = Niko ne može da je povrati umesto tebe.
help-forgot-password-a = Tvoja lozinka štiti tvoje ključeve za šifrovanje i nikad nam se ne šalje: niko - ni mi - ne može da dešifruje tvoje projekte umesto tebe. Čuvaj je na sigurnom, idealno u menadžeru lozinki.
help-archive-delete-q = Kako da arhiviram ili obrišem projekat?
help-archive-delete-a = Na ekranu projekta otvori meni i izaberi
help-archive-delete-b = da ga sakriješ, a zadržiš. Projekat se trajno briše kad ga napusti poslednji član.
help-delete-account-q = Kako da obrišem nalog?
help-delete-account-a = Otvori Podešavanja i koristi „Obriši moj nalog”. Trenutno je i ne može da se poništi.
help-contact = Još pitanja? Piši nam na

# Receipt scanning (mobile only)
expense-scan = Skeniraj račun
scan-in-progress = Čitanje računa…
scan-error-capture = Fotografija nije uspela. Pokušaj ponovo ili unesi trošak ručno.
scan-error-unreadable = Ništa čitljivo na tom računu. Unesi trošak ručno.
scan-check-amount = Proveri ukupan iznos - nije bio jasno odštampan.
scan-take-photo = Snimi fotografiju
scan-choose-photo = Izaberi fotografiju
expense-converted-from = Plaćeno { $amount } { $from } · 1 { $from } = { $rate } { $to }
project-currency = Valuta
project-currency-locked = Valuta se određuje pri pravljenju projekta.
currency-search = Pretraži valutu

update-required-title = Potrebno ažuriranje
update-required-body = Ova verzija Counteda prestara je za komunikaciju sa serverom. Ažuriraj je da nastaviš da koristiš aplikaciju.
update-required-button = Ažuriraj

notifications-label = Obaveštenja
notifications-title = Obaveštenja
notifications-empty = Ništa novo
notifications-friend-request = Zahtev za prijateljstvo

friends-title = Prijatelji
friends-anonymous-body = Prijatelji su vezani za tvoj nalog. Prijavi se da dodaješ osobe i pozivaš ih u projekte bez deljenja linka.
friends-add-title = Dodaj prijatelja
friends-add-hint = Ako ta osoba ima nalog, videće tvoj zahtev u aplikaciji. Ako nema, javi joj: pronaći će ga kad se registruje sa ovom adresom.
friends-add-button = Dodaj
friends-add-from-project = Dodaj kao prijatelja
friends-request-sent = Zahtev poslat
friends-no-account-key = Prijavi se ponovo da upravljaš prijateljima na ovom uređaju.
friends-incoming-title = Zahtevi
friends-accept = Prihvati
friends-decline = Odbij
friends-list-title = Moji prijatelji
friends-list-empty = Još nema prijatelja. Dodaj nekoga imejlom iznad ili iz projekta koji delite.
friends-remove = Ukloni
friends-remove-confirm-title = Ukloni prijatelja
friends-remove-confirm-message = { $email } više neće biti među tvojim prijateljima, a ni ti među njegovima. Bilo ko od vas može kasnije poslati novi zahtev.
friends-no-key = Još nije spremno
friends-key-changed = Kod promenjen
friends-key-changed-hint = Sigurnosni kod prijatelja se promenio. To se dešava kad zaštiti svoj nalog, ili ako je njegov ključ zamenjen. Pozivnice s njim su blokirane dok jedno drugom ne pročitate novi kod uživo ili telefonom i ne dodirneš „Kod se podudara”.
friends-fingerprint = Sigurnosni kod
friends-fingerprint-hint = Dva prijatelja koja jedno drugom pročitaju isti sigurnosni kod znaju da niko ne stoji između njih - čak ni naš server.
friends-key-mismatch = Server je za tvoj nalog poslao javni ključ koji se ne poklapa s tvojim privatnim ključem. Tvoj bezbednosni kod je sakriven, a pozivnice su isključene na ovom uređaju: neko možda dira server.
keypair-mismatch = Ključevi koje je server poslao za tvoj nalog ne odgovaraju onima na ovom uređaju. Pozivnice i promene ključa su ovde isključene dok se ne poklope: neko možda manipuliše serverom.
friends-outgoing-title = Poslato
friends-outgoing-hint = Čeka se odgovor. Videćeš ih među prijateljima kad prihvate.
friends-withdraw = Otkaži
friends-notify = Javi porukom
friends-notify-label = Javi: { $email }
friends-notify-message = Prelazim na Counted za naše zajedničke troškove. Besplatna registracija na https://counted.fr sa adresom { $email }: tamo čeka moj zahtev za prijateljstvo.
friends-notify-copied = Poruka kopirana
friends-notify-copy-failed = Poruku nije moguće kopirati
guests-hint-title =
    { $count ->
        [one] { $count } učesnik se još nije pridružio projektu
        [few] { $count } učesnika se još nisu pridružila projektu
       *[other] { $count } učesnika se još nije pridružilo projektu
    }
guests-hint-later = Kasnije
guests-hint-send = Pozovi ih
guests-send-title = Pošalji projekat učesnicima
guests-send-friend = Pozovi prijatelja sa Counteda
guests-send-friend-badge = Bez linka
guests-send-friend-hint = Ako već ima nalog, dobija šifrovanu pozivnicu; ništa ne ide preko tvoje aplikacije za poruke.
guests-send-link = Podeli link projekta
guests-send-link-hint = Za one bez naloga. Projekat otvaraju jednim dodirom.
guests-send-link-warning = Svako ko ima ovaj link može da čita i menja projekat. Tvoja aplikacija za poruke ga takođe vidi.
invite-friends-title = Pozovi prijatelje
invite-friends-hint = Ključ projekta šifruje se za svakog prijatelja na ovom uređaju. Server ga nikad ne vidi.
invite-friends-empty = Još nema prijatelja za pozivanje.
invite-friends-button = Pozovi
invite-sent = { $count ->
    [one] Pozivnica poslata
    [few] Poslate { $count } pozivnice
   *[other] Poslato { $count } pozivnica
}
invitation-badge = Pozivnica
invitation-to = Pridruži se „{ $name }”
invitation-to-unnamed = Pridruži se projektu
invitation-unreadable = Ova pozivnica ne može da se otvori na ovom uređaju
invitation-from = Od { $email }
invitation-accept = Pridruži se
invitation-decline = Odbij

# Participants in the create and edit modals, and the "who are you?" picker - see
# docs/plans/friends.md §11.
participants-you-label = Tvoje ime u ovom projektu
participants-you-badge = Ti
participants-others = Ostali učesnici
participants-removed = Uklonjeni
participant-restore = Vrati
participants-empty = Dodaj učesnike ispod.
participants-duplicate = „{ $name }” je već na listi.
participants-input-label = Dodaj prijatelja ili upiši ime
participants-input-placeholder = Prijatelj ili bilo koje ime
participants-suggest-friend = Prijatelj · pridružuje se kao „{ $name }”, dobija pozivnicu
participants-suggest-not-ready = Prijatelj · još nije spreman
participants-suggest-guest = Dodaj „{ $text }”
participants-suggest-guest-sub = Gost
participants-friends = Tvoji prijatelji
participants-all-friends = Svi prijatelji
participants-login-hint = Prijavi se da dodaješ ljude direktno sa liste prijatelja.
participants-invite-badge = Pozovi
participants-guest-badge = Gost
participants-rename = Preimenuj: { $name }
participants-remove = Ukloni: { $name }
participants-rename-label = Novo ime
participants-rename-save = Sačuvaj ime
participants-rename-hint = Ime koje svi vide u ovom projektu. Pozivnica i dalje ide na { $email }.
participants-invited-badge = Pozvan
participants-invited-sub = { $email } · još nije prihvaćeno
participants-invited-pending = Pozivnica još nije prihvaćena
participants-unlinked = Nije povezano s nalogom
add-project-create-invite = Napravi i pozovi: { $count }
edit-project-save-invite = Sačuvaj i pozovi: { $count }
edit-project-you-are = Na ovom uređaju ti si { $name }
edit-project-no-identity = Još nisi izabrao/la ko si
edit-project-switch = Promeni
edit-project-choose = Izaberi
invite-failed = Ove pozivnice nije bilo moguće poslati: { $emails }
invite-again = Pozovi ponovo
friend-picker-title = Dodaj prijatelje
user-selection-invited-hint = { $email } te je pozvao/la u „{ $project }”.
user-selection-suggested = Predloženo
user-selection-suggested-sub = { $email } te je dodao/la pod ovim imenom
user-selection-confirm-as = Ja sam { $name }
user-selection-missing = Tvog imena nema? Zamoli učesnika da te doda u podešavanjima projekta.

## Ponavljajući troškovi

repeat-label = Ponavljaj
repeat-none = Ne ponavlja se
repeat-weekly = Svake nedelje
repeat-biweekly = Svake 2 nedelje
repeat-monthly = Svakog meseca
repeat-quarterly = Svaka 3 meseca
repeat-yearly = Svake godine
repeat-every-weeks = Svakih { $count } ned.
repeat-every-months = Svakih { $count } mes.
repeat-every-years = Svakih { $count } god.
repeat-custom = Prilagođeno…
repeat-every = Svakih
repeat-unit-weeks = ned.
repeat-unit-months = mes.
repeat-unit-years = god.
repeat-on-weekday = dan: { $weekday }
repeat-on-day = { $day }. u mesecu
repeat-on-day-month = { $day }. { $month }
repeat-month-end = U kraćim mesecima pada na poslednji dan.
repeat-ends = Završava se
repeat-ends-never = Nikad
repeat-ends-on = Na datum
repeat-ends-after = Posle
repeat-fewer = Manje
repeat-more = Više
repeat-last-on = poslednji { $date }
repeat-variable = Iznos se svaki put menja
repeat-variable-hint = Svaki se dodaje sa poslednjim iznosom i označava „za potvrdu”.
repeat-done = Gotovo
repeat-no-end = Bez kraja
repeat-until = Do { $date }
repeat-occurrences = Broj ponavljanja: { $count }
repeat-offline = Potrebna je veza. Sam trošak se ipak može dodati.
repeat-foreign = Ponavlja se kao { $amount } { $currency }, jednom preračunato po današnjem kursu. Dobićeš upozorenje ako se kurs promeni za više od 5 %.
repeat-backfill = Počinje u prošlosti. Troškovi dodati sada: { $count }.
add-and-repeat = Dodaj i ponavljaj
weekday-1 = ponedeljak
weekday-2 = utorak
weekday-3 = sreda
weekday-4 = četvrtak
weekday-5 = petak
weekday-6 = subota
weekday-7 = nedelja
recurring-title = Ponavljajući troškovi
recurring-strip = Ponavljajući troškovi: { $count }
recurring-next = Sledeći: { $name }, { $date }
recurring-to-confirm = Za potvrdu: { $count }
recurring-per-month = Mesečno, otprilike
recurring-your-share = Tvoj deo
recurring-active = Aktivni
recurring-paused = Pauzirani
recurring-finished = Završeni
recurring-paid-by = plaća { $name }
recurring-next-on = Sledeći { $date }
recurring-progress = { $done } od { $total }
recurring-rate-badge = Kurs promenjen za { $percent } %
recurring-empty = Još se ništa ne ponavlja. Izaberi „Ponavljaj” kad dodaješ trošak: kirija, pretplate, računi.
recurring-next-ones = Sledeći
recurring-added-so-far = Do sada dodato
recurring-set-up-by = Postavio/la
recurring-pause = Pauziraj
recurring-resume = Nastavi
recurring-stop = Prekini ponavljanje
recurring-stop-title = Prekinuti „{ $name }”?
recurring-stop-message = Više se neće ponavljati. Već dodati troškovi ostaju.
recurring-resume-title = Nastaviti „{ $name }”?
recurring-resume-message = Sledeći { $date }. Datumi propušteni tokom pauze neće biti dodati.
recurring-edit-title = Uredi ponavljajući trošak
recurring-edit-banner = Promene važe od { $date }. Već dodati troškovi ostaju kakvi jesu.
recurring-next-on-label = Sledeći
recurring-next-too-early = Sledeći datum mora biti posle poslednjeg već dodatog troška.
recurring-use-stop = Za završetak koristi „Prekini ponavljanje” na ponavljajućem trošku.
recurring-drift = Kurs { $currency } promenio se za { $percent } % od postavljanja. Svaki se i dalje dodaje kao { $amount } { $project_currency } (1 { $currency } = { $rate }). Po današnjem kursu bilo bi { $today_amount } { $project_currency }.
recurring-use-rate = Koristi današnji kurs
recurring-keep = Zadrži { $amount } { $currency }
recurring-added = Dodati ponavljajući troškovi: { $count }
recurring-blocks-removal = { $name } se još ne može ukloniti: deo je { $rules }. Skloni { $name } iz njih ili ih prekini pa ponovo sačuvaj.
history-recurring-added = Automatski dodato: { $name } ({ $date })
history-recurring-created = Ponavljajući trošak postavljen: { $name }
history-recurring-edited = Ponavljajući trošak izmenjen: { $name }
history-recurring-paused = Ponavljajući trošak pauziran: { $name }
history-recurring-resumed = Ponavljajući trošak nastavljen: { $name }
history-recurring-stopped = Ponavljajući trošak zaustavljen: { $name }
occurrence-recurring = Ponavljajući trošak
occurrence-auto = Automatski dodato iz ponavljajućeg troška.
occurrence-auto-next = Automatski dodato iz ponavljajućeg troška. Sledeći { $date }.
occurrence-auto-stopped = Automatski dodato iz ponavljajućeg troška koji je u međuvremenu prekinut.
occurrence-manage = Upravljaj
estimate-badge = Za potvrdu
expense-pending = Na čekanju
estimate-title = Iznos za potvrdu.
estimate-body = Dodato sa prethodnim iznosom. Unesi pravi kad ga saznaš.
estimate-confirm = Potvrdi iznos
apply-to = Primeni na
apply-this-only = Samo ovaj trošak
apply-and-next = Ovaj i sledeće
apply-and-next-hint = Ponavljajući trošak se menja od { $date }
apply-rule-failed = Trošak je sačuvan, ali ponavljajući trošak nije promenjen.
occurrence-delete-message = „{ $name }” od { $date } biće trajno izbrisan. Ponavljanje se nastavlja i ovaj datum se neće vratiti.
occurrence-delete-one = Izbriši samo ovaj
occurrence-delete-stop = Izbriši i prekini ponavljanje
error-recurring-clock = Sat ovog uređaja žuri. Proveri datum i vreme.
error-recurring-not-found = Ovaj ponavljajući trošak više ne postoji.
error-recurring-stale = Neko je u međuvremenu promenio ovaj ponavljajući trošak. Ponovo je učitan: proveri ga i ponovo sačuvaj.
error-too-many-recurring = Ovaj projekat već ima 50 ponavljajućih troškova. Prekini jedan koji ti više ne treba da bi dodao novi.
error-user-in-recurring = Ovaj učesnik je deo ponavljajućeg troška. Prvo ga skloni iz njega ili prekini trošak.

home-owed-to-you = Duguju ti
home-you-owe = Duguješ
home-details-show = Prikaži detalje
home-details-hide = Sakrij detalje
home-all-settled = Sve je izmireno
home-pay = Plati
home-remind = Podseti
home-remind-message = Mali podsetnik: u { $project } još ima neizmirenih povraćaja.
home-row-owed = duguju ti
home-row-owe = duguješ
home-row-settled = Izmireno
home-totals-partial = Bez projekata bez kursa
error-account-key-changed = Ključ tvog naloga promenjen je na drugom uređaju. Ponovo se prijavi da nastaviš.
settings-security = Bezbednost
settings-change-password = Promeni lozinku
settings-change-password-hint = Tvoji projekti i podaci ostaju kakvi jesu. Tvoji ostali uređaji biće odjavljeni.
change-password-title = Promeni svoju lozinku
change-password-current = Trenutna lozinka
change-password-new = Nova lozinka
change-password-confirm = Potvrdi novu lozinku
change-password-mismatch = Dve nove lozinke se ne podudaraju.
change-password-submit = Promeni lozinku
change-password-submitting = Menjam…
change-password-done = Lozinka promenjena. Tvoji ostali uređaji su odjavljeni.
error-rotation-incomplete = Tvoj nalog se u međuvremenu promenio. Pokušaj ponovo.
error-key-lifecycle-unavailable = Još nije dostupno: prvo svima treba najnovija verzija aplikacije.
settings-compromised = Neko možda zna moju lozinku
settings-compromised-hint = Nova lozinka, novi ključevi i odjava svih ostalih uređaja.
settings-compromised-open = Zaštiti moj nalog
compromised-title = Zaštiti svoj nalog
compromised-body = Tvoja lozinka i ključevi naloga biće zamenjeni, a svi ostali uređaji odjavljeni. Tvoji prijatelji videće da se tvoj sigurnosni kod promenio: ponovo ga uporedite.
compromised-keeps = Osoba sa tvojom lozinkom zadržava sve što je već otvoreno: projekte koji su dosad bili čitljivi.
compromised-submit = Zameni lozinku i ključeve
compromised-done = Tvoj nalog je zaštićen. Zamoli prijatelje da uporede tvoj novi sigurnosni kod.
friends-new-fingerprint = Novi kod:
friends-code-matches = Kod se podudara
friends-code-trusted = Kod potvrđen.
settings-recovery-key = Ključ za oporavak
settings-recovery-key-none = Bez njega te zaboravljena lozinka košta podešavanja, podataka za plaćanje i kodova prijatelja, a oporavak traje 3 dana.
settings-recovery-key-set = Napravljen. Ako zaboraviš lozinku, vraća ti nalog sa svim sadržajem.
settings-recovery-key-pending = Novi ključ ga zamenjuje { $when }. Nisi bio ti? Otkaži trenutnim ključem.
recovery-key-create = Napravi
recovery-key-replace = Zameni
recovery-key-cancel-replacement = Otkaži novi ključ
recovery-key-title = Tvoj ključ za oporavak
recovery-key-body = Zapiši ga ili sačuvaj tamo gde samo ti možeš da dođeš. Counted ne može ponovo da ga prikaže niti da ga vrati umesto tebe.
recovery-key-copy = Kopiraj
recovery-key-copied = Kopirano
recovery-key-save = Sačuvaj kao datoteku
recovery-key-print = Odštampaj
recovery-key-next = Nastavi
recovery-key-confirm-body = Za proveru kopije upiši grupu { $n } ključa.
recovery-key-confirm-wrong = Ta se grupa ne podudara. Ponovo pogledaj kopiju.
recovery-key-password-body = Upiši lozinku da sačuvaš ključ na nalog.
recovery-key-submit = Sačuvaj ključ
recovery-key-done = Ključ za oporavak sačuvan.
recovery-key-replacement-done = Novi ključ sačuvan. Zamenjuje trenutni za 72 sata; do tada trenutni još radi.
recovery-key-field = Ključ za oporavak
recovery-key-file-intro = Ključ za oporavak za Counted
recovery-key-cancel-done = Novi ključ je otkazan.
compromised-new-key = Sačuvaj novi ključ za oporavak sada: zamenjuje stari za 72 sata, a do tada stari još radi.
pending-reset-title = Zatraženo je ponovno postavljanje tvog naloga
pending-reset-body = Stupa na snagu { $when }: tada se gube podešavanja, podaci za plaćanje i kodovi prijatelja, a uređaji moraju ponovo da se prijave. Nisi bio ti? Otkaži trenutnom lozinkom.
pending-reset-cancel = Otkaži ponovno postavljanje
pending-reset-cancelled = Ponovno postavljanje je otkazano.
login-forgot = Zaboravljena lozinka?
forgot-title = Zaboravljena lozinka
forgot-body = Upiši svoju e-poštu. Ako ima nalog, šaljemo link za izbor nove lozinke.
forgot-submit = Pošalji link
forgot-sent = Ako ova adresa ima nalog, link je na putu. Ističe za 24 sata.
reset-title = Vrati svoj nalog
reset-with-key-body = Upiši ključ za oporavak i novu lozinku: sve se vraća kako je bilo.
reset-submit-key = Vrati moj nalog
reset-no-key = Nemam ključ za oporavak
reset-without-key-title = Ponovo postavi bez ključa za oporavak
reset-without-key-body = Tvoj nalog počinje ispočetka s novom lozinkom za 72 sata. Projekti se vraćaju s tvojih uređaja i linkova za deljenje; podešavanja, podaci za plaćanje i kodovi prijatelja ne. Do tada trenutna lozinka radi i može da otkaže ponovno postavljanje.
reset-name = Tvoje ime
reset-submit-request = Ponovo postavi moj nalog
reset-requested = Gotovo. Nova lozinka radi od { $when }. Do tada trenutna lozinka radi i može to da otkaže.
reset-pending-already = Ponovno postavljanje već čeka do { $when }. Novi zahtev pokreće 72 sata ispočetka.
recovered-done = Nalog je vraćen. Ostali uređaji su odjavljeni.
login-reset-undoable = Ovu lozinku je u poslednjih 30 dana zamenilo ponovno postavljanje. Ako ga nisi tražio, poništi ga: nalog se vraća kakav je bio.
login-undo-reset = Poništi ponovno postavljanje
reset-undone = Ponovno postavljanje je poništeno. Nalog je kao pre.
error-reset-pending = Nova lozinka radi kad ponovno postavljanje stupi na snagu, 72 sata nakon zahteva.
error-account-disabled = Ovaj nalog je izbrisan ubrzo nakon ponovnog postavljanja i čeka brisanje. Prethodna lozinka još može da poništi ponovno postavljanje.
error-recovery-key-invalid = Ovaj ključ za oporavak ne otvara ovaj nalog.
error-project-key-required = Ovaj projekat treba svoj ključ. Ponovo ga otvori preko linka za deljenje.
names-or = { $first } ili { $last }
participants-admin-badge = Admin
edit-project-admins = Admini: { $count }
edit-project-succession = Ako u projektu ne ostane nijedan admin, ulogu preuzima osoba sa nalogom koja je najduže u projektu.
member-sheet-account = Član sa nalogom
member-actions = Radnje: { $name }
member-make-admin = Dodeli ulogu admina
member-make-admin-hint = Dobija mogućnost da promeni ključ projekta i ukloni članove.
member-made-admin = { $name } sada ima ulogu admina.
group-security = Bezbednost projekta
group-security-admin = Bezbednost projekta · imaš ulogu admina
group-rotate = Promeni ključ projekta
group-rotate-hint = Ako je link projekta procureo: oni koji imaju stari link gube pristup projektu.
group-remove = Ukloni nekoga iz projekta
group-remove-hint = Oduzima pristup i istovremeno menja ključ.
group-admins-only = Samo za admine.
group-admins-only-account = Samo za admine, koji imaju nalog.
group-ask-admin = Pitaj admina projekta: { $names }.
group-ask-admins = Pitaj admine projekta: { $names }.
group-ask-an-admin = Pitaj nekog od admina projekta.
group-with-account = Sa nalogom
group-free = Besplatno
group-sign-in-hint = Admin projekta ({ $names }) ti zatim može dodeliti ulogu.
group-sign-in-hint-any = Admin projekta ti zatim može dodeliti ulogu.
group-no-admin = Ovaj projekat nema admina, pa se njegov ključ ne može promeniti.
group-hide = Sakrij nekoga
group-hide-hint = Skriva osobu sa spiskova bez oduzimanja pristupa: link i ključ i dalje rade.
group-recreate-hint = Ovaj projekat nema admina. Da nekoga ukloniš, napravi novi projekat.
group-recreate = Napravi novi projekat
group-no-admin-yet = Ovaj projekat još nema admina.
group-creator-hint = Samo ti možeš da postaneš admin ovog projekta, ako napraviš nalog.
group-creator-elsewhere-hint = Samo osoba koja je napravila projekat može da postane njegov admin, ako napravi nalog. U suprotnom, napravi novi projekat da nekoga ukloniš.
member-revoke-admin = Oduzmi ulogu admina
member-revoke-admin-hint = Više neće moći da menja ključ projekta niti da uklanja članove.
member-revoked-admin = { $name } više nema ulogu admina.
member-step-down = Prestani da budeš admin
member-step-down-hint = Više nećeš moći da menjaš ključ projekta niti da uklanjaš članove.
member-stepped-down = Više nemaš ulogu admina.
member-remove = Ukloni iz projekta
member-remove-hint = Menja ključ projekta: { $name } više neće moći da čita ništa novo.
member-hide = Sakrij sa spiskova
error-user-not-admin = Ovaj član nije admin.
error-creator-proof-invalid = Ovaj uređaj nije napravio ovaj projekat.
error-project-has-admin = Ovaj projekat već ima admina.
error-not-project-admin = To mogu da urade samo admini projekta.
error-user-not-claimed = Ova osoba nema nalog, pa ne može da bude admin.
error-project-key-rotated = Ključ projekta je promenjen. Zatraži novi link od člana.
error-project-rotating = Ključ projekta se upravo menja. Pokušaj ponovo za trenutak.
error-rotation-expired = Promena ključa je trajala predugo i otkazana je. Počni ispočetka.
error-rotation-rows-changed = Projekat se promenio tokom promene ključa. Počni ispočetka.
project-rotated-title = Više nemaš pristup ovom projektu
project-rotated-body = Ključ ovog projekta je promenjen. Zatraži novi link od admina projekta.
project-rotated-open-link = Otvori link
rotation-remove-title = Ukloniti { $name } iz projekta?
rotation-rotate-title = Promeniti ključ projekta?
rotation-remove-body = Uklanjanje člana znači pravljenje novog ključa. Novi link će trebati podeliti sa osobama bez naloga.
rotation-rotate-body = Ko ima stari link, gubi pristup projektu. Novi link će trebati podeliti sa osobama bez naloga.
rotation-boxed =
    { $count ->
        [0] Nijedan drugi član neće automatski dobiti ključ
        [one] Ključ će automatski dobiti { $count } član
        [few] Ključ će automatski dobiti { $count } člana
       *[other] Ključ će automatski dobiti { $count } članova
    }
rotation-by-link-title = Osobe kojima ćeš morati da pošalješ novi link
rotation-by-link-hint = Te osobe nemaju nalog. Poslaćeš im link odmah nakon toga.
rotation-names-and = { $first } i { $last }
rotation-names-and-others = { $names } i još { $count }
rotation-confirm-remove = Ukloni i promeni ključ
rotation-confirm-rotate = Promeni ključ
rotation-done-title = Novi ključ važi
rotation-done-removed = { $name } više nije deo projekta.
rotation-done-rotated = Ključ projekta je promenjen.
rotation-done-boxed =
    { $count ->
        [0] Ključ nije automatski poslat nijednom drugom članu.
        [one] Ključ je automatski poslat { $count } članu.
        [few] Ključ je automatski poslat { $count } članovima.
       *[other] Ključ je automatski poslat { $count } članovima.
    }
rotation-new-link =
    { $count ->
        [0] Novi link projekta
        [one] Novi link za slanje · { $count } osoba bez naloga
        [few] Novi link za slanje · { $count } osobe bez naloga
       *[other] Novi link za slanje · { $count } osoba bez naloga
    }
rotation-link-renewed = Link je uspešno obnovljen.
rotation-done = Gotovo
history-key-changed = Ključ projekta promenjen
history-member-removed = { $name } više nije deo projekta
project-rotated-row-hint = Ključ je promenjen. Zatraži novi link.
