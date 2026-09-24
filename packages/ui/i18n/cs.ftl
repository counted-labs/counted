# Čeština. Kompletní s výjimkou právních textů (legal-, terms-, privacy-), které existují jen anglicky
# a francouzsky a u každé zprávy zvlášť se vracejí k en.ftl.

### Common

loading = Načítání…
cancel = Zrušit
confirm = Potvrdit
retry = Zkusit znovu
delete = Smazat
back = Zpět
language = Jazyk

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
error-generic = Něco se pokazilo. Zkus to znovu.

error-invalid-email = Tato e-mailová adresa není platná.
error-invalid-password = Toto heslo není platné.
error-password-too-short = Heslo musí mít alespoň 8 znaků.
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
login-resend = Poslat ověřovací odkaz znovu
login-resending = Odesílání…
login-resend-sent = E-mail odeslán - zkontroluj schránku.

register-submit = Vytvořit účet
register-submitting = Vytváření…
register-have-account = Už máš účet?
register-password-placeholder = Alespoň 8 znaků
register-password-warning = Heslo si zapiš. Pokud ho zapomeneš, účet nelze obnovit.
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
settings-about = O aplikaci
settings-anonymous-title = Nejsi přihlášen
settings-upsell-title = Tvoje projekty na každém zařízení
settings-upsell-free = Zdarma
settings-upsell-body = Counted funguje i bez účtu. S bezplatným účtem tě tvoje projekty a předvolby následují na telefon, notebook i web - stále šifrované, stále pro nás nečitelné.
settings-locked-badge = Účet
settings-locked-friends = Vytvoř si účet, abys mohl přidávat přátele a zvát je do projektu přímo z aplikace - bez posílání odkazu.
settings-locked-payment-methods = Ulož si jednou svůj IBAN nebo platební aplikaci a sdílej je s vybranými projekty. Kdo ti dluží, uvidí je vedle tvého jména.
settings-friends-hint = Přidávej přátele a zvi je do projektů bez sdílení odkazu.

account-member-since = Členem od
account-logout = Odhlásit se
account-logging-out = Odhlašování…
account-delete-title = Smazat můj účet
account-delete-warning = Okamžité a trvalé, bez koše. Výdaje, které jsi zadal ve sdíleném projektu, zůstanou ostatním členům viditelné - jsou součástí jejich účtů.
account-delete-confirm-title = Smazat účet
account-delete-confirm-message = Tvůj účet, relace a seznam projektů budou trvale smazány. Bez hesla se pro tebe šifrovaná data sdíleného projektu stanou nečitelnými - to nelze vrátit zpět.

settings-payment-methods = Platební údaje
settings-payment-methods-hint = Jak chceš dostávat peníze zpět. Šifrováno s tvým účtem.
payment-method-kind = Způsob
payment-method-kind-other = Jiný
payment-method-label = Název
payment-method-label-placeholder = Hlavní účet
payment-method-value = Údaje
payment-method-value-placeholder = IBAN, telefonní číslo, uživatelské jméno…
payment-method-add = Přidat
payment-method-remove = Odebrat { $name }
payment-method-empty = Zatím jsi nepřidal žádné platební údaje.
payment-method-deleted = Platební metoda smazána.
payment-method-value-required = Vyplň údaje každé platební metody, nebo ji odeber.
payment-method-label-required = Pojmenuj svou vlastní metodu.
payment-method-too-long = To je příliš dlouhé - zkrať to.
payment-method-invalid-characters = Odstraň zalomení řádků nebo neviditelné znaky.
payment-method-limit = Můžeš uložit až { $max } platebních metod.
payment-methods-saved = Platební údaje uloženy.
payment-methods-offline = Pro uložení platebních údajů musíš být online.
payment-methods-stale = Tvoje platební údaje byly změněny na jiném zařízení. Byly znovu načteny — zkus to prosím znovu.
payment-methods-key-missing = Pro správu platebních údajů se znovu přihlas.
settings-payment-methods-share-warning = Sdílená metoda je viditelná pro každého člena projektů, kde sis vybral své jméno - pro každého, kdo má jeden z těch odkazů.
payment-method-share = Sdílet s mými projekty
payment-method-share-hint = Zobrazí se vedle tvého jména, když ti někdo dluží.
payment-method-copy = Kopírovat { $name }
payment-method-copied = Zkopírováno.
payment-method-copy-failed = Kopírování selhalo - označ text a zkopíruj ho ručně.

verify-email-checking = Ověřování e-mailové adresy…
verify-email-welcome = E-mail ověřen - vítej v Counted!
verify-email-back-to-login = Zpět na přihlášení

### Project status

project-close = Uzavřít
project-archive = Archivovat
project-reopen = Znovu otevřít
project-unarchive = Obnovit z archivu

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
field-optional = Nepovinné

### Projects

projects-filter-active = Aktivní
projects-filter-all = Vše
projects-count-label = Projekty
projects-empty = Žádné projekty
projects-empty-hint = Vytvoř projekt tlačítkem níže
projects-offline-banner = Offline data - pro obnovení se znovu připoj.
projects-no-local-data = Žádná místní data
projects-no-local-data-hint = Přihlas se a načti své projekty poprvé.
projects-add = Přidat projekt
projects-create = Vytvořit projekt
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
add-project-remove-participant = Odebrat účastníka
add-project-me-badge = Já
add-project-thats-me = To jsem já!
add-project-offline = Offline nelze projekt vytvořit. Znovu se připoj a zkus to znovu.
add-project-name-required = Projekt potřebuje název.
add-project-need-two-participants = Přidej alespoň 2 účastníky.
add-project-pick-yourself = Řekni nám, který účastník jsi ty.

join-link-label = Odkaz pro sdílení
join-link-hint = Odkaz obsahuje dešifrovací klíč - zkopíruj ho celý.
join-invalid-link = Tento odkaz není platný. Vlož celý odkaz pro sdílení včetně části za #.
join-wrong-project = Tento odkaz patří jinému projektu.

import-tricount-link-label = Odkaz nebo klíč Tricount
import-tricount-key-required = Zadej odkaz nebo klíč Tricount.
import-tricount-encryption-failed = Šifrování selhalo.

### Expenses

save = Uložit
saving = Ukládání…
adding = Přidávání…
link-copied = Odkaz zkopírován
missing-encryption-key = Chybí šifrovací klíč.
missing-encryption-key-title = Chybí šifrovací klíč
missing-encryption-key-hint = Použitý odkaz neobsahuje klíč potřebný k dešifrování tohoto projektu. Použij úplný odkaz od toho, kdo projekt vytvořil.
project-locked-hint = Toto zařízení nemá klíč k tomuto projektu. Pro odemknutí otevři jeho odkaz pro sdílení.
project-unlock = Odemknout
project-no-local-data-hint = Přihlas se a načti data tohoto projektu poprvé.
project-gone-title = Tento projekt už neexistuje
project-gone-hint = Byl smazán, když ho opustil poslední člen. Odkaz pro sdílení už nefunguje, ani když ho znovu otevřeš.

expense-add = Přidat výdaj
transfer-add = Přidat převod
expense-edit-title = Upravit výdaj
expense-category = Kategorie
expense-category-auto = Auto · { $emoji }
expense-currency = Měna částky
amount-op-add = Plus
amount-op-subtract = Minus
amount-op-multiply = Krát
amount-op-divide = Děleno
amount-op-equals = Rovná se
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
participants-deselect-all = Zrušit výběr
participants-by-shares = Podle podílů
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
reimbursements-empty-title = Vše vyrovnáno!
reimbursements-empty-hint = Návrhy na vyrovnání se zde objeví, když účty nesedí
reimbursement-owes = { $debtor } dluží { $creditor }
reimbursement-record = Vyrovnat
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
edit-project-deferred-me = výběr „To jsem já“
edit-project-offline-deferred = Offline: { $items } se použije po opětovném připojení.

export-saved = Soubor uložen:
    { $path }
export-failed = Export selhal: { $reason }

history-expense-added = Výdaj přidán: { $name }
history-expense-edited = Výdaj upraven: { $name }
history-expense-deleted = Výdaj smazán: { $name }
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
expense-inconsistent-detail = Částky nesouhlasí: zaplaceno { $paid }, dluženo { $owed }, u výdaje { $total }. Uprav výdaj a oprav to.
missing-access-key = Chybí přístupový klíč. Otevři tento projekt přes jeho odkaz pro sdílení.
filter-all = Vše
filter-my-payments = Moje platby
filter-my-debts = Co dlužím
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
charts-tab-per-person = Na osobu
charts-tab-trends = Trendy
charts-by-category = Rozdělení podle kategorií
charts-per-person = Výdaje na osobu
charts-categories-by-month = Kategorie po měsících
charts-total-spent = Celkem utraceno
charts-avg-per-person = Prům. na osobu
charts-expense-count =
    { $count ->
        [one] { $count } výdaj
        [few] { $count } výdaje
        [many] { $count } výdaje
       *[other] { $count } výdajů
    }
charts-clear-category-filter = Zrušit filtr kategorie
charts-no-expenses = Žádné výdaje.
charts-pick-a-project = Vyber projekt a uvidíš výdaje na osobu.
charts-nothing-to-show = Není co zobrazit
charts-my-share-note = Tyto částky jsou tvůj podíl na každém výdaji.
charts-my-share-skipped =
    { $count ->
        [one] 1 projekt se nepočítá — nebyl vybrán účastník, nebo se nepodařilo načíst jeho data.
        [few] { $count } projekty se nepočítají — nebyl vybrán účastník, nebo se nepodařilo načíst jejich data.
        [many] { $count } projektu se nepočítá — nebyl vybrán účastník, nebo se nepodařilo načíst jejich data.
       *[other] { $count } projektů se nepočítá — nebyl vybrán účastník, nebo se nepodařilo načíst jejich data.
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
charts-person = Osoba
charts-project = Projekt
charts-all-projects = Všechny projekty
charts-whole-project = Celý projekt
charts-date-from = Od
charts-date-to = Do
charts-total = Celkem
charts-payments-per-person-by-month = Platby na osobu po měsících
history-empty = Žádné události
history-by = { $name }
not-found-hint = Tato stránka neexistuje, nebo se přesunula.
payers-title-paid-by = Zaplatil(a)
payers-title-sender = Odesílatel
payers-title-contributors = Přispěvatelé
debtors-title-debtors = Dluží
debtors-title-recipients = Příjemci
debtors-title-beneficiaries = Příjemci

### Welcome

welcome-title = Do tvých účtů nikomu nic není.
welcome-subtitle = Děl se o výdaje s přáteli.
welcome-e2ee-title = Vše šifrované
welcome-e2ee-body = Jména, částky, projekty: vše se šifruje na tvém zařízení. Klíč máš jen ty. Nikdo nemůže číst tvoje vyúčtování. Ani my.
welcome-e2ee-note = Nečitelné i pro nás (nulový přístup serveru)
welcome-eu-title = 100% evropské
welcome-eu-body = Servery v Německu, e-maily odesílané z Francie. Tvoje data nikdy neopustí Evropskou unii.
welcome-noads-title = Žádné reklamy. Žádné sledování.
welcome-noads-body = Nic nesbíráme a tvoje data neprodáváme. To není náš model.
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
help-forgot-password-warning = Tvoje data budou nenávratně ztracena.
help-forgot-password-a = Šifrovací klíč se odvozuje z hesla, takže reset není možný: bez něj nikdo - ani my - tvoje projekty nedešifruje. Uchovej ho v bezpečí, ideálně ve správci hesel.
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
project-currency-hint = Každá částka se zobrazuje v této měně. Později ji nelze změnit.
project-currency-locked = Měna se nastavuje při vytvoření projektu.

update-required-title = Vyžadována aktualizace
update-required-body = Tato verze Counted je příliš stará na komunikaci se serverem. Aktualizuj ji, abys mohl aplikaci dál používat.
update-required-body-testflight = Tato verze Counted je příliš stará na komunikaci se serverem. Otevři TestFlight a nainstaluj nejnovější sestavení, abys mohl aplikaci dál používat.
update-required-button = Aktualizovat

notifications-label = Oznámení
notifications-title = Oznámení
notifications-empty = Nic nového
notifications-friend-request = Žádost o přátelství

friends-title = Přátelé
friends-anonymous-body = Přátelé jsou vázáni na tvůj účet. Přihlas se, abys mohl přidávat lidi a zvát je do projektů bez sdílení odkazu.
friends-add-title = Přidat přítele
friends-add-hint = Žádost uvidí po přihlášení. Ani jeden z vás se nedozví, zda má druhý účet, dokud není žádost přijata.
friends-add-button = Přidat
friends-add-from-project = Přidat jako přítele
friends-request-sent = Žádost odeslána
friends-no-account-key = Pro správu přátel na tomto zařízení se znovu přihlas.
friends-incoming-title = Žádosti
friends-accept = Přijmout
friends-decline = Odmítnout
friends-list-title = Moji přátelé
friends-list-empty = Zatím žádní přátelé. Přidej někoho e-mailem výše, nebo ze sdíleného projektu.
friends-remove = Odebrat
friends-remove-confirm-title = Odebrat přítele
friends-remove-confirm-message = { $email } už nebude mezi vašimi přáteli a vy mezi jeho. Kdokoli z vás může později poslat novou žádost.
friends-no-key = Zatím není připraveno
friends-fingerprint = Bezpečnostní kód
friends-fingerprint-hint = Dva přátelé, kteří si přečtou stejný bezpečnostní kód, vědí, že mezi nimi nikdo nesedí - ani náš server.
friends-outgoing-title = Odeslané
friends-outgoing-hint = Čeká na odpověď. Po přijetí se objeví mezi tvými přáteli.
friends-withdraw = Zrušit
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
