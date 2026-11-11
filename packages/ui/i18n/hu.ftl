# Magyar. Teljes, a jogi szövegek (legal-, terms-, privacy-) kivételével, amelyek csak angolul és
# franciául léteznek, és üzenetenként az en.ftl-re esnek vissza.

### Common

loading = Betöltés…
cancel = Mégse
discard-title = Elveted a módosításokat?
discard-message = Amit beírtál, elvész.
discard = Elvetés
confirm = Megerősítés
retry = Újra
delete = Törlés
back = Vissza
language = Nyelv
theme = Téma
theme-light = Világos
theme-dark = Sötét
theme-system = Rendszer
theme-hint-device = Csak ezen az eszközön van mentve.
theme-hint-system = Az eszköz megjelenését követi.

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
error-rate-limited = A szerver túlterhelt. Próbáld újra egy kicsit később.
error-generic = Valami hiba történt. Próbáld újra.

error-invalid-email = Ez az e-mail-cím érvénytelen.
error-invalid-password = Ez a jelszó érvénytelen.
error-password-too-short = A jelszónak legalább 12 karakter hosszúnak kell lennie.
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
login-maybe-unverified = Most regisztráltál? Előbb erősítsd meg a címed az általunk küldött linkkel, vagy kérd újra.
login-resend = Megerősítő link újraküldése
login-resending = Küldés…
login-resend-sent = E-mail elküldve - nézd meg a postaládád.

register-submit = Fiók létrehozása
register-submitting = Létrehozás…
register-have-account = Már van fiókod?
register-password-placeholder = Legalább 12 karakter
register-password-warning = Írd fel a jelszavad: mi sosem látjuk, és nem tudjuk visszaadni neked.
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
settings-anonymous-title = Nem vagy bejelentkezve
settings-upsell-title = A projektjeid, minden eszközön
settings-upsell-free = Ingyenes

account-logout = Kijelentkezés
account-logging-out = Kijelentkezés…
account-delete-title = Fiókom törlése
account-delete-confirm-title = Fiók törlése
account-delete-confirm-message = A fiókod, a munkameneteid és a projektlistád végleg törlődik. A jelszavad nélkül a megosztott projekt titkosított adatai olvashatatlanná válnak számodra - ez nem vonható vissza.
erase-device-title = Törlöd ezt az eszközt?
erase-device-body = Az eszközön lévő projektek, kulcsaik és a gyorsítótár törlődnek, majd a munkamenet véget ér. A projektjeid megmaradnak a fiókodban: jelentkezz be újra, hogy visszakapd őket.
erase-device-checking = Ellenőrzöm, hogy a projektjeid a fiókodban vannak-e…
erase-device-pending = Elküldésre váró módosítások: { $count }. Csatlakozz újra az internethez, és próbáld újra.
erase-device-unreachable = Nem sikerült ellenőrizni, hogy a projektjeid a fiókodban vannak-e. Ellenőrizd a kapcsolatot, és próbáld újra.
erase-device-at-risk = Ezek a projektek nincsenek a fiókodban, és elvesznek. Ha meg akarod tartani őket, szakítsd meg, és előbb másold ki a linkjüket:
erase-device-confirm = Törlés és kijelentkezés
erase-device-confirm-anyway = Törlés mégis
erase-device-erasing = Törlés…

settings-payment-methods = Fizetési adatok
payment-method-kind = Mód
payment-method-kind-other = Egyéb
payment-method-label = Név
payment-method-label-placeholder = Fő számla
payment-method-value = Adatok
payment-method-value-placeholder = IBAN, telefonszám, felhasználónév…
payment-method-add = Hozzáadás
payment-method-deleted = Fizetési mód törölve.
payment-method-value-required = Töltsd ki minden fizetési mód adatait, vagy távolítsd el.
payment-method-label-required = Adj nevet az egyéni fizetési módnak.
payment-method-too-long = Ez túl hosszú - rövidítsd le.
payment-method-invalid-characters = Távolítsd el a sortöréseket vagy láthatatlan karaktereket.
payment-method-limit = Legfeljebb { $max } fizetési módot menthetsz.
payment-methods-saved = Fizetési adatok mentve.
payment-methods-offline = A fizetési adatok mentéséhez online kell lenned.
payment-methods-stale = A fizetési adataid egy másik eszközön módosultak. Újratöltöttük őket - próbáld újra.
payment-methods-key-missing = Jelentkezz be újra a fizetési adatok kezeléséhez.
payment-method-copy = { $name } másolása
payment-method-copied = Másolva.
payment-method-copy-failed = Nem sikerült másolni - jelöld ki a szöveget és másold kézzel.

verify-email-checking = E-mail-cím megerősítése…
verify-email-welcome = E-mail megerősítve - üdv a Countedben!
verify-email-back-to-login = Vissza a bejelentkezéshez
verify-email-intro = Add meg a jelszavad, hogy megerősítsd: ez a cím a tiéd.
verify-email-submit = Címem megerősítése

### Project status

project-archive = Archiválás
project-sheet-invite = Meghívás
project-sheet-recurring = Ismétlődő
project-sheet-edit = Projekt szerkesztése
project-sheet-close = Projekt lezárása
project-sheet-archive = Projekt archiválása
project-sheet-reopen = Projekt újranyitása
project-sheet-unarchive = Projekt visszaállítása archívumból
project-sheet-leave = Kilépés a projektből

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
projects-offline-banner = Offline adatok - frissítéshez csatlakozz újra.
demo-banner = Bemutató projekt - csak olvasható.
demo-start-own = Saját projekt indítása
demo-badge = Demó
demo-bar = A módosításaid eltűnnek, amikor bezárod ezt a lapot.
demo-leave = Kezdés élesben
demo-preparing = A demó előkészítése…
demo-failed = A demót nem sikerült előkészíteni.
demo-sign-in-blocked = A demóban a bejelentkezés ki van kapcsolva. Lépj ki belőle a bejelentkezéshez vagy fiók létrehozásához; a saját projektjeid érintetlenek maradnak.
projects-no-local-data = Nincsenek helyi adatok
projects-no-local-data-hint = Jelentkezz be a projektjeid első betöltéséhez.
projects-add = Projekt hozzáadása
projects-create = Projekt létrehozása
projects-personal-expense = Saját kiadás
personal-expense-banner = Hozzáadva a Saját kiadásokhoz
personal-create-title = Hozd létre a saját projekted
personal-create-hint = Kövesd, mit költesz egyedül
group-create-title = Hozz létre közös projektet
group-create-hint = Oszd meg a kiadásokat barátokkal és családdal
personal-create-action = Létrehozás
personal-create-signup = Fiók létrehozása
personal-sheet-title = A saját projekted
personal-sheet-intro = Arra, amit egyedül költesz. A csoportjaiddal együtt jelenik meg a Statisztikában.
personal-sheet-point-alone = Egy tag: te, a fiókod nevével. Nincs megosztási link.
personal-sheet-point-encrypted = Az eszközeiden titkosítva, mint a többi projekted.
personal-sheet-point-stats = A csoportokban rád eső részek a Statisztikában hozzáadódnak ezekhez a kiadásokhoz.
personal-default-name = Saját kiadások
my-spending-prev-month = Előző hónap
my-spending-next-month = Következő hónap
my-spending-spent = Elköltött összeg
my-spending-groups = Csoportok
my-spending-income = Bevételek
my-spending-balance = A hónap egyenlege
my-spending-unconverted = A más pénznemű részek kimaradnak, amíg nincs ismert árfolyam.
my-spending-empty = Ebben a hónapban semmi
my-spending-empty-hint = Adj hozzá kiadást a + gombbal. A csoportokban lévő részeid automatikusan itt jelennek meg.
my-spending-share-of = { $amount } összegből
my-spending-uncounted = nem számít bele
payer-you = te
expense-received-by = kapta:
counted-switch = Beszámítás ide: Saját kiadások
counted-switch-on = Be: a részed beleszámít ide: Saját kiadások
counted-switch-off = Ki: a részed nem jelenik meg itt: Saját kiadások
counted-groups = Beszámított csoportok
counted-groups-intro = A részed ezekben a csoportokban beleszámít ide: Saját kiadások, és a költségkeretekbe is.
counted-groups-converted = { $from }, átváltva { $to } pénznemre
counted-groups-encrypted = Ez a választás a fiókoddal titkosított, és követ az eszközeiden.
counted-groups-empty = Ezen az eszközön még nincs csoport.
counted-groups-done = OK
counted-save-failed = Ezt a választást nem sikerült menteni. Próbáld újra.
tab-budget = Költségkeret
budget-title = Költségkeretek
budget-manage = Kezelés
budget-left = Maradt { $amount }
budget-over = Túllépve { $amount } összeggel
budget-empty = Nincs költségkeret
budget-empty-hint = Állíts be felső határt kategóriánként. A csoportokban lévő részeid is beleszámítanak.
budget-define = Költségkeret beállítása
budget-sheet-title = Havi költségkeretek
budget-sheet-intro = A részedre: saját és csoportok. Hagyd üresen, ha nem követnél egy kategóriát.
budget-average = Átl. az előző 3 hónapban: { $amount }
budget-for = Költségkeret: { $category }
budget-alert = Figyelmeztess 80 %-nál
budget-alert-hint = Ezen az eszközön számolva
budget-invalid = Érvénytelen összeg: { $category }.
budget-offline = A költségkeretek csak online menthetők. Csatlakozz újra, és próbáld meg ismét.
budget-alert-near = { $category }: a havi keret { $percent } %-a
budget-alert-over = { $category }: a havi keret túllépve
my-spending-search-summary =
    { $count ->
        [one] { $count } találat
       *[other] { $count } találat
    } · { $total } · saját és csoportok, minden hónap
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
expenses-unreadable = Néhány tétel nem olvasható ezen az eszközön, és kimarad az összegekből. Frissítsd az alkalmazást, hogy lásd őket.
project-unlock = Feloldás
project-no-local-data-hint = Jelentkezz be a projekt adatainak első betöltéséhez.
project-gone-title = Ez a projekt már nem létezik
project-gone-hint = Törlődött, amikor az utolsó tagja kilépett. A megosztási link már nem működik, akkor sem, ha újra megnyitod.

expense-add = Kiadás hozzáadása
transfer-add = Átutalás hozzáadása
expense-edit-title = Kiadás szerkesztése
expense-category = Kategória
expense-category-auto = Auto · { $emoji }
emoji-picker-label = Emoji
emoji-none = Nincs emoji
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
expense-type-group = Típus
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
balance-gets-back = Visszakap
balance-owes = Tartozik
balance-settled = Rendezve
reimbursements-empty-title = Minden el van számolva!
reimbursements-empty-hint = Az elszámolási javaslatok itt jelennek meg, ha az egyenlegek nem egyeznek
reimbursement-owes = { $debtor } tartozik { $creditor } részére
reimbursement-record = Elszámolás
settlement-recorded = Elszámolás rögzítve.
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
move-expense = Áthelyezés projektbe
move-expense-title = Áthelyezés ide…
move-current = Jelenlegi
move-converted = { $currency }, havi árfolyamon átváltva
move-note-personal = A következő lépésben választod ki, ki fizetett és kinek. A kiadás kikerül a Saját kiadásokból.
move-note-group = A kiadás kikerül a(z) { $project } projektből minden tag számára, és a csoport egyenlege újraszámolódik. A következő lépésben választod ki, ki fizetett és kinek.
move-banner = { $from } → { $to }
move-submit = Áthelyezés
move-done = Kiadás áthelyezve ide: { $project }
history-expense-moved = Kiadás áthelyezve egy másik projektbe: { $name }
personal-badge = Saját
move-members =
    { $count ->
        [one] { $count } tag
       *[other] { $count } tag
    }
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
expense-share-percent = { $percent }%
expense-added-by = Hozzáadta: { $name }, { $date }
expense-inconsistent-detail = Az összegek nem egyeznek: { $paid } fizetve, { $owed } tartozás, egy { $total } összegű kiadásnál. Szerkeszd a kiadást a javításhoz.
missing-access-key = Hiányzik a hozzáférési kulcs. Nyisd meg a projektet a megosztási linkjén keresztül.
filter-all = Összes
filter-my-payments = Saját fizetéseim
filter-my-debts = Amivel tartozom
expense-filter-group = Szűrő
expenses-search = Kiadások keresése
expenses-no-match = Nincs egyező kiadás.
expenses-search-open = Keresés
expenses-search-clear = Keresés törlése
expenses-search-summary =
    { $count ->
        [one] { $count } találat
       *[other] { $count } találat
    } · { $total } · minden hónap
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
charts-my-share-skipped =
    { $count ->
        [one] 1 projekt nincs beszámítva - nincs kiválasztott résztvevő, vagy az adatai nem töltődtek be.
       *[other] { $count } projekt nincs beszámítva - nincs kiválasztott résztvevő, vagy az adataik nem töltődtek be.
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
categories-manage = Kategóriák kezelése
categories-title = Kategóriák
categories-yours = A te kategóriáid
categories-builtin = Alapértelmezett
categories-add = Kategória hozzáadása
categories-in-parent = { $parent } kategóriában
categories-renamed-from = átnevezve · eredetileg { $name }
categories-hidden-hint = elrejtve · kiadásai ide kerülnek: { $other }
categories-always-shown = mindig látható
categories-hidden = Elrejtve
categories-new = Új kategória
categories-edit = Kategória szerkesztése
categories-parent = Ebbe számít bele
categories-parent-hint = Minden projekted statisztikájában „{ $name }” ide számít bele: { $parent }.
categories-create = Létrehozás
categories-hide = Kategória elrejtése
categories-hide-hint = Kiadásai minden tag számára itt jelennek meg: { $other }.
categories-delete-title = Törlöd ezt: „{ $name }”?
categories-delete-body = Az érintett kiadások ({ $count }) visszakerülnek ide: { $parent }. Egyetlen kiadás sem törlődik.
categories-delete-body-unknown = Kiadásai visszakerülnek ide: { $parent }. Egyetlen kiadás sem törlődik.
categories-delete-warning = A kategória a projekt minden tagjánál eltűnik.
categories-summary-created = Létrehozva: { $count }
categories-summary-renamed = Átnevezve: { $count }
categories-summary-hidden = Elrejtve: { $count }
categories-summary-default = Nincsenek saját kategóriák
categories-offline = A kategóriák csak online kezelhetők. Csatlakozz újra, és próbáld meg ismét.
categories-error-too-many = Egy projektnek legfeljebb 20 saját kategóriája lehet.
categories-error-full = Ez a projekt elérte a kategóriák korlátját.
categories-error-name = Adj nevet a kategóriának.
categories-error-missing = Ez a kategória közben megváltozott. Zárd be, és próbáld újra.
history-category-created = Kategória létrehozva: { $name }
history-category-edited = Kategória szerkesztve: { $name }
history-category-deleted = Kategória törölve: { $name }
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
charts-paid-more = { $amount } többet, mint az én részem
charts-paid-less = { $amount } kevesebbet, mint az én részem
charts-paid-even = Pont az én részem
charts-part-title = A részed minden kategóriából
charts-part-desc = Szürke a csoport költése, színes a te fogyasztásod.
charts-breakdown-title = Bontás kategóriák szerint
charts-breakdown-desc = Koppints egy szeletre vagy sorra a kiadásokért.
charts-of-total = { $amount } / { $total }
charts-show-all = Összes mutatása ({ $count })
charts-show-less = Kevesebb
charts-spend-title = Kiadások időben
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
charts-projects-desc = Az összegek pénznemenként külön maradnak.
history-empty = Nincs esemény
history-by = { $name }
not-found-hint = Ez az oldal nem létezik, vagy elköltözött.
payers-title-paid-by = Fizette
payers-title-sender = Küldő
payers-title-contributors = Hozzájárulók
debtors-title-debtors = Tartozik
debtors-title-recipients = Címzettek
debtors-title-beneficiaries = Kedvezményezettek
debtors-title-for = Kinek
debtors-title-recipient = Címzett
expense-split-between = { $count } fő között elosztva
expense-people =
    { $count ->
        [one] { $count } fő
       *[other] { $count } fő
    }

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
help-forgot-password-warning = Senki sem tudja helyetted visszaszerezni.
help-forgot-password-a = A jelszavad védi a titkosítási kulcsaidat, és sosem kerül hozzánk: senki - mi sem - nem tudja helyetted visszafejteni a projektjeidet. Őrizd biztonságban, ideális esetben jelszókezelőben.
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
project-currency-locked = A pénznem a projekt létrehozásakor rögzül.
currency-search = Pénznem keresése

update-required-title = Frissítés szükséges
update-required-body = A Counted ezen verziója túl régi a szerverrel való kommunikációhoz. Frissítsd az alkalmazás további használatához.
update-required-button = Frissítés

notifications-label = Értesítések
notifications-title = Értesítések
notifications-empty = Semmi új

friends-title = Barátok
friends-anonymous-body = A barátok a fiókodhoz tartoznak. Jelentkezz be, hogy személyeket adj hozzá és meghívd őket a projektjeidbe link megosztása nélkül.
friends-add-title = Barát hozzáadása
friends-add-from-project = Hozzáadás barátként
friends-request-sent = Kérelem elküldve
friends-no-account-key = Jelentkezz be újra a barátaid kezeléséhez ezen az eszközön.
friends-incoming-title = Kérelmek
friends-accept = Elfogadás
friends-decline = Elutasítás
friends-list-title = Barátaim
friends-remove = Eltávolítás
friends-remove-confirm-title = Barát eltávolítása
friends-remove-confirm-message = { $email } többé nem lesz a barátaid között, és te sem az övéi között. Bármelyikőtök küldhet később új kérést.
friends-no-key = Még nem áll készen
friends-key-mismatch = A szerver olyan nyilvános kulcsot küldött a fiókodhoz, amely nem illik a privát kulcsodhoz. A biztonsági kódod rejtve van, a meghívások ki vannak kapcsolva ezen az eszközön: lehet, hogy valaki manipulálja a szervert.
keypair-mismatch = A szerver által a fiókodhoz küldött kulcsok nem egyeznek az ezen az eszközön lévőkkel. A meghívók és a kulcscserék itt ki vannak kapcsolva, amíg nem egyeznek: lehet, hogy valaki manipulálja a szervert.
friends-outgoing-title = Elküldve
friends-notify = Szólj üzenetben
friends-notify-message = Átállok a Countedre a közös kiadásainkhoz. A barátkérelmem a https://counted.fr oldalon vár, ingyenes regisztráció után, ezzel a címmel: { $email }
friends-notify-copied = Üzenet másolva
friends-notify-copy-failed = Nem sikerült másolni az üzenetet
guests-hint-title =
    { $count ->
        [one] { $count } résztvevő még nem csatlakozott a projekthez
       *[other] { $count } résztvevő még nem csatlakozott a projekthez
    }
guests-hint-later = Később
guests-hint-send = Hívd meg őket
guests-send-title = Projekt küldése a résztvevőknek
guests-send-friend = Counted-barát meghívása
guests-send-friend-badge = Link nélkül
guests-send-friend-hint = Ha már van fiókja, titkosított meghívót kap; semmi sem megy át az üzenetküldődön.
guests-send-link = Projektlink megosztása
guests-send-link-hint = Azoknak, akiknek nincs fiókjuk. Egy koppintással megnyitják a projektet.
guests-send-link-warning = Bárki, akinél ez a link van, olvashatja és módosíthatja a projektet. Az üzenetküldőd is látja.
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
participants-others = További résztvevők
participants-removed = Eltávolítva
participant-restore = Visszaállítás
participants-empty = Adj hozzá résztvevőket lent.
participants-duplicate = „{ $name }” már szerepel a listában.
participants-input-label = Barát hozzáadása vagy név beírása
participants-input-placeholder = Barát vagy bármilyen név
participants-suggest-friend = Barát · „{ $name }” néven csatlakozik, meghívót kap
participants-suggest-not-ready = Barát · még nem áll készen
participants-suggest-guest = „{ $text }” hozzáadása
participants-suggest-guest-sub = Vendég
participants-friends = A barátaid
participants-all-friends = Minden barát
participants-login-hint = Jelentkezz be, hogy közvetlenül a barátlistádból adhass hozzá embereket.
participants-invite-badge = Meghívás
participants-guest-badge = Vendég
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
edit-project-you-are = Ebben a projektben te vagy: { $name }
edit-project-no-identity = Még nem választottad ki, ki vagy
edit-project-switch = Váltás
edit-project-choose = Kiválasztás
invite-failed = Ezeket a meghívókat nem sikerült elküldeni: { $emails }
invite-waiting = Ezeknek a barátoknak egyszer meg kell nyitniuk a Countedot, mielőtt meghívhatók: { $emails }
key-offer-title = Másik kulcs ehhez: { $name }
key-offer-body = Ezen az eszközön egy másik kulcs van ehhez a projekthez, amely itt még soha nem nyitotta meg.
key-offer-choices = Kulcs használata: a projekt ennek a linknek vagy meghívónak a kulcsával nyílik meg. Mégse: az eszköz megtartja a jelenlegi kulcsát, és semmi sem változik.
key-offer-use = Kulcs használata
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
expense-pending = Függőben
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

home-owed-to-you = Tartoznak neked
home-you-owe = Tartozol
home-details-show = Részletek
home-details-hide = Részletek elrejtése
home-all-settled = Minden rendezve
home-pay = Fizetés
home-remind = Emlékeztetés
home-remind-message = Kis emlékeztető: a(z) { $project } projektben még vannak rendezetlen visszafizetések.
home-row-owed = tartoznak neked
home-row-owe = tartozol
home-row-settled = Rendezve
home-row-quits = Nálad rendezve
home-row-pending =
    { $count ->
        [one] { $count } visszafizetés függőben
       *[other] { $count } visszafizetés függőben
    }
home-totals-partial = Árfolyam nélküli projektek nélkül
error-account-key-changed = A fiókod kulcsa megváltozott egy másik eszközön. Jelentkezz be újra a folytatáshoz.
settings-security = Biztonság
settings-change-password-hint = A projektjeid és adataid változatlanok maradnak. A többi eszközödön kijelentkezel.
change-password-title = Jelszó módosítása
change-password-current = Jelenlegi jelszó
change-password-new = Új jelszó
change-password-confirm = Új jelszó megerősítése
change-password-mismatch = A két új jelszó nem egyezik.
change-password-submit = Jelszó módosítása
change-password-submitting = Módosítás…
change-password-done = Jelszó módosítva. A többi eszközödön kijelentkeztél.
error-rotation-incomplete = A fiókod közben megváltozott. Próbáld újra.
error-key-lifecycle-unavailable = Még nem érhető el: előbb mindenkinek a legújabb alkalmazásverzió kell.
settings-compromised-open = Fiókom védelme
compromised-title = Fiókod védelme
compromised-body = A jelszavad és a fiókod kulcsai lecserélődnek, és minden más eszközön kijelentkezel. A barátaid látni fogják, hogy a biztonsági kódod megváltozott: hasonlítsátok össze újra.
compromised-keeps = Aki ismerte a jelszavadat, megtartja, amit már megnyitott: a projekteket, amelyeket eddig olvashatott.
compromised-submit = Jelszó és kulcsok cseréje
compromised-done = A fiókod védett. Kérd meg a barátaidat, hogy hasonlítsák össze az új biztonsági kódodat.
friends-code-matches = A kód egyezik
friends-code-trusted = Kód megerősítve.
settings-recovery-key = Helyreállítási kulcs
settings-recovery-key-pending = Egy új kulcs váltja le { $when }. Nem te voltál? Vond vissza a jelenlegi kulcsoddal.
recovery-key-cancel-replacement = Új kulcs visszavonása
recovery-key-title = A helyreállítási kulcsod
recovery-key-body = Írd le, vagy mentsd olyan helyre, amelyhez csak te férsz hozzá. A Counted nem tudja újra megmutatni vagy visszaszerezni helyetted.
recovery-key-copy = Másolás
recovery-key-copied = Másolva
recovery-key-save = Mentés fájlként
recovery-key-print = Nyomtatás
recovery-key-next = Tovább
recovery-key-confirm-body = A másolatod ellenőrzéséhez írd be a kulcs { $n }. csoportját.
recovery-key-confirm-wrong = Ez a csoport nem egyezik. Nézd meg újra a másolatod.
recovery-key-password-body = Add meg a jelszavad, hogy a kulcsot a fiókodba mentsd.
recovery-key-submit = Kulcs mentése
recovery-key-done = Helyreállítási kulcs mentve.
recovery-key-replacement-done = Új kulcs mentve. 72 óra múlva váltja a jelenlegit; addig a jelenlegi még működik.
recovery-key-field = Helyreállítási kulcs
recovery-key-file-intro = Counted helyreállítási kulcs
recovery-key-cancel-done = Az új kulcs visszavonva.
compromised-new-key = Mentsd el most az új helyreállítási kulcsod: 72 óra múlva váltja a régit, addig a régi még működik.
pending-reset-title = Visszaállítást kértek
pending-reset-body = { $when } időpontban lép életbe. Nem te voltál? Vond vissza a jelszavaddal.
pending-reset-cancel = Visszaállítás visszavonása
pending-reset-cancelled = A visszaállítás visszavonva.
login-forgot = Elfelejtett jelszó?
forgot-title = Elfelejtett jelszó
forgot-body = Add meg az e-mail-címed. Ha tartozik hozzá fiók, küldünk egy linket új jelszó választásához.
forgot-submit = Link küldése
forgot-sent = Ha ehhez a címhez tartozik fiók, úton van egy link. 24 óra múlva lejár.
reset-title = Szerezd vissza a fiókod
reset-with-key-body = Add meg a helyreállítási kulcsot és egy új jelszót: minden úgy tér vissza, ahogy volt.
reset-submit-key = Fiókom helyreállítása
reset-no-key = Nincs meg a helyreállítási kulcsom
reset-without-key-title = Visszaállítás helyreállítási kulcs nélkül
reset-without-key-body = A fiókod 72 óra múlva új jelszóval indul újra. A projektjeid visszajönnek az eszközeidről és a megosztási linkekből; a beállítások, fizetési adatok és a barátok kódjai nem. Addig a jelenlegi jelszavad működik, és vele visszavonhatod a visszaállítást.
reset-name = A neved
reset-submit-request = Fiókom visszaállítása
reset-requested = Kész. Az új jelszavad { $when } időponttól működik. Addig a jelenlegi működik, és visszavonhatja.
reset-pending-already = Már vár egy visszaállítás { $when } időpontig. Egy új kérés újraindítja a 72 órát.
recovered-done = Visszakaptad a fiókod. A többi eszközödön kijelentkeztél.
login-reset-undoable = Ezt a jelszót az elmúlt 30 napban egy visszaállítás váltotta le. Ha nem te kérted, vond vissza: a fiókod úgy tér vissza, ahogy volt.
login-undo-reset = Visszaállítás visszavonása
reset-undone = A visszaállítás visszavonva. A fiókod olyan, mint előtte.
error-reset-pending = Az új jelszó akkor működik, amikor a visszaállítás életbe lép, 72 órával a kérés után.
error-account-disabled = Ezt a fiókot nem sokkal egy visszaállítás után törölték, és törlésre vár. Az előző jelszó még visszavonhatja a visszaállítást.
error-recovery-key-invalid = Ez a helyreállítási kulcs nem nyitja meg ezt a fiókot.
error-project-key-required = Ehhez a projekthez kell a kulcsa. Nyisd meg újra a megosztási linkjéről.
names-or = { $first } vagy { $last }
participants-admin-badge = Admin
edit-project-admins = Adminok: { $count }
edit-project-succession = Ha az utolsó admin kilép a projektből, a legrégebben ott lévő, fiókkal rendelkező tag lesz az admin.
member-sheet-account = Fiókkal rendelkező tag
member-actions = Műveletek: { $name }
member-make-admin = Admin szerep megadása
member-make-admin-hint = Ezután megváltoztathatja a projekt kulcsát és eltávolíthat valakit.
member-made-admin = { $name } mostantól admin.
group-security = Projektbiztonság
group-security-admin = Projektbiztonság · admin vagy
group-rotate = Projektkulcs cseréje
group-rotate-hint = Ha a projekt linkje kiszivárgott: a régi link birtokosai elveszítik a hozzáférést a projekthez.
group-remove = Valaki eltávolítása a projektből
group-remove-hint = Megvonja a hozzáférést, és közben a kulcsot is lecseréli.
group-admins-only = Csak adminoknak.
group-admins-only-account = Csak adminoknak, akiknek van fiókjuk.
group-ask-admin = Kérdezd a projekt adminját: { $names }.
group-ask-admins = Kérdezd a projekt adminjait: { $names }.
group-ask-an-admin = Kérdezd a projekt egyik adminját.
group-with-account = Fiókkal
group-free = Ingyenes
group-sign-in-hint = A projekt egyik adminja ({ $names }) ezután megadhatja neked a szerepet.
group-sign-in-hint-any = A projekt egyik adminja ezután megadhatja neked a szerepet.
group-no-admin = Ennek a projektnek nincs adminja, így a kulcsa nem cserélhető.
group-hide = Valaki elrejtése
group-hide-hint = Elrejti a listákból anélkül, hogy megvonná a hozzáférést: a link és a kulcs továbbra is működik.
group-recreate-hint = Ennek a projektnek nincs adminja. Ha valakit el akarsz távolítani, hozz létre egy új projektet.
group-recreate = Új projekt létrehozása
group-no-admin-yet = Ennek a projektnek még nincs adminja.
group-creator-hint = Csak te lehetsz ennek a projektnek az adminja, ha létrehozol egy fiókot.
group-creator-elsewhere-hint = Csak a projekt létrehozója lehet az adminja, ha létrehoz egy fiókot. Különben hozz létre egy új projektet, ha valakit el akarsz távolítani.
member-revoke-admin = Admin szerep visszavonása
member-revoke-admin-hint = Többé nem változtathatja meg a projekt kulcsát, és nem távolíthat el senkit.
member-revoked-admin = { $name } már nem admin.
member-step-down = Lemondás az admin szerepről
member-step-down-hint = Többé nem változtathatod meg a projekt kulcsát, és nem távolíthatsz el senkit.
member-stepped-down = Már nem vagy admin.
member-remove = Eltávolítás a projektből
member-remove-hint = Megváltoztatja a projekt kulcsát: { $name } többé semmi újat nem olvashat.
member-hide = Elrejtés a listákból
error-user-not-admin = Ez a tag nem admin.
error-creator-proof-invalid = Nem ez az eszköz hozta létre ezt a projektet.
error-project-has-admin = Ennek a projektnek már van adminja.
error-not-project-admin = Ezt csak a projekt adminjai tehetik meg.
error-user-not-claimed = Ennek a résztvevőnek nincs fiókja, ezért nem lehet admin.
error-project-key-rotated = A projekt kulcsa megváltozott. Kérd el az új linket egy tagtól.
error-project-rotating = A projekt kulcsa éppen változik. Próbáld újra egy pillanat múlva.
error-rotation-expired = A kulcscsere túl sokáig tartott, és megszakadt. Kezdd újra.
error-rotation-rows-changed = A projekt megváltozott a kulcscsere közben. Kezdd újra.
project-rotated-title = Már nincs hozzáférésed ehhez a projekthez
project-rotated-body = A projekt kulcsát az admin megváltoztatta. Kérd el tőle az új linket.
project-rotated-open-link = Link megnyitása
rotation-remove-title = { $name } eltávolítása a projektből?
rotation-rotate-title = Lecseréled a projektkulcsot?
rotation-remove-body = Egy tag eltávolításakor új kulcs készül. Az új linket meg kell osztanod azokkal, akiknek nincs fiókjuk.
rotation-rotate-body = A régi link birtokosai elveszítik a hozzáférést a projekthez. Az új linket meg kell osztanod azokkal, akiknek nincs fiókjuk.
rotation-boxed =
    { $count ->
        [0] Más tag nem kapja meg automatikusan a kulcsot
       *[other] { $count } tag automatikusan megkapja a kulcsot
    }
rotation-by-link-title = Akiknek el kell küldened az új linket
rotation-by-link-hint = Nekik nincs fiókjuk. Rögtön ezután elküldöd nekik a linket.
rotation-names-and = { $first } és { $last }
rotation-names-and-others = { $names } és még { $count } fő
rotation-confirm-remove = Eltávolítás és kulcscsere
rotation-confirm-rotate = Kulcscsere
rotation-done-title = Az új kulcs érvényben van
rotation-done-removed = { $name } már nem tagja a projektnek.
rotation-done-rotated = A projektkulcs megváltozott.
rotation-done-boxed =
    { $count ->
        [0] Más tag nem kapta meg automatikusan a kulcsot.
       *[other] { $count } tag automatikusan megkapta a kulcsot.
    }
rotation-new-link =
    { $count ->
        [0] A projekt új linkje
       *[other] Elküldendő új link · { $count } fiók nélküli személy
    }
rotation-link-renewed = A link sikeresen megújult.
rotation-done = Kész
history-key-changed = Projektkulcs lecserélve
history-member-removed = { $name } már nem tagja a projektnek
project-rotated-row-hint = A kulcs megváltozott. Kérd el az új linket.

## Settings and Friends redesign (docs/plans/friends-settings-redesign.md)
settings-group-sharing = Megosztás
settings-group-help = Súgó és információk
settings-friends-changed = { $count ->
    [one] 1 kód megváltozott
   *[other] { $count } kód megváltozott
}
settings-recovery-key-created = Létrehozva
settings-recovery-key-missing = Létrehozandó
settings-recovery-key-replacing = Csere folyamatban
settings-password = Jelszó
settings-compromised-short = Más is ismeri a jelszavad?
settings-recovery-key-footnote = Helyreállítási kulcs nélkül egy elfelejtett jelszó törli a beállításaidat, a fizetési adataidat és a barátok kódjait.
erase-device-row = Az eszköz törlése
erase-device-row-hint = Kijelentkeztet, és csak a nyelvet tartja meg.
account-delete-hint = Azonnali és végleges.
settings-open-source = Nyílt forráskód, AGPL-3.0
settings-upsell-short = Fiók nélkül a projektjeid ezen az eszközön maradnak. Fiókkal mindenhová követnek, titkosítva.
settings-locked-group = Fiókkal
settings-locked-friends-short = Meghívás link nélkül
settings-locked-payment-short = Azok látják, akik tartoznak neked
payment-methods-title = Fizetési módok
payment-methods-row-hint = Megosztva a barátaiddal
payment-methods-intro = Megosztva a barátaiddal. A fiókoddal titkosítva.
payment-methods-footnote = A fizetési módot (legfeljebb 20) a projektjeid tagjaival osztod meg, az Elszámolás lapon.
payment-method-shared-pill = Megosztva
payment-method-share-groups = Megosztás a csoportjaidban
payment-method-label-optional = Név (nem kötelező)
payment-method-delete-confirm-title = Törlöd ezt a fizetési módot?
payment-method-delete-confirm-message = { $name } többé nem jelenik meg a projektjeidben.
payment-methods-empty-title = Nincs fizetési mód
payment-methods-empty-body = Add hozzá az IBAN-odat vagy a fizetési alkalmazásodat: akik tartoznak neked, látni fogják.
payment-method-masked = { $kind }, vége: { $last }
friends-to-check = Ellenőrizendő
friends-code-changed-row = Megváltozott a kódja
friends-own-code = A biztonsági kódod
friends-own-code-hint = A barátaid ezt a kódot látják a címed mellett. Ha egyezik az övékkel, téged hívnak meg, és senki mást.
friends-received = Beérkezett
friends-received-on = Beérkezett: { $date }
friends-pending = Függőben
friends-from-project = Projektből
friends-accept-label = { $email } kérelmének elfogadása
friends-decline-label = { $email } kérelmének elutasítása
friends-add-hint-short = Ha van fiókja, látni fogja a kérelmedet. Ha nincs, szólj neki.
friends-send = Küldés
friends-sent-hint = Még nincs fiókja? Szólj neki: az üzenet arra hívja, hogy ezzel a címmel regisztráljon.
friends-done = Kész
friends-their-code = A biztonsági kódja
friends-code-compare-hint = Olvassátok fel együtt. Neki is ugyanezt a kódot kell látnia a Barátok oldala tetején.
friends-no-key-hint = Még nem áll készen: a kulcsa akkor jön létre, amikor legközelebb megnyitja a Countedet. Addig nem hívhatod meg.
friends-remove-row = Eltávolítás a barátaim közül
friends-new-code = Új kód
friends-key-changed-hint-short = Ellenőrizzétek ezt a kódot együtt, ezen az alkalmazáson kívül. Addig a meghívók vele le vannak tiltva.
friends-kem-changed-title = A kulcsát lecserélték
friends-kem-changed-body = A kulcsa poszt-kvantum része megváltozott, a kódja viszont nem. A fiók védelme mindkettőt megváltoztatja, így lehet, hogy valaki kicserélte ezt a kulcsot. A meghívók vele továbbra is le vannak tiltva.
friends-kem-changed-hint = Eltávolíthatod, majd új kérelmet küldhetsz: a kulcsa újra rögzítve lesz.
friends-pending-sheet = Kérelem elküldve, függőben.
friends-withdraw-row = Kérelem visszavonása
friends-empty-title = Még nincsenek barátaid
friends-empty-body = Adj hozzá valakit e-mailben, majd hívd meg egy projektbe link megosztása nélkül.
friends-requests-empty-title = Nincsenek kérelmek
friends-requests-empty-body = A kapott és küldött kérelmek itt jelennek meg.
friends-code-hidden = A kódod rejtve van.
friends-code-hidden-body = A fiókodhoz küldött egyik kulcs nem egyezik ezzel az eszközzel. A meghívók itt ki vannak kapcsolva.
notifications-manage-friends = Barátaim kezelése
notifications-friend-request-on = Barátkérelem · { $date }

### Link preview

share-preview-title = Közös kiadásaink a Countedon
share-preview-description = Nyisd meg a linket, és lásd a kiadásokat, meg hogy ki kinek mennyivel tartozik. Fiók nélkül, a saját eszközödön titkosítva.

### Currency names

currency-EUR = Euró
currency-AED = EAE-dirham
currency-AFN = Afgán afghani
currency-ALL = Albán lek
currency-AMD = Örmény dram
currency-AOA = Angolai kwanza
currency-ARS = Argentin peso
currency-AUD = Ausztrál dollár
currency-AWG = Arubai florin
currency-AZN = Azerbajdzsáni manat
currency-BAM = Bosznia-hercegovinai konvertibilis márka
currency-BBD = Barbadosi dollár
currency-BDT = Bangladesi taka
currency-BHD = Bahreini dinár
currency-BIF = Burundi frank
currency-BMD = Bermudai dollár
currency-BND = Brunei dollár
currency-BOB = Bolíviai boliviano
currency-BRL = Brazil real
currency-BSD = Bahamai dollár
currency-BTN = Bhutáni ngultrum
currency-BWP = Botswanai pula
currency-BYN = Belarusz rubel
currency-BZD = Belize-i dollár
currency-CAD = Kanadai dollár
currency-CDF = Kongói frank
currency-CHF = Svájci frank
currency-CLP = Chilei peso
currency-CNY = Kínai jüan
currency-COP = Kolumbiai peso
currency-CRC = Costa Rica-i colon
currency-CUP = Kubai peso
currency-CVE = Zöld-foki escudo
currency-CZK = Cseh korona
currency-DJF = Dzsibuti frank
currency-DKK = Dán korona
currency-DOP = Dominikai peso
currency-DZD = Algériai dínár
currency-EGP = Egyiptomi font
currency-ERN = Eritreai nakfa
currency-ETB = Etiópiai birr
currency-FJD = Fidzsi dollár
currency-FKP = Falkland-szigeteki font
currency-GBP = Angol font
currency-GEL = Grúz lari
currency-GHS = Ghánai cedi
currency-GIP = Gibraltári font
currency-GMD = Gambiai dalasi
currency-GNF = Guineai frank
currency-GTQ = Guatemalai quetzal
currency-GYD = Guyanai dollár
currency-HKD = Hongkongi dollár
currency-HNL = Hondurasi lempira
currency-HTG = Haiti gourde
currency-HUF = Magyar forint
currency-IDR = Indonéz rúpia
currency-ILS = Izraeli új sékel
currency-INR = Indiai rúpia
currency-IQD = Iraki dínár
currency-IRR = Iráni riál
currency-ISK = Izlandi korona
currency-JMD = Jamaicai dollár
currency-JOD = Jordániai dínár
currency-JPY = Japán jen
currency-KES = Kenyai shilling
currency-KGS = Kirgizisztáni szom
currency-KHR = Kambodzsai riel
currency-KMF = Comorei frank
currency-KPW = Észak-koreai won
currency-KRW = Dél-koreai won
currency-KWD = Kuvaiti dínár
currency-KYD = Kajmán-szigeteki dollár
currency-KZT = Kazahsztáni tenge
currency-LAK = Laoszi kip
currency-LBP = Libanoni font
currency-LKR = Srí Lanka-i rúpia
currency-LRD = Libériai dollár
currency-LSL = Lesothoi loti
currency-LYD = Líbiai dínár
currency-MAD = Marokkói dirham
currency-MDL = Moldován lei
currency-MGA = Madagaszkári ariary
currency-MKD = Macedón dénár
currency-MMK = Mianmari kyat
currency-MNT = Mongóliai tugrik
currency-MOP = Makaói pataca
currency-MRU = Mauritániai ouguiya
currency-MUR = Mauritiusi rúpia
currency-MVR = Maldív-szigeteki rufiyaa
currency-MWK = Malawi kwacha
currency-MXN = Mexikói peso
currency-MYR = Malajziai ringgit
currency-MZN = Mozambiki metikális
currency-NAD = Namíbiai dollár
currency-NGN = Nigériai naira
currency-NIO = Nicaraguai córdoba
currency-NOK = Norvég korona
currency-NPR = Nepáli rúpia
currency-NZD = Új-zélandi dollár
currency-OMR = Ománi riál
currency-PAB = Panamai balboa
currency-PEN = Perui sol
currency-PGK = Pápua új-guineai kina
currency-PHP = Fülöp-szigeteki peso
currency-PKR = Pakisztáni rúpia
currency-PLN = Lengyel zloty
currency-PYG = Paraguayi guarani
currency-QAR = Katari riál
currency-RON = Román lej
currency-RSD = Szerb dinár
currency-RUB = Orosz rubel
currency-RWF = Ruandai frank
currency-SAR = Szaúdi riál
currency-SBD = Salamon-szigeteki dollár
currency-SCR = Seychelle-szigeteki rúpia
currency-SDG = Szudáni font
currency-SEK = Svéd korona
currency-SGD = Szingapúri dollár
currency-SHP = Szent Ilona-i font
currency-SLE = Sierra Leone-i leone
currency-SOS = Szomáli shilling
currency-SRD = Suriname-i dollár
currency-SSP = Dél-szudáni font
currency-STN = São Tomé és Príncipe-i dobra
currency-SVC = Salvadori colón
currency-SYP = Szíriai font
currency-SZL = Szvázi lilangeni
currency-THB = Thai baht
currency-TJS = Tádzsikisztáni somoni
currency-TMT = Türkmenisztáni manat
currency-TND = Tunéziai dínár
currency-TOP = Tongai paanga
currency-TRY = Török líra
currency-TTD = Trinidad és Tobago-i dollár
currency-TWD = Tajvani új dollár
currency-TZS = Tanzániai shilling
currency-UAH = Ukrán hrivnya
currency-UGX = Ugandai shilling
currency-USD = USA-dollár
currency-UYU = Uruguayi peso
currency-UZS = Üzbegisztáni szom
currency-VES = Venezuelai bolivar
currency-VND = Vietnámi dong
currency-VUV = Vanuatui vatu
currency-WST = Nyugat-szamoai tala
currency-XAF = CFA frank BEAC
currency-XCD = Kelet-karibi dollár
currency-XCG = Caribbean Guilder
currency-XOF = CFA frank BCEAO
currency-XPF = Csendes-óceáni valutaközösségi frank
currency-YER = Jemeni riál
currency-ZAR = Dél-afrikai rand
currency-ZMW = Zambiai kwacha
currency-ZWG = Zimbabwe Gold
