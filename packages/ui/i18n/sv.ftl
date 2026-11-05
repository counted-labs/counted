# Svenska. Komplett förutom de juridiska texterna (legal-, terms-, privacy-), som bara finns på
# engelska och franska och faller tillbaka på en.ftl meddelande för meddelande.

### Common

loading = Laddar…
cancel = Avbryt
confirm = Bekräfta
retry = Försök igen
delete = Ta bort
back = Tillbaka
language = Språk
theme = Tema
theme-light = Ljust
theme-dark = Mörkt
theme-system = System
theme-hint-device = Sparas bara på den här enheten.
theme-hint-system = Följer enhetens utseende.

### Navigation

nav-main = Huvudnavigering
nav-projects = Projekt
nav-charts = Statistik
nav-settings = Inställningar

### Connectivity

offline-banner = Offline
offline-pending =
    { $count ->
        [one] { $count } väntar
       *[other] { $count } väntar
    }

sync-conflict-edit = Konflikt: redigeringen av ”{ $name }” misslyckades (objektet borttaget). Hoppades över.
sync-conflict-delete = Konflikt: borttagningen av ”{ $name }” misslyckades (objektet borttaget). Hoppades över.
sync-conflict-other = Konflikt: åtgärden på ”{ $name }” misslyckades (objektet borttaget). Hoppades över.
sync-error = Synkfel: { $reason }

### Errors

error-network = Det går inte att nå servern. Kontrollera din internetanslutning.
error-rate-limited = Servern är upptagen. Försök igen om en stund.
error-generic = Något gick fel. Försök igen.

error-invalid-email = Den e-postadressen är inte giltig.
error-invalid-password = Det lösenordet är inte giltigt.
error-password-too-short = Lösenordet måste vara minst 12 tecken.
error-client-outdated = Den här versionen av appen är föråldrad. Uppdatera den för att logga in.
error-invalid-link = Den här länken är inte giltig.
error-batch-too-large = För många objekt på en gång.
error-payers-required = Välj minst en betalare.
error-debtors-required = Välj minst en person som är skyldig.
error-duplicate-participant = En deltagare förekommer två gånger på samma sida.
error-participant-not-in-project = Den deltagaren är inte med i projektet.
error-too-many-participants = För många deltagare för en utgift.
error-invalid-credentials = Fel e-post eller lösenord.
error-unauthenticated = Logga in för att göra det.
error-email-not-verified = Din e-postadress är inte verifierad ännu.
error-project-not-found = Det här projektet finns inte längre.
error-expense-not-found = Den här utgiften finns inte längre.
error-storage-full = Lagringsutrymmet är fullt: nyckeln till det här projektet kunde inte sparas på den här enheten. Spara delningslänken.
error-user-not-found = Den här deltagaren finns inte längre.
error-tricount-not-found = Tricount hittades inte, eller så gav dess API ett fel.
error-too-many-members = Projektet har nått sin gräns för medlemmar.
error-identity-taken = Ett annat konto har redan gjort anspråk på den här deltagaren.
error-claim-proof-invalid = Den här enheten har inte projektnyckeln och kan därför inte göra anspråk på en deltagare. Öppna delningslänken igen.
error-user-has-payments = Den här deltagaren har utgifter i projektet och kan inte tas bort.
error-resend-cooldown = Vänta 60 sekunder innan du begär ett nytt mejl.
error-self-friend-request = Du kan inte lägga till dig själv som vän.
error-not-a-friend = Du kan bara bjuda in personer från din vänlista.
error-friend-has-no-key = Den här vännen har inte öppnat den senaste versionen av appen ännu. Be hen logga in en gång och försök sedan igen.
error-friend-request-not-found = Den här vänförfrågan finns inte längre.
error-invitation-not-found = Den här inbjudan finns inte längre.
error-too-many-friend-requests = För många vänförfrågningar just nu. Försök igen imorgon.
error-too-many-invitations = För många väntande inbjudningar.
error-invalid-kdf-salt = Krypteringsinställningarna är ogiltiga. Uppdatera appen och försök igen.
error-mixed-project-batch = De här deltagarna tillhör inte samma projekt.
error-invalid-payload = Den här versionen av appen skickade data som servern inte accepterar. Uppdatera den och försök igen.
error-invalid-public-key = Din krypteringsnyckel är ogiltig. Uppdatera appen och försök igen.
error-payment-methods-stale = Dina betaluppgifter ändrades på en annan enhet. Ladda om och försök igen.

### Auth

field-email = E-post
field-email-placeholder = du@exempel.se
field-password = Lösenord
field-name = Namn
field-name-placeholder = Anna Andersson

login-title = Logga in
login-submit = Logga in
login-submitting = Loggar in…
login-password-placeholder = Ditt lösenord
login-no-account = Har du inget konto än?
login-unverified = Din e-postadress är inte verifierad ännu. Kolla din inkorg, eller skicka länken igen.
login-maybe-unverified = Har du precis registrerat dig? Verifiera först din adress med länken vi skickade, eller skicka den igen.
login-resend = Skicka verifieringslänken igen
login-resending = Skickar…
login-resend-sent = Mejl skickat - kolla din inkorg.

register-submit = Skapa ett konto
register-submitting = Skapar…
register-have-account = Har du redan ett konto?
register-password-placeholder = Minst 12 tecken
register-password-warning = Skriv ner ditt lösenord: vi ser det aldrig och kan inte ge dig det igen.
register-check-email-title = Kolla din e-post
register-email-sent = Mejl skickat
register-email-sent-hint = Klicka på länken i din inkorg för att aktivera ditt konto.
register-not-received-prefix = Fick du inget? Kolla skräpposten, eller
register-sign-in-link = logga in
register-not-received-suffix = för att skicka länken igen.
register-terms-prefix = Genom att skapa ett konto godkänner du våra
register-terms-link = användarvillkor
register-terms-and = och vår
register-privacy-link = integritetspolicy

settings-title = Inställningar
settings-preferences = Inställningar
settings-preferences-local = Sparade på den här enheten.
settings-preferences-synced = Synkade med ditt konto, krypterade.
settings-about = Om
settings-anonymous-title = Du är inte inloggad
settings-upsell-title = Dina projekt, på alla enheter
settings-upsell-free = Gratis
settings-upsell-body = Counted fungerar utan konto. Med ett gratis konto följer dina projekt och inställningar med till din mobil, din dator och webben - fortfarande krypterade, fortfarande oläsbara för oss.
settings-locked-badge = Konto
settings-locked-friends = Skapa ett konto för att lägga till vänner och bjuda in dem till ett projekt från appen - ingen länk att skicka runt.
settings-locked-payment-methods = Spara ditt IBAN eller din betalapp en gång och dela med de projekt du väljer. Den som är skyldig dig ser det bredvid ditt namn.
settings-friends-hint = Lägg till vänner och bjud in dem till dina projekt utan att dela en länk.

account-member-since = Medlem sedan
account-logout = Logga ut
account-logging-out = Loggar ut…
account-delete-title = Ta bort mitt konto
account-delete-warning = Omedelbar och permanent radering av ditt konto och dina uppgifter.
account-delete-confirm-title = Ta bort konto
account-delete-confirm-message = Ditt konto, dina sessioner och din projektlista tas bort permanent. Utan ditt lösenord blir den krypterade datan i ett delat projekt oläsbar för dig - det går inte att ångra.
erase-device-open = Logga ut och rensa den här enheten
erase-device-title = Rensa den här enheten?
erase-device-body = Projekten, deras nycklar och cachen på den här enheten raderas, sedan avslutas din session. Dina projekt finns kvar i ditt konto: logga in igen för att få tillbaka dem.
erase-device-checking = Kontrollerar att dina projekt finns i ditt konto…
erase-device-pending = Ändringar som väntar på att skickas: { $count }. Anslut till internet igen och försök sedan på nytt.
erase-device-unreachable = Kunde inte kontrollera att dina projekt finns i ditt konto. Kontrollera din anslutning och försök sedan på nytt.
erase-device-at-risk = De här projekten finns inte i ditt konto och går förlorade. Vill du behålla dem, avbryt och kopiera först deras länk:
erase-device-confirm = Rensa och logga ut
erase-device-confirm-anyway = Rensa ändå
erase-device-erasing = Rensar…

settings-payment-methods = Betalningsuppgifter
settings-payment-methods-hint = Hur du vill få tillbaka pengar. Krypterat med ditt konto.
payment-method-kind = Metod
payment-method-kind-other = Annat
payment-method-label = Namn
payment-method-label-placeholder = Huvudkonto
payment-method-value = Uppgifter
payment-method-value-placeholder = IBAN, telefonnummer, användarnamn…
payment-method-add = Lägg till
payment-method-remove = Ta bort { $name }
payment-method-empty = Du har inte lagt till några betalningsuppgifter än.
payment-method-deleted = Betalningsmetod borttagen.
payment-method-value-required = Fyll i uppgifterna för varje betalningsmetod, eller ta bort den.
payment-method-label-required = Ge din egen metod ett namn.
payment-method-too-long = Det är för långt - korta ner det.
payment-method-invalid-characters = Ta bort radbrytningar eller osynliga tecken.
payment-method-limit = Du kan spara upp till { $max } betalningsmetoder.
payment-methods-saved = Betalningsuppgifter sparade.
payment-methods-offline = Du måste vara online för att spara dina betalningsuppgifter.
payment-methods-stale = Dina betalningsuppgifter ändrades på en annan enhet. De har laddats om - försök igen.
payment-methods-key-missing = Logga in igen för att hantera dina betalningsuppgifter.
settings-payment-methods-share-warning = En delad metod är synlig för alla medlemmar i dina projekt.
payment-method-share = Dela med mina projekt
payment-method-share-hint = Visas bredvid ditt namn när någon är skyldig dig pengar.
payment-method-copy = Kopiera { $name }
payment-method-copied = Kopierat.
payment-method-copy-failed = Det gick inte att kopiera - markera texten och kopiera den för hand.

verify-email-checking = Verifierar din e-postadress…
verify-email-welcome = E-post verifierad - välkommen till Counted!
verify-email-back-to-login = Tillbaka till inloggning
verify-email-intro = Ange ditt lösenord för att bekräfta att adressen är din.
verify-email-submit = Verifiera min adress

### Project status

project-archive = Arkivera
project-sheet-invite = Bjud in
project-sheet-recurring = Återkommande
project-sheet-edit = Redigera projekt
project-sheet-close = Stäng projekt
project-sheet-archive = Arkivera projekt
project-sheet-reopen = Öppna projekt igen
project-sheet-unarchive = Återställ projekt från arkiv
project-sheet-leave = Lämna projekt

### Dates

date-long = { $day } { $month } { $year }

month-1 = januari
month-2 = februari
month-3 = mars
month-4 = april
month-5 = maj
month-6 = juni
month-7 = juli
month-8 = augusti
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
month-short-8 = aug
month-short-9 = sep
month-short-10 = okt
month-short-11 = nov
month-short-12 = dec

### Actions

add = Lägg till
create = Skapa
creating = Skapar…
edit = Redigera
leave = Lämna
close = Stäng
paste = Klistra in
join = Gå med
import = Importera
importing = Importerar…
field-description = Beskrivning
field-date = Datum
date-today = I dag
date-yesterday = I går
field-optional = Valfritt

### Projects

projects-filter-active = Aktiva
projects-filter-all = Alla
projects-count-label = Projekt
projects-empty = Inga projekt
projects-offline-banner = Offlinedata - anslut igen för att uppdatera.
demo-banner = Demoprojekt - skrivskyddat.
demo-start-own = Starta ett eget projekt
demo-badge = Demo
demo-bar = Dina ändringar försvinner när du stänger den här fliken.
demo-leave = Kom igång på riktigt
demo-preparing = Förbereder din demo…
demo-failed = Demon kunde inte förberedas.
demo-sign-in-blocked = Inloggning är avstängd i demon. Lämna den för att logga in eller skapa ett konto; dina egna projekt är orörda.
projects-no-local-data = Ingen lokal data
projects-no-local-data-hint = Logga in för att ladda dina projekt för första gången.
projects-add = Lägg till ett projekt
projects-create = Skapa ett projekt
projects-personal-expense = Personlig utgift
personal-expense-banner = Tillagd i Mina utgifter
personal-create-title = Skapa ditt personliga projekt
personal-create-hint = Håll koll på vad du spenderar själv
group-create-title = Skapa ett delat projekt
group-create-hint = Dela utgifter med vänner och familj
personal-create-action = Skapa
personal-create-signup = Skapa ett konto
personal-sheet-title = Ditt personliga projekt
personal-sheet-intro = För det du spenderar själv. Det visas med dina grupper i Statistik.
personal-sheet-point-alone = En medlem: du, med ditt kontonamn. Ingen delningslänk.
personal-sheet-point-encrypted = Krypterat på dina enheter, som dina andra projekt.
personal-sheet-point-stats = Dina andelar i grupper läggs till dessa utgifter i Statistik.
personal-default-name = Mina utgifter
my-spending-prev-month = Föregående månad
my-spending-next-month = Nästa månad
my-spending-spent = Du har spenderat
my-spending-groups = Grupper
my-spending-income = Inkomster
my-spending-balance = Månadens saldo
my-spending-unconverted = Andelar i en annan valuta lämnas utanför tills en kurs är känd.
my-spending-empty = Inget den här månaden
my-spending-empty-hint = Lägg till en utgift med +. Dina andelar i grupper visas här automatiskt.
my-spending-share-of = av { $amount }
my-spending-uncounted = räknas inte
payer-you = dig
expense-received-by = mottagen av
counted-switch = Räkna med i Mina utgifter
counted-switch-on = På: din andel läggs till Mina utgifter
counted-switch-off = Av: din andel visas inte i Mina utgifter
counted-groups = Grupper som räknas
counted-groups-intro = Din andel i dessa grupper läggs till Mina utgifter och budgetarna.
counted-groups-converted = { $from }, omräknat till { $to }
counted-groups-encrypted = Valet är krypterat med ditt konto och följer dina enheter.
counted-groups-empty = Inga grupper på den här enheten ännu.
counted-groups-done = OK
counted-save-failed = Valet kunde inte sparas. Försök igen.
tab-budget = Budget
budget-title = Budgetar
budget-manage = Hantera
budget-left = { $amount } kvar
budget-over = Överskriden med { $amount }
budget-empty = Ingen budget
budget-empty-hint = Sätt ett tak per kategori. Dina andelar i grupper räknas också.
budget-define = Sätt en budget
budget-sheet-title = Månadsbudgetar
budget-sheet-intro = På din andel: personliga och grupper. Lämna tomt för att inte följa en kategori.
budget-average = Snitt senaste 3 månaderna: { $amount }
budget-for = Budget { $category }
budget-alert = Varna mig vid 80 %
budget-alert-hint = Beräknas på den här enheten
budget-invalid = Ogiltigt belopp för { $category }.
budget-offline = Budgetar kan bara sparas online. Anslut igen och försök på nytt.
budget-alert-near = { $category }: { $percent } % av månadens budget
budget-alert-over = { $category }: månadens budget är överskriden
personal-card-budget = { $month } · budget { $amount }
my-spending-search-summary =
    { $count ->
        [one] { $count } resultat
       *[other] { $count } resultat
    } · { $total } · personliga och grupper, alla månader
projects-join = Gå med i ett projekt
projects-import-tricount = Importera från Tricount
project-actions = Projektåtgärder

status-ongoing = Pågående
status-closed = Avslutat
status-archived = Arkiverat

nav-help = Hjälp
nav-privacy = Integritetspolicy
nav-terms = Användarvillkor
nav-legal = Juridisk information

leave-project-title = Lämna projektet?
leave-project-message = Du förlorar åtkomsten från den här enheten. Om ingen medlem finns kvar tas projektet och alla dess utgifter bort permanent.

add-project-title = Nytt projekt
add-project-name-label = Projektnamn
add-project-name-placeholder = Min resa, Kollektivet 2024…
add-project-participants = Deltagare
add-project-participant-name = Deltagarens namn
add-project-participant-placeholder = Clark Kent
add-project-offline = Du kan inte skapa ett projekt offline. Anslut igen och försök på nytt.
add-project-name-required = Projektet behöver ett namn.

join-link-label = Delningslänk
join-link-hint = Länken innehåller dekrypteringsnyckeln - kopiera hela.
join-invalid-link = Den länken är inte giltig. Klistra in hela delningslänken, inklusive delen efter #.
join-wrong-project = Den länken gäller ett annat projekt.

import-tricount-link-label = Tricount-länk eller -nyckel
import-tricount-key-required = Ange en Tricount-länk eller -nyckel.
import-tricount-encryption-failed = Krypteringen misslyckades.
import-tricount-unimportable = Inget importerades: den här Tricounten har medlemmar med Tricount-konto eller belopp som inte går ihop (berörda poster: { $count }).

### Expenses

save = Spara
saving = Sparar…
adding = Lägger till…
link-copied = Länk kopierad
missing-encryption-key = Krypteringsnyckel saknas.
missing-encryption-key-title = Krypteringsnyckel saknas
missing-encryption-key-hint = Länken du använde innehåller inte nyckeln som behövs för att dekryptera projektet. Använd hela länken som den som skapade det delade.
project-locked-hint = Den här enheten har inte projektets nyckel. Öppna dess delningslänk för att låsa upp det.
expenses-unreadable = Vissa poster kan inte läsas på den här enheten och räknas inte med i summorna. Uppdatera appen för att se dem.
project-unlock = Lås upp
project-no-local-data-hint = Logga in för att ladda projektets data för första gången.
project-gone-title = Det här projektet finns inte längre
project-gone-hint = Det togs bort när den sista medlemmen lämnade. Delningslänken fungerar inte längre, även om du öppnar den igen.

expense-add = Lägg till en utgift
transfer-add = Lägg till en överföring
expense-edit-title = Redigera utgiften
expense-category = Kategori
expense-category-auto = Auto · { $emoji }
emoji-picker-label = Emoji
emoji-none = Ingen emoji
expense-currency = Beloppets valuta
amount-op-add = Plus
amount-op-subtract = Minus
amount-op-multiply = Gånger
amount-op-divide = Delat med
amount-op-equals = Lika med
amount-op-done = Klar
expense-rate = Växelkurs (valfritt)
expense-rate-hint = Lämna tomt för att använda Europeiska kommissionens (InforEuro) kurs för { $month }: 1 { $from } = { $rate } { $to }.
expense-rate-invalid = Ange en växelkurs större än 0.
expense-rate-unavailable = Ingen automatisk kurs tillgänglig - ange en för hand.
expense-delete-title = Ta bort utgiften
expense-delete-message = ”{ $name }” tas bort permanent. Det går inte att ångra.
expense-inconsistent-amounts = Beloppen stämmer inte
expenses-empty = Inga utgifter
expenses-empty-hint = Börja med att lägga till utgifter med knappen nedan
expenses-show-more = Visa fler ({ $count } kvar)

expense-type-expense = Utgift
expense-type-transfer = Överföring
expense-type-gain = Inkomst
expense-type-group = Typ
expense-paid-by = betald av
expense-sent-by = skickad av
expense-contributed-by = bidrag från

expense-name-required = Ett namn krävs.
expense-amount-not-positive = Beloppet måste vara större än 0.
expense-no-payer = Välj minst en betalare.
expense-no-debtor = Välj minst en person som är skyldig.
expense-invalid-date = Det datumet är inte giltigt.
expense-payers-mismatch = Betalarna summerar till { $sum }, vilket inte stämmer med utgiftens belopp ({ $total }).
expense-debtors-mismatch = De skyldiga summerar till { $sum }, vilket inte stämmer med utgiftens belopp ({ $total }).

participants-none = Ingen
participants-everyone = Alla ({ $count })
participants-some = { $count } av { $total }
participants-select-all = Markera alla
participants-by-shares = Efter andelar
split-amounts = Belopp
participants-remaining = { $amount } kvar
participants-over-by = { $amount } för mycket
participants-who-paid = Vem betalade?
participants-who-received = Vem tog emot?
participants-who-transfers = Vem för över?
participants-who-receives = Vem tar emot?
participants-for-whom = För vem?

stats-total-expenses = Totala utgifter
stats-my-expenses = Mina utgifter

tab-expenses = Utgifter
tab-balance = Saldo
tab-reimbursements = Gör upp
balance-gets-back = Får tillbaka
balance-owes = Är skyldig
balance-settled = Kvitt
reimbursements-empty-title = Allt är uppgjort!
reimbursements-empty-hint = Förslag på uppgörelser visas här när räkenskaperna inte går ihop
reimbursement-owes = { $debtor } är skyldig { $creditor }
reimbursement-record = Gör upp
reimbursement-pay-with = Betala
reimbursement-pay-shared-by = Delat av { $name } - kontrollera mottagarnamnet som din app visar innan du skickar.
reimbursement-pay-title = Betala { $name }
reimbursements-mine-title = Du är skyldig
reimbursements-others-title = Andra återbetalningar
copy = Kopiera

user-selection-title = Vilken deltagare är du?
user-selection-hint = Välj ditt namn i listan.
user-selection-required = Välj en deltagare.
identity-claimed = Kopplad till ett konto
identity-claimed-by = { $name }s konto
identity-taken-repick = Ett annat konto har gjort anspråk på deltagaren du använde. Välj en annan.
participant-gone-repick = Deltagaren du använde har tagits bort från projektet. Välj en annan.

edit-project-title = Redigera projektet
edit-project-new-badge = ny
edit-project-deferred-new-members = tillägg av nya medlemmar
edit-project-deferred-removals = borttagning av medlemmar
edit-project-offline-deferred = Offline: { $items } tillämpas när du ansluter igen.

export-failed = Exporten misslyckades: { $reason }

history-expense-added = Utgift tillagd: { $name }
history-expense-edited = Utgift redigerad: { $name }
history-expense-deleted = Utgift borttagen: { $name }
move-expense = Flytta till ett projekt
move-expense-title = Flytta till…
move-current = Nuvarande
move-converted = { $currency }, omräknat till månadens kurs
move-note-personal = Du väljer vem som betalade och för vem i nästa steg. Utgiften lämnar Mina utgifter.
move-note-group = Utgiften lämnar { $project } för alla medlemmar, och gruppens saldo räknas om. Du väljer vem som betalade och för vem i nästa steg.
move-banner = Från { $from } till { $to }
move-submit = Flytta
move-done = Utgift flyttad till { $project }
move-delete-failed = Utgiften lades till i det andra projektet men finns kvar i { $project }: radera den där.
history-expense-moved = Utgift flyttad till ett annat projekt: { $name }
personal-badge = Personlig
move-members =
    { $count ->
        [one] { $count } medlem
       *[other] { $count } medlemmar
    }
history-project-edited = Projekt redigerat: { $name }
history-name-changed = Namn: ”{ $from }” → ”{ $to }”
history-description-added = Beskrivning tillagd: ”{ $value }”
history-description-removed = Beskrivning borttagen: ”{ $value }”
history-description-changed = Beskrivning: ”{ $from }” → ”{ $to }”

### Sweep

field-amount = Belopp
expense-name-placeholder = Restaurang, matvaror…
expense-actions = Utgiftsåtgärder
expense-your-share = Din andel
expense-your-share-value = Din andel: { $amount } { $currency }
expense-share-percent = { $percent } %
expense-added-by = Tillagd av { $name } den { $date }
expense-inconsistent-detail = Beloppen stämmer inte: { $paid } betalt, { $owed } skyldigt, för en utgift på { $total }. Redigera utgiften för att rätta till det.
missing-access-key = Åtkomstnyckel saknas. Öppna projektet via dess delningslänk.
filter-all = Alla
filter-my-payments = Mina betalningar
filter-my-debts = Vad jag är skyldig
expense-filter-group = Filter
expenses-search = Sök utgifter
expenses-no-match = Inga utgifter matchar.
expenses-search-open = Sök
expenses-search-clear = Rensa sökning
expenses-search-summary =
    { $count ->
        [one] { $count } resultat
       *[other] { $count } resultat
    } · { $total } · alla månader
participants-shares-for = Andelar för { $name }
participants-amount-for = Belopp för { $name }
reimbursement-add = Lägg till en uppgörelse
project-forget = Ta bort från min lista
project-history-title = Historik
history-kind-add = Tillagd
history-kind-delete = Borttagen
history-kind-edit = Redigerad
export = Exportera
export-json = Exportera JSON
export-csv = Exportera CSV
share-link = Dela
copy-link-failed = Det gick inte att kopiera länken
open-in-app = Öppna i appen
not-found-title = Sidan hittades inte
not-found-back = Tillbaka till projekten

### Charts

charts-period = Period
period-all = Allt
period-month = Månad
period-3months = 3 mån
period-year = År
period-custom = Anpassad
charts-tab-categories = Kategorier
charts-tab-trends = Trender
charts-total-spent = Totalt spenderat
charts-avg-per-person = Snitt per person
charts-expense-count =
    { $count ->
        [one] { $count } utgift
       *[other] { $count } utgifter
    }
charts-nothing-to-show = Inget att visa
charts-my-share-skipped =
    { $count ->
        [one] 1 projekt räknas inte med - ingen deltagare vald, eller så laddades inte dess data.
       *[other] { $count } projekt räknas inte med - ingen deltagare vald, eller så laddades inte deras data.
    }

### Categories

category-food = Mat
category-transport = Transport
category-accommodation = Boende
category-leisure = Fritid
category-shopping = Shopping
category-services = Tjänster
category-parties-gifts = Fester & presenter
category-other = Annat
categories-manage = Hantera kategorier
categories-title = Kategorier
categories-yours = Dina kategorier
categories-builtin = Standard
categories-add = Lägg till en kategori
categories-in-parent = i { $parent }
categories-renamed-from = omdöpt · ursprungligen { $name }
categories-hidden-hint = dold · utgifterna hamnar i { $other }
categories-always-shown = visas alltid
categories-hidden = Dold
categories-new = Ny kategori
categories-edit = Redigera kategori
categories-parent = Räknas i
categories-parent-hint = I statistiken för alla dina projekt räknas { $name } i { $parent }.
categories-create = Skapa
categories-hide = Dölj den här kategorin
categories-hide-hint = Dess utgifter visas i { $other } för alla medlemmar.
categories-delete-title = Ta bort ”{ $name }”?
categories-delete-body = De berörda utgifterna ({ $count }) går tillbaka till { $parent }. Ingen utgift tas bort.
categories-delete-body-unknown = Dess utgifter går tillbaka till { $parent }. Ingen utgift tas bort.
categories-delete-warning = Kategorin försvinner för alla medlemmar i projektet.
categories-summary-created = Skapade: { $count }
categories-summary-renamed = Omdöpta: { $count }
categories-summary-hidden = Dolda: { $count }
categories-summary-default = Inga egna kategorier
categories-offline = Kategorier kan bara hanteras online. Anslut igen och försök på nytt.
categories-error-too-many = Ett projekt har högst 20 egna kategorier.
categories-error-full = Projektet har nått sin gräns för kategorier.
categories-error-name = Ge kategorin ett namn.
categories-error-missing = Kategorin har ändrats under tiden. Stäng och försök igen.
history-category-created = Kategori skapad: { $name }
history-category-edited = Kategori redigerad: { $name }
history-category-deleted = Kategori borttagen: { $name }
charts-project = Projekt
charts-all-projects = Alla projekt
charts-date-from = Från
charts-date-to = Till
charts-total = Totalt
charts-tab-people = Personer
charts-tab-projects = Projekt
charts-scope = Vems utgifter
charts-scope-group = Grupp
charts-scope-me = Jag
charts-currency = Valuta
charts-my-share = Min andel
charts-share-of-total = { $pct } % av { $total }
charts-i-paid = Jag betalade
charts-paid-more = { $amount } mer än min andel
charts-paid-less = { $amount } mindre än min andel
charts-paid-even = Exakt min andel
charts-part-title = Din del av varje kategori
charts-part-desc = Grått är vad gruppen spenderade, färg är vad du konsumerade.
charts-breakdown-title = Fördelning per kategori
charts-breakdown-desc = Tryck på en sektor eller rad för att se utgifterna.
charts-of-total = { $amount } av { $total }
charts-show-all = Visa alla ({ $count })
charts-show-less = Visa färre
charts-spend-title = Utgifter över tid
charts-group-by = Gruppera per
bucket-day = Dag
bucket-week = Vecka
bucket-month = Månad
charts-avg = snitt
charts-cat-title-day = { $category }, dag för dag
charts-cat-title-week = { $category }, vecka för vecka
charts-cat-title-month = { $category }, månad för månad
charts-cat-desc = Välj en kategori för att följa den över tid.
charts-running-title = Löpande summa
charts-running-desc = Sedan { $date }.
charts-avg-per-day = { $amount } / dag i snitt
charts-avg-per-week = { $amount } / vecka i snitt
charts-avg-per-month = { $amount } / månad i snitt
charts-people-title = Vem bar gruppen
charts-people-desc = Vad varje person betalade, bredvid vad de konsumerade.
charts-paid = Betalt
charts-fair-share = Rättvis andel
charts-you = (du)
charts-net-more = betalade mer
charts-net-less = betalade mindre
charts-balance-title = Ditt saldo över tid
charts-balance-desc = Ovanför linjen är gruppen skyldig dig. Under den är du skyldig gruppen.
charts-owed = Du ska få
charts-owe = Du är skyldig
charts-projects-title = Din andel per projekt
charts-projects-desc = Summor hålls per valuta.
history-empty = Inga händelser
history-by = Av { $name }
not-found-hint = Den här sidan finns inte, eller har flyttats.
payers-title-paid-by = Betald av
payers-title-sender = Avsändare
payers-title-contributors = Bidragsgivare
debtors-title-debtors = Skyldiga
debtors-title-recipients = Mottagare
debtors-title-beneficiaries = Förmånstagare
debtors-title-for = För
debtors-title-recipient = Mottagare
expense-split-between = delat mellan { $count }
expense-people =
    { $count ->
        [one] { $count } person
       *[other] { $count } personer
    }

### Welcome

welcome-title = Din bokföring angår ingen annan.
welcome-subtitle = Dela utgifter med vänner.
welcome-note = Gratis. Inget konto behövs. Inga annonser.
welcome-link-title = En länk, och alla är med.
welcome-link-body = Ingen behöver skapa ett konto.
welcome-link-account = Ett konto? Aldrig ett krav. Det används för att hitta dina projekt på en annan enhet, bjuda in vänner från appen och dela dina betalningsuppgifter.
welcome-demo-project = Helg i Lyon
welcome-private-title = Ingen kan läsa dina räkenskaper. Inte ens vi.
welcome-private-body = Namn, belopp, projekt: allt krypteras på din enhet. Bara du har nyckeln.
welcome-private-names = Namn
welcome-private-amounts = Belopp
welcome-private-projects = Projekt
welcome-scan-title = Fota kvittot.
welcome-scan-body = Belopp, datum och kategori fylls i av sig själva. Allt sker i din telefon. Fotot sparas inte.
welcome-eu-title = 100 % europeiskt
welcome-no-ads = Inga annonser
welcome-no-trackers = Inga spårare
welcome-step = Steg { $current } av { $total }
welcome-next = Nästa
welcome-skip = Hoppa över
welcome-start = Kom igång
welcome-how-it-works = Hur fungerar det, egentligen?

### Help

help-intro = En vanlig fråga? Tryck för att fälla ut svaret.
help-create-project-q = Hur skapar jag ett projekt?
help-create-project-a = Från startskärmen, tryck på +-knappen längst ner. Ge projektet ett namn, välj valuta, så är du klar.
help-add-participants-q = Hur lägger jag till deltagare?
help-add-participants-a = Öppna projektet och lägg till deltagare från medlemslistan. Varje deltagare kan betala för eller vara skyldig för en utgift.
help-share-project-q = Hur delar jag ett projekt?
help-share-project-a = Dela projektets URL (den i adressfältet). Alla med länken kan se och redigera projektet.
help-add-expense-q = Hur lägger jag till en utgift?
help-add-expense-a = I ett projekt, tryck på +, ange beloppet, vem som betalade och vilka det ska delas mellan. Du kan också välja ett annat datum än idag.
help-types-q = Vad är skillnaden mellan utgift, överföring och inkomst?
help-types-expense = - ett köp gjort av en person och delat mellan flera.
help-types-transfer = - en återbetalning från en person till en annan, utan delning.
help-types-gain = - pengar som tagits emot (en återbäring, en gåva) att dela mellan flera personer.
help-past-date-q = Kan jag datera en utgift bakåt i tiden?
help-past-date-a = Ja, datumfältet är fritt. Postens skapelsetid sparas separat.
help-who-owes-q = Hur räknar Counted ut vem som är skyldig vad?
help-who-owes-a = Counted beräknar varje deltagares nettosaldo (vad hen lagt ut minus vad hen är skyldig) och föreslår sedan den kortaste serien överföringar som gör upp för alla.
help-minimal-transfers-q = Varför är antalet föreslagna överföringar minimalt?
help-minimal-transfers-a = Algoritmen parar först ihop saldon som tar ut varandra exakt, och går sedan igenom resten från största fordringsägare till största gäldenär. Resultatet: färre överföringar för att göra upp allt.
help-import-tricount-q = Hur importerar jag ett projekt från Tricount?
help-import-tricount-a = Från startskärmen, tryck på ”+”-knappen längst ner, sedan
help-import-tricount-b = Klistra in delningslänken för det Tricount du vill importera.
help-encryption-q = Är min data krypterad?
help-encryption-a = Ja. Counted kombinerar två garantier:
help-encryption-e2ee-term = Totalsträckskryptering
help-encryption-e2ee-def = - allt mellan dig och servern skickas krypterat.
help-encryption-zero-term = Noll åtkomst
help-encryption-zero-def = - du krypterar datan innan den skickas, och servern lagrar bara chiffertext. Vi har inget sätt att läsa den.
help-encryption-see = För detaljer, se
help-forgot-password-q = Vad händer om jag glömmer mitt lösenord?
help-forgot-password-warning = Ingen kan ta fram det åt dig.
help-forgot-password-a = Ditt lösenord skyddar dina krypteringsnycklar och skickas aldrig till oss: ingen - inte ens vi - kan dekryptera dina projekt åt dig. Förvara det säkert, helst i en lösenordshanterare.
help-archive-delete-q = Hur arkiverar eller tar jag bort ett projekt?
help-archive-delete-a = Från projektskärmen, öppna menyn och välj
help-archive-delete-b = för att dölja det men behålla det. Ett projekt tas bort för gott när dess sista medlem lämnar det.
help-delete-account-q = Hur tar jag bort mitt konto?
help-delete-account-a = Öppna Inställningar och använd ”Ta bort mitt konto”. Det sker omedelbart och kan inte ångras.
help-contact = En annan fråga? Skriv till oss på

# Receipt scanning (mobile only)
expense-scan = Skanna ett kvitto
scan-in-progress = Läser kvittot…
scan-error-capture = Det gick inte att ta bilden. Försök igen, eller ange utgiften för hand.
scan-error-unreadable = Inget läsbart på det kvittot. Ange utgiften för hand.
scan-check-amount = Kontrollera totalen - den var inte tydligt tryckt.
scan-take-photo = Ta en bild
scan-choose-photo = Välj en bild
expense-converted-from = Betalt { $amount } { $from } · 1 { $from } = { $rate } { $to }
project-currency = Valuta
project-currency-locked = Valutan låses när projektet skapas.
currency-search = Sök valuta

update-required-title = Uppdatering krävs
update-required-body = Den här versionen av Counted är för gammal för att prata med servern. Uppdatera den för att fortsätta använda appen.
update-required-button = Uppdatera

notifications-label = Aviseringar
notifications-title = Aviseringar
notifications-empty = Inget nytt
notifications-friend-request = Vänförfrågan

friends-title = Vänner
friends-anonymous-body = Vänner sparas med ditt konto. Logga in för att lägga till personer och bjuda in dem till dina projekt utan att dela en länk.
friends-add-title = Lägg till en vän
friends-add-hint = Har personen ett konto ser hen din förfrågan i appen. Annars, säg till: hen hittar den när hen registrerar sig med den här adressen.
friends-add-button = Lägg till
friends-add-from-project = Lägg till som vän
friends-request-sent = Förfrågan skickad
friends-no-account-key = Logga in igen för att hantera dina vänner på den här enheten.
friends-incoming-title = Förfrågningar
friends-accept = Acceptera
friends-decline = Avböj
friends-list-title = Mina vänner
friends-list-empty = Inga vänner ännu. Lägg till någon via e-post ovan, eller från ett projekt ni delar.
friends-remove = Ta bort
friends-remove-confirm-title = Ta bort vän
friends-remove-confirm-message = { $email } finns inte längre bland dina vänner, och du inte bland deras. Vem som helst av er kan skicka en ny förfrågan senare.
friends-no-key = Inte redo ännu
friends-key-changed = Koden ändrad
friends-key-changed-hint = En väns säkerhetskod har ändrats. Det händer när vännen säkrar sitt konto, eller om nyckeln har bytts ut. Inbjudningar med vännen är blockerade tills ni läser upp den nya koden för varandra, personligen eller i telefon, och du trycker på ”Koden stämmer”.
friends-fingerprint = Säkerhetskod
friends-fingerprint-hint = Två vänner som läser upp samma säkerhetskod för varandra vet att ingen sitter emellan dem - inte ens vår server.
friends-key-mismatch = Servern skickade en offentlig nyckel för ditt konto som inte passar din privata nyckel. Din säkerhetskod är dold och inbjudningar är avstängda på den här enheten: någon kanske manipulerar servern.
keypair-mismatch = Nycklarna som servern skickade för ditt konto stämmer inte med dem på den här enheten. Inbjudningar och nyckelbyten är avstängda här tills de stämmer: någon kanske manipulerar servern.
friends-outgoing-title = Skickade
friends-outgoing-hint = Väntar på svar. Du ser dem bland dina vänner när de accepterat.
friends-withdraw = Avbryt
friends-notify = Säg till med ett meddelande
friends-notify-label = Säg till { $email }
friends-notify-message = Jag börjar använda Counted för våra gemensamma utgifter. Gratis registrering på https://counted.fr med adressen { $email }: min vänförfrågan väntar där.
friends-notify-copied = Meddelandet kopierat
friends-notify-copy-failed = Det gick inte att kopiera meddelandet
guests-hint-title =
    { $count ->
        [one] { $count } deltagare har inte gått med i projektet än
       *[other] { $count } deltagare har inte gått med i projektet än
    }
guests-hint-later = Senare
guests-hint-send = Bjud in dem
guests-send-title = Skicka projektet till deltagarna
guests-send-friend = Bjud in en Counted-vän
guests-send-friend-badge = Utan länk
guests-send-friend-hint = Har hen redan ett konto får hen en krypterad inbjudan; inget går via din meddelandeapp.
guests-send-link = Dela projektlänken
guests-send-link-hint = För dem utan konto. Ett tryck öppnar projektet.
guests-send-link-warning = Alla som har den här länken kan läsa och ändra projektet. Din meddelandeapp ser den också.
invite-friends-title = Bjud in vänner
invite-friends-hint = Projektnyckeln krypteras för varje vän på den här enheten. Servern ser den aldrig.
invite-friends-empty = Inga vänner att bjuda in ännu.
invite-friends-button = Bjud in
invite-sent = { $count ->
    [one] Inbjudan skickad
   *[other] { $count } inbjudningar skickade
}
invitation-badge = Inbjudan
invitation-to = Gå med i ”{ $name }”
invitation-to-unnamed = Gå med i ett projekt
invitation-unreadable = Den här inbjudan kan inte öppnas på den här enheten
invitation-from = Från { $email }
invitation-accept = Gå med
invitation-decline = Avböj

# Participants in the create and edit modals, and the "who are you?" picker - see
# docs/plans/friends.md §11.
participants-you-label = Ditt namn i det här projektet
participants-you-badge = Du
participants-others = Andra deltagare
participants-removed = Borttagna
participant-restore = Återställ
participants-empty = Lägg till deltagare nedan.
participants-duplicate = ”{ $name }” finns redan i listan.
participants-input-label = Lägg till en vän eller skriv ett namn
participants-input-placeholder = Vän eller valfritt namn
participants-suggest-friend = Vän · går med som ”{ $name }”, får en inbjudan
participants-suggest-not-ready = Vän · inte redo än
participants-suggest-guest = Lägg till ”{ $text }”
participants-suggest-guest-sub = Gäst
participants-friends = Dina vänner
participants-all-friends = Alla vänner
participants-login-hint = Logga in för att lägga till personer direkt från din vänlista.
participants-invite-badge = Bjud in
participants-guest-badge = Gäst
participants-rename = Byt namn på { $name }
participants-remove = Ta bort { $name }
participants-rename-label = Nytt namn
participants-rename-save = Spara namnet
participants-rename-hint = Namnet som alla ser i det här projektet. Inbjudan går fortfarande till { $email }.
participants-invited-badge = Inbjuden
participants-invited-sub = { $email } · inte accepterad än
participants-invited-pending = Inbjudan inte accepterad än
participants-unlinked = Inte kopplad till något konto
add-project-create-invite = Skapa och bjud in { $count }
edit-project-save-invite = Spara och bjud in { $count }
edit-project-you-are = På den här enheten är du { $name }
edit-project-no-identity = Du har inte valt vem du är än
edit-project-switch = Byt
edit-project-choose = Välj
invite-failed = De här inbjudningarna kunde inte skickas: { $emails }
invite-again = Bjud in igen
friend-picker-title = Lägg till vänner
user-selection-invited-hint = { $email } har bjudit in dig till ”{ $project }”.
user-selection-suggested = Föreslagen
user-selection-suggested-sub = { $email } lade till dig med det här namnet
user-selection-confirm-as = Jag är { $name }
user-selection-missing = Finns inte ditt namn? Be en deltagare lägga till dig i projektets inställningar.

## Återkommande utgifter

repeat-label = Upprepa
repeat-none = Upprepas inte
repeat-weekly = Varje vecka
repeat-biweekly = Varannan vecka
repeat-monthly = Varje månad
repeat-quarterly = Var tredje månad
repeat-yearly = Varje år
repeat-every-weeks = Var { $count }:e vecka
repeat-every-months = Var { $count }:e månad
repeat-every-years = Vart { $count }:e år
repeat-custom = Anpassad…
repeat-every = Var
repeat-unit-weeks = veckor
repeat-unit-months = månader
repeat-unit-years = år
repeat-on-weekday = på { $weekday }
repeat-on-day = den { $day }:e
repeat-on-day-month = den { $day } { $month }
repeat-month-end = I kortare månader hamnar den på sista dagen.
repeat-ends = Slutar
repeat-ends-never = Aldrig
repeat-ends-on = Ett datum
repeat-ends-after = Efter
repeat-fewer = Färre
repeat-more = Fler
repeat-last-on = sista den { $date }
repeat-variable = Beloppet ändras varje gång
repeat-variable-hint = Var och en läggs till med det senaste beloppet och markeras ”att bekräfta”.
repeat-done = Klar
repeat-no-end = Inget slut
repeat-until = Till { $date }
repeat-occurrences = Antal gånger: { $count }
repeat-offline = Kräver en anslutning. Själva utgiften kan ändå läggas till.
repeat-foreign = Upprepas som { $amount } { $currency }, omräknat en gång till dagens kurs. Du varnas om kursen ändras mer än 5 %.
repeat-backfill = Börjar i det förflutna. Utgifter som läggs till nu: { $count }.
add-and-repeat = Lägg till och upprepa
weekday-1 = måndag
weekday-2 = tisdag
weekday-3 = onsdag
weekday-4 = torsdag
weekday-5 = fredag
weekday-6 = lördag
weekday-7 = söndag
recurring-title = Återkommande utgifter
recurring-strip = Återkommande utgifter: { $count }
recurring-next = Nästa: { $name }, { $date }
recurring-to-confirm = Att bekräfta: { $count }
recurring-per-month = Per månad, ungefär
recurring-your-share = Din andel
recurring-active = Aktiva
recurring-paused = Pausade
recurring-finished = Avslutade
recurring-paid-by = betald av { $name }
recurring-next-on = Nästa den { $date }
recurring-progress = { $done } av { $total }
recurring-rate-badge = Kursen ändrad { $percent } %
recurring-empty = Inget upprepas ännu. Välj ”Upprepa” när du lägger till en utgift: hyra, abonnemang, räkningar.
recurring-next-ones = Kommande
recurring-added-so-far = Tillagda hittills
recurring-set-up-by = Skapad av
recurring-pause = Pausa
recurring-resume = Återuppta
recurring-stop = Sluta upprepa
recurring-stop-title = Sluta upprepa ”{ $name }”?
recurring-stop-message = Den upprepas inte längre. Redan tillagda utgifter finns kvar.
recurring-resume-title = Återuppta ”{ $name }”?
recurring-resume-message = Nästa den { $date }. Datum som missades under pausen läggs inte till.
recurring-edit-title = Redigera återkommande utgift
recurring-edit-banner = Ändringarna gäller från { $date }. Redan tillagda utgifter förblir som de är.
recurring-next-on-label = Nästa den
recurring-next-too-early = Nästa datum måste ligga efter den senast tillagda utgiften.
recurring-use-stop = Använd ”Sluta upprepa” på den återkommande utgiften för att avsluta den.
recurring-drift = Kursen för { $currency } har ändrats { $percent } % sedan den skapades. Var och en läggs fortfarande till som { $amount } { $project_currency } (1 { $currency } = { $rate }). Till dagens kurs skulle det bli { $today_amount } { $project_currency }.
recurring-use-rate = Använd dagens kurs
recurring-keep = Behåll { $amount } { $currency }
recurring-added = Återkommande utgifter tillagda: { $count }
recurring-blocks-removal = { $name } kan inte tas bort än: ingår i { $rules }. Ta bort { $name } därifrån eller avsluta dem, och spara igen.
history-recurring-added = Tillagd automatiskt: { $name } ({ $date })
history-recurring-created = Återkommande utgift skapad: { $name }
history-recurring-edited = Återkommande utgift redigerad: { $name }
history-recurring-paused = Återkommande utgift pausad: { $name }
history-recurring-resumed = Återkommande utgift återupptagen: { $name }
history-recurring-stopped = Återkommande utgift stoppad: { $name }
occurrence-recurring = Återkommande utgift
occurrence-auto = Tillagd automatiskt av en återkommande utgift.
occurrence-auto-next = Tillagd automatiskt av en återkommande utgift. Nästa den { $date }.
occurrence-auto-stopped = Tillagd automatiskt av en återkommande utgift som sedan har avslutats.
occurrence-manage = Hantera
estimate-badge = Att bekräfta
expense-pending = Väntar
estimate-title = Belopp att bekräfta.
estimate-body = Tillagd med det förra beloppet. Ange det riktiga när det kommer.
estimate-confirm = Bekräfta belopp
apply-to = Gäller
apply-this-only = Bara den här utgiften
apply-and-next = Den här och de kommande
apply-and-next-hint = Den återkommande utgiften ändras från { $date }
apply-rule-failed = Utgiften sparades, men den återkommande utgiften ändrades inte.
occurrence-delete-message = ”{ $name }” från { $date } raderas permanent. Den fortsätter att upprepas och det här datumet kommer inte tillbaka.
occurrence-delete-one = Radera bara den här
occurrence-delete-stop = Radera och sluta upprepa
error-recurring-clock = Enhetens klocka går före. Kontrollera datum och tid.
error-recurring-not-found = Den återkommande utgiften finns inte längre.
error-recurring-stale = Någon har ändrat den återkommande utgiften under tiden. Den har laddats om: kontrollera den och spara igen.
error-too-many-recurring = Projektet har redan 50 återkommande utgifter. Avsluta en du inte längre behöver för att lägga till en ny.
error-user-in-recurring = Deltagaren ingår i en återkommande utgift. Ta bort deltagaren därifrån eller avsluta utgiften först.

home-owed-to-you = Du har att få
home-you-owe = Du är skyldig
home-details-show = Visa detaljer
home-details-hide = Dölj detaljer
home-all-settled = Allt är reglerat
home-pay = Betala
home-remind = Påminn
home-remind-message = Liten påminnelse: det finns fortfarande återbetalningar kvar i { $project }.
home-row-owed = du har att få
home-row-owe = du är skyldig
home-row-settled = Reglerat
home-totals-partial = Utan projekt utan växelkurs
error-account-key-changed = Nyckeln till ditt konto ändrades på en annan enhet. Logga in igen för att fortsätta.
settings-security = Säkerhet
settings-change-password = Byt lösenord
settings-change-password-hint = Dina projekt och data förblir som de är. Dina andra enheter loggas ut.
change-password-title = Byt ditt lösenord
change-password-current = Nuvarande lösenord
change-password-new = Nytt lösenord
change-password-confirm = Bekräfta det nya lösenordet
change-password-mismatch = De två nya lösenorden stämmer inte överens.
change-password-submit = Byt lösenord
change-password-submitting = Byter…
change-password-done = Lösenordet är bytt. Dina andra enheter har loggats ut.
error-rotation-incomplete = Ditt konto ändrades under tiden. Försök igen.
error-key-lifecycle-unavailable = Inte tillgängligt än: först behöver alla den senaste versionen av appen.
settings-compromised = Någon kan känna till mitt lösenord
settings-compromised-hint = Nytt lösenord, nya nycklar och alla andra enheter utloggade.
settings-compromised-open = Säkra mitt konto
compromised-title = Säkra ditt konto
compromised-body = Ditt lösenord och ditt kontos nycklar byts ut, och alla andra enheter loggas ut. Dina vänner ser att din säkerhetskod ändras: jämför den med dem igen.
compromised-keeps = Den som hade ditt lösenord behåller det som redan öppnats: projekten som kunde läsas hittills.
compromised-submit = Byt lösenord och nycklar
compromised-done = Ditt konto är säkrat. Be dina vänner jämföra din nya säkerhetskod.
friends-new-fingerprint = Ny kod:
friends-code-matches = Koden stämmer
friends-code-trusted = Koden är bekräftad.
settings-recovery-key = Återställningsnyckel
settings-recovery-key-none = Utan den kostar ett glömt lösenord dina inställningar, betalningsuppgifter och vänners koder, och det tar 3 dagar.
settings-recovery-key-set = Skapad. Glömmer du lösenordet får du tillbaka kontot med allt innehåll.
settings-recovery-key-pending = En ny nyckel ersätter den { $when }. Var det inte du? Avbryt med din nuvarande nyckel.
recovery-key-create = Skapa
recovery-key-replace = Ersätt
recovery-key-cancel-replacement = Avbryt den nya nyckeln
recovery-key-title = Din återställningsnyckel
recovery-key-body = Skriv ner den eller spara den där bara du kommer åt den. Counted kan inte visa den igen eller ta fram den åt dig.
recovery-key-copy = Kopiera
recovery-key-copied = Kopierad
recovery-key-save = Spara som fil
recovery-key-print = Skriv ut
recovery-key-next = Fortsätt
recovery-key-confirm-body = Skriv grupp { $n } av nyckeln för att kontrollera din kopia.
recovery-key-confirm-wrong = Gruppen stämmer inte. Titta på din kopia igen.
recovery-key-password-body = Ange ditt lösenord för att spara nyckeln på ditt konto.
recovery-key-submit = Spara nyckeln
recovery-key-done = Återställningsnyckeln är sparad.
recovery-key-replacement-done = Ny nyckel sparad. Den ersätter den nuvarande om 72 timmar; tills dess fungerar den nuvarande.
recovery-key-field = Återställningsnyckel
recovery-key-file-intro = Återställningsnyckel för Counted
recovery-key-cancel-done = Den nya nyckeln är avbruten.
compromised-new-key = Spara din nya återställningsnyckel nu: den ersätter den gamla om 72 timmar, och tills dess fungerar den gamla.
pending-reset-title = En återställning av ditt konto har begärts
pending-reset-body = Den träder i kraft { $when }: då försvinner dina inställningar, betalningsuppgifter och vänners koder, och dina enheter måste logga in igen. Var det inte du? Avbryt med ditt nuvarande lösenord.
pending-reset-cancel = Avbryt återställningen
pending-reset-cancelled = Återställningen är avbruten.
login-forgot = Glömt lösenordet?
forgot-title = Glömt lösenordet
forgot-body = Ange din e-post. Om den har ett konto skickar vi en länk för att välja ett nytt lösenord.
forgot-submit = Skicka länken
forgot-sent = Om adressen har ett konto är en länk på väg. Den går ut om 24 timmar.
reset-title = Få tillbaka ditt konto
reset-with-key-body = Ange din återställningsnyckel och ett nytt lösenord: allt kommer tillbaka som det var.
reset-submit-key = Återställ mitt konto
reset-no-key = Jag har inte min återställningsnyckel
reset-without-key-title = Återställ utan återställningsnyckeln
reset-without-key-body = Ditt konto börjar om med ett nytt lösenord om 72 timmar. Dina projekt kommer tillbaka från dina enheter och delningslänkar; dina inställningar, betalningsuppgifter och vänners koder gör det inte. Tills dess fungerar ditt nuvarande lösenord och kan avbryta återställningen.
reset-name = Ditt namn
reset-submit-request = Återställ mitt konto
reset-requested = Klart. Ditt nya lösenord fungerar från { $when }. Tills dess fungerar ditt nuvarande lösenord och kan avbryta det.
reset-pending-already = En återställning väntar redan till { $when }. En ny begäran startar om de 72 timmarna.
recovered-done = Ditt konto är tillbaka. Dina andra enheter har loggats ut.
login-reset-undoable = Det här lösenordet ersattes av en återställning de senaste 30 dagarna. Bad du inte om det, ångra det: ditt konto kommer tillbaka som det var.
login-undo-reset = Ångra återställningen
reset-undone = Återställningen är ångrad. Ditt konto är som förut.
error-reset-pending = Det nya lösenordet fungerar när återställningen träder i kraft, 72 timmar efter begäran.
error-account-disabled = Kontot raderades strax efter en återställning och väntar på att tas bort. Det tidigare lösenordet kan fortfarande ångra återställningen.
error-recovery-key-invalid = Den här återställningsnyckeln öppnar inte kontot.
error-project-key-required = Projektet behöver sin nyckel. Öppna det igen från dess delningslänk.
names-or = { $first } eller { $last }
participants-admin-badge = Admin
edit-project-admins = Admins: { $count }
edit-project-succession = Lämnar den sista adminen projektet blir den medlem med konto som varit med längst admin.
member-sheet-account = Medlem med konto
member-actions = Åtgärder för { $name }
member-make-admin = Gör till admin
member-make-admin-hint = Kan då byta projektets nyckel och ta bort någon.
member-made-admin = { $name } är nu admin.
group-security = Projektsäkerhet
group-security-admin = Projektsäkerhet · du är admin
group-rotate = Byt projektets nyckel
group-rotate-hint = Om projektets länk har läckt: de som har den gamla länken förlorar åtkomsten till projektet.
group-remove = Ta bort någon från projektet
group-remove-hint = Tar bort åtkomsten och byter nyckeln samtidigt.
group-admins-only = Bara för admins.
group-admins-only-account = Bara för admins, som har ett konto.
group-ask-admin = Fråga { $names }, projektets admin.
group-ask-admins = Fråga { $names }, projektets admins.
group-ask-an-admin = Fråga någon av projektets admins.
group-with-account = Med konto
group-free = Gratis
group-sign-in-hint = En admin i projektet, { $names }, kan sedan ge dig rollen.
group-sign-in-hint-any = En admin i projektet kan sedan ge dig rollen.
group-no-admin = Projektet har ingen admin, så dess nyckel kan inte bytas.
group-hide = Dölj någon
group-hide-hint = Döljer personen i listorna utan att ta bort åtkomsten: länken och nyckeln fungerar fortfarande.
group-recreate-hint = Projektet har ingen admin. Skapa ett nytt projekt för att ta bort någon.
group-recreate = Skapa ett nytt projekt
group-no-admin-yet = Projektet har ingen admin än.
group-creator-hint = Bara du kan bli admin för det här projektet, genom att skapa ett konto.
group-creator-elsewhere-hint = Bara personen som skapade projektet kan bli dess admin, genom att skapa ett konto. Annars kan du skapa ett nytt projekt för att ta bort någon.
member-revoke-admin = Ta bort admin-rollen
member-revoke-admin-hint = Kan inte längre byta projektets nyckel eller ta bort någon.
member-revoked-admin = { $name } är inte längre admin.
member-step-down = Sluta vara admin
member-step-down-hint = Du kan inte längre byta projektets nyckel eller ta bort någon.
member-stepped-down = Du är inte längre admin.
member-remove = Ta bort från projektet
member-remove-hint = Byter projektets nyckel: { $name } kan inte längre läsa något nytt.
member-hide = Dölj i listorna
error-user-not-admin = Medlemmen är inte admin.
error-creator-proof-invalid = Den här enheten skapade inte projektet.
error-project-has-admin = Projektet har redan en admin.
error-not-project-admin = Bara projektets admins kan göra det.
error-user-not-claimed = Den här deltagaren har inget konto och kan därför inte vara admin.
error-project-key-rotated = Projektets nyckel har bytts. Be en medlem om den nya länken.
error-project-rotating = Projektets nyckel byts just nu. Försök igen om en stund.
error-rotation-expired = Nyckelbytet tog för lång tid och avbröts. Börja om.
error-rotation-rows-changed = Projektet ändrades under nyckelbytet. Börja om.
project-rotated-title = Du har inte längre åtkomst till det här projektet
project-rotated-body = Nyckeln till det här projektet har bytts av en admin. Be hen om den nya länken.
project-rotated-open-link = Öppna en länk
rotation-remove-title = Ta bort { $name } från projektet?
rotation-rotate-title = Byta projektets nyckel?
rotation-remove-body = Att ta bort en medlem innebär att en ny nyckel skapas. Den nya länken måste delas med dem som inte har något konto.
rotation-rotate-body = De som har den gamla länken förlorar åtkomsten till projektet. Den nya länken måste delas med dem som inte har något konto.
rotation-boxed =
    { $count ->
        [0] Ingen annan medlem får nyckeln automatiskt
        [one] { $count } medlem får nyckeln automatiskt
       *[other] { $count } medlemmar får nyckeln automatiskt
    }
rotation-by-link-title = Personer du behöver skicka den nya länken till
rotation-by-link-hint = De här personerna har inget konto. Du skickar länken till dem direkt efteråt.
rotation-names-and = { $first } och { $last }
rotation-names-and-others = { $names } och { $count } till
rotation-confirm-remove = Ta bort och byt nyckel
rotation-confirm-rotate = Byt nyckel
rotation-done-title = Ny nyckel på plats
rotation-done-removed = { $name } är inte längre med i projektet.
rotation-done-rotated = Projektets nyckel har bytts.
rotation-done-boxed =
    { $count ->
        [0] Ingen annan medlem fick nyckeln automatiskt.
        [one] { $count } medlem fick nyckeln automatiskt.
       *[other] { $count } medlemmar fick nyckeln automatiskt.
    }
rotation-new-link =
    { $count ->
        [0] Ny länk till projektet
        [one] Ny länk att skicka · { $count } person utan konto
       *[other] Ny länk att skicka · { $count } personer utan konto
    }
rotation-link-renewed = Länken har förnyats.
rotation-done = Klar
history-key-changed = Projektets nyckel bytt
history-member-removed = { $name } är inte längre med i projektet
project-rotated-row-hint = Nyckeln har bytts. Be om den nya länken.
