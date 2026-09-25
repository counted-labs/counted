# Slovenčina. Kompletné s výnimkou právnych textov (legal-, terms-, privacy-), ktoré existujú len po
# anglicky a francúzsky a pri každej správe zvlášť sa vracajú k en.ftl.

### Common

loading = Načítava sa…
cancel = Zrušiť
confirm = Potvrdiť
retry = Skúsiť znova
delete = Odstrániť
back = Späť
language = Jazyk

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
error-generic = Niečo sa pokazilo. Skús to znova.

error-invalid-email = Táto e-mailová adresa nie je platná.
error-invalid-password = Toto heslo nie je platné.
error-password-too-short = Heslo musí mať aspoň 8 znakov.
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
login-resend = Poslať overovací odkaz znova
login-resending = Odosiela sa…
login-resend-sent = E-mail odoslaný - skontroluj schránku.

register-submit = Vytvoriť účet
register-submitting = Vytvára sa…
register-have-account = Už máš účet?
register-password-placeholder = Aspoň 8 znakov
register-password-warning = Heslo si zapíš. Ak ho zabudneš, účet sa nedá obnoviť.
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
account-delete-warning = Okamžité a trvalé, bez koša. Výdavky, ktoré si zadal v zdieľanom projekte, ostanú ostatným členom viditeľné - sú súčasťou ich účtov.
account-delete-confirm-title = Odstrániť účet
account-delete-confirm-message = Tvoj účet, relácie a zoznam projektov budú trvalo odstránené. Bez hesla sa pre teba šifrované dáta zdieľaného projektu stanú nečitateľnými - to sa nedá vrátiť späť.

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
payment-methods-stale = Tvoje platobné údaje boli zmenené na inom zariadení. Boli znova načítané — skús to prosím znova.
payment-methods-key-missing = Na správu platobných údajov sa znova prihlás.
settings-payment-methods-share-warning = Zdieľaná metóda je viditeľná pre každého člena projektov, kde si si vybral svoje meno - pre každého, kto má jeden z tých odkazov.
payment-method-share = Zdieľať s mojimi projektmi
payment-method-share-hint = Zobrazí sa vedľa tvojho mena, keď ti niekto dlhuje.
payment-method-copy = Kopírovať { $name }
payment-method-copied = Skopírované.
payment-method-copy-failed = Kopírovanie zlyhalo - označ text a skopíruj ho ručne.

verify-email-checking = Overuje sa e-mailová adresa…
verify-email-welcome = E-mail overený - vitaj v Counted!
verify-email-back-to-login = Späť na prihlásenie

### Project status

project-close = Uzavrieť
project-archive = Archivovať
project-reopen = Znova otvoriť
project-unarchive = Obnoviť z archívu

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
projects-empty-hint = Vytvor projekt tlačidlom nižšie
projects-offline-banner = Offline dáta - na obnovenie sa znova pripoj.
projects-no-local-data = Žiadne miestne dáta
projects-no-local-data-hint = Prihlás sa a načítaj svoje projekty prvýkrát.
projects-add = Pridať projekt
projects-create = Vytvoriť projekt
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
add-project-remove-participant = Odstrániť účastníka
add-project-me-badge = Ja
add-project-thats-me = To som ja!
add-project-offline = Offline nemožno vytvoriť projekt. Znova sa pripoj a skús to znova.
add-project-name-required = Projekt potrebuje názov.
add-project-need-two-participants = Pridaj aspoň 2 účastníkov.
add-project-pick-yourself = Povedz nám, ktorý účastník si ty.

join-link-label = Odkaz na zdieľanie
join-link-hint = Odkaz obsahuje dešifrovací kľúč - skopíruj ho celý.
join-invalid-link = Tento odkaz nie je platný. Prilep celý odkaz na zdieľanie vrátane časti za #.
join-wrong-project = Tento odkaz patrí inému projektu.

import-tricount-link-label = Odkaz alebo kľúč Tricount
import-tricount-key-required = Zadaj odkaz alebo kľúč Tricount.
import-tricount-encryption-failed = Šifrovanie zlyhalo.

### Expenses

save = Uložiť
saving = Ukladá sa…
adding = Pridáva sa…
link-copied = Odkaz skopírovaný
missing-encryption-key = Chýba šifrovací kľúč.
missing-encryption-key-title = Chýba šifrovací kľúč
missing-encryption-key-hint = Použitý odkaz neobsahuje kľúč potrebný na dešifrovanie tohto projektu. Použi úplný odkaz od toho, kto projekt vytvoril.
project-locked-hint = Toto zariadenie nemá kľúč k tomuto projektu. Na odomknutie otvor jeho odkaz na zdieľanie.
project-unlock = Odomknúť
project-no-local-data-hint = Prihlás sa a načítaj dáta tohto projektu prvýkrát.
project-gone-title = Tento projekt už neexistuje
project-gone-hint = Bol odstránený, keď ho opustil posledný člen. Odkaz na zdieľanie už nefunguje, ani keď ho znova otvoríš.

expense-add = Pridať výdavok
transfer-add = Pridať prevod
expense-edit-title = Upraviť výdavok
expense-category = Kategória
expense-category-auto = Auto · { $emoji }
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
participants-deselect-all = Zrušiť výber
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
reimbursements-empty-title = Všetko vyrovnané!
reimbursements-empty-hint = Návrhy na vyrovnanie sa tu objavia, keď účty nesedia
reimbursement-owes = { $debtor } dlhuje { $creditor }
reimbursement-record = Vyrovnať
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
edit-project-deferred-me = výber „To som ja“
edit-project-offline-deferred = Offline: { $items } sa použije po opätovnom pripojení.

export-saved = Súbor uložený:
    { $path }
export-failed = Export zlyhal: { $reason }

history-expense-added = Výdavok pridaný: { $name }
history-expense-edited = Výdavok upravený: { $name }
history-expense-deleted = Výdavok odstránený: { $name }
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
expense-inconsistent-detail = Sumy nesedia: zaplatené { $paid }, dlžné { $owed }, pri výdavku { $total }. Uprav výdavok a oprav to.
missing-access-key = Chýba prístupový kľúč. Otvor tento projekt cez jeho odkaz na zdieľanie.
filter-all = Všetko
filter-my-payments = Moje platby
filter-my-debts = Čo dlhujem
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
charts-tab-per-person = Na osobu
charts-tab-trends = Trendy
charts-by-category = Rozdelenie podľa kategórií
charts-per-person = Výdavky na osobu
charts-categories-by-month = Kategórie po mesiacoch
charts-total-spent = Celkom minuté
charts-avg-per-person = Priem. na osobu
charts-expense-count =
    { $count ->
        [one] { $count } výdavok
        [few] { $count } výdavky
        [many] { $count } výdavku
       *[other] { $count } výdavkov
    }
charts-clear-category-filter = Zrušiť filter kategórie
charts-no-expenses = Žiadne výdavky.
charts-pick-a-project = Vyber projekt a uvidíš výdavky na osobu.
charts-nothing-to-show = Nie je čo zobraziť
charts-my-share-note = Tieto sumy sú tvoj podiel na každom výdavku.
charts-my-share-skipped =
    { $count ->
        [one] 1 projekt sa nepočíta — nebol vybraný účastník alebo sa nepodarilo načítať jeho dáta.
        [few] { $count } projekty sa nepočítajú — nebol vybraný účastník alebo sa nepodarilo načítať ich dáta.
        [many] { $count } projektu sa nepočíta — nebol vybraný účastník alebo sa nepodarilo načítať ich dáta.
       *[other] { $count } projektov sa nepočíta — nebol vybraný účastník alebo sa nepodarilo načítať ich dáta.
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
charts-person = Osoba
charts-project = Projekt
charts-all-projects = Všetky projekty
charts-whole-project = Celý projekt
charts-date-from = Od
charts-date-to = Do
charts-total = Celkom
charts-payments-per-person-by-month = Platby na osobu po mesiacoch
history-empty = Žiadne udalosti
history-by = { $name }
not-found-hint = Táto stránka neexistuje alebo sa presunula.
payers-title-paid-by = Zaplatil(a)
payers-title-sender = Odosielateľ
payers-title-contributors = Prispievatelia
debtors-title-debtors = Dlhuje
debtors-title-recipients = Príjemcovia
debtors-title-beneficiaries = Príjemcovia

### Welcome

welcome-title = Do tvojich účtov nikoho nič nie je.
welcome-subtitle = Deľ sa o výdavky s priateľmi.
welcome-e2ee-title = Všetko šifrované
welcome-e2ee-body = Mená, sumy, projekty: všetko sa šifruje na tvojom zariadení. Kľúč máš len ty. Nikto nemôže čítať tvoje vyúčtovania. Ani my.
welcome-e2ee-note = Nečitateľné aj pre nás (nulový prístup servera)
welcome-eu-title = 100 % európske
welcome-eu-body = Servery v Nemecku, e-maily odosielané z Francúzska. Tvoje dáta nikdy neopustia Európsku úniu.
welcome-noads-title = Žiadne reklamy. Žiadne sledovanie.
welcome-noads-body = Nič nezbierame a tvoje údaje nepredávame. To nie je náš model.
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
help-forgot-password-warning = Tvoje dáta budú nenávratne stratené.
help-forgot-password-a = Šifrovací kľúč sa odvodzuje z hesla, takže reset nie je možný: bez neho nikto - ani my - tvoje projekty nedešifruje. Uchovaj ho v bezpečí, ideálne v správcovi hesiel.
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
project-currency-hint = Každá suma sa zobrazuje v tejto mene. Neskôr ju nemožno zmeniť.
project-currency-locked = Mena sa nastavuje pri vytvorení projektu.

update-required-title = Vyžaduje sa aktualizácia
update-required-body = Táto verzia Counted je príliš stará na komunikáciu so serverom. Aktualizuj ju, aby si mohol aplikáciu ďalej používať.
update-required-body-testflight = Táto verzia Counted je príliš stará na komunikáciu so serverom. Otvor TestFlight a nainštaluj najnovšie zostavenie, aby si mohol aplikáciu ďalej používať.
update-required-button = Aktualizovať

notifications-label = Oznámenia
notifications-title = Oznámenia
notifications-empty = Nič nové
notifications-friend-request = Žiadosť o priateľstvo

friends-title = Priatelia
friends-anonymous-body = Priatelia sú viazaní na tvoj účet. Prihlás sa, aby si mohol pridávať ľudí a pozývať ich do projektov bez zdieľania odkazu.
friends-add-title = Pridať priateľa
friends-add-hint = Žiadosť uvidí po prihlásení. Ani jeden z vás sa nedozvie, či má druhý účet, kým žiadosť nie je prijatá.
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
friends-remove-confirm-message = { $email } už nebude medzi vašimi priateľmi a vy medzi jeho. Ktokoľvek z vás môže neskôr poslať novú žiadosť.
friends-no-key = Zatiaľ nie je pripravené
friends-fingerprint = Bezpečnostný kód
friends-fingerprint-hint = Dvaja priatelia, ktorí si prečítajú rovnaký bezpečnostný kód, vedia, že medzi nimi nikto nesedí - ani náš server.
friends-outgoing-title = Odoslané
friends-outgoing-hint = Čaká na odpoveď. Po prijatí sa objavia medzi tvojimi priateľmi.
friends-withdraw = Zrušiť
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
