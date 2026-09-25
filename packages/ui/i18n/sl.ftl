# Slovenščina. Popolna razen pravnih besedil (legal-, terms-, privacy-), ki obstajajo le v angleščini
# in francoščini in se za vsako sporočilo posebej vrnejo na en.ftl.

### Common

loading = Nalaganje…
cancel = Prekliči
confirm = Potrdi
retry = Poskusi znova
delete = Izbriši
back = Nazaj
language = Jezik

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
error-generic = Nekaj je šlo narobe. Poskusi znova.

error-invalid-email = Ta e-poštni naslov ni veljaven.
error-invalid-password = To geslo ni veljavno.
error-password-too-short = Geslo mora imeti vsaj 8 znakov.
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
login-resend = Znova pošlji povezavo za potrditev
login-resending = Pošiljanje…
login-resend-sent = E-pošta poslana - preveri nabiralnik.

register-submit = Ustvari račun
register-submitting = Ustvarjanje…
register-have-account = Že imaš račun?
register-password-placeholder = Vsaj 8 znakov
register-password-warning = Zapiši si geslo. Če ga pozabiš, računa ni mogoče obnoviti.
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
settings-about = O aplikaciji
settings-anonymous-title = Nisi prijavljen
settings-upsell-title = Tvoji projekti na vsaki napravi
settings-upsell-free = Brezplačno
settings-upsell-body = Counted deluje brez računa. Z brezplačnim računom ti projekti in nastavitve sledijo na telefon, prenosnik in splet - še vedno šifrirani, še vedno neberljivi za nas.
settings-locked-badge = Račun
settings-locked-friends = Ustvari račun, da dodaš prijatelje in jih povabiš v projekt iz aplikacije - brez pošiljanja povezave.
settings-locked-payment-methods = Enkrat shrani svoj IBAN ali plačilno aplikacijo in ju deli z izbranimi projekti. Kdor ti dolguje, ju vidi ob tvojem imenu.
settings-friends-hint = Dodaj prijatelje in jih povabi v svoje projekte brez deljenja povezave.

account-member-since = Član od
account-logout = Odjava
account-logging-out = Odjavljanje…
account-delete-title = Izbriši moj račun
account-delete-warning = Takoj in trajno, brez koša. Stroški, ki si jih vnesel v deljen projekt, ostanejo vidni drugim članom - so del njihovih računov.
account-delete-confirm-title = Izbriši račun
account-delete-confirm-message = Tvoj račun, seje in seznam projektov bodo trajno izbrisani. Brez gesla postanejo šifrirani podatki deljenega projekta zate neberljivi - tega ni mogoče razveljaviti.

settings-payment-methods = Podatki za plačilo
settings-payment-methods-hint = Kako želiš prejeti povračilo. Šifrirano s tvojim računom.
payment-method-kind = Način
payment-method-kind-other = Drugo
payment-method-label = Ime
payment-method-label-placeholder = Glavni račun
payment-method-value = Podatki
payment-method-value-placeholder = IBAN, telefonska številka, uporabniško ime…
payment-method-add = Dodaj
payment-method-remove = Odstrani { $name }
payment-method-empty = Podatkov za plačilo še nisi dodal.
payment-method-deleted = Plačilni način izbrisan.
payment-method-value-required = Izpolni podatke vsakega plačilnega načina ali ga odstrani.
payment-method-label-required = Poimenuj svoj način po meri.
payment-method-too-long = To je predolgo - skrajšaj.
payment-method-invalid-characters = Odstrani prelome vrstic ali nevidne znake.
payment-method-limit = Shraniš lahko do { $max } plačilnih načinov.
payment-methods-saved = Podatki za plačilo shranjeni.
payment-methods-offline = Za shranjevanje podatkov za plačilo moraš biti povezan.
payment-methods-stale = Tvoji podatki za plačilo so bili spremenjeni na drugi napravi. Znova so naloženi — poskusi znova.
payment-methods-key-missing = Za upravljanje podatkov za plačilo se znova prijavi.
settings-payment-methods-share-warning = Deljen način je viden vsakemu članu projektov, v katerih si izbral svoje ime - vsakomur, ki ima eno od teh povezav.
payment-method-share = Deli z mojimi projekti
payment-method-share-hint = Prikazano ob tvojem imenu, ko ti kdo dolguje.
payment-method-copy = Kopiraj { $name }
payment-method-copied = Kopirano.
payment-method-copy-failed = Kopiranje ni uspelo - označi besedilo in ga kopiraj ročno.

verify-email-checking = Preverjanje e-poštnega naslova…
verify-email-welcome = E-pošta potrjena - dobrodošel v Counted!
verify-email-back-to-login = Nazaj na prijavo

### Project status

project-close = Zapri
project-archive = Arhiviraj
project-reopen = Znova odpri
project-unarchive = Vrni iz arhiva

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
projects-empty-hint = Ustvari projekt z gumbom spodaj
projects-offline-banner = Podatki brez povezave - za osvežitev se znova poveži.
projects-no-local-data = Ni lokalnih podatkov
projects-no-local-data-hint = Prijavi se, da prvič naložiš svoje projekte.
projects-add = Dodaj projekt
projects-create = Ustvari projekt
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
add-project-remove-participant = Odstrani udeleženca
add-project-me-badge = Jaz
add-project-thats-me = To sem jaz!
add-project-offline = Brez povezave ni mogoče ustvariti projekta. Znova se poveži in poskusi znova.
add-project-name-required = Projekt potrebuje ime.
add-project-need-two-participants = Dodaj vsaj 2 udeleženca.
add-project-pick-yourself = Povej nam, kateri udeleženec si.

join-link-label = Povezava za deljenje
join-link-hint = Povezava vsebuje ključ za dešifriranje - kopiraj jo v celoti.
join-invalid-link = Ta povezava ni veljavna. Prilepi celotno povezavo za deljenje, vključno z delom za #.
join-wrong-project = Ta povezava je za drug projekt.

import-tricount-link-label = Povezava ali ključ Tricount
import-tricount-key-required = Vnesi povezavo ali ključ Tricount.
import-tricount-encryption-failed = Šifriranje ni uspelo.

### Expenses

save = Shrani
saving = Shranjevanje…
adding = Dodajanje…
link-copied = Povezava kopirana
missing-encryption-key = Manjka šifrirni ključ.
missing-encryption-key-title = Manjka šifrirni ključ
missing-encryption-key-hint = Uporabljena povezava ne vsebuje ključa za dešifriranje tega projekta. Uporabi celotno povezavo, ki jo je delil ustvarjalec.
project-locked-hint = Ta naprava nima ključa tega projekta. Odpri njegovo povezavo za deljenje, da ga odkleneš.
project-unlock = Odkleni
project-no-local-data-hint = Prijavi se, da prvič naložiš podatke tega projekta.
project-gone-title = Ta projekt ne obstaja več
project-gone-hint = Izbrisan je bil, ko ga je zapustil zadnji član. Povezava za deljenje ne deluje več, tudi če jo znova odpreš.

expense-add = Dodaj strošek
transfer-add = Dodaj nakazilo
expense-edit-title = Uredi strošek
expense-category = Kategorija
expense-category-auto = Samodejno · { $emoji }
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
participants-deselect-all = Prekliči izbor
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
reimbursements-empty-title = Vse je poravnano!
reimbursements-empty-hint = Predlogi za poravnavo se prikažejo tukaj, ko računi niso uravnoteženi
reimbursement-owes = { $debtor } dolguje { $creditor }
reimbursement-record = Poravnaj
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
edit-project-deferred-me = izbira »To sem jaz«
edit-project-offline-deferred = Brez povezave: { $items } bo uveljavljeno ob ponovni povezavi.

export-saved = Datoteka shranjena:
    { $path }
export-failed = Izvoz ni uspel: { $reason }

history-expense-added = Strošek dodan: { $name }
history-expense-edited = Strošek urejen: { $name }
history-expense-deleted = Strošek izbrisan: { $name }
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
expense-inconsistent-detail = Zneski se ne ujemajo: { $paid } plačano, { $owed } dolgovano, za strošek { $total }. Uredi strošek, da to popraviš.
missing-access-key = Manjka dostopni ključ. Odpri ta projekt prek njegove povezave za deljenje.
filter-all = Vse
filter-my-payments = Moja plačila
filter-my-debts = Kaj dolgujem
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
charts-tab-per-person = Na osebo
charts-tab-trends = Trendi
charts-by-category = Razčlenitev po kategorijah
charts-per-person = Poraba na osebo
charts-categories-by-month = Kategorije po mesecih
charts-total-spent = Skupaj porabljeno
charts-avg-per-person = Povp. na osebo
charts-expense-count =
    { $count ->
        [one] { $count } strošek
        [two] { $count } stroška
        [few] { $count } stroški
       *[other] { $count } stroškov
    }
charts-clear-category-filter = Počisti filter kategorije
charts-no-expenses = Ni stroškov.
charts-pick-a-project = Izberi projekt za prikaz porabe na osebo.
charts-nothing-to-show = Ničesar za prikaz
charts-my-share-note = Ti zneski so tvoj delež vsakega stroška.
charts-my-share-skipped =
    { $count ->
        [one] 1 projekt ni upoštevan — udeleženec ni izbran ali se njegovi podatki niso naložili.
        [two] { $count } projekta nista upoštevana — udeleženec ni izbran ali se njuni podatki niso naložili.
        [few] { $count } projekti niso upoštevani — udeleženec ni izbran ali se njihovi podatki niso naložili.
       *[other] { $count } projektov ni upoštevanih — udeleženec ni izbran ali se njihovi podatki niso naložili.
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
charts-person = Oseba
charts-project = Projekt
charts-all-projects = Vsi projekti
charts-whole-project = Celoten projekt
charts-date-from = Od
charts-date-to = Do
charts-total = Skupaj
charts-payments-per-person-by-month = Plačila na osebo po mesecih
history-empty = Ni dogodkov
history-by = { $name }
not-found-hint = Ta stran ne obstaja ali je bila premaknjena.
payers-title-paid-by = Plačal(a)
payers-title-sender = Pošiljatelj
payers-title-contributors = Prispevali
debtors-title-debtors = Dolguje
debtors-title-recipients = Prejemniki
debtors-title-beneficiaries = Upravičenci

### Welcome

welcome-title = Tvoji računi niso nikogaršnja stvar.
welcome-subtitle = Deli stroške s prijatelji.
welcome-e2ee-title = Vse šifrirano
welcome-e2ee-body = Imena, zneski, projekti: vse se šifrira na tvoji napravi. Ključ imaš samo ti. Nihče ne more brati tvojih računov. Niti mi.
welcome-e2ee-note = Neberljivo tudi za nas (ničelni dostop strežnika)
welcome-eu-title = 100 % evropsko
welcome-eu-body = Strežniki v Nemčiji, e-pošta poslana iz Francije. Tvoji podatki nikoli ne zapustijo Evropske unije.
welcome-noads-title = Brez oglasov. Brez sledilnikov.
welcome-noads-body = Ničesar ne zbiramo in tvojih podatkov ne prodajamo. To ni naš model.
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
help-forgot-password-warning = Tvoji podatki bodo trajno izgubljeni.
help-forgot-password-a = Šifrirni ključ je izpeljan iz tvojega gesla, zato ponastavitev ni mogoča: nihče - niti mi - ne more dešifrirati tvojih projektov brez njega. Hrani ga na varnem, najbolje v upravitelju gesel.
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
project-currency-hint = Vsak znesek je prikazan v tej valuti. Pozneje je ni mogoče spremeniti.
project-currency-locked = Valuta se določi ob ustvarjanju projekta.

update-required-title = Potrebna je posodobitev
update-required-body = Ta različica Counteda je prestara za komunikacijo s strežnikom. Posodobi jo, da nadaljuješ z uporabo aplikacije.
update-required-body-testflight = Ta različica Counteda je prestara za komunikacijo s strežnikom. Odpri TestFlight in namesti najnovejšo gradnjo, da nadaljuješ z uporabo aplikacije.
update-required-button = Posodobi

notifications-label = Obvestila
notifications-title = Obvestila
notifications-empty = Nič novega
notifications-friend-request = Prošnja za prijateljstvo

friends-title = Prijatelji
friends-anonymous-body = Prijatelji so vezani na tvoj račun. Prijavi se, da dodaš osebe in jih povabiš v svoje projekte brez deljenja povezave.
friends-add-title = Dodaj prijatelja
friends-add-hint = Tvojo prošnjo bo videl ob prijavi. Nobeden od vaju ne izve, ali ima drugi račun, dokler prošnja ni sprejeta.
friends-add-button = Dodaj
friends-add-from-project = Dodaj kot prijatelja
friends-request-sent = Prošnja poslana
friends-no-account-key = Za upravljanje prijateljev na tej napravi se znova prijavi.
friends-incoming-title = Prošnje
friends-accept = Sprejmi
friends-decline = Zavrni
friends-list-title = Moji prijatelji
friends-list-empty = Še ni prijateljev. Dodaj nekoga po e-pošti zgoraj ali iz projekta, ki ga delita.
friends-remove = Odstrani
friends-remove-confirm-title = Odstrani prijatelja
friends-remove-confirm-message = { $email } ne bo več med vašimi prijatelji, vi pa ne več med njegovimi. Kdorkoli od vaju lahko kasneje pošlje novo prošnjo.
friends-no-key = Še ni pripravljeno
friends-fingerprint = Varnostna koda
friends-fingerprint-hint = Dva prijatelja, ki si prebereta isto varnostno kodo, vesta, da med njima ni nikogar - niti našega strežnika.
friends-outgoing-title = Poslano
friends-outgoing-hint = Čaka na odgovor. Med prijatelji jih boš videl, ko sprejmejo.
friends-withdraw = Prekliči
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
