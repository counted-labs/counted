# Polski. Kompletny z wyjątkiem tekstów prawnych (legal-, terms-, privacy-), które istnieją tylko po
# angielsku i francusku i dla każdego komunikatu z osobna wracają do en.ftl.

### Common

loading = Ładowanie…
cancel = Anuluj
confirm = Potwierdź
retry = Spróbuj ponownie
delete = Usuń
back = Wstecz
language = Język

### Navigation

nav-main = Nawigacja główna
nav-projects = Projekty
nav-charts = Statystyki
nav-settings = Ustawienia

### Connectivity

offline-banner = Offline
offline-pending =
    { $count ->
        [one] { $count } oczekująca
        [few] { $count } oczekujące
        [many] { $count } oczekujących
       *[other] { $count } oczekujących
    }

sync-conflict-edit = Konflikt: edycja „{ $name }” nie powiodła się (element usunięty). Pominięto.
sync-conflict-delete = Konflikt: usunięcie „{ $name }” nie powiodło się (element usunięty). Pominięto.
sync-conflict-other = Konflikt: operacja na „{ $name }” nie powiodła się (element usunięty). Pominięto.
sync-error = Błąd synchronizacji: { $reason }

### Errors

error-network = Nie można połączyć się z serwerem. Sprawdź połączenie z internetem.
error-generic = Coś poszło nie tak. Spróbuj ponownie.

error-invalid-email = Ten adres e-mail jest nieprawidłowy.
error-invalid-password = To hasło jest nieprawidłowe.
error-password-too-short = Hasło musi mieć co najmniej 8 znaków.
error-client-outdated = Ta wersja aplikacji jest nieaktualna. Zaktualizuj ją, aby się zalogować.
error-invalid-link = Ten link jest nieprawidłowy.
error-batch-too-large = Zbyt wiele elementów naraz.
error-payers-required = Wybierz co najmniej jedną osobę płacącą.
error-debtors-required = Wybierz co najmniej jedną osobę, która jest dłużna.
error-duplicate-participant = Uczestnik pojawia się dwa razy po tej samej stronie.
error-participant-not-in-project = Ten uczestnik nie należy do tego projektu.
error-too-many-participants = Zbyt wielu uczestników w jednym wydatku.
error-invalid-credentials = Nieprawidłowy e-mail lub hasło.
error-unauthenticated = Zaloguj się, aby to zrobić.
error-email-not-verified = Twój adres e-mail nie został jeszcze zweryfikowany.
error-project-not-found = Ten projekt już nie istnieje.
error-expense-not-found = Ten wydatek już nie istnieje.
error-user-not-found = Ten uczestnik już nie istnieje.
error-tricount-not-found = Nie znaleziono Tricounta lub jego API zwróciło błąd.
error-too-many-members = Ten projekt osiągnął limit członków.
error-identity-taken = Inne konto już przejęło tego uczestnika.
error-claim-proof-invalid = To urządzenie nie ma klucza projektu, więc nie może przejąć uczestnika. Otwórz ponownie link udostępniania.
error-user-has-payments = Ten uczestnik ma wydatki w projekcie i nie można go usunąć.
error-resend-cooldown = Odczekaj 60 sekund przed poproszeniem o kolejny e-mail.
error-self-friend-request = Nie możesz dodać siebie jako znajomego.
error-not-a-friend = Możesz zapraszać tylko osoby z listy znajomych.
error-friend-has-no-key = Ten znajomy nie otworzył jeszcze najnowszej wersji aplikacji. Poproś go, aby raz się zalogował, i spróbuj ponownie.
error-friend-request-not-found = To zaproszenie do znajomych już nie istnieje.
error-invitation-not-found = To zaproszenie już nie istnieje.
error-too-many-friend-requests = Na razie zbyt wiele zaproszeń do znajomych. Spróbuj jutro.
error-too-many-invitations = Zbyt wiele oczekujących zaproszeń.

### Auth

field-email = E-mail
field-email-placeholder = ty@przyklad.pl
field-password = Hasło
field-name = Imię
field-name-placeholder = Anna Kowalska

login-title = Zaloguj się
login-submit = Zaloguj się
login-submitting = Logowanie…
login-password-placeholder = Twoje hasło
login-no-account = Nie masz jeszcze konta?
login-unverified = Twój adres e-mail nie został jeszcze zweryfikowany. Sprawdź skrzynkę lub wyślij link ponownie.
login-resend = Wyślij link weryfikacyjny ponownie
login-resending = Wysyłanie…
login-resend-sent = E-mail wysłany - sprawdź skrzynkę.

register-submit = Utwórz konto
register-submitting = Tworzenie…
register-have-account = Masz już konto?
register-password-placeholder = Co najmniej 8 znaków
register-password-warning = Zapisz swoje hasło. Jeśli je zapomnisz, konta nie da się odzyskać.
register-check-email-title = Sprawdź pocztę
register-email-sent = E-mail wysłany
register-email-sent-hint = Kliknij link w wiadomości, aby aktywować konto.
register-not-received-prefix = Nie dotarł? Sprawdź folder spam lub
register-sign-in-link = zaloguj się
register-not-received-suffix = aby wysłać link ponownie.
register-terms-prefix = Tworząc konto, akceptujesz nasz
register-terms-link = regulamin
register-terms-and = i naszą
register-privacy-link = politykę prywatności

settings-title = Ustawienia
settings-preferences = Preferencje
settings-preferences-local = Zapisane na tym urządzeniu.
settings-preferences-synced = Zsynchronizowane z Twoim kontem, zaszyfrowane.
settings-about = O aplikacji
settings-anonymous-title = Nie jesteś zalogowany
settings-upsell-title = Twoje projekty na każdym urządzeniu
settings-upsell-free = Bezpłatnie
settings-upsell-body = Counted działa bez konta. Z bezpłatnym kontem Twoje projekty i preferencje podążają za Tobą na telefon, laptop i do przeglądarki - nadal zaszyfrowane, nadal nieczytelne dla nas.
settings-locked-badge = Konto
settings-locked-friends = Utwórz konto, aby dodawać znajomych i zapraszać ich do projektu z aplikacji - bez przekazywania linku.
settings-locked-payment-methods = Zapisz raz swój IBAN lub aplikację płatniczą i udostępnij je wybranym projektom. Kto jest Ci dłużny, zobaczy je obok Twojego imienia.
settings-friends-hint = Dodawaj znajomych i zapraszaj ich do projektów bez udostępniania linku.

account-member-since = Konto od
account-logout = Wyloguj się
account-logging-out = Wylogowywanie…
account-delete-title = Usuń moje konto
account-delete-warning = Natychmiastowe i nieodwracalne, bez kosza. Wydatki, które wpisałeś we wspólnym projekcie, pozostają widoczne dla pozostałych członków - są częścią ich rozliczeń.
account-delete-confirm-title = Usuń konto
account-delete-confirm-message = Twoje konto, sesje i lista projektów zostaną trwale usunięte. Bez Twojego hasła zaszyfrowane dane wspólnego projektu staną się dla Ciebie nieczytelne - tego nie da się cofnąć.

settings-payment-methods = Dane do płatności
settings-payment-methods-hint = Jak chcesz otrzymywać zwroty. Zaszyfrowane z Twoim kontem.
payment-method-kind = Metoda
payment-method-kind-other = Inna
payment-method-label = Nazwa
payment-method-label-placeholder = Konto główne
payment-method-value = Dane
payment-method-value-placeholder = IBAN, numer telefonu, nazwa użytkownika…
payment-method-add = Dodaj
payment-method-remove = Usuń { $name }
payment-method-empty = Nie dodano jeszcze żadnych danych do płatności.
payment-method-deleted = Metoda płatności usunięta.
payment-method-value-required = Uzupełnij dane każdej metody płatności lub ją usuń.
payment-method-label-required = Nadaj nazwę swojej własnej metodzie.
payment-method-too-long = To za długie - skróć.
payment-method-invalid-characters = Usuń znaki nowej linii lub niewidoczne znaki.
payment-method-limit = Możesz zapisać maksymalnie { $max } metod płatności.
payment-methods-saved = Dane do płatności zapisane.
payment-methods-offline = Aby zapisać dane do płatności, musisz być online.
payment-methods-stale = Twoje dane do płatności zostały zmienione na innym urządzeniu. Zostały wczytane ponownie — spróbuj jeszcze raz.
payment-methods-key-missing = Zaloguj się ponownie, aby zarządzać danymi do płatności.
settings-payment-methods-share-warning = Udostępniona metoda jest widoczna dla każdego członka projektów, w których wybrałeś swoje imię - dla każdego, kto ma jeden z tych linków.
payment-method-share = Udostępnij moim projektom
payment-method-share-hint = Pokazywane obok Twojego imienia, gdy ktoś jest Ci dłużny.
payment-method-copy = Kopiuj { $name }
payment-method-copied = Skopiowano.
payment-method-copy-failed = Nie udało się skopiować - zaznacz tekst i skopiuj go ręcznie.

verify-email-checking = Weryfikacja adresu e-mail…
verify-email-welcome = E-mail zweryfikowany - witaj w Counted!
verify-email-back-to-login = Wróć do logowania

### Project status

project-close = Zamknij
project-archive = Archiwizuj
project-reopen = Otwórz ponownie
project-unarchive = Przywróć z archiwum

### Dates

date-long = { $day } { $month } { $year }

month-1 = stycznia
month-2 = lutego
month-3 = marca
month-4 = kwietnia
month-5 = maja
month-6 = czerwca
month-7 = lipca
month-8 = sierpnia
month-9 = września
month-10 = października
month-11 = listopada
month-12 = grudnia

month-short-1 = sty
month-short-2 = lut
month-short-3 = mar
month-short-4 = kwi
month-short-5 = maj
month-short-6 = cze
month-short-7 = lip
month-short-8 = sie
month-short-9 = wrz
month-short-10 = paź
month-short-11 = lis
month-short-12 = gru

### Actions

add = Dodaj
create = Utwórz
creating = Tworzenie…
edit = Edytuj
leave = Opuść
close = Zamknij
paste = Wklej
join = Dołącz
import = Importuj
importing = Importowanie…
field-description = Opis
field-date = Data
date-today = Dziś
date-yesterday = Wczoraj
field-optional = Opcjonalnie

### Projects

projects-filter-active = Aktywne
projects-filter-all = Wszystkie
projects-count-label = Projekty
projects-empty = Brak projektów
projects-empty-hint = Utwórz projekt przyciskiem poniżej
projects-offline-banner = Dane offline - połącz się ponownie, aby odświeżyć.
projects-no-local-data = Brak danych lokalnych
projects-no-local-data-hint = Zaloguj się, aby po raz pierwszy wczytać swoje projekty.
projects-add = Dodaj projekt
projects-create = Utwórz projekt
projects-join = Dołącz do projektu
projects-import-tricount = Importuj z Tricount
project-actions = Akcje projektu

status-ongoing = W toku
status-closed = Zamknięty
status-archived = Zarchiwizowany

nav-help = Pomoc
nav-privacy = Polityka prywatności
nav-terms = Regulamin
nav-legal = Nota prawna

leave-project-title = Opuścić projekt?
leave-project-message = Stracisz dostęp z tego urządzenia. Jeśli nie zostanie żaden członek, projekt i wszystkie jego wydatki zostaną trwale usunięte.

add-project-title = Nowy projekt
add-project-name-label = Nazwa projektu
add-project-name-placeholder = Moja podróż, Mieszkanie 2024…
add-project-participants = Uczestnicy
add-project-participant-name = Imię uczestnika
add-project-participant-placeholder = Clark Kent
add-project-remove-participant = Usuń uczestnika
add-project-me-badge = Ja
add-project-thats-me = To ja!
add-project-offline = Nie można utworzyć projektu offline. Połącz się ponownie i spróbuj jeszcze raz.
add-project-name-required = Projekt potrzebuje nazwy.
add-project-need-two-participants = Dodaj co najmniej 2 uczestników.
add-project-pick-yourself = Wskaż, którym uczestnikiem jesteś.

join-link-label = Link udostępniania
join-link-hint = Link zawiera klucz deszyfrujący - skopiuj go w całości.
join-invalid-link = Ten link jest nieprawidłowy. Wklej cały link udostępniania, łącznie z częścią po #.
join-wrong-project = Ten link dotyczy innego projektu.

import-tricount-link-label = Link lub klucz Tricount
import-tricount-key-required = Wpisz link lub klucz Tricount.
import-tricount-encryption-failed = Szyfrowanie nie powiodło się.

### Expenses

save = Zapisz
saving = Zapisywanie…
adding = Dodawanie…
link-copied = Link skopiowany
missing-encryption-key = Brak klucza szyfrowania.
missing-encryption-key-title = Brak klucza szyfrowania
missing-encryption-key-hint = Użyty link nie zawiera klucza potrzebnego do odszyfrowania tego projektu. Użyj pełnego linku udostępnionego przez osobę, która go utworzyła.
project-locked-hint = To urządzenie nie ma klucza do tego projektu. Otwórz jego link udostępniania, aby go odblokować.
project-unlock = Odblokuj
project-no-local-data-hint = Zaloguj się, aby po raz pierwszy wczytać dane tego projektu.
project-gone-title = Ten projekt już nie istnieje
project-gone-hint = Został usunięty, gdy opuścił go ostatni członek. Link udostępniania już nie działa, nawet jeśli otworzysz go ponownie.

expense-add = Dodaj wydatek
transfer-add = Dodaj przelew
expense-edit-title = Edytuj wydatek
expense-category = Kategoria
expense-category-auto = Auto · { $emoji }
expense-currency = Waluta kwoty
amount-op-add = Plus
amount-op-subtract = Minus
amount-op-multiply = Razy
amount-op-divide = Podziel
amount-op-equals = Równa się
amount-op-done = Gotowe
expense-rate = Kurs wymiany (opcjonalnie)
expense-rate-hint = Zostaw puste, aby użyć kursu Komisji Europejskiej (InforEuro) za { $month }: 1 { $from } = { $rate } { $to }.
expense-rate-invalid = Wpisz kurs wymiany większy niż 0.
expense-rate-unavailable = Brak automatycznego kursu - wpisz go ręcznie.
expense-delete-title = Usuń wydatek
expense-delete-message = „{ $name }” zostanie trwale usunięty. Tego nie da się cofnąć.
expense-inconsistent-amounts = Kwoty się nie zgadzają
expenses-empty = Brak wydatków
expenses-empty-hint = Zacznij od dodania wydatków przyciskiem poniżej
expenses-show-more = Pokaż więcej (pozostało { $count })

expense-type-expense = Wydatek
expense-type-transfer = Przelew
expense-type-gain = Wpływ
expense-paid-by = zapłacił(a)
expense-sent-by = wysłał(a)
expense-contributed-by = wniósł/wniosła

expense-name-required = Nazwa jest wymagana.
expense-amount-not-positive = Kwota musi być większa niż 0.
expense-no-payer = Wybierz co najmniej jedną osobę płacącą.
expense-no-debtor = Wybierz co najmniej jedną osobę, która jest dłużna.
expense-invalid-date = Ta data jest nieprawidłowa.
expense-payers-mismatch = Suma płacących wynosi { $sum }, co nie zgadza się z kwotą wydatku ({ $total }).
expense-debtors-mismatch = Suma dłużników wynosi { $sum }, co nie zgadza się z kwotą wydatku ({ $total }).

participants-none = Nikt
participants-everyone = Wszyscy ({ $count })
participants-some = { $count } z { $total }
participants-select-all = Zaznacz wszystkich
participants-deselect-all = Odznacz wszystkich
participants-by-shares = Według udziałów
split-amounts = Kwoty
participants-remaining = Pozostało { $amount }
participants-over-by = O { $amount } za dużo
participants-who-paid = Kto zapłacił?
participants-who-received = Kto otrzymał?
participants-who-transfers = Kto przelewa?
participants-who-receives = Kto otrzymuje?
participants-for-whom = Dla kogo?

stats-total-expenses = Suma wydatków
stats-my-expenses = Moje wydatki

tab-expenses = Wydatki
tab-balance = Bilans
tab-reimbursements = Rozliczenie
reimbursements-empty-title = Wszystko rozliczone!
reimbursements-empty-hint = Propozycje rozliczeń pojawią się tutaj, gdy rachunki się nie zbilansują
reimbursement-owes = { $debtor } jest dłużny { $creditor }
reimbursement-record = Rozlicz
reimbursement-pay-with = Zapłać
reimbursement-pay-shared-by = Udostępnione przez { $name } - przed wysłaniem sprawdź nazwę odbiorcy pokazywaną przez Twoją aplikację.
reimbursement-pay-title = Zapłać { $name }
reimbursements-mine-title = Jesteś dłużny
reimbursements-others-title = Pozostałe zwroty
copy = Kopiuj

user-selection-title = Którym uczestnikiem jesteś?
user-selection-hint = Wybierz swoje imię z listy.
user-selection-required = Wybierz uczestnika.
identity-claimed = Powiązany z kontem
identity-claimed-by = Konto { $name }
identity-taken-repick = Inne konto przejęło uczestnika, którego używałeś. Wybierz innego.
participant-gone-repick = Uczestnik, którego używałeś, został usunięty z tego projektu. Wybierz innego.

edit-project-title = Edytuj projekt
edit-project-new-badge = nowy
edit-project-deferred-new-members = dodanie nowych członków
edit-project-deferred-removals = usunięcie członków
edit-project-deferred-me = wybór „To ja”
edit-project-offline-deferred = Offline: { $items } zostanie zastosowane po ponownym połączeniu.

export-saved = Plik zapisany:
    { $path }
export-failed = Eksport nie powiódł się: { $reason }

history-expense-added = Dodano wydatek: { $name }
history-expense-edited = Edytowano wydatek: { $name }
history-expense-deleted = Usunięto wydatek: { $name }
history-project-edited = Edytowano projekt: { $name }
history-name-changed = Nazwa: „{ $from }” → „{ $to }”
history-description-added = Dodano opis: „{ $value }”
history-description-removed = Usunięto opis: „{ $value }”
history-description-changed = Opis: „{ $from }” → „{ $to }”

### Sweep

field-amount = Kwota
expense-name-placeholder = Restauracja, zakupy…
expense-actions = Akcje wydatku
expense-your-share = Twój udział
expense-your-share-value = Twój udział: { $amount } { $currency }
expense-inconsistent-detail = Kwoty się nie zgadzają: zapłacono { $paid }, należne { $owed }, przy wydatku { $total }. Edytuj wydatek, aby to poprawić.
missing-access-key = Brak klucza dostępu. Otwórz ten projekt przez jego link udostępniania.
filter-all = Wszystkie
filter-my-payments = Moje płatności
filter-my-debts = Moje długi
participants-shares-for = Udziały: { $name }
participants-amount-for = Kwota: { $name }
reimbursement-add = Dodaj rozliczenie
project-forget = Usuń z mojej listy
project-history-title = Historia
history-kind-add = Dodano
history-kind-delete = Usunięto
history-kind-edit = Edytowano
export = Eksportuj
export-json = Eksportuj JSON
export-csv = Eksportuj CSV
share-link = Udostępnij
copy-link-failed = Nie udało się skopiować linku
open-in-app = Otwórz w aplikacji
not-found-title = Nie znaleziono strony
not-found-back = Wróć do projektów

### Charts

charts-period = Okres
period-all = Wszystko
period-month = Miesiąc
period-3months = 3 mies.
period-year = Rok
period-custom = Własny
charts-tab-categories = Kategorie
charts-tab-per-person = Na osobę
charts-tab-trends = Trendy
charts-by-category = Podział według kategorii
charts-per-person = Wydatki na osobę
charts-categories-by-month = Kategorie według miesięcy
charts-total-spent = Wydano łącznie
charts-avg-per-person = Śr. na osobę
charts-expense-count =
    { $count ->
        [one] { $count } wydatek
        [few] { $count } wydatki
        [many] { $count } wydatków
       *[other] { $count } wydatków
    }
charts-clear-category-filter = Wyczyść filtr kategorii
charts-no-expenses = Brak wydatków.
charts-pick-a-project = Wybierz projekt, aby zobaczyć wydatki na osobę.
charts-nothing-to-show = Nic do pokazania
charts-my-share-note = Te kwoty to Twój udział w każdym wydatku.
charts-my-share-skipped =
    { $count ->
        [one] 1 projekt nie jest liczony — nie wybrano uczestnika lub nie udało się wczytać jego danych.
        [few] { $count } projekty nie są liczone — nie wybrano uczestnika lub nie udało się wczytać ich danych.
        [many] { $count } projektów nie jest liczonych — nie wybrano uczestnika lub nie udało się wczytać ich danych.
       *[other] { $count } projektów nie jest liczonych — nie wybrano uczestnika lub nie udało się wczytać ich danych.
    }

### Categories

category-food = Jedzenie
category-transport = Transport
category-accommodation = Nocleg
category-leisure = Rozrywka
category-shopping = Zakupy
category-services = Usługi
category-parties-gifts = Imprezy i prezenty
category-other = Inne
charts-person = Osoba
charts-project = Projekt
charts-all-projects = Wszystkie projekty
charts-whole-project = Cały projekt
charts-date-from = Od
charts-date-to = Do
charts-total = Razem
charts-payments-per-person-by-month = Płatności na osobę według miesięcy
history-empty = Brak zdarzeń
history-by = { $name }
not-found-hint = Ta strona nie istnieje lub została przeniesiona.
payers-title-paid-by = Zapłacił(a)
payers-title-sender = Nadawca
payers-title-contributors = Wpłacający
debtors-title-debtors = Dłużni
debtors-title-recipients = Odbiorcy
debtors-title-beneficiaries = Beneficjenci

### Welcome

welcome-title = Twoje rachunki to nie czyjaś sprawa.
welcome-subtitle = Dziel wydatki ze znajomymi.
welcome-e2ee-title = Wszystko zaszyfrowane
welcome-e2ee-body = Imiona, kwoty, projekty: wszystko jest szyfrowane na Twoim urządzeniu. Tylko Ty masz klucz. Nikt nie może odczytać Twoich rozliczeń. Nawet my.
welcome-e2ee-note = Nieczytelne nawet dla nas (zero dostępu po stronie serwera)
welcome-eu-title = 100% europejskie
welcome-eu-body = Serwery w Niemczech, e-maile wysyłane z Francji. Twoje dane nigdy nie opuszczają Unii Europejskiej.
welcome-noads-title = Bez reklam. Bez śledzenia.
welcome-noads-body = Niczego nie zbieramy i nie sprzedajemy Twoich danych. To nie nasz model.
welcome-start = Zaczynamy
welcome-how-it-works = Jak to właściwie działa?

### Help

help-intro = Częste pytanie? Dotknij, aby rozwinąć odpowiedź.
help-create-project-q = Jak utworzyć projekt?
help-create-project-a = Na ekranie głównym dotknij przycisku + na dole. Nadaj projektowi nazwę, wybierz walutę i gotowe.
help-add-participants-q = Jak dodać uczestników?
help-add-participants-a = Otwórz projekt i dodaj uczestników z listy członków. Każdy uczestnik może płacić za wydatek lub być za niego dłużny.
help-share-project-q = Jak udostępnić projekt?
help-share-project-a = Udostępnij adres URL projektu (ten z paska adresu). Każdy, kto ma link, może przeglądać i edytować projekt.
help-add-expense-q = Jak dodać wydatek?
help-add-expense-a = W projekcie dotknij +, wpisz kwotę, wskaż, kto zapłacił i między kogo podzielić. Możesz też wybrać datę inną niż dzisiejsza.
help-types-q = Czym różni się wydatek, przelew i wpływ?
help-types-expense = - zakup dokonany przez jedną osobę i podzielony między kilka.
help-types-transfer = - zwrot od jednej osoby do drugiej, bez podziału.
help-types-gain = - otrzymane pieniądze (zwrot, prezent) do podziału między kilka osób.
help-past-date-q = Czy mogę datować wydatek wstecz?
help-past-date-a = Tak, pole daty jest dowolne. Czas utworzenia wpisu jest przechowywany osobno.
help-who-owes-q = Jak Counted ustala, kto ile jest dłużny?
help-who-owes-a = Counted oblicza saldo netto każdego uczestnika (ile wyłożył minus ile jest dłużny), a następnie proponuje najkrótszą serię przelewów, która rozlicza wszystkich.
help-minimal-transfers-q = Dlaczego liczba proponowanych przelewów jest minimalna?
help-minimal-transfers-a = Algorytm najpierw paruje salda, które dokładnie się znoszą, a potem przechodzi przez resztę od największego wierzyciela do największego dłużnika. Efekt: mniej przelewów, by wszystko rozliczyć.
help-import-tricount-q = Jak zaimportować projekt z Tricount?
help-import-tricount-a = Na ekranie głównym dotknij przycisku „+” na dole, a następnie
help-import-tricount-b = Wklej link udostępniania Tricounta, który chcesz zaimportować.
help-encryption-q = Czy moje dane są zaszyfrowane?
help-encryption-a = Tak. Counted łączy dwie gwarancje:
help-encryption-e2ee-term = Szyfrowanie end-to-end
help-encryption-e2ee-def = - wszystko między Tobą a serwerem podróżuje zaszyfrowane.
help-encryption-zero-term = Zero dostępu
help-encryption-zero-def = - szyfrujesz dane przed wysłaniem, a serwer przechowuje wyłącznie szyfrogram. Nie mamy jak go odczytać.
help-encryption-see = Szczegóły znajdziesz w
help-forgot-password-q = Co się stanie, jeśli zapomnę hasła?
help-forgot-password-warning = Twoje dane zostaną bezpowrotnie utracone.
help-forgot-password-a = Klucz szyfrowania jest wyprowadzany z Twojego hasła, więc reset nie jest możliwy: nikt - łącznie z nami - nie odszyfruje Twoich projektów bez niego. Przechowuj je bezpiecznie, najlepiej w menedżerze haseł.
help-archive-delete-q = Jak zarchiwizować lub usunąć projekt?
help-archive-delete-a = Na ekranie projektu otwórz menu i wybierz
help-archive-delete-b = aby go ukryć, zachowując go. Projekt jest usuwany na dobre, gdy opuści go ostatni członek.
help-delete-account-q = Jak usunąć konto?
help-delete-account-a = Otwórz Ustawienia i użyj „Usuń moje konto”. Jest to natychmiastowe i nieodwracalne.
help-contact = Inne pytanie? Napisz do nas na

# Receipt scanning (mobile only)
expense-scan = Zeskanuj paragon
scan-in-progress = Odczytywanie paragonu…
scan-error-capture = Nie udało się zrobić zdjęcia. Spróbuj ponownie lub wpisz wydatek ręcznie.
scan-error-unreadable = Nic czytelnego na tym paragonie. Wpisz wydatek ręcznie.
scan-check-amount = Sprawdź sumę - nie była wyraźnie wydrukowana.
scan-take-photo = Zrób zdjęcie
scan-choose-photo = Wybierz zdjęcie
expense-converted-from = Zapłacono { $amount } { $from } · 1 { $from } = { $rate } { $to }
project-currency = Waluta
project-currency-hint = Każda kwota jest pokazywana w tej walucie. Nie można jej później zmienić.
project-currency-locked = Waluta jest ustalana przy tworzeniu projektu.

update-required-title = Wymagana aktualizacja
update-required-body = Ta wersja Counted jest za stara, by komunikować się z serwerem. Zaktualizuj ją, aby dalej korzystać z aplikacji.
update-required-body-testflight = Ta wersja Counted jest za stara, by komunikować się z serwerem. Otwórz TestFlight i zainstaluj najnowszą kompilację, aby dalej korzystać z aplikacji.
update-required-button = Aktualizuj

notifications-label = Powiadomienia
notifications-title = Powiadomienia
notifications-empty = Nic nowego
notifications-friend-request = Zaproszenie do znajomych

friends-title = Znajomi
friends-anonymous-body = Znajomi są powiązani z Twoim kontem. Zaloguj się, aby dodawać osoby i zapraszać je do projektów bez udostępniania linku.
friends-add-title = Dodaj znajomego
friends-add-hint = Zobaczy Twoje zaproszenie po zalogowaniu. Żadne z Was nie dowie się, czy drugie ma konto, dopóki zaproszenie nie zostanie przyjęte.
friends-add-button = Dodaj
friends-add-from-project = Dodaj do znajomych
friends-request-sent = Zaproszenie wysłane
friends-no-account-key = Zaloguj się ponownie, aby zarządzać znajomymi na tym urządzeniu.
friends-incoming-title = Zaproszenia
friends-accept = Przyjmij
friends-decline = Odrzuć
friends-list-title = Moi znajomi
friends-list-empty = Nie masz jeszcze znajomych. Dodaj kogoś przez e-mail powyżej lub ze wspólnego projektu.
friends-remove = Usuń
friends-remove-confirm-title = Usuń znajomego
friends-remove-confirm-message = { $email } nie będzie już wśród Twoich znajomych, a Ty wśród jego. Każde z was może później wysłać nowe zaproszenie.
friends-no-key = Jeszcze niegotowe
friends-fingerprint = Kod bezpieczeństwa
friends-fingerprint-hint = Dwoje znajomych, którzy odczytają sobie ten sam kod bezpieczeństwa, wie, że nikt nie stoi między nimi - nawet nasz serwer.
friends-outgoing-title = Wysłane
friends-outgoing-hint = Oczekiwanie na odpowiedź. Pojawią się wśród znajomych, gdy przyjmą zaproszenie.
friends-withdraw = Anuluj
invite-friends-title = Zaproś znajomych
invite-friends-hint = Klucz projektu jest szyfrowany dla każdego znajomego na tym urządzeniu. Serwer nigdy go nie widzi.
invite-friends-empty = Nie ma jeszcze znajomych do zaproszenia.
invite-friends-button = Zaproś
invite-sent = { $count ->
    [one] Zaproszenie wysłane
    [few] Wysłano { $count } zaproszenia
    [many] Wysłano { $count } zaproszeń
   *[other] Wysłano { $count } zaproszeń
}
invitation-badge = Zaproszenie
invitation-to = Dołącz do „{ $name }”
invitation-to-unnamed = Dołącz do projektu
invitation-unreadable = Tego zaproszenia nie można otworzyć na tym urządzeniu
invitation-from = Od { $email }
invitation-accept = Dołącz
invitation-decline = Odrzuć
