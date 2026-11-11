# Українська. Повна, за винятком юридичних текстів (legal-, terms-, privacy-), які існують лише
# англійською та французькою і для кожного повідомлення окремо повертаються до en.ftl.

### Common

loading = Завантаження…
cancel = Скасувати
discard-title = Відкинути зміни?
discard-message = Введене буде втрачено.
discard = Відкинути
confirm = Підтвердити
retry = Спробувати ще раз
delete = Видалити
back = Назад
language = Мова
theme = Тема
theme-light = Світла
theme-dark = Темна
theme-system = Системна
theme-hint-device = Зберігається лише на цьому пристрої.
theme-hint-system = Відповідає вигляду цього пристрою.

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
error-rate-limited = Сервер перевантажений. Спробуй ще раз за мить.
error-generic = Щось пішло не так. Спробуй ще раз.

error-invalid-email = Ця електронна адреса недійсна.
error-invalid-password = Цей пароль недійсний.
error-password-too-short = Пароль має містити щонайменше 12 символів.
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
login-maybe-unverified = Новий акаунт? Спершу підтверди адресу посиланням, яке ми тобі надіслали, або надішли його ще раз.
login-resend = Надіслати посилання для підтвердження ще раз
login-resending = Надсилання…
login-resend-sent = Лист надіслано - перевір скриньку.

register-submit = Створити обліковий запис
register-submitting = Створення…
register-have-account = Уже є обліковий запис?
register-password-placeholder = Щонайменше 12 символів
register-password-warning = Запиши свій пароль: ми ніколи його не бачимо й не можемо його тобі повернути.
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
settings-anonymous-title = Ти не ввійшов
settings-upsell-title = Твої проєкти на кожному пристрої
settings-upsell-free = Безкоштовно

account-logout = Вийти
account-logging-out = Вихід…
account-delete-title = Видалити мій обліковий запис
account-delete-confirm-title = Видалити обліковий запис
account-delete-confirm-message = Твій обліковий запис, сеанси та список проєктів буде видалено назавжди. Без пароля зашифровані дані спільного проєкту стануть для тебе нечитабельними - це неможливо скасувати.
erase-device-title = Очистити цей пристрій?
erase-device-body = Проєкти, їхні ключі та кеш на цьому пристрої буде видалено, після чого сеанс завершиться. Твої проєкти залишаються у твоєму обліковому записі: увійди знову, щоб їх повернути.
erase-device-checking = Перевіряю, чи твої проєкти є у твоєму обліковому записі…
erase-device-pending = Зміни, що чекають на надсилання: { $count }. Підключися до інтернету й спробуй ще раз.
erase-device-unreachable = Не вдалося перевірити, чи твої проєкти є у твоєму обліковому записі. Перевір з'єднання й спробуй ще раз.
erase-device-at-risk = Цих проєктів немає у твоєму обліковому записі, і їх буде втрачено. Щоб їх зберегти, скасуй і спершу скопіюй їхні посилання:
erase-device-confirm = Очистити й вийти
erase-device-confirm-anyway = Усе одно очистити
erase-device-erasing = Очищення…

settings-payment-methods = Платіжні реквізити
payment-method-kind = Спосіб
payment-method-kind-other = Інший
payment-method-label = Назва
payment-method-label-placeholder = Основний рахунок
payment-method-value = Реквізити
payment-method-value-placeholder = IBAN, номер телефону, ім’я користувача…
payment-method-add = Додати
payment-method-deleted = Спосіб оплати видалено.
payment-method-value-required = Заповни реквізити кожного способу оплати або видали його.
payment-method-label-required = Дай назву своєму власному способу.
payment-method-too-long = Це задовго - скороти.
payment-method-invalid-characters = Прибери розриви рядків або невидимі символи.
payment-method-limit = Можна зберегти до { $max } способів оплати.
payment-methods-saved = Платіжні реквізити збережено.
payment-methods-offline = Щоб зберегти платіжні реквізити, потрібне з’єднання з інтернетом.
payment-methods-stale = Твої платіжні реквізити було змінено на іншому пристрої. Їх перезавантажено - спробуй ще раз.
payment-methods-key-missing = Увійди ще раз, щоб керувати платіжними реквізитами.
payment-method-copy = Копіювати { $name }
payment-method-copied = Скопійовано.
payment-method-copy-failed = Не вдалося скопіювати - виділи текст і скопіюй його вручну.

verify-email-checking = Підтвердження електронної адреси…
verify-email-welcome = Адресу підтверджено - ласкаво просимо до Counted!
verify-email-back-to-login = Назад до входу
verify-email-intro = Введи свій пароль, щоб підтвердити, що ця адреса твоя.
verify-email-submit = Підтвердити мою адресу

### Project status

project-archive = Архівувати
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
projects-offline-banner = Дані офлайн - під’єднайся, щоб оновити.
demo-banner = Демонстраційний проєкт - лише для читання.
demo-start-own = Створити власний проєкт
demo-badge = Демо
demo-bar = Твої зміни зникнуть, коли ти закриєш цю вкладку.
demo-leave = Почати по-справжньому
demo-preparing = Готуємо твою демоверсію…
demo-failed = Не вдалося підготувати демоверсію.
demo-sign-in-blocked = У демоверсії вхід вимкнено. Вийди з неї, щоб увійти або створити обліковий запис; твої власні проєкти не змінено.
projects-no-local-data = Немає локальних даних
projects-no-local-data-hint = Увійди, щоб уперше завантажити свої проєкти.
projects-add = Додати проєкт
projects-create = Створити проєкт
projects-personal-expense = Особиста витрата
personal-expense-banner = Додано до Моїх витрат
personal-create-title = Створи особистий проєкт
personal-create-hint = Стеж, що витрачаєш сам
group-create-title = Створи спільний проєкт
group-create-hint = Ділися витратами з друзями та родиною
personal-create-action = Створити
personal-create-signup = Створити обліковий запис
personal-sheet-title = Твій особистий проєкт
personal-sheet-intro = Щоб записувати, що ти витрачаєш сам. Він показується з твоїми групами в Статистиці.
personal-sheet-point-alone = Один учасник: ти, з іменем твого облікового запису. Без посилання для поширення.
personal-sheet-point-encrypted = Зашифровано на твоїх пристроях, як і інші проєкти.
personal-sheet-point-stats = Твої частки в групах додаються до цих витрат у Статистиці.
personal-default-name = Мої витрати
my-spending-prev-month = Попередній місяць
my-spending-next-month = Наступний місяць
my-spending-spent = Ти витратив
my-spending-groups = Групи
my-spending-income = Доходи
my-spending-balance = Баланс місяця
my-spending-unconverted = Частки в іншій валюті не враховуються, доки курс невідомий.
my-spending-empty = Цього місяця нічого
my-spending-empty-hint = Додай витрату кнопкою +. Твої частки в групах з’являтимуться тут автоматично.
my-spending-share-of = з { $amount }
my-spending-uncounted = не враховано
payer-you = ти
expense-received-by = отримав(ла)
counted-switch = Враховувати в Мої витрати
counted-switch-on = Увімкнено: твоя частка додається до Мої витрати
counted-switch-off = Вимкнено: твоя частка не показується в Мої витрати
counted-groups = Враховані групи
counted-groups-intro = Твоя частка в цих групах додається до Мої витрати і до бюджетів.
counted-groups-converted = { $from }, переведено в { $to }
counted-groups-encrypted = Цей вибір зашифровано твоїм обліковим записом, і він супроводжує тебе на твоїх пристроях.
counted-groups-empty = На цьому пристрої ще немає груп.
counted-groups-done = OK
counted-save-failed = Не вдалося зберегти цей вибір. Спробуй ще раз.
tab-budget = Бюджет
budget-title = Бюджети
budget-manage = Керувати
budget-left = Залишилось { $amount }
budget-over = Перевищено на { $amount }
budget-empty = Немає бюджету
budget-empty-hint = Встанови ліміт для кожної категорії. Твої частки в групах теж враховуються.
budget-define = Встановити бюджет
budget-sheet-title = Місячні бюджети
budget-sheet-intro = На твою частку: особисті та групи. Залиш порожнім, щоб не стежити за категорією.
budget-average = Сер. за останні 3 місяці: { $amount }
budget-for = Бюджет { $category }
budget-alert = Попередити мене на 80 %
budget-alert-hint = Обчислюється на цьому пристрої
budget-invalid = Неправильна сума для { $category }.
budget-offline = Бюджети зберігаються лише онлайн. Підключись та спробуй ще раз.
budget-alert-near = { $category }: { $percent } % бюджету місяця
budget-alert-over = { $category }: бюджет місяця перевищено
my-spending-search-summary =
    { $count ->
        [one] { $count } результат
        [few] { $count } результати
        [many] { $count } результатів
       *[other] { $count } результату
    } · { $total } · особисті та групи, усі місяці
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
expenses-unreadable = Деякі записи неможливо прочитати на цьому пристрої, і вони не враховані в сумах. Онови застосунок, щоб їх побачити.
project-unlock = Розблокувати
project-no-local-data-hint = Увійди, щоб уперше завантажити дані цього проєкту.
project-gone-title = Цього проєкту більше не існує
project-gone-hint = Його було видалено, коли вийшов останній учасник. Посилання більше не працює, навіть якщо відкрити його знову.

expense-add = Додати витрату
transfer-add = Додати переказ
expense-edit-title = Змінити витрату
expense-category = Категорія
expense-category-auto = Авто · { $emoji }
emoji-picker-label = Емодзі
emoji-none = Без емодзі
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
expense-type-group = Тип
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
settlement-recorded = Розрахунок записано.
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
move-expense = Перемістити до проєкту
move-expense-title = Перемістити до…
move-current = Поточний
move-converted = { $currency }, за курсом місяця
move-note-personal = На наступному кроці ти обереш, хто платив і за кого. Витрата покине Мої витрати.
move-note-group = Витрата покине { $project } для всіх учасників, а баланс групи буде перераховано. На наступному кроці ти обереш, хто платив і за кого.
move-banner = З { $from } до { $to }
move-submit = Перемістити
move-done = Витрату переміщено до { $project }
history-expense-moved = Витрату переміщено до іншого проєкту: { $name }
personal-badge = Особисте
move-members =
    { $count ->
        [one] { $count } учасник
        [few] { $count } учасники
        [many] { $count } учасників
       *[other] { $count } учасника
    }
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
expense-share-percent = { $percent }%
expense-added-by = Додано: { $name }, { $date }
expense-inconsistent-detail = Суми не сходяться: заплачено { $paid }, винні { $owed }, для витрати { $total }. Зміни витрату, щоб виправити.
missing-access-key = Відсутній ключ доступу. Відкрий цей проєкт через його посилання для спільного доступу.
filter-all = Усі
filter-my-payments = Мої платежі
filter-my-debts = Що я винен
expense-filter-group = Фільтр
expenses-search = Пошук витрат
expenses-no-match = Жодна витрата не відповідає.
expenses-search-open = Пошук
expenses-search-clear = Очистити пошук
expenses-search-summary =
    { $count ->
        [one] { $count } результат
        [few] { $count } результати
        [many] { $count } результатів
       *[other] { $count } результату
    } · { $total } · усі місяці
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
charts-my-share-skipped =
    { $count ->
        [one] { $count } проєкт не враховано - не вибрано учасника, або його дані не завантажилися.
        [few] { $count } проєкти не враховано - не вибрано учасника, або їхні дані не завантажилися.
        [many] { $count } проєктів не враховано - не вибрано учасника, або їхні дані не завантажилися.
       *[other] { $count } проєкту не враховано - не вибрано учасника, або їхні дані не завантажилися.
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
categories-manage = Керувати категоріями
categories-title = Категорії
categories-yours = Твої категорії
categories-builtin = Стандартні
categories-add = Додати категорію
categories-in-parent = у категорії { $parent }
categories-renamed-from = перейменовано · спочатку { $name }
categories-hidden-hint = приховано · її витрати йдуть до категорії { $other }
categories-always-shown = завжди видима
categories-hidden = Прихована
categories-new = Нова категорія
categories-edit = Змінити категорію
categories-parent = Зараховується до категорії
categories-parent-hint = У статистиці всіх твоїх проєктів { $name } зараховується до категорії { $parent }.
categories-create = Створити
categories-hide = Приховати цю категорію
categories-hide-hint = Її витрати показуються всім учасникам у категорії { $other }.
categories-delete-title = Видалити «{ $name }»?
categories-delete-body = Відповідні витрати ({ $count }) повертаються до категорії { $parent }. Жодну витрату не буде видалено.
categories-delete-body-unknown = Її витрати повертаються до категорії { $parent }. Жодну витрату не буде видалено.
categories-delete-warning = Категорія зникне для всіх учасників проєкту.
categories-summary-created = Створені: { $count }
categories-summary-renamed = Перейменовані: { $count }
categories-summary-hidden = Приховані: { $count }
categories-summary-default = Немає власних категорій
categories-offline = Керувати категоріями можна лише онлайн. Підключись та спробуй ще раз.
categories-error-too-many = Проєкт має щонайбільше 20 власних категорій.
categories-error-full = Цей проєкт досяг ліміту категорій.
categories-error-name = Дай категорії назву.
categories-error-missing = Ця категорія тим часом змінилася. Закрий і спробуй ще раз.
history-category-created = Категорію створено: { $name }
history-category-edited = Категорію змінено: { $name }
history-category-deleted = Категорію видалено: { $name }
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
charts-paid-more = На { $amount } більше за мою частку
charts-paid-less = На { $amount } менше за мою частку
charts-paid-even = Рівно моя частка
charts-part-title = Твоя частка в кожній категорії
charts-part-desc = Сірим - що витратила група, кольором - що спожив ти.
charts-breakdown-title = Розподіл за категоріями
charts-breakdown-desc = Торкнися сектора чи рядка, щоб побачити витрати.
charts-of-total = { $amount } з { $total }
charts-show-all = Показати все ({ $count })
charts-show-less = Показати менше
charts-spend-title = Витрати в часі
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
charts-projects-desc = Суми ведуться за валютами.
history-empty = Немає подій
history-by = { $name }
not-found-hint = Цієї сторінки не існує, або її перемістили.
payers-title-paid-by = Заплатив(ла)
payers-title-sender = Відправник
payers-title-contributors = Внесли
debtors-title-debtors = Винні
debtors-title-recipients = Отримувачі
debtors-title-beneficiaries = Бенефіціари
debtors-title-for = Для
debtors-title-recipient = Отримувач
expense-split-between = розділено між { $count }
expense-people =
    { $count ->
        [one] { $count } особа
        [few] { $count } особи
        [many] { $count } осіб
       *[other] { $count } особи
    }

### Welcome

welcome-title = Твої рахунки - нікого не стосуються.
welcome-subtitle = Діліть витрати з друзями.
welcome-note = Безкоштовно. Без облікового запису. Без реклами.
welcome-link-title = Одне посилання - і всі долучаються.
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
help-forgot-password-warning = Ніхто не зможе відновити його замість тебе.
help-forgot-password-a = Твій пароль захищає твої ключі шифрування й ніколи не надсилається нам: ніхто - і ми теж - не зможе розшифрувати твої проєкти замість тебе. Зберігай його надійно, найкраще в менеджері паролів.
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
project-currency-locked = Валюта фіксується під час створення проєкту.
currency-search = Пошук валюти

update-required-title = Потрібне оновлення
update-required-body = Ця версія Counted застара для зв’язку із сервером. Онови її, щоб продовжити користуватися застосунком.
update-required-button = Оновити

notifications-label = Сповіщення
notifications-title = Сповіщення
notifications-empty = Нічого нового

friends-title = Друзі
friends-anonymous-body = Друзі прив’язані до твого облікового запису. Увійди, щоб додавати людей і запрошувати їх у свої проєкти без поширення посилання.
friends-add-title = Додати друга
friends-add-from-project = Додати в друзі
friends-request-sent = Запит надіслано
friends-no-account-key = Увійди ще раз, щоб керувати друзями на цьому пристрої.
friends-incoming-title = Запити
friends-accept = Прийняти
friends-decline = Відхилити
friends-list-title = Мої друзі
friends-remove = Видалити
friends-remove-confirm-title = Видалити друга
friends-remove-confirm-message = { $email } більше не буде у твоїх друзях, а ти - у його. Будь-хто з вас зможе надіслати новий запит пізніше.
friends-no-key = Ще не готово
friends-key-mismatch = Сервер надіслав для твого акаунта публічний ключ, який не збігається з твоїм приватним ключем. Твій код безпеки приховано, а запрошення на цьому пристрої вимкнено: можливо, хтось втручається в сервер.
keypair-mismatch = Ключі, які сервер надіслав для твого облікового запису, не збігаються з тими, що на цьому пристрої. Запрошення та зміни ключа тут вимкнено, доки вони не збіжаться: хтось, можливо, втручається в сервер.
friends-outgoing-title = Надіслані
friends-notify = Написати повідомлення
friends-notify-message = Переходжу на Counted для наших спільних витрат. Безкоштовна реєстрація на https://counted.fr з адресою { $email }: там чекає мій запит у друзі.
friends-notify-copied = Повідомлення скопійовано
friends-notify-copy-failed = Не вдалося скопіювати повідомлення
guests-hint-title =
    { $count ->
        [one] { $count } учасник ще не приєднався до проєкту
        [few] { $count } учасники ще не приєдналися до проєкту
        [many] { $count } учасників ще не приєдналося до проєкту
       *[other] { $count } учасника ще не приєдналися до проєкту
    }
guests-hint-later = Пізніше
guests-hint-send = Запроси їх
guests-send-title = Надіслати проєкт учасникам
guests-send-friend = Запросити друга з Counted
guests-send-friend-badge = Без посилання
guests-send-friend-hint = Якщо вже має обліковий запис, отримує зашифроване запрошення; ніщо не проходить через твій месенджер.
guests-send-link = Поділитися посиланням на проєкт
guests-send-link-hint = Для тих, хто без облікового запису. Відкривають проєкт одним дотиком.
guests-send-link-warning = Будь-хто з цим посиланням може читати й змінювати проєкт. Твій месенджер теж його бачить.
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
participants-others = Інші учасники
participants-removed = Вилучені
participant-restore = Відновити
participants-empty = Додай учасників нижче.
participants-duplicate = «{ $name }» уже є в списку.
participants-input-label = Додати друга або ввести ім’я
participants-input-placeholder = Друг або будь-яке ім’я
participants-suggest-friend = Друг · приєднається як «{ $name }», отримає запрошення
participants-suggest-not-ready = Друг · ще не готовий
participants-suggest-guest = Додати «{ $text }»
participants-suggest-guest-sub = Гість
participants-friends = Твої друзі
participants-all-friends = Усі друзі
participants-login-hint = Увійди, щоб додавати людей просто зі списку друзів.
participants-invite-badge = Запросити
participants-guest-badge = Гість
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
edit-project-you-are = У цьому проєкті ти { $name }
edit-project-no-identity = Ти ще не вибрав(-ла), хто ти
edit-project-switch = Змінити
edit-project-choose = Вибрати
invite-failed = Не вдалося надіслати ці запрошення: { $emails }
invite-waiting = Ці друзі мають один раз відкрити Counted, перш ніж їх можна буде запросити: { $emails }
key-offer-title = Інший ключ для { $name }
key-offer-body = Цей пристрій має інший ключ до цього проєкту, який ніколи не відкривав його тут.
key-offer-choices = Використати цей ключ: проєкт відкривається ключем із цього посилання чи запрошення. Скасувати: пристрій залишає свій поточний ключ, і нічого не змінюється.
key-offer-use = Використати цей ключ
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
expense-pending = Очікує
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

home-owed-to-you = Тобі винні
home-you-owe = Ти винен
home-details-show = Докладніше
home-details-hide = Сховати подробиці
home-all-settled = Усе розраховано
home-pay = Сплатити
home-remind = Нагадати
home-remind-message = Невелике нагадування: у { $project } ще залишилися невирівняні повернення.
home-row-owed = тобі винні
home-row-owe = ти винен
home-row-settled = Розраховано
home-row-quits = У тебе розраховано
home-row-pending =
    { $count ->
        [one] { $count } очікуване повернення
        [few] { $count } очікувані повернення
        [many] { $count } очікуваних повернень
       *[other] { $count } очікуваного повернення
    }
home-totals-partial = Без проєктів без обмінного курсу
error-account-key-changed = Ключ твого облікового запису змінено на іншому пристрої. Увійди знову, щоб продовжити.
settings-security = Безпека
settings-change-password-hint = Твої проєкти й дані залишаються як є. На інших твоїх пристроях буде виконано вихід.
change-password-title = Зміни свій пароль
change-password-current = Поточний пароль
change-password-new = Новий пароль
change-password-confirm = Підтверди новий пароль
change-password-mismatch = Два нові паролі не збігаються.
change-password-submit = Змінити пароль
change-password-submitting = Змінюю…
change-password-done = Пароль змінено. На інших твоїх пристроях виконано вихід.
error-rotation-incomplete = Твій обліковий запис тим часом змінився. Спробуй ще раз.
error-key-lifecycle-unavailable = Ще недоступно: спершу всім потрібна найновіша версія застосунку.
settings-compromised-open = Захистити мій обліковий запис
compromised-title = Захисти свій обліковий запис
compromised-body = Твій пароль і ключі облікового запису замінюються, а на всіх інших пристроях виконується вихід. Твої друзі побачать, що твій код безпеки змінився: порівняйте його знову.
compromised-keeps = Людина, яка мала твій пароль, зберігає все, що вже було відкрито: проєкти, які досі можна було читати.
compromised-submit = Замінити пароль і ключі
compromised-done = Твій обліковий запис захищено. Попроси друзів порівняти твій новий код безпеки.
friends-code-matches = Код збігається
friends-code-trusted = Код підтверджено.
settings-recovery-key = Ключ відновлення
settings-recovery-key-pending = Новий ключ замінить його { $when }. Це не ти? Скасуй поточним ключем.
recovery-key-cancel-replacement = Скасувати новий ключ
recovery-key-title = Твій ключ відновлення
recovery-key-body = Запиши його або збережи там, куди маєш доступ лише ти. Counted не може показати його знову чи відновити замість тебе.
recovery-key-copy = Копіювати
recovery-key-copied = Скопійовано
recovery-key-save = Зберегти як файл
recovery-key-print = Друкувати
recovery-key-next = Продовжити
recovery-key-confirm-body = Щоб перевірити копію, введи групу { $n } ключа.
recovery-key-confirm-wrong = Ця група не збігається. Подивись на копію ще раз.
recovery-key-password-body = Введи пароль, щоб зберегти ключ в обліковому записі.
recovery-key-submit = Зберегти ключ
recovery-key-done = Ключ відновлення збережено.
recovery-key-replacement-done = Новий ключ збережено. Він замінить поточний за 72 години; доти поточний ще працює.
recovery-key-field = Ключ відновлення
recovery-key-file-intro = Ключ відновлення Counted
recovery-key-cancel-done = Новий ключ скасовано.
compromised-new-key = Збережи новий ключ відновлення зараз: він замінить старий за 72 години, а доти старий ще працює.
pending-reset-title = Запит на скидання
pending-reset-body = Набуде чинності { $when }. Це не ти? Скасуй своїм паролем.
pending-reset-cancel = Скасувати скидання
pending-reset-cancelled = Скидання скасовано.
login-forgot = Забув пароль?
forgot-title = Забутий пароль
forgot-body = Введи свою пошту. Якщо до неї прив’язано обліковий запис, ми надішлемо посилання для вибору нового пароля.
forgot-submit = Надіслати посилання
forgot-sent = Якщо ця адреса має обліковий запис, посилання вже в дорозі. Воно дійсне 24 години.
reset-title = Поверни свій обліковий запис
reset-with-key-body = Введи ключ відновлення й новий пароль: усе повернеться як було.
reset-submit-key = Відновити мій обліковий запис
reset-no-key = У мене немає ключа відновлення
reset-without-key-title = Скинути без ключа відновлення
reset-without-key-body = Твій обліковий запис почнеться заново з новим паролем за 72 години. Проєкти повернуться з твоїх пристроїв і посилань для спільного доступу; налаштування, платіжні дані й коди друзів ні. Доти поточний пароль працює й може скасувати скидання.
reset-name = Твоє ім’я
reset-submit-request = Скинути мій обліковий запис
reset-requested = Готово. Новий пароль працюватиме з { $when }. Доти поточний пароль працює й може це скасувати.
reset-pending-already = Скидання вже очікує до { $when }. Новий запит почне 72 години заново.
recovered-done = Обліковий запис повернуто. На інших пристроях виконано вихід.
login-reset-undoable = Цей пароль замінили скиданням за останні 30 днів. Якщо ти цього не просив, скасуй: обліковий запис повернеться як був.
login-undo-reset = Скасувати скидання
reset-undone = Скидання скасовано. Обліковий запис такий, як був.
error-reset-pending = Новий пароль працюватиме, коли скидання набуде чинності, через 72 години після запиту.
error-account-disabled = Цей обліковий запис видалили невдовзі після скидання, і він чекає на стирання. Попередній пароль ще може скасувати скидання.
error-recovery-key-invalid = Цей ключ відновлення не відкриває цей обліковий запис.
error-project-key-required = Цьому проєкту потрібен його ключ. Відкрий його знову за посиланням для спільного доступу.
names-or = { $first } або { $last }
participants-admin-badge = Адмін
edit-project-admins = Адміни: { $count }
edit-project-succession = Якщо в проєкті не залишиться адміна, цю роль отримає людина з обліковим записом, яка перебуває в ньому найдовше.
member-sheet-account = Учасник з обліковим записом
member-actions = Дії: { $name }
member-make-admin = Надати роль адміна
member-make-admin-hint = Зможе змінювати ключ проєкту й видаляти учасників.
member-made-admin = { $name } тепер має роль адміна.
group-security = Безпека проєкту
group-security-admin = Безпека проєкту · ти маєш роль адміна
group-rotate = Змінити ключ проєкту
group-rotate-hint = Якщо посилання на проєкт витекло: власники старого посилання втратять доступ до проєкту.
group-remove = Видалити когось із проєкту
group-remove-hint = Забирає доступ і водночас змінює ключ.
group-admins-only = Лише для адмінів.
group-admins-only-account = Лише для адмінів, які мають обліковий запис.
group-ask-admin = Попроси адміна проєкту: { $names }.
group-ask-admins = Попроси адмінів проєкту: { $names }.
group-ask-an-admin = Попроси когось з адмінів проєкту.
group-with-account = З обліковим записом
group-free = Безкоштовно
group-sign-in-hint = Адмін проєкту ({ $names }) потім зможе надати тобі цю роль.
group-sign-in-hint-any = Адмін проєкту потім зможе надати тобі цю роль.
group-no-admin = У цього проєкту немає адміна, тому його ключ не можна змінити.
group-hide = Приховати когось
group-hide-hint = Приховує людину в списках, не забираючи доступу: посилання й ключ і далі працюють.
group-recreate-hint = У цього проєкту немає адміна. Щоб видалити когось, створи новий проєкт.
group-recreate = Створити новий проєкт
group-no-admin-yet = У цього проєкту ще немає адміна.
group-creator-hint = Лише ти можеш стати адміном цього проєкту, створивши обліковий запис.
group-creator-elsewhere-hint = Адміном може стати лише особа, яка створила проєкт, створивши обліковий запис. Інакше створи новий проєкт, щоб видалити когось.
member-revoke-admin = Забрати роль адміна
member-revoke-admin-hint = Більше не зможе змінювати ключ проєкту чи видаляти учасників.
member-revoked-admin = { $name } більше не має ролі адміна.
member-step-down = Перестати бути адміном
member-step-down-hint = Ти більше не зможеш змінювати ключ проєкту чи видаляти учасників.
member-stepped-down = Ти більше не маєш ролі адміна.
member-remove = Видалити з проєкту
member-remove-hint = Змінює ключ проєкту: { $name } більше не зможе читати нічого нового.
member-hide = Приховати в списках
error-user-not-admin = Цей учасник не адмін.
error-creator-proof-invalid = Цей пристрій не створював цей проєкт.
error-project-has-admin = Цей проєкт уже має адміна.
error-not-project-admin = Це можуть зробити лише адміни проєкту.
error-user-not-claimed = Ця людина не має облікового запису, тож не може бути адміном.
error-project-key-rotated = Ключ проєкту змінився. Попроси нове посилання в учасника.
error-project-rotating = Ключ проєкту зараз змінюється. Спробуй ще раз за мить.
error-rotation-expired = Зміна ключа тривала задовго й була скасована. Почни спочатку.
error-rotation-rows-changed = Проєкт змінився під час зміни ключа. Почни спочатку.
project-rotated-title = Ти більше не маєш доступу до цього проєкту
project-rotated-body = Ключ цього проєкту змінено адміном. Попроси в адміна нове посилання.
project-rotated-open-link = Відкрити посилання
rotation-remove-title = Видалити { $name } із проєкту?
rotation-rotate-title = Змінити ключ проєкту?
rotation-remove-body = Видалення учасника означає створення нового ключа. Нове посилання доведеться надіслати людям без облікового запису.
rotation-rotate-body = Ті, хто має старе посилання, втратять доступ до проєкту. Нове посилання доведеться надіслати людям без облікового запису.
rotation-boxed =
    { $count ->
        [0] Ключ не буде автоматично надіслано жодному іншому учаснику
        [one] Ключ буде автоматично надіслано { $count } учаснику
        [few] Ключ буде автоматично надіслано { $count } учасникам
        [many] Ключ буде автоматично надіслано { $count } учасникам
       *[other] Ключ буде автоматично надіслано { $count } учасникам
    }
rotation-by-link-title = Люди, яким треба надіслати нове посилання
rotation-by-link-hint = Ці люди не мають облікового запису. Ти надішлеш їм посилання одразу після цього.
rotation-names-and = { $first } і { $last }
rotation-names-and-others = { $names } і ще { $count }
rotation-confirm-remove = Видалити й змінити ключ
rotation-confirm-rotate = Змінити ключ
rotation-done-title = Новий ключ діє
rotation-done-removed = { $name } більше не входить до проєкту.
rotation-done-rotated = Ключ проєкту змінено.
rotation-done-boxed =
    { $count ->
        [0] Ключ не надіслано автоматично жодному іншому учаснику.
        [one] Ключ автоматично надіслано { $count } учаснику.
        [few] Ключ автоматично надіслано { $count } учасникам.
        [many] Ключ автоматично надіслано { $count } учасникам.
       *[other] Ключ автоматично надіслано { $count } учасникам.
    }
rotation-new-link =
    { $count ->
        [0] Нове посилання на проєкт
        [one] Нове посилання для надсилання · { $count } людина без облікового запису
        [few] Нове посилання для надсилання · { $count } людини без облікового запису
        [many] Нове посилання для надсилання · { $count } людей без облікового запису
       *[other] Нове посилання для надсилання · { $count } людини без облікового запису
    }
rotation-link-renewed = Посилання успішно оновлено.
rotation-done = Готово
history-key-changed = Ключ проєкту змінено
history-member-removed = { $name } більше не входить до проєкту
project-rotated-row-hint = Ключ змінено. Попроси нове посилання.

## Settings and Friends redesign (docs/plans/friends-settings-redesign.md)
settings-group-sharing = Спільний доступ
settings-group-help = Довідка та інформація
settings-friends-changed = { $count ->
    [one] { $count } код змінився
    [few] { $count } коди змінилися
    [many] { $count } кодів змінилося
   *[other] { $count } коду змінилося
}
settings-recovery-key-created = Створено
settings-recovery-key-missing = Треба створити
settings-recovery-key-replacing = Заміна очікує
settings-password = Пароль
settings-compromised-short = Хтось інший знає твій пароль?
settings-recovery-key-footnote = Без ключа відновлення забутий пароль стирає твої уподобання, платіжні реквізити й коди друзів.
erase-device-row = Очистити цей пристрій
erase-device-row-hint = Виходить із облікового запису й зберігає лише мову.
account-delete-hint = Негайно й остаточно.
settings-open-source = Відкритий код, AGPL-3.0
settings-upsell-short = Без облікового запису твої проєкти залишаються на цьому пристрої. З ним вони всюди з тобою, зашифровані.
settings-locked-group = З обліковим записом
settings-locked-friends-short = Запрошуй без посилання
settings-locked-payment-short = Бачать ті, хто тобі винен
payment-methods-title = Способи оплати
payment-methods-row-hint = Спільні з друзями
payment-methods-intro = Спільні з друзями. Зашифровано твоїм обліковим записом.
payment-methods-footnote = Спосіб оплати доступний (макс. 20) учасникам твоїх проєктів на вкладці Розрахунок.
payment-method-shared-pill = Спільний
payment-method-share-groups = Поділитися у своїх групах
payment-method-label-optional = Назва (необов’язково)
payment-method-delete-confirm-title = Видалити цей спосіб оплати?
payment-method-delete-confirm-message = { $name } більше не показуватиметься у твоїх проєктах.
payment-methods-empty-title = Немає способу оплати
payment-methods-empty-body = Додай свій IBAN або платіжний застосунок: його побачать ті, хто тобі винен.
payment-method-masked = { $kind }, що закінчується на { $last }
friends-to-check = Перевірити
friends-code-changed-row = Його код змінився
friends-own-code = Твій код безпеки
friends-own-code-hint = Твої друзі бачать цей код поруч із твоєю адресою. Якщо він збігається з їхнім, вони запрошують саме тебе й нікого іншого.
friends-received = Отримані
friends-received-on = Отримано { $date }
friends-pending = Очікує
friends-from-project = З проєкту
friends-accept-label = Прийняти запит від { $email }
friends-decline-label = Відхилити запит від { $email }
friends-add-hint-short = Ця людина побачить твій запит, якщо має обліковий запис. Якщо ні, повідом її.
friends-send = Надіслати
friends-sent-hint = Ще немає облікового запису? Повідом: повідомлення запрошує зареєструватися з цією адресою.
friends-done = Готово
friends-their-code = Його код безпеки
friends-code-compare-hint = Прочитайте його разом. Він має бачити той самий код угорі своєї сторінки Друзі.
friends-no-key-hint = Ще не готово: його ключ створиться, коли він наступного разу відкриє Counted. Доти ти не можеш його запросити.
friends-remove-row = Видалити з моїх друзів
friends-new-code = Новий код
friends-key-changed-hint-short = Перевірте цей код разом, поза цим застосунком. Доти запрошення з ним заблоковані.
friends-kem-changed-title = Його ключ замінено
friends-kem-changed-body = Постквантова частина його ключа змінилася, а код ні. Захист облікового запису змінює обидва, тож хтось міг підмінити цей ключ. Запрошення з ним залишаються заблокованими.
friends-kem-changed-hint = Можеш видалити його, а потім надіслати новий запит: його ключ буде закріплено знову.
friends-pending-sheet = Запит надіслано, очікує.
friends-withdraw-row = Скасувати запит
friends-empty-title = Друзів ще немає
friends-empty-body = Додай когось за електронною поштою, а потім запроси в проєкт без посилання.
friends-requests-empty-title = Запитів немає
friends-requests-empty-body = Тут з’являються отримані й надіслані запити.
friends-code-hidden = Твій код приховано.
friends-code-hidden-body = Ключ, наданий для твого облікового запису, не відповідає цьому пристрою. Запрошення тут вимкнено.
notifications-manage-friends = Керувати друзями
notifications-friend-request-on = Запит у друзі · { $date }

### Link preview

share-preview-title = Наші витрати в Counted
share-preview-description = Відкрий посилання, щоб побачити витрати й хто кому скільки винен. Без облікового запису, зашифровано на твоєму пристрої.

### Currency names

currency-EUR = Євро
currency-AED = Дирхам ОАЕ
currency-AFN = Афганський афгані
currency-ALL = Албанський лек
currency-AMD = Вірменський драм
currency-AOA = Ангольська кванза
currency-ARS = Аргентинський песо
currency-AUD = Австралійський долар
currency-AWG = Арубський флорин
currency-AZN = Азербайджанський манат
currency-BAM = Конвертована марка Боснії і Герцеговини
currency-BBD = Барбадоський долар
currency-BDT = Бангладеська така
currency-BHD = Бахрейнський динар
currency-BIF = Бурундійський франк
currency-BMD = Бермудський долар
currency-BND = Брунейський долар
currency-BOB = Болівійський болівіано
currency-BRL = Бразильський реал
currency-BSD = Багамський долар
currency-BTN = Бутанський нгултрум
currency-BWP = Ботсванська пула
currency-BYN = Білоруський рубль
currency-BZD = Белізький долар
currency-CAD = Канадський долар
currency-CDF = Конголезький франк
currency-CHF = Швейцарський франк
currency-CLP = Чилійський песо
currency-CNY = Китайський юань
currency-COP = Колумбійський песо
currency-CRC = Костариканський колон
currency-CUP = Кубинський песо
currency-CVE = Ескудо Кабо-Верде
currency-CZK = Чеська крона
currency-DJF = Джибутійський франк
currency-DKK = Данська крона
currency-DOP = Домініканський песо
currency-DZD = Алжирський динар
currency-EGP = Єгипетський фунт
currency-ERN = Еритрейська накфа
currency-ETB = Ефіопський бир
currency-FJD = Фіджійський долар
currency-FKP = Фунт Фолклендських островів
currency-GBP = Англійський фунт
currency-GEL = Грузинський ларі
currency-GHS = Ганський седі
currency-GIP = Гібралтарський фунт
currency-GMD = Гамбійський даласі
currency-GNF = Гвінейський франк
currency-GTQ = Гватемальський кетсаль
currency-GYD = Гаянський долар
currency-HKD = Гонконгський долар
currency-HNL = Гондураська лемпіра
currency-HTG = Гаїтянський гурд
currency-HUF = Угорський форинт
currency-IDR = Індонезійська рупія
currency-ILS = Ізраїльський новий шекель
currency-INR = Індійська рупія
currency-IQD = Іракський динар
currency-IRR = Іранський ріал
currency-ISK = Ісландська крона
currency-JMD = Ямайський долар
currency-JOD = Йорданський динар
currency-JPY = Японська єна
currency-KES = Кенійський шилінг
currency-KGS = Киргизький сом
currency-KHR = Камбоджійський рієль
currency-KMF = Коморський франк
currency-KPW = Північнокорейський вон
currency-KRW = Південнокорейський вон
currency-KWD = Кувейтський динар
currency-KYD = Долар Кайманових островів
currency-KZT = Казахстанський тенге
currency-LAK = Лаоський кіп
currency-LBP = Ліванський фунт
currency-LKR = Шрі-ланкійська рупія
currency-LRD = Ліберійський долар
currency-LSL = Лесотський лоті
currency-LYD = Лівійський динар
currency-MAD = Марокканський дирхам
currency-MDL = Молдовський лей
currency-MGA = Малагасійський аріарі
currency-MKD = Македонський денар
currency-MMK = Кʼят Мʼянми
currency-MNT = Монгольський тугрик
currency-MOP = Патака Макао
currency-MRU = Мавританська угія
currency-MUR = Маврикійська рупія
currency-MVR = Мальдівська руфія
currency-MWK = Малавійська квача
currency-MXN = Мексиканський песо
currency-MYR = Малайзійський рингіт
currency-MZN = Мозамбіцький метикал
currency-NAD = Намібійський долар
currency-NGN = Нігерійська найра
currency-NIO = Нікарагуанська кордоба
currency-NOK = Норвезька крона
currency-NPR = Непальська рупія
currency-NZD = Новозеландський долар
currency-OMR = Оманський ріал
currency-PAB = Панамське бальбоа
currency-PEN = Перуанський новий сол
currency-PGK = Кіна Папуа-Нової Ґвінеї
currency-PHP = Філіппінський песо
currency-PKR = Пакистанська рупія
currency-PLN = Польський злотий
currency-PYG = Парагвайський гуарані
currency-QAR = Катарський ріал
currency-RON = Румунський лей
currency-RSD = Сербський динар
currency-RUB = Російський рубль
currency-RWF = Руандійський франк
currency-SAR = Саудівський ріал
currency-SBD = Долар Соломонових Островів
currency-SCR = Сейшельська рупія
currency-SDG = Суданський фунт
currency-SEK = Шведська крона
currency-SGD = Сінгапурський долар
currency-SHP = Фунт острова Святої Єлени
currency-SLE = Леоне Сьєрра-Леоне
currency-SOS = Сомалійський шилінг
currency-SRD = Суринамський долар
currency-SSP = Південносуданський фунт
currency-STN = Добра Сан-Томе і Прінсіпі
currency-SVC = Сальвадорський колон
currency-SYP = Сирійський фунт
currency-SZL = Свазілендський лілангені
currency-THB = Таїландський бат
currency-TJS = Таджицький сомоні
currency-TMT = Туркменський манат
currency-TND = Туніський динар
currency-TOP = Тонганська паанга
currency-TRY = Турецька ліра
currency-TTD = Долар Тринідаду і Тобаго
currency-TWD = Новий тайванський долар
currency-TZS = Танзанійський шилінг
currency-UAH = Українська гривня
currency-UGX = Угандійський шилінг
currency-USD = Долар США
currency-UYU = Уругвайський песо
currency-UZS = Узбецький сум
currency-VES = Венесуельський болівар
currency-VND = Вʼєтнамський донг
currency-VUV = Вануатський вату
currency-WST = Самоанська тала
currency-XAF = Центральноафриканський франк
currency-XCD = Східнокарибський долар
currency-XCG = Caribbean Guilder
currency-XOF = Західноафриканський франк
currency-XPF = Французький тихоокеанський франк
currency-YER = Єменський ріал
currency-ZAR = Південноафриканський ранд
currency-ZMW = Замбійська квача
currency-ZWG = Zimbabwe Gold
