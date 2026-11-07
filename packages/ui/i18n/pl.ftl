# Polski. Kompletny z wyjątkiem tekstów prawnych (legal-, terms-, privacy-), które istnieją tylko po
# angielsku i francusku i dla każdego komunikatu z osobna wracają do en.ftl.

### Common

loading = Ładowanie…
cancel = Anuluj
discard-title = Odrzucić zmiany?
discard-message = Wpisane dane zostaną utracone.
discard = Odrzuć
confirm = Potwierdź
retry = Spróbuj ponownie
delete = Usuń
back = Wstecz
language = Język
theme = Motyw
theme-light = Jasny
theme-dark = Ciemny
theme-system = Systemowy
theme-hint-device = Zapisany tylko na tym urządzeniu.
theme-hint-system = Dopasowuje się do wyglądu tego urządzenia.

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
error-rate-limited = Serwer jest przeciążony. Spróbuj ponownie za chwilę.
error-generic = Coś poszło nie tak. Spróbuj ponownie.

error-invalid-email = Ten adres e-mail jest nieprawidłowy.
error-invalid-password = To hasło jest nieprawidłowe.
error-password-too-short = Hasło musi mieć co najmniej 12 znaków.
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
error-storage-full = Pamięć jest pełna: nie udało się zapisać klucza tego projektu na tym urządzeniu. Zachowaj link do udostępniania.
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
error-invalid-kdf-salt = Ustawienia szyfrowania są nieprawidłowe. Zaktualizuj aplikację i spróbuj ponownie.
error-mixed-project-batch = Ci uczestnicy nie należą wszyscy do tego samego projektu.
error-invalid-payload = Ta wersja aplikacji wysłała dane, których serwer nie przyjmuje. Zaktualizuj ją i spróbuj ponownie.
error-invalid-public-key = Twój klucz szyfrowania jest nieprawidłowy. Zaktualizuj aplikację i spróbuj ponownie.
error-payment-methods-stale = Twoje dane płatności zostały zmienione na innym urządzeniu. Odśwież i spróbuj ponownie.

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
login-maybe-unverified = Dopiero się rejestrujesz? Najpierw potwierdź adres linkiem, który ci wysłaliśmy, albo wyślij go ponownie.
login-resend = Wyślij link weryfikacyjny ponownie
login-resending = Wysyłanie…
login-resend-sent = E-mail wysłany - sprawdź skrzynkę.

register-submit = Utwórz konto
register-submitting = Tworzenie…
register-have-account = Masz już konto?
register-password-placeholder = Co najmniej 12 znaków
register-password-warning = Zapisz swoje hasło: nigdy go nie widzimy i nie możemy ci go zwrócić.
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
account-delete-warning = Natychmiastowe i nieodwracalne usunięcie Twojego konta i Twoich danych.
account-delete-confirm-title = Usuń konto
account-delete-confirm-message = Twoje konto, sesje i lista projektów zostaną trwale usunięte. Bez Twojego hasła zaszyfrowane dane wspólnego projektu staną się dla Ciebie nieczytelne - tego nie da się cofnąć.
erase-device-open = Wyloguj się i wyczyść to urządzenie
erase-device-title = Wyczyścić to urządzenie?
erase-device-body = Projekty, ich klucze i pamięć podręczna na tym urządzeniu zostaną usunięte, a potem sesja zostanie zakończona. Twoje projekty zostają na Twoim koncie: zaloguj się ponownie, aby je odzyskać.
erase-device-checking = Sprawdzam, czy Twoje projekty są na Twoim koncie…
erase-device-pending = Zmiany czekające na wysłanie: { $count }. Połącz się z internetem i spróbuj ponownie.
erase-device-unreachable = Nie udało się sprawdzić, czy Twoje projekty są na Twoim koncie. Sprawdź połączenie i spróbuj ponownie.
erase-device-at-risk = Tych projektów nie ma na Twoim koncie i zostaną utracone. Aby je zachować, anuluj i najpierw skopiuj ich link:
erase-device-confirm = Wyczyść i wyloguj
erase-device-confirm-anyway = Wyczyść mimo to
erase-device-erasing = Czyszczenie…

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
payment-methods-stale = Twoje dane do płatności zostały zmienione na innym urządzeniu. Zostały wczytane ponownie - spróbuj jeszcze raz.
payment-methods-key-missing = Zaloguj się ponownie, aby zarządzać danymi do płatności.
settings-payment-methods-share-warning = Udostępniona metoda jest widoczna dla wszystkich członków Twoich projektów.
payment-method-share = Udostępnij moim projektom
payment-method-share-hint = Pokazywane obok Twojego imienia, gdy ktoś jest Ci dłużny.
payment-method-copy = Kopiuj { $name }
payment-method-copied = Skopiowano.
payment-method-copy-failed = Nie udało się skopiować - zaznacz tekst i skopiuj go ręcznie.

verify-email-checking = Weryfikacja adresu e-mail…
verify-email-welcome = E-mail zweryfikowany - witaj w Counted!
verify-email-back-to-login = Wróć do logowania
verify-email-intro = Wpisz swoje hasło, aby potwierdzić, że ten adres należy do ciebie.
verify-email-submit = Potwierdź mój adres

### Project status

project-archive = Archiwizuj
project-sheet-invite = Zaproś
project-sheet-recurring = Cykliczne
project-sheet-edit = Edytuj projekt
project-sheet-close = Zamknij projekt
project-sheet-archive = Archiwizuj projekt
project-sheet-reopen = Otwórz projekt ponownie
project-sheet-unarchive = Przywróć projekt z archiwum
project-sheet-leave = Opuść projekt

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
projects-offline-banner = Dane offline - połącz się ponownie, aby odświeżyć.
demo-banner = Projekt demonstracyjny - tylko do odczytu.
demo-start-own = Utwórz własny projekt
demo-badge = Demo
demo-bar = Twoje zmiany znikną po zamknięciu tej karty.
demo-leave = Zacznij naprawdę
demo-preparing = Przygotowujemy Twoje demo…
demo-failed = Nie udało się przygotować demo.
demo-sign-in-blocked = Logowanie jest wyłączone w demo. Opuść je, aby się zalogować lub założyć konto; Twoje własne projekty pozostają nietknięte.
projects-no-local-data = Brak danych lokalnych
projects-no-local-data-hint = Zaloguj się, aby po raz pierwszy wczytać swoje projekty.
projects-add = Dodaj projekt
projects-create = Utwórz projekt
projects-personal-expense = Wydatek osobisty
personal-expense-banner = Dodano do Moich wydatków
personal-create-title = Utwórz swój projekt osobisty
personal-create-hint = Śledź, ile wydajesz sam
group-create-title = Utwórz wspólny projekt
group-create-hint = Dziel wydatki z przyjaciółmi i rodziną
personal-create-action = Utwórz
personal-create-signup = Załóż konto
personal-sheet-title = Twój projekt osobisty
personal-sheet-intro = Do zapisywania tego, co wydajesz sam. Pojawia się z twoimi grupami w Statystykach.
personal-sheet-point-alone = Jeden członek: ty, z nazwą twojego konta. Bez linku do udostępniania.
personal-sheet-point-encrypted = Szyfrowany na twoich urządzeniach, jak inne projekty.
personal-sheet-point-stats = Twoje udziały w grupach dodają się do tych wydatków w Statystykach.
personal-default-name = Moje wydatki
my-spending-prev-month = Poprzedni miesiąc
my-spending-next-month = Następny miesiąc
my-spending-spent = Wydałeś(-aś)
my-spending-groups = Grupy
my-spending-income = Przychody
my-spending-balance = Bilans miesiąca
my-spending-unconverted = Udziały w innej walucie są pomijane, dopóki kurs nie jest znany.
my-spending-empty = Nic w tym miesiącu
my-spending-empty-hint = Dodaj wydatek przyciskiem +. Twoje udziały w grupach pojawią się tu automatycznie.
my-spending-share-of = z { $amount }
my-spending-uncounted = nieliczone
payer-you = ty
expense-received-by = otrzymał(a)
counted-switch = Licz w Moje wydatki
counted-switch-on = Włączone: twój udział dolicza się do Moje wydatki
counted-switch-off = Wyłączone: twój udział nie pojawia się w Moje wydatki
counted-groups = Liczone grupy
counted-groups-intro = Twój udział w tych grupach dolicza się do Moje wydatki i do budżetów.
counted-groups-converted = { $from }, przeliczone na { $to }
counted-groups-encrypted = Ten wybór jest szyfrowany twoim kontem i podąża za tobą na twoich urządzeniach.
counted-groups-empty = Na tym urządzeniu nie ma jeszcze żadnej grupy.
counted-groups-done = OK
counted-save-failed = Nie udało się zapisać tego wyboru. Spróbuj ponownie.
tab-budget = Budżet
budget-title = Budżety
budget-manage = Zarządzaj
budget-left = Zostało { $amount }
budget-over = Przekroczono o { $amount }
budget-empty = Brak budżetu
budget-empty-hint = Ustaw limit dla każdej kategorii. Twoje udziały w grupach też się liczą.
budget-define = Ustaw budżet
budget-sheet-title = Budżety miesięczne
budget-sheet-intro = Na twój udział: osobiste i grupy. Zostaw puste, aby nie śledzić kategorii.
budget-average = Śr. z ostatnich 3 miesięcy: { $amount }
budget-for = Budżet { $category }
budget-alert = Ostrzeż mnie przy 80 %
budget-alert-hint = Liczone na tym urządzeniu
budget-invalid = Nieprawidłowa kwota dla { $category }.
budget-offline = Budżety można zapisać tylko online. Połącz się ponownie i spróbuj jeszcze raz.
budget-alert-near = { $category }: { $percent } % budżetu miesiąca
budget-alert-over = { $category }: budżet miesiąca przekroczony
my-spending-search-summary =
    { $count ->
        [one] { $count } wynik
        [few] { $count } wyniki
        [many] { $count } wyników
       *[other] { $count } wyników
    } · { $total } · osobiste i grupy, wszystkie miesiące
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
add-project-offline = Nie można utworzyć projektu offline. Połącz się ponownie i spróbuj jeszcze raz.
add-project-name-required = Projekt potrzebuje nazwy.

join-link-label = Link udostępniania
join-link-hint = Link zawiera klucz deszyfrujący - skopiuj go w całości.
join-invalid-link = Ten link jest nieprawidłowy. Wklej cały link udostępniania, łącznie z częścią po #.
join-wrong-project = Ten link dotyczy innego projektu.

import-tricount-link-label = Link lub klucz Tricount
import-tricount-key-required = Wpisz link lub klucz Tricount.
import-tricount-encryption-failed = Szyfrowanie nie powiodło się.
import-tricount-unimportable = Nic nie zostało zaimportowane: ten Tricount ma członków z kontem Tricount lub kwoty, które się nie zgadzają (dotyczy wpisów: { $count }).

### Expenses

save = Zapisz
saving = Zapisywanie…
adding = Dodawanie…
link-copied = Link skopiowany
missing-encryption-key = Brak klucza szyfrowania.
missing-encryption-key-title = Brak klucza szyfrowania
missing-encryption-key-hint = Użyty link nie zawiera klucza potrzebnego do odszyfrowania tego projektu. Użyj pełnego linku udostępnionego przez osobę, która go utworzyła.
project-locked-hint = To urządzenie nie ma klucza do tego projektu. Otwórz jego link udostępniania, aby go odblokować.
expenses-unreadable = Niektórych wpisów nie da się odczytać na tym urządzeniu i nie są wliczone do sum. Zaktualizuj aplikację, aby je zobaczyć.
project-unlock = Odblokuj
project-no-local-data-hint = Zaloguj się, aby po raz pierwszy wczytać dane tego projektu.
project-gone-title = Ten projekt już nie istnieje
project-gone-hint = Został usunięty, gdy opuścił go ostatni członek. Link udostępniania już nie działa, nawet jeśli otworzysz go ponownie.

expense-add = Dodaj wydatek
transfer-add = Dodaj przelew
expense-edit-title = Edytuj wydatek
expense-category = Kategoria
expense-category-auto = Auto · { $emoji }
emoji-picker-label = Emoji
emoji-none = Bez emoji
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
expense-type-group = Typ
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
balance-gets-back = Odzyskuje
balance-owes = Ma do oddania
balance-settled = Rozliczeni
reimbursements-empty-title = Wszystko rozliczone!
reimbursements-empty-hint = Propozycje rozliczeń pojawią się tutaj, gdy rachunki się nie zbilansują
reimbursement-owes = { $debtor } jest dłużny { $creditor }
reimbursement-record = Rozlicz
settlement-recorded = Rozliczenie zapisane.
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
edit-project-offline-deferred = Offline: { $items } zostanie zastosowane po ponownym połączeniu.

export-failed = Eksport nie powiódł się: { $reason }

history-expense-added = Dodano wydatek: { $name }
history-expense-edited = Edytowano wydatek: { $name }
history-expense-deleted = Usunięto wydatek: { $name }
move-expense = Przenieś do projektu
move-expense-title = Przenieś do…
move-current = Bieżący
move-converted = { $currency }, przeliczone po kursie miesiąca
move-note-personal = W następnym kroku wybierzesz, kto zapłacił i za kogo. Wydatek opuści Moje wydatki.
move-note-group = Wydatek opuści { $project } dla wszystkich członków, a bilans grupy zostanie przeliczony. W następnym kroku wybierzesz, kto zapłacił i za kogo.
move-banner = Z { $from } do { $to }
move-submit = Przenieś
move-done = Wydatek przeniesiono do { $project }
history-expense-moved = Wydatek przeniesiono do innego projektu: { $name }
personal-badge = Osobisty
move-members =
    { $count ->
        [one] { $count } członek
        [few] { $count } członków
        [many] { $count } członków
       *[other] { $count } członka
    }
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
expense-share-percent = { $percent }%
expense-added-by = Dodane przez { $name }, { $date }
expense-inconsistent-detail = Kwoty się nie zgadzają: zapłacono { $paid }, należne { $owed }, przy wydatku { $total }. Edytuj wydatek, aby to poprawić.
missing-access-key = Brak klucza dostępu. Otwórz ten projekt przez jego link udostępniania.
filter-all = Wszystkie
filter-my-payments = Moje płatności
filter-my-debts = Moje długi
expense-filter-group = Filtr
expenses-search = Szukaj wydatków
expenses-no-match = Brak pasujących wydatków.
expenses-search-open = Szukaj
expenses-search-clear = Wyczyść wyszukiwanie
expenses-search-summary =
    { $count ->
        [one] { $count } wynik
        [few] { $count } wyniki
        [many] { $count } wyników
       *[other] { $count } wyników
    } · { $total } · wszystkie miesiące
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
charts-tab-trends = Trendy
charts-total-spent = Wydano łącznie
charts-avg-per-person = Śr. na osobę
charts-expense-count =
    { $count ->
        [one] { $count } wydatek
        [few] { $count } wydatki
        [many] { $count } wydatków
       *[other] { $count } wydatków
    }
charts-nothing-to-show = Nic do pokazania
charts-my-share-skipped =
    { $count ->
        [one] 1 projekt nie jest liczony - nie wybrano uczestnika lub nie udało się wczytać jego danych.
        [few] { $count } projekty nie są liczone - nie wybrano uczestnika lub nie udało się wczytać ich danych.
        [many] { $count } projektów nie jest liczonych - nie wybrano uczestnika lub nie udało się wczytać ich danych.
       *[other] { $count } projektów nie jest liczonych - nie wybrano uczestnika lub nie udało się wczytać ich danych.
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
categories-manage = Zarządzaj kategoriami
categories-title = Kategorie
categories-yours = Twoje kategorie
categories-builtin = Domyślne
categories-add = Dodaj kategorię
categories-in-parent = w kategorii { $parent }
categories-renamed-from = zmieniona nazwa · pierwotnie { $name }
categories-hidden-hint = ukryta · jej wydatki trafiają do kategorii { $other }
categories-always-shown = zawsze widoczna
categories-hidden = Ukryta
categories-new = Nowa kategoria
categories-edit = Edytuj kategorię
categories-parent = Liczona w kategorii
categories-parent-hint = W statystykach wszystkich twoich projektów { $name } liczy się do kategorii { $parent }.
categories-create = Utwórz
categories-hide = Ukryj tę kategorię
categories-hide-hint = Jej wydatki pojawiają się w kategorii { $other } dla wszystkich członków.
categories-delete-title = Usunąć „{ $name }”?
categories-delete-body = Wydatki z tej kategorii ({ $count }) wracają do kategorii { $parent }. Żaden wydatek nie zostanie usunięty.
categories-delete-body-unknown = Jej wydatki wracają do kategorii { $parent }. Żaden wydatek nie zostanie usunięty.
categories-delete-warning = Kategoria zniknie dla wszystkich członków projektu.
categories-summary-created = Utworzone: { $count }
categories-summary-renamed = Ze zmienioną nazwą: { $count }
categories-summary-hidden = Ukryte: { $count }
categories-summary-default = Brak własnych kategorii
categories-offline = Kategoriami można zarządzać tylko online. Połącz się ponownie i spróbuj jeszcze raz.
categories-error-too-many = Projekt może mieć najwyżej 20 własnych kategorii.
categories-error-full = Ten projekt osiągnął limit kategorii.
categories-error-name = Nadaj kategorii nazwę.
categories-error-missing = Ta kategoria w międzyczasie się zmieniła. Zamknij i spróbuj ponownie.
history-category-created = Utworzono kategorię: { $name }
history-category-edited = Edytowano kategorię: { $name }
history-category-deleted = Usunięto kategorię: { $name }
charts-project = Projekt
charts-all-projects = Wszystkie projekty
charts-date-from = Od
charts-date-to = Do
charts-total = Razem
charts-tab-people = Osoby
charts-tab-projects = Projekty
charts-scope = Czyje wydatki
charts-scope-group = Grupa
charts-scope-me = Ja
charts-currency = Waluta
charts-my-share = Mój udział
charts-share-of-total = { $pct }% z { $total }
charts-i-paid = Zapłaciłem
charts-paid-more = { $amount } więcej niż mój udział
charts-paid-less = { $amount } mniej niż mój udział
charts-paid-even = Dokładnie mój udział
charts-part-title = Twoja część każdej kategorii
charts-part-desc = Szare to wydatki grupy, kolor to to, co zużyłeś.
charts-breakdown-title = Podział według kategorii
charts-breakdown-desc = Dotknij fragmentu lub wiersza, aby zobaczyć wydatki.
charts-of-total = { $amount } z { $total }
charts-show-all = Pokaż wszystko ({ $count })
charts-show-less = Pokaż mniej
charts-spend-title = Wydatki w czasie
charts-group-by = Grupuj według
bucket-day = Dzień
bucket-week = Tydzień
bucket-month = Miesiąc
charts-avg = śr.
charts-cat-title-day = { $category }, dzień po dniu
charts-cat-title-week = { $category }, tydzień po tygodniu
charts-cat-title-month = { $category }, miesiąc po miesiącu
charts-cat-desc = Wybierz kategorię, aby śledzić ją w czasie.
charts-running-title = Suma narastająco
charts-running-desc = Od { $date }.
charts-avg-per-day = { $amount } / dzień średnio
charts-avg-per-week = { $amount } / tydzień średnio
charts-avg-per-month = { $amount } / miesiąc średnio
charts-people-title = Kto dźwigał grupę
charts-people-desc = Ile każdy zapłacił, obok tego, ile zużył.
charts-paid = Zapłacone
charts-fair-share = Uczciwy udział
charts-you = (Ty)
charts-net-more = zapłacił więcej
charts-net-less = zapłacił mniej
charts-balance-title = Twoje saldo w czasie
charts-balance-desc = Nad linią grupa jest Ci winna. Pod nią Ty jesteś winien grupie.
charts-owed = Należy Ci się
charts-owe = Jesteś winien
charts-projects-title = Twój udział według projektu
charts-projects-desc = Sumy są liczone osobno dla każdej waluty.
history-empty = Brak zdarzeń
history-by = { $name }
not-found-hint = Ta strona nie istnieje lub została przeniesiona.
payers-title-paid-by = Zapłacił(a)
payers-title-sender = Nadawca
payers-title-contributors = Wpłacający
debtors-title-debtors = Dłużni
debtors-title-recipients = Odbiorcy
debtors-title-beneficiaries = Beneficjenci
debtors-title-for = Dla
debtors-title-recipient = Odbiorca
expense-split-between = podzielone między { $count }
expense-people =
    { $count ->
        [one] { $count } osoba
        [few] { $count } osoby
        [many] { $count } osób
       *[other] { $count } osoby
    }

### Welcome

welcome-title = Twoje rachunki to nie czyjaś sprawa.
welcome-subtitle = Dziel wydatki ze znajomymi.
welcome-note = Za darmo. Bez zakładania konta. Bez reklam.
welcome-link-title = Jeden link i wszyscy są w grze.
welcome-link-body = Nikt nie musi zakładać konta.
welcome-link-account = Konto? Nigdy nie jest obowiązkowe. Służy do odnajdywania projektów na innym urządzeniu, zapraszania znajomych z aplikacji i udostępniania Twoich danych do płatności.
welcome-demo-project = Weekend w Lyonie
welcome-private-title = Nikt nie może odczytać Twoich rozliczeń. Nawet my.
welcome-private-body = Imiona, kwoty, projekty: wszystko jest szyfrowane na Twoim urządzeniu. Tylko Ty masz klucz.
welcome-private-names = Imiona
welcome-private-amounts = Kwoty
welcome-private-projects = Projekty
welcome-scan-title = Zrób zdjęcie paragonu.
welcome-scan-body = Kwota, data i kategoria uzupełniają się same. Wszystko dzieje się na Twoim telefonie. Zdjęcie nie jest przechowywane.
welcome-eu-title = 100% europejskie
welcome-no-ads = Bez reklam
welcome-no-trackers = Bez śledzenia
welcome-step = Krok { $current } z { $total }
welcome-next = Dalej
welcome-skip = Pomiń
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
help-forgot-password-warning = Nikt nie odzyska go za Ciebie.
help-forgot-password-a = Twoje hasło chroni Twoje klucze szyfrowania i nigdy nie jest do nas wysyłane: nikt - łącznie z nami - nie odszyfruje Twoich projektów za Ciebie. Przechowuj je bezpiecznie, najlepiej w menedżerze haseł.
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
project-currency-locked = Waluta jest ustalana przy tworzeniu projektu.
currency-search = Szukaj waluty

update-required-title = Wymagana aktualizacja
update-required-body = Ta wersja Counted jest za stara, by komunikować się z serwerem. Zaktualizuj ją, aby dalej korzystać z aplikacji.
update-required-button = Aktualizuj

notifications-label = Powiadomienia
notifications-title = Powiadomienia
notifications-empty = Nic nowego
notifications-friend-request = Zaproszenie do znajomych

friends-title = Znajomi
friends-anonymous-body = Znajomi są powiązani z Twoim kontem. Zaloguj się, aby dodawać osoby i zapraszać je do projektów bez udostępniania linku.
friends-add-title = Dodaj znajomego
friends-add-hint = Jeśli ta osoba ma konto, zobaczy Twoje zaproszenie w aplikacji. Jeśli nie, daj jej znać: znajdzie je po rejestracji na ten adres.
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
friends-key-changed = Kod zmieniony
friends-key-changed-hint = Kod bezpieczeństwa znajomego się zmienił. Dzieje się tak, gdy zabezpiecza on konto, albo gdy jego klucz został podmieniony. Zaproszenia z nim są zablokowane, dopóki nie odczytacie sobie nowego kodu osobiście lub przez telefon i nie dotkniesz „Kod się zgadza”.
friends-fingerprint = Kod bezpieczeństwa
friends-fingerprint-hint = Dwoje znajomych, którzy odczytają sobie ten sam kod bezpieczeństwa, wie, że nikt nie stoi między nimi - nawet nasz serwer.
friends-key-mismatch = Serwer wysłał dla twojego konta klucz publiczny, który nie pasuje do twojego klucza prywatnego. Twój kod bezpieczeństwa jest ukryty, a zaproszenia są wyłączone na tym urządzeniu: ktoś może manipulować serwerem.
keypair-mismatch = Klucze, które serwer przysłał dla twojego konta, nie zgadzają się z tymi na tym urządzeniu. Zaproszenia i zmiany klucza są tu wyłączone, dopóki się nie zgodzą: ktoś może manipulować serwerem.
friends-outgoing-title = Wysłane
friends-outgoing-hint = Oczekiwanie na odpowiedź. Pojawią się wśród znajomych, gdy przyjmą zaproszenie.
friends-withdraw = Anuluj
friends-notify = Daj znać wiadomością
friends-notify-label = Daj znać: { $email }
friends-notify-message = Przechodzę na Counted dla naszych wspólnych wydatków. Darmowa rejestracja na https://counted.fr z adresem { $email }: czeka tam moje zaproszenie do znajomych.
friends-notify-copied = Skopiowano wiadomość
friends-notify-copy-failed = Nie udało się skopiować wiadomości
guests-hint-title =
    { $count ->
        [one] { $count } uczestnik jeszcze nie dołączył do projektu
        [few] { $count } uczestnicy jeszcze nie dołączyli do projektu
        [many] { $count } uczestników jeszcze nie dołączyło do projektu
       *[other] { $count } uczestnika jeszcze nie dołączyło do projektu
    }
guests-hint-later = Później
guests-hint-send = Zaproś ich
guests-send-title = Wyślij projekt uczestnikom
guests-send-friend = Zaproś znajomego z Counted
guests-send-friend-badge = Bez linku
guests-send-friend-hint = Jeśli ma już konto, dostaje zaszyfrowane zaproszenie; nic nie przechodzi przez Twój komunikator.
guests-send-link = Udostępnij link do projektu
guests-send-link-hint = Dla osób bez konta. Otwierają projekt jednym dotknięciem.
guests-send-link-warning = Każdy, kto ma ten link, może czytać i edytować projekt. Twój komunikator też go widzi.
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

# Participants in the create and edit modals, and the "who are you?" picker - see
# docs/plans/friends.md §11.
participants-you-label = Twoje imię w tym projekcie
participants-you-badge = Ty
participants-others = Pozostali uczestnicy
participants-removed = Usunięci
participant-restore = Przywróć
participants-empty = Dodaj uczestników poniżej.
participants-duplicate = „{ $name }” jest już na liście.
participants-input-label = Dodaj znajomego lub wpisz imię
participants-input-placeholder = Znajomy lub dowolne imię
participants-suggest-friend = Znajomy · dołączy jako „{ $name }”, dostanie zaproszenie
participants-suggest-not-ready = Znajomy · jeszcze niegotowy
participants-suggest-guest = Dodaj „{ $text }”
participants-suggest-guest-sub = Gość
participants-friends = Twoi znajomi
participants-all-friends = Wszyscy znajomi
participants-login-hint = Zaloguj się, aby dodawać osoby prosto z listy znajomych.
participants-invite-badge = Zaproś
participants-guest-badge = Gość
participants-rename = Zmień nazwę: { $name }
participants-remove = Usuń: { $name }
participants-rename-label = Nowe imię
participants-rename-save = Zapisz imię
participants-rename-hint = Imię, które wszyscy widzą w tym projekcie. Zaproszenie nadal trafi na { $email }.
participants-invited-badge = Zaproszony
participants-invited-sub = { $email } · jeszcze nie przyjęte
participants-invited-pending = Zaproszenie jeszcze nie przyjęte
participants-unlinked = Niepowiązany z kontem
add-project-create-invite = Utwórz i zaproś: { $count }
edit-project-save-invite = Zapisz i zaproś: { $count }
edit-project-you-are = W tym projekcie jesteś { $name }
edit-project-no-identity = Nie wybrano jeszcze, kim jesteś
edit-project-switch = Zmień
edit-project-choose = Wybierz
invite-failed = Nie udało się wysłać tych zaproszeń: { $emails }
invite-waiting = Ci znajomi muszą raz otworzyć Counted, zanim będzie można ich zaprosić: { $emails }
key-offer-title = Inny klucz do { $name }
key-offer-body = To urządzenie ma inny klucz do tego projektu, który nigdy go tu nie otworzył.
key-offer-choices = Użyj tego klucza: projekt otworzy się kluczem z tego linku lub zaproszenia. Anuluj: urządzenie zachowa obecny klucz i nic się nie zmieni.
key-offer-use = Użyj tego klucza
invite-again = Zaproś ponownie
friend-picker-title = Dodaj znajomych
user-selection-invited-hint = { $email } zaprasza Cię do „{ $project }”.
user-selection-suggested = Sugerowane
user-selection-suggested-sub = { $email } dodał(a) Cię pod tym imieniem
user-selection-confirm-as = To ja: { $name }
user-selection-missing = Nie ma tu Twojego imienia? Poproś uczestnika, by dodał Cię w ustawieniach projektu.

## Wydatki cykliczne

repeat-label = Powtarzaj
repeat-none = Nie powtarza się
repeat-weekly = Co tydzień
repeat-biweekly = Co 2 tygodnie
repeat-monthly = Co miesiąc
repeat-quarterly = Co 3 miesiące
repeat-yearly = Co rok
repeat-every-weeks = Co { $count } tyg.
repeat-every-months = Co { $count } mies.
repeat-every-years = Co { $count } lat(a)
repeat-custom = Niestandardowo…
repeat-every = Co
repeat-unit-weeks = tyg.
repeat-unit-months = mies.
repeat-unit-years = lat(a)
repeat-on-weekday = w dniu: { $weekday }
repeat-on-day = { $day }. dnia miesiąca
repeat-on-day-month = { $day } { $month }
repeat-month-end = W krótszych miesiącach wypada w ostatni dzień.
repeat-ends = Koniec
repeat-ends-never = Nigdy
repeat-ends-on = W dniu
repeat-ends-after = Po
repeat-fewer = Mniej
repeat-more = Więcej
repeat-last-on = ostatni: { $date }
repeat-variable = Kwota zmienia się za każdym razem
repeat-variable-hint = Każdy jest dodawany z ostatnią kwotą i oznaczany „do potwierdzenia”.
repeat-done = Gotowe
repeat-no-end = Bez końca
repeat-until = Do { $date }
repeat-occurrences = Liczba powtórzeń: { $count }
repeat-offline = Wymaga połączenia. Sam wydatek nadal można dodać.
repeat-foreign = Powtarza się jako { $amount } { $currency }, przeliczone raz po dzisiejszym kursie. Dostaniesz ostrzeżenie, jeśli kurs zmieni się o ponad 5%.
repeat-backfill = Zaczyna się w przeszłości. Wydatki dodane teraz: { $count }.
add-and-repeat = Dodaj i powtarzaj
weekday-1 = poniedziałek
weekday-2 = wtorek
weekday-3 = środa
weekday-4 = czwartek
weekday-5 = piątek
weekday-6 = sobota
weekday-7 = niedziela
recurring-title = Wydatki cykliczne
recurring-strip = Wydatki cykliczne: { $count }
recurring-next = Następny: { $name }, { $date }
recurring-to-confirm = Do potwierdzenia: { $count }
recurring-per-month = Miesięcznie, ok.
recurring-your-share = Twój udział
recurring-active = Aktywne
recurring-paused = Wstrzymane
recurring-finished = Zakończone
recurring-paid-by = płaci: { $name }
recurring-next-on = Następny: { $date }
recurring-progress = { $done } z { $total }
recurring-rate-badge = Kurs zmienił się o { $percent }%
recurring-empty = Nic się jeszcze nie powtarza. Wybierz „Powtarzaj” przy dodawaniu wydatku: czynsz, subskrypcje, rachunki.
recurring-next-ones = Kolejne
recurring-added-so-far = Dodane dotąd
recurring-set-up-by = Utworzył(a)
recurring-pause = Wstrzymaj
recurring-resume = Wznów
recurring-stop = Zakończ powtarzanie
recurring-stop-title = Zakończyć „{ $name }”?
recurring-stop-message = Przestanie się powtarzać. Już dodane wydatki zostają.
recurring-resume-title = Wznowić „{ $name }”?
recurring-resume-message = Następny: { $date }. Daty pominięte w czasie wstrzymania nie zostaną dodane.
recurring-edit-title = Edytuj wydatek cykliczny
recurring-edit-banner = Zmiany obowiązują od { $date }. Już dodane wydatki pozostają bez zmian.
recurring-next-on-label = Następny
recurring-next-too-early = Następna data musi być późniejsza niż ostatni dodany wydatek.
recurring-use-stop = Aby go zakończyć, użyj „Zakończ powtarzanie” w wydatku cyklicznym.
recurring-drift = Kurs { $currency } zmienił się o { $percent }% od utworzenia. Każdy jest nadal dodawany jako { $amount } { $project_currency } (1 { $currency } = { $rate }). Po dzisiejszym kursie byłoby to { $today_amount } { $project_currency }.
recurring-use-rate = Użyj dzisiejszego kursu
recurring-keep = Zachowaj { $amount } { $currency }
recurring-added = Dodane wydatki cykliczne: { $count }
recurring-blocks-removal = Nie można jeszcze usunąć: { $name }. Uczestnik należy do: { $rules }. Usuń { $name } z tych wydatków lub je zakończ, potem zapisz ponownie.
history-recurring-added = Dodano automatycznie: { $name } ({ $date })
history-recurring-created = Utworzono wydatek cykliczny: { $name }
history-recurring-edited = Edytowano wydatek cykliczny: { $name }
history-recurring-paused = Wstrzymano wydatek cykliczny: { $name }
history-recurring-resumed = Wznowiono wydatek cykliczny: { $name }
history-recurring-stopped = Zakończono wydatek cykliczny: { $name }
occurrence-recurring = Wydatek cykliczny
occurrence-auto = Dodany automatycznie przez wydatek cykliczny.
occurrence-auto-next = Dodany automatycznie przez wydatek cykliczny. Następny: { $date }.
occurrence-auto-stopped = Dodany automatycznie przez wydatek cykliczny, który został już zakończony.
occurrence-manage = Zarządzaj
estimate-badge = Do potwierdzenia
expense-pending = Oczekuje
estimate-title = Kwota do potwierdzenia.
estimate-body = Dodany z poprzednią kwotą. Wpisz prawdziwą, gdy ją poznasz.
estimate-confirm = Potwierdź kwotę
apply-to = Zastosuj do
apply-this-only = Tylko tego wydatku
apply-and-next = Tego i następnych
apply-and-next-hint = Wydatek cykliczny zmienia się od { $date }
apply-rule-failed = Wydatek został zapisany, ale wydatek cykliczny nie został zmieniony.
occurrence-delete-message = „{ $name }” z dnia { $date } zostanie trwale usunięty. Powtarzanie trwa dalej, a ta data nie wróci.
occurrence-delete-one = Usuń tylko ten
occurrence-delete-stop = Usuń i zakończ powtarzanie
error-recurring-clock = Zegar tego urządzenia się spieszy. Sprawdź datę i godzinę.
error-recurring-not-found = Ten wydatek cykliczny już nie istnieje.
error-recurring-stale = Ktoś w międzyczasie zmienił ten wydatek cykliczny. Został wczytany ponownie: sprawdź go i zapisz jeszcze raz.
error-too-many-recurring = Ten projekt ma już 50 wydatków cyklicznych. Zakończ niepotrzebny, aby dodać nowy.
error-user-in-recurring = Ten uczestnik należy do wydatku cyklicznego. Najpierw usuń go stamtąd lub zakończ wydatek.

home-owed-to-you = Należy ci się
home-you-owe = Jesteś winien
home-details-show = Pokaż szczegóły
home-details-hide = Ukryj szczegóły
home-all-settled = Wszystko rozliczone
home-pay = Zapłać
home-remind = Przypomnij
home-remind-message = Małe przypomnienie: w { $project } zostały jeszcze zwroty do rozliczenia.
home-row-owed = należy ci się
home-row-owe = jesteś winien
home-row-settled = Rozliczone
home-row-quits = U ciebie rozliczone
home-row-pending =
    { $count ->
        [one] { $count } oczekujący zwrot
        [few] { $count } oczekujące zwroty
        [many] { $count } oczekujących zwrotów
       *[other] { $count } oczekującego zwrotu
    }
home-totals-partial = Bez projektów bez kursu wymiany
error-account-key-changed = Klucz twojego konta zmienił się na innym urządzeniu. Zaloguj się ponownie, aby kontynuować.
settings-security = Bezpieczeństwo
settings-change-password = Zmień hasło
settings-change-password-hint = Twoje projekty i dane pozostają bez zmian. Twoje inne urządzenia zostaną wylogowane.
change-password-title = Zmień swoje hasło
change-password-current = Obecne hasło
change-password-new = Nowe hasło
change-password-confirm = Potwierdź nowe hasło
change-password-mismatch = Dwa nowe hasła nie są takie same.
change-password-submit = Zmień hasło
change-password-submitting = Zmieniam…
change-password-done = Hasło zmienione. Twoje inne urządzenia zostały wylogowane.
error-rotation-incomplete = Twoje konto zmieniło się w międzyczasie. Spróbuj ponownie.
error-key-lifecycle-unavailable = Jeszcze niedostępne: najpierw wszyscy potrzebują najnowszej wersji aplikacji.
settings-compromised = Ktoś może znać moje hasło
settings-compromised-hint = Nowe hasło, nowe klucze i wylogowanie wszystkich innych urządzeń.
settings-compromised-open = Zabezpiecz moje konto
compromised-title = Zabezpiecz swoje konto
compromised-body = Twoje hasło i klucze konta zostaną zastąpione, a wszystkie inne urządzenia wylogowane. Twoi znajomi zobaczą zmianę twojego kodu bezpieczeństwa: porównajcie go ponownie.
compromised-keeps = Osoba, która miała twoje hasło, zachowa wszystko, co już zostało otwarte: projekty, które dotąd dało się przeczytać.
compromised-submit = Zastąp hasło i klucze
compromised-done = Twoje konto jest zabezpieczone. Poproś znajomych o porównanie nowego kodu bezpieczeństwa.
friends-new-fingerprint = Nowy kod:
friends-code-matches = Kod się zgadza
friends-code-trusted = Kod potwierdzony.
settings-recovery-key = Klucz odzyskiwania
settings-recovery-key-none = Bez niego zapomniane hasło kosztuje cię ustawienia, dane płatności i kody znajomych, a odzyskanie trwa 3 dni.
settings-recovery-key-set = Utworzony. Jeśli zapomnisz hasła, odzyskasz nim konto z całą zawartością.
settings-recovery-key-pending = Nowy klucz zastąpi go { $when }. To nie ty? Anuluj obecnym kluczem.
recovery-key-create = Utwórz
recovery-key-replace = Zastąp
recovery-key-cancel-replacement = Anuluj nowy klucz
recovery-key-title = Twój klucz odzyskiwania
recovery-key-body = Zapisz go lub zachowaj tam, gdzie tylko ty masz dostęp. Counted nie może pokazać go ponownie ani odzyskać za ciebie.
recovery-key-copy = Kopiuj
recovery-key-copied = Skopiowano
recovery-key-save = Zapisz jako plik
recovery-key-print = Drukuj
recovery-key-next = Dalej
recovery-key-confirm-body = Aby sprawdzić kopię, wpisz grupę { $n } klucza.
recovery-key-confirm-wrong = Ta grupa się nie zgadza. Spójrz jeszcze raz na kopię.
recovery-key-password-body = Wpisz hasło, aby zapisać klucz na koncie.
recovery-key-submit = Zapisz klucz
recovery-key-done = Klucz odzyskiwania zapisany.
recovery-key-replacement-done = Nowy klucz zapisany. Zastąpi obecny za 72 godziny; do tego czasu obecny nadal działa.
recovery-key-field = Klucz odzyskiwania
recovery-key-file-intro = Klucz odzyskiwania Counted
recovery-key-cancel-done = Nowy klucz został anulowany.
compromised-new-key = Zapisz teraz nowy klucz odzyskiwania: zastąpi stary za 72 godziny, a do tego czasu stary nadal działa.
pending-reset-title = Poproszono o zresetowanie twojego konta
pending-reset-body = Wejdzie w życie { $when }: wtedy przepadną ustawienia, dane płatności i kody znajomych, a urządzenia będą musiały zalogować się ponownie. To nie ty? Anuluj obecnym hasłem.
pending-reset-cancel = Anuluj reset
pending-reset-cancelled = Reset został anulowany.
login-forgot = Nie pamiętasz hasła?
forgot-title = Nie pamiętasz hasła
forgot-body = Wpisz swój e-mail. Jeśli ma konto, wyślemy link do wyboru nowego hasła.
forgot-submit = Wyślij link
forgot-sent = Jeśli ten adres ma konto, link jest w drodze. Wygaśnie za 24 godziny.
reset-title = Odzyskaj swoje konto
reset-with-key-body = Wpisz klucz odzyskiwania i nowe hasło: wszystko wróci tak, jak było.
reset-submit-key = Odzyskaj moje konto
reset-no-key = Nie mam klucza odzyskiwania
reset-without-key-title = Zresetuj bez klucza odzyskiwania
reset-without-key-body = Twoje konto zacznie od nowa z nowym hasłem za 72 godziny. Projekty wrócą z twoich urządzeń i linków udostępniania; ustawienia, dane płatności i kody znajomych nie. Do tego czasu obecne hasło działa i może anulować reset.
reset-name = Twoje imię
reset-submit-request = Zresetuj moje konto
reset-requested = Gotowe. Nowe hasło zadziała od { $when }. Do tego czasu obecne hasło działa i może to anulować.
reset-pending-already = Reset już czeka do { $when }. Nowa prośba zaczyna 72 godziny od nowa.
recovered-done = Odzyskano konto. Inne urządzenia zostały wylogowane.
login-reset-undoable = To hasło zostało zastąpione resetem w ciągu ostatnich 30 dni. Jeśli o to nie prosiłeś, cofnij to: konto wróci takie, jakie było.
login-undo-reset = Cofnij reset
reset-undone = Reset został cofnięty. Konto jest takie jak wcześniej.
error-reset-pending = Nowe hasło zadziała, gdy reset wejdzie w życie, 72 godziny po prośbie.
error-account-disabled = To konto usunięto wkrótce po resecie i czeka na wymazanie. Poprzednie hasło nadal może cofnąć reset.
error-recovery-key-invalid = Ten klucz odzyskiwania nie otwiera tego konta.
error-project-key-required = Ten projekt wymaga swojego klucza. Otwórz go ponownie z linku udostępniania.
names-or = { $first } lub { $last }
participants-admin-badge = Admin
edit-project-admins = Admini: { $count }
edit-project-succession = Jeśli projekt zostanie bez admina, rolę przejmuje osoba z kontem, która należy do niego najdłużej.
member-sheet-account = Członek z kontem
member-actions = Działania: { $name }
member-make-admin = Nadaj rolę admina
member-make-admin-hint = Zyska możliwość zmiany klucza projektu i usuwania osób.
member-made-admin = { $name } ma teraz rolę admina.
group-security = Bezpieczeństwo projektu
group-security-admin = Bezpieczeństwo projektu · masz rolę admina
group-rotate = Zmień klucz projektu
group-rotate-hint = Jeśli link do projektu wyciekł: osoby ze starym linkiem stracą dostęp do projektu.
group-remove = Usuń kogoś z projektu
group-remove-hint = Odbiera dostęp i jednocześnie zmienia klucz.
group-admins-only = Tylko dla adminów.
group-admins-only-account = Tylko dla adminów, którzy mają konto.
group-ask-admin = Poproś admina projektu: { $names }.
group-ask-admins = Poproś adminów projektu: { $names }.
group-ask-an-admin = Poproś jednego z adminów projektu.
group-with-account = Z kontem
group-free = Za darmo
group-sign-in-hint = Admin projektu ({ $names }) może potem nadać ci tę rolę.
group-sign-in-hint-any = Admin projektu może potem nadać ci tę rolę.
group-no-admin = Ten projekt nie ma admina, więc jego klucza nie można zmienić.
group-hide = Ukryj kogoś
group-hide-hint = Ukrywa tę osobę na listach bez odbierania dostępu: link i klucz nadal działają.
group-recreate-hint = Ten projekt nie ma admina. Aby kogoś usunąć, utwórz nowy projekt.
group-recreate = Utwórz nowy projekt
group-no-admin-yet = Ten projekt nie ma jeszcze admina.
group-creator-hint = Tylko ty możesz zostać adminem tego projektu, zakładając konto.
group-creator-elsewhere-hint = Adminem projektu może zostać tylko osoba, która go utworzyła, zakładając konto. W przeciwnym razie utwórz nowy projekt, aby kogoś usunąć.
member-revoke-admin = Odbierz rolę admina
member-revoke-admin-hint = Straci możliwość zmiany klucza projektu i usuwania osób.
member-revoked-admin = { $name } nie ma już roli admina.
member-step-down = Przestań być adminem
member-step-down-hint = Stracisz możliwość zmiany klucza projektu i usuwania osób.
member-stepped-down = Nie masz już roli admina.
member-remove = Usuń z projektu
member-remove-hint = Zmienia klucz projektu: { $name } nie przeczyta już niczego nowego.
member-hide = Ukryj na listach
error-user-not-admin = Ta osoba nie jest adminem.
error-creator-proof-invalid = To urządzenie nie utworzyło tego projektu.
error-project-has-admin = Ten projekt ma już admina.
error-not-project-admin = Tylko admini projektu mogą to zrobić.
error-user-not-claimed = Ta osoba nie ma konta, więc nie może być adminem.
error-project-key-rotated = Klucz projektu się zmienił. Poproś członka o nowy link.
error-project-rotating = Klucz projektu jest właśnie zmieniany. Spróbuj ponownie za chwilę.
error-rotation-expired = Zmiana klucza trwała zbyt długo i została anulowana. Zacznij od nowa.
error-rotation-rows-changed = Projekt zmienił się podczas zmiany klucza. Zacznij od nowa.
project-rotated-title = Nie masz już dostępu do tego projektu
project-rotated-body = Klucz tego projektu został zmieniony przez admina. Poproś admina o nowy link.
project-rotated-open-link = Otwórz link
rotation-remove-title = Usunąć { $name } z projektu?
rotation-rotate-title = Zmienić klucz projektu?
rotation-remove-body = Usunięcie członka oznacza wygenerowanie nowego klucza. Nowy link trzeba będzie udostępnić osobom bez konta.
rotation-rotate-body = Osoby ze starym linkiem stracą dostęp do projektu. Nowy link trzeba będzie udostępnić osobom bez konta.
rotation-boxed =
    { $count ->
        [0] Klucz nie zostanie automatycznie wysłany do nikogo innego
        [one] Klucz zostanie automatycznie wysłany do { $count } członka
        [few] Klucz zostanie automatycznie wysłany do { $count } członków
        [many] Klucz zostanie automatycznie wysłany do { $count } członków
       *[other] Klucz zostanie automatycznie wysłany do { $count } członka
    }
rotation-by-link-title = Osoby, którym musisz wysłać nowy link
rotation-by-link-hint = Te osoby nie mają konta. Wyślesz im link zaraz potem.
rotation-names-and = { $first } i { $last }
rotation-names-and-others =
    { $names } i { $count ->
        [one] { $count } inna osoba
        [few] { $count } inne osoby
        [many] { $count } innych osób
       *[other] { $count } innej osoby
    }
rotation-confirm-remove = Usuń i zmień klucz
rotation-confirm-rotate = Zmień klucz
rotation-done-title = Nowy klucz działa
rotation-done-removed = { $name } nie należy już do projektu.
rotation-done-rotated = Klucz projektu został zmieniony.
rotation-done-boxed =
    { $count ->
        [0] Klucz nie został automatycznie wysłany do nikogo innego.
        [one] Klucz został automatycznie wysłany do { $count } członka.
        [few] Klucz został automatycznie wysłany do { $count } członków.
        [many] Klucz został automatycznie wysłany do { $count } członków.
       *[other] Klucz został automatycznie wysłany do { $count } członka.
    }
rotation-new-link =
    { $count ->
        [0] Nowy link do projektu
        [one] Nowy link do wysłania · { $count } osoba bez konta
        [few] Nowy link do wysłania · { $count } osoby bez konta
        [many] Nowy link do wysłania · { $count } osób bez konta
       *[other] Nowy link do wysłania · { $count } osoby bez konta
    }
rotation-link-renewed = Link został odnowiony.
rotation-done = Gotowe
history-key-changed = Zmieniono klucz projektu
history-member-removed = { $name } nie należy już do projektu
project-rotated-row-hint = Klucz został zmieniony. Poproś o nowy link.
