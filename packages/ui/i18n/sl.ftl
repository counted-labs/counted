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
projects-empty-hint = Ustvari projekt z gumbom spodaj
projects-offline-banner = Podatki brez povezave - za osvežitev se znova poveži.
demo-banner = Predstavitveni projekt - samo za branje.
demo-start-own = Ustvari svoj projekt
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
charts-paid-more = { $amount } več od tvojega deleža
charts-paid-less = { $amount } manj od tvojega deleža
charts-paid-even = Točno tvoj delež
charts-part-title = Tvoj del v vsaki kategoriji
charts-part-desc = Sivo je, kar je porabila skupina, barvno, kar si porabil ti.
charts-breakdown-title = Razčlenitev po kategorijah
charts-breakdown-desc = Tapni izsek ali vrstico za prikaz stroškov.
charts-of-total = { $amount } od { $total }
charts-show-all = Pokaži vse ({ $count })
charts-show-less = Pokaži manj
charts-spend-title = Stroški skozi čas
charts-spend-desc = Kratka obdobja po dnevih, daljša po tednih ali mesecih.
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
charts-projects-desc = Vsote se vodijo po valutah in se nikoli ne seštevajo.
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
currency-search = Poišči valuto

update-required-title = Potrebna je posodobitev
update-required-body = Ta različica Counteda je prestara za komunikacijo s strežnikom. Posodobi jo, da nadaljuješ z uporabo aplikacije.
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

# Participants in the create and edit modals, and the "who are you?" picker - see
# docs/plans/friends.md §11.
participants-you-label = Tvoje ime v tem projektu
participants-you-badge = Ti
participants-you-from-account = Prevzeto iz imena tvojega računa. Tukaj ga spremeni samo za ta projekt.
participants-you-required = Obvezno. Tako te bodo videli drugi.
participants-others = Drugi udeleženci
participants-empty = Še nikogar. Spodaj izberi prijatelja ali vpiši katero koli ime.
participants-empty-signed-out = Še nikogar. Vpiši ime, da nekoga dodaš.
participants-duplicate = »{ $name }« je že na seznamu.
participants-input-label = Dodaj prijatelja ali vpiši ime
participants-input-placeholder = Prijatelj ali katero koli ime
participants-suggest-friend = Prijatelj · pridruži se kot »{ $name }«, prejme povabilo
participants-suggest-not-ready = Prijatelj · še ni pripravljen
participants-suggest-guest = Dodaj »{ $text }« brez računa
participants-suggest-guest-sub = Brez računa, samo ime
participants-friends = Tvoji prijatelji
participants-all-friends = Vsi prijatelji
participants-login-hint = Prijavi se, da dodajaš ljudi neposredno s seznama prijateljev.
participants-invite-badge = Povabi
participants-guest-badge = Brez računa
participants-guest-sub = Brez računa, samo ime
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
edit-project-you-are = Na tej napravi si { $name }
edit-project-no-identity = Še nisi izbral(a), kdo si
edit-project-switch = Zamenjaj
edit-project-choose = Izberi
invite-failed = Teh povabil ni bilo mogoče poslati: { $emails }
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
