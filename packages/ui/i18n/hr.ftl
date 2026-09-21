# Hrvatski. Potpun osim pravnih tekstova (legal-, terms-, privacy-), koji postoje samo na engleskom
# i francuskom i za svaku poruku zasebno padaju na en.ftl.

### Common

loading = Učitavanje…
cancel = Odustani
confirm = Potvrdi
retry = Pokušaj ponovno
delete = Izbriši
back = Natrag
language = Jezik

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
error-generic = Nešto je pošlo po zlu. Pokušaj ponovno.

error-invalid-email = Ta e-mail adresa nije valjana.
error-invalid-password = Ta lozinka nije valjana.
error-password-too-short = Lozinka mora imati najmanje 8 znakova.
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
login-resend = Ponovno pošalji poveznicu za potvrdu
login-resending = Slanje…
login-resend-sent = E-mail poslan - provjeri sandučić.

register-submit = Stvori račun
register-submitting = Stvaranje…
register-have-account = Već imaš račun?
register-password-placeholder = Najmanje 8 znakova
register-password-warning = Zapiši svoju lozinku. Ako je zaboraviš, račun se ne može oporaviti.
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
account-delete-warning = Trenutno i trajno, bez koša za smeće. Troškovi koje si unio u dijeljeni projekt ostaju vidljivi ostalim članovima - dio su njihovih računa.
account-delete-confirm-title = Izbriši račun
account-delete-confirm-message = Tvoj račun, sesije i popis projekata bit će trajno izbrisani. Bez tvoje lozinke šifrirani podaci dijeljenog projekta postaju ti nečitljivi - to se ne može poništiti.

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
payment-methods-stale = Tvoji podaci za plaćanje promijenjeni su na drugom uređaju. Ponovno su učitani — pokušaj ponovno.
payment-methods-key-missing = Prijavi se ponovno za upravljanje podacima za plaćanje.
settings-payment-methods-share-warning = Podijeljeni način vidljiv je svakom članu projekata u kojima si odabrao svoje ime - svakome tko ima jednu od tih poveznica.
payment-method-share = Podijeli s mojim projektima
payment-method-share-hint = Prikazuje se pored tvog imena kad ti netko duguje.
payment-method-copy = Kopiraj { $name }
payment-method-copied = Kopirano.
payment-method-copy-failed = Kopiranje nije uspjelo - označi tekst i kopiraj ga ručno.

verify-email-checking = Potvrđivanje e-mail adrese…
verify-email-welcome = E-mail potvrđen - dobro došao u Counted!
verify-email-back-to-login = Natrag na prijavu

### Project status

project-close = Zatvori
project-archive = Arhiviraj
project-reopen = Ponovno otvori
project-unarchive = Vrati iz arhive

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
field-optional = Neobavezno

### Projects

projects-filter-active = Aktivni
projects-filter-all = Svi
projects-count-label = Projekti
projects-empty = Nema projekata
projects-empty-hint = Stvori projekt gumbom ispod
projects-offline-banner = Podaci izvan mreže - ponovno se spoji za osvježavanje.
projects-no-local-data = Nema lokalnih podataka
projects-no-local-data-hint = Prijavi se da prvi put učitaš svoje projekte.
projects-add = Dodaj projekt
projects-create = Stvori projekt
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
add-project-remove-participant = Ukloni sudionika
add-project-me-badge = Ja
add-project-thats-me = To sam ja!
add-project-offline = Ne možeš stvoriti projekt izvan mreže. Ponovno se spoji i pokušaj ponovno.
add-project-name-required = Projekt treba naziv.
add-project-need-two-participants = Dodaj barem 2 sudionika.
add-project-pick-yourself = Reci nam koji si sudionik.

join-link-label = Poveznica za dijeljenje
join-link-hint = Poveznica sadrži ključ za dešifriranje - kopiraj je cijelu.
join-invalid-link = Ta poveznica nije valjana. Zalijepi cijelu poveznicu za dijeljenje, uključujući dio nakon #.
join-wrong-project = Ta poveznica je za drugi projekt.

import-tricount-link-label = Tricount poveznica ili ključ
import-tricount-key-required = Unesi Tricount poveznicu ili ključ.
import-tricount-encryption-failed = Šifriranje nije uspjelo.

### Expenses

save = Spremi
saving = Spremanje…
adding = Dodavanje…
link-copied = Poveznica kopirana
missing-encryption-key = Nedostaje ključ za šifriranje.
missing-encryption-key-title = Nedostaje ključ za šifriranje
missing-encryption-key-hint = Poveznica koju si koristio ne sadrži ključ potreban za dešifriranje ovog projekta. Koristi punu poveznicu koju je podijelio onaj tko ga je stvorio.
project-locked-hint = Ovaj uređaj nema ključ ovog projekta. Otvori njegovu poveznicu za dijeljenje da ga otključaš.
project-unlock = Otključaj
project-no-local-data-hint = Prijavi se da prvi put učitaš podatke ovog projekta.
project-gone-title = Ovaj projekt više ne postoji
project-gone-hint = Izbrisan je kad ga je napustio posljednji član. Poveznica za dijeljenje više ne radi, čak i ako je ponovno otvoriš.

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
participants-deselect-all = Poništi odabir
participants-by-shares = Po udjelima
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
reimbursements-empty-title = Sve je podmireno!
reimbursements-empty-hint = Prijedlozi za podmirenje pojavljuju se ovdje kad računi nisu u ravnoteži
reimbursement-owes = { $debtor } duguje { $creditor }
reimbursement-record = Podmiri
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
edit-project-deferred-me = odabir „To sam ja”
edit-project-offline-deferred = Izvan mreže: { $items } primijenit će se pri ponovnom spajanju.

export-saved = Datoteka spremljena:
    { $path }
export-failed = Izvoz nije uspio: { $reason }

history-expense-added = Trošak dodan: { $name }
history-expense-edited = Trošak uređen: { $name }
history-expense-deleted = Trošak izbrisan: { $name }
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
expense-inconsistent-detail = Iznosi se ne slažu: { $paid } plaćeno, { $owed } dugovano, za trošak od { $total }. Uredi trošak da to ispraviš.
missing-access-key = Nedostaje pristupni ključ. Otvori ovaj projekt putem njegove poveznice za dijeljenje.
filter-all = Sve
filter-my-payments = Moja plaćanja
filter-my-debts = Što dugujem
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
charts-tab-per-person = Po osobi
charts-tab-trends = Trendovi
charts-by-category = Raspodjela po kategorijama
charts-per-person = Potrošnja po osobi
charts-categories-by-month = Kategorije po mjesecima
charts-total-spent = Ukupno potrošeno
charts-avg-per-person = Prosj. po osobi
charts-expense-count =
    { $count ->
        [one] { $count } trošak
        [few] { $count } troška
       *[other] { $count } troškova
    }
charts-clear-category-filter = Ukloni filtar kategorije
charts-no-expenses = Nema troškova.
charts-pick-a-project = Odaberi projekt za prikaz potrošnje po osobi.
charts-nothing-to-show = Nema što prikazati
charts-my-share-note = Ovi iznosi su tvoj udio u svakom trošku.
charts-my-share-skipped =
    { $count ->
        [one] 1 projekt nije uračunat — nije odabran sudionik ili se njegovi podaci nisu učitali.
        [few] { $count } projekta nisu uračunata — nije odabran sudionik ili se njihovi podaci nisu učitali.
       *[other] { $count } projekata nije uračunato — nije odabran sudionik ili se njihovi podaci nisu učitali.
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
charts-person = Osoba
charts-project = Projekt
charts-all-projects = Svi projekti
charts-whole-project = Cijeli projekt
charts-date-from = Od
charts-date-to = Do
charts-total = Ukupno
charts-payments-per-person-by-month = Plaćanja po osobi po mjesecima
history-empty = Nema događaja
history-by = { $name }
not-found-hint = Ova stranica ne postoji ili je premještena.
payers-title-paid-by = Platio/la
payers-title-sender = Pošiljatelj
payers-title-contributors = Sudionici
debtors-title-debtors = Duguje
debtors-title-recipients = Primatelji
debtors-title-beneficiaries = Korisnici

### Welcome

welcome-title = Tvoji računi nisu ničija briga.
welcome-subtitle = Dijeli troškove s prijateljima.
welcome-e2ee-title = Sve šifrirano
welcome-e2ee-body = Imena, iznosi, projekti: sve se šifrira na tvom uređaju. Samo ti imaš ključ. Nitko ne može čitati tvoje račune. Čak ni mi.
welcome-e2ee-note = Nečitljivo čak i nama (nulti pristup poslužitelja)
welcome-eu-title = 100 % europski
welcome-eu-body = Poslužitelji u Njemačkoj, e-mail se šalje iz Francuske. Tvoji podaci nikad ne napuštaju Europsku uniju.
welcome-noads-title = Bez oglasa. Bez praćenja.
welcome-noads-body = Ništa ne prikupljamo i ne prodajemo tvoje podatke. To nije naš model.
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
help-forgot-password-warning = Tvoji podaci bit će trajno izgubljeni.
help-forgot-password-a = Ključ za šifriranje izvodi se iz tvoje lozinke pa poništavanje nije moguće: nitko - ni mi - ne može dešifrirati tvoje projekte bez nje. Čuvaj je na sigurnom, idealno u upravitelju lozinki.
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
expense-converted-from = Plaćeno { $amount } { $from } · 1 { $from } = { $rate } { $to }
project-currency = Valuta
project-currency-hint = Svaki iznos prikazuje se u ovoj valuti. Ne može se kasnije promijeniti.
project-currency-locked = Valuta se određuje pri stvaranju projekta.

update-required-title = Potrebno ažuriranje
update-required-body = Ova verzija Counteda prestara je za komunikaciju s poslužiteljem. Ažuriraj je da nastaviš koristiti aplikaciju.
update-required-body-testflight = Ova verzija Counteda prestara je za komunikaciju s poslužiteljem. Otvori TestFlight i instaliraj najnoviju verziju da nastaviš koristiti aplikaciju.
update-required-button = Ažuriraj

notifications-label = Obavijesti
notifications-title = Obavijesti
notifications-empty = Ništa novo
notifications-friend-request = Zahtjev za prijateljstvo

friends-title = Prijatelji
friends-anonymous-body = Prijatelji su vezani uz tvoj račun. Prijavi se za dodavanje osoba i pozivanje u projekte bez dijeljenja poveznice.
friends-add-title = Dodaj prijatelja
friends-add-hint = Vidjet će tvoj zahtjev kad se prijavi. Nitko od vas ne saznaje ima li drugi račun dok zahtjev nije prihvaćen.
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
friends-no-key = Još nije spremno
friends-fingerprint = Sigurnosni kod
friends-fingerprint-hint = Dva prijatelja koja jedno drugom pročitaju isti sigurnosni kod znaju da nitko ne stoji između njih - čak ni naš poslužitelj.
friends-outgoing-title = Poslano
friends-outgoing-hint = Čeka se odgovor. Vidjet ćeš ih među prijateljima kad prihvate.
friends-withdraw = Odustani
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
