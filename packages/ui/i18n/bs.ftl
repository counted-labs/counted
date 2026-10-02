# Bosanski. Potpun osim pravnih tekstova (legal-, terms-, privacy-), koji postoje samo na engleskom
# i francuskom i za svaku poruku zasebno padaju na en.ftl.

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
nav-settings = Postavke

### Connectivity

offline-banner = Van mreže
offline-pending =
    { $count ->
        [one] { $count } na čekanju
        [few] { $count } na čekanju
       *[other] { $count } na čekanju
    }

sync-conflict-edit = Sukob: uređivanje „{ $name }” nije uspjelo (stavka obrisana). Preskočeno.
sync-conflict-delete = Sukob: brisanje „{ $name }” nije uspjelo (stavka obrisana). Preskočeno.
sync-conflict-other = Sukob: operacija na „{ $name }” nije uspjela (stavka obrisana). Preskočeno.
sync-error = Greška sinhronizacije: { $reason }

### Errors

error-network = Server nije dostupan. Provjeri internet vezu.
error-generic = Nešto je pošlo po zlu. Pokušaj ponovo.

error-invalid-email = Ta e-mail adresa nije važeća.
error-invalid-password = Ta lozinka nije važeća.
error-password-too-short = Lozinka mora imati najmanje 8 znakova.
error-client-outdated = Ova verzija aplikacije je zastarjela. Ažuriraj je za prijavu.
error-invalid-link = Ovaj link nije važeći.
error-batch-too-large = Previše stavki odjednom.
error-payers-required = Odaberi barem jednog platioca.
error-debtors-required = Odaberi barem jednu osobu koja duguje.
error-duplicate-participant = Učesnik se pojavljuje dvaput na istoj strani.
error-participant-not-in-project = Taj učesnik nije dio ovog projekta.
error-too-many-participants = Previše učesnika za jedan trošak.
error-invalid-credentials = Netačan e-mail ili lozinka.
error-unauthenticated = Prijavi se za tu radnju.
error-email-not-verified = Tvoja e-mail adresa još nije potvrđena.
error-project-not-found = Ovaj projekat više ne postoji.
error-expense-not-found = Ovaj trošak više ne postoji.
error-storage-full = Pohrana je puna: ključ ovog projekta nije se mogao sačuvati na ovom uređaju. Sačuvaj link za dijeljenje.
error-user-not-found = Ovaj učesnik više ne postoji.
error-tricount-not-found = Tricount nije pronađen ili je njegov API vratio grešku.
error-too-many-members = Ovaj projekat je dostigao ograničenje broja članova.
error-identity-taken = Drugi račun je već preuzeo ovog učesnika.
error-claim-proof-invalid = Ovaj uređaj nema ključ projekta pa ne može preuzeti učesnika. Ponovo otvori link za dijeljenje.
error-user-has-payments = Ovaj učesnik ima troškove u projektu i ne može se ukloniti.
error-resend-cooldown = Sačekaj 60 sekundi prije traženja novog e-maila.
error-self-friend-request = Ne možeš dodati sebe kao prijatelja.
error-not-a-friend = Možeš pozvati samo osobe sa liste prijatelja.
error-friend-has-no-key = Ovaj prijatelj još nije otvorio najnoviju verziju aplikacije. Zamoli ga da se jednom prijavi, pa pokušaj ponovo.
error-friend-request-not-found = Ovaj zahtjev za prijateljstvo više ne postoji.
error-invitation-not-found = Ova pozivnica više ne postoji.
error-too-many-friend-requests = Zasad previše zahtjeva za prijateljstvo. Pokušaj sutra.
error-too-many-invitations = Previše pozivnica na čekanju.
error-invalid-kdf-salt = Postavke šifriranja nisu važeće. Ažuriraj aplikaciju i pokušaj ponovo.
error-mixed-project-batch = Ovi učesnici nisu svi u istom projektu.
error-invalid-payload = Ova verzija aplikacije je poslala podatke koje server ne prihvata. Ažuriraj je i pokušaj ponovo.
error-invalid-public-key = Tvoj ključ za šifriranje nije važeći. Ažuriraj aplikaciju i pokušaj ponovo.
error-payment-methods-stale = Tvoji podaci za plaćanje su promijenjeni na drugom uređaju. Osvježi i pokušaj ponovo.

### Auth

field-email = E-mail
field-email-placeholder = ti@primjer.ba
field-password = Lozinka
field-name = Ime
field-name-placeholder = Amina Hodžić

login-title = Prijava
login-submit = Prijavi se
login-submitting = Prijava…
login-password-placeholder = Tvoja lozinka
login-no-account = Još nemaš račun?
login-unverified = Tvoja e-mail adresa još nije potvrđena. Provjeri sanduče ili ponovo pošalji link.
login-resend = Ponovo pošalji link za potvrdu
login-resending = Slanje…
login-resend-sent = E-mail poslan - provjeri sanduče.

register-submit = Kreiraj račun
register-submitting = Kreiranje…
register-have-account = Već imaš račun?
register-password-placeholder = Najmanje 8 znakova
register-password-warning = Zapiši svoju lozinku. Ako je zaboraviš, račun se ne može oporaviti.
register-check-email-title = Provjeri e-mail
register-email-sent = E-mail poslan
register-email-sent-hint = Klikni link u sandučetu za aktivaciju računa.
register-not-received-prefix = Nisi ga dobio? Provjeri spam ili se
register-sign-in-link = prijavi
register-not-received-suffix = da ponovo pošalješ link.
register-terms-prefix = Kreiranjem računa prihvataš naše
register-terms-link = uslove korištenja
register-terms-and = i našu
register-privacy-link = politiku privatnosti

settings-title = Postavke
settings-preferences = Postavke
settings-preferences-local = Sačuvano na ovom uređaju.
settings-preferences-synced = Sinhronizovano s tvojim računom, šifrovano.
settings-about = O aplikaciji
settings-anonymous-title = Nisi prijavljen
settings-upsell-title = Tvoji projekti na svakom uređaju
settings-upsell-free = Besplatno
settings-upsell-body = Counted radi i bez računa. S besplatnim računom tvoji projekti i postavke prate te na telefon, laptop i web - i dalje šifrovani, i dalje nečitljivi nama.
settings-locked-badge = Račun
settings-locked-friends = Kreiraj račun za dodavanje prijatelja i pozivanje u projekat iz aplikacije - bez dijeljenja linka.
settings-locked-payment-methods = Sačuvaj svoj IBAN ili aplikaciju za plaćanje jednom i podijeli ih s projektima po izboru. Ko ti duguje, vidi ih pored tvog imena.
settings-friends-hint = Dodaj prijatelje i pozovi ih u svoje projekte bez dijeljenja linka.

account-member-since = Član od
account-logout = Odjava
account-logging-out = Odjava…
account-delete-title = Obriši moj račun
account-delete-warning = Trenutno i trajno, bez korpe za smeće. Troškovi koje si unio u dijeljeni projekat ostaju vidljivi ostalim članovima - dio su njihovih računa.
account-delete-confirm-title = Obriši račun
account-delete-confirm-message = Tvoj račun, sesije i lista projekata bit će trajno obrisani. Bez tvoje lozinke šifrovani podaci dijeljenog projekta postaju ti nečitljivi - to se ne može poništiti.

settings-payment-methods = Podaci za plaćanje
settings-payment-methods-hint = Kako želiš da ti se vrati novac. Šifrovano s tvojim računom.
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
payment-method-too-long = To je predugo - skrati.
payment-method-invalid-characters = Ukloni prelome redova ili nevidljive znakove.
payment-method-limit = Možeš sačuvati do { $max } načina plaćanja.
payment-methods-saved = Podaci za plaćanje sačuvani.
payment-methods-offline = Moraš biti na mreži da sačuvaš podatke za plaćanje.
payment-methods-stale = Tvoji podaci za plaćanje promijenjeni su na drugom uređaju. Ponovo su učitani — pokušaj ponovo.
payment-methods-key-missing = Prijavi se ponovo za upravljanje podacima za plaćanje.
settings-payment-methods-share-warning = Podijeljeni način vidljiv je svakom članu projekata u kojima si odabrao svoje ime - svakome ko ima jedan od tih linkova.
payment-method-share = Podijeli s mojim projektima
payment-method-share-hint = Prikazuje se pored tvog imena kad ti neko duguje.
payment-method-copy = Kopiraj { $name }
payment-method-copied = Kopirano.
payment-method-copy-failed = Kopiranje nije uspjelo - označi tekst i kopiraj ga ručno.

verify-email-checking = Potvrđivanje e-mail adrese…
verify-email-welcome = E-mail potvrđen - dobro došao u Counted!
verify-email-back-to-login = Nazad na prijavu

### Project status

project-close = Zatvori
project-archive = Arhiviraj
project-reopen = Ponovo otvori
project-unarchive = Vrati iz arhive
project-sheet-invite = Pozovi
project-sheet-recurring = Ponavljajući
project-sheet-edit = Uredi projekat
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
month-8 = augusta
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
month-short-8 = aug
month-short-9 = sep
month-short-10 = okt
month-short-11 = nov
month-short-12 = dec

### Actions

add = Dodaj
create = Kreiraj
creating = Kreiranje…
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
field-optional = Opcionalno

### Projects

projects-filter-active = Aktivni
projects-filter-all = Svi
projects-count-label = Projekti
projects-empty = Nema projekata
projects-empty-hint = Kreiraj projekat dugmetom ispod
projects-offline-banner = Podaci van mreže - ponovo se poveži za osvježavanje.
demo-banner = Demo projekat - samo za čitanje.
demo-start-own = Pokreni svoj projekat
projects-no-local-data = Nema lokalnih podataka
projects-no-local-data-hint = Prijavi se da prvi put učitaš svoje projekte.
projects-add = Dodaj projekat
projects-create = Kreiraj projekat
projects-join = Pridruži se projektu
projects-import-tricount = Uvezi iz Tricounta
project-actions = Radnje projekta

status-ongoing = U toku
status-closed = Zatvoren
status-archived = Arhiviran

nav-help = Pomoć
nav-privacy = Politika privatnosti
nav-terms = Uslovi korištenja
nav-legal = Pravne informacije

leave-project-title = Napustiti projekat?
leave-project-message = Izgubit ćeš pristup s ovog uređaja. Ako ne ostane nijedan član, projekat i svi njegovi troškovi trajno se brišu.

add-project-title = Novi projekat
add-project-name-label = Naziv projekta
add-project-name-placeholder = Moje putovanje, Cimeri 2024…
add-project-participants = Učesnici
add-project-participant-name = Ime učesnika
add-project-participant-placeholder = Clark Kent
add-project-offline = Ne možeš kreirati projekat van mreže. Ponovo se poveži i pokušaj ponovo.
add-project-name-required = Projekat treba naziv.

join-link-label = Link za dijeljenje
join-link-hint = Link sadrži ključ za dešifrovanje - kopiraj ga cijelog.
join-invalid-link = Taj link nije važeći. Zalijepi cijeli link za dijeljenje, uključujući dio nakon #.
join-wrong-project = Taj link je za drugi projekat.

import-tricount-link-label = Tricount link ili ključ
import-tricount-key-required = Unesi Tricount link ili ključ.
import-tricount-encryption-failed = Šifrovanje nije uspjelo.
import-tricount-unimportable = Ništa nije uvezeno: ovaj Tricount ima članove s Tricount računom ili iznose koji se ne slažu (pogođeni unosi: { $count }).

### Expenses

save = Sačuvaj
saving = Čuvanje…
adding = Dodavanje…
link-copied = Link kopiran
missing-encryption-key = Nedostaje ključ za šifrovanje.
missing-encryption-key-title = Nedostaje ključ za šifrovanje
missing-encryption-key-hint = Link koji si koristio ne sadrži ključ potreban za dešifrovanje ovog projekta. Koristi puni link koji je podijelio onaj ko ga je kreirao.
project-locked-hint = Ovaj uređaj nema ključ ovog projekta. Otvori njegov link za dijeljenje da ga otključaš.
project-unlock = Otključaj
project-no-local-data-hint = Prijavi se da prvi put učitaš podatke ovog projekta.
project-gone-title = Ovaj projekat više ne postoji
project-gone-hint = Obrisan je kad ga je napustio posljednji član. Link za dijeljenje više ne radi, čak i ako ga ponovo otvoriš.

expense-add = Dodaj trošak
transfer-add = Dodaj prijenos
expense-edit-title = Uredi trošak
expense-category = Kategorija
expense-category-auto = Auto · { $emoji }
expense-currency = Valuta iznosa
amount-op-add = Plus
amount-op-subtract = Minus
amount-op-multiply = Puta
amount-op-divide = Podijeljeno
amount-op-equals = Jednako
amount-op-done = Gotovo
expense-rate = Kurs (opcionalno)
expense-rate-hint = Ostavi prazno za kurs Evropske komisije (InforEuro) za { $month }: 1 { $from } = { $rate } { $to }.
expense-rate-invalid = Unesi kurs veći od 0.
expense-rate-unavailable = Automatski kurs nije dostupan - unesi ga ručno.
expense-delete-title = Obriši trošak
expense-delete-message = „{ $name }” bit će trajno obrisan. To se ne može poništiti.
expense-inconsistent-amounts = Iznosi se ne slažu
expenses-empty = Nema troškova
expenses-empty-hint = Počni dodavanjem troškova dugmetom ispod
expenses-show-more = Prikaži više (još { $count })

expense-type-expense = Trošak
expense-type-transfer = Prijenos
expense-type-gain = Prihod
expense-paid-by = platio/la
expense-sent-by = poslao/la
expense-contributed-by = doprinio/la

expense-name-required = Naziv je obavezan.
expense-amount-not-positive = Iznos mora biti veći od 0.
expense-no-payer = Odaberi barem jednog platioca.
expense-no-debtor = Odaberi barem jednu osobu koja duguje.
expense-invalid-date = Taj datum nije važeći.
expense-payers-mismatch = Zbir platilaca je { $sum }, što ne odgovara iznosu troška ({ $total }).
expense-debtors-mismatch = Zbir dužnika je { $sum }, što ne odgovara iznosu troška ({ $total }).

participants-none = Niko
participants-everyone = Svi ({ $count })
participants-some = { $count } od { $total }
participants-select-all = Odaberi sve
participants-by-shares = Po udjelima
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
reimbursements-empty-hint = Prijedlozi za izmirenje pojavljuju se ovdje kad računi nisu u ravnoteži
reimbursement-owes = { $debtor } duguje { $creditor }
reimbursement-record = Izmiri
reimbursement-pay-with = Plati
reimbursement-pay-shared-by = Podijelio { $name } - provjeri ime primaoca koje prikazuje tvoja aplikacija prije slanja.
reimbursement-pay-title = Plati { $name }
reimbursements-mine-title = Ti duguješ
reimbursements-others-title = Ostali povrati
copy = Kopiraj

user-selection-title = Koji si učesnik?
user-selection-hint = Odaberi svoje ime s liste.
user-selection-required = Odaberi učesnika.
identity-claimed = Povezano s računom
identity-claimed-by = Račun { $name }
identity-taken-repick = Drugi račun je preuzeo učesnika kojeg si koristio. Odaberi drugog.
participant-gone-repick = Učesnik kojeg si koristio uklonjen je iz ovog projekta. Odaberi drugog.

edit-project-title = Uredi projekat
edit-project-new-badge = novo
edit-project-deferred-new-members = dodavanje novih članova
edit-project-deferred-removals = uklanjanje članova
edit-project-offline-deferred = Van mreže: { $items } primijenit će se pri ponovnom povezivanju.

export-failed = Izvoz nije uspio: { $reason }

history-expense-added = Trošak dodan: { $name }
history-expense-edited = Trošak uređen: { $name }
history-expense-deleted = Trošak obrisan: { $name }
history-project-edited = Projekat uređen: { $name }
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
expense-inconsistent-detail = Iznosi se ne slažu: { $paid } plaćeno, { $owed } dugovano, za trošak od { $total }. Uredi trošak da to ispraviš.
missing-access-key = Nedostaje pristupni ključ. Otvori ovaj projekat putem njegovog linka za dijeljenje.
filter-all = Sve
filter-my-payments = Moja plaćanja
filter-my-debts = Šta dugujem
participants-shares-for = Udjeli za { $name }
participants-amount-for = Iznos za { $name }
reimbursement-add = Dodaj izmirenje
project-forget = Ukloni s moje liste
project-history-title = Historija
history-kind-add = Dodano
history-kind-delete = Obrisano
history-kind-edit = Uređeno
export = Izvezi
export-json = Izvezi JSON
export-csv = Izvezi CSV
share-link = Podijeli
copy-link-failed = Link se nije mogao kopirati
open-in-app = Otvori u aplikaciji
not-found-title = Stranica nije pronađena
not-found-back = Nazad na projekte

### Charts

charts-period = Period
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
charts-nothing-to-show = Nema šta prikazati
charts-my-share-note = Ovi iznosi su tvoj udio u svakom trošku.
charts-my-share-skipped =
    { $count ->
        [one] 1 projekat nije uračunat — nije odabran učesnik ili se njegovi podaci nisu učitali.
        [few] { $count } projekta nisu uračunata — nije odabran učesnik ili se njihovi podaci nisu učitali.
       *[other] { $count } projekata nije uračunato — nije odabran učesnik ili se njihovi podaci nisu učitali.
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
charts-my-share = Moj udio
charts-share-of-total = { $pct } % od { $total }
charts-i-paid = Platio sam
charts-paid-more = { $amount } više od tvog udjela
charts-paid-less = { $amount } manje od tvog udjela
charts-paid-even = Tačno tvoj udio
charts-part-title = Tvoj dio svake kategorije
charts-part-desc = Sivo je šta je grupa potrošila, boja je šta si ti potrošio.
charts-breakdown-title = Raspodjela po kategorijama
charts-breakdown-desc = Dodirni isječak ili red za spisak troškova.
charts-of-total = { $amount } od { $total }
charts-show-all = Prikaži sve ({ $count })
charts-show-less = Prikaži manje
charts-spend-title = Troškovi kroz vrijeme
charts-spend-desc = Kratki periodi po danima, duži po sedmicama ili mjesecima.
charts-group-by = Grupiši po
bucket-day = Dan
bucket-week = Sedmica
bucket-month = Mjesec
charts-avg = prosj.
charts-cat-title-day = { $category }, dan po dan
charts-cat-title-week = { $category }, sedmicu po sedmicu
charts-cat-title-month = { $category }, mjesec po mjesec
charts-cat-desc = Odaberi kategoriju i prati je kroz vrijeme.
charts-running-title = Ukupno do sada
charts-running-desc = Od { $date }.
charts-avg-per-day = { $amount } / dan u prosjeku
charts-avg-per-week = { $amount } / sedmica u prosjeku
charts-avg-per-month = { $amount } / mjesec u prosjeku
charts-people-title = Ko je nosio grupu
charts-people-desc = Šta je ko platio, pored onoga što je potrošio.
charts-paid = Plaćeno
charts-fair-share = Pošten udio
charts-you = (ti)
charts-net-more = platio više
charts-net-less = platio manje
charts-balance-title = Tvoje stanje kroz vrijeme
charts-balance-desc = Iznad linije grupa duguje tebi. Ispod nje ti duguješ grupi.
charts-owed = Duguju ti
charts-owe = Duguješ
charts-projects-title = Tvoj udio po projektu
charts-projects-desc = Zbirovi se vode po valuti i nikad se ne sabiraju.
history-empty = Nema događaja
history-by = { $name }
not-found-hint = Ova stranica ne postoji ili je premještena.
payers-title-paid-by = Platio/la
payers-title-sender = Pošiljalac
payers-title-contributors = Učesnici
debtors-title-debtors = Duguje
debtors-title-recipients = Primaoci
debtors-title-beneficiaries = Korisnici

### Welcome

welcome-title = Tvoji računi nisu ničija briga.
welcome-subtitle = Dijeli troškove s prijateljima.
welcome-note = Besplatno. Bez računa. Bez reklama.
welcome-link-title = Jedan link i svi učestvuju.
welcome-link-body = Niko ne mora praviti račun.
welcome-link-account = Račun? Nikad obavezan. Služi da pronađeš svoje projekte na drugom uređaju, pozoveš prijatelje iz aplikacije i podijeliš svoje podatke za plaćanje.
welcome-demo-project = Vikend u Lyonu
welcome-private-title = Niko ne može čitati tvoje račune. Čak ni mi.
welcome-private-body = Imena, iznosi, projekti: sve se šifruje na tvom uređaju. Samo ti imaš ključ.
welcome-private-names = Imena
welcome-private-amounts = Iznosi
welcome-private-projects = Projekti
welcome-scan-title = Fotografiši račun.
welcome-scan-body = Iznos, datum i kategorija popune se sami. Sve se dešava na tvom telefonu. Fotografija se ne čuva.
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
help-create-project-q = Kako kreirati projekat?
help-create-project-a = Na početnom ekranu dodirni dugme + pri dnu. Daj projektu naziv, odaberi valutu i gotovo.
help-add-participants-q = Kako dodati učesnike?
help-add-participants-a = Otvori projekat i dodaj učesnike s liste članova. Svaki učesnik može platiti ili dugovati za trošak.
help-share-project-q = Kako podijeliti projekat?
help-share-project-a = Podijeli URL projekta (onaj u adresnoj traci). Svako s linkom može pregledati i uređivati projekat.
help-add-expense-q = Kako dodati trošak?
help-add-expense-a = U projektu dodirni +, unesi iznos, navedi ko je platio i među kime podijeliti. Možeš odabrati i datum različit od današnjeg.
help-types-q = Koja je razlika između troška, prijenosa i prihoda?
help-types-expense = - kupovina koju je obavila jedna osoba i podijelila među više njih.
help-types-transfer = - povrat od jedne osobe drugoj, bez podjele.
help-types-gain = - primljeni novac (povrat, poklon) za podjelu među više osoba.
help-past-date-q = Mogu li datirati trošak u prošlost?
help-past-date-a = Da, polje datuma je slobodno. Vrijeme kreiranja zapisa čuva se zasebno.
help-who-owes-q = Kako Counted računa ko šta duguje?
help-who-owes-a = Counted računa neto stanje svakog učesnika (šta je platio unaprijed minus šta duguje), a zatim predlaže najkraći niz prijenosa koji sve izmiruje.
help-minimal-transfers-q = Zašto je broj predloženih prijenosa minimalan?
help-minimal-transfers-a = Algoritam prvo uparuje stanja koja se tačno poništavaju, a zatim prolazi kroz ostatak od najvećeg povjerioca do najvećeg dužnika. Rezultat: manje prijenosa da se sve izmiri.
help-import-tricount-q = Kako uvesti projekat iz Tricounta?
help-import-tricount-a = Na početnom ekranu dodirni dugme „+” pri dnu, a zatim
help-import-tricount-b = Zalijepi link za dijeljenje Tricounta koji želiš uvesti.
help-encryption-q = Jesu li moji podaci šifrovani?
help-encryption-a = Da. Counted kombinuje dvije garancije:
help-encryption-e2ee-term = Šifrovanje s kraja na kraj
help-encryption-e2ee-def = - sve između tebe i servera putuje šifrovano.
help-encryption-zero-term = Nulti pristup
help-encryption-zero-def = - podatke šifruješ prije slanja, a server pohranjuje samo šifrovani tekst. Nemamo ga kako pročitati.
help-encryption-see = Za detalje pogledaj
help-forgot-password-q = Šta ako zaboravim lozinku?
help-forgot-password-warning = Tvoji podaci bit će trajno izgubljeni.
help-forgot-password-a = Ključ za šifrovanje izvodi se iz tvoje lozinke pa resetovanje nije moguće: niko - ni mi - ne može dešifrovati tvoje projekte bez nje. Čuvaj je na sigurnom, idealno u upravitelju lozinki.
help-archive-delete-q = Kako arhivirati ili obrisati projekat?
help-archive-delete-a = Na ekranu projekta otvori meni i odaberi
help-archive-delete-b = da ga sakriješ, a zadržiš. Projekat se trajno briše kad ga napusti posljednji član.
help-delete-account-q = Kako obrisati račun?
help-delete-account-a = Otvori Postavke i koristi „Obriši moj račun”. Trenutno je i ne može se poništiti.
help-contact = Još pitanja? Piši nam na

# Receipt scanning (mobile only)
expense-scan = Skeniraj račun
scan-in-progress = Čitanje računa…
scan-error-capture = Fotografija nije uspjela. Pokušaj ponovo ili unesi trošak ručno.
scan-error-unreadable = Ništa čitljivo na tom računu. Unesi trošak ručno.
scan-check-amount = Provjeri ukupan iznos - nije bio jasno odštampan.
scan-take-photo = Snimi fotografiju
scan-choose-photo = Odaberi fotografiju
expense-converted-from = Plaćeno { $amount } { $from } · 1 { $from } = { $rate } { $to }
project-currency = Valuta
project-currency-hint = Svaki iznos prikazuje se u ovoj valuti. Ne može se kasnije promijeniti.
project-currency-locked = Valuta se određuje pri kreiranju projekta.
currency-search = Pretraži valutu

update-required-title = Potrebno ažuriranje
update-required-body = Ova verzija Counteda prestara je za komunikaciju sa serverom. Ažuriraj je da nastaviš koristiti aplikaciju.
update-required-button = Ažuriraj

notifications-label = Obavještenja
notifications-title = Obavještenja
notifications-empty = Ništa novo
notifications-friend-request = Zahtjev za prijateljstvo

friends-title = Prijatelji
friends-anonymous-body = Prijatelji su vezani uz tvoj račun. Prijavi se za dodavanje osoba i pozivanje u projekte bez dijeljenja linka.
friends-add-title = Dodaj prijatelja
friends-add-hint = Vidjet će tvoj zahtjev kad se prijavi. Niko od vas ne saznaje ima li drugi račun dok zahtjev nije prihvaćen.
friends-add-button = Dodaj
friends-add-from-project = Dodaj kao prijatelja
friends-request-sent = Zahtjev poslan
friends-no-account-key = Prijavi se ponovo za upravljanje prijateljima na ovom uređaju.
friends-incoming-title = Zahtjevi
friends-accept = Prihvati
friends-decline = Odbij
friends-list-title = Moji prijatelji
friends-list-empty = Još nema prijatelja. Dodaj nekoga e-mailom iznad ili iz projekta koji dijelite.
friends-remove = Ukloni
friends-remove-confirm-title = Ukloni prijatelja
friends-remove-confirm-message = { $email } više neće biti među vašim prijateljima, a ni vi među njegovima. Bilo ko od vas može kasnije poslati novi zahtjev.
friends-no-key = Još nije spremno
friends-fingerprint = Sigurnosni kod
friends-fingerprint-hint = Dva prijatelja koja jedno drugom pročitaju isti sigurnosni kod znaju da niko ne stoji između njih - čak ni naš server.
friends-outgoing-title = Poslano
friends-outgoing-hint = Čeka se odgovor. Vidjet ćeš ih među prijateljima kad prihvate.
friends-withdraw = Otkaži
invite-friends-title = Pozovi prijatelje
invite-friends-hint = Ključ projekta šifruje se za svakog prijatelja na ovom uređaju. Server ga nikad ne vidi.
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
participants-you-from-account = Preuzeto iz imena tvog računa. Ovdje ga promijeni samo za ovaj projekat.
participants-you-required = Obavezno. Ovako će te vidjeti ostali.
participants-others = Ostali učesnici
participants-empty = Još nikoga. Odaberi prijatelja ispod ili upiši bilo koje ime.
participants-empty-signed-out = Još nikoga. Upiši ime da nekoga dodaš.
participants-duplicate = „{ $name }” je već na listi.
participants-input-label = Dodaj prijatelja ili upiši ime
participants-input-placeholder = Prijatelj ili bilo koje ime
participants-suggest-friend = Prijatelj · pridružuje se kao „{ $name }”, dobija pozivnicu
participants-suggest-not-ready = Prijatelj · još nije spreman
participants-suggest-guest = Dodaj „{ $text }” bez računa
participants-suggest-guest-sub = Bez računa, samo ime
participants-friends = Tvoji prijatelji
participants-all-friends = Svi prijatelji
participants-login-hint = Prijavi se da dodaješ ljude direktno sa liste prijatelja.
participants-invite-badge = Pozovi
participants-guest-badge = Bez računa
participants-guest-sub = Bez računa, samo ime
participants-rename = Preimenuj: { $name }
participants-remove = Ukloni: { $name }
participants-rename-label = Novo ime
participants-rename-save = Sačuvaj ime
participants-rename-hint = Ime koje svi vide u ovom projektu. Pozivnica i dalje ide na { $email }.
participants-invited-badge = Pozvan
participants-invited-sub = { $email } · još nije prihvaćeno
participants-invited-pending = Pozivnica još nije prihvaćena
participants-unlinked = Nije povezano s računom
add-project-create-invite = Kreiraj i pozovi: { $count }
edit-project-save-invite = Sačuvaj i pozovi: { $count }
edit-project-you-are = Na ovom uređaju ti si { $name }
edit-project-no-identity = Još nisi odabrao/la ko si
edit-project-switch = Promijeni
edit-project-choose = Odaberi
invite-failed = Ove pozivnice nije bilo moguće poslati: { $emails }
invite-again = Pozovi ponovo
friend-picker-title = Dodaj prijatelje
user-selection-invited-hint = { $email } te pozvao/la u „{ $project }”.
user-selection-suggested = Predloženo
user-selection-suggested-sub = { $email } te dodao/la pod ovim imenom
user-selection-confirm-as = Ja sam { $name }
user-selection-missing = Tvog imena nema? Zamoli učesnika da te doda u postavkama projekta.

## Ponavljajući troškovi

repeat-label = Ponavljaj
repeat-none = Ne ponavlja se
repeat-weekly = Svake sedmice
repeat-biweekly = Svake 2 sedmice
repeat-monthly = Svakog mjeseca
repeat-quarterly = Svaka 3 mjeseca
repeat-yearly = Svake godine
repeat-every-weeks = Svakih { $count } sed.
repeat-every-months = Svakih { $count } mj.
repeat-every-years = Svakih { $count } god.
repeat-custom = Prilagođeno…
repeat-every = Svakih
repeat-unit-weeks = sed.
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
repeat-foreign = Ponavlja se kao { $amount } { $currency }, jednom preračunato po današnjem kursu. Dobit ćeš upozorenje ako se kurs promijeni za više od 5 %.
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
recurring-rate-badge = Kurs promijenjen za { $percent } %
recurring-empty = Još se ništa ne ponavlja. Odaberi „Ponavljaj” kad dodaješ trošak: kirija, pretplate, računi.
recurring-next-ones = Sljedeći
recurring-added-so-far = Dosad dodano
recurring-set-up-by = Postavio/la
recurring-pause = Pauziraj
recurring-resume = Nastavi
recurring-stop = Prekini ponavljanje
recurring-stop-title = Prekinuti „{ $name }”?
recurring-stop-message = Više se neće ponavljati. Već dodani troškovi ostaju.
recurring-resume-title = Nastaviti „{ $name }”?
recurring-resume-message = Sljedeći { $date }. Datumi propušteni tokom pauze neće se dodati.
recurring-edit-title = Uredi ponavljajući trošak
recurring-edit-banner = Promjene važe od { $date }. Već dodani troškovi ostaju kakvi jesu.
recurring-next-on-label = Sljedeći
recurring-next-too-early = Sljedeći datum mora biti nakon zadnjeg već dodanog troška.
recurring-use-stop = Za završetak koristi „Prekini ponavljanje” na ponavljajućem trošku.
recurring-drift = Kurs { $currency } promijenio se za { $percent } % od postavljanja. Svaki se i dalje dodaje kao { $amount } { $project_currency } (1 { $currency } = { $rate }). Po današnjem kursu bilo bi { $today_amount } { $project_currency }.
recurring-use-rate = Koristi današnji kurs
recurring-keep = Zadrži { $amount } { $currency }
recurring-added = Dodani ponavljajući troškovi: { $count }
recurring-blocks-removal = { $name } se još ne može ukloniti: dio je { $rules }. Skloni { $name } iz njih ili ih prekini pa ponovo sačuvaj.
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
estimate-title = Iznos za potvrdu.
estimate-body = Dodano s prethodnim iznosom. Unesi pravi kad ga saznaš.
estimate-confirm = Potvrdi iznos
apply-to = Primijeni na
apply-this-only = Samo ovaj trošak
apply-and-next = Ovaj i sljedeće
apply-and-next-hint = Ponavljajući trošak mijenja se od { $date }
apply-rule-failed = Trošak je sačuvan, ali ponavljajući trošak nije promijenjen.
occurrence-delete-message = „{ $name }” od { $date } bit će trajno izbrisan. Ponavljanje se nastavlja i ovaj se datum neće vratiti.
occurrence-delete-one = Izbriši samo ovaj
occurrence-delete-stop = Izbriši i prekini ponavljanje
error-recurring-clock = Sat ovog uređaja žuri. Provjeri datum i vrijeme.
error-recurring-not-found = Ovaj ponavljajući trošak više ne postoji.
error-recurring-stale = Neko je u međuvremenu promijenio ovaj ponavljajući trošak. Ponovo je učitan: provjeri ga i ponovo sačuvaj.
error-too-many-recurring = Ovaj projekat već ima 50 ponavljajućih troškova. Prekini jedan koji ti više ne treba da bi dodao novi.
error-user-in-recurring = Ovaj učesnik je dio ponavljajućeg troška. Prvo ga skloni iz njega ili prekini trošak.
