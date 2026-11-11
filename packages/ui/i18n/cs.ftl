# Čeština. Kompletní s výjimkou právních textů (legal-, terms-, privacy-), které existují jen anglicky
# a francouzsky a u každé zprávy zvlášť se vracejí k en.ftl.

### Common

loading = Načítání…
cancel = Zrušit
discard-title = Zahodit změny?
discard-message = Zadané údaje se ztratí.
discard = Zahodit
confirm = Potvrdit
retry = Zkusit znovu
delete = Smazat
back = Zpět
language = Jazyk
theme = Motiv
theme-light = Světlý
theme-dark = Tmavý
theme-system = Systémový
theme-hint-device = Uloženo jen v tomto zařízení.
theme-hint-system = Řídí se vzhledem tohoto zařízení.

### Navigation

nav-main = Hlavní navigace
nav-projects = Projekty
nav-charts = Statistiky
nav-settings = Nastavení

### Connectivity

offline-banner = Offline
offline-pending =
    { $count ->
        [one] { $count } čekající
        [few] { $count } čekající
        [many] { $count } čekajících
       *[other] { $count } čekajících
    }

sync-conflict-edit = Konflikt: úprava „{ $name }“ selhala (položka smazána). Přeskočeno.
sync-conflict-delete = Konflikt: smazání „{ $name }“ selhalo (položka smazána). Přeskočeno.
sync-conflict-other = Konflikt: operace s „{ $name }“ selhala (položka smazána). Přeskočeno.
sync-error = Chyba synchronizace: { $reason }

### Errors

error-network = Server je nedostupný. Zkontroluj připojení k internetu.
error-rate-limited = Server je přetížený. Zkus to znovu za chvíli.
error-generic = Něco se pokazilo. Zkus to znovu.

error-invalid-email = Tato e-mailová adresa není platná.
error-invalid-password = Toto heslo není platné.
error-password-too-short = Heslo musí mít alespoň 12 znaků.
error-client-outdated = Tato verze aplikace je zastaralá. Pro přihlášení ji aktualizuj.
error-invalid-link = Tento odkaz není platný.
error-batch-too-large = Příliš mnoho položek najednou.
error-payers-required = Vyber alespoň jednoho plátce.
error-debtors-required = Vyber alespoň jednu osobu, která dluží.
error-duplicate-participant = Účastník se na stejné straně objevuje dvakrát.
error-participant-not-in-project = Tento účastník není součástí projektu.
error-too-many-participants = Příliš mnoho účastníků na jeden výdaj.
error-invalid-credentials = Nesprávný e-mail nebo heslo.
error-unauthenticated = Pro tuto akci se přihlas.
error-email-not-verified = Tvoje e-mailová adresa ještě není ověřená.
error-project-not-found = Tento projekt už neexistuje.
error-expense-not-found = Tento výdaj už neexistuje.
error-storage-full = Úložiště je plné: klíč tohoto projektu se na tomto zařízení nepodařilo uložit. Uschovej si odkaz ke sdílení.
error-user-not-found = Tento účastník už neexistuje.
error-tricount-not-found = Tricount nenalezen, nebo jeho API vrátilo chybu.
error-too-many-members = Tento projekt dosáhl limitu členů.
error-identity-taken = Tohoto účastníka si už nárokoval jiný účet.
error-claim-proof-invalid = Toto zařízení nemá klíč projektu, takže si nemůže nárokovat účastníka. Otevři znovu odkaz pro sdílení.
error-user-has-payments = Tento účastník má v projektu výdaje a nelze ho odebrat.
error-resend-cooldown = Před vyžádáním dalšího e-mailu počkej 60 sekund.
error-self-friend-request = Nemůžeš si přidat sám sebe jako přítele.
error-not-a-friend = Pozvat můžeš jen lidi ze svého seznamu přátel.
error-friend-has-no-key = Tento přítel ještě neotevřel nejnovější verzi aplikace. Požádej ho, ať se jednou přihlásí, a zkus to znovu.
error-friend-request-not-found = Tato žádost o přátelství už neexistuje.
error-invitation-not-found = Tato pozvánka už neexistuje.
error-too-many-friend-requests = Příliš mnoho žádostí o přátelství. Zkus to zítra.
error-too-many-invitations = Příliš mnoho čekajících pozvánek.
error-invalid-kdf-salt = Nastavení šifrování není platné. Aktualizuj aplikaci a zkus to znovu.
error-mixed-project-batch = Tito účastníci nejsou všichni ve stejném projektu.
error-invalid-payload = Tato verze aplikace odeslala data, která server nepřijímá. Aktualizuj ji a zkus to znovu.
error-invalid-public-key = Tvůj šifrovací klíč není platný. Aktualizuj aplikaci a zkus to znovu.
error-payment-methods-stale = Tvoje platební údaje byly změněny na jiném zařízení. Načti znovu a zkus to.

### Auth

field-email = E-mail
field-email-placeholder = ty@priklad.cz
field-password = Heslo
field-name = Jméno
field-name-placeholder = Jana Nováková

login-title = Přihlásit se
login-submit = Přihlásit se
login-submitting = Přihlašování…
login-password-placeholder = Tvoje heslo
login-no-account = Ještě nemáš účet?
login-unverified = Tvoje e-mailová adresa ještě není ověřená. Zkontroluj schránku, nebo si nech odkaz poslat znovu.
login-maybe-unverified = Nový účet? Nejdřív ověř svou adresu odkazem, který jsme ti poslali, nebo si ho pošli znovu.
login-resend = Poslat ověřovací odkaz znovu
login-resending = Odesílání…
login-resend-sent = E-mail odeslán - zkontroluj schránku.

register-submit = Vytvořit účet
register-submitting = Vytváření…
register-have-account = Už máš účet?
register-password-placeholder = Alespoň 12 znaků
register-password-warning = Heslo si zapiš: nikdy ho nevidíme a nemůžeme ti ho vrátit.
register-check-email-title = Zkontroluj e-mail
register-email-sent = E-mail odeslán
register-email-sent-hint = Pro aktivaci účtu klikni na odkaz ve schránce.
register-not-received-prefix = Nepřišel? Zkontroluj spam, nebo se
register-sign-in-link = přihlas
register-not-received-suffix = a nech si odkaz poslat znovu.
register-terms-prefix = Vytvořením účtu přijímáš naše
register-terms-link = podmínky použití
register-terms-and = a naše
register-privacy-link = zásady ochrany soukromí

settings-title = Nastavení
settings-preferences = Předvolby
settings-preferences-local = Uloženo na tomto zařízení.
settings-preferences-synced = Synchronizováno s tvým účtem, šifrováno.
settings-anonymous-title = Nejsi přihlášen
settings-upsell-title = Tvoje projekty na každém zařízení
settings-upsell-free = Zdarma

account-logout = Odhlásit se
account-logging-out = Odhlašování…
account-delete-title = Smazat můj účet
account-delete-confirm-title = Smazat účet
account-delete-confirm-message = Tvůj účet, relace a seznam projektů budou trvale smazány. Bez hesla se pro tebe šifrovaná data sdíleného projektu stanou nečitelnými - to nelze vrátit zpět.
erase-device-title = Vymazat toto zařízení?
erase-device-body = Projekty, jejich klíče a mezipaměť na tomto zařízení budou smazány a potom bude relace ukončena. Tvoje projekty zůstávají na tvém účtu: přihlas se znovu a získáš je zpět.
erase-device-checking = Ověřuji, že tvoje projekty jsou na tvém účtu…
erase-device-pending = Změny čekající na odeslání: { $count }. Připoj se znovu k internetu a zkus to znovu.
erase-device-unreachable = Nepodařilo se ověřit, že tvoje projekty jsou na tvém účtu. Zkontroluj připojení a zkus to znovu.
erase-device-at-risk = Tyto projekty nejsou na tvém účtu a budou ztraceny. Pokud si je chceš ponechat, zruš to a nejdřív zkopíruj jejich odkaz:
erase-device-confirm = Vymazat a odhlásit
erase-device-confirm-anyway = Přesto vymazat
erase-device-erasing = Mazání…

settings-payment-methods = Platební údaje
payment-method-kind = Způsob
payment-method-kind-other = Jiný
payment-method-label = Název
payment-method-label-placeholder = Hlavní účet
payment-method-value = Údaje
payment-method-value-placeholder = IBAN, telefonní číslo, uživatelské jméno…
payment-method-add = Přidat
payment-method-deleted = Platební metoda smazána.
payment-method-value-required = Vyplň údaje každé platební metody, nebo ji odeber.
payment-method-label-required = Pojmenuj svou vlastní metodu.
payment-method-too-long = To je příliš dlouhé - zkrať to.
payment-method-invalid-characters = Odstraň zalomení řádků nebo neviditelné znaky.
payment-method-limit = Můžeš uložit až { $max } platebních metod.
payment-methods-saved = Platební údaje uloženy.
payment-methods-offline = Pro uložení platebních údajů musíš být online.
payment-methods-stale = Tvoje platební údaje byly změněny na jiném zařízení. Byly znovu načteny - zkus to prosím znovu.
payment-methods-key-missing = Pro správu platebních údajů se znovu přihlas.
payment-method-copy = Kopírovat { $name }
payment-method-copied = Zkopírováno.
payment-method-copy-failed = Kopírování selhalo - označ text a zkopíruj ho ručně.

verify-email-checking = Ověřování e-mailové adresy…
verify-email-welcome = E-mail ověřen - vítej v Counted!
verify-email-back-to-login = Zpět na přihlášení
verify-email-intro = Zadej své heslo a potvrď, že tahle adresa je tvoje.
verify-email-submit = Ověřit moji adresu

### Project status

project-archive = Archivovat
project-sheet-invite = Pozvat
project-sheet-recurring = Opakované
project-sheet-edit = Upravit projekt
project-sheet-close = Uzavřít projekt
project-sheet-archive = Archivovat projekt
project-sheet-reopen = Znovu otevřít projekt
project-sheet-unarchive = Obnovit projekt z archivu
project-sheet-leave = Opustit projekt

### Dates

date-long = { $day }. { $month } { $year }

month-1 = ledna
month-2 = února
month-3 = března
month-4 = dubna
month-5 = května
month-6 = června
month-7 = července
month-8 = srpna
month-9 = září
month-10 = října
month-11 = listopadu
month-12 = prosince

month-short-1 = led
month-short-2 = úno
month-short-3 = bře
month-short-4 = dub
month-short-5 = kvě
month-short-6 = čvn
month-short-7 = čvc
month-short-8 = srp
month-short-9 = zář
month-short-10 = říj
month-short-11 = lis
month-short-12 = pro

### Actions

add = Přidat
create = Vytvořit
creating = Vytváření…
edit = Upravit
leave = Opustit
close = Zavřít
paste = Vložit
join = Připojit se
import = Importovat
importing = Importování…
field-description = Popis
field-date = Datum
date-today = Dnes
date-yesterday = Včera
field-optional = Nepovinné

### Projects

projects-filter-active = Aktivní
projects-filter-all = Vše
projects-count-label = Projekty
projects-empty = Žádné projekty
projects-offline-banner = Offline data - pro obnovení se znovu připoj.
demo-banner = Ukázkový projekt - pouze pro čtení.
demo-start-own = Založit vlastní projekt
demo-badge = Demo
demo-bar = Tvoje změny zmizí po zavření této karty.
demo-leave = Začít naostro
demo-preparing = Připravujeme tvoje demo…
demo-failed = Demo se nepodařilo připravit.
demo-sign-in-blocked = Přihlášení je v demu vypnuté. Opusť ho, aby ses mohl přihlásit nebo vytvořit účet; tvoje vlastní projekty zůstanou nedotčené.
projects-no-local-data = Žádná místní data
projects-no-local-data-hint = Přihlas se a načti své projekty poprvé.
projects-add = Přidat projekt
projects-create = Vytvořit projekt
projects-personal-expense = Osobní výdaj
personal-expense-banner = Přidáno do Mých výdajů
personal-create-title = Vytvoř si osobní projekt
personal-create-hint = Sleduj, co utrácíš sám
group-create-title = Vytvoř sdílený projekt
group-create-hint = Rozděl si výdaje s přáteli a rodinou
personal-create-action = Vytvořit
personal-create-signup = Vytvořit účet
personal-sheet-title = Tvůj osobní projekt
personal-sheet-intro = Pro zapisování toho, co utrácíš sám. Zobrazí se s tvými skupinami ve Statistikách.
personal-sheet-point-alone = Jediný člen: ty, se jménem tvého účtu. Bez odkazu ke sdílení.
personal-sheet-point-encrypted = Šifrovaný na tvých zařízeních, jako tvoje další projekty.
personal-sheet-point-stats = Tvoje podíly ve skupinách se ve Statistikách připočítají k těmto výdajům.
personal-default-name = Moje výdaje
my-spending-prev-month = Předchozí měsíc
my-spending-next-month = Další měsíc
my-spending-spent = Utratil jsi
my-spending-groups = Skupiny
my-spending-income = Příjmy
my-spending-balance = Bilance měsíce
my-spending-unconverted = Podíly v jiné měně zůstávají stranou, dokud není znám kurz.
my-spending-empty = Tento měsíc nic
my-spending-empty-hint = Přidej výdaj pomocí +. Tvoje podíly ve skupinách se tu zobrazí automaticky.
my-spending-share-of = z { $amount }
my-spending-uncounted = nezapočteno
payer-you = ty
expense-received-by = přijal(a)
counted-switch = Započítat do Moje výdaje
counted-switch-on = Zapnuto: tvůj podíl se přičítá k Moje výdaje
counted-switch-off = Vypnuto: tvůj podíl se v Moje výdaje nezobrazuje
counted-groups = Započtené skupiny
counted-groups-intro = Tvůj podíl v těchto skupinách se přičítá k Moje výdaje a k rozpočtům.
counted-groups-converted = { $from }, převedeno na { $to }
counted-groups-encrypted = Tato volba je šifrována tvým účtem a následuje tě na tvých zařízeních.
counted-groups-empty = Na tomto zařízení zatím není žádná skupina.
counted-groups-done = OK
counted-save-failed = Tuto volbu se nepodařilo uložit. Zkus to znovu.
tab-budget = Rozpočet
budget-title = Rozpočty
budget-manage = Spravovat
budget-left = Zbývá { $amount }
budget-over = Překročeno o { $amount }
budget-empty = Žádný rozpočet
budget-empty-hint = Nastav strop pro každou kategorii. Tvoje podíly ve skupinách se počítají také.
budget-define = Nastavit rozpočet
budget-sheet-title = Měsíční rozpočty
budget-sheet-intro = Na tvůj podíl: osobní a skupiny. Nech prázdné, pokud kategorii nechceš sledovat.
budget-average = Prům. poslední 3 měsíce: { $amount }
budget-for = Rozpočet { $category }
budget-alert = Upozornit mě při 80 %
budget-alert-hint = Počítáno na tomto zařízení
budget-invalid = Neplatná částka pro { $category }.
budget-offline = Rozpočty lze uložit jen online. Připoj se a zkus to znovu.
budget-alert-near = { $category }: { $percent } % měsíčního rozpočtu
budget-alert-over = { $category }: měsíční rozpočet překročen
my-spending-search-summary =
    { $count ->
        [one] { $count } výsledek
        [few] { $count } výsledky
        [many] { $count } výsledku
       *[other] { $count } výsledků
    } · { $total } · osobní a skupiny, všechny měsíce
projects-join = Připojit se k projektu
projects-import-tricount = Importovat z Tricountu
project-actions = Akce projektu

status-ongoing = Probíhá
status-closed = Uzavřený
status-archived = Archivovaný

nav-help = Nápověda
nav-privacy = Zásady ochrany soukromí
nav-terms = Podmínky použití
nav-legal = Právní informace

leave-project-title = Opustit projekt?
leave-project-message = Ztratíš přístup z tohoto zařízení. Pokud nezůstane žádný člen, projekt a všechny jeho výdaje budou trvale smazány.

add-project-title = Nový projekt
add-project-name-label = Název projektu
add-project-name-placeholder = Můj výlet, Spolubydlení 2024…
add-project-participants = Účastníci
add-project-participant-name = Jméno účastníka
add-project-participant-placeholder = Clark Kent
add-project-offline = Offline nelze projekt vytvořit. Znovu se připoj a zkus to znovu.
add-project-name-required = Projekt potřebuje název.

join-link-label = Odkaz pro sdílení
join-link-hint = Odkaz obsahuje dešifrovací klíč - zkopíruj ho celý.
join-invalid-link = Tento odkaz není platný. Vlož celý odkaz pro sdílení včetně části za #.
join-wrong-project = Tento odkaz patří jinému projektu.

import-tricount-link-label = Odkaz nebo klíč Tricount
import-tricount-key-required = Zadej odkaz nebo klíč Tricount.
import-tricount-encryption-failed = Šifrování selhalo.
import-tricount-unimportable = Nic nebylo importováno: tento Tricount má členy s účtem Tricount nebo částky, které nesedí (dotčené položky: { $count }).

### Expenses

save = Uložit
saving = Ukládání…
adding = Přidávání…
link-copied = Odkaz zkopírován
missing-encryption-key = Chybí šifrovací klíč.
missing-encryption-key-title = Chybí šifrovací klíč
missing-encryption-key-hint = Použitý odkaz neobsahuje klíč potřebný k dešifrování tohoto projektu. Použij úplný odkaz od toho, kdo projekt vytvořil.
project-locked-hint = Toto zařízení nemá klíč k tomuto projektu. Pro odemknutí otevři jeho odkaz pro sdílení.
expenses-unreadable = Některé položky nejdou na tomto zařízení přečíst a nejsou započítané do součtů. Pro jejich zobrazení aktualizuj aplikaci.
project-unlock = Odemknout
project-no-local-data-hint = Přihlas se a načti data tohoto projektu poprvé.
project-gone-title = Tento projekt už neexistuje
project-gone-hint = Byl smazán, když ho opustil poslední člen. Odkaz pro sdílení už nefunguje, ani když ho znovu otevřeš.

expense-add = Přidat výdaj
transfer-add = Přidat převod
expense-edit-title = Upravit výdaj
expense-category = Kategorie
expense-category-auto = Auto · { $emoji }
emoji-picker-label = Emoji
emoji-none = Bez emoji
expense-currency = Měna částky
amount-op-add = Plus
amount-op-subtract = Minus
amount-op-multiply = Krát
amount-op-divide = Děleno
amount-op-equals = Rovná se
amount-op-done = Hotovo
expense-rate = Směnný kurz (nepovinné)
expense-rate-hint = Nech prázdné pro použití kurzu Evropské komise (InforEuro) za { $month }: 1 { $from } = { $rate } { $to }.
expense-rate-invalid = Zadej směnný kurz větší než 0.
expense-rate-unavailable = Automatický kurz není k dispozici - zadej ho ručně.
expense-delete-title = Smazat výdaj
expense-delete-message = „{ $name }“ bude trvale smazán. To nelze vrátit zpět.
expense-inconsistent-amounts = Částky nesouhlasí
expenses-empty = Žádné výdaje
expenses-empty-hint = Začni přidáním výdajů tlačítkem níže
expenses-show-more = Zobrazit více (zbývá { $count })

expense-type-expense = Výdaj
expense-type-transfer = Převod
expense-type-gain = Příjem
expense-type-group = Typ
expense-paid-by = zaplatil(a)
expense-sent-by = poslal(a)
expense-contributed-by = přispěl(a)

expense-name-required = Název je povinný.
expense-amount-not-positive = Částka musí být větší než 0.
expense-no-payer = Vyber alespoň jednoho plátce.
expense-no-debtor = Vyber alespoň jednu osobu, která dluží.
expense-invalid-date = Toto datum není platné.
expense-payers-mismatch = Součet plátců je { $sum }, což neodpovídá částce výdaje ({ $total }).
expense-debtors-mismatch = Součet dlužníků je { $sum }, což neodpovídá částce výdaje ({ $total }).

participants-none = Nikdo
participants-everyone = Všichni ({ $count })
participants-some = { $count } z { $total }
participants-select-all = Vybrat vše
participants-by-shares = Podle podílů
split-amounts = Částky
participants-remaining = Zbývá { $amount }
participants-over-by = O { $amount } navíc
participants-who-paid = Kdo platil?
participants-who-received = Kdo přijal?
participants-who-transfers = Kdo převádí?
participants-who-receives = Kdo přijímá?
participants-for-whom = Pro koho?

stats-total-expenses = Celkové výdaje
stats-my-expenses = Moje výdaje

tab-expenses = Výdaje
tab-balance = Bilance
tab-reimbursements = Vyrovnání
balance-gets-back = Dostane zpět
balance-owes = Dluží
balance-settled = Vyrovnáno
reimbursements-empty-title = Vše vyrovnáno!
reimbursements-empty-hint = Návrhy na vyrovnání se zde objeví, když účty nesedí
reimbursement-owes = { $debtor } dluží { $creditor }
reimbursement-record = Vyrovnat
settlement-recorded = Vyrovnání zaznamenáno.
reimbursement-pay-with = Zaplatit
reimbursement-pay-shared-by = Sdílí { $name } - před odesláním zkontroluj jméno příjemce, které zobrazuje tvoje aplikace.
reimbursement-pay-title = Zaplatit { $name }
reimbursements-mine-title = Dlužíš
reimbursements-others-title = Ostatní vyrovnání
copy = Kopírovat

user-selection-title = Který účastník jsi ty?
user-selection-hint = Vyber své jméno ze seznamu.
user-selection-required = Vyber prosím účastníka.
identity-claimed = Propojeno s účtem
identity-claimed-by = Účet { $name }
identity-taken-repick = Účastníka, kterého jsi používal, si nárokoval jiný účet. Vyber si prosím jiného.
participant-gone-repick = Účastník, kterého jsi používal, byl z projektu odebrán. Vyber si prosím jiného.

edit-project-title = Upravit projekt
edit-project-new-badge = nový
edit-project-deferred-new-members = přidání nových členů
edit-project-deferred-removals = odebrání členů
edit-project-offline-deferred = Offline: { $items } se použije po opětovném připojení.

export-failed = Export selhal: { $reason }

history-expense-added = Výdaj přidán: { $name }
history-expense-edited = Výdaj upraven: { $name }
history-expense-deleted = Výdaj smazán: { $name }
move-expense = Přesunout do projektu
move-expense-title = Přesunout do…
move-current = Aktuální
move-converted = { $currency }, přepočteno kurzem měsíce
move-note-personal = V dalším kroku zvolíš, kdo platil a za koho. Výdaj opustí Moje výdaje.
move-note-group = Výdaj opustí { $project } pro všechny členy a bilance skupiny se přepočítá. V dalším kroku zvolíš, kdo platil a za koho.
move-banner = Z { $from } do { $to }
move-submit = Přesunout
move-done = Výdaj přesunut do { $project }
history-expense-moved = Výdaj přesunut do jiného projektu: { $name }
personal-badge = Osobní
move-members =
    { $count ->
        [one] { $count } člen
        [few] { $count } členové
        [many] { $count } člena
       *[other] { $count } členů
    }
history-project-edited = Projekt upraven: { $name }
history-name-changed = Název: „{ $from }“ → „{ $to }“
history-description-added = Popis přidán: „{ $value }“
history-description-removed = Popis odebrán: „{ $value }“
history-description-changed = Popis: „{ $from }“ → „{ $to }“

### Sweep

field-amount = Částka
expense-name-placeholder = Restaurace, nákup…
expense-actions = Akce výdaje
expense-your-share = Tvůj podíl
expense-your-share-value = Tvůj podíl: { $amount } { $currency }
expense-share-percent = { $percent } %
expense-added-by = Přidáno: { $name }, { $date }
expense-inconsistent-detail = Částky nesouhlasí: zaplaceno { $paid }, dluženo { $owed }, u výdaje { $total }. Uprav výdaj a oprav to.
missing-access-key = Chybí přístupový klíč. Otevři tento projekt přes jeho odkaz pro sdílení.
filter-all = Vše
filter-my-payments = Moje platby
filter-my-debts = Co dlužím
expense-filter-group = Filtr
expenses-search = Hledat výdaje
expenses-no-match = Žádný výdaj neodpovídá.
expenses-search-open = Hledat
expenses-search-clear = Vymazat hledání
expenses-search-summary =
    { $count ->
        [one] { $count } výsledek
        [few] { $count } výsledky
        [many] { $count } výsledku
       *[other] { $count } výsledků
    } · { $total } · všechny měsíce
participants-shares-for = Podíly pro { $name }
participants-amount-for = Částka pro { $name }
reimbursement-add = Přidat vyrovnání
project-forget = Odebrat z mého seznamu
project-history-title = Historie
history-kind-add = Přidáno
history-kind-delete = Smazáno
history-kind-edit = Upraveno
export = Exportovat
export-json = Exportovat JSON
export-csv = Exportovat CSV
share-link = Sdílet
copy-link-failed = Odkaz se nepodařilo zkopírovat
open-in-app = Otevřít v aplikaci
not-found-title = Stránka nenalezena
not-found-back = Zpět na projekty

### Charts

charts-period = Období
period-all = Vše
period-month = Měsíc
period-3months = 3 měs.
period-year = Rok
period-custom = Vlastní
charts-tab-categories = Kategorie
charts-tab-trends = Trendy
charts-total-spent = Celkem utraceno
charts-avg-per-person = Prům. na osobu
charts-expense-count =
    { $count ->
        [one] { $count } výdaj
        [few] { $count } výdaje
        [many] { $count } výdaje
       *[other] { $count } výdajů
    }
charts-nothing-to-show = Není co zobrazit
charts-my-share-skipped =
    { $count ->
        [one] 1 projekt se nepočítá - nebyl vybrán účastník, nebo se nepodařilo načíst jeho data.
        [few] { $count } projekty se nepočítají - nebyl vybrán účastník, nebo se nepodařilo načíst jejich data.
        [many] { $count } projektu se nepočítá - nebyl vybrán účastník, nebo se nepodařilo načíst jejich data.
       *[other] { $count } projektů se nepočítá - nebyl vybrán účastník, nebo se nepodařilo načíst jejich data.
    }

### Categories

category-food = Jídlo
category-transport = Doprava
category-accommodation = Ubytování
category-leisure = Volný čas
category-shopping = Nákupy
category-services = Služby
category-parties-gifts = Oslavy a dárky
category-other = Ostatní
categories-manage = Spravovat kategorie
categories-title = Kategorie
categories-yours = Tvoje kategorie
categories-builtin = Výchozí
categories-add = Přidat kategorii
categories-in-parent = v kategorii { $parent }
categories-renamed-from = přejmenována · původně { $name }
categories-hidden-hint = skryta · její výdaje jdou do kategorie { $other }
categories-always-shown = vždy zobrazena
categories-hidden = Skrytá
categories-new = Nová kategorie
categories-edit = Upravit kategorii
categories-parent = Počítá se do kategorie
categories-parent-hint = Ve statistikách všech tvých projektů se { $name } počítá do kategorie { $parent }.
categories-create = Vytvořit
categories-hide = Skrýt tuto kategorii
categories-hide-hint = Její výdaje se všem členům zobrazí v kategorii { $other }.
categories-delete-title = Smazat „{ $name }“?
categories-delete-body = Dotčené výdaje ({ $count }) se vrátí do kategorie { $parent }. Žádný výdaj se nesmaže.
categories-delete-body-unknown = Její výdaje se vrátí do kategorie { $parent }. Žádný výdaj se nesmaže.
categories-delete-warning = Kategorie zmizí všem členům projektu.
categories-summary-created = Vytvořené: { $count }
categories-summary-renamed = Přejmenované: { $count }
categories-summary-hidden = Skryté: { $count }
categories-summary-default = Žádné vlastní kategorie
categories-offline = Kategorie lze spravovat jen online. Připoj se a zkus to znovu.
categories-error-too-many = Projekt má nejvýše 20 vlastních kategorií.
categories-error-full = Tento projekt dosáhl limitu kategorií.
categories-error-name = Pojmenuj kategorii.
categories-error-missing = Tato kategorie se mezitím změnila. Zavři a zkus to znovu.
history-category-created = Kategorie vytvořena: { $name }
history-category-edited = Kategorie upravena: { $name }
history-category-deleted = Kategorie smazána: { $name }
charts-project = Projekt
charts-all-projects = Všechny projekty
charts-date-from = Od
charts-date-to = Do
charts-total = Celkem
charts-tab-people = Lidé
charts-tab-projects = Projekty
charts-scope = Čí výdaje
charts-scope-group = Skupina
charts-scope-me = Já
charts-currency = Měna
charts-my-share = Můj podíl
charts-share-of-total = { $pct } % z { $total }
charts-i-paid = Zaplatil jsem
charts-paid-more = O { $amount } víc než můj podíl
charts-paid-less = O { $amount } méně než můj podíl
charts-paid-even = Přesně můj podíl
charts-part-title = Tvůj podíl v každé kategorii
charts-part-desc = Šedě je, co utratila skupina, barevně, co jsi spotřeboval ty.
charts-breakdown-title = Rozdělení podle kategorií
charts-breakdown-desc = Klepni na výseč nebo řádek a uvidíš výdaje.
charts-of-total = { $amount } z { $total }
charts-show-all = Zobrazit vše ({ $count })
charts-show-less = Zobrazit méně
charts-spend-title = Výdaje v čase
charts-group-by = Seskupit podle
bucket-day = Den
bucket-week = Týden
bucket-month = Měsíc
charts-avg = prům.
charts-cat-title-day = { $category }, den po dni
charts-cat-title-week = { $category }, týden po týdnu
charts-cat-title-month = { $category }, měsíc po měsíci
charts-cat-desc = Vyber kategorii a sleduj ji v čase.
charts-running-title = Průběžný součet
charts-running-desc = Od { $date }.
charts-avg-per-day = { $amount } / den v průměru
charts-avg-per-week = { $amount } / týden v průměru
charts-avg-per-month = { $amount } / měsíc v průměru
charts-people-title = Kdo táhl skupinu
charts-people-desc = Co kdo zaplatil, vedle toho, co spotřeboval.
charts-paid = Zaplaceno
charts-fair-share = Férový podíl
charts-you = (ty)
charts-net-more = zaplatil víc
charts-net-less = zaplatil méně
charts-balance-title = Tvůj zůstatek v čase
charts-balance-desc = Nad čarou ti skupina dluží. Pod ní dlužíš ty skupině.
charts-owed = Dluží ti
charts-owe = Dlužíš
charts-projects-title = Tvůj podíl podle projektu
charts-projects-desc = Součty se vedou po měnách.
history-empty = Žádné události
history-by = { $name }
not-found-hint = Tato stránka neexistuje, nebo se přesunula.
payers-title-paid-by = Zaplatil(a)
payers-title-sender = Odesílatel
payers-title-contributors = Přispěvatelé
debtors-title-debtors = Dluží
debtors-title-recipients = Příjemci
debtors-title-beneficiaries = Příjemci
debtors-title-for = Pro
debtors-title-recipient = Příjemce
expense-split-between = rozděleno mezi { $count }
expense-people =
    { $count ->
        [one] { $count } osoba
        [few] { $count } osoby
        [many] { $count } osoby
       *[other] { $count } osob
    }

### Welcome

welcome-title = Do tvých účtů nikomu nic není.
welcome-subtitle = Děl se o výdaje s přáteli.
welcome-note = Zdarma. Bez účtu. Bez reklam.
welcome-link-title = Jeden odkaz a všichni se zapojí.
welcome-link-body = Nikdo si nemusí zakládat účet.
welcome-link-account = Účet? Nikdy není povinný. Slouží k nalezení tvých projektů na jiném zařízení, zvaní přátel z aplikace a sdílení tvých platebních údajů.
welcome-demo-project = Víkend v Lyonu
welcome-private-title = Nikdo nemůže číst tvoje vyúčtování. Ani my.
welcome-private-body = Jména, částky, projekty: vše se šifruje na tvém zařízení. Klíč máš jen ty.
welcome-private-names = Jména
welcome-private-amounts = Částky
welcome-private-projects = Projekty
welcome-scan-title = Vyfoť účtenku.
welcome-scan-body = Částka, datum a kategorie se vyplní samy. Vše probíhá v tvém telefonu. Fotka se neukládá.
welcome-eu-title = 100% evropské
welcome-no-ads = Žádné reklamy
welcome-no-trackers = Žádné sledování
welcome-step = Krok { $current } z { $total }
welcome-next = Další
welcome-skip = Přeskočit
welcome-start = Začít
welcome-how-it-works = Jak to přesně funguje?

### Help

help-intro = Častá otázka? Klepnutím rozbalíš odpověď.
help-create-project-q = Jak vytvořím projekt?
help-create-project-a = Na domovské obrazovce klepni na tlačítko + dole. Pojmenuj projekt, vyber měnu a je hotovo.
help-add-participants-q = Jak přidám účastníky?
help-add-participants-a = Otevři projekt a přidej účastníky ze seznamu členů. Každý účastník může za výdaj platit nebo ho dlužit.
help-share-project-q = Jak sdílím projekt?
help-share-project-a = Sdílej URL projektu (tu v adresním řádku). Kdokoli s odkazem může projekt prohlížet a upravovat.
help-add-expense-q = Jak přidám výdaj?
help-add-expense-a = V projektu klepni na +, zadej částku, urči, kdo platil a mezi koho výdaj rozdělit. Můžeš také zvolit jiné datum než dnešní.
help-types-q = Jaký je rozdíl mezi výdajem, převodem a příjmem?
help-types-expense = - nákup jedné osoby rozdělený mezi několik.
help-types-transfer = - vrácení peněz od jedné osoby druhé, bez rozdělení.
help-types-gain = - přijaté peníze (vratka, dar) k rozdělení mezi několik osob.
help-past-date-q = Mohu výdaj datovat do minulosti?
help-past-date-a = Ano, pole data je libovolné. Čas vytvoření záznamu se ukládá zvlášť.
help-who-owes-q = Jak Counted zjistí, kdo co dluží?
help-who-owes-a = Counted spočítá čistou bilanci každého účastníka (co zaplatil minus co dluží) a pak navrhne nejkratší sérii převodů, která všechny vyrovná.
help-minimal-transfers-q = Proč je počet navržených převodů minimální?
help-minimal-transfers-a = Algoritmus nejprve spáruje bilance, které se přesně vyruší, a pak projde zbytek od největšího věřitele k největšímu dlužníkovi. Výsledek: méně převodů k úplnému vyrovnání.
help-import-tricount-q = Jak importuji projekt z Tricountu?
help-import-tricount-a = Na domovské obrazovce klepni na tlačítko „+“ dole a pak
help-import-tricount-b = Vlož odkaz pro sdílení Tricountu, který chceš importovat.
help-encryption-q = Jsou moje data šifrovaná?
help-encryption-a = Ano. Counted kombinuje dvě záruky:
help-encryption-e2ee-term = End-to-end šifrování
help-encryption-e2ee-def = - vše mezi tebou a serverem cestuje šifrovaně.
help-encryption-zero-term = Nulový přístup
help-encryption-zero-def = - data šifruješ před odesláním a server ukládá jen šifrovaný text. Nemáme jak ho přečíst.
help-encryption-see = Podrobnosti najdeš v
help-forgot-password-q = Co se stane, když zapomenu heslo?
help-forgot-password-warning = Nikdo ho za tebe nemůže obnovit.
help-forgot-password-a = Tvoje heslo chrání tvé šifrovací klíče a nikdy se k nám neposílá: nikdo - ani my - tvoje projekty za tebe nedešifruje. Uchovej ho v bezpečí, ideálně ve správci hesel.
help-archive-delete-q = Jak projekt archivuji nebo smažu?
help-archive-delete-a = Na obrazovce projektu otevři menu a zvol
help-archive-delete-b = a projekt se skryje, ale zůstane zachován. Projekt je trvale smazán, jakmile ho opustí poslední člen.
help-delete-account-q = Jak smažu svůj účet?
help-delete-account-a = Otevři Nastavení a použij „Smazat můj účet“. Je to okamžité a nelze to vrátit zpět.
help-contact = Další otázka? Napiš nám na

# Receipt scanning (mobile only)
expense-scan = Naskenovat účtenku
scan-in-progress = Čtení účtenky…
scan-error-capture = Fotku se nepodařilo pořídit. Zkus to znovu, nebo zadej výdaj ručně.
scan-error-unreadable = Na účtence není nic čitelného. Zadej výdaj ručně.
scan-check-amount = Zkontroluj součet - nebyl zřetelně vytištěn.
scan-take-photo = Vyfotit
scan-choose-photo = Vybrat fotku
expense-converted-from = Zaplaceno { $amount } { $from } · 1 { $from } = { $rate } { $to }
project-currency = Měna
project-currency-locked = Měna se nastavuje při vytvoření projektu.
currency-search = Hledat měnu

update-required-title = Vyžadována aktualizace
update-required-body = Tato verze Counted je příliš stará na komunikaci se serverem. Aktualizuj ji, abys mohl aplikaci dál používat.
update-required-button = Aktualizovat

notifications-label = Oznámení
notifications-title = Oznámení
notifications-empty = Nic nového

friends-title = Přátelé
friends-anonymous-body = Přátelé jsou vázáni na tvůj účet. Přihlas se, abys mohl přidávat lidi a zvát je do projektů bez sdílení odkazu.
friends-add-title = Přidat přítele
friends-add-from-project = Přidat jako přítele
friends-request-sent = Žádost odeslána
friends-no-account-key = Pro správu přátel na tomto zařízení se znovu přihlas.
friends-incoming-title = Žádosti
friends-accept = Přijmout
friends-decline = Odmítnout
friends-list-title = Moji přátelé
friends-remove = Odebrat
friends-remove-confirm-title = Odebrat přítele
friends-remove-confirm-message = { $email } už nebude mezi tvými přáteli a ty mezi jeho. Kdokoli z vás může později poslat novou žádost.
friends-no-key = Zatím není připraveno
friends-key-mismatch = Server poslal pro tvůj účet veřejný klíč, který neodpovídá tvému soukromému klíči. Tvůj bezpečnostní kód je skrytý a pozvánky jsou na tomto zařízení vypnuté: někdo možná manipuluje se serverem.
keypair-mismatch = Klíče, které server poslal pro tvůj účet, neodpovídají těm na tomto zařízení. Pozvánky a změny klíče jsou tu vypnuté, dokud nebudou sedět: někdo možná manipuluje se serverem.
friends-outgoing-title = Odeslané
friends-notify = Dát vědět zprávou
friends-notify-message = Přecházím na Counted kvůli našim společným výdajům. Registrace zdarma na https://counted.fr s adresou { $email }: čeká tam moje žádost o přátelství.
friends-notify-copied = Zpráva zkopírována
friends-notify-copy-failed = Zprávu se nepodařilo zkopírovat
guests-hint-title =
    { $count ->
        [one] { $count } účastník se ještě nepřipojil k projektu
        [few] { $count } účastníci se ještě nepřipojili k projektu
        [many] { $count } účastníka se ještě nepřipojilo k projektu
       *[other] { $count } účastníků se ještě nepřipojilo k projektu
    }
guests-hint-later = Později
guests-hint-send = Pozvi je
guests-send-title = Poslat projekt účastníkům
guests-send-friend = Pozvat přítele z Countedu
guests-send-friend-badge = Bez odkazu
guests-send-friend-hint = Pokud už má účet, dostane šifrovanou pozvánku; nic neprochází tvou aplikací na zprávy.
guests-send-link = Sdílet odkaz na projekt
guests-send-link-hint = Pro ty, kdo nemají účet. Projekt otevřou jedním klepnutím.
guests-send-link-warning = Kdokoli s tímto odkazem může projekt číst a upravovat. Tvá aplikace na zprávy ho vidí také.
invite-friends-title = Pozvat přátele
invite-friends-hint = Klíč projektu se na tomto zařízení šifruje pro každého přítele. Server ho nikdy nevidí.
invite-friends-empty = Zatím žádní přátelé k pozvání.
invite-friends-button = Pozvat
invite-sent = { $count ->
    [one] Pozvánka odeslána
    [few] Odeslány { $count } pozvánky
    [many] Odesláno { $count } pozvánky
   *[other] Odesláno { $count } pozvánek
}
invitation-badge = Pozvánka
invitation-to = Připojit se k „{ $name }“
invitation-to-unnamed = Připojit se k projektu
invitation-unreadable = Tuto pozvánku nelze na tomto zařízení otevřít
invitation-from = Od { $email }
invitation-accept = Připojit se
invitation-decline = Odmítnout

# Participants in the create and edit modals, and the "who are you?" picker - see
# docs/plans/friends.md §11.
participants-you-label = Tvoje jméno v tomto projektu
participants-you-badge = Ty
participants-others = Další účastníci
participants-removed = Odebraní
participant-restore = Obnovit
participants-empty = Přidej účastníky níže.
participants-duplicate = „{ $name }“ už v seznamu je.
participants-input-label = Přidat přítele nebo napsat jméno
participants-input-placeholder = Přítel nebo jakékoli jméno
participants-suggest-friend = Přítel · připojí se jako „{ $name }“, dostane pozvánku
participants-suggest-not-ready = Přítel · zatím není připraven
participants-suggest-guest = Přidat „{ $text }“
participants-suggest-guest-sub = Host
participants-friends = Tvoji přátelé
participants-all-friends = Všichni přátelé
participants-login-hint = Přihlas se a přidávej lidi přímo ze seznamu přátel.
participants-invite-badge = Pozvat
participants-guest-badge = Host
participants-rename = Přejmenovat: { $name }
participants-remove = Odebrat: { $name }
participants-rename-label = Nové jméno
participants-rename-save = Uložit jméno
participants-rename-hint = Jméno, které v tomto projektu vidí všichni. Pozvánka stále půjde na { $email }.
participants-invited-badge = Pozván
participants-invited-sub = { $email } · zatím nepřijato
participants-invited-pending = Pozvánka zatím nepřijata
participants-unlinked = Nepropojeno s účtem
add-project-create-invite = Vytvořit a pozvat: { $count }
edit-project-save-invite = Uložit a pozvat: { $count }
edit-project-you-are = V tomto projektu jsi { $name }
edit-project-no-identity = Ještě jsi nevybral(a), kdo jsi
edit-project-switch = Změnit
edit-project-choose = Vybrat
invite-failed = Tyto pozvánky se nepodařilo odeslat: { $emails }
invite-waiting = Tito přátelé musí jednou otevřít Counted, než je bude možné pozvat: { $emails }
key-offer-title = Jiný klíč pro { $name }
key-offer-body = Toto zařízení má k tomuto projektu jiný klíč, který ho tu nikdy neotevřel.
key-offer-choices = Použít tento klíč: projekt se otevře klíčem z tohoto odkazu nebo pozvánky. Zrušit: zařízení si ponechá svůj současný klíč a nic se nezmění.
key-offer-use = Použít tento klíč
invite-again = Pozvat znovu
friend-picker-title = Přidat přátele
user-selection-invited-hint = { $email } tě pozval(a) do „{ $project }“.
user-selection-suggested = Navrženo
user-selection-suggested-sub = { $email } tě přidal(a) pod tímto jménem
user-selection-confirm-as = Jsem { $name }
user-selection-missing = Není tu tvoje jméno? Požádej účastníka, ať tě přidá v nastavení projektu.

## Opakované výdaje

repeat-label = Opakovat
repeat-none = Neopakuje se
repeat-weekly = Každý týden
repeat-biweekly = Každé 2 týdny
repeat-monthly = Každý měsíc
repeat-quarterly = Každé 3 měsíce
repeat-yearly = Každý rok
repeat-every-weeks = Každých { $count } týd.
repeat-every-months = Každých { $count } měs.
repeat-every-years = Každých { $count } let
repeat-custom = Vlastní…
repeat-every = Každých
repeat-unit-weeks = týd.
repeat-unit-months = měs.
repeat-unit-years = let
repeat-on-weekday = v den: { $weekday }
repeat-on-day = { $day }. den v měsíci
repeat-on-day-month = { $day }. { $month }
repeat-month-end = V kratších měsících připadne na poslední den.
repeat-ends = Konec
repeat-ends-never = Nikdy
repeat-ends-on = V datum
repeat-ends-after = Po
repeat-fewer = Méně
repeat-more = Více
repeat-last-on = poslední { $date }
repeat-variable = Částka se pokaždé mění
repeat-variable-hint = Každý se přidá s poslední částkou a označí „k potvrzení“.
repeat-done = Hotovo
repeat-no-end = Bez konce
repeat-until = Do { $date }
repeat-occurrences = Počet opakování: { $count }
repeat-offline = Vyžaduje připojení. Samotný výdaj lze přidat i tak.
repeat-foreign = Opakuje se jako { $amount } { $currency }, přepočteno jednou dnešním kurzem. Upozorníme tě, pokud se kurz změní o více než 5 %.
repeat-backfill = Začíná v minulosti. Výdaje přidané teď: { $count }.
add-and-repeat = Přidat a opakovat
weekday-1 = pondělí
weekday-2 = úterý
weekday-3 = středa
weekday-4 = čtvrtek
weekday-5 = pátek
weekday-6 = sobota
weekday-7 = neděle
recurring-title = Opakované výdaje
recurring-strip = Opakované výdaje: { $count }
recurring-next = Další: { $name }, { $date }
recurring-to-confirm = K potvrzení: { $count }
recurring-per-month = Měsíčně, cca
recurring-your-share = Tvůj podíl
recurring-active = Aktivní
recurring-paused = Pozastavené
recurring-finished = Ukončené
recurring-paid-by = platí { $name }
recurring-next-on = Další { $date }
recurring-progress = { $done } z { $total }
recurring-rate-badge = Kurz se změnil o { $percent } %
recurring-empty = Zatím se nic neopakuje. Při přidávání výdaje zvol „Opakovat“: nájem, předplatné, účty.
recurring-next-ones = Další termíny
recurring-added-so-far = Zatím přidáno
recurring-set-up-by = Vytvořil(a)
recurring-pause = Pozastavit
recurring-resume = Obnovit
recurring-stop = Ukončit opakování
recurring-stop-title = Ukončit „{ $name }“?
recurring-stop-message = Už se nebude opakovat. Již přidané výdaje zůstanou.
recurring-resume-title = Obnovit „{ $name }“?
recurring-resume-message = Další { $date }. Termíny zmeškané během pozastavení se nepřidají.
recurring-edit-title = Upravit opakovaný výdaj
recurring-edit-banner = Změny platí od { $date }. Již přidané výdaje zůstanou beze změny.
recurring-next-on-label = Další
recurring-next-too-early = Další datum musí být po posledním již přidaném výdaji.
recurring-use-stop = K ukončení použij „Ukončit opakování“ u opakovaného výdaje.
recurring-drift = Kurz { $currency } se od vytvoření změnil o { $percent } %. Každý se stále přidává jako { $amount } { $project_currency } (1 { $currency } = { $rate }). Dnešním kurzem by to bylo { $today_amount } { $project_currency }.
recurring-use-rate = Použít dnešní kurz
recurring-keep = Ponechat { $amount } { $currency }
recurring-added = Přidané opakované výdaje: { $count }
recurring-blocks-removal = { $name } zatím nelze odebrat: je součástí { $rules }. Odeber { $name } z nich nebo je ukonči a pak znovu ulož.
history-recurring-added = Přidáno automaticky: { $name } ({ $date })
history-recurring-created = Opakovaný výdaj vytvořen: { $name }
history-recurring-edited = Opakovaný výdaj upraven: { $name }
history-recurring-paused = Opakovaný výdaj pozastaven: { $name }
history-recurring-resumed = Opakovaný výdaj obnoven: { $name }
history-recurring-stopped = Opakovaný výdaj ukončen: { $name }
occurrence-recurring = Opakovaný výdaj
occurrence-auto = Přidáno automaticky z opakovaného výdaje.
occurrence-auto-next = Přidáno automaticky z opakovaného výdaje. Další { $date }.
occurrence-auto-stopped = Přidáno automaticky z opakovaného výdaje, který byl mezitím ukončen.
occurrence-manage = Spravovat
estimate-badge = K potvrzení
expense-pending = Čeká
estimate-title = Částka k potvrzení.
estimate-body = Přidáno s předchozí částkou. Zadej skutečnou, až ji budeš znát.
estimate-confirm = Potvrdit částku
apply-to = Použít na
apply-this-only = Jen tento výdaj
apply-and-next = Tento a další
apply-and-next-hint = Opakovaný výdaj se mění od { $date }
apply-rule-failed = Výdaj byl uložen, ale opakovaný výdaj se nezměnil.
occurrence-delete-message = „{ $name }“ z { $date } bude trvale smazán. Opakování pokračuje a toto datum se nevrátí.
occurrence-delete-one = Smazat jen tento
occurrence-delete-stop = Smazat a ukončit opakování
error-recurring-clock = Hodiny tohoto zařízení jdou napřed. Zkontroluj datum a čas.
error-recurring-not-found = Tento opakovaný výdaj už neexistuje.
error-recurring-stale = Někdo mezitím tento opakovaný výdaj změnil. Byl znovu načten: zkontroluj ho a ulož znovu.
error-too-many-recurring = Tento projekt už má 50 opakovaných výdajů. Ukonči jeden, který už nepotřebuješ, a přidej nový.
error-user-in-recurring = Tento účastník je součástí opakovaného výdaje. Nejdřív ho z něj odeber nebo výdaj ukonči.

home-owed-to-you = Dluží ti
home-you-owe = Dlužíš
home-details-show = Zobrazit podrobnosti
home-details-hide = Skrýt podrobnosti
home-all-settled = Vše je vyrovnáno
home-pay = Zaplatit
home-remind = Připomenout
home-remind-message = Malá připomínka: v { $project } ještě zbývají vyrovnání.
home-row-owed = dluží ti
home-row-owe = dlužíš
home-row-settled = Vyrovnáno
home-row-quits = U tebe vyrovnáno
home-row-pending =
    { $count ->
        [one] { $count } čekající platba
        [few] { $count } čekající platby
        [many] { $count } čekající platby
       *[other] { $count } čekajících plateb
    }
home-totals-partial = Bez projektů bez směnného kurzu
error-account-key-changed = Klíč tvého účtu se změnil na jiném zařízení. Přihlas se znovu a pokračuj.
settings-security = Zabezpečení
settings-change-password-hint = Tvé projekty a data zůstanou, jak jsou. Tvá ostatní zařízení budou odhlášena.
change-password-title = Změň své heslo
change-password-current = Současné heslo
change-password-new = Nové heslo
change-password-confirm = Potvrď nové heslo
change-password-mismatch = Obě nová hesla se neshodují.
change-password-submit = Změnit heslo
change-password-submitting = Měním…
change-password-done = Heslo změněno. Tvá ostatní zařízení byla odhlášena.
error-rotation-incomplete = Tvůj účet se mezitím změnil. Zkus to znovu.
error-key-lifecycle-unavailable = Zatím nedostupné: nejdřív všichni potřebují nejnovější verzi aplikace.
settings-compromised-open = Zabezpečit můj účet
compromised-title = Zabezpeč svůj účet
compromised-body = Tvé heslo a klíče účtu se nahradí a všechna ostatní zařízení se odhlásí. Tví přátelé uvidí, že se tvůj bezpečnostní kód změnil: porovnejte ho znovu.
compromised-keeps = Kdo měl tvé heslo, si ponechá vše, co už bylo otevřeno: projekty, které šlo dosud číst.
compromised-submit = Nahradit heslo a klíče
compromised-done = Tvůj účet je zabezpečený. Požádej přátele, ať porovnají tvůj nový bezpečnostní kód.
friends-code-matches = Kód sedí
friends-code-trusted = Kód potvrzen.
settings-recovery-key = Klíč pro obnovení
settings-recovery-key-pending = Nový klíč ho nahradí { $when }. Nebyl jsi to ty? Zruš ho svým současným klíčem.
recovery-key-cancel-replacement = Zrušit nový klíč
recovery-key-title = Tvůj klíč pro obnovení
recovery-key-body = Zapiš si ho nebo ho ulož tam, kam se dostaneš jen ty. Counted ho nemůže znovu ukázat ani za tebe obnovit.
recovery-key-copy = Kopírovat
recovery-key-copied = Zkopírováno
recovery-key-save = Uložit jako soubor
recovery-key-print = Vytisknout
recovery-key-next = Pokračovat
recovery-key-confirm-body = Pro kontrolu kopie zadej skupinu { $n } klíče.
recovery-key-confirm-wrong = Tato skupina nesedí. Podívej se znovu na kopii.
recovery-key-password-body = Zadej heslo a ulož klíč do účtu.
recovery-key-submit = Uložit klíč
recovery-key-done = Klíč pro obnovení uložen.
recovery-key-replacement-done = Nový klíč uložen. Nahradí současný za 72 hodin; do té doby současný stále funguje.
recovery-key-field = Klíč pro obnovení
recovery-key-file-intro = Klíč pro obnovení Counted
recovery-key-cancel-done = Nový klíč je zrušen.
compromised-new-key = Ulož si teď nový klíč pro obnovení: nahradí starý za 72 hodin a do té doby starý stále funguje.
pending-reset-title = Požádáno o obnovení
pending-reset-body = Vstoupí v platnost { $when }. Nebyl jsi to ty? Zruš to svým heslem.
pending-reset-cancel = Zrušit obnovení
pending-reset-cancelled = Obnovení je zrušeno.
login-forgot = Zapomenuté heslo?
forgot-title = Zapomenuté heslo
forgot-body = Zadej svůj e-mail. Pokud k němu existuje účet, pošleme odkaz pro nové heslo.
forgot-submit = Poslat odkaz
forgot-sent = Pokud k adrese existuje účet, odkaz je na cestě. Vyprší za 24 hodin.
reset-title = Získej zpět svůj účet
reset-with-key-body = Zadej klíč pro obnovení a nové heslo: vše se vrátí, jak bylo.
reset-submit-key = Obnovit můj účet
reset-no-key = Nemám klíč pro obnovení
reset-without-key-title = Obnovit bez klíče pro obnovení
reset-without-key-body = Tvůj účet začne znovu s novým heslem za 72 hodin. Projekty se vrátí z tvých zařízení a odkazů pro sdílení; nastavení, platební údaje a kódy přátel ne. Do té doby současné heslo funguje a může obnovení zrušit.
reset-name = Tvé jméno
reset-submit-request = Obnovit můj účet od začátku
reset-requested = Hotovo. Nové heslo funguje od { $when }. Do té doby současné heslo funguje a může to zrušit.
reset-pending-already = Obnovení už čeká do { $when }. Nová žádost spustí 72 hodin znovu.
recovered-done = Účet je zpět. Ostatní zařízení byla odhlášena.
login-reset-undoable = Toto heslo bylo v posledních 30 dnech nahrazeno obnovením. Pokud jsi o to nežádal, vrať to zpět: účet se vrátí, jak byl.
login-undo-reset = Vrátit obnovení zpět
reset-undone = Obnovení je vráceno. Účet je jako dřív.
error-reset-pending = Nové heslo začne fungovat, až obnovení vstoupí v platnost, 72 hodin po žádosti.
error-account-disabled = Tento účet byl smazán krátce po obnovení a čeká na vymazání. Předchozí heslo může obnovení ještě vrátit.
error-recovery-key-invalid = Tento klíč pro obnovení neotevírá tento účet.
error-project-key-required = Tento projekt potřebuje svůj klíč. Otevři ho znovu z odkazu pro sdílení.
names-or = { $first } nebo { $last }
participants-admin-badge = Admin
edit-project-admins = Admini: { $count }
edit-project-succession = Když v projektu nezůstane žádný admin, roli převezme osoba s účtem, která je v projektu nejdéle.
member-sheet-account = Člen s účtem
member-actions = Akce: { $name }
member-make-admin = Udělit roli admina
member-make-admin-hint = Získá možnost změnit klíč projektu a někoho odebrat.
member-made-admin = { $name } má teď roli admina.
group-security = Zabezpečení projektu
group-security-admin = Zabezpečení projektu · máš roli admina
group-rotate = Změnit klíč projektu
group-rotate-hint = Pokud odkaz na projekt unikl: držitelé starého odkazu ztratí přístup k projektu.
group-remove = Odebrat někoho z projektu
group-remove-hint = Odebere přístup a zároveň změní klíč.
group-admins-only = Jen pro adminy.
group-admins-only-account = Jen pro adminy, kteří mají účet.
group-ask-admin = Požádej admina projektu: { $names }.
group-ask-admins = Požádej adminy projektu: { $names }.
group-ask-an-admin = Požádej některého z adminů projektu.
group-with-account = S účtem
group-free = Zdarma
group-sign-in-hint = Admin projektu ({ $names }) ti pak může roli udělit.
group-sign-in-hint-any = Admin projektu ti pak může roli udělit.
group-no-admin = Tento projekt nemá admina, takže jeho klíč nejde změnit.
group-hide = Skrýt někoho
group-hide-hint = Skryje tuto osobu v seznamech, ale přístup jí ponechá: odkaz i klíč dál fungují.
group-recreate-hint = Tento projekt nemá admina. Chceš-li někoho odebrat, vytvoř nový projekt.
group-recreate = Vytvořit nový projekt
group-no-admin-yet = Tento projekt zatím nemá admina.
group-creator-hint = Jen ty se můžeš stát adminem tohoto projektu, a to vytvořením účtu.
group-creator-elsewhere-hint = Adminem projektu se může stát jen osoba, která ho vytvořila, a to vytvořením účtu. Jinak vytvoř nový projekt, chceš-li někoho odebrat.
member-revoke-admin = Odebrat roli admina
member-revoke-admin-hint = Už nebude moct měnit klíč projektu ani nikoho odebrat.
member-revoked-admin = { $name } už nemá roli admina.
member-step-down = Přestat být adminem
member-step-down-hint = Už nebudeš moct měnit klíč projektu ani nikoho odebrat.
member-stepped-down = Už nemáš roli admina.
member-remove = Odebrat z projektu
member-remove-hint = Změní klíč projektu: { $name } už nebude moct číst nic nového.
member-hide = Skrýt v seznamech
error-user-not-admin = Tento člen není admin.
error-creator-proof-invalid = Toto zařízení tento projekt nevytvořilo.
error-project-has-admin = Tento projekt už má admina.
error-not-project-admin = Tohle mohou udělat jen admini projektu.
error-user-not-claimed = Tato osoba nemá účet, takže nemůže být adminem.
error-project-key-rotated = Klíč projektu se změnil. Požádej člena o nový odkaz.
error-project-rotating = Klíč projektu se právě mění. Zkus to za chvíli znovu.
error-rotation-expired = Změna klíče trvala příliš dlouho a byla zrušena. Začni znovu.
error-rotation-rows-changed = Projekt se během změny klíče změnil. Začni znovu.
project-rotated-title = K tomuto projektu už nemáš přístup
project-rotated-body = Klíč tohoto projektu byl změněn. Požádej admina projektu o nový odkaz.
project-rotated-open-link = Otevřít odkaz
rotation-remove-title = Odebrat { $name } z projektu?
rotation-rotate-title = Změnit klíč projektu?
rotation-remove-body = Odebrání člena znamená vytvoření nového klíče. Nový odkaz bude potřeba sdílet s lidmi bez účtu.
rotation-rotate-body = Kdo má starý odkaz, ztratí přístup k projektu. Nový odkaz bude potřeba sdílet s lidmi bez účtu.
rotation-boxed =
    { $count ->
        [0] Klíč automaticky neobdrží nikdo další
        [one] Klíč automaticky obdrží { $count } člen
        [few] Klíč automaticky obdrží { $count } členové
        [many] Klíč automaticky obdrží { $count } člena
       *[other] Klíč automaticky obdrží { $count } členů
    }
rotation-by-link-title = Lidé, kterým budeš muset poslat nový odkaz
rotation-by-link-hint = Tito lidé nemají účet. Odkaz jim pošleš hned potom.
rotation-names-and = { $first } a { $last }
rotation-names-and-others =
    { $names } a { $count ->
        [one] { $count } další
        [few] { $count } další
        [many] { $count } dalšího
       *[other] { $count } dalších
    }
rotation-confirm-remove = Odebrat a změnit klíč
rotation-confirm-rotate = Změnit klíč
rotation-done-title = Nový klíč platí
rotation-done-removed = { $name } už není součástí projektu.
rotation-done-rotated = Klíč projektu se změnil.
rotation-done-boxed =
    { $count ->
        [0] Klíč nebyl automaticky předán nikomu dalšímu.
        [one] Klíč byl automaticky předán { $count } členovi.
        [few] Klíč byl automaticky předán { $count } členům.
        [many] Klíč byl automaticky předán { $count } člena.
       *[other] Klíč byl automaticky předán { $count } členům.
    }
rotation-new-link =
    { $count ->
        [0] Nový odkaz na projekt
        [one] Nový odkaz k odeslání · { $count } člověk bez účtu
        [few] Nový odkaz k odeslání · { $count } lidé bez účtu
        [many] Nový odkaz k odeslání · { $count } člověka bez účtu
       *[other] Nový odkaz k odeslání · { $count } lidí bez účtu
    }
rotation-link-renewed = Odkaz byl úspěšně obnoven.
rotation-done = Hotovo
history-key-changed = Klíč projektu změněn
history-member-removed = { $name } už není součástí projektu
project-rotated-row-hint = Klíč byl změněn. Požádej o nový odkaz.

## Settings and Friends redesign (docs/plans/friends-settings-redesign.md)
settings-group-sharing = Sdílení
settings-group-help = Nápověda a informace
settings-friends-changed = { $count ->
    [one] { $count } kód změněn
    [few] { $count } kódy změněny
    [many] { $count } kódu změněno
   *[other] { $count } kódů změněno
}
settings-recovery-key-created = Vytvořen
settings-recovery-key-missing = K vytvoření
settings-recovery-key-replacing = Výměna čeká
settings-password = Heslo
settings-compromised-short = Zná tvé heslo někdo jiný?
settings-recovery-key-footnote = Bez klíče pro obnovení smaže zapomenuté heslo tvé předvolby, platební údaje a kódy přátel.
erase-device-row = Vymazat toto zařízení
erase-device-row-hint = Odhlásí tě a ponechá jen jazyk.
account-delete-hint = Okamžité a trvalé.
settings-open-source = Otevřený zdrojový kód, AGPL-3.0
settings-upsell-short = Bez účtu zůstanou tvé projekty na tomto zařízení. S účtem tě následují všude, šifrované.
settings-locked-group = S účtem
settings-locked-friends-short = Zvi bez odkazu
settings-locked-payment-short = Uvidí to ti, kdo ti dluží
payment-methods-title = Platební metody
payment-methods-row-hint = Sdíleno s přáteli
payment-methods-intro = Sdíleno s přáteli. Šifrováno tvým účtem.
payment-methods-footnote = Platební metoda se sdílí (max. 20) se členy tvých projektů na kartě Vyrovnání.
payment-method-shared-pill = Sdíleno
payment-method-share-groups = Sdílet ve svých skupinách
payment-method-label-optional = Název (nepovinné)
payment-method-delete-confirm-title = Smazat tuto platební metodu?
payment-method-delete-confirm-message = { $name } se už nebude zobrazovat ve tvých projektech.
payment-methods-empty-title = Žádná platební metoda
payment-methods-empty-body = Přidej svůj IBAN nebo platební aplikaci: uvidí ho ti, kdo ti dluží.
payment-method-masked = { $kind } končící na { $last }
friends-to-check = Ke kontrole
friends-code-changed-row = Jeho kód se změnil
friends-own-code = Tvůj bezpečnostní kód
friends-own-code-hint = Tví přátelé vidí tento kód vedle tvé adresy. Pokud se shoduje s jejich, zvou tebe a nikoho jiného.
friends-received = Přijaté
friends-received-on = Přijato { $date }
friends-pending = Čeká
friends-from-project = Z projektu
friends-accept-label = Přijmout žádost od { $email }
friends-decline-label = Odmítnout žádost od { $email }
friends-add-hint-short = Tato osoba uvidí tvou žádost, pokud má účet. Jinak jí dej vědět.
friends-send = Odeslat
friends-sent-hint = Ještě nemá účet? Dej jí vědět: zpráva ji pozve k registraci s touto adresou.
friends-done = Hotovo
friends-their-code = Jeho bezpečnostní kód
friends-code-compare-hint = Přečtěte si ho spolu. Musí vidět stejný kód nahoře na své stránce Přátelé.
friends-no-key-hint = Zatím není připraveno: jeho klíč se vytvoří, až příště otevře Counted. Do té doby ho nemůžeš pozvat.
friends-remove-row = Odebrat z mých přátel
friends-new-code = Nový kód
friends-key-changed-hint-short = Ověřte si tento kód spolu, mimo tuto aplikaci. Do té doby jsou pozvánky s ním zablokované.
friends-kem-changed-title = Jeho klíč byl nahrazen
friends-kem-changed-body = Postkvantová část jeho klíče se změnila, ale jeho kód ne. Zabezpečení účtu mění obojí, takže někdo mohl tento klíč podvrhnout. Pozvánky s ním zůstávají zablokované.
friends-kem-changed-hint = Můžeš ho odebrat a pak poslat novou žádost: jeho klíč se znovu připne.
friends-pending-sheet = Žádost odeslána, čeká.
friends-withdraw-row = Zrušit žádost
friends-empty-title = Zatím žádní přátelé
friends-empty-body = Přidej někoho e-mailem a pak ho pozvi do projektu bez sdílení odkazu.
friends-requests-empty-title = Žádné žádosti
friends-requests-empty-body = Přijaté a odeslané žádosti se zobrazí tady.
friends-code-hidden = Tvůj kód je skrytý.
friends-code-hidden-body = Klíč poskytnutý pro tvůj účet neodpovídá tomuto zařízení. Pozvánky jsou tu vypnuté.
notifications-manage-friends = Spravovat mé přátele
notifications-friend-request-on = Žádost o přátelství · { $date }

### Link preview

share-preview-title = Naše výdaje v Countedu
share-preview-description = Otevři odkaz a uvidíš výdaje a kdo komu kolik dluží. Bez účtu, šifrováno ve tvém zařízení.

### Currency names

currency-EUR = Euro
currency-AED = SAE dirham
currency-AFN = Afghánský afghán
currency-ALL = Albánský lek
currency-AMD = Arménský dram
currency-AOA = Angolská kwanza
currency-ARS = Argentinské peso
currency-AUD = Australský dolar
currency-AWG = Arubský zlatý
currency-AZN = Ázerbájdžánský manat
currency-BAM = Bosenská konvertibilní marka
currency-BBD = Barbadoský dolar
currency-BDT = Bangladéšská taka
currency-BHD = Bahrajnský dinár
currency-BIF = Burundský frank
currency-BMD = Bermudský dolar
currency-BND = Brunejský dolar
currency-BOB = Bolivijský boliviano
currency-BRL = Brazilský real
currency-BSD = Bahamský dolar
currency-BTN = Bhútánský ngultrum
currency-BWP = Botswanská pula
currency-BYN = Běloruský rubl
currency-BZD = Belizský dolar
currency-CAD = Kanadský dolar
currency-CDF = Konžský frank
currency-CHF = Švýcarský frank
currency-CLP = Chilské peso
currency-CNY = Čínský jüan
currency-COP = Kolumbijské peso
currency-CRC = Kostarický colón
currency-CUP = Kubánské peso
currency-CVE = Kapverdské escudo
currency-CZK = Česká koruna
currency-DJF = Džibutský frank
currency-DKK = Dánská koruna
currency-DOP = Dominikánské peso
currency-DZD = Alžírský dinár
currency-EGP = Egyptská libra
currency-ERN = Eritrejská nakfa
currency-ETB = Etiopský birr
currency-FJD = Fidžijský dolar
currency-FKP = Falklandská libra
currency-GBP = Britská libra
currency-GEL = Gruzínské lari
currency-GHS = Ghanský cedi
currency-GIP = Gibraltarská libra
currency-GMD = Gambijský dalasi
currency-GNF = Guinejský frank
currency-GTQ = Guatemalský quetzal
currency-GYD = Guyanský dolar
currency-HKD = Hongkongský dolar
currency-HNL = Honduraská lempira
currency-HTG = Haitský gourde
currency-HUF = Maďarský forint
currency-IDR = Indonéská rupie
currency-ILS = Izraelský nový šekel
currency-INR = Indická rupie
currency-IQD = Irácký dinár
currency-IRR = Íránský rijál
currency-ISK = Islandská koruna
currency-JMD = Jamajský dolar
currency-JOD = Jordánský dinár
currency-JPY = Japonský jen
currency-KES = Keňský šilink
currency-KGS = Kyrgyzský som
currency-KHR = Kambodžský riel
currency-KMF = Komorský frank
currency-KPW = Severokorejský won
currency-KRW = Jihokorejský won
currency-KWD = Kuvajtský dinár
currency-KYD = Kajmanský dolar
currency-KZT = Kazašské tenge
currency-LAK = Laoský kip
currency-LBP = Libanonská libra
currency-LKR = Srílanská rupie
currency-LRD = Liberijský dolar
currency-LSL = Lesothský loti
currency-LYD = Libyjský dinár
currency-MAD = Marocký dinár
currency-MDL = Moldavský leu
currency-MGA = Madagaskarský ariary
currency-MKD = Makedonský denár
currency-MMK = Myanmarský kyat
currency-MNT = Mongolský tugrik
currency-MOP = Macajská pataca
currency-MRU = Mauritánská ouguiya
currency-MUR = Mauricijská rupie
currency-MVR = Maledivská rupie
currency-MWK = Malawijská kwacha
currency-MXN = Mexické peso
currency-MYR = Malajsijský ringgit
currency-MZN = Mozambický metical
currency-NAD = Namibijský dolar
currency-NGN = Nigerijská naira
currency-NIO = Nikaragujská córdoba
currency-NOK = Norská koruna
currency-NPR = Nepálská rupie
currency-NZD = Novozélandský dolar
currency-OMR = Ománský rijál
currency-PAB = Panamská balboa
currency-PEN = Peruánský sol
currency-PGK = Papuánská nová kina
currency-PHP = Filipínské peso
currency-PKR = Pákistánská rupie
currency-PLN = Polský zlotý
currency-PYG = Paraguajské guarani
currency-QAR = Katarský rijál
currency-RON = Rumunský leu
currency-RSD = Srbský dinár
currency-RUB = Ruský rubl
currency-RWF = Rwandský frank
currency-SAR = Saúdský rijál
currency-SBD = Šalamounský dolar
currency-SCR = Seychelská rupie
currency-SDG = Súdánská libra
currency-SEK = Švédská koruna
currency-SGD = Singapurský dolar
currency-SHP = Svatohelenská libra
currency-SLE = Sierraleonský leone
currency-SOS = Somálský šilink
currency-SRD = Surinamský dolar
currency-SSP = Jihosúdánská libra
currency-STN = Svatotomášská dobra
currency-SVC = Salvadorský colón
currency-SYP = Syrská libra
currency-SZL = Svazijský lilangeni
currency-THB = Thajský baht
currency-TJS = Tádžické somoni
currency-TMT = Turkmenský manat
currency-TND = Tuniský dinár
currency-TOP = Tonžská paanga
currency-TRY = Turecká lira
currency-TTD = Trinidadský dolar
currency-TWD = Tchajwanský dolar
currency-TZS = Tanzanský šilink
currency-UAH = Ukrajinská hřivna
currency-UGX = Ugandský šilink
currency-USD = Americký dolar
currency-UYU = Uruguayské peso
currency-UZS = Uzbecký sum
currency-VES = Venezuelský bolívar
currency-VND = Vietnamský dong
currency-VUV = Vanuatský vatu
currency-WST = Samojská tala
currency-XAF = CFA/BEAC frank
currency-XCD = Východokaribský dolar
currency-XCG = Caribbean Guilder
currency-XOF = CFA/BCEAO frank
currency-XPF = CFP frank
currency-YER = Jemenský rijál
currency-ZAR = Jihoafrický rand
currency-ZMW = Zambijská kwacha
currency-ZWG = Zimbabwe Gold
