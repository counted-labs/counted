# Українська. Повна, за винятком юридичних текстів (legal-, terms-, privacy-), які існують лише
# англійською та французькою і для кожного повідомлення окремо повертаються до en.ftl.

### Common

loading = Завантаження…
cancel = Скасувати
confirm = Підтвердити
retry = Спробувати ще раз
delete = Видалити
back = Назад
language = Мова

### Navigation

nav-main = Головна навігація
nav-projects = Проєкти
nav-charts = Статистика
nav-settings = Налаштування

### Connectivity

offline-banner = Офлайн
offline-pending =
    { $count ->
        [one] { $count } в очікуванні
        [few] { $count } в очікуванні
        [many] { $count } в очікуванні
       *[other] { $count } в очікуванні
    }

sync-conflict-edit = Конфлікт: не вдалося змінити «{ $name }» (елемент видалено). Пропущено.
sync-conflict-delete = Конфлікт: не вдалося видалити «{ $name }» (елемент видалено). Пропущено.
sync-conflict-other = Конфлікт: операція з «{ $name }» не вдалася (елемент видалено). Пропущено.
sync-error = Помилка синхронізації: { $reason }

### Errors

error-network = Не вдається з’єднатися із сервером. Перевір з’єднання з інтернетом.
error-generic = Щось пішло не так. Спробуй ще раз.

error-invalid-email = Ця електронна адреса недійсна.
error-invalid-password = Цей пароль недійсний.
error-password-too-short = Пароль має містити щонайменше 8 символів.
error-client-outdated = Ця версія застосунку застаріла. Онови її, щоб увійти.
error-invalid-link = Це посилання недійсне.
error-batch-too-large = Забагато елементів за раз.
error-payers-required = Вибери принаймні одного платника.
error-debtors-required = Вибери принаймні одну особу, яка винна.
error-duplicate-participant = Учасник двічі з’являється з одного боку.
error-participant-not-in-project = Цей учасник не належить до цього проєкту.
error-too-many-participants = Забагато учасників для однієї витрати.
error-invalid-credentials = Неправильна електронна адреса або пароль.
error-unauthenticated = Увійди, щоб зробити це.
error-email-not-verified = Твою електронну адресу ще не підтверджено.
error-project-not-found = Цього проєкту більше не існує.
error-expense-not-found = Цієї витрати більше не існує.
error-storage-full = Сховище заповнене: ключ цього проєкту не вдалося зберегти на цьому пристрої. Збережи посилання для спільного доступу.
error-user-not-found = Цього учасника більше не існує.
error-tricount-not-found = Tricount не знайдено, або його API повернув помилку.
error-too-many-members = Цей проєкт досяг ліміту учасників.
error-identity-taken = Інший обліковий запис уже закріпив за собою цього учасника.
error-claim-proof-invalid = На цьому пристрої немає ключа проєкту, тому він не може закріпити учасника. Відкрий посилання для спільного доступу ще раз.
error-user-has-payments = У цього учасника є витрати в проєкті, тому його не можна видалити.
error-resend-cooldown = Зачекай 60 секунд, перш ніж запитувати новий лист.
error-self-friend-request = Не можна додати себе в друзі.
error-not-a-friend = Запрошувати можна лише людей зі списку друзів.
error-friend-has-no-key = Цей друг ще не відкривав останню версію застосунку. Попроси його один раз увійти, а потім спробуй ще раз.
error-friend-request-not-found = Цього запиту в друзі більше не існує.
error-invitation-not-found = Цього запрошення більше не існує.
error-too-many-friend-requests = Наразі забагато запитів у друзі. Спробуй завтра.
error-too-many-invitations = Забагато запрошень в очікуванні.
error-invalid-kdf-salt = Налаштування шифрування недійсні. Онови застосунок і спробуй ще раз.
error-mixed-project-batch = Ці учасники не всі в одному проєкті.
error-invalid-payload = Ця версія застосунку надіслала дані, які сервер не приймає. Онови її і спробуй ще раз.
error-invalid-public-key = Твій ключ шифрування недійсний. Онови застосунок і спробуй ще раз.
error-payment-methods-stale = Твої платіжні дані змінено на іншому пристрої. Перезавантаж і спробуй ще раз.

### Auth

field-email = Електронна пошта
field-email-placeholder = ти@приклад.ua
field-password = Пароль
field-name = Ім’я
field-name-placeholder = Олена Шевченко

login-title = Увійти
login-submit = Увійти
login-submitting = Вхід…
login-password-placeholder = Твій пароль
login-no-account = Ще немає облікового запису?
login-unverified = Твою електронну адресу ще не підтверджено. Перевір скриньку або надішли посилання ще раз.
login-resend = Надіслати посилання для підтвердження ще раз
login-resending = Надсилання…
login-resend-sent = Лист надіслано - перевір скриньку.

register-submit = Створити обліковий запис
register-submitting = Створення…
register-have-account = Уже є обліковий запис?
register-password-placeholder = Щонайменше 8 символів
register-password-warning = Запиши свій пароль. Якщо забудеш його, обліковий запис не вдасться відновити.
register-check-email-title = Перевір пошту
register-email-sent = Лист надіслано
register-email-sent-hint = Натисни посилання в листі, щоб активувати обліковий запис.
register-not-received-prefix = Не отримав? Перевір папку зі спамом або
register-sign-in-link = увійди
register-not-received-suffix = щоб надіслати посилання ще раз.
register-terms-prefix = Створюючи обліковий запис, ти приймаєш наші
register-terms-link = умови використання
register-terms-and = та нашу
register-privacy-link = політику конфіденційності

settings-title = Налаштування
settings-preferences = Уподобання
settings-preferences-local = Збережено на цьому пристрої.
settings-preferences-synced = Синхронізовано з твоїм обліковим записом, зашифровано.
settings-about = Про застосунок
settings-anonymous-title = Ти не ввійшов
settings-upsell-title = Твої проєкти на кожному пристрої
settings-upsell-free = Безкоштовно
settings-upsell-body = Counted працює без облікового запису. З безкоштовним обліковим записом твої проєкти та вподобання йдуть за тобою на телефон, ноутбук і в браузер - так само зашифровані, так само нечитабельні для нас.
settings-locked-badge = Обліковий запис
settings-locked-friends = Створи обліковий запис, щоб додавати друзів і запрошувати їх у проєкт із застосунку - без передавання посилання.
settings-locked-payment-methods = Збережи свій IBAN або платіжний застосунок один раз і поділися ним із вибраними проєктами. Той, хто тобі винен, побачить його поруч із твоїм ім’ям.
settings-friends-hint = Додавай друзів і запрошуй їх у свої проєкти без поширення посилання.

account-member-since = Учасник з
account-logout = Вийти
account-logging-out = Вихід…
account-delete-title = Видалити мій обліковий запис
account-delete-warning = Негайно й назавжди, без кошика. Витрати, які ти вніс у спільний проєкт, залишаться видимими іншим учасникам - вони є частиною їхніх рахунків.
account-delete-confirm-title = Видалити обліковий запис
account-delete-confirm-message = Твій обліковий запис, сеанси та список проєктів буде видалено назавжди. Без пароля зашифровані дані спільного проєкту стануть для тебе нечитабельними - це неможливо скасувати.

settings-payment-methods = Платіжні реквізити
settings-payment-methods-hint = Як ти хочеш отримувати повернення. Зашифровано з твоїм обліковим записом.
payment-method-kind = Спосіб
payment-method-kind-other = Інший
payment-method-label = Назва
payment-method-label-placeholder = Основний рахунок
payment-method-value = Реквізити
payment-method-value-placeholder = IBAN, номер телефону, ім’я користувача…
payment-method-add = Додати
payment-method-remove = Видалити { $name }
payment-method-empty = Ти ще не додав платіжних реквізитів.
payment-method-deleted = Спосіб оплати видалено.
payment-method-value-required = Заповни реквізити кожного способу оплати або видали його.
payment-method-label-required = Дай назву своєму власному способу.
payment-method-too-long = Це задовго - скороти.
payment-method-invalid-characters = Прибери розриви рядків або невидимі символи.
payment-method-limit = Можна зберегти до { $max } способів оплати.
payment-methods-saved = Платіжні реквізити збережено.
payment-methods-offline = Щоб зберегти платіжні реквізити, потрібне з’єднання з інтернетом.
payment-methods-stale = Твої платіжні реквізити було змінено на іншому пристрої. Їх перезавантажено — спробуй ще раз.
payment-methods-key-missing = Увійди ще раз, щоб керувати платіжними реквізитами.
settings-payment-methods-share-warning = Спільний спосіб бачить кожен учасник проєктів, у яких ти вибрав своє ім’я - будь-хто, у кого є одне з цих посилань.
payment-method-share = Поділитися з моїми проєктами
payment-method-share-hint = Показується поруч із твоїм ім’ям, коли хтось тобі винен.
payment-method-copy = Копіювати { $name }
payment-method-copied = Скопійовано.
payment-method-copy-failed = Не вдалося скопіювати - виділи текст і скопіюй його вручну.

verify-email-checking = Підтвердження електронної адреси…
verify-email-welcome = Адресу підтверджено - ласкаво просимо до Counted!
verify-email-back-to-login = Назад до входу

### Project status

project-close = Закрити
project-archive = Архівувати
project-reopen = Відкрити знову
project-unarchive = Розархівувати
project-sheet-invite = Запросити
project-sheet-recurring = Регулярні
project-sheet-edit = Змінити проєкт
project-sheet-close = Закрити проєкт
project-sheet-archive = Архівувати проєкт
project-sheet-reopen = Відкрити проєкт знову
project-sheet-unarchive = Розархівувати проєкт
project-sheet-leave = Вийти з проєкту

### Dates

date-long = { $day } { $month } { $year }

month-1 = січня
month-2 = лютого
month-3 = березня
month-4 = квітня
month-5 = травня
month-6 = червня
month-7 = липня
month-8 = серпня
month-9 = вересня
month-10 = жовтня
month-11 = листопада
month-12 = грудня

month-short-1 = січ
month-short-2 = лют
month-short-3 = бер
month-short-4 = кві
month-short-5 = тра
month-short-6 = чер
month-short-7 = лип
month-short-8 = сер
month-short-9 = вер
month-short-10 = жов
month-short-11 = лис
month-short-12 = гру

### Actions

add = Додати
create = Створити
creating = Створення…
edit = Змінити
leave = Вийти
close = Закрити
paste = Вставити
join = Приєднатися
import = Імпортувати
importing = Імпортування…
field-description = Опис
field-date = Дата
date-today = Сьогодні
date-yesterday = Вчора
field-optional = Необов’язково

### Projects

projects-filter-active = Активні
projects-filter-all = Усі
projects-count-label = Проєкти
projects-empty = Немає проєктів
projects-empty-hint = Створи проєкт кнопкою нижче
projects-offline-banner = Дані офлайн - під’єднайся, щоб оновити.
demo-banner = Демонстраційний проєкт - лише для читання.
demo-start-own = Створити власний проєкт
projects-no-local-data = Немає локальних даних
projects-no-local-data-hint = Увійди, щоб уперше завантажити свої проєкти.
projects-add = Додати проєкт
projects-create = Створити проєкт
projects-join = Приєднатися до проєкту
projects-import-tricount = Імпортувати з Tricount
project-actions = Дії з проєктом

status-ongoing = Триває
status-closed = Закрито
status-archived = В архіві

nav-help = Довідка
nav-privacy = Політика конфіденційності
nav-terms = Умови використання
nav-legal = Юридична інформація

leave-project-title = Вийти з проєкту?
leave-project-message = Ти втратиш доступ із цього пристрою. Якщо не залишиться жодного учасника, проєкт і всі його витрати буде видалено назавжди.

add-project-title = Новий проєкт
add-project-name-label = Назва проєкту
add-project-name-placeholder = Моя подорож, Квартира 2024…
add-project-participants = Учасники
add-project-participant-name = Ім’я учасника
add-project-participant-placeholder = Кларк Кент
add-project-offline = Офлайн не можна створити проєкт. Під’єднайся і спробуй ще раз.
add-project-name-required = Проєкту потрібна назва.

join-link-label = Посилання для спільного доступу
join-link-hint = Посилання містить ключ розшифрування - скопіюй його повністю.
join-invalid-link = Це посилання недійсне. Встав повне посилання, включно з частиною після #.
join-wrong-project = Це посилання від іншого проєкту.

import-tricount-link-label = Посилання або ключ Tricount
import-tricount-key-required = Введи посилання або ключ Tricount.
import-tricount-encryption-failed = Шифрування не вдалося.
import-tricount-unimportable = Нічого не імпортовано: цей Tricount має учасників з обліковим записом Tricount або суми, що не сходяться (зачеплені записи: { $count }).

### Expenses

save = Зберегти
saving = Збереження…
adding = Додавання…
link-copied = Посилання скопійовано
missing-encryption-key = Відсутній ключ шифрування.
missing-encryption-key-title = Відсутній ключ шифрування
missing-encryption-key-hint = Використане посилання не містить ключа, потрібного для розшифрування цього проєкту. Скористайся повним посиланням від того, хто його створив.
project-locked-hint = На цьому пристрої немає ключа цього проєкту. Відкрий його посилання для спільного доступу, щоб розблокувати.
project-unlock = Розблокувати
project-no-local-data-hint = Увійди, щоб уперше завантажити дані цього проєкту.
project-gone-title = Цього проєкту більше не існує
project-gone-hint = Його було видалено, коли вийшов останній учасник. Посилання більше не працює, навіть якщо відкрити його знову.

expense-add = Додати витрату
transfer-add = Додати переказ
expense-edit-title = Змінити витрату
expense-category = Категорія
expense-category-auto = Авто · { $emoji }
expense-currency = Валюта суми
amount-op-add = Плюс
amount-op-subtract = Мінус
amount-op-multiply = Помножити
amount-op-divide = Поділити
amount-op-equals = Дорівнює
amount-op-done = Готово
expense-rate = Курс обміну (необов’язково)
expense-rate-hint = Залиш порожнім, щоб застосувати курс Європейської комісії (InforEuro) за { $month }: 1 { $from } = { $rate } { $to }.
expense-rate-invalid = Введи курс обміну, більший за 0.
expense-rate-unavailable = Автоматичний курс недоступний - введи його вручну.
expense-delete-title = Видалити витрату
expense-delete-message = «{ $name }» буде видалено назавжди. Це неможливо скасувати.
expense-inconsistent-amounts = Суми не сходяться
expenses-empty = Немає витрат
expenses-empty-hint = Почни з додавання витрат кнопкою нижче
expenses-show-more = Показати більше (ще { $count })

expense-type-expense = Витрата
expense-type-transfer = Переказ
expense-type-gain = Надходження
expense-paid-by = заплатив(ла)
expense-sent-by = надіслав(ла)
expense-contributed-by = вніс(ла)

expense-name-required = Потрібна назва.
expense-amount-not-positive = Сума має бути більшою за 0.
expense-no-payer = Вибери принаймні одного платника.
expense-no-debtor = Вибери принаймні одну особу, яка винна.
expense-invalid-date = Ця дата недійсна.
expense-payers-mismatch = Сума платників становить { $sum }, що не збігається із сумою витрати ({ $total }).
expense-debtors-mismatch = Сума боржників становить { $sum }, що не збігається із сумою витрати ({ $total }).

participants-none = Ніхто
participants-everyone = Усі ({ $count })
participants-some = { $count } з { $total }
participants-select-all = Вибрати всіх
participants-by-shares = За частками
split-amounts = Суми
participants-remaining = Залишилось { $amount }
participants-over-by = На { $amount } більше
participants-who-paid = Хто заплатив?
participants-who-received = Хто отримав?
participants-who-transfers = Хто переказує?
participants-who-receives = Хто отримує?
participants-for-whom = Для кого?

stats-total-expenses = Загальні витрати
stats-my-expenses = Мої витрати

tab-expenses = Витрати
tab-balance = Баланс
tab-reimbursements = Розрахунок
balance-gets-back = Має отримати
balance-owes = Має віддати
balance-settled = Розраховано
reimbursements-empty-title = Усе розраховано!
reimbursements-empty-hint = Пропозиції розрахунку з’являться тут, коли рахунки не зійдуться
reimbursement-owes = { $debtor } винен { $creditor }
reimbursement-record = Розрахуватися
reimbursement-pay-with = Заплатити
reimbursement-pay-shared-by = Поділився { $name } - перед надсиланням перевір ім’я отримувача, яке показує твій застосунок.
reimbursement-pay-title = Заплатити { $name }
reimbursements-mine-title = Ти винен
reimbursements-others-title = Інші повернення
copy = Копіювати

user-selection-title = Який учасник - ти?
user-selection-hint = Вибери своє ім’я зі списку.
user-selection-required = Вибери учасника.
identity-claimed = Пов’язано з обліковим записом
identity-claimed-by = Обліковий запис { $name }
identity-taken-repick = Інший обліковий запис закріпив за собою учасника, якого ти використовував. Вибери іншого.
participant-gone-repick = Учасника, якого ти використовував, видалено з цього проєкту. Вибери іншого.

edit-project-title = Змінити проєкт
edit-project-new-badge = новий
edit-project-deferred-new-members = додавання нових учасників
edit-project-deferred-removals = видалення учасників
edit-project-offline-deferred = Офлайн: { $items } буде застосовано після під’єднання.

export-failed = Експорт не вдався: { $reason }

history-expense-added = Витрату додано: { $name }
history-expense-edited = Витрату змінено: { $name }
history-expense-deleted = Витрату видалено: { $name }
history-project-edited = Проєкт змінено: { $name }
history-name-changed = Назва: «{ $from }» → «{ $to }»
history-description-added = Опис додано: «{ $value }»
history-description-removed = Опис видалено: «{ $value }»
history-description-changed = Опис: «{ $from }» → «{ $to }»

### Sweep

field-amount = Сума
expense-name-placeholder = Ресторан, продукти…
expense-actions = Дії з витратою
expense-your-share = Твоя частка
expense-your-share-value = Твоя частка: { $amount } { $currency }
expense-inconsistent-detail = Суми не сходяться: заплачено { $paid }, винні { $owed }, для витрати { $total }. Зміни витрату, щоб виправити.
missing-access-key = Відсутній ключ доступу. Відкрий цей проєкт через його посилання для спільного доступу.
filter-all = Усі
filter-my-payments = Мої платежі
filter-my-debts = Що я винен
participants-shares-for = Частки для { $name }
participants-amount-for = Сума для { $name }
reimbursement-add = Додати розрахунок
project-forget = Видалити з мого списку
project-history-title = Історія
history-kind-add = Додано
history-kind-delete = Видалено
history-kind-edit = Змінено
export = Експортувати
export-json = Експортувати JSON
export-csv = Експортувати CSV
share-link = Поділитися
copy-link-failed = Не вдалося скопіювати посилання
open-in-app = Відкрити в застосунку
not-found-title = Сторінку не знайдено
not-found-back = Назад до проєктів

### Charts

charts-period = Період
period-all = Усе
period-month = Місяць
period-3months = 3 міс.
period-year = Рік
period-custom = Свій
charts-tab-categories = Категорії
charts-tab-trends = Тренди
charts-total-spent = Витрачено загалом
charts-avg-per-person = Сер. на особу
charts-expense-count =
    { $count ->
        [one] { $count } витрата
        [few] { $count } витрати
        [many] { $count } витрат
       *[other] { $count } витрати
    }
charts-nothing-to-show = Нічого показувати
charts-my-share-note = Ці суми - твоя частка в кожній витраті.
charts-my-share-skipped =
    { $count ->
        [one] { $count } проєкт не враховано — не вибрано учасника, або його дані не завантажилися.
        [few] { $count } проєкти не враховано — не вибрано учасника, або їхні дані не завантажилися.
        [many] { $count } проєктів не враховано — не вибрано учасника, або їхні дані не завантажилися.
       *[other] { $count } проєкту не враховано — не вибрано учасника, або їхні дані не завантажилися.
    }

### Categories

category-food = Їжа
category-transport = Транспорт
category-accommodation = Житло
category-leisure = Дозвілля
category-shopping = Покупки
category-services = Послуги
category-parties-gifts = Свята та подарунки
category-other = Інше
charts-project = Проєкт
charts-all-projects = Усі проєкти
charts-date-from = З
charts-date-to = По
charts-total = Разом
charts-tab-people = Люди
charts-tab-projects = Проєкти
charts-scope = Чиї витрати
charts-scope-group = Група
charts-scope-me = Я
charts-currency = Валюта
charts-my-share = Моя частка
charts-share-of-total = { $pct }% від { $total }
charts-i-paid = Я заплатив
charts-paid-more = На { $amount } більше за твою частку
charts-paid-less = На { $amount } менше за твою частку
charts-paid-even = Рівно твоя частка
charts-part-title = Твоя частка в кожній категорії
charts-part-desc = Сірим — що витратила група, кольором — що спожив ти.
charts-breakdown-title = Розподіл за категоріями
charts-breakdown-desc = Торкнися сектора чи рядка, щоб побачити витрати.
charts-of-total = { $amount } з { $total }
charts-show-all = Показати все ({ $count })
charts-show-less = Показати менше
charts-spend-title = Витрати в часі
charts-spend-desc = Короткі періоди — по днях, довші — по тижнях чи місяцях.
charts-group-by = Групувати за
bucket-day = День
bucket-week = Тиждень
bucket-month = Місяць
charts-avg = сер.
charts-cat-title-day = { $category }, день за днем
charts-cat-title-week = { $category }, тиждень за тижнем
charts-cat-title-month = { $category }, місяць за місяцем
charts-cat-desc = Обери категорію, щоб стежити за нею в часі.
charts-running-title = Наростаючий підсумок
charts-running-desc = Від { $date }.
charts-avg-per-day = { $amount } / день у середньому
charts-avg-per-week = { $amount } / тиждень у середньому
charts-avg-per-month = { $amount } / місяць у середньому
charts-people-title = Хто тягнув групу
charts-people-desc = Скільки кожен заплатив, поруч із тим, скільки спожив.
charts-paid = Заплачено
charts-fair-share = Справедлива частка
charts-you = (ти)
charts-net-more = заплатив більше
charts-net-less = заплатив менше
charts-balance-title = Твій баланс у часі
charts-balance-desc = Над лінією група винна тобі. Під нею ти винен групі.
charts-owed = Тобі винні
charts-owe = Ти винен
charts-projects-title = Твоя частка за проєктами
charts-projects-desc = Суми ведуться за валютами й ніколи не додаються.
history-empty = Немає подій
history-by = { $name }
not-found-hint = Цієї сторінки не існує, або її перемістили.
payers-title-paid-by = Заплатив(ла)
payers-title-sender = Відправник
payers-title-contributors = Внесли
debtors-title-debtors = Винні
debtors-title-recipients = Отримувачі
debtors-title-beneficiaries = Бенефіціари

### Welcome

welcome-title = Твої рахунки - нікого не стосуються.
welcome-subtitle = Діліть витрати з друзями.
welcome-note = Безкоштовно. Без облікового запису. Без реклами.
welcome-link-title = Одне посилання — і всі долучаються.
welcome-link-body = Нікому не потрібно створювати обліковий запис.
welcome-link-account = Обліковий запис? Ніколи не обов'язковий. Він потрібен, щоб знаходити свої проєкти на іншому пристрої, запрошувати друзів із застосунку й ділитися своїми платіжними реквізитами.
welcome-demo-project = Вихідні в Ліоні
welcome-private-title = Ніхто не може прочитати твої рахунки. Навіть ми.
welcome-private-body = Імена, суми, проєкти: усе шифрується на твоєму пристрої. Ключ є лише в тебе.
welcome-private-names = Імена
welcome-private-amounts = Суми
welcome-private-projects = Проєкти
welcome-scan-title = Сфотографуй чек.
welcome-scan-body = Сума, дата й категорія заповнюються самі. Усе відбувається на твоєму телефоні. Фото не зберігається.
welcome-eu-title = 100% європейський
welcome-no-ads = Без реклами
welcome-no-trackers = Без трекерів
welcome-step = Крок { $current } з { $total }
welcome-next = Далі
welcome-skip = Пропустити
welcome-start = Почати
welcome-how-it-works = Як саме це працює?

### Help

help-intro = Поширене запитання? Торкнись, щоб розгорнути відповідь.
help-create-project-q = Як створити проєкт?
help-create-project-a = На головному екрані торкнись кнопки + внизу. Дай проєкту назву, вибери валюту - і готово.
help-add-participants-q = Як додати учасників?
help-add-participants-a = Відкрий проєкт і додай учасників зі списку. Кожен учасник може платити за витрату або бути винним за неї.
help-share-project-q = Як поділитися проєктом?
help-share-project-a = Поділися URL-адресою проєкту (тією, що в адресному рядку). Будь-хто з посиланням може переглядати й змінювати проєкт.
help-add-expense-q = Як додати витрату?
help-add-expense-a = У проєкті торкнись +, введи суму, вкажи, хто заплатив і між ким розділити. Можна також вибрати дату, відмінну від сьогоднішньої.
help-types-q = Чим відрізняються витрата, переказ і надходження?
help-types-expense = - покупка однієї особи, розділена між кількома.
help-types-transfer = - повернення від однієї особи іншій, без розподілу.
help-types-gain = - отримані гроші (повернення, подарунок), які розділяють між кількома особами.
help-past-date-q = Чи можна датувати витрату минулим?
help-past-date-a = Так, поле дати вільне. Час створення запису зберігається окремо.
help-who-owes-q = Як Counted визначає, хто кому винен?
help-who-owes-a = Counted обчислює чистий баланс кожного учасника (скільки він заплатив мінус скільки винен), а потім пропонує найкоротшу серію переказів, яка розраховує всіх.
help-minimal-transfers-q = Чому кількість запропонованих переказів мінімальна?
help-minimal-transfers-a = Алгоритм спочатку поєднує баланси, які точно взаємно знищуються, а потім проходить решту від найбільшого кредитора до найбільшого боржника. Результат: менше переказів, щоб усе розрахувати.
help-import-tricount-q = Як імпортувати проєкт із Tricount?
help-import-tricount-a = На головному екрані торкнись кнопки «+» внизу, а потім
help-import-tricount-b = Встав посилання для спільного доступу до Tricount, який хочеш імпортувати.
help-encryption-q = Чи зашифровані мої дані?
help-encryption-a = Так. Counted поєднує дві гарантії:
help-encryption-e2ee-term = Наскрізне шифрування
help-encryption-e2ee-def = - усе між тобою та сервером передається зашифрованим.
help-encryption-zero-term = Нульовий доступ
help-encryption-zero-def = - ти шифруєш дані перед надсиланням, а сервер зберігає лише шифротекст. Ми не маємо способу його прочитати.
help-encryption-see = Подробиці дивись у
help-forgot-password-q = Що буде, якщо я забуду пароль?
help-forgot-password-warning = Твої дані буде втрачено назавжди.
help-forgot-password-a = Ключ шифрування виводиться з твого пароля, тому скидання неможливе: ніхто - і ми теж - не зможе розшифрувати твої проєкти без нього. Зберігай його надійно, найкраще в менеджері паролів.
help-archive-delete-q = Як архівувати або видалити проєкт?
help-archive-delete-a = На екрані проєкту відкрий меню й вибери
help-archive-delete-b = щоб сховати його, зберігши. Проєкт видаляється назавжди, коли з нього виходить останній учасник.
help-delete-account-q = Як видалити обліковий запис?
help-delete-account-a = Відкрий Налаштування і скористайся «Видалити мій обліковий запис». Це відбувається негайно і не скасовується.
help-contact = Інше запитання? Напиши нам на

# Receipt scanning (mobile only)
expense-scan = Сканувати чек
scan-in-progress = Читання чека…
scan-error-capture = Не вдалося зробити фото. Спробуй ще раз або введи витрату вручну.
scan-error-unreadable = На цьому чеку нічого не читається. Введи витрату вручну.
scan-check-amount = Перевір підсумок - його надруковано нечітко.
scan-take-photo = Зробити фото
scan-choose-photo = Вибрати фото
expense-converted-from = Сплачено { $amount } { $from } · 1 { $from } = { $rate } { $to }
project-currency = Валюта
project-currency-hint = Усі суми показуються в цій валюті. Пізніше її не можна змінити.
project-currency-locked = Валюта фіксується під час створення проєкту.
currency-search = Пошук валюти

update-required-title = Потрібне оновлення
update-required-body = Ця версія Counted застара для зв’язку із сервером. Онови її, щоб продовжити користуватися застосунком.
update-required-button = Оновити

notifications-label = Сповіщення
notifications-title = Сповіщення
notifications-empty = Нічого нового
notifications-friend-request = Запит у друзі

friends-title = Друзі
friends-anonymous-body = Друзі прив’язані до твого облікового запису. Увійди, щоб додавати людей і запрошувати їх у свої проєкти без поширення посилання.
friends-add-title = Додати друга
friends-add-hint = Він побачить твій запит після входу. Ніхто з вас не дізнається, чи є в іншого обліковий запис, доки запит не буде прийнято.
friends-add-button = Додати
friends-add-from-project = Додати в друзі
friends-request-sent = Запит надіслано
friends-no-account-key = Увійди ще раз, щоб керувати друзями на цьому пристрої.
friends-incoming-title = Запити
friends-accept = Прийняти
friends-decline = Відхилити
friends-list-title = Мої друзі
friends-list-empty = Друзів ще немає. Додай когось за електронною поштою вище або зі спільного проєкту.
friends-remove = Видалити
friends-remove-confirm-title = Видалити друга
friends-remove-confirm-message = { $email } більше не буде у ваших друзях, а ви — у його. Будь-хто з вас зможе надіслати новий запит пізніше.
friends-no-key = Ще не готово
friends-fingerprint = Код безпеки
friends-fingerprint-hint = Двоє друзів, які прочитали одне одному однаковий код безпеки, знають, що між ними ніхто не стоїть - навіть наш сервер.
friends-outgoing-title = Надіслані
friends-outgoing-hint = Очікує відповіді. Вони з’являться серед друзів, щойно приймуть.
friends-withdraw = Скасувати
invite-friends-title = Запросити друзів
invite-friends-hint = Ключ проєкту шифрується для кожного друга на цьому пристрої. Сервер ніколи його не бачить.
invite-friends-empty = Ще немає друзів, яких можна запросити.
invite-friends-button = Запросити
invite-sent = { $count ->
    [one] Запрошення надіслано
    [few] Надіслано { $count } запрошення
    [many] Надіслано { $count } запрошень
   *[other] Надіслано { $count } запрошення
}
invitation-badge = Запрошення
invitation-to = Приєднатися до «{ $name }»
invitation-to-unnamed = Приєднатися до проєкту
invitation-unreadable = Це запрошення не можна відкрити на цьому пристрої
invitation-from = Від { $email }
invitation-accept = Приєднатися
invitation-decline = Відхилити

# Participants in the create and edit modals, and the "who are you?" picker - see
# docs/plans/friends.md §11.
participants-you-label = Твоє ім’я в цьому проєкті
participants-you-badge = Ти
participants-you-from-account = Взято з імені твого облікового запису. Зміни його тут лише для цього проєкту.
participants-you-required = Обов’язково. Так тебе бачитимуть інші.
participants-others = Інші учасники
participants-empty = Поки нікого. Вибери друга нижче або введи будь-яке ім’я.
participants-empty-signed-out = Поки нікого. Введи ім’я, щоб когось додати.
participants-duplicate = «{ $name }» уже є в списку.
participants-input-label = Додати друга або ввести ім’я
participants-input-placeholder = Друг або будь-яке ім’я
participants-suggest-friend = Друг · приєднається як «{ $name }», отримає запрошення
participants-suggest-not-ready = Друг · ще не готовий
participants-suggest-guest = Додати «{ $text }» без облікового запису
participants-suggest-guest-sub = Без облікового запису, лише ім’я
participants-friends = Твої друзі
participants-all-friends = Усі друзі
participants-login-hint = Увійди, щоб додавати людей просто зі списку друзів.
participants-invite-badge = Запросити
participants-guest-badge = Без облікового запису
participants-guest-sub = Без облікового запису, лише ім’я
participants-rename = Перейменувати: { $name }
participants-remove = Прибрати: { $name }
participants-rename-label = Нове ім’я
participants-rename-save = Зберегти ім’я
participants-rename-hint = Ім’я, яке всі бачать у цьому проєкті. Запрошення все одно піде на { $email }.
participants-invited-badge = Запрошено
participants-invited-sub = { $email } · ще не прийнято
participants-invited-pending = Запрошення ще не прийнято
participants-unlinked = Не пов’язано з обліковим записом
add-project-create-invite = Створити й запросити: { $count }
edit-project-save-invite = Зберегти й запросити: { $count }
edit-project-you-are = На цьому пристрої ти { $name }
edit-project-no-identity = Ти ще не вибрав(-ла), хто ти
edit-project-switch = Змінити
edit-project-choose = Вибрати
invite-failed = Не вдалося надіслати ці запрошення: { $emails }
invite-again = Запросити знову
friend-picker-title = Додати друзів
user-selection-invited-hint = { $email } запрошує тебе до «{ $project }».
user-selection-suggested = Запропоновано
user-selection-suggested-sub = { $email } додав(-ла) тебе під цим ім’ям
user-selection-confirm-as = Я { $name }
user-selection-missing = Твого імені немає? Попроси учасника додати тебе в налаштуваннях проєкту.

## Регулярні витрати

repeat-label = Повторювати
repeat-none = Не повторюється
repeat-weekly = Щотижня
repeat-biweekly = Кожні 2 тижні
repeat-monthly = Щомісяця
repeat-quarterly = Кожні 3 місяці
repeat-yearly = Щороку
repeat-every-weeks = Кожні { $count } тиж.
repeat-every-months = Кожні { $count } міс.
repeat-every-years = Кожні { $count } р.
repeat-custom = Власний варіант…
repeat-every = Кожні
repeat-unit-weeks = тиж.
repeat-unit-months = міс.
repeat-unit-years = р.
repeat-on-weekday = день: { $weekday }
repeat-on-day = { $day }-го числа
repeat-on-day-month = { $day } { $month }
repeat-month-end = У коротших місяцях припадає на останній день.
repeat-ends = Закінчується
repeat-ends-never = Ніколи
repeat-ends-on = У дату
repeat-ends-after = Після
repeat-fewer = Менше
repeat-more = Більше
repeat-last-on = остання { $date }
repeat-variable = Сума щоразу змінюється
repeat-variable-hint = Кожна додається з останньою сумою й позначкою «підтвердити».
repeat-done = Готово
repeat-no-end = Без кінця
repeat-until = До { $date }
repeat-occurrences = Кількість повторів: { $count }
repeat-offline = Потрібне з’єднання. Саму витрату все одно можна додати.
repeat-foreign = Повторюється як { $amount } { $currency }, перераховано один раз за сьогоднішнім курсом. Ти отримаєш попередження, якщо курс зміниться більш ніж на 5 %.
repeat-backfill = Починається в минулому. Витрат додано зараз: { $count }.
add-and-repeat = Додати й повторювати
weekday-1 = понеділок
weekday-2 = вівторок
weekday-3 = середа
weekday-4 = четвер
weekday-5 = пʼятниця
weekday-6 = субота
weekday-7 = неділя
recurring-title = Регулярні витрати
recurring-strip = Регулярні витрати: { $count }
recurring-next = Наступна: { $name }, { $date }
recurring-to-confirm = Підтвердити: { $count }
recurring-per-month = На місяць, приблизно
recurring-your-share = Твоя частка
recurring-active = Активні
recurring-paused = Призупинені
recurring-finished = Завершені
recurring-paid-by = платить { $name }
recurring-next-on = Наступна { $date }
recurring-progress = { $done } з { $total }
recurring-rate-badge = Курс змінився на { $percent } %
recurring-empty = Поки нічого не повторюється. Обери «Повторювати», коли додаєш витрату: оренда, підписки, рахунки.
recurring-next-ones = Наступні
recurring-added-so-far = Додано досі
recurring-set-up-by = Створено
recurring-pause = Призупинити
recurring-resume = Відновити
recurring-stop = Припинити повторення
recurring-stop-title = Припинити «{ $name }»?
recurring-stop-message = Вона більше не повторюватиметься. Уже додані витрати залишаться.
recurring-resume-title = Відновити «{ $name }»?
recurring-resume-message = Наступна { $date }. Дати, пропущені під час паузи, не додаються.
recurring-edit-title = Редагувати регулярну витрату
recurring-edit-banner = Зміни діють з { $date }. Уже додані витрати залишаються без змін.
recurring-next-on-label = Наступна
recurring-next-too-early = Наступна дата має бути після останньої вже доданої витрати.
recurring-use-stop = Щоб завершити її, скористайся «Припинити повторення» в регулярній витраті.
recurring-drift = Курс { $currency } змінився на { $percent } % від створення. Кожна досі додається як { $amount } { $project_currency } (1 { $currency } = { $rate }). За сьогоднішнім курсом було б { $today_amount } { $project_currency }.
recurring-use-rate = Використати сьогоднішній курс
recurring-keep = Залишити { $amount } { $currency }
recurring-added = Додано регулярних витрат: { $count }
recurring-blocks-removal = { $name } поки не можна вилучити: учасник { $rules }. Вилучи { $name } з них або припини їх, потім збережи знову.
history-recurring-added = Додано автоматично: { $name } ({ $date })
history-recurring-created = Повторювану витрату створено: { $name }
history-recurring-edited = Повторювану витрату змінено: { $name }
history-recurring-paused = Повторювану витрату призупинено: { $name }
history-recurring-resumed = Повторювану витрату відновлено: { $name }
history-recurring-stopped = Повторювану витрату припинено: { $name }
occurrence-recurring = Регулярна витрата
occurrence-auto = Додано автоматично з регулярної витрати.
occurrence-auto-next = Додано автоматично з регулярної витрати. Наступна { $date }.
occurrence-auto-stopped = Додано автоматично з регулярної витрати, яку відтоді припинено.
occurrence-manage = Керувати
estimate-badge = Підтвердити
estimate-title = Суму треба підтвердити.
estimate-body = Додано з попередньою сумою. Введи справжню, коли дізнаєшся.
estimate-confirm = Підтвердити суму
apply-to = Застосувати до
apply-this-only = Лише цієї витрати
apply-and-next = Цієї та наступних
apply-and-next-hint = Регулярна витрата змінюється з { $date }
apply-rule-failed = Витрату збережено, але регулярну витрату не змінено.
occurrence-delete-message = «{ $name }» від { $date } буде видалено назавжди. Повторення триває, і ця дата не повернеться.
occurrence-delete-one = Видалити лише цю
occurrence-delete-stop = Видалити й припинити повторення
error-recurring-clock = Годинник цього пристрою поспішає. Перевір дату й час.
error-recurring-not-found = Цієї регулярної витрати більше не існує.
error-recurring-stale = Хтось тим часом змінив цю регулярну витрату. Її перезавантажено: перевір і збережи знову.
error-too-many-recurring = У проєкті вже 50 регулярних витрат. Припини непотрібну, щоб додати нову.
error-user-in-recurring = Цей учасник входить до регулярної витрати. Спершу вилучи його з неї або припини її.
