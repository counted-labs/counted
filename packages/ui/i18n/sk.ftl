# Slovenčina. Kompletné s výnimkou právnych textov (legal-, terms-, privacy-), ktoré existujú len po
# anglicky a francúzsky a pri každej správe zvlášť sa vracajú k en.ftl.

### Common

loading = Načítava sa…
cancel = Zrušiť
discard-title = Zahodiť zmeny?
discard-message = Zadané údaje sa stratia.
discard = Zahodiť
confirm = Potvrdiť
retry = Skúsiť znova
delete = Odstrániť
back = Späť
language = Jazyk
theme = Motív
theme-light = Svetlý
theme-dark = Tmavý
theme-system = Systémový
theme-hint-device = Uložené len v tomto zariadení.
theme-hint-system = Riadi sa vzhľadom tohto zariadenia.

### Navigation

nav-main = Hlavná navigácia
nav-projects = Projekty
nav-charts = Štatistiky
nav-settings = Nastavenia

### Connectivity

offline-banner = Offline
offline-pending =
    { $count ->
        [one] { $count } čakajúca
        [few] { $count } čakajúce
        [many] { $count } čakajúcich
       *[other] { $count } čakajúcich
    }

sync-conflict-edit = Konflikt: úprava „{ $name }“ zlyhala (položka odstránená). Preskočené.
sync-conflict-delete = Konflikt: odstránenie „{ $name }“ zlyhalo (položka odstránená). Preskočené.
sync-conflict-other = Konflikt: operácia s „{ $name }“ zlyhala (položka odstránená). Preskočené.
sync-error = Chyba synchronizácie: { $reason }

### Errors

error-network = Server je nedostupný. Skontroluj pripojenie na internet.
error-rate-limited = Server je preťažený. Skús to znova o chvíľu.
error-generic = Niečo sa pokazilo. Skús to znova.

error-invalid-email = Táto e-mailová adresa nie je platná.
error-invalid-password = Toto heslo nie je platné.
error-password-too-short = Heslo musí mať aspoň 12 znakov.
error-client-outdated = Táto verzia aplikácie je zastaraná. Na prihlásenie ju aktualizuj.
error-invalid-link = Tento odkaz nie je platný.
error-batch-too-large = Príliš veľa položiek naraz.
error-payers-required = Vyber aspoň jedného platcu.
error-debtors-required = Vyber aspoň jednu osobu, ktorá dlhuje.
error-duplicate-participant = Účastník sa na rovnakej strane objavuje dvakrát.
error-participant-not-in-project = Tento účastník nie je súčasťou projektu.
error-too-many-participants = Príliš veľa účastníkov na jeden výdavok.
error-invalid-credentials = Nesprávny e-mail alebo heslo.
error-unauthenticated = Na túto akciu sa prihlás.
error-email-not-verified = Tvoja e-mailová adresa ešte nie je overená.
error-project-not-found = Tento projekt už neexistuje.
error-expense-not-found = Tento výdavok už neexistuje.
error-storage-full = Úložisko je plné: kľúč tohto projektu sa v tomto zariadení nepodarilo uložiť. Uschovaj si odkaz na zdieľanie.
error-user-not-found = Tento účastník už neexistuje.
error-tricount-not-found = Tricount sa nenašiel alebo jeho API vrátilo chybu.
error-too-many-members = Tento projekt dosiahol limit členov.
error-identity-taken = Tohto účastníka si už nárokoval iný účet.
error-claim-proof-invalid = Toto zariadenie nemá kľúč projektu, takže si nemôže nárokovať účastníka. Znova otvor odkaz na zdieľanie.
error-user-has-payments = Tento účastník má v projekte výdavky a nedá sa odstrániť.
error-resend-cooldown = Pred vyžiadaním ďalšieho e-mailu počkaj 60 sekúnd.
error-self-friend-request = Nemôžeš si pridať sám seba ako priateľa.
error-not-a-friend = Pozvať môžeš len ľudí zo svojho zoznamu priateľov.
error-friend-has-no-key = Tento priateľ ešte neotvoril najnovšiu verziu aplikácie. Požiadaj ho, aby sa raz prihlásil, a skús to znova.
error-friend-request-not-found = Táto žiadosť o priateľstvo už neexistuje.
error-invitation-not-found = Táto pozvánka už neexistuje.
error-too-many-friend-requests = Príliš veľa žiadostí o priateľstvo. Skús to zajtra.
error-too-many-invitations = Príliš veľa čakajúcich pozvánok.
error-invalid-kdf-salt = Nastavenia šifrovania nie sú platné. Aktualizuj aplikáciu a skús to znova.
error-mixed-project-batch = Títo účastníci nie sú všetci v rovnakom projekte.
error-invalid-payload = Táto verzia aplikácie odoslala údaje, ktoré server neprijíma. Aktualizuj ju a skús to znova.
error-invalid-public-key = Tvoj šifrovací kľúč nie je platný. Aktualizuj aplikáciu a skús to znova.
error-payment-methods-stale = Tvoje platobné údaje sa zmenili na inom zariadení. Načítaj znova a skús to.

### Auth

field-email = E-mail
field-email-placeholder = ty@priklad.sk
field-password = Heslo
field-name = Meno
field-name-placeholder = Jana Nováková

login-title = Prihlásiť sa
login-submit = Prihlásiť sa
login-submitting = Prihlasovanie…
login-password-placeholder = Tvoje heslo
login-no-account = Ešte nemáš účet?
login-unverified = Tvoja e-mailová adresa ešte nie je overená. Skontroluj schránku alebo si nechaj odkaz poslať znova.
login-maybe-unverified = Nový účet? Najprv over svoju adresu odkazom, ktorý sme ti poslali, alebo si ho pošli znova.
login-resend = Poslať overovací odkaz znova
login-resending = Odosiela sa…
login-resend-sent = E-mail odoslaný - skontroluj schránku.

register-submit = Vytvoriť účet
register-submitting = Vytvára sa…
register-have-account = Už máš účet?
register-password-placeholder = Aspoň 12 znakov
register-password-warning = Heslo si zapíš: nikdy ho nevidíme a nemôžeme ti ho vrátiť.
register-check-email-title = Skontroluj e-mail
register-email-sent = E-mail odoslaný
register-email-sent-hint = Na aktiváciu účtu klikni na odkaz v schránke.
register-not-received-prefix = Neprišiel? Skontroluj spam alebo sa
register-sign-in-link = prihlás
register-not-received-suffix = a nechaj si odkaz poslať znova.
register-terms-prefix = Vytvorením účtu prijímaš naše
register-terms-link = podmienky používania
register-terms-and = a naše
register-privacy-link = zásady ochrany súkromia

settings-title = Nastavenia
settings-preferences = Predvoľby
settings-preferences-local = Uložené na tomto zariadení.
settings-preferences-synced = Synchronizované s tvojím účtom, šifrované.
settings-about = O aplikácii
settings-anonymous-title = Nie si prihlásený
settings-upsell-title = Tvoje projekty na každom zariadení
settings-upsell-free = Zadarmo
settings-upsell-body = Counted funguje aj bez účtu. S bezplatným účtom ťa tvoje projekty a predvoľby nasledujú na telefón, notebook aj web - stále šifrované, stále pre nás nečitateľné.
settings-locked-badge = Účet
settings-locked-friends = Vytvor si účet, aby si mohol pridávať priateľov a pozývať ich do projektu priamo z aplikácie - bez posielania odkazu.
settings-locked-payment-methods = Ulož si raz svoj IBAN alebo platobnú aplikáciu a zdieľaj ich s vybranými projektmi. Kto ti dlhuje, uvidí ich vedľa tvojho mena.
settings-friends-hint = Pridávaj priateľov a pozývaj ich do projektov bez zdieľania odkazu.

account-member-since = Členom od
account-logout = Odhlásiť sa
account-logging-out = Odhlasovanie…
account-delete-title = Odstrániť môj účet
account-delete-warning = Okamžité a trvalé odstránenie tvojho účtu a tvojich údajov.
account-delete-confirm-title = Odstrániť účet
account-delete-confirm-message = Tvoj účet, relácie a zoznam projektov budú trvalo odstránené. Bez hesla sa pre teba šifrované dáta zdieľaného projektu stanú nečitateľnými - to sa nedá vrátiť späť.
erase-device-open = Odhlásiť sa a vymazať toto zariadenie
erase-device-title = Vymazať toto zariadenie?
erase-device-body = Projekty, ich kľúče a vyrovnávacia pamäť na tomto zariadení sa vymažú a potom sa relácia ukončí. Tvoje projekty zostávajú na tvojom účte: prihlás sa znova a získaš ich späť.
erase-device-checking = Overujem, či sú tvoje projekty na tvojom účte…
erase-device-pending = Zmeny čakajúce na odoslanie: { $count }. Pripoj sa znova na internet a skús to znova.
erase-device-unreachable = Nepodarilo sa overiť, či sú tvoje projekty na tvojom účte. Skontroluj pripojenie a skús to znova.
erase-device-at-risk = Tieto projekty nie sú na tvojom účte a stratia sa. Ak si ich chceš ponechať, zruš to a najprv skopíruj ich odkaz:
erase-device-confirm = Vymazať a odhlásiť
erase-device-confirm-anyway = Napriek tomu vymazať
erase-device-erasing = Mazanie…

settings-payment-methods = Platobné údaje
settings-payment-methods-hint = Ako chceš dostávať peniaze späť. Šifrované s tvojím účtom.
payment-method-kind = Spôsob
payment-method-kind-other = Iný
payment-method-label = Názov
payment-method-label-placeholder = Hlavný účet
payment-method-value = Údaje
payment-method-value-placeholder = IBAN, telefónne číslo, používateľské meno…
payment-method-add = Pridať
payment-method-remove = Odstrániť { $name }
payment-method-empty = Zatiaľ si nepridal žiadne platobné údaje.
payment-method-deleted = Platobná metóda odstránená.
payment-method-value-required = Vyplň údaje každej platobnej metódy alebo ju odstráň.
payment-method-label-required = Pomenuj svoju vlastnú metódu.
payment-method-too-long = To je príliš dlhé - skráť to.
payment-method-invalid-characters = Odstráň zalomenia riadkov alebo neviditeľné znaky.
payment-method-limit = Môžeš uložiť až { $max } platobných metód.
payment-methods-saved = Platobné údaje uložené.
payment-methods-offline = Na uloženie platobných údajov musíš byť online.
payment-methods-stale = Tvoje platobné údaje boli zmenené na inom zariadení. Boli znova načítané - skús to prosím znova.
payment-methods-key-missing = Na správu platobných údajov sa znova prihlás.
settings-payment-methods-share-warning = Zdieľaná metóda je viditeľná pre všetkých členov tvojich projektov.
payment-method-share = Zdieľať s mojimi projektmi
payment-method-share-hint = Zobrazí sa vedľa tvojho mena, keď ti niekto dlhuje.
payment-method-copy = Kopírovať { $name }
payment-method-copied = Skopírované.
payment-method-copy-failed = Kopírovanie zlyhalo - označ text a skopíruj ho ručne.

verify-email-checking = Overuje sa e-mailová adresa…
verify-email-welcome = E-mail overený - vitaj v Counted!
verify-email-back-to-login = Späť na prihlásenie
verify-email-intro = Zadaj svoje heslo a potvrď, že táto adresa je tvoja.
verify-email-submit = Overiť moju adresu

### Project status

project-archive = Archivovať
project-sheet-invite = Pozvať
project-sheet-recurring = Opakované
project-sheet-edit = Upraviť projekt
project-sheet-close = Uzavrieť projekt
project-sheet-archive = Archivovať projekt
project-sheet-reopen = Znova otvoriť projekt
project-sheet-unarchive = Obnoviť projekt z archívu
project-sheet-leave = Opustiť projekt

### Dates

date-long = { $day }. { $month } { $year }

month-1 = januára
month-2 = februára
month-3 = marca
month-4 = apríla
month-5 = mája
month-6 = júna
month-7 = júla
month-8 = augusta
month-9 = septembra
month-10 = októbra
month-11 = novembra
month-12 = decembra

month-short-1 = jan
month-short-2 = feb
month-short-3 = mar
month-short-4 = apr
month-short-5 = máj
month-short-6 = jún
month-short-7 = júl
month-short-8 = aug
month-short-9 = sep
month-short-10 = okt
month-short-11 = nov
month-short-12 = dec

### Actions

add = Pridať
create = Vytvoriť
creating = Vytvára sa…
edit = Upraviť
leave = Opustiť
close = Zavrieť
paste = Prilepiť
join = Pripojiť sa
import = Importovať
importing = Importuje sa…
field-description = Popis
field-date = Dátum
date-today = Dnes
date-yesterday = Včera
field-optional = Nepovinné

### Projects

projects-filter-active = Aktívne
projects-filter-all = Všetky
projects-count-label = Projekty
projects-empty = Žiadne projekty
projects-offline-banner = Offline dáta - na obnovenie sa znova pripoj.
demo-banner = Ukážkový projekt - iba na čítanie.
demo-start-own = Založiť vlastný projekt
demo-badge = Demo
demo-bar = Tvoje zmeny zmiznú po zatvorení tejto karty.
demo-leave = Začať naostro
demo-preparing = Pripravujeme tvoje demo…
demo-failed = Demo sa nepodarilo pripraviť.
demo-sign-in-blocked = Prihlásenie je v deme vypnuté. Opusti ho, aby si sa mohol prihlásiť alebo vytvoriť účet; tvoje vlastné projekty zostanú nedotknuté.
projects-no-local-data = Žiadne miestne dáta
projects-no-local-data-hint = Prihlás sa a načítaj svoje projekty prvýkrát.
projects-add = Pridať projekt
projects-create = Vytvoriť projekt
projects-personal-expense = Osobný výdavok
personal-expense-banner = Pridané do Mojich výdavkov
personal-create-title = Vytvor si osobný projekt
personal-create-hint = Sleduj, čo míňaš sám
group-create-title = Vytvor zdieľaný projekt
group-create-hint = Rozdeľ si výdavky s priateľmi a rodinou
personal-create-action = Vytvoriť
personal-create-signup = Vytvoriť účet
personal-sheet-title = Tvoj osobný projekt
personal-sheet-intro = Na zapisovanie toho, čo míňaš sám. Zobrazí sa s tvojimi skupinami v Štatistikách.
personal-sheet-point-alone = Jediný člen: ty, s menom tvojho účtu. Bez odkazu na zdieľanie.
personal-sheet-point-encrypted = Šifrovaný na tvojich zariadeniach, ako tvoje ostatné projekty.
personal-sheet-point-stats = Tvoje podiely v skupinách sa v Štatistikách pripočítajú k týmto výdavkom.
personal-default-name = Moje výdavky
my-spending-prev-month = Predchádzajúci mesiac
my-spending-next-month = Nasledujúci mesiac
my-spending-spent = Minul si
my-spending-groups = Skupiny
my-spending-income = Príjmy
my-spending-balance = Bilancia mesiaca
my-spending-unconverted = Podiely v inej mene zostávajú bokom, kým nie je známy kurz.
my-spending-empty = Tento mesiac nič
my-spending-empty-hint = Pridaj výdavok pomocou +. Tvoje podiely v skupinách sa tu zobrazia automaticky.
my-spending-share-of = z { $amount }
my-spending-uncounted = nezapočítané
payer-you = ty
expense-received-by = prijal(a)
counted-switch = Započítať do Moje výdavky
counted-switch-on = Zapnuté: tvoj podiel sa pripočíta k Moje výdavky
counted-switch-off = Vypnuté: tvoj podiel sa v Moje výdavky nezobrazuje
counted-groups = Započítané skupiny
counted-groups-intro = Tvoj podiel v týchto skupinách sa pripočíta k Moje výdavky a k rozpočtom.
counted-groups-converted = { $from }, prepočítané na { $to }
counted-groups-encrypted = Táto voľba je šifrovaná tvojím účtom a nasleduje ťa na tvojich zariadeniach.
counted-groups-empty = Na tomto zariadení zatiaľ nie je žiadna skupina.
counted-groups-done = OK
counted-save-failed = Túto voľbu sa nepodarilo uložiť. Skús to znova.
tab-budget = Rozpočet
budget-title = Rozpočty
budget-manage = Spravovať
budget-left = Zostáva { $amount }
budget-over = Prekročené o { $amount }
budget-empty = Žiadny rozpočet
budget-empty-hint = Nastav strop pre každú kategóriu. Tvoje podiely v skupinách sa počítajú tiež.
budget-define = Nastaviť rozpočet
budget-sheet-title = Mesačné rozpočty
budget-sheet-intro = Na tvoj podiel: osobné a skupiny. Nechaj prázdne, ak kategóriu nechceš sledovať.
budget-average = Priem. posledné 3 mesiace: { $amount }
budget-for = Rozpočet { $category }
budget-alert = Upozorniť ma pri 80 %
budget-alert-hint = Vypočítané na tomto zariadení
budget-invalid = Neplatná suma pre { $category }.
budget-offline = Rozpočty sa dajú uložiť len online. Pripoj sa a skús to znova.
budget-alert-near = { $category }: { $percent } % mesačného rozpočtu
budget-alert-over = { $category }: mesačný rozpočet prekročený
personal-card-budget = { $month } · rozpočet { $amount }
my-spending-search-summary =
    { $count ->
        [one] { $count } výsledok
        [few] { $count } výsledky
        [many] { $count } výsledku
       *[other] { $count } výsledkov
    } · { $total } · osobné a skupiny, všetky mesiace
projects-join = Pripojiť sa k projektu
projects-import-tricount = Importovať z Tricountu
project-actions = Akcie projektu

status-ongoing = Prebieha
status-closed = Uzavretý
status-archived = Archivovaný

nav-help = Pomocník
nav-privacy = Zásady ochrany súkromia
nav-terms = Podmienky používania
nav-legal = Právne informácie

leave-project-title = Opustiť projekt?
leave-project-message = Stratíš prístup z tohto zariadenia. Ak neostane žiadny člen, projekt a všetky jeho výdavky budú trvalo odstránené.

add-project-title = Nový projekt
add-project-name-label = Názov projektu
add-project-name-placeholder = Môj výlet, Spolubývanie 2024…
add-project-participants = Účastníci
add-project-participant-name = Meno účastníka
add-project-participant-placeholder = Clark Kent
add-project-offline = Offline nemožno vytvoriť projekt. Znova sa pripoj a skús to znova.
add-project-name-required = Projekt potrebuje názov.

join-link-label = Odkaz na zdieľanie
join-link-hint = Odkaz obsahuje dešifrovací kľúč - skopíruj ho celý.
join-invalid-link = Tento odkaz nie je platný. Prilep celý odkaz na zdieľanie vrátane časti za #.
join-wrong-project = Tento odkaz patrí inému projektu.

import-tricount-link-label = Odkaz alebo kľúč Tricount
import-tricount-key-required = Zadaj odkaz alebo kľúč Tricount.
import-tricount-encryption-failed = Šifrovanie zlyhalo.
import-tricount-unimportable = Nič sa neimportovalo: tento Tricount má členov s účtom Tricount alebo sumy, ktoré nesedia (dotknuté položky: { $count }).

### Expenses

save = Uložiť
saving = Ukladá sa…
adding = Pridáva sa…
link-copied = Odkaz skopírovaný
missing-encryption-key = Chýba šifrovací kľúč.
missing-encryption-key-title = Chýba šifrovací kľúč
missing-encryption-key-hint = Použitý odkaz neobsahuje kľúč potrebný na dešifrovanie tohto projektu. Použi úplný odkaz od toho, kto projekt vytvoril.
project-locked-hint = Toto zariadenie nemá kľúč k tomuto projektu. Na odomknutie otvor jeho odkaz na zdieľanie.
expenses-unreadable = Niektoré položky sa na tomto zariadení nedajú prečítať a nie sú započítané do súčtov. Na ich zobrazenie aktualizuj aplikáciu.
project-unlock = Odomknúť
project-no-local-data-hint = Prihlás sa a načítaj dáta tohto projektu prvýkrát.
project-gone-title = Tento projekt už neexistuje
project-gone-hint = Bol odstránený, keď ho opustil posledný člen. Odkaz na zdieľanie už nefunguje, ani keď ho znova otvoríš.

expense-add = Pridať výdavok
transfer-add = Pridať prevod
expense-edit-title = Upraviť výdavok
expense-category = Kategória
expense-category-auto = Auto · { $emoji }
emoji-picker-label = Emoji
emoji-none = Bez emoji
expense-currency = Mena sumy
amount-op-add = Plus
amount-op-subtract = Mínus
amount-op-multiply = Krát
amount-op-divide = Delené
amount-op-equals = Rovná sa
amount-op-done = Hotovo
expense-rate = Výmenný kurz (nepovinné)
expense-rate-hint = Nechaj prázdne na použitie kurzu Európskej komisie (InforEuro) za { $month }: 1 { $from } = { $rate } { $to }.
expense-rate-invalid = Zadaj výmenný kurz väčší ako 0.
expense-rate-unavailable = Automatický kurz nie je k dispozícii - zadaj ho ručne.
expense-delete-title = Odstrániť výdavok
expense-delete-message = „{ $name }“ bude trvalo odstránený. To sa nedá vrátiť späť.
expense-inconsistent-amounts = Sumy nesedia
expenses-empty = Žiadne výdavky
expenses-empty-hint = Začni pridaním výdavkov tlačidlom nižšie
expenses-show-more = Zobraziť viac (zostáva { $count })

expense-type-expense = Výdavok
expense-type-transfer = Prevod
expense-type-gain = Príjem
expense-type-group = Typ
expense-paid-by = zaplatil(a)
expense-sent-by = poslal(a)
expense-contributed-by = prispel(a)

expense-name-required = Názov je povinný.
expense-amount-not-positive = Suma musí byť väčšia ako 0.
expense-no-payer = Vyber aspoň jedného platcu.
expense-no-debtor = Vyber aspoň jednu osobu, ktorá dlhuje.
expense-invalid-date = Tento dátum nie je platný.
expense-payers-mismatch = Súčet platcov je { $sum }, čo nezodpovedá sume výdavku ({ $total }).
expense-debtors-mismatch = Súčet dlžníkov je { $sum }, čo nezodpovedá sume výdavku ({ $total }).

participants-none = Nikto
participants-everyone = Všetci ({ $count })
participants-some = { $count } z { $total }
participants-select-all = Vybrať všetko
participants-by-shares = Podľa podielov
split-amounts = Sumy
participants-remaining = Zostáva { $amount }
participants-over-by = O { $amount } navyše
participants-who-paid = Kto platil?
participants-who-received = Kto prijal?
participants-who-transfers = Kto prevádza?
participants-who-receives = Kto prijíma?
participants-for-whom = Pre koho?

stats-total-expenses = Celkové výdavky
stats-my-expenses = Moje výdavky

tab-expenses = Výdavky
tab-balance = Bilancia
tab-reimbursements = Vyrovnanie
balance-gets-back = Dostane späť
balance-owes = Dlhuje
balance-settled = Vyrovnané
reimbursements-empty-title = Všetko vyrovnané!
reimbursements-empty-hint = Návrhy na vyrovnanie sa tu objavia, keď účty nesedia
reimbursement-owes = { $debtor } dlhuje { $creditor }
reimbursement-record = Vyrovnať
settlement-recorded = Vyrovnanie zaznamenané.
reimbursement-pay-with = Zaplatiť
reimbursement-pay-shared-by = Zdieľa { $name } - pred odoslaním skontroluj meno príjemcu, ktoré zobrazuje tvoja aplikácia.
reimbursement-pay-title = Zaplatiť { $name }
reimbursements-mine-title = Dlhuješ
reimbursements-others-title = Ostatné vyrovnania
copy = Kopírovať

user-selection-title = Ktorý účastník si ty?
user-selection-hint = Vyber svoje meno zo zoznamu.
user-selection-required = Vyber prosím účastníka.
identity-claimed = Prepojené s účtom
identity-claimed-by = Účet { $name }
identity-taken-repick = Účastníka, ktorého si používal, si nárokoval iný účet. Vyber si prosím iného.
participant-gone-repick = Účastník, ktorého si používal, bol z projektu odstránený. Vyber si prosím iného.

edit-project-title = Upraviť projekt
edit-project-new-badge = nový
edit-project-deferred-new-members = pridanie nových členov
edit-project-deferred-removals = odstránenie členov
edit-project-offline-deferred = Offline: { $items } sa použije po opätovnom pripojení.

export-failed = Export zlyhal: { $reason }

history-expense-added = Výdavok pridaný: { $name }
history-expense-edited = Výdavok upravený: { $name }
history-expense-deleted = Výdavok odstránený: { $name }
move-expense = Presunúť do projektu
move-expense-title = Presunúť do…
move-current = Aktuálny
move-converted = { $currency }, prepočítané kurzom mesiaca
move-note-personal = V ďalšom kroku zvolíš, kto platil a za koho. Výdavok opustí Moje výdavky.
move-note-group = Výdavok opustí { $project } pre všetkých členov a bilancia skupiny sa prepočíta. V ďalšom kroku zvolíš, kto platil a za koho.
move-banner = Z { $from } do { $to }
move-submit = Presunúť
move-done = Výdavok presunutý do { $project }
history-expense-moved = Výdavok presunutý do iného projektu: { $name }
personal-badge = Osobné
move-members =
    { $count ->
        [one] { $count } člen
        [few] { $count } členovia
        [many] { $count } člena
       *[other] { $count } členov
    }
history-project-edited = Projekt upravený: { $name }
history-name-changed = Názov: „{ $from }“ → „{ $to }“
history-description-added = Popis pridaný: „{ $value }“
history-description-removed = Popis odstránený: „{ $value }“
history-description-changed = Popis: „{ $from }“ → „{ $to }“

### Sweep

field-amount = Suma
expense-name-placeholder = Reštaurácia, nákup…
expense-actions = Akcie výdavku
expense-your-share = Tvoj podiel
expense-your-share-value = Tvoj podiel: { $amount } { $currency }
expense-share-percent = { $percent } %
expense-added-by = Pridané: { $name }, { $date }
expense-inconsistent-detail = Sumy nesedia: zaplatené { $paid }, dlžné { $owed }, pri výdavku { $total }. Uprav výdavok a oprav to.
missing-access-key = Chýba prístupový kľúč. Otvor tento projekt cez jeho odkaz na zdieľanie.
filter-all = Všetko
filter-my-payments = Moje platby
filter-my-debts = Čo dlhujem
expense-filter-group = Filter
expenses-search = Hľadať výdavky
expenses-no-match = Žiadny výdavok nezodpovedá.
expenses-search-open = Hľadať
expenses-search-clear = Vymazať hľadanie
expenses-search-summary =
    { $count ->
        [one] { $count } výsledok
        [few] { $count } výsledky
        [many] { $count } výsledku
       *[other] { $count } výsledkov
    } · { $total } · všetky mesiace
participants-shares-for = Podiely pre { $name }
participants-amount-for = Suma pre { $name }
reimbursement-add = Pridať vyrovnanie
project-forget = Odstrániť z môjho zoznamu
project-history-title = História
history-kind-add = Pridané
history-kind-delete = Odstránené
history-kind-edit = Upravené
export = Exportovať
export-json = Exportovať JSON
export-csv = Exportovať CSV
share-link = Zdieľať
copy-link-failed = Odkaz sa nepodarilo skopírovať
open-in-app = Otvoriť v aplikácii
not-found-title = Stránka sa nenašla
not-found-back = Späť na projekty

### Charts

charts-period = Obdobie
period-all = Všetko
period-month = Mesiac
period-3months = 3 mes.
period-year = Rok
period-custom = Vlastné
charts-tab-categories = Kategórie
charts-tab-trends = Trendy
charts-total-spent = Celkom minuté
charts-avg-per-person = Priem. na osobu
charts-expense-count =
    { $count ->
        [one] { $count } výdavok
        [few] { $count } výdavky
        [many] { $count } výdavku
       *[other] { $count } výdavkov
    }
charts-nothing-to-show = Nie je čo zobraziť
charts-my-share-skipped =
    { $count ->
        [one] 1 projekt sa nepočíta - nebol vybraný účastník alebo sa nepodarilo načítať jeho dáta.
        [few] { $count } projekty sa nepočítajú - nebol vybraný účastník alebo sa nepodarilo načítať ich dáta.
        [many] { $count } projektu sa nepočíta - nebol vybraný účastník alebo sa nepodarilo načítať ich dáta.
       *[other] { $count } projektov sa nepočíta - nebol vybraný účastník alebo sa nepodarilo načítať ich dáta.
    }

### Categories

category-food = Jedlo
category-transport = Doprava
category-accommodation = Ubytovanie
category-leisure = Voľný čas
category-shopping = Nákupy
category-services = Služby
category-parties-gifts = Oslavy a darčeky
category-other = Ostatné
categories-manage = Spravovať kategórie
categories-title = Kategórie
categories-yours = Tvoje kategórie
categories-builtin = Predvolené
categories-add = Pridať kategóriu
categories-in-parent = v kategórii { $parent }
categories-renamed-from = premenovaná · pôvodne { $name }
categories-hidden-hint = skrytá · jej výdavky idú do kategórie { $other }
categories-always-shown = vždy zobrazená
categories-hidden = Skrytá
categories-new = Nová kategória
categories-edit = Upraviť kategóriu
categories-parent = Počíta sa do kategórie
categories-parent-hint = V štatistikách všetkých tvojich projektov sa { $name } počíta do kategórie { $parent }.
categories-create = Vytvoriť
categories-hide = Skryť túto kategóriu
categories-hide-hint = Jej výdavky sa všetkým členom zobrazia v kategórii { $other }.
categories-delete-title = Zmazať „{ $name }“?
categories-delete-body = Dotknuté výdavky ({ $count }) sa vrátia do kategórie { $parent }. Žiadny výdavok sa nezmaže.
categories-delete-body-unknown = Jej výdavky sa vrátia do kategórie { $parent }. Žiadny výdavok sa nezmaže.
categories-delete-warning = Kategória zmizne všetkým členom projektu.
categories-summary-created = Vytvorené: { $count }
categories-summary-renamed = Premenované: { $count }
categories-summary-hidden = Skryté: { $count }
categories-summary-default = Žiadne vlastné kategórie
categories-offline = Kategórie sa dajú spravovať len online. Pripoj sa a skús to znova.
categories-error-too-many = Projekt má najviac 20 vlastných kategórií.
categories-error-full = Tento projekt dosiahol limit kategórií.
categories-error-name = Pomenuj kategóriu.
categories-error-missing = Táto kategória sa medzitým zmenila. Zavri a skús to znova.
history-category-created = Kategória vytvorená: { $name }
history-category-edited = Kategória upravená: { $name }
history-category-deleted = Kategória zmazaná: { $name }
charts-project = Projekt
charts-all-projects = Všetky projekty
charts-date-from = Od
charts-date-to = Do
charts-total = Celkom
charts-tab-people = Ľudia
charts-tab-projects = Projekty
charts-scope = Čie výdavky
charts-scope-group = Skupina
charts-scope-me = Ja
charts-currency = Mena
charts-my-share = Môj podiel
charts-share-of-total = { $pct } % z { $total }
charts-i-paid = Zaplatil som
charts-paid-more = O { $amount } viac než môj podiel
charts-paid-less = O { $amount } menej než môj podiel
charts-paid-even = Presne môj podiel
charts-part-title = Tvoj podiel v každej kategórii
charts-part-desc = Sivé je, čo minula skupina, farebné, čo si spotreboval ty.
charts-breakdown-title = Rozdelenie podľa kategórií
charts-breakdown-desc = Ťukni na výsek alebo riadok a uvidíš výdavky.
charts-of-total = { $amount } z { $total }
charts-show-all = Zobraziť všetko ({ $count })
charts-show-less = Zobraziť menej
charts-spend-title = Výdavky v čase
charts-group-by = Zoskupiť podľa
bucket-day = Deň
bucket-week = Týždeň
bucket-month = Mesiac
charts-avg = priem.
charts-cat-title-day = { $category }, deň po dni
charts-cat-title-week = { $category }, týždeň po týždni
charts-cat-title-month = { $category }, mesiac po mesiaci
charts-cat-desc = Vyber kategóriu a sleduj ju v čase.
charts-running-title = Priebežný súčet
charts-running-desc = Od { $date }.
charts-avg-per-day = { $amount } / deň v priemere
charts-avg-per-week = { $amount } / týždeň v priemere
charts-avg-per-month = { $amount } / mesiac v priemere
charts-people-title = Kto ťahal skupinu
charts-people-desc = Čo kto zaplatil, vedľa toho, čo spotreboval.
charts-paid = Zaplatené
charts-fair-share = Férový podiel
charts-you = (ty)
charts-net-more = zaplatil viac
charts-net-less = zaplatil menej
charts-balance-title = Tvoj zostatok v čase
charts-balance-desc = Nad čiarou ti skupina dlží. Pod ňou dlžíš ty skupine.
charts-owed = Dlžia ti
charts-owe = Dlžíš
charts-projects-title = Tvoj podiel podľa projektu
charts-projects-desc = Súčty sa vedú podľa mien.
history-empty = Žiadne udalosti
history-by = { $name }
not-found-hint = Táto stránka neexistuje alebo sa presunula.
payers-title-paid-by = Zaplatil(a)
payers-title-sender = Odosielateľ
payers-title-contributors = Prispievatelia
debtors-title-debtors = Dlhuje
debtors-title-recipients = Príjemcovia
debtors-title-beneficiaries = Príjemcovia
debtors-title-for = Pre
debtors-title-recipient = Príjemca
expense-split-between = rozdelené medzi { $count }
expense-people =
    { $count ->
        [one] { $count } osoba
        [few] { $count } osoby
        [many] { $count } osoby
       *[other] { $count } osôb
    }

### Welcome

welcome-title = Do tvojich účtov nikoho nič nie je.
welcome-subtitle = Deľ sa o výdavky s priateľmi.
welcome-note = Zadarmo. Bez účtu. Bez reklám.
welcome-link-title = Jeden odkaz a všetci sa zapoja.
welcome-link-body = Nikto si nemusí zakladať účet.
welcome-link-account = Účet? Nikdy nie je povinný. Slúži na nájdenie tvojich projektov na inom zariadení, pozývanie priateľov z aplikácie a zdieľanie tvojich platobných údajov.
welcome-demo-project = Víkend v Lyone
welcome-private-title = Nikto nemôže čítať tvoje vyúčtovania. Ani my.
welcome-private-body = Mená, sumy, projekty: všetko sa šifruje na tvojom zariadení. Kľúč máš len ty.
welcome-private-names = Mená
welcome-private-amounts = Sumy
welcome-private-projects = Projekty
welcome-scan-title = Odfoť účtenku.
welcome-scan-body = Suma, dátum a kategória sa vyplnia samy. Všetko prebieha v tvojom telefóne. Fotka sa neukladá.
welcome-eu-title = 100 % európske
welcome-no-ads = Žiadne reklamy
welcome-no-trackers = Žiadne sledovanie
welcome-step = Krok { $current } z { $total }
welcome-next = Ďalej
welcome-skip = Preskočiť
welcome-start = Začať
welcome-how-it-works = Ako to presne funguje?

### Help

help-intro = Častá otázka? Klepnutím rozbalíš odpoveď.
help-create-project-q = Ako vytvorím projekt?
help-create-project-a = Na domovskej obrazovke klepni na tlačidlo + dole. Pomenuj projekt, vyber menu a je hotovo.
help-add-participants-q = Ako pridám účastníkov?
help-add-participants-a = Otvor projekt a pridaj účastníkov zo zoznamu členov. Každý účastník môže za výdavok platiť alebo ho dlhovať.
help-share-project-q = Ako zdieľam projekt?
help-share-project-a = Zdieľaj URL projektu (tú v adresnom riadku). Ktokoľvek s odkazom môže projekt prezerať a upravovať.
help-add-expense-q = Ako pridám výdavok?
help-add-expense-a = V projekte klepni na +, zadaj sumu, urči, kto platil a medzi koho výdavok rozdeliť. Môžeš tiež zvoliť iný dátum ako dnešný.
help-types-q = Aký je rozdiel medzi výdavkom, prevodom a príjmom?
help-types-expense = - nákup jednej osoby rozdelený medzi viacero.
help-types-transfer = - vrátenie peňazí od jednej osoby druhej, bez rozdelenia.
help-types-gain = - prijaté peniaze (vrátenie, dar) na rozdelenie medzi viacero osôb.
help-past-date-q = Môžem výdavok datovať do minulosti?
help-past-date-a = Áno, pole dátumu je ľubovoľné. Čas vytvorenia záznamu sa ukladá zvlášť.
help-who-owes-q = Ako Counted zistí, kto čo dlhuje?
help-who-owes-a = Counted vypočíta čistú bilanciu každého účastníka (čo zaplatil mínus čo dlhuje) a potom navrhne najkratšiu sériu prevodov, ktorá všetkých vyrovná.
help-minimal-transfers-q = Prečo je počet navrhnutých prevodov minimálny?
help-minimal-transfers-a = Algoritmus najprv spáruje bilancie, ktoré sa presne vyrušia, a potom prejde zvyšok od najväčšieho veriteľa k najväčšiemu dlžníkovi. Výsledok: menej prevodov na úplné vyrovnanie.
help-import-tricount-q = Ako importujem projekt z Tricountu?
help-import-tricount-a = Na domovskej obrazovke klepni na tlačidlo „+“ dole a potom
help-import-tricount-b = Prilep odkaz na zdieľanie Tricountu, ktorý chceš importovať.
help-encryption-q = Sú moje dáta šifrované?
help-encryption-a = Áno. Counted kombinuje dve záruky:
help-encryption-e2ee-term = End-to-end šifrovanie
help-encryption-e2ee-def = - všetko medzi tebou a serverom cestuje šifrovane.
help-encryption-zero-term = Nulový prístup
help-encryption-zero-def = - dáta šifruješ pred odoslaním a server ukladá len šifrovaný text. Nemáme ako ho prečítať.
help-encryption-see = Podrobnosti nájdeš v
help-forgot-password-q = Čo sa stane, keď zabudnem heslo?
help-forgot-password-warning = Nikto ho za teba nemôže obnoviť.
help-forgot-password-a = Tvoje heslo chráni tvoje šifrovacie kľúče a nikdy sa k nám neposiela: nikto - ani my - tvoje projekty za teba nedešifruje. Uchovaj ho v bezpečí, ideálne v správcovi hesiel.
help-archive-delete-q = Ako projekt archivujem alebo odstránim?
help-archive-delete-a = Na obrazovke projektu otvor menu a zvoľ
help-archive-delete-b = a projekt sa skryje, ale ostane zachovaný. Projekt je trvalo odstránený, keď ho opustí posledný člen.
help-delete-account-q = Ako odstránim svoj účet?
help-delete-account-a = Otvor Nastavenia a použi „Odstrániť môj účet“. Je to okamžité a nedá sa to vrátiť späť.
help-contact = Ďalšia otázka? Napíš nám na

# Receipt scanning (mobile only)
expense-scan = Naskenovať účtenku
scan-in-progress = Číta sa účtenka…
scan-error-capture = Fotku sa nepodarilo urobiť. Skús to znova alebo zadaj výdavok ručne.
scan-error-unreadable = Na účtenke nie je nič čitateľné. Zadaj výdavok ručne.
scan-check-amount = Skontroluj súčet - nebol zreteľne vytlačený.
scan-take-photo = Odfotiť
scan-choose-photo = Vybrať fotku
expense-converted-from = Zaplatené { $amount } { $from } · 1 { $from } = { $rate } { $to }
project-currency = Mena
project-currency-locked = Mena sa nastavuje pri vytvorení projektu.
currency-search = Hľadať menu

update-required-title = Vyžaduje sa aktualizácia
update-required-body = Táto verzia Counted je príliš stará na komunikáciu so serverom. Aktualizuj ju, aby si mohol aplikáciu ďalej používať.
update-required-button = Aktualizovať

notifications-label = Oznámenia
notifications-title = Oznámenia
notifications-empty = Nič nové
notifications-friend-request = Žiadosť o priateľstvo

friends-title = Priatelia
friends-anonymous-body = Priatelia sú viazaní na tvoj účet. Prihlás sa, aby si mohol pridávať ľudí a pozývať ich do projektov bez zdieľania odkazu.
friends-add-title = Pridať priateľa
friends-add-hint = Ak má táto osoba účet, uvidí tvoju žiadosť v aplikácii. Ak nie, daj jej vedieť: nájde ju, keď sa zaregistruje s touto adresou.
friends-add-button = Pridať
friends-add-from-project = Pridať ako priateľa
friends-request-sent = Žiadosť odoslaná
friends-no-account-key = Na správu priateľov na tomto zariadení sa znova prihlás.
friends-incoming-title = Žiadosti
friends-accept = Prijať
friends-decline = Odmietnuť
friends-list-title = Moji priatelia
friends-list-empty = Zatiaľ žiadni priatelia. Pridaj niekoho e-mailom vyššie alebo zo zdieľaného projektu.
friends-remove = Odstrániť
friends-remove-confirm-title = Odstrániť priateľa
friends-remove-confirm-message = { $email } už nebude medzi tvojimi priateľmi a ty medzi jeho. Ktokoľvek z vás môže neskôr poslať novú žiadosť.
friends-no-key = Zatiaľ nie je pripravené
friends-key-changed = Kód sa zmenil
friends-key-changed-hint = Bezpečnostný kód priateľa sa zmenil. Stáva sa to, keď si zabezpečí účet, alebo keď bol jeho kľúč podvrhnutý. Pozvánky s ním sú zablokované, kým si nový kód neprečítate osobne alebo po telefóne a neťukneš na „Kód sedí“.
friends-fingerprint = Bezpečnostný kód
friends-fingerprint-hint = Dvaja priatelia, ktorí si prečítajú rovnaký bezpečnostný kód, vedia, že medzi nimi nikto nesedí - ani náš server.
friends-key-mismatch = Server poslal pre tvoj účet verejný kľúč, ktorý nezodpovedá tvojmu súkromnému kľúču. Tvoj bezpečnostný kód je skrytý a pozvánky sú na tomto zariadení vypnuté: niekto možno manipuluje so serverom.
keypair-mismatch = Kľúče, ktoré server poslal pre tvoj účet, nezodpovedajú tým na tomto zariadení. Pozvánky a zmeny kľúča sú tu vypnuté, kým nebudú sedieť: niekto možno manipuluje so serverom.
friends-outgoing-title = Odoslané
friends-outgoing-hint = Čaká na odpoveď. Po prijatí sa objavia medzi tvojimi priateľmi.
friends-withdraw = Zrušiť
friends-notify = Dať vedieť správou
friends-notify-label = Dať vedieť: { $email }
friends-notify-message = Prechádzam na Counted kvôli našim spoločným výdavkom. Registrácia zadarmo na https://counted.fr s adresou { $email }: čaká tam moja žiadosť o priateľstvo.
friends-notify-copied = Správa skopírovaná
friends-notify-copy-failed = Správu sa nepodarilo skopírovať
guests-hint-title =
    { $count ->
        [one] { $count } účastník sa ešte nepripojil k projektu
        [few] { $count } účastníci sa ešte nepripojili k projektu
        [many] { $count } účastníka sa ešte nepripojilo k projektu
       *[other] { $count } účastníkov sa ešte nepripojilo k projektu
    }
guests-hint-later = Neskôr
guests-hint-send = Pozvi ich
guests-send-title = Poslať projekt účastníkom
guests-send-friend = Pozvať priateľa z Countedu
guests-send-friend-badge = Bez odkazu
guests-send-friend-hint = Ak už má účet, dostane šifrovanú pozvánku; nič neprechádza cez tvoju aplikáciu na správy.
guests-send-link = Zdieľať odkaz na projekt
guests-send-link-hint = Pre tých, čo nemajú účet. Projekt otvoria jedným ťuknutím.
guests-send-link-warning = Ktokoľvek s týmto odkazom môže projekt čítať a upravovať. Tvoja aplikácia na správy ho vidí tiež.
invite-friends-title = Pozvať priateľov
invite-friends-hint = Kľúč projektu sa na tomto zariadení šifruje pre každého priateľa. Server ho nikdy nevidí.
invite-friends-empty = Zatiaľ žiadni priatelia na pozvanie.
invite-friends-button = Pozvať
invite-sent = { $count ->
    [one] Pozvánka odoslaná
    [few] Odoslané { $count } pozvánky
    [many] Odoslaných { $count } pozvánky
   *[other] Odoslaných { $count } pozvánok
}
invitation-badge = Pozvánka
invitation-to = Pripojiť sa k „{ $name }“
invitation-to-unnamed = Pripojiť sa k projektu
invitation-unreadable = Túto pozvánku nemožno na tomto zariadení otvoriť
invitation-from = Od { $email }
invitation-accept = Pripojiť sa
invitation-decline = Odmietnuť

# Participants in the create and edit modals, and the "who are you?" picker - see
# docs/plans/friends.md §11.
participants-you-label = Tvoje meno v tomto projekte
participants-you-badge = Ty
participants-others = Ďalší účastníci
participants-removed = Odstránení
participant-restore = Obnoviť
participants-empty = Pridaj účastníkov nižšie.
participants-duplicate = „{ $name }“ už je v zozname.
participants-input-label = Pridať priateľa alebo napísať meno
participants-input-placeholder = Priateľ alebo akékoľvek meno
participants-suggest-friend = Priateľ · pripojí sa ako „{ $name }“, dostane pozvánku
participants-suggest-not-ready = Priateľ · zatiaľ nie je pripravený
participants-suggest-guest = Pridať „{ $text }“
participants-suggest-guest-sub = Hosť
participants-friends = Tvoji priatelia
participants-all-friends = Všetci priatelia
participants-login-hint = Prihlás sa a pridávaj ľudí priamo zo zoznamu priateľov.
participants-invite-badge = Pozvať
participants-guest-badge = Hosť
participants-rename = Premenovať: { $name }
participants-remove = Odobrať: { $name }
participants-rename-label = Nové meno
participants-rename-save = Uložiť meno
participants-rename-hint = Meno, ktoré v tomto projekte vidia všetci. Pozvánka stále pôjde na { $email }.
participants-invited-badge = Pozvaný
participants-invited-sub = { $email } · zatiaľ neprijaté
participants-invited-pending = Pozvánka zatiaľ neprijatá
participants-unlinked = Neprepojené s účtom
add-project-create-invite = Vytvoriť a pozvať: { $count }
edit-project-save-invite = Uložiť a pozvať: { $count }
edit-project-you-are = Na tomto zariadení si { $name }
edit-project-no-identity = Ešte si nevybral(a), kto si
edit-project-switch = Zmeniť
edit-project-choose = Vybrať
invite-failed = Tieto pozvánky sa nepodarilo odoslať: { $emails }
invite-waiting = Títo priatelia musia raz otvoriť Counted, kým ich bude možné pozvať: { $emails }
key-offer-title = Iný kľúč pre { $name }
key-offer-body = Toto zariadenie má k tomuto projektu iný kľúč, ktorý ho tu nikdy neotvoril.
key-offer-choices = Použiť tento kľúč: projekt sa otvorí kľúčom z tohto odkazu alebo pozvánky. Zrušiť: zariadenie si ponechá svoj súčasný kľúč a nič sa nezmení.
key-offer-use = Použiť tento kľúč
invite-again = Pozvať znova
friend-picker-title = Pridať priateľov
user-selection-invited-hint = { $email } ťa pozval(a) do „{ $project }“.
user-selection-suggested = Navrhnuté
user-selection-suggested-sub = { $email } ťa pridal(a) pod týmto menom
user-selection-confirm-as = Som { $name }
user-selection-missing = Nie je tu tvoje meno? Požiadaj účastníka, nech ťa pridá v nastaveniach projektu.

## Opakované výdavky

repeat-label = Opakovať
repeat-none = Neopakuje sa
repeat-weekly = Každý týždeň
repeat-biweekly = Každé 2 týždne
repeat-monthly = Každý mesiac
repeat-quarterly = Každé 3 mesiace
repeat-yearly = Každý rok
repeat-every-weeks = Každých { $count } týž.
repeat-every-months = Každých { $count } mes.
repeat-every-years = Každých { $count } rokov
repeat-custom = Vlastné…
repeat-every = Každých
repeat-unit-weeks = týž.
repeat-unit-months = mes.
repeat-unit-years = rokov
repeat-on-weekday = v deň: { $weekday }
repeat-on-day = { $day }. deň v mesiaci
repeat-on-day-month = { $day }. { $month }
repeat-month-end = V kratších mesiacoch pripadne na posledný deň.
repeat-ends = Koniec
repeat-ends-never = Nikdy
repeat-ends-on = V dátum
repeat-ends-after = Po
repeat-fewer = Menej
repeat-more = Viac
repeat-last-on = posledný { $date }
repeat-variable = Suma sa zakaždým mení
repeat-variable-hint = Každý sa pridá s poslednou sumou a označí sa „na potvrdenie“.
repeat-done = Hotovo
repeat-no-end = Bez konca
repeat-until = Do { $date }
repeat-occurrences = Počet opakovaní: { $count }
repeat-offline = Vyžaduje pripojenie. Samotný výdavok sa dá pridať aj tak.
repeat-foreign = Opakuje sa ako { $amount } { $currency }, prepočítané raz dnešným kurzom. Upozorníme ťa, ak sa kurz zmení o viac ako 5 %.
repeat-backfill = Začína v minulosti. Výdavky pridané teraz: { $count }.
add-and-repeat = Pridať a opakovať
weekday-1 = pondelok
weekday-2 = utorok
weekday-3 = streda
weekday-4 = štvrtok
weekday-5 = piatok
weekday-6 = sobota
weekday-7 = nedeľa
recurring-title = Opakované výdavky
recurring-strip = Opakované výdavky: { $count }
recurring-next = Ďalší: { $name }, { $date }
recurring-to-confirm = Na potvrdenie: { $count }
recurring-per-month = Mesačne, cca
recurring-your-share = Tvoj podiel
recurring-active = Aktívne
recurring-paused = Pozastavené
recurring-finished = Ukončené
recurring-paid-by = platí { $name }
recurring-next-on = Ďalší { $date }
recurring-progress = { $done } z { $total }
recurring-rate-badge = Kurz sa zmenil o { $percent } %
recurring-empty = Zatiaľ sa nič neopakuje. Pri pridávaní výdavku zvoľ „Opakovať“: nájom, predplatné, účty.
recurring-next-ones = Ďalšie termíny
recurring-added-so-far = Zatiaľ pridané
recurring-set-up-by = Vytvoril(a)
recurring-pause = Pozastaviť
recurring-resume = Obnoviť
recurring-stop = Ukončiť opakovanie
recurring-stop-title = Ukončiť „{ $name }“?
recurring-stop-message = Už sa nebude opakovať. Už pridané výdavky zostanú.
recurring-resume-title = Obnoviť „{ $name }“?
recurring-resume-message = Ďalší { $date }. Termíny zmeškané počas pozastavenia sa nepridajú.
recurring-edit-title = Upraviť opakovaný výdavok
recurring-edit-banner = Zmeny platia od { $date }. Už pridané výdavky zostanú bez zmeny.
recurring-next-on-label = Ďalší
recurring-next-too-early = Ďalší dátum musí byť po poslednom už pridanom výdavku.
recurring-use-stop = Na ukončenie použi „Ukončiť opakovanie“ pri opakovanom výdavku.
recurring-drift = Kurz { $currency } sa od vytvorenia zmenil o { $percent } %. Každý sa stále pridáva ako { $amount } { $project_currency } (1 { $currency } = { $rate }). Dnešným kurzom by to bolo { $today_amount } { $project_currency }.
recurring-use-rate = Použiť dnešný kurz
recurring-keep = Ponechať { $amount } { $currency }
recurring-added = Pridané opakované výdavky: { $count }
recurring-blocks-removal = { $name } zatiaľ nemožno odstrániť: je súčasťou { $rules }. Odstráň { $name } z nich alebo ich ukonči a potom znova ulož.
history-recurring-added = Pridané automaticky: { $name } ({ $date })
history-recurring-created = Opakovaný výdavok vytvorený: { $name }
history-recurring-edited = Opakovaný výdavok upravený: { $name }
history-recurring-paused = Opakovaný výdavok pozastavený: { $name }
history-recurring-resumed = Opakovaný výdavok obnovený: { $name }
history-recurring-stopped = Opakovaný výdavok ukončený: { $name }
occurrence-recurring = Opakovaný výdavok
occurrence-auto = Pridané automaticky z opakovaného výdavku.
occurrence-auto-next = Pridané automaticky z opakovaného výdavku. Ďalší { $date }.
occurrence-auto-stopped = Pridané automaticky z opakovaného výdavku, ktorý bol medzitým ukončený.
occurrence-manage = Spravovať
estimate-badge = Na potvrdenie
expense-pending = Čaká
estimate-title = Suma na potvrdenie.
estimate-body = Pridané s predchádzajúcou sumou. Zadaj skutočnú, keď ju budeš vedieť.
estimate-confirm = Potvrdiť sumu
apply-to = Použiť na
apply-this-only = Len tento výdavok
apply-and-next = Tento a ďalšie
apply-and-next-hint = Opakovaný výdavok sa mení od { $date }
apply-rule-failed = Výdavok bol uložený, ale opakovaný výdavok sa nezmenil.
occurrence-delete-message = „{ $name }“ z { $date } bude natrvalo odstránený. Opakovanie pokračuje a tento dátum sa nevráti.
occurrence-delete-one = Odstrániť len tento
occurrence-delete-stop = Odstrániť a ukončiť opakovanie
error-recurring-clock = Hodiny tohto zariadenia idú napred. Skontroluj dátum a čas.
error-recurring-not-found = Tento opakovaný výdavok už neexistuje.
error-recurring-stale = Niekto medzitým zmenil tento opakovaný výdavok. Bol znova načítaný: skontroluj ho a ulož znova.
error-too-many-recurring = Tento projekt už má 50 opakovaných výdavkov. Ukonči jeden nepotrebný, aby si mohol pridať nový.
error-user-in-recurring = Tento účastník je súčasťou opakovaného výdavku. Najprv ho z neho odstráň alebo výdavok ukonči.

home-owed-to-you = Dlhujú ti
home-you-owe = Dlhuješ
home-details-show = Zobraziť podrobnosti
home-details-hide = Skryť podrobnosti
home-all-settled = Všetko je vyrovnané
home-pay = Zaplatiť
home-remind = Pripomenúť
home-remind-message = Malá pripomienka: v { $project } ešte zostávajú vyrovnania.
home-row-owed = dlhujú ti
home-row-owe = dlhuješ
home-row-settled = Vyrovnané
home-row-quits = U teba vyrovnané
home-row-pending =
    { $count ->
        [one] { $count } čakajúca platba
        [few] { $count } čakajúce platby
        [many] { $count } čakajúcej platby
       *[other] { $count } čakajúcich platieb
    }
home-totals-partial = Bez projektov bez výmenného kurzu
error-account-key-changed = Kľúč tvojho účtu sa zmenil na inom zariadení. Prihlás sa znova a pokračuj.
settings-security = Zabezpečenie
settings-change-password = Zmeniť heslo
settings-change-password-hint = Tvoje projekty a údaje zostanú, ako sú. Tvoje ostatné zariadenia budú odhlásené.
change-password-title = Zmeň svoje heslo
change-password-current = Súčasné heslo
change-password-new = Nové heslo
change-password-confirm = Potvrď nové heslo
change-password-mismatch = Obe nové heslá sa nezhodujú.
change-password-submit = Zmeniť heslo
change-password-submitting = Mením…
change-password-done = Heslo zmenené. Tvoje ostatné zariadenia boli odhlásené.
error-rotation-incomplete = Tvoj účet sa medzitým zmenil. Skús to znova.
error-key-lifecycle-unavailable = Zatiaľ nedostupné: najprv všetci potrebujú najnovšiu verziu aplikácie.
settings-compromised = Niekto možno pozná moje heslo
settings-compromised-hint = Nové heslo, nové kľúče a odhlásenie všetkých ostatných zariadení.
settings-compromised-open = Zabezpečiť môj účet
compromised-title = Zabezpeč svoj účet
compromised-body = Tvoje heslo a kľúče účtu sa nahradia a všetky ostatné zariadenia sa odhlásia. Tvoji priatelia uvidia, že sa tvoj bezpečnostný kód zmenil: porovnajte ho znova.
compromised-keeps = Kto mal tvoje heslo, si ponechá všetko, čo už bolo otvorené: projekty, ktoré sa doteraz dali čítať.
compromised-submit = Nahradiť heslo a kľúče
compromised-done = Tvoj účet je zabezpečený. Požiadaj priateľov, nech porovnajú tvoj nový bezpečnostný kód.
friends-new-fingerprint = Nový kód:
friends-code-matches = Kód sedí
friends-code-trusted = Kód potvrdený.
settings-recovery-key = Kľúč na obnovenie
settings-recovery-key-none = Bez neho ťa zabudnuté heslo stojí nastavenia, platobné údaje a kódy priateľov a obnovenie trvá 3 dni.
settings-recovery-key-set = Vytvorený. Keď zabudneš heslo, vráti ti účet so všetkým obsahom.
settings-recovery-key-pending = Nový kľúč ho nahradí { $when }. Nebol si to ty? Zruš ho svojím súčasným kľúčom.
recovery-key-create = Vytvoriť
recovery-key-replace = Nahradiť
recovery-key-cancel-replacement = Zrušiť nový kľúč
recovery-key-title = Tvoj kľúč na obnovenie
recovery-key-body = Zapíš si ho alebo ho ulož tam, kam sa dostaneš len ty. Counted ho nemôže znova ukázať ani za teba obnoviť.
recovery-key-copy = Kopírovať
recovery-key-copied = Skopírované
recovery-key-save = Uložiť ako súbor
recovery-key-print = Vytlačiť
recovery-key-next = Pokračovať
recovery-key-confirm-body = Na kontrolu kópie zadaj skupinu { $n } kľúča.
recovery-key-confirm-wrong = Táto skupina nesedí. Pozri sa znova na kópiu.
recovery-key-password-body = Zadaj heslo a ulož kľúč do účtu.
recovery-key-submit = Uložiť kľúč
recovery-key-done = Kľúč na obnovenie uložený.
recovery-key-replacement-done = Nový kľúč uložený. Nahradí súčasný o 72 hodín; dovtedy súčasný stále funguje.
recovery-key-field = Kľúč na obnovenie
recovery-key-file-intro = Kľúč na obnovenie Counted
recovery-key-cancel-done = Nový kľúč je zrušený.
compromised-new-key = Ulož si teraz nový kľúč na obnovenie: nahradí starý o 72 hodín a dovtedy starý stále funguje.
pending-reset-title = Bolo požiadané o obnovenie tvojho účtu
pending-reset-body = Vstúpi do platnosti { $when }: potom prídeš o nastavenia, platobné údaje a kódy priateľov a zariadenia sa musia znova prihlásiť. Nebol si to ty? Zruš to súčasným heslom.
pending-reset-cancel = Zrušiť obnovenie
pending-reset-cancelled = Obnovenie je zrušené.
login-forgot = Zabudnuté heslo?
forgot-title = Zabudnuté heslo
forgot-body = Zadaj svoj e-mail. Ak k nemu existuje účet, pošleme odkaz na nové heslo.
forgot-submit = Poslať odkaz
forgot-sent = Ak k adrese existuje účet, odkaz je na ceste. Vyprší o 24 hodín.
reset-title = Získaj späť svoj účet
reset-with-key-body = Zadaj kľúč na obnovenie a nové heslo: všetko sa vráti, ako bolo.
reset-submit-key = Obnoviť môj účet
reset-no-key = Nemám kľúč na obnovenie
reset-without-key-title = Obnoviť bez kľúča na obnovenie
reset-without-key-body = Tvoj účet začne znova s novým heslom o 72 hodín. Projekty sa vrátia z tvojich zariadení a odkazov na zdieľanie; nastavenia, platobné údaje a kódy priateľov nie. Dovtedy súčasné heslo funguje a môže obnovenie zrušiť.
reset-name = Tvoje meno
reset-submit-request = Obnoviť môj účet od začiatku
reset-requested = Hotovo. Nové heslo funguje od { $when }. Dovtedy súčasné heslo funguje a môže to zrušiť.
reset-pending-already = Obnovenie už čaká do { $when }. Nová žiadosť spustí 72 hodín znova.
recovered-done = Účet je späť. Ostatné zariadenia boli odhlásené.
login-reset-undoable = Toto heslo bolo v posledných 30 dňoch nahradené obnovením. Ak si o to nežiadal, vráť to späť: účet sa vráti, ako bol.
login-undo-reset = Vrátiť obnovenie späť
reset-undone = Obnovenie je vrátené. Účet je ako predtým.
error-reset-pending = Nové heslo začne fungovať, keď obnovenie vstúpi do platnosti, 72 hodín po žiadosti.
error-account-disabled = Tento účet bol vymazaný krátko po obnovení a čaká na zmazanie. Predchádzajúce heslo môže obnovenie ešte vrátiť.
error-recovery-key-invalid = Tento kľúč na obnovenie neotvára tento účet.
error-project-key-required = Tento projekt potrebuje svoj kľúč. Otvor ho znova z odkazu na zdieľanie.
names-or = { $first } alebo { $last }
participants-admin-badge = Admin
edit-project-admins = Admini: { $count }
edit-project-succession = Keď v projekte nezostane žiadny admin, rolu prevezme osoba s účtom, ktorá je v projekte najdlhšie.
member-sheet-account = Člen s účtom
member-actions = Akcie: { $name }
member-make-admin = Udeliť rolu admina
member-make-admin-hint = Získa možnosť zmeniť kľúč projektu a niekoho odobrať.
member-made-admin = { $name } má teraz rolu admina.
group-security = Zabezpečenie projektu
group-security-admin = Zabezpečenie projektu · máš rolu admina
group-rotate = Zmeniť kľúč projektu
group-rotate-hint = Ak odkaz na projekt unikol: držitelia starého odkazu stratia prístup k projektu.
group-remove = Odobrať niekoho z projektu
group-remove-hint = Odoberie prístup a zároveň zmení kľúč.
group-admins-only = Len pre adminov.
group-admins-only-account = Len pre adminov, ktorí majú účet.
group-ask-admin = Požiadaj admina projektu: { $names }.
group-ask-admins = Požiadaj adminov projektu: { $names }.
group-ask-an-admin = Požiadaj niektorého z adminov projektu.
group-with-account = S účtom
group-free = Zadarmo
group-sign-in-hint = Admin projektu ({ $names }) ti potom môže rolu udeliť.
group-sign-in-hint-any = Admin projektu ti potom môže rolu udeliť.
group-no-admin = Tento projekt nemá admina, takže jeho kľúč sa nedá zmeniť.
group-hide = Skryť niekoho
group-hide-hint = Skryje túto osobu v zoznamoch bez odobratia prístupu: odkaz aj kľúč naďalej fungujú.
group-recreate-hint = Tento projekt nemá admina. Ak chceš niekoho odobrať, vytvor nový projekt.
group-recreate = Vytvoriť nový projekt
group-no-admin-yet = Tento projekt zatiaľ nemá admina.
group-creator-hint = Len ty sa môžeš stať adminom tohto projektu, a to vytvorením účtu.
group-creator-elsewhere-hint = Adminom projektu sa môže stať len osoba, ktorá ho vytvorila, a to vytvorením účtu. Inak vytvor nový projekt, ak chceš niekoho odobrať.
member-revoke-admin = Odobrať rolu admina
member-revoke-admin-hint = Už nebude môcť meniť kľúč projektu ani nikoho odobrať.
member-revoked-admin = { $name } už nemá rolu admina.
member-step-down = Prestať byť adminom
member-step-down-hint = Už nebudeš môcť meniť kľúč projektu ani nikoho odobrať.
member-stepped-down = Už nemáš rolu admina.
member-remove = Odobrať z projektu
member-remove-hint = Zmení kľúč projektu: { $name } už nebude môcť čítať nič nové.
member-hide = Skryť v zoznamoch
error-user-not-admin = Tento člen nie je admin.
error-creator-proof-invalid = Toto zariadenie tento projekt nevytvorilo.
error-project-has-admin = Tento projekt už má admina.
error-not-project-admin = Toto môžu urobiť len admini projektu.
error-user-not-claimed = Táto osoba nemá účet, takže nemôže byť adminom.
error-project-key-rotated = Kľúč projektu sa zmenil. Požiadaj člena o nový odkaz.
error-project-rotating = Kľúč projektu sa práve mení. Skús to o chvíľu znova.
error-rotation-expired = Zmena kľúča trvala príliš dlho a bola zrušená. Začni znova.
error-rotation-rows-changed = Projekt sa počas zmeny kľúča zmenil. Začni znova.
project-rotated-title = K tomuto projektu už nemáš prístup
project-rotated-body = Kľúč tohto projektu bol zmenený. Požiadaj admina projektu o nový odkaz.
project-rotated-open-link = Otvoriť odkaz
rotation-remove-title = Odobrať { $name } z projektu?
rotation-rotate-title = Zmeniť kľúč projektu?
rotation-remove-body = Odobratie člena znamená vytvorenie nového kľúča. Nový odkaz bude treba zdieľať s ľuďmi bez účtu.
rotation-rotate-body = Kto má starý odkaz, stratí prístup k projektu. Nový odkaz bude treba zdieľať s ľuďmi bez účtu.
rotation-boxed =
    { $count ->
        [0] Kľúč automaticky nedostane nikto ďalší
        [one] Kľúč automaticky dostane { $count } člen
        [few] Kľúč automaticky dostanú { $count } členovia
        [many] Kľúč automaticky dostane { $count } člena
       *[other] Kľúč automaticky dostane { $count } členov
    }
rotation-by-link-title = Ľudia, ktorým budeš musieť poslať nový odkaz
rotation-by-link-hint = Títo ľudia nemajú účet. Odkaz im pošleš hneď potom.
rotation-names-and = { $first } a { $last }
rotation-names-and-others =
    { $names } a { $count ->
        [one] { $count } ďalší
        [few] { $count } ďalší
        [many] { $count } ďalšieho
       *[other] { $count } ďalších
    }
rotation-confirm-remove = Odobrať a zmeniť kľúč
rotation-confirm-rotate = Zmeniť kľúč
rotation-done-title = Nový kľúč platí
rotation-done-removed = { $name } už nie je súčasťou projektu.
rotation-done-rotated = Kľúč projektu sa zmenil.
rotation-done-boxed =
    { $count ->
        [0] Kľúč nebol automaticky odovzdaný nikomu ďalšiemu.
        [one] Kľúč bol automaticky odovzdaný { $count } členovi.
        [few] Kľúč bol automaticky odovzdaný { $count } členom.
        [many] Kľúč bol automaticky odovzdaný { $count } člena.
       *[other] Kľúč bol automaticky odovzdaný { $count } členom.
    }
rotation-new-link =
    { $count ->
        [0] Nový odkaz na projekt
        [one] Nový odkaz na odoslanie · { $count } človek bez účtu
        [few] Nový odkaz na odoslanie · { $count } ľudia bez účtu
        [many] Nový odkaz na odoslanie · { $count } človeka bez účtu
       *[other] Nový odkaz na odoslanie · { $count } ľudí bez účtu
    }
rotation-link-renewed = Odkaz bol úspešne obnovený.
rotation-done = Hotovo
history-key-changed = Kľúč projektu zmenený
history-member-removed = { $name } už nie je súčasťou projektu
project-rotated-row-hint = Kľúč bol zmenený. Požiadaj o nový odkaz.
