# Български. Пълен, с изключение на правните текстове (legal-, terms-, privacy-), които съществуват
# само на английски и френски и за всяко съобщение поотделно се връщат към en.ftl.

### Common

loading = Зареждане…
cancel = Отказ
confirm = Потвърди
retry = Опитай отново
delete = Изтрий
back = Назад
language = Език

### Navigation

nav-main = Главна навигация
nav-projects = Проекти
nav-charts = Статистика
nav-settings = Настройки

### Connectivity

offline-banner = Офлайн
offline-pending =
    { $count ->
        [one] { $count } чакаща
       *[other] { $count } чакащи
    }

sync-conflict-edit = Конфликт: редакцията на „{ $name }“ не успя (елементът е изтрит). Пропуснато.
sync-conflict-delete = Конфликт: изтриването на „{ $name }“ не успя (елементът е изтрит). Пропуснато.
sync-conflict-other = Конфликт: операцията върху „{ $name }“ не успя (елементът е изтрит). Пропуснато.
sync-error = Грешка при синхронизация: { $reason }

### Errors

error-network = Сървърът не е достъпен. Провери интернет връзката си.
error-generic = Нещо се обърка. Опитай отново.

error-invalid-email = Този имейл адрес не е валиден.
error-invalid-password = Тази парола не е валидна.
error-password-too-short = Паролата трябва да е поне 8 знака.
error-client-outdated = Тази версия на приложението е остаряла. Обнови я, за да влезеш.
error-invalid-link = Тази връзка не е валидна.
error-batch-too-large = Твърде много елементи наведнъж.
error-payers-required = Избери поне един платец.
error-debtors-required = Избери поне един човек, който дължи.
error-duplicate-participant = Участник се появява два пъти от една и съща страна.
error-participant-not-in-project = Този участник не е част от проекта.
error-too-many-participants = Твърде много участници за един разход.
error-invalid-credentials = Грешен имейл или парола.
error-unauthenticated = Влез, за да направиш това.
error-email-not-verified = Имейл адресът ти още не е потвърден.
error-project-not-found = Този проект вече не съществува.
error-expense-not-found = Този разход вече не съществува.
error-storage-full = Хранилището е пълно: ключът на този проект не можа да се запази на това устройство. Запази връзката за споделяне.
error-user-not-found = Този участник вече не съществува.
error-tricount-not-found = Tricount не е намерен или неговото API върна грешка.
error-too-many-members = Този проект е достигнал лимита си за членове.
error-identity-taken = Друг акаунт вече е заявил този участник.
error-claim-proof-invalid = Това устройство няма ключа на проекта, затова не може да заяви участник. Отвори отново връзката за споделяне.
error-user-has-payments = Този участник има разходи в проекта и не може да бъде премахнат.
error-resend-cooldown = Изчакай 60 секунди, преди да поискаш нов имейл.
error-self-friend-request = Не можеш да добавиш себе си като приятел.
error-not-a-friend = Можеш да каниш само хора от списъка си с приятели.
error-friend-has-no-key = Този приятел още не е отворил последната версия на приложението. Помоли го да влезе веднъж, после опитай отново.
error-friend-request-not-found = Тази заявка за приятелство вече не съществува.
error-invitation-not-found = Тази покана вече не съществува.
error-too-many-friend-requests = Твърде много заявки за приятелство засега. Опитай утре.
error-too-many-invitations = Твърде много чакащи покани.
error-invalid-kdf-salt = Настройките за криптиране не са валидни. Обнови приложението и опитай отново.
error-mixed-project-batch = Тези участници не са всички в един и същ проект.
error-invalid-payload = Тази версия на приложението изпрати данни, които сървърът не приема. Обнови я и опитай отново.
error-invalid-public-key = Ключът ти за криптиране не е валиден. Обнови приложението и опитай отново.
error-payment-methods-stale = Платежните ти данни са променени на друго устройство. Презареди и опитай отново.

### Auth

field-email = Имейл
field-email-placeholder = ti@primer.bg
field-password = Парола
field-name = Име
field-name-placeholder = Мария Иванова

login-title = Вход
login-submit = Влез
login-submitting = Влизане…
login-password-placeholder = Твоята парола
login-no-account = Още нямаш акаунт?
login-unverified = Имейл адресът ти още не е потвърден. Провери пощата си или изпрати връзката отново.
login-resend = Изпрати връзката за потвърждение отново
login-resending = Изпращане…
login-resend-sent = Имейлът е изпратен - провери пощата си.

register-submit = Създай акаунт
register-submitting = Създаване…
register-have-account = Вече имаш акаунт?
register-password-placeholder = Поне 8 знака
register-password-warning = Запиши си паролата. Ако я забравиш, акаунтът не може да бъде възстановен.
register-check-email-title = Провери имейла си
register-email-sent = Имейлът е изпратен
register-email-sent-hint = Натисни връзката в пощата си, за да активираш акаунта.
register-not-received-prefix = Не го получи? Провери папката за спам или
register-sign-in-link = влез
register-not-received-suffix = за да изпратиш връзката отново.
register-terms-prefix = Със създаването на акаунт приемаш нашите
register-terms-link = условия за ползване
register-terms-and = и нашата
register-privacy-link = политика за поверителност

settings-title = Настройки
settings-preferences = Предпочитания
settings-preferences-local = Запазени на това устройство.
settings-preferences-synced = Синхронизирани с акаунта ти, шифровани.
settings-about = За приложението
settings-anonymous-title = Не си влязъл
settings-upsell-title = Твоите проекти на всяко устройство
settings-upsell-free = Безплатно
settings-upsell-body = Counted работи без акаунт. С безплатен акаунт проектите и предпочитанията ти те следват на телефона, лаптопа и в браузъра - все така шифровани, все така нечетими за нас.
settings-locked-badge = Акаунт
settings-locked-friends = Създай акаунт, за да добавяш приятели и да ги каниш в проект от приложението - без връзка за раздаване.
settings-locked-payment-methods = Запази своя IBAN или платежно приложение веднъж и го сподели с проектите, които избереш. Който ти дължи, ще го види до името ти.
settings-friends-hint = Добавяй приятели и ги кани в проектите си, без да споделяш връзка.

account-member-since = Член от
account-logout = Изход
account-logging-out = Излизане…
account-delete-title = Изтрий акаунта ми
account-delete-warning = Незабавно и окончателно, без кошче. Разходите, които си въвел в споделен проект, остават видими за другите членове - те са част от техните сметки.
account-delete-confirm-title = Изтриване на акаунт
account-delete-confirm-message = Акаунтът, сесиите и списъкът с проекти ще бъдат изтрити окончателно. Без паролата ти шифрованите данни на споделен проект стават нечетими за теб - това не може да бъде отменено.

settings-payment-methods = Данни за плащане
settings-payment-methods-hint = Как искаш да ти връщат пари. Шифровани с акаунта ти.
payment-method-kind = Начин
payment-method-kind-other = Друг
payment-method-label = Име
payment-method-label-placeholder = Основна сметка
payment-method-value = Данни
payment-method-value-placeholder = IBAN, телефонен номер, потребителско име…
payment-method-add = Добави
payment-method-remove = Премахни { $name }
payment-method-empty = Още не си добавил данни за плащане.
payment-method-deleted = Начинът на плащане е изтрит.
payment-method-value-required = Попълни данните на всеки начин на плащане или го премахни.
payment-method-label-required = Дай име на своя собствен начин.
payment-method-too-long = Твърде дълго е - съкрати го.
payment-method-invalid-characters = Премахни новите редове или невидимите знаци.
payment-method-limit = Можеш да запазиш до { $max } начина на плащане.
payment-methods-saved = Данните за плащане са запазени.
payment-methods-offline = Трябва да си онлайн, за да запазиш данните за плащане.
payment-methods-stale = Данните ти за плащане бяха променени на друго устройство. Презаредени са — опитай отново.
payment-methods-key-missing = Влез отново, за да управляваш данните си за плащане.
settings-payment-methods-share-warning = Споделен начин е видим за всеки член на проектите, в които си избрал името си - всеки, който държи някоя от тези връзки.
payment-method-share = Сподели с моите проекти
payment-method-share-hint = Показва се до името ти, когато някой ти дължи пари.
payment-method-copy = Копирай { $name }
payment-method-copied = Копирано.
payment-method-copy-failed = Неуспешно копиране - маркирай текста и го копирай ръчно.

verify-email-checking = Потвърждаване на имейл адреса…
verify-email-welcome = Имейлът е потвърден - добре дошъл в Counted!
verify-email-back-to-login = Обратно към входа

### Project status

project-close = Затвори
project-archive = Архивирай
project-reopen = Отвори отново
project-unarchive = Разархивирай

### Dates

date-long = { $day } { $month } { $year } г.

month-1 = януари
month-2 = февруари
month-3 = март
month-4 = април
month-5 = май
month-6 = юни
month-7 = юли
month-8 = август
month-9 = септември
month-10 = октомври
month-11 = ноември
month-12 = декември

month-short-1 = яну
month-short-2 = фев
month-short-3 = мар
month-short-4 = апр
month-short-5 = май
month-short-6 = юни
month-short-7 = юли
month-short-8 = авг
month-short-9 = сеп
month-short-10 = окт
month-short-11 = ное
month-short-12 = дек

### Actions

add = Добави
create = Създай
creating = Създаване…
edit = Редактирай
leave = Напусни
close = Затвори
paste = Постави
join = Присъедини се
import = Импортирай
importing = Импортиране…
field-description = Описание
field-date = Дата
date-today = Днес
date-yesterday = Вчера
field-optional = По избор

### Projects

projects-filter-active = Активни
projects-filter-all = Всички
projects-count-label = Проекти
projects-empty = Няма проекти
projects-empty-hint = Създай проект с бутона по-долу
projects-offline-banner = Офлайн данни - свържи се отново, за да обновиш.
projects-no-local-data = Няма локални данни
projects-no-local-data-hint = Влез, за да заредиш проектите си за първи път.
projects-add = Добави проект
projects-create = Създай проект
projects-join = Присъедини се към проект
projects-import-tricount = Импортирай от Tricount
project-actions = Действия с проекта

status-ongoing = Текущ
status-closed = Затворен
status-archived = Архивиран

nav-help = Помощ
nav-privacy = Политика за поверителност
nav-terms = Условия за ползване
nav-legal = Правна информация

leave-project-title = Напускане на проекта?
leave-project-message = Ще загубиш достъп от това устройство. Ако не остане нито един член, проектът и всичките му разходи се изтриват окончателно.

add-project-title = Нов проект
add-project-name-label = Име на проекта
add-project-name-placeholder = Моето пътуване, Съквартиранти 2024…
add-project-participants = Участници
add-project-participant-name = Име на участника
add-project-participant-placeholder = Кларк Кент
add-project-offline = Не можеш да създадеш проект офлайн. Свържи се отново и опитай пак.
add-project-name-required = Проектът има нужда от име.

join-link-label = Връзка за споделяне
join-link-hint = Връзката съдържа ключа за дешифриране - копирай я цялата.
join-invalid-link = Тази връзка не е валидна. Постави цялата връзка за споделяне, включително частта след #.
join-wrong-project = Тази връзка е за друг проект.

import-tricount-link-label = Връзка или ключ на Tricount
import-tricount-key-required = Въведи връзка или ключ на Tricount.
import-tricount-encryption-failed = Шифроването не успя.
import-tricount-unimportable = Нищо не е импортирано: този Tricount има участници с акаунт в Tricount или суми, които не се връзват (засегнати записи: { $count }).

### Expenses

save = Запази
saving = Запазване…
adding = Добавяне…
link-copied = Връзката е копирана
missing-encryption-key = Липсва ключ за шифроване.
missing-encryption-key-title = Липсва ключ за шифроване
missing-encryption-key-hint = Използваната връзка не съдържа ключа, нужен за дешифриране на този проект. Използвай пълната връзка, споделена от този, който го е създал.
project-locked-hint = Това устройство няма ключа на този проект. Отвори връзката му за споделяне, за да го отключиш.
project-unlock = Отключи
project-no-local-data-hint = Влез, за да заредиш данните на този проект за първи път.
project-gone-title = Този проект вече не съществува
project-gone-hint = Беше изтрит, когато последният му член го напусна. Връзката за споделяне вече не работи, дори да я отвориш отново.

expense-add = Добави разход
transfer-add = Добави превод
expense-edit-title = Редактиране на разхода
expense-category = Категория
expense-category-auto = Авто · { $emoji }
expense-currency = Валута на сумата
amount-op-add = Плюс
amount-op-subtract = Минус
amount-op-multiply = Умножи
amount-op-divide = Раздели
amount-op-equals = Равно
amount-op-done = Готово
expense-rate = Обменен курс (по избор)
expense-rate-hint = Остави празно, за да използваш курса на Европейската комисия (InforEuro) за { $month }: 1 { $from } = { $rate } { $to }.
expense-rate-invalid = Въведи обменен курс, по-голям от 0.
expense-rate-unavailable = Няма автоматичен курс - въведи го ръчно.
expense-delete-title = Изтриване на разхода
expense-delete-message = „{ $name }“ ще бъде изтрит окончателно. Това не може да бъде отменено.
expense-inconsistent-amounts = Сумите не се равняват
expenses-empty = Няма разходи
expenses-empty-hint = Започни, като добавиш разходи с бутона по-долу
expenses-show-more = Покажи още (остават { $count })

expense-type-expense = Разход
expense-type-transfer = Превод
expense-type-gain = Приход
expense-paid-by = платен от
expense-sent-by = изпратен от
expense-contributed-by = внесен от

expense-name-required = Името е задължително.
expense-amount-not-positive = Сумата трябва да е по-голяма от 0.
expense-no-payer = Избери поне един платец.
expense-no-debtor = Избери поне един човек, който дължи.
expense-invalid-date = Тази дата не е валидна.
expense-payers-mismatch = Платците дават общо { $sum }, което не съвпада със сумата на разхода ({ $total }).
expense-debtors-mismatch = Длъжниците дават общо { $sum }, което не съвпада със сумата на разхода ({ $total }).

participants-none = Никой
participants-everyone = Всички ({ $count })
participants-some = { $count } от { $total }
participants-select-all = Избери всички
participants-by-shares = По дялове
split-amounts = Суми
participants-remaining = Остават { $amount }
participants-over-by = { $amount } в повече
participants-who-paid = Кой плати?
participants-who-received = Кой получи?
participants-who-transfers = Кой превежда?
participants-who-receives = Кой получава?
participants-for-whom = За кого?

stats-total-expenses = Общо разходи
stats-my-expenses = Моите разходи

tab-expenses = Разходи
tab-balance = Баланс
tab-reimbursements = Уреждане
reimbursements-empty-title = Всичко е уредено!
reimbursements-empty-hint = Предложения за уреждане се появяват тук, когато сметките не са балансирани
reimbursement-owes = { $debtor } дължи на { $creditor }
reimbursement-record = Уреди
reimbursement-pay-with = Плати
reimbursement-pay-shared-by = Споделено от { $name } - преди да изпратиш, провери името на получателя, което показва приложението ти.
reimbursement-pay-title = Плати на { $name }
reimbursements-mine-title = Ти дължиш
reimbursements-others-title = Други връщания
copy = Копирай

user-selection-title = Кой участник си ти?
user-selection-hint = Избери името си от списъка.
user-selection-required = Моля, избери участник.
identity-claimed = Свързан с акаунт
identity-claimed-by = Акаунт на { $name }
identity-taken-repick = Друг акаунт е заявил участника, който използваше. Моля, избери друг.
participant-gone-repick = Участникът, който използваше, е премахнат от този проект. Моля, избери друг.

edit-project-title = Редактиране на проекта
edit-project-new-badge = нов
edit-project-deferred-new-members = добавянето на нови членове
edit-project-deferred-removals = премахването на членове
edit-project-offline-deferred = Офлайн: { $items } ще се приложи при повторно свързване.

export-failed = Експортът не успя: { $reason }

history-expense-added = Добавен разход: { $name }
history-expense-edited = Редактиран разход: { $name }
history-expense-deleted = Изтрит разход: { $name }
history-project-edited = Редактиран проект: { $name }
history-name-changed = Име: „{ $from }“ → „{ $to }“
history-description-added = Добавено описание: „{ $value }“
history-description-removed = Премахнато описание: „{ $value }“
history-description-changed = Описание: „{ $from }“ → „{ $to }“

### Sweep

field-amount = Сума
expense-name-placeholder = Ресторант, пазаруване…
expense-actions = Действия с разхода
expense-your-share = Твоят дял
expense-your-share-value = Твоят дял: { $amount } { $currency }
expense-inconsistent-detail = Сумите не се равняват: { $paid } платени, { $owed } дължими, за разход от { $total }. Редактирай разхода, за да го поправиш.
missing-access-key = Липсва ключ за достъп. Отвори този проект през връзката му за споделяне.
filter-all = Всички
filter-my-payments = Моите плащания
filter-my-debts = Какво дължа
participants-shares-for = Дялове за { $name }
participants-amount-for = Сума за { $name }
reimbursement-add = Добави уреждане
project-forget = Премахни от списъка ми
project-history-title = История
history-kind-add = Добавено
history-kind-delete = Изтрито
history-kind-edit = Редактирано
export = Експортирай
export-json = Експортирай JSON
export-csv = Експортирай CSV
share-link = Сподели
copy-link-failed = Връзката не можа да бъде копирана
open-in-app = Отвори в приложението
not-found-title = Страницата не е намерена
not-found-back = Обратно към проектите

### Charts

charts-period = Период
period-all = Всичко
period-month = Месец
period-3months = 3 мес.
period-year = Година
period-custom = По избор
charts-tab-categories = Категории
charts-tab-trends = Тенденции
charts-total-spent = Общо похарчено
charts-avg-per-person = Средно на човек
charts-expense-count =
    { $count ->
        [one] { $count } разход
       *[other] { $count } разхода
    }
charts-nothing-to-show = Няма нищо за показване
charts-my-share-note = Тези суми са твоят дял от всеки разход.
charts-my-share-skipped =
    { $count ->
        [one] 1 проект не е отчетен — не е избран участник или данните му не се заредиха.
       *[other] { $count } проекта не са отчетени — не е избран участник или данните им не се заредиха.
    }

### Categories

category-food = Храна
category-transport = Транспорт
category-accommodation = Настаняване
category-leisure = Развлечения
category-shopping = Пазаруване
category-services = Услуги
category-parties-gifts = Партита и подаръци
category-other = Друго
charts-project = Проект
charts-all-projects = Всички проекти
charts-date-from = От
charts-date-to = До
charts-total = Общо
charts-tab-people = Хора
charts-tab-projects = Проекти
charts-scope = Чии разходи
charts-scope-group = Група
charts-scope-me = Аз
charts-currency = Валута
charts-my-share = Моят дял
charts-share-of-total = { $pct }% от { $total }
charts-i-paid = Платих
charts-paid-more = С { $amount } повече от твоя дял
charts-paid-less = С { $amount } по-малко от твоя дял
charts-paid-even = Точно твоят дял
charts-part-title = Твоят дял във всяка категория
charts-part-desc = Сивото е похарченото от групата, цветното – потребеното от теб.
charts-breakdown-title = Разбивка по категории
charts-breakdown-desc = Докосни сектор или ред, за да видиш разходите.
charts-of-total = { $amount } от { $total }
charts-show-all = Покажи всички ({ $count })
charts-show-less = Покажи по-малко
charts-spend-title = Разходи във времето
charts-spend-desc = Кратките периоди са по дни, по-дългите – по седмици или месеци.
charts-group-by = Групирай по
bucket-day = Ден
bucket-week = Седмица
bucket-month = Месец
charts-avg = ср.
charts-cat-title-day = { $category }, ден по ден
charts-cat-title-week = { $category }, седмица по седмица
charts-cat-title-month = { $category }, месец по месец
charts-cat-desc = Избери категория, за да я следиш във времето.
charts-running-title = Натрупана сума
charts-running-desc = От { $date }.
charts-avg-per-day = { $amount } / ден средно
charts-avg-per-week = { $amount } / седмица средно
charts-avg-per-month = { $amount } / месец средно
charts-people-title = Кой носеше групата
charts-people-desc = Колко е платил всеки, до това, което е потребил.
charts-paid = Платено
charts-fair-share = Справедлив дял
charts-you = (ти)
charts-net-more = платил повече
charts-net-less = платил по-малко
charts-balance-title = Твоят баланс във времето
charts-balance-desc = Над линията групата ти дължи. Под нея ти дължиш на групата.
charts-owed = Дължат ти
charts-owe = Дължиш
charts-projects-title = Твоят дял по проекти
charts-projects-desc = Сумите се водят по валута и никога не се събират.
history-empty = Няма събития
history-by = От { $name }
not-found-hint = Тази страница не съществува или е преместена.
payers-title-paid-by = Платено от
payers-title-sender = Подател
payers-title-contributors = Участници
debtors-title-debtors = Дължат
debtors-title-recipients = Получатели
debtors-title-beneficiaries = Бенефициенти

### Welcome

welcome-title = Сметките ти не са ничия работа.
welcome-subtitle = Дели разходи с приятели.
welcome-note = Безплатно. Без акаунт. Без реклами.
welcome-link-title = Един линк и всички участват.
welcome-link-body = Никой не трябва да създава акаунт.
welcome-link-account = Акаунт? Никога задължителен. Служи да намираш проектите си на друго устройство, да каниш приятели от приложението и да споделяш данните си за плащане.
welcome-demo-project = Уикенд в Лион
welcome-private-title = Никой не може да чете сметките ти. Дори ние.
welcome-private-body = Имена, суми, проекти: всичко се шифрова на устройството ти. Само ти държиш ключа.
welcome-private-names = Имена
welcome-private-amounts = Суми
welcome-private-projects = Проекти
welcome-scan-title = Снимай касовата бележка.
welcome-scan-body = Сумата, датата и категорията се попълват сами. Всичко става на телефона ти. Снимката не се пази.
welcome-eu-title = 100% европейско
welcome-no-ads = Без реклами
welcome-no-trackers = Без тракери
welcome-step = Стъпка { $current } от { $total }
welcome-next = Напред
welcome-skip = Пропусни
welcome-start = Започни
welcome-how-it-works = Как точно работи?

### Help

help-intro = Често задаван въпрос? Докосни, за да разгънеш отговора.
help-create-project-q = Как да създам проект?
help-create-project-a = От началния екран докосни бутона + отдолу. Дай име на проекта, избери валутата му и си готов.
help-add-participants-q = Как да добавя участници?
help-add-participants-a = Отвори проекта и добави участници от списъка с членове. Всеки участник може да плаща или да дължи по разход.
help-share-project-q = Как да споделя проект?
help-share-project-a = Сподели URL адреса на проекта (този в адресната лента). Всеки с връзката може да вижда и редактира проекта.
help-add-expense-q = Как да добавя разход?
help-add-expense-a = В проекта докосни +, въведи сумата, посочи кой е платил и между кого да се раздели. Можеш да избереш и дата, различна от днешната.
help-types-q = Каква е разликата между разход, превод и приход?
help-types-expense = - покупка, направена от един човек и разделена между няколко.
help-types-transfer = - връщане на пари от един човек на друг, без разделяне.
help-types-gain = - получени пари (възстановяване, подарък), които се разделят между няколко души.
help-past-date-q = Мога ли да датирам разход в миналото?
help-past-date-a = Да, полето за дата е свободно. Времето на създаване на записа се пази отделно.
help-who-owes-q = Как Counted изчислява кой какво дължи?
help-who-owes-a = Counted изчислява нетния баланс на всеки участник (колко е дал минус колко дължи), после предлага най-кратката поредица от преводи, която урежда всички.
help-minimal-transfers-q = Защо броят на предложените преводи е минимален?
help-minimal-transfers-a = Алгоритъмът първо съчетава баланси, които се компенсират точно, после обработва останалите от най-големия кредитор към най-големия длъжник. Резултатът: по-малко преводи, за да се уреди всичко.
help-import-tricount-q = Как да импортирам проект от Tricount?
help-import-tricount-a = От началния екран докосни бутона „+“ отдолу, после
help-import-tricount-b = Постави връзката за споделяне на Tricount, който искаш да импортираш.
help-encryption-q = Шифровани ли са данните ми?
help-encryption-a = Да. Counted съчетава две гаранции:
help-encryption-e2ee-term = Шифроване от край до край
help-encryption-e2ee-def = - всичко между теб и сървъра пътува шифровано.
help-encryption-zero-term = Нулев достъп
help-encryption-zero-def = - ти шифроваш данните, преди да ги изпратиш, а сървърът пази само шифрован текст. Нямаме начин да го прочетем.
help-encryption-see = За подробности виж
help-forgot-password-q = Какво става, ако забравя паролата си?
help-forgot-password-warning = Данните ти ще бъдат загубени окончателно.
help-forgot-password-a = Ключът за шифроване се извлича от паролата ти, затова нулиране не е възможно: никой - включително ние - не може да дешифрира проектите ти без нея. Пази я добре, най-добре в мениджър на пароли.
help-archive-delete-q = Как да архивирам или изтрия проект?
help-archive-delete-a = От екрана на проекта отвори менюто и избери
help-archive-delete-b = за да го скриеш, като го запазиш. Проектът се изтрива окончателно, когато последният му член го напусне.
help-delete-account-q = Как да изтрия акаунта си?
help-delete-account-a = Отвори Настройки и използвай „Изтрий акаунта ми“. Става незабавно и не може да бъде отменено.
help-contact = Друг въпрос? Пиши ни на

# Receipt scanning (mobile only)
expense-scan = Сканирай касова бележка
scan-in-progress = Четене на бележката…
scan-error-capture = Снимката не се получи. Опитай отново или въведи разхода ръчно.
scan-error-unreadable = Нищо четимо на тази бележка. Въведи разхода ръчно.
scan-check-amount = Провери сумата - не беше отпечатана ясно.
scan-take-photo = Направи снимка
scan-choose-photo = Избери снимка
expense-converted-from = Платени { $amount } { $from } · 1 { $from } = { $rate } { $to }
project-currency = Валута
project-currency-hint = Всяка сума се показва в тази валута. Не може да се променя по-късно.
project-currency-locked = Валутата се определя при създаването на проекта.

update-required-title = Необходимо е обновяване
update-required-body = Тази версия на Counted е твърде стара, за да общува със сървъра. Обнови я, за да продължиш да използваш приложението.
update-required-button = Обнови

notifications-label = Известия
notifications-title = Известия
notifications-empty = Нищо ново
notifications-friend-request = Заявка за приятелство

friends-title = Приятели
friends-anonymous-body = Приятелите се пазят с акаунта ти. Влез, за да добавяш хора и да ги каниш в проектите си, без да споделяш връзка.
friends-add-title = Добави приятел
friends-add-hint = Ще види заявката ти, когато влезе. Никой от двама ви не научава дали другият има акаунт, докато заявката не бъде приета.
friends-add-button = Добави
friends-add-from-project = Добави като приятел
friends-request-sent = Заявката е изпратена
friends-no-account-key = Влез отново, за да управляваш приятелите си на това устройство.
friends-incoming-title = Заявки
friends-accept = Приеми
friends-decline = Откажи
friends-list-title = Моите приятели
friends-list-empty = Още няма приятели. Добави някого по имейл по-горе или от проект, който споделяте.
friends-remove = Премахни
friends-remove-confirm-title = Премахване на приятел
friends-remove-confirm-message = { $email } вече няма да е сред приятелите ви, а вие – сред неговите. Всеки от вас може да изпрати нова покана по-късно.
friends-no-key = Още не е готово
friends-fingerprint = Код за сигурност
friends-fingerprint-hint = Двама приятели, които си прочетат един и същ код за сигурност, знаят, че никой не стои между тях - дори нашият сървър.
friends-outgoing-title = Изпратени
friends-outgoing-hint = Чака отговор. Ще ги видиш сред приятелите си, щом приемат.
friends-withdraw = Отмени
invite-friends-title = Покани приятели
invite-friends-hint = Ключът на проекта се шифрова за всеки приятел на това устройство. Сървърът никога не го вижда.
invite-friends-empty = Още няма приятели за покана.
invite-friends-button = Покани
invite-sent = { $count ->
    [one] Поканата е изпратена
   *[other] Изпратени са { $count } покани
}
invitation-badge = Покана
invitation-to = Присъедини се към „{ $name }“
invitation-to-unnamed = Присъедини се към проект
invitation-unreadable = Тази покана не може да бъде отворена на това устройство
invitation-from = От { $email }
invitation-accept = Присъедини се
invitation-decline = Откажи

# Participants in the create and edit modals, and the "who are you?" picker - see
# docs/plans/friends.md §11.
participants-you-label = Името ти в този проект
participants-you-badge = Ти
participants-you-from-account = Взето от името на профила ти. Промени го тук само за този проект.
participants-you-required = Задължително. Така ще те виждат останалите.
participants-others = Други участници
participants-empty = Още никой. Избери приятел по-долу или въведи някакво име.
participants-empty-signed-out = Още никой. Въведи име, за да добавиш някого.
participants-duplicate = „{ $name }“ вече е в списъка.
participants-input-label = Добави приятел или въведи име
participants-input-placeholder = Приятел или някакво име
participants-suggest-friend = Приятел · влиза като „{ $name }“, получава покана
participants-suggest-not-ready = Приятел · още не е готов
participants-suggest-guest = Добави „{ $text }“ без профил
participants-suggest-guest-sub = Без профил, само име
participants-friends = Твоите приятели
participants-all-friends = Всички приятели
participants-login-hint = Влез, за да добавяш хора направо от списъка си с приятели.
participants-invite-badge = Покани
participants-guest-badge = Без профил
participants-guest-sub = Без профил, само име
participants-rename = Преименувай: { $name }
participants-remove = Премахни: { $name }
participants-rename-label = Ново име
participants-rename-save = Запази името
participants-rename-hint = Името, което всички виждат в този проект. Поканата пак отива до { $email }.
participants-invited-badge = Поканен
participants-invited-sub = { $email } · още не е приета
participants-invited-pending = Поканата още не е приета
participants-unlinked = Не е свързан с профил
add-project-create-invite = Създай и покани: { $count }
edit-project-save-invite = Запази и покани: { $count }
edit-project-you-are = На това устройство ти си { $name }
edit-project-no-identity = Още не си избрал(а) кой си
edit-project-switch = Смени
edit-project-choose = Избери
invite-failed = Тези покани не можаха да бъдат изпратени: { $emails }
invite-again = Покани отново
friend-picker-title = Добави приятели
user-selection-invited-hint = { $email } те покани в „{ $project }“.
user-selection-suggested = Предложено
user-selection-suggested-sub = { $email } те добави с това име
user-selection-confirm-as = Аз съм { $name }
user-selection-missing = Името ти го няма? Помоли участник да те добави в настройките на проекта.

## Повтарящи се разходи

repeat-label = Повтаряне
repeat-none = Не се повтаря
repeat-weekly = Всяка седмица
repeat-biweekly = На всеки 2 седмици
repeat-monthly = Всеки месец
repeat-quarterly = На всеки 3 месеца
repeat-yearly = Всяка година
repeat-every-weeks = На всеки { $count } седм.
repeat-every-months = На всеки { $count } мес.
repeat-every-years = На всеки { $count } г.
repeat-custom = Персонализирано…
repeat-every = На всеки
repeat-unit-weeks = седм.
repeat-unit-months = мес.
repeat-unit-years = г.
repeat-on-weekday = ден: { $weekday }
repeat-on-day = на { $day }-о число
repeat-on-day-month = на { $day } { $month }
repeat-month-end = В по-кратките месеци се пада на последния ден.
repeat-ends = Край
repeat-ends-never = Никога
repeat-ends-on = На дата
repeat-ends-after = След
repeat-fewer = По-малко
repeat-more = Повече
repeat-last-on = последен на { $date }
repeat-variable = Сумата се променя всеки път
repeat-variable-hint = Всеки се добавя с последната сума и се отбелязва „за потвърждение“.
repeat-done = Готово
repeat-no-end = Без край
repeat-until = До { $date }
repeat-occurrences = Брой повторения: { $count }
repeat-offline = Нужна е връзка. Самият разход пак може да се добави.
repeat-foreign = Повтаря се като { $amount } { $currency }, превалутирано веднъж по днешния курс. Ще бъдеш предупреден, ако курсът се промени с повече от 5 %.
repeat-backfill = Започва в миналото. Разходи, добавени сега: { $count }.
add-and-repeat = Добави и повтаряй
weekday-1 = понеделник
weekday-2 = вторник
weekday-3 = сряда
weekday-4 = четвъртък
weekday-5 = петък
weekday-6 = събота
weekday-7 = неделя
recurring-title = Повтарящи се разходи
recurring-strip = Повтарящи се разходи: { $count }
recurring-next = Следващ: { $name }, { $date }
recurring-to-confirm = За потвърждение: { $count }
recurring-per-month = На месец, прибл.
recurring-your-share = Твоят дял
recurring-active = Активни
recurring-paused = На пауза
recurring-finished = Приключени
recurring-paid-by = плаща { $name }
recurring-next-on = Следващ на { $date }
recurring-progress = { $done } от { $total }
recurring-rate-badge = Курсът се промени с { $percent } %
recurring-empty = Още нищо не се повтаря. Избери „Повтаряне“, когато добавяш разход: наем, абонаменти, сметки.
recurring-next-ones = Следващи
recurring-added-so-far = Добавени досега
recurring-set-up-by = Създаден от
recurring-pause = Пауза
recurring-resume = Продължи
recurring-stop = Спри повтарянето
recurring-stop-title = Да се спре ли „{ $name }“?
recurring-stop-message = Няма да се повтаря повече. Вече добавените разходи остават.
recurring-resume-title = Да продължи ли „{ $name }“?
recurring-resume-message = Следващ на { $date }. Пропуснатите по време на паузата дати не се добавят.
recurring-edit-title = Редактиране на повтарящ се разход
recurring-edit-banner = Промените важат от { $date }. Вече добавените разходи остават непроменени.
recurring-next-on-label = Следващ на
recurring-next-too-early = Следващата дата трябва да е след последния вече добавен разход.
recurring-use-stop = За да го прекратиш, използвай „Спри повтарянето“ в повтарящия се разход.
recurring-drift = Курсът на { $currency } се е променил с { $percent } % от създаването. Всеки още се добавя като { $amount } { $project_currency } (1 { $currency } = { $rate }). По днешния курс би бил { $today_amount } { $project_currency }.
recurring-use-rate = Използвай днешния курс
recurring-keep = Запази { $amount } { $currency }
recurring-added = Добавени повтарящи се разходи: { $count }
recurring-blocks-removal = { $name } още не може да бъде премахнат: участва в { $rules }. Извади { $name } от тях или ги спри и запази отново.
history-recurring-added = Добавен автоматично: { $name } ({ $date })
history-recurring-created = Създаден повтарящ се разход: { $name }
history-recurring-edited = Редактиран повтарящ се разход: { $name }
history-recurring-paused = Повтарящ се разход на пауза: { $name }
history-recurring-resumed = Възобновен повтарящ се разход: { $name }
history-recurring-stopped = Спрян повтарящ се разход: { $name }
occurrence-recurring = Повтарящ се разход
occurrence-auto = Добавен автоматично от повтарящ се разход.
occurrence-auto-next = Добавен автоматично от повтарящ се разход. Следващ на { $date }.
occurrence-auto-stopped = Добавен автоматично от повтарящ се разход, който оттогава е спрян.
occurrence-manage = Управление
estimate-badge = За потвърждение
estimate-title = Сума за потвърждение.
estimate-body = Добавен с предишната сума. Въведи истинската, когато я научиш.
estimate-confirm = Потвърди сумата
apply-to = Приложи за
apply-this-only = Само този разход
apply-and-next = Този и следващите
apply-and-next-hint = Повтарящият се разход се променя от { $date }
apply-rule-failed = Разходът е запазен, но повтарящият се разход не е променен.
occurrence-delete-message = „{ $name }“ от { $date } ще бъде изтрит окончателно. Повтарянето продължава и тази дата няма да се върне.
occurrence-delete-one = Изтрий само този
occurrence-delete-stop = Изтрий и спри повтарянето
error-recurring-clock = Часовникът на това устройство избързва. Провери датата и часа.
error-recurring-not-found = Този повтарящ се разход вече не съществува.
error-recurring-stale = Някой междувременно промени този повтарящ се разход. Той е презареден: провери го и запази отново.
error-too-many-recurring = Този проект вече има 50 повтарящи се разхода. Спри някой ненужен, за да добавиш нов.
error-user-in-recurring = Този участник е част от повтарящ се разход. Първо го извади оттам или спри разхода.
