# Magyar. Teljes, a jogi szövegek (legal-, terms-, privacy-) kivételével, amelyek csak angolul és
# franciául léteznek, és üzenetenként az en.ftl-re esnek vissza.

### Common

loading = Betöltés…
cancel = Mégse
confirm = Megerősítés
retry = Újra
delete = Törlés
back = Vissza
language = Nyelv

### Navigation

nav-main = Főnavigáció
nav-projects = Projektek
nav-charts = Statisztika
nav-settings = Beállítások

### Connectivity

offline-banner = Offline
offline-pending =
    { $count ->
        [one] { $count } függőben
       *[other] { $count } függőben
    }

sync-conflict-edit = Ütközés: „{ $name }” szerkesztése nem sikerült (a tétel törölve). Kihagyva.
sync-conflict-delete = Ütközés: „{ $name }” törlése nem sikerült (a tétel törölve). Kihagyva.
sync-conflict-other = Ütközés: a művelet „{ $name }” tételen nem sikerült (a tétel törölve). Kihagyva.
sync-error = Szinkronizálási hiba: { $reason }

### Errors

error-network = A szerver nem érhető el. Ellenőrizd az internetkapcsolatot.
error-generic = Valami hiba történt. Próbáld újra.

error-invalid-email = Ez az e-mail-cím érvénytelen.
error-invalid-password = Ez a jelszó érvénytelen.
error-password-too-short = A jelszónak legalább 8 karakter hosszúnak kell lennie.
error-client-outdated = Az alkalmazás ezen verziója elavult. Frissítsd a bejelentkezéshez.
error-invalid-link = Ez a link érvénytelen.
error-batch-too-large = Túl sok elem egyszerre.
error-payers-required = Válassz legalább egy fizetőt.
error-debtors-required = Válassz legalább egy személyt, aki tartozik.
error-duplicate-participant = Egy résztvevő kétszer szerepel ugyanazon az oldalon.
error-participant-not-in-project = Ez a résztvevő nem tagja a projektnek.
error-too-many-participants = Túl sok résztvevő egy kiadáshoz.
error-invalid-credentials = Hibás e-mail-cím vagy jelszó.
error-unauthenticated = Ehhez jelentkezz be.
error-email-not-verified = Az e-mail-címed még nincs megerősítve.
error-project-not-found = Ez a projekt már nem létezik.
error-expense-not-found = Ez a kiadás már nem létezik.
error-storage-full = A tárhely megtelt: a projekt kulcsát nem sikerült menteni ezen az eszközön. Őrizd meg a megosztási linket.
error-user-not-found = Ez a résztvevő már nem létezik.
error-tricount-not-found = A Tricount nem található, vagy az API-ja hibát adott vissza.
error-too-many-members = Ez a projekt elérte a tagok számának határát.
error-identity-taken = Egy másik fiók már magáénak jelölte ezt a résztvevőt.
error-claim-proof-invalid = Ez az eszköz nem rendelkezik a projekt kulcsával, így nem jelölhet meg résztvevőt. Nyisd meg újra a megosztási linket.
error-user-has-payments = Ennek a résztvevőnek kiadásai vannak a projektben, ezért nem távolítható el.
error-resend-cooldown = Várj 60 másodpercet, mielőtt újabb e-mailt kérsz.
error-self-friend-request = Magadat nem veheted fel barátnak.
error-not-a-friend = Csak a barátlistádon szereplőket hívhatod meg.
error-friend-has-no-key = Ez a barátod még nem nyitotta meg az alkalmazás legújabb verzióját. Kérd meg, hogy egyszer jelentkezzen be, aztán próbáld újra.
error-friend-request-not-found = Ez a barátkérelem már nem létezik.
error-invitation-not-found = Ez a meghívó már nem létezik.
error-too-many-friend-requests = Egyelőre túl sok barátkérelem. Próbáld holnap.
error-too-many-invitations = Túl sok függőben lévő meghívó.
error-invalid-kdf-salt = A titkosítási beállítások érvénytelenek. Frissítsd az alkalmazást, és próbáld újra.
error-mixed-project-batch = Ezek a résztvevők nem mind ugyanahhoz a projekthez tartoznak.
error-invalid-payload = Az alkalmazás ezen verziója olyan adatokat küldött, amelyeket a kiszolgáló nem fogad el. Frissítsd, és próbáld újra.
error-invalid-public-key = A titkosítási kulcsod érvénytelen. Frissítsd az alkalmazást, és próbáld újra.
error-payment-methods-stale = A fizetési adataidat egy másik eszközön módosították. Töltsd újra, és próbáld újra.

### Auth

field-email = E-mail
field-email-placeholder = te@pelda.hu
field-password = Jelszó
field-name = Név
field-name-placeholder = Kovács Anna

login-title = Bejelentkezés
login-submit = Bejelentkezés
login-submitting = Bejelentkezés…
login-password-placeholder = A jelszavad
login-no-account = Még nincs fiókod?
login-unverified = Az e-mail-címed még nincs megerősítve. Nézd meg a postaládád, vagy küldd el újra a linket.
login-resend = Megerősítő link újraküldése
login-resending = Küldés…
login-resend-sent = E-mail elküldve - nézd meg a postaládád.

register-submit = Fiók létrehozása
register-submitting = Létrehozás…
register-have-account = Már van fiókod?
register-password-placeholder = Legalább 8 karakter
register-password-warning = Írd fel a jelszavad. Ha elfelejted, a fiókod nem állítható helyre.
register-check-email-title = Nézd meg az e-mailjeidet
register-email-sent = E-mail elküldve
register-email-sent-hint = Kattints a postaládádban lévő linkre a fiók aktiválásához.
register-not-received-prefix = Nem kaptad meg? Nézd meg a spam mappát, vagy
register-sign-in-link = jelentkezz be
register-not-received-suffix = a link újraküldéséhez.
register-terms-prefix = A fiók létrehozásával elfogadod a
register-terms-link = felhasználási feltételeinket
register-terms-and = és az
register-privacy-link = adatvédelmi tájékoztatónkat

settings-title = Beállítások
settings-preferences = Beállítások
settings-preferences-local = Ezen az eszközön tárolva.
settings-preferences-synced = A fiókoddal szinkronizálva, titkosítva.
settings-about = Névjegy
settings-anonymous-title = Nem vagy bejelentkezve
settings-upsell-title = A projektjeid, minden eszközön
settings-upsell-free = Ingyenes
settings-upsell-body = A Counted fiók nélkül is működik. Ingyenes fiókkal a projektjeid és beállításaid követnek a telefonodra, a laptopodra és a webre - továbbra is titkosítva, továbbra is olvashatatlanul számunkra.
settings-locked-badge = Fiók
settings-locked-friends = Hozz létre fiókot, hogy barátokat adj hozzá és meghívd őket egy projektbe az alkalmazásból - link küldözgetése nélkül.
settings-locked-payment-methods = Mentsd el egyszer az IBAN-od vagy a fizetési appod, és oszd meg a választott projektekkel. Aki tartozik neked, a neved mellett látja.
settings-friends-hint = Adj hozzá barátokat és hívd meg őket a projektjeidbe link megosztása nélkül.

account-member-since = Tag ekkortól
account-logout = Kijelentkezés
account-logging-out = Kijelentkezés…
account-delete-title = Fiókom törlése
account-delete-warning = Azonnali és végleges, lomtár nélkül. A megosztott projektben rögzített kiadásaid a többi tag számára láthatók maradnak - az ő elszámolásuk része.
account-delete-confirm-title = Fiók törlése
account-delete-confirm-message = A fiókod, a munkameneteid és a projektlistád végleg törlődik. A jelszavad nélkül a megosztott projekt titkosított adatai olvashatatlanná válnak számodra - ez nem vonható vissza.

settings-payment-methods = Fizetési adatok
settings-payment-methods-hint = Hogyan szeretnéd visszakapni a pénzed. A fiókoddal titkosítva.
payment-method-kind = Mód
payment-method-kind-other = Egyéb
payment-method-label = Név
payment-method-label-placeholder = Fő számla
payment-method-value = Adatok
payment-method-value-placeholder = IBAN, telefonszám, felhasználónév…
payment-method-add = Hozzáadás
payment-method-remove = { $name } eltávolítása
payment-method-empty = Még nem adtál meg fizetési adatokat.
payment-method-deleted = Fizetési mód törölve.
payment-method-value-required = Töltsd ki minden fizetési mód adatait, vagy távolítsd el.
payment-method-label-required = Adj nevet az egyéni fizetési módnak.
payment-method-too-long = Ez túl hosszú - rövidítsd le.
payment-method-invalid-characters = Távolítsd el a sortöréseket vagy láthatatlan karaktereket.
payment-method-limit = Legfeljebb { $max } fizetési módot menthetsz.
payment-methods-saved = Fizetési adatok mentve.
payment-methods-offline = A fizetési adatok mentéséhez online kell lenned.
payment-methods-stale = A fizetési adataid egy másik eszközön módosultak. Újratöltöttük őket — próbáld újra.
payment-methods-key-missing = Jelentkezz be újra a fizetési adatok kezeléséhez.
settings-payment-methods-share-warning = A megosztott mód látható minden olyan projekt tagjai számára, ahol kiválasztottad a neved - bárkinek, aki rendelkezik az egyik linkkel.
payment-method-share = Megosztás a projektjeimmel
payment-method-share-hint = A neved mellett jelenik meg, ha valaki tartozik neked.
payment-method-copy = { $name } másolása
payment-method-copied = Másolva.
payment-method-copy-failed = Nem sikerült másolni - jelöld ki a szöveget és másold kézzel.

verify-email-checking = E-mail-cím megerősítése…
verify-email-welcome = E-mail megerősítve - üdv a Countedben!
verify-email-back-to-login = Vissza a bejelentkezéshez

### Project status

project-close = Lezárás
project-archive = Archiválás
project-reopen = Újranyitás
project-unarchive = Visszaállítás archívumból

### Dates

date-long = { $year }. { $month } { $day }.

month-1 = január
month-2 = február
month-3 = március
month-4 = április
month-5 = május
month-6 = június
month-7 = július
month-8 = augusztus
month-9 = szeptember
month-10 = október
month-11 = november
month-12 = december

month-short-1 = jan
month-short-2 = feb
month-short-3 = már
month-short-4 = ápr
month-short-5 = máj
month-short-6 = jún
month-short-7 = júl
month-short-8 = aug
month-short-9 = sze
month-short-10 = okt
month-short-11 = nov
month-short-12 = dec

### Actions

add = Hozzáadás
create = Létrehozás
creating = Létrehozás…
edit = Szerkesztés
leave = Kilépés
close = Bezárás
paste = Beillesztés
join = Csatlakozás
import = Importálás
importing = Importálás…
field-description = Leírás
field-date = Dátum
date-today = Ma
date-yesterday = Tegnap
field-optional = Nem kötelező

### Projects

projects-filter-active = Aktív
projects-filter-all = Összes
projects-count-label = Projektek
projects-empty = Nincs projekt
projects-empty-hint = Hozz létre egy projektet a lenti gombbal
projects-offline-banner = Offline adatok - frissítéshez csatlakozz újra.
projects-no-local-data = Nincsenek helyi adatok
projects-no-local-data-hint = Jelentkezz be a projektjeid első betöltéséhez.
projects-add = Projekt hozzáadása
projects-create = Projekt létrehozása
projects-join = Csatlakozás projekthez
projects-import-tricount = Importálás Tricountból
project-actions = Projektműveletek

status-ongoing = Folyamatban
status-closed = Lezárva
status-archived = Archiválva

nav-help = Súgó
nav-privacy = Adatvédelmi tájékoztató
nav-terms = Felhasználási feltételek
nav-legal = Impresszum

leave-project-title = Kilépsz a projektből?
leave-project-message = Erről az eszközről elveszíted a hozzáférést. Ha nem marad egyetlen tag sem, a projekt és minden kiadása végleg törlődik.

add-project-title = Új projekt
add-project-name-label = Projekt neve
add-project-name-placeholder = Utazásom, Lakótársak 2024…
add-project-participants = Résztvevők
add-project-participant-name = Résztvevő neve
add-project-participant-placeholder = Clark Kent
add-project-offline = Offline nem hozhatsz létre projektet. Csatlakozz újra, és próbáld meg ismét.
add-project-name-required = A projektnek névre van szüksége.

join-link-label = Megosztási link
join-link-hint = A link tartalmazza a visszafejtő kulcsot - másold ki teljes egészében.
join-invalid-link = Ez a link érvénytelen. Illeszd be a teljes megosztási linket, a # utáni résszel együtt.
join-wrong-project = Ez a link egy másik projekthez tartozik.

import-tricount-link-label = Tricount link vagy kulcs
import-tricount-key-required = Adj meg egy Tricount linket vagy kulcsot.
import-tricount-encryption-failed = A titkosítás nem sikerült.
import-tricount-unimportable = Semmi sem lett importálva: ebben a Tricountban Tricount-fiókkal rendelkező tagok vagy nem egyező összegek vannak (érintett tételek: { $count }).

### Expenses

save = Mentés
saving = Mentés…
adding = Hozzáadás…
link-copied = Link másolva
missing-encryption-key = Hiányzik a titkosítási kulcs.
missing-encryption-key-title = Hiányzik a titkosítási kulcs
missing-encryption-key-hint = A használt link nem tartalmazza a projekt visszafejtéséhez szükséges kulcsot. Használd a teljes linket, amit a projekt létrehozója megosztott.
project-locked-hint = Ezen az eszközön nincs meg a projekt kulcsa. Nyisd meg a megosztási linkjét a feloldáshoz.
project-unlock = Feloldás
project-no-local-data-hint = Jelentkezz be a projekt adatainak első betöltéséhez.
project-gone-title = Ez a projekt már nem létezik
project-gone-hint = Törlődött, amikor az utolsó tagja kilépett. A megosztási link már nem működik, akkor sem, ha újra megnyitod.

expense-add = Kiadás hozzáadása
transfer-add = Átutalás hozzáadása
expense-edit-title = Kiadás szerkesztése
expense-category = Kategória
expense-category-auto = Auto · { $emoji }
expense-currency = Az összeg pénzneme
amount-op-add = Plusz
amount-op-subtract = Mínusz
amount-op-multiply = Szorzás
amount-op-divide = Osztás
amount-op-equals = Egyenlő
amount-op-done = Kész
expense-rate = Árfolyam (nem kötelező)
expense-rate-hint = Hagyd üresen az Európai Bizottság (InforEuro) { $month } havi árfolyamához: 1 { $from } = { $rate } { $to }.
expense-rate-invalid = Adj meg 0-nál nagyobb árfolyamot.
expense-rate-unavailable = Nincs automatikus árfolyam - add meg kézzel.
expense-delete-title = Kiadás törlése
expense-delete-message = „{ $name }” végleg törlődik. Ez nem vonható vissza.
expense-inconsistent-amounts = Az összegek nem egyeznek
expenses-empty = Nincs kiadás
expenses-empty-hint = Kezdd kiadások hozzáadásával a lenti gombbal
expenses-show-more = Több mutatása (még { $count })

expense-type-expense = Kiadás
expense-type-transfer = Átutalás
expense-type-gain = Bevétel
expense-paid-by = fizette:
expense-sent-by = küldte:
expense-contributed-by = hozzájárult:

expense-name-required = A név kötelező.
expense-amount-not-positive = Az összegnek 0-nál nagyobbnak kell lennie.
expense-no-payer = Válassz legalább egy fizetőt.
expense-no-debtor = Válassz legalább egy személyt, aki tartozik.
expense-invalid-date = Ez a dátum érvénytelen.
expense-payers-mismatch = A fizetők összege { $sum }, ami nem egyezik a kiadás összegével ({ $total }).
expense-debtors-mismatch = A tartozók összege { $sum }, ami nem egyezik a kiadás összegével ({ $total }).

participants-none = Senki
participants-everyone = Mindenki ({ $count })
participants-some = { $count } / { $total }
participants-select-all = Összes kijelölése
participants-by-shares = Részarány szerint
split-amounts = Összegek
participants-remaining = Még { $amount }
participants-over-by = { $amount } többlet
participants-who-paid = Ki fizetett?
participants-who-received = Ki kapta?
participants-who-transfers = Ki utal?
participants-who-receives = Ki kapja?
participants-for-whom = Kinek?

stats-total-expenses = Összes kiadás
stats-my-expenses = Saját kiadásaim

tab-expenses = Kiadások
tab-balance = Egyenleg
tab-reimbursements = Elszámolás
reimbursements-empty-title = Minden el van számolva!
reimbursements-empty-hint = Az elszámolási javaslatok itt jelennek meg, ha az egyenlegek nem egyeznek
reimbursement-owes = { $debtor } tartozik { $creditor } részére
reimbursement-record = Elszámolás
reimbursement-pay-with = Fizetés
reimbursement-pay-shared-by = { $name } osztotta meg - küldés előtt ellenőrizd az appod által mutatott címzett nevét.
reimbursement-pay-title = Fizetés { $name } részére
reimbursements-mine-title = Te tartozol
reimbursements-others-title = Egyéb visszafizetések
copy = Másolás

user-selection-title = Melyik résztvevő vagy?
user-selection-hint = Válaszd ki a neved a listából.
user-selection-required = Válassz egy résztvevőt.
identity-claimed = Fiókhoz kapcsolva
identity-claimed-by = { $name } fiókja
identity-taken-repick = Egy másik fiók magáénak jelölte az általad használt résztvevőt. Válassz másikat.
participant-gone-repick = Az általad használt résztvevőt eltávolították a projektből. Válassz másikat.

edit-project-title = Projekt szerkesztése
edit-project-new-badge = új
edit-project-deferred-new-members = új tagok hozzáadása
edit-project-deferred-removals = tagok eltávolítása
edit-project-offline-deferred = Offline: { $items } újracsatlakozáskor lép érvénybe.

export-failed = Az exportálás nem sikerült: { $reason }

history-expense-added = Kiadás hozzáadva: { $name }
history-expense-edited = Kiadás szerkesztve: { $name }
history-expense-deleted = Kiadás törölve: { $name }
history-project-edited = Projekt szerkesztve: { $name }
history-name-changed = Név: „{ $from }” → „{ $to }”
history-description-added = Leírás hozzáadva: „{ $value }”
history-description-removed = Leírás eltávolítva: „{ $value }”
history-description-changed = Leírás: „{ $from }” → „{ $to }”

### Sweep

field-amount = Összeg
expense-name-placeholder = Étterem, bevásárlás…
expense-actions = Kiadásműveletek
expense-your-share = A te részed
expense-your-share-value = A te részed: { $amount } { $currency }
expense-inconsistent-detail = Az összegek nem egyeznek: { $paid } fizetve, { $owed } tartozás, egy { $total } összegű kiadásnál. Szerkeszd a kiadást a javításhoz.
missing-access-key = Hiányzik a hozzáférési kulcs. Nyisd meg a projektet a megosztási linkjén keresztül.
filter-all = Összes
filter-my-payments = Saját fizetéseim
filter-my-debts = Amivel tartozom
participants-shares-for = { $name } részaránya
participants-amount-for = { $name } összege
reimbursement-add = Elszámolás hozzáadása
project-forget = Eltávolítás a listámról
project-history-title = Előzmények
history-kind-add = Hozzáadva
history-kind-delete = Törölve
history-kind-edit = Szerkesztve
export = Exportálás
export-json = Exportálás JSON-ba
export-csv = Exportálás CSV-be
share-link = Megosztás
copy-link-failed = A linket nem sikerült másolni
open-in-app = Megnyitás az alkalmazásban
not-found-title = Az oldal nem található
not-found-back = Vissza a projektekhez

### Charts

charts-period = Időszak
period-all = Összes
period-month = Hónap
period-3months = 3 hó
period-year = Év
period-custom = Egyéni
charts-tab-categories = Kategóriák
charts-tab-trends = Trendek
charts-total-spent = Összes költés
charts-avg-per-person = Átlag/fő
charts-expense-count =
    { $count ->
        [one] { $count } kiadás
       *[other] { $count } kiadás
    }
charts-nothing-to-show = Nincs megjeleníthető adat
charts-my-share-note = Ezek a számok a te részed minden kiadásból.
charts-my-share-skipped =
    { $count ->
        [one] 1 projekt nincs beszámítva — nincs kiválasztott résztvevő, vagy az adatai nem töltődtek be.
       *[other] { $count } projekt nincs beszámítva — nincs kiválasztott résztvevő, vagy az adataik nem töltődtek be.
    }

### Categories

category-food = Étel
category-transport = Közlekedés
category-accommodation = Szállás
category-leisure = Szabadidő
category-shopping = Vásárlás
category-services = Szolgáltatások
category-parties-gifts = Bulik és ajándékok
category-other = Egyéb
charts-project = Projekt
charts-all-projects = Összes projekt
charts-date-from = Ettől
charts-date-to = Eddig
charts-total = Összesen
charts-tab-people = Emberek
charts-tab-projects = Projektek
charts-scope = Kinek a kiadásai
charts-scope-group = Csoport
charts-scope-me = Én
charts-currency = Pénznem
charts-my-share = Az én részem
charts-share-of-total = { $total } { $pct }%-a
charts-i-paid = Én fizettem
charts-paid-more = { $amount } többet, mint a részed
charts-paid-less = { $amount } kevesebbet, mint a részed
charts-paid-even = Pont a részed
charts-part-title = A részed minden kategóriából
charts-part-desc = Szürke a csoport költése, színes a te fogyasztásod.
charts-breakdown-title = Bontás kategóriák szerint
charts-breakdown-desc = Koppints egy szeletre vagy sorra a kiadásokért.
charts-of-total = { $amount } / { $total }
charts-show-all = Összes mutatása ({ $count })
charts-show-less = Kevesebb
charts-spend-title = Kiadások időben
charts-spend-desc = Rövid időszak naponta, hosszabb hetente vagy havonta.
charts-group-by = Csoportosítás
bucket-day = Nap
bucket-week = Hét
bucket-month = Hónap
charts-avg = átl.
charts-cat-title-day = { $category }, napról napra
charts-cat-title-week = { $category }, hétről hétre
charts-cat-title-month = { $category }, hónapról hónapra
charts-cat-desc = Válassz kategóriát, hogy kövesd időben.
charts-running-title = Halmozott összeg
charts-running-desc = { $date } óta.
charts-avg-per-day = { $amount } / nap átlagosan
charts-avg-per-week = { $amount } / hét átlagosan
charts-avg-per-month = { $amount } / hónap átlagosan
charts-people-title = Ki vitte a csoportot
charts-people-desc = Ki mennyit fizetett, mellette mennyit fogyasztott.
charts-paid = Fizetett
charts-fair-share = Méltányos rész
charts-you = (te)
charts-net-more = többet fizetett
charts-net-less = kevesebbet fizetett
charts-balance-title = Egyenleged időben
charts-balance-desc = A vonal felett a csoport tartozik neked. Alatta te tartozol a csoportnak.
charts-owed = Tartoznak neked
charts-owe = Tartozol
charts-projects-title = A részed projektenként
charts-projects-desc = Az összegek pénznemenként külön maradnak, sosem adódnak össze.
history-empty = Nincs esemény
history-by = { $name }
not-found-hint = Ez az oldal nem létezik, vagy elköltözött.
payers-title-paid-by = Fizette
payers-title-sender = Küldő
payers-title-contributors = Hozzájárulók
debtors-title-debtors = Tartozik
debtors-title-recipients = Címzettek
debtors-title-beneficiaries = Kedvezményezettek

### Welcome

welcome-title = Az elszámolásod senki másra nem tartozik.
welcome-subtitle = Oszd meg a kiadásokat a barátaiddal.
welcome-note = Ingyenes. Nem kell fiók. Nincs reklám.
welcome-link-title = Egy link, és mindenki benne van.
welcome-link-body = Senkinek sem kell fiókot létrehoznia.
welcome-link-account = Fiók? Soha nem kötelező. Arra való, hogy másik eszközön is megtaláld a projektjeidet, meghívd a barátaidat az alkalmazásból, és megoszd a fizetési adataidat.
welcome-demo-project = Hétvége Lyonban
welcome-private-title = Senki sem olvashatja el az elszámolásaidat. Még mi sem.
welcome-private-body = Nevek, összegek, projektek: minden az eszközödön titkosítódik. Egyedül te birtoklod a kulcsot.
welcome-private-names = Nevek
welcome-private-amounts = Összegek
welcome-private-projects = Projektek
welcome-scan-title = Fotózd le a nyugtát.
welcome-scan-body = Az összeg, a dátum és a kategória magától kitöltődik. Minden a telefonodon történik. A fotót nem őrizzük meg.
welcome-eu-title = 100% európai
welcome-no-ads = Nincs reklám
welcome-no-trackers = Nincs nyomkövető
welcome-step = { $current }. lépés / { $total }
welcome-next = Tovább
welcome-skip = Kihagyás
welcome-start = Kezdjük
welcome-how-it-works = Pontosan hogyan működik?

### Help

help-intro = Gyakori kérdés? Koppints a válasz kibontásához.
help-create-project-q = Hogyan hozok létre projektet?
help-create-project-a = A kezdőképernyőn koppints az alsó + gombra. Adj nevet a projektnek, válassz pénznemet, és kész.
help-add-participants-q = Hogyan adok hozzá résztvevőket?
help-add-participants-a = Nyisd meg a projektet, majd add hozzá a résztvevőket a taglistából. Minden résztvevő fizethet vagy tartozhat egy kiadásnál.
help-share-project-q = Hogyan osztok meg egy projektet?
help-share-project-a = Oszd meg a projekt URL-jét (ami a címsorban van). Bárki, akinek megvan a link, megtekintheti és szerkesztheti a projektet.
help-add-expense-q = Hogyan adok hozzá kiadást?
help-add-expense-a = Egy projektben koppints a + gombra, add meg az összeget, hogy ki fizetett és kik között osztjuk el. A mai helyett más dátumot is választhatsz.
help-types-q = Mi a különbség kiadás, átutalás és bevétel között?
help-types-expense = - egy személy vásárlása, több személy között elosztva.
help-types-transfer = - visszafizetés egyik személytől a másiknak, elosztás nélkül.
help-types-gain = - kapott pénz (visszatérítés, ajándék), több személy között elosztva.
help-past-date-q = Dátumozhatok kiadást a múltba?
help-past-date-a = Igen, a dátummező szabad. A bejegyzés létrehozási ideje külön tárolódik.
help-who-owes-q = Hogyan számolja ki a Counted, ki mivel tartozik?
help-who-owes-a = A Counted kiszámolja minden résztvevő nettó egyenlegét (amit előlegezett mínusz amivel tartozik), majd a legrövidebb átutalás-sorozatot javasolja, ami mindenkit kiegyenlít.
help-minimal-transfers-q = Miért minimális a javasolt átutalások száma?
help-minimal-transfers-a = Az algoritmus először a pontosan kioltó egyenlegeket párosítja, majd a többit a legnagyobb hitelezőtől a legnagyobb adósig dolgozza fel. Az eredmény: kevesebb átutalás mindennek a rendezéséhez.
help-import-tricount-q = Hogyan importálok projektet Tricountból?
help-import-tricount-a = A kezdőképernyőn koppints az alsó „+” gombra, majd
help-import-tricount-b = Illeszd be az importálni kívánt Tricount megosztási linkjét.
help-encryption-q = Titkosítva vannak az adataim?
help-encryption-a = Igen. A Counted két garanciát egyesít:
help-encryption-e2ee-term = Végpontok közötti titkosítás
help-encryption-e2ee-def = - minden titkosítva utazik közted és a szerver között.
help-encryption-zero-term = Zéró hozzáférés
help-encryption-zero-def = - küldés előtt titkosítod az adatokat, a szerver pedig csak a titkosított szöveget tárolja. Nincs módunk elolvasni.
help-encryption-see = A részletekért lásd az
help-forgot-password-q = Mi történik, ha elfelejtem a jelszavam?
help-forgot-password-warning = Az adataid végleg elvesznek.
help-forgot-password-a = A titkosítási kulcs a jelszavadból származik, így visszaállítás nem lehetséges: senki - mi sem - nem tudja visszafejteni a projektjeidet nélküle. Őrizd biztonságban, ideális esetben jelszókezelőben.
help-archive-delete-q = Hogyan archiválok vagy törlök egy projektet?
help-archive-delete-a = A projekt képernyőjén nyisd meg a menüt, és válaszd az
help-archive-delete-b = lehetőséget, hogy elrejtsd, de megtartsd. A projekt végleg törlődik, amikor az utolsó tagja kilép belőle.
help-delete-account-q = Hogyan törlöm a fiókomat?
help-delete-account-a = Nyisd meg a Beállításokat, és használd a „Fiókom törlése” gombot. Azonnali és nem vonható vissza.
help-contact = Más kérdésed van? Írj nekünk:

# Receipt scanning (mobile only)
expense-scan = Nyugta beolvasása
scan-in-progress = Nyugta olvasása…
scan-error-capture = Nem sikerült a fotó. Próbáld újra, vagy add meg a kiadást kézzel.
scan-error-unreadable = Semmi olvasható ezen a nyugtán. Add meg a kiadást kézzel.
scan-check-amount = Ellenőrizd a végösszeget - nem volt tisztán nyomtatva.
scan-take-photo = Fotó készítése
scan-choose-photo = Fotó kiválasztása
expense-converted-from = Fizetve { $amount } { $from } · 1 { $from } = { $rate } { $to }
project-currency = Pénznem
project-currency-hint = Minden összeg ebben a pénznemben jelenik meg. Később nem módosítható.
project-currency-locked = A pénznem a projekt létrehozásakor rögzül.

update-required-title = Frissítés szükséges
update-required-body = A Counted ezen verziója túl régi a szerverrel való kommunikációhoz. Frissítsd az alkalmazás további használatához.
update-required-button = Frissítés

notifications-label = Értesítések
notifications-title = Értesítések
notifications-empty = Semmi új
notifications-friend-request = Barátkérelem

friends-title = Barátok
friends-anonymous-body = A barátok a fiókodhoz tartoznak. Jelentkezz be, hogy személyeket adj hozzá és meghívd őket a projektjeidbe link megosztása nélkül.
friends-add-title = Barát hozzáadása
friends-add-hint = Bejelentkezéskor látni fogja a kérelmedet. Egyikőtök sem tudja meg, hogy a másiknak van-e fiókja, amíg a kérelmet el nem fogadják.
friends-add-button = Hozzáadás
friends-add-from-project = Hozzáadás barátként
friends-request-sent = Kérelem elküldve
friends-no-account-key = Jelentkezz be újra a barátaid kezeléséhez ezen az eszközön.
friends-incoming-title = Kérelmek
friends-accept = Elfogadás
friends-decline = Elutasítás
friends-list-title = Barátaim
friends-list-empty = Még nincsenek barátaid. Adj hozzá valakit e-mailben fent, vagy egy közös projektből.
friends-remove = Eltávolítás
friends-remove-confirm-title = Barát eltávolítása
friends-remove-confirm-message = { $email } többé nem lesz a barátaid között, és te sem az övéi között. Bármelyikőtök küldhet később új kérést.
friends-no-key = Még nem áll készen
friends-fingerprint = Biztonsági kód
friends-fingerprint-hint = Két barát, akik ugyanazt a biztonsági kódot olvassák fel egymásnak, tudják, hogy senki nem áll közöttük - még a szerverünk sem.
friends-outgoing-title = Elküldve
friends-outgoing-hint = Válaszra vár. Elfogadás után megjelennek a barátaid között.
friends-withdraw = Mégse
invite-friends-title = Barátok meghívása
invite-friends-hint = A projekt kulcsa minden barátnak ezen az eszközön titkosítódik. A szerver soha nem látja.
invite-friends-empty = Még nincs meghívható barát.
invite-friends-button = Meghívás
invite-sent = { $count ->
    [one] Meghívó elküldve
   *[other] { $count } meghívó elküldve
}
invitation-badge = Meghívó
invitation-to = Csatlakozás: „{ $name }”
invitation-to-unnamed = Csatlakozás egy projekthez
invitation-unreadable = Ez a meghívó nem nyitható meg ezen az eszközön
invitation-from = Feladó: { $email }
invitation-accept = Csatlakozás
invitation-decline = Elutasítás

# Participants in the create and edit modals, and the "who are you?" picker - see
# docs/plans/friends.md §11.
participants-you-label = A neved ebben a projektben
participants-you-badge = Te
participants-you-from-account = A fiókneved alapján kitöltve. Itt csak ehhez a projekthez módosíthatod.
participants-you-required = Kötelező. Így látnak majd a többiek.
participants-others = További résztvevők
participants-empty = Még senki. Válassz lent egy barátot, vagy írj be egy nevet.
participants-empty-signed-out = Még senki. Írj be egy nevet, hogy hozzáadj valakit.
participants-duplicate = „{ $name }” már szerepel a listában.
participants-input-label = Barát hozzáadása vagy név beírása
participants-input-placeholder = Barát vagy bármilyen név
participants-suggest-friend = Barát · „{ $name }” néven csatlakozik, meghívót kap
participants-suggest-not-ready = Barát · még nem áll készen
participants-suggest-guest = „{ $text }” hozzáadása fiók nélkül
participants-suggest-guest-sub = Nincs fiókja, csak egy név
participants-friends = A barátaid
participants-all-friends = Minden barát
participants-login-hint = Jelentkezz be, hogy közvetlenül a barátlistádból adhass hozzá embereket.
participants-invite-badge = Meghívás
participants-guest-badge = Fiók nélkül
participants-guest-sub = Nincs fiókja, csak egy név
participants-rename = Átnevezés: { $name }
participants-remove = Eltávolítás: { $name }
participants-rename-label = Új név
participants-rename-save = Név mentése
participants-rename-hint = A név, amelyet mindenki lát ebben a projektben. A meghívó továbbra is ide megy: { $email }.
participants-invited-badge = Meghívva
participants-invited-sub = { $email } · még nincs elfogadva
participants-invited-pending = A meghívó még nincs elfogadva
participants-unlinked = Nincs fiókhoz kapcsolva
add-project-create-invite = Létrehozás és { $count } meghívása
edit-project-save-invite = Mentés és { $count } meghívása
edit-project-you-are = Ezen az eszközön te vagy: { $name }
edit-project-no-identity = Még nem választottad ki, ki vagy
edit-project-switch = Váltás
edit-project-choose = Kiválasztás
invite-failed = Ezeket a meghívókat nem sikerült elküldeni: { $emails }
invite-again = Újra meghívás
friend-picker-title = Barátok hozzáadása
user-selection-invited-hint = { $email } meghívott ide: „{ $project }”.
user-selection-suggested = Javasolt
user-selection-suggested-sub = { $email } ezen a néven adott hozzá
user-selection-confirm-as = Én vagyok: { $name }
user-selection-missing = Nincs itt a neved? Kérj meg egy résztvevőt, hogy adjon hozzá a projekt beállításaiban.

## Ismétlődő kiadások

repeat-label = Ismétlés
repeat-none = Nem ismétlődik
repeat-weekly = Minden héten
repeat-biweekly = Kéthetente
repeat-monthly = Minden hónapban
repeat-quarterly = Háromhavonta
repeat-yearly = Minden évben
repeat-every-weeks = { $count } hetente
repeat-every-months = { $count } havonta
repeat-every-years = { $count } évente
repeat-custom = Egyéni…
repeat-every = Gyakoriság:
repeat-unit-weeks = hetente
repeat-unit-months = havonta
repeat-unit-years = évente
repeat-on-weekday = napja: { $weekday }
repeat-on-day = a hónap { $day }. napján
repeat-on-day-month = { $month } { $day }.
repeat-month-end = Rövidebb hónapokban az utolsó napra esik.
repeat-ends = Vége
repeat-ends-never = Soha
repeat-ends-on = Adott napon
repeat-ends-after = Utána
repeat-fewer = Kevesebb
repeat-more = Több
repeat-last-on = utoljára: { $date }
repeat-variable = Az összeg minden alkalommal változik
repeat-variable-hint = Mindegyik az utolsó összeggel kerül be, „megerősítendő” jelöléssel.
repeat-done = Kész
repeat-no-end = Nincs vége
repeat-until = Eddig: { $date }
repeat-occurrences = Alkalmak száma: { $count }
repeat-offline = Kapcsolat szükséges. Maga a kiadás így is hozzáadható.
repeat-foreign = Ismétlődik { $amount } { $currency } összeggel, egyszer átváltva a mai árfolyamon. Figyelmeztetünk, ha az árfolyam 5 %-nál többet változik.
repeat-backfill = A múltban kezdődik. Most hozzáadott kiadások: { $count }.
add-and-repeat = Hozzáadás és ismétlés
weekday-1 = hétfő
weekday-2 = kedd
weekday-3 = szerda
weekday-4 = csütörtök
weekday-5 = péntek
weekday-6 = szombat
weekday-7 = vasárnap
recurring-title = Ismétlődő kiadások
recurring-strip = Ismétlődő kiadások: { $count }
recurring-next = Következő: { $name }, { $date }
recurring-to-confirm = Megerősítendő: { $count }
recurring-per-month = Havonta kb.
recurring-your-share = A te részed
recurring-active = Aktív
recurring-paused = Szüneteltetett
recurring-finished = Befejezett
recurring-paid-by = fizeti: { $name }
recurring-next-on = Következő: { $date }
recurring-progress = { $done } / { $total }
recurring-rate-badge = Árfolyam { $percent } %-kal változott
recurring-empty = Még semmi sem ismétlődik. Kiadás hozzáadásakor válaszd az „Ismétlés” lehetőséget: lakbér, előfizetések, számlák.
recurring-next-ones = Következők
recurring-added-so-far = Eddig hozzáadva
recurring-set-up-by = Létrehozta
recurring-pause = Szüneteltetés
recurring-resume = Folytatás
recurring-stop = Ismétlés leállítása
recurring-stop-title = Leállítod: „{ $name }”?
recurring-stop-message = Többé nem ismétlődik. A már hozzáadott kiadások megmaradnak.
recurring-resume-title = Folytatod: „{ $name }”?
recurring-resume-message = Következő: { $date }. A szünet alatt kimaradt dátumok nem kerülnek hozzáadásra.
recurring-edit-title = Ismétlődő kiadás szerkesztése
recurring-edit-banner = A módosítások ettől érvényesek: { $date }. A már hozzáadott kiadások változatlanok maradnak.
recurring-next-on-label = Következő
recurring-next-too-early = A következő dátumnak a legutóbb hozzáadott kiadás utánra kell esnie.
recurring-use-stop = A befejezéshez használd az „Ismétlés leállítása” lehetőséget az ismétlődő kiadásnál.
recurring-drift = A(z) { $currency } árfolyama { $percent } %-kal változott a létrehozás óta. Mindegyik továbbra is { $amount } { $project_currency } összeggel kerül be (1 { $currency } = { $rate }). A mai árfolyamon { $today_amount } { $project_currency } lenne.
recurring-use-rate = Mai árfolyam használata
recurring-keep = { $amount } { $currency } megtartása
recurring-added = Hozzáadott ismétlődő kiadások: { $count }
recurring-blocks-removal = { $name } még nem távolítható el: része ezeknek: { $rules }. Vedd ki { $name } résztvevőt belőlük vagy állítsd le őket, majd mentsd újra.
history-recurring-added = Automatikusan hozzáadva: { $name } ({ $date })
history-recurring-created = Ismétlődő kiadás létrehozva: { $name }
history-recurring-edited = Ismétlődő kiadás szerkesztve: { $name }
history-recurring-paused = Ismétlődő kiadás szüneteltetve: { $name }
history-recurring-resumed = Ismétlődő kiadás folytatva: { $name }
history-recurring-stopped = Ismétlődő kiadás leállítva: { $name }
occurrence-recurring = Ismétlődő kiadás
occurrence-auto = Automatikusan hozzáadva egy ismétlődő kiadásból.
occurrence-auto-next = Automatikusan hozzáadva egy ismétlődő kiadásból. Következő: { $date }.
occurrence-auto-stopped = Automatikusan hozzáadva egy azóta leállított ismétlődő kiadásból.
occurrence-manage = Kezelés
estimate-badge = Megerősítendő
estimate-title = Az összeget meg kell erősíteni.
estimate-body = Az előző összeggel került be. Add meg a valódit, amikor megtudod.
estimate-confirm = Összeg megerősítése
apply-to = Alkalmazás erre
apply-this-only = Csak erre a kiadásra
apply-and-next = Erre és a következőkre
apply-and-next-hint = Az ismétlődő kiadás ettől változik: { $date }
apply-rule-failed = A kiadás mentve, de az ismétlődő kiadás nem módosult.
occurrence-delete-message = „{ $name }” ({ $date }) véglegesen törlődik. Az ismétlés folytatódik, és ez a dátum nem tér vissza.
occurrence-delete-one = Csak ennek a törlése
occurrence-delete-stop = Törlés és az ismétlés leállítása
error-recurring-clock = Az eszköz órája siet. Ellenőrizd a dátumot és az időt.
error-recurring-not-found = Ez az ismétlődő kiadás már nem létezik.
error-recurring-stale = Valaki időközben módosította ezt az ismétlődő kiadást. Újratöltöttük: ellenőrizd, és mentsd újra.
error-too-many-recurring = Ennek a projektnek már 50 ismétlődő kiadása van. Állíts le egy feleslegeset, hogy újat adhass hozzá.
error-user-in-recurring = Ez a résztvevő egy ismétlődő kiadás része. Előbb vedd ki belőle, vagy állítsd le a kiadást.
