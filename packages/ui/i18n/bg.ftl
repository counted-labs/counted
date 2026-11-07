# Български. Пълен, с изключение на правните текстове (legal-, terms-, privacy-), които съществуват
# само на английски и френски и за всяко съобщение поотделно се връщат към en.ftl.

### Common

loading = Зареждане…
cancel = Отказ
discard-title = Да се отхвърлят ли промените?
discard-message = Въведеното ще се загуби.
discard = Отхвърли
confirm = Потвърди
retry = Опитай отново
delete = Изтрий
back = Назад
language = Език
theme = Тема
theme-light = Светла
theme-dark = Тъмна
theme-system = Системна
theme-hint-device = Запазва се само на това устройство.
theme-hint-system = Следва облика на това устройство.

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
error-rate-limited = Сървърът е претоварен. Опитай отново след малко.
error-generic = Нещо се обърка. Опитай отново.

error-invalid-email = Този имейл адрес не е валиден.
error-invalid-password = Тази парола не е валидна.
error-password-too-short = Паролата трябва да е поне 12 знака.
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
login-maybe-unverified = Току-що се регистрира? Първо потвърди адреса си с линка, който ти изпратихме, или го изпрати отново.
login-resend = Изпрати връзката за потвърждение отново
login-resending = Изпращане…
login-resend-sent = Имейлът е изпратен - провери пощата си.

register-submit = Създай акаунт
register-submitting = Създаване…
register-have-account = Вече имаш акаунт?
register-password-placeholder = Поне 12 знака
register-password-warning = Запиши си паролата: ние никога не я виждаме и не можем да ти я върнем.
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
account-delete-warning = Незабавно и окончателно изтриване на профила и данните ти.
account-delete-confirm-title = Изтриване на акаунт
account-delete-confirm-message = Акаунтът, сесиите и списъкът с проекти ще бъдат изтрити окончателно. Без паролата ти шифрованите данни на споделен проект стават нечетими за теб - това не може да бъде отменено.
erase-device-open = Изход и изтриване на това устройство
erase-device-title = Да се изтрие ли това устройство?
erase-device-body = Проектите, техните ключове и кешът на това устройство ще бъдат изтрити, след което сесията ще бъде прекратена. Проектите ти остават в акаунта ти: влез отново, за да си ги върнеш.
erase-device-checking = Проверявам дали проектите ти са в акаунта ти…
erase-device-pending = Промени, които чакат да бъдат изпратени: { $count }. Свържи се отново с интернет и опитай пак.
erase-device-unreachable = Не успях да проверя дали проектите ти са в акаунта ти. Провери връзката и опитай пак.
erase-device-at-risk = Тези проекти не са в акаунта ти и ще бъдат загубени. За да ги запазиш, откажи и първо копирай връзката им:
erase-device-confirm = Изтрий и излез
erase-device-confirm-anyway = Изтрий въпреки това
erase-device-erasing = Изтриване…

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
payment-methods-stale = Данните ти за плащане бяха променени на друго устройство. Презаредени са - опитай отново.
payment-methods-key-missing = Влез отново, за да управляваш данните си за плащане.
settings-payment-methods-share-warning = Споделен начин е видим за всички членове на твоите проекти.
payment-method-share = Сподели с моите проекти
payment-method-share-hint = Показва се до името ти, когато някой ти дължи пари.
payment-method-copy = Копирай { $name }
payment-method-copied = Копирано.
payment-method-copy-failed = Неуспешно копиране - маркирай текста и го копирай ръчно.

verify-email-checking = Потвърждаване на имейл адреса…
verify-email-welcome = Имейлът е потвърден - добре дошъл в Counted!
verify-email-back-to-login = Обратно към входа
verify-email-intro = Въведи паролата си, за да потвърдиш, че този адрес е твой.
verify-email-submit = Потвърди адреса ми

### Project status

project-archive = Архивирай
project-sheet-invite = Покани
project-sheet-recurring = Повтарящи се
project-sheet-edit = Редактирай проекта
project-sheet-close = Затвори проекта
project-sheet-archive = Архивирай проекта
project-sheet-reopen = Отвори проекта отново
project-sheet-unarchive = Разархивирай проекта
project-sheet-leave = Напусни проекта

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
projects-offline-banner = Офлайн данни - свържи се отново, за да обновиш.
demo-banner = Демонстрационен проект - само за четене.
demo-start-own = Създай свой проект
demo-badge = Демо
demo-bar = Промените ти изчезват, когато затвориш този раздел.
demo-leave = Започни наистина
demo-preparing = Демото се подготвя…
demo-failed = Демото не можа да бъде подготвено.
demo-sign-in-blocked = Влизането е изключено в демото. Излез от него, за да влезеш или създадеш акаунт; твоите проекти остават непокътнати.
projects-no-local-data = Няма локални данни
projects-no-local-data-hint = Влез, за да заредиш проектите си за първи път.
projects-add = Добави проект
projects-create = Създай проект
projects-personal-expense = Личен разход
personal-expense-banner = Добавено в Моите разходи
personal-create-title = Създай личен проект
personal-create-hint = Следи какво харчиш сам
group-create-title = Създай споделен проект
group-create-hint = Подели разходите с приятели и семейство
personal-create-action = Създай
personal-create-signup = Създай профил
personal-sheet-title = Твоят личен проект
personal-sheet-intro = За отбелязване на това, което харчиш сам. Показва се с групите ти в Статистика.
personal-sheet-point-alone = Един член: ти, с името на профила си. Без връзка за споделяне.
personal-sheet-point-encrypted = Шифрован на устройствата ти, като другите проекти.
personal-sheet-point-stats = Дяловете ти в групите се добавят към тези разходи в Статистика.
personal-default-name = Моите разходи
my-spending-prev-month = Предишен месец
my-spending-next-month = Следващ месец
my-spending-spent = Похарчи
my-spending-groups = Групи
my-spending-income = Приходи
my-spending-balance = Баланс за месеца
my-spending-unconverted = Дяловете в друга валута остават настрана, докато не е известен курс.
my-spending-empty = Нищо този месец
my-spending-empty-hint = Добави разход с +. Дяловете ти в групите ще се появяват тук автоматично.
my-spending-share-of = от { $amount }
my-spending-uncounted = не се отчита
payer-you = теб
expense-received-by = получен от
counted-switch = Отчитай в Моите разходи
counted-switch-on = Включено: дялът ти се добавя към Моите разходи
counted-switch-off = Изключено: дялът ти не се показва в Моите разходи
counted-groups = Отчитани групи
counted-groups-intro = Дялът ти в тези групи се добавя към Моите разходи и към бюджетите.
counted-groups-converted = { $from }, превалутирано в { $to }
counted-groups-encrypted = Този избор е шифрован с акаунта ти и те следва на устройствата ти.
counted-groups-empty = Все още няма група на това устройство.
counted-groups-done = OK
counted-save-failed = Изборът не можа да бъде запазен. Опитай отново.
tab-budget = Бюджет
budget-title = Бюджети
budget-manage = Управление
budget-left = Остават { $amount }
budget-over = Надвишен с { $amount }
budget-empty = Няма бюджет
budget-empty-hint = Задай таван за всяка категория. Дяловете ти в групите също се броят.
budget-define = Задаване на бюджет
budget-sheet-title = Месечни бюджети
budget-sheet-intro = Върху твоя дял: лични и групи. Остави празно, за да не следиш категория.
budget-average = Ср. последни 3 месеца: { $amount }
budget-for = Бюджет { $category }
budget-alert = Предупреди ме при 80 %
budget-alert-hint = Изчислено на това устройство
budget-invalid = Невалидна сума за { $category }.
budget-offline = Бюджетите се запазват само онлайн. Свържи се отново и опитай пак.
budget-alert-near = { $category }: { $percent } % от бюджета за месеца
budget-alert-over = { $category }: бюджетът за месеца е надвишен
my-spending-search-summary =
    { $count ->
        [one] { $count } резултат
       *[other] { $count } резултата
    } · { $total } · лични и групи, всички месеци
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
expenses-unreadable = Някои записи не могат да се прочетат на това устройство и не са включени в сумите. Обнови приложението, за да ги видиш.
project-unlock = Отключи
project-no-local-data-hint = Влез, за да заредиш данните на този проект за първи път.
project-gone-title = Този проект вече не съществува
project-gone-hint = Беше изтрит, когато последният му член го напусна. Връзката за споделяне вече не работи, дори да я отвориш отново.

expense-add = Добави разход
transfer-add = Добави превод
expense-edit-title = Редактиране на разхода
expense-category = Категория
expense-category-auto = Авто · { $emoji }
emoji-picker-label = Емоджи
emoji-none = Без емоджи
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
expense-type-group = Вид
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
balance-gets-back = Получава обратно
balance-owes = Дължи
balance-settled = Уредено
reimbursements-empty-title = Всичко е уредено!
reimbursements-empty-hint = Предложения за уреждане се появяват тук, когато сметките не са балансирани
reimbursement-owes = { $debtor } дължи на { $creditor }
reimbursement-record = Уреди
settlement-recorded = Изравняването е записано.
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
move-expense = Преместване в проект
move-expense-title = Преместване в…
move-current = Текущ
move-converted = { $currency }, превърнато по месечния курс
move-note-personal = В следващата стъпка ще избереш кой е платил и за кого. Разходът ще напусне Моите разходи.
move-note-group = Разходът ще напусне { $project } за всички членове и балансът на групата ще бъде преизчислен. В следващата стъпка ще избереш кой е платил и за кого.
move-banner = От { $from } в { $to }
move-submit = Премести
move-done = Разходът е преместен в { $project }
history-expense-moved = Разходът е преместен в друг проект: { $name }
personal-badge = Лично
move-members =
    { $count ->
        [one] { $count } член
       *[other] { $count } члена
    }
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
expense-share-percent = { $percent }%
expense-added-by = Добавено от { $name } на { $date }
expense-inconsistent-detail = Сумите не се равняват: { $paid } платени, { $owed } дължими, за разход от { $total }. Редактирай разхода, за да го поправиш.
missing-access-key = Липсва ключ за достъп. Отвори този проект през връзката му за споделяне.
filter-all = Всички
filter-my-payments = Моите плащания
filter-my-debts = Какво дължа
expense-filter-group = Филтър
expenses-search = Търсене на разходи
expenses-no-match = Няма съвпадащи разходи.
expenses-search-open = Търсене
expenses-search-clear = Изчистване на търсенето
expenses-search-summary =
    { $count ->
        [one] { $count } резултат
       *[other] { $count } резултата
    } · { $total } · всички месеци
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
charts-my-share-skipped =
    { $count ->
        [one] 1 проект не е отчетен - не е избран участник или данните му не се заредиха.
       *[other] { $count } проекта не са отчетени - не е избран участник или данните им не се заредиха.
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
categories-manage = Управлявай категориите
categories-title = Категории
categories-yours = Твоите категории
categories-builtin = Стандартни
categories-add = Добави категория
categories-in-parent = в { $parent }
categories-renamed-from = преименувана · първоначално { $name }
categories-hidden-hint = скрита · разходите ѝ отиват в { $other }
categories-always-shown = винаги видима
categories-hidden = Скрита
categories-new = Нова категория
categories-edit = Редактирай категорията
categories-parent = Отчита се в
categories-parent-hint = В статистиката на всичките ти проекти { $name } се отчита в { $parent }.
categories-create = Създай
categories-hide = Скрий тази категория
categories-hide-hint = Разходите ѝ се показват в { $other } за всички членове.
categories-delete-title = Да се изтрие ли „{ $name }“?
categories-delete-body = Засегнатите разходи ({ $count }) се връщат в { $parent }. Нито един разход не се изтрива.
categories-delete-body-unknown = Разходите ѝ се връщат в { $parent }. Нито един разход не се изтрива.
categories-delete-warning = Категорията изчезва за всички членове на проекта.
categories-summary-created = Създадени: { $count }
categories-summary-renamed = Преименувани: { $count }
categories-summary-hidden = Скрити: { $count }
categories-summary-default = Няма собствени категории
categories-offline = Категориите се управляват само онлайн. Свържи се отново и опитай пак.
categories-error-too-many = Един проект има най-много 20 собствени категории.
categories-error-full = Този проект достигна лимита си за категории.
categories-error-name = Дай име на категорията.
categories-error-missing = Тази категория междувременно се промени. Затвори и опитай пак.
history-category-created = Създадена категория: { $name }
history-category-edited = Редактирана категория: { $name }
history-category-deleted = Изтрита категория: { $name }
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
charts-paid-more = С { $amount } повече от моя дял
charts-paid-less = С { $amount } по-малко от моя дял
charts-paid-even = Точно моят дял
charts-part-title = Твоят дял във всяка категория
charts-part-desc = Сивото е похарченото от групата, цветното – потребеното от теб.
charts-breakdown-title = Разбивка по категории
charts-breakdown-desc = Докосни сектор или ред, за да видиш разходите.
charts-of-total = { $amount } от { $total }
charts-show-all = Покажи всички ({ $count })
charts-show-less = Покажи по-малко
charts-spend-title = Разходи във времето
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
charts-projects-desc = Сумите се водят по валута.
history-empty = Няма събития
history-by = От { $name }
not-found-hint = Тази страница не съществува или е преместена.
payers-title-paid-by = Платено от
payers-title-sender = Подател
payers-title-contributors = Участници
debtors-title-debtors = Дължат
debtors-title-recipients = Получатели
debtors-title-beneficiaries = Бенефициенти
debtors-title-for = За
debtors-title-recipient = Получател
expense-split-between = разделено между { $count }
expense-people =
    { $count ->
        [one] { $count } човек
       *[other] { $count } души
    }

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
help-forgot-password-warning = Никой не може да я възстанови вместо теб.
help-forgot-password-a = Паролата ти защитава ключовете ти за шифроване и никога не ни се изпраща: никой - включително ние - не може да дешифрира проектите ти вместо теб. Пази я добре, най-добре в мениджър на пароли.
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
project-currency-locked = Валутата се определя при създаването на проекта.
currency-search = Търсене на валута

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
friends-add-hint = Ако този човек има акаунт, ще види заявката ти в приложението. Ако няма, кажи му: ще я намери, когато се регистрира с този адрес.
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
friends-remove-confirm-message = { $email } вече няма да е сред приятелите ти, а ти – сред неговите. Всеки от вас може да изпрати нова покана по-късно.
friends-no-key = Още не е готово
friends-key-changed = Кодът е сменен
friends-key-changed-hint = Кодът за сигурност на приятел се промени. Това става, когато защити акаунта си, или ако ключът му е подменен. Поканите с него са блокирани, докато не си прочетете новия код лично или по телефона и не докоснеш „Кодът съвпада“.
friends-fingerprint = Код за сигурност
friends-fingerprint-hint = Двама приятели, които си прочетат един и същ код за сигурност, знаят, че никой не стои между тях - дори нашият сървър.
friends-key-mismatch = Сървърът изпрати за акаунта ти публичен ключ, който не съвпада с личния ти ключ. Кодът ти за сигурност е скрит, а поканите са изключени на това устройство: някой може би манипулира сървъра.
keypair-mismatch = Ключовете, които сървърът изпрати за твоя акаунт, не съвпадат с тези на това устройство. Поканите и смените на ключ са изключени тук, докато не съвпаднат: някой може да манипулира сървъра.
friends-outgoing-title = Изпратени
friends-outgoing-hint = Чака отговор. Ще ги видиш сред приятелите си, щом приемат.
friends-withdraw = Отмени
friends-notify = Изпрати съобщение
friends-notify-label = Съобщи на { $email }
friends-notify-message = Минавам на Counted за общите ни разходи. Безплатна регистрация на https://counted.fr с адреса { $email }: там чака моята покана за приятелство.
friends-notify-copied = Съобщението е копирано
friends-notify-copy-failed = Съобщението не можа да се копира
guests-hint-title =
    { $count ->
        [one] { $count } участник все още не се е присъединил към проекта
       *[other] { $count } участници все още не са се присъединили към проекта
    }
guests-hint-later = По-късно
guests-hint-send = Покани ги
guests-send-title = Изпрати проекта на участниците
guests-send-friend = Покани приятел от Counted
guests-send-friend-badge = Без линк
guests-send-friend-hint = Ако вече има акаунт, получава шифрована покана; нищо не минава през приложението ти за съобщения.
guests-send-link = Сподели линка на проекта
guests-send-link-hint = За тези без акаунт. Отварят проекта с едно докосване.
guests-send-link-warning = Всеки с този линк може да чете и променя проекта. Приложението ти за съобщения също го вижда.
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
participants-others = Други участници
participants-removed = Премахнати
participant-restore = Възстанови
participants-empty = Добави участници по-долу.
participants-duplicate = „{ $name }“ вече е в списъка.
participants-input-label = Добави приятел или въведи име
participants-input-placeholder = Приятел или някакво име
participants-suggest-friend = Приятел · влиза като „{ $name }“, получава покана
participants-suggest-not-ready = Приятел · още не е готов
participants-suggest-guest = Добави „{ $text }“
participants-suggest-guest-sub = Гост
participants-friends = Твоите приятели
participants-all-friends = Всички приятели
participants-login-hint = Влез, за да добавяш хора направо от списъка си с приятели.
participants-invite-badge = Покани
participants-guest-badge = Гост
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
edit-project-you-are = В този проект ти си { $name }
edit-project-no-identity = Още не си избрал(а) кой си
edit-project-switch = Смени
edit-project-choose = Избери
invite-failed = Тези покани не можаха да бъдат изпратени: { $emails }
invite-waiting = Тези приятели трябва да отворят Counted веднъж, преди да могат да бъдат поканени: { $emails }
key-offer-title = Друг ключ за { $name }
key-offer-body = Това устройство има друг ключ за този проект, който никога не го е отварял тук.
key-offer-choices = Използвай този ключ: проектът се отваря с ключа от тази връзка или покана. Отказ: устройството запазва сегашния си ключ и нищо не се променя.
key-offer-use = Използвай този ключ
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
expense-pending = Изчаква
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

home-owed-to-you = Дължат ти
home-you-owe = Дължиш
home-details-show = Подробности
home-details-hide = Скрий подробностите
home-all-settled = Всичко е уредено
home-pay = Плати
home-remind = Напомни
home-remind-message = Малко напомняне: в { $project } още има неуредени възстановявания.
home-row-owed = дължат ти
home-row-owe = дължиш
home-row-settled = Уредено
home-row-quits = При теб е уредено
home-row-pending =
    { $count ->
        [one] { $count } чакащо плащане
       *[other] { $count } чакащи плащания
    }
home-totals-partial = Без проекти без обменен курс
error-account-key-changed = Ключът на акаунта ти беше сменен на друго устройство. Влез отново, за да продължиш.
settings-security = Сигурност
settings-change-password = Смяна на паролата
settings-change-password-hint = Проектите и данните ти остават същите. Другите ти устройства излизат от акаунта.
change-password-title = Смени паролата си
change-password-current = Текуща парола
change-password-new = Нова парола
change-password-confirm = Потвърди новата парола
change-password-mismatch = Двете нови пароли не съвпадат.
change-password-submit = Смени паролата
change-password-submitting = Сменя се…
change-password-done = Паролата е сменена. Другите ти устройства излязоха от акаунта.
error-rotation-incomplete = Акаунтът ти се промени междувременно. Опитай отново.
error-key-lifecycle-unavailable = Още не е налично: първо всички трябва да имат най-новата версия на приложението.
settings-compromised = Някой може да знае паролата ми
settings-compromised-hint = Нова парола, нови ключове и изход от всички други устройства.
settings-compromised-open = Защити акаунта ми
compromised-title = Защити акаунта си
compromised-body = Паролата и ключовете на акаунта ти се сменят, а всички други устройства излизат. Приятелите ти ще видят, че кодът ти за сигурност се променя: сравнете го отново.
compromised-keeps = Лицето, което е имало паролата ти, запазва вече отвореното: проектите, които досега са можели да се четат.
compromised-submit = Смени паролата и ключовете
compromised-done = Акаунтът ти е защитен. Помоли приятелите си да сравнят новия ти код за сигурност.
friends-new-fingerprint = Нов код:
friends-code-matches = Кодът съвпада
friends-code-trusted = Кодът е потвърден.
settings-recovery-key = Ключ за възстановяване
settings-recovery-key-none = Без него забравената парола ти коства настройките, данните за плащане и кодовете на приятелите, и отнема 3 дни.
settings-recovery-key-set = Създаден. Ако забравиш паролата, ти връща акаунта с всичко в него.
settings-recovery-key-pending = Нов ключ го заменя на { $when }. Не беше ти? Отмени с текущия си ключ.
recovery-key-create = Създай
recovery-key-replace = Замени
recovery-key-cancel-replacement = Отмени новия ключ
recovery-key-title = Твоят ключ за възстановяване
recovery-key-body = Запиши го или го запази там, където само ти имаш достъп. Counted не може да го покаже отново или да го възстанови вместо теб.
recovery-key-copy = Копирай
recovery-key-copied = Копирано
recovery-key-save = Запази като файл
recovery-key-print = Отпечатай
recovery-key-next = Продължи
recovery-key-confirm-body = За да провериш копието си, въведи група { $n } от ключа.
recovery-key-confirm-wrong = Тази група не съвпада. Погледни копието си отново.
recovery-key-password-body = Въведи паролата си, за да запазиш ключа в акаунта.
recovery-key-submit = Запази ключа
recovery-key-done = Ключът за възстановяване е запазен.
recovery-key-replacement-done = Новият ключ е запазен. Заменя текущия след 72 часа; дотогава текущият още работи.
recovery-key-field = Ключ за възстановяване
recovery-key-file-intro = Ключ за възстановяване на Counted
recovery-key-cancel-done = Новият ключ е отменен.
compromised-new-key = Запази новия ключ за възстановяване сега: заменя стария след 72 часа, а дотогава старият още работи.
pending-reset-title = Поискано е нулиране на акаунта ти
pending-reset-body = Влиза в сила на { $when }: тогава настройките, данните за плащане и кодовете на приятелите се губят, а устройствата трябва да влязат отново. Не беше ти? Отмени с текущата си парола.
pending-reset-cancel = Отмени нулирането
pending-reset-cancelled = Нулирането е отменено.
login-forgot = Забравена парола?
forgot-title = Забравена парола
forgot-body = Въведи имейла си. Ако има акаунт, ще изпратим връзка за избор на нова парола.
forgot-submit = Изпрати връзката
forgot-sent = Ако този адрес има акаунт, връзката е на път. Изтича след 24 часа.
reset-title = Върни акаунта си
reset-with-key-body = Въведи ключа за възстановяване и нова парола: всичко се връща както беше.
reset-submit-key = Възстанови акаунта ми
reset-no-key = Нямам ключа за възстановяване
reset-without-key-title = Нулиране без ключ за възстановяване
reset-without-key-body = Акаунтът ти започва отначало с нова парола след 72 часа. Проектите се връщат от устройствата ти и връзките за споделяне; настройките, данните за плащане и кодовете на приятелите не. Дотогава текущата парола работи и може да отмени нулирането.
reset-name = Твоето име
reset-submit-request = Нулирай акаунта ми
reset-requested = Готово. Новата парола работи от { $when }. Дотогава текущата парола работи и може да отмени.
reset-pending-already = Вече има чакащо нулиране до { $when }. Нова заявка започва 72-та часа отначало.
recovered-done = Акаунтът ти е върнат. Другите устройства излязоха.
login-reset-undoable = Тази парола беше заменена с нулиране през последните 30 дни. Ако не си го поискал, отмени го: акаунтът се връща какъвто беше.
login-undo-reset = Отмени нулирането
reset-undone = Нулирането е отменено. Акаунтът е както преди.
error-reset-pending = Новата парола работи, когато нулирането влезе в сила, 72 часа след заявката.
error-account-disabled = Този акаунт беше изтрит малко след нулиране и чака да бъде заличен. Предишната парола все още може да отмени нулирането.
error-recovery-key-invalid = Този ключ за възстановяване не отваря този акаунт.
error-project-key-required = Този проект се нуждае от ключа си. Отвори го отново от връзката за споделяне.
names-or = { $first } или { $last }
participants-admin-badge = Админ
edit-project-admins = Админи: { $count }
edit-project-succession = Ако в проекта не остане админ, ролята поема членът с акаунт, който е в него най-отдавна.
member-sheet-account = Член с акаунт
member-actions = Действия: { $name }
member-make-admin = Дай роля на админ
member-make-admin-hint = Ще може да сменя ключа на проекта и да премахва членове.
member-made-admin = { $name } вече има роля на админ.
group-security = Сигурност на проекта
group-security-admin = Сигурност на проекта · имаш роля на админ
group-rotate = Смени ключа на проекта
group-rotate-hint = Ако връзката към проекта е изтекла: притежателите на старата връзка губят достъп до проекта.
group-remove = Премахни някого от проекта
group-remove-hint = Отнема достъпа и едновременно сменя ключа.
group-admins-only = Само за админи.
group-admins-only-account = Само за админи, които имат акаунт.
group-ask-admin = Попитай админа на проекта: { $names }.
group-ask-admins = Попитай админите на проекта: { $names }.
group-ask-an-admin = Попитай някой от админите на проекта.
group-with-account = С акаунт
group-free = Безплатно
group-sign-in-hint = Админ на проекта ({ $names }) после може да ти даде ролята.
group-sign-in-hint-any = Админ на проекта после може да ти даде ролята.
group-no-admin = Този проект няма админ, затова ключът му не може да се смени.
group-hide = Скрий някого
group-hide-hint = Скрива човека от списъците, без да отнема достъпа му: връзката и ключът продължават да работят.
group-recreate-hint = Този проект няма админ. За да премахнеш някого, създай нов проект.
group-recreate = Създай нов проект
group-no-admin-yet = Този проект все още няма админ.
group-creator-hint = Само ти можеш да станеш админ на този проект, като създадеш акаунт.
group-creator-elsewhere-hint = Само лицето, създало проекта, може да стане негов админ, като създаде акаунт. Иначе създай нов проект, за да премахнеш някого.
member-revoke-admin = Отнеми ролята на админ
member-revoke-admin-hint = Вече няма да може да сменя ключа на проекта, нито да премахва членове.
member-revoked-admin = { $name } вече няма роля на админ.
member-step-down = Спри да бъдеш админ
member-step-down-hint = Вече няма да можеш да сменяш ключа на проекта, нито да премахваш членове.
member-stepped-down = Вече нямаш роля на админ.
member-remove = Премахни от проекта
member-remove-hint = Сменя ключа на проекта: { $name } няма да може да чете нищо ново.
member-hide = Скрий от списъците
error-user-not-admin = Този член не е админ.
error-creator-proof-invalid = Това устройство не е създало този проект.
error-project-has-admin = Този проект вече има админ.
error-not-project-admin = Само админите на проекта могат да направят това.
error-user-not-claimed = Това лице няма акаунт, затова не може да е админ.
error-project-key-rotated = Ключът на проекта е сменен. Поискай новата връзка от член.
error-project-rotating = Ключът на проекта се сменя. Опитай отново след малко.
error-rotation-expired = Смяната на ключа отне твърде дълго и беше отменена. Започни отначало.
error-rotation-rows-changed = Проектът се промени по време на смяната на ключа. Започни отначало.
project-rotated-title = Вече нямаш достъп до този проект
project-rotated-body = Ключът на този проект е сменен от админа. Поискай новата връзка от админа.
project-rotated-open-link = Отвори връзка
rotation-remove-title = Да се премахне ли { $name } от проекта?
rotation-rotate-title = Да се смени ли ключът на проекта?
rotation-remove-body = Премахването на член означава нов ключ. Новата връзка трябва да се сподели с хората без акаунт.
rotation-rotate-body = Със старата връзка вече няма да има достъп до проекта. Новата връзка трябва да се сподели с хората без акаунт.
rotation-boxed =
    { $count ->
        [0] Никой друг член няма да получи ключа автоматично
        [one] { $count } член ще получи ключа автоматично
       *[other] { $count } члена ще получат ключа автоматично
    }
rotation-by-link-title = Хора, на които трябва да изпратиш новата връзка
rotation-by-link-hint = Тези хора нямат акаунт. Ще им изпратиш връзката веднага след това.
rotation-names-and = { $first } и { $last }
rotation-names-and-others = { $names } и още { $count }
rotation-confirm-remove = Премахни и смени ключа
rotation-confirm-rotate = Смени ключа
rotation-done-title = Новият ключ е в сила
rotation-done-removed = { $name } вече не е част от проекта.
rotation-done-rotated = Ключът на проекта е сменен.
rotation-done-boxed =
    { $count ->
        [0] Никой друг член не получи ключа автоматично.
        [one] { $count } член получи ключа автоматично.
       *[other] { $count } члена получиха ключа автоматично.
    }
rotation-new-link =
    { $count ->
        [0] Нова връзка към проекта
        [one] Нова връзка за изпращане · { $count } човек без акаунт
       *[other] Нова връзка за изпращане · { $count } души без акаунт
    }
rotation-link-renewed = Връзката е подновена успешно.
rotation-done = Готово
history-key-changed = Ключът на проекта е сменен
history-member-removed = { $name } вече не е част от проекта
project-rotated-row-hint = Ключът е сменен. Поискай новата връзка.
