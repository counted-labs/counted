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
error-generic = Nešto je pošlo naopako. Pokušaj ponovo.

error-invalid-email = Ta imejl adresa nije važeća.
error-invalid-password = Ta lozinka nije važeća.
error-password-too-short = Lozinka mora imati najmanje 8 znakova.
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
login-resend = Ponovo pošalji link za potvrdu
login-resending = Slanje…
login-resend-sent = Imejl poslat - proveri sanduče.

register-submit = Napravi nalog
register-submitting = Pravljenje…
register-have-account = Već imaš nalog?
register-password-placeholder = Najmanje 8 znakova
register-password-warning = Zapiši svoju lozinku. Ako je zaboraviš, nalog ne može da se povrati.
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
account-delete-warning = Trenutno i trajno, bez korpe za otpatke. Troškovi koje si uneo u deljeni projekat ostaju vidljivi ostalim članovima - deo su njihovih računa.
account-delete-confirm-title = Obriši nalog
account-delete-confirm-message = Tvoj nalog, sesije i lista projekata biće trajno obrisani. Bez tvoje lozinke šifrovani podaci deljenog projekta postaju ti nečitljivi - to ne može da se poništi.

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
payment-methods-stale = Tvoji podaci za plaćanje promenjeni su na drugom uređaju. Ponovo su učitani — pokušaj ponovo.
payment-methods-key-missing = Prijavi se ponovo da upravljaš podacima za plaćanje.
settings-payment-methods-share-warning = Podeljeni način vidljiv je svakom članu projekata u kojima si izabrao svoje ime - svakome ko ima jedan od tih linkova.
payment-method-share = Podeli s mojim projektima
payment-method-share-hint = Prikazuje se pored tvog imena kad ti neko duguje.
payment-method-copy = Kopiraj { $name }
payment-method-copied = Kopirano.
payment-method-copy-failed = Kopiranje nije uspelo - označi tekst i kopiraj ga ručno.

verify-email-checking = Potvrđivanje imejl adrese…
verify-email-welcome = Imejl potvrđen - dobro došao u Counted!
verify-email-back-to-login = Nazad na prijavu

### Project status

project-close = Zatvori
project-archive = Arhiviraj
project-reopen = Ponovo otvori
project-unarchive = Vrati iz arhive

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
projects-empty-hint = Napravi projekat dugmetom ispod
projects-offline-banner = Podaci van mreže - ponovo se poveži za osvežavanje.
projects-no-local-data = Nema lokalnih podataka
projects-no-local-data-hint = Prijavi se da prvi put učitaš svoje projekte.
projects-add = Dodaj projekat
projects-create = Napravi projekat
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
project-unlock = Otključaj
project-no-local-data-hint = Prijavi se da prvi put učitaš podatke ovog projekta.
project-gone-title = Ovaj projekat više ne postoji
project-gone-hint = Obrisan je kad ga je napustio poslednji član. Link za deljenje više ne radi, čak i ako ga ponovo otvoriš.

expense-add = Dodaj trošak
transfer-add = Dodaj prenos
expense-edit-title = Izmeni trošak
expense-category = Kategorija
expense-category-auto = Auto · { $emoji }
expense-currency = Valuta iznosa
amount-op-add = Plus
amount-op-subtract = Minus
amount-op-multiply = Puta
amount-op-divide = Podeljeno
amount-op-equals = Једнако
amount-op-done = Готово
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
expense-inconsistent-detail = Iznosi se ne slažu: { $paid } plaćeno, { $owed } dugovano, za trošak od { $total }. Izmeni trošak da to ispraviš.
missing-access-key = Nedostaje pristupni ključ. Otvori ovaj projekat preko njegovog linka za deljenje.
filter-all = Sve
filter-my-payments = Moja plaćanja
filter-my-debts = Šta dugujem
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
charts-my-share-note = Ovi iznosi su tvoj udeo u svakom trošku.
charts-my-share-skipped =
    { $count ->
        [one] 1 projekat nije uračunat — nije izabran učesnik ili se njegovi podaci nisu učitali.
        [few] { $count } projekta nisu uračunata — nije izabran učesnik ili se njihovi podaci nisu učitali.
       *[other] { $count } projekata nije uračunato — nije izabran učesnik ili se njihovi podaci nisu učitali.
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
charts-paid-more = { $amount } više od tvog udela
charts-paid-less = { $amount } manje od tvog udela
charts-paid-even = Tačno tvoj udeo
charts-part-title = Tvoj deo svake kategorije
charts-part-desc = Sivo je šta je grupa potrošila, boja je šta si ti potrošio.
charts-breakdown-title = Raspodela po kategorijama
charts-breakdown-desc = Dodirni isečak ili red za spisak troškova.
charts-of-total = { $amount } od { $total }
charts-show-all = Prikaži sve ({ $count })
charts-show-less = Prikaži manje
charts-spend-title = Troškovi kroz vreme
charts-spend-desc = Kratki periodi po danima, duži po nedeljama ili mesecima.
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
charts-projects-desc = Zbirovi se vode po valuti i nikad se ne sabiraju.
history-empty = Nema događaja
history-by = { $name }
not-found-hint = Ova stranica ne postoji ili je premeštena.
payers-title-paid-by = Platio/la
payers-title-sender = Pošiljalac
payers-title-contributors = Učesnici
debtors-title-debtors = Duguje
debtors-title-recipients = Primaoci
debtors-title-beneficiaries = Korisnici

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
help-forgot-password-warning = Tvoji podaci biće trajno izgubljeni.
help-forgot-password-a = Ključ za šifrovanje izvodi se iz tvoje lozinke pa resetovanje nije moguće: niko - ni mi - ne može da dešifruje tvoje projekte bez nje. Čuvaj je na sigurnom, idealno u menadžeru lozinki.
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
project-currency-hint = Svaki iznos prikazuje se u ovoj valuti. Ne može kasnije da se promeni.
project-currency-locked = Valuta se određuje pri pravljenju projekta.

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
friends-add-hint = Videće tvoj zahtev kad se prijavi. Niko od vas ne saznaje da li drugi ima nalog dok zahtev nije prihvaćen.
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
friends-remove-confirm-message = { $email } više neće biti među vašim prijateljima, a ni vi među njegovima. Bilo ko od vas može kasnije poslati novi zahtev.
friends-no-key = Još nije spremno
friends-fingerprint = Sigurnosni kod
friends-fingerprint-hint = Dva prijatelja koja jedno drugom pročitaju isti sigurnosni kod znaju da niko ne stoji između njih - čak ni naš server.
friends-outgoing-title = Poslato
friends-outgoing-hint = Čeka se odgovor. Videćeš ih među prijateljima kad prihvate.
friends-withdraw = Otkaži
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
participants-you-from-account = Preuzeto iz imena tvog naloga. Ovde ga promeni samo za ovaj projekat.
participants-you-required = Obavezno. Ovako će te videti ostali.
participants-others = Ostali učesnici
participants-empty = Još nikoga. Izaberi prijatelja ispod ili upiši bilo koje ime.
participants-empty-signed-out = Još nikoga. Upiši ime da nekoga dodaš.
participants-duplicate = „{ $name }” je već na listi.
participants-input-label = Dodaj prijatelja ili upiši ime
participants-input-placeholder = Prijatelj ili bilo koje ime
participants-suggest-friend = Prijatelj · pridružuje se kao „{ $name }”, dobija pozivnicu
participants-suggest-not-ready = Prijatelj · još nije spreman
participants-suggest-guest = Dodaj „{ $text }” bez naloga
participants-suggest-guest-sub = Bez naloga, samo ime
participants-friends = Tvoji prijatelji
participants-all-friends = Svi prijatelji
participants-login-hint = Prijavi se da dodaješ ljude direktno sa liste prijatelja.
participants-invite-badge = Pozovi
participants-guest-badge = Bez naloga
participants-guest-sub = Bez naloga, samo ime
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
