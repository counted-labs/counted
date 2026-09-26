# Македонски. Целосен, освен правните текстови (legal-, terms-, privacy-), кои постојат само на
# англиски и француски и за секоја порака одделно се враќаат на en.ftl.

### Common

loading = Вчитување…
cancel = Откажи
confirm = Потврди
retry = Обиди се повторно
delete = Избриши
back = Назад
language = Јазик

### Navigation

nav-main = Главна навигација
nav-projects = Проекти
nav-charts = Статистика
nav-settings = Поставки

### Connectivity

offline-banner = Офлајн
offline-pending =
    { $count ->
        [one] { $count } на чекање
       *[other] { $count } на чекање
    }

sync-conflict-edit = Конфликт: уредувањето на „{ $name }“ не успеа (ставката е избришана). Прескокнато.
sync-conflict-delete = Конфликт: бришењето на „{ $name }“ не успеа (ставката е избришана). Прескокнато.
sync-conflict-other = Конфликт: операцијата на „{ $name }“ не успеа (ставката е избришана). Прескокнато.
sync-error = Грешка при синхронизација: { $reason }

### Errors

error-network = Серверот не е достапен. Провери ја интернет врската.
error-generic = Нешто тргна наопаку. Обиди се повторно.

error-invalid-email = Оваа е-пошта не е важечка.
error-invalid-password = Оваа лозинка не е важечка.
error-password-too-short = Лозинката мора да има најмалку 8 знаци.
error-client-outdated = Оваа верзија на апликацијата е застарена. Ажурирај ја за да се најавиш.
error-invalid-link = Овој линк не е важечки.
error-batch-too-large = Премногу ставки одеднаш.
error-payers-required = Избери барем еден плаќач.
error-debtors-required = Избери барем едно лице кое должи.
error-duplicate-participant = Учесник се појавува двапати на истата страна.
error-participant-not-in-project = Тој учесник не е дел од овој проект.
error-too-many-participants = Премногу учесници за еден трошок.
error-invalid-credentials = Погрешна е-пошта или лозинка.
error-unauthenticated = Најави се за да го направиш тоа.
error-email-not-verified = Твојата е-пошта сè уште не е потврдена.
error-project-not-found = Овој проект повеќе не постои.
error-expense-not-found = Овој трошок повеќе не постои.
error-user-not-found = Овој учесник повеќе не постои.
error-tricount-not-found = Tricount не е пронајден или неговото API врати грешка.
error-too-many-members = Овој проект го достигна лимитот на членови.
error-identity-taken = Друга сметка веќе го презеде овој учесник.
error-claim-proof-invalid = Овој уред го нема клучот на проектот, па не може да преземе учесник. Отвори го повторно линкот за споделување.
error-user-has-payments = Овој учесник има трошоци во проектот и не може да се отстрани.
error-resend-cooldown = Почекај 60 секунди пред да побараш нова е-пошта.
error-self-friend-request = Не можеш да се додадеш себеси како пријател.
error-not-a-friend = Можеш да покануваш само луѓе од листата на пријатели.
error-friend-has-no-key = Овој пријател сè уште не ја отворил најновата верзија на апликацијата. Замоли го да се најави еднаш, па обиди се повторно.
error-friend-request-not-found = Ова барање за пријателство повеќе не постои.
error-invitation-not-found = Оваа покана повеќе не постои.
error-too-many-friend-requests = Засега премногу барања за пријателство. Обиди се утре.
error-too-many-invitations = Премногу покани на чекање.
error-invalid-kdf-salt = Поставките за криптирање не се валидни. Ажурирај ја апликацијата и обиди се повторно.
error-mixed-project-batch = Овие учесници не се сите во ист проект.
error-invalid-payload = Оваа верзија на апликацијата испрати податоци што серверот не ги прифаќа. Ажурирај ја и обиди се повторно.
error-invalid-public-key = Твојот клуч за криптирање не е валиден. Ажурирај ја апликацијата и обиди се повторно.
error-payment-methods-stale = Твоите податоци за плаќање се сменети на друг уред. Освежи и обиди се повторно.

### Auth

field-email = Е-пошта
field-email-placeholder = ti@primer.mk
field-password = Лозинка
field-name = Име
field-name-placeholder = Ана Стојанова

login-title = Најава
login-submit = Најави се
login-submitting = Најавување…
login-password-placeholder = Твојата лозинка
login-no-account = Сè уште немаш сметка?
login-unverified = Твојата е-пошта сè уште не е потврдена. Провери го сандачето или испрати го линкот повторно.
login-resend = Испрати го линкот за потврда повторно
login-resending = Испраќање…
login-resend-sent = Е-поштата е испратена - провери го сандачето.

register-submit = Создај сметка
register-submitting = Создавање…
register-have-account = Веќе имаш сметка?
register-password-placeholder = Најмалку 8 знаци
register-password-warning = Запиши ја лозинката. Ако ја заборавиш, сметката не може да се врати.
register-check-email-title = Провери ја е-поштата
register-email-sent = Е-поштата е испратена
register-email-sent-hint = Кликни на линкот во сандачето за да ја активираш сметката.
register-not-received-prefix = Не ја доби? Провери во спам или
register-sign-in-link = најави се
register-not-received-suffix = за да го испратиш линкот повторно.
register-terms-prefix = Со создавање сметка ги прифаќаш нашите
register-terms-link = услови за користење
register-terms-and = и нашата
register-privacy-link = политика за приватност

settings-title = Поставки
settings-preferences = Претпочитања
settings-preferences-local = Зачувано на овој уред.
settings-preferences-synced = Синхронизирано со твојата сметка, шифрирано.
settings-about = За апликацијата
settings-anonymous-title = Не си најавен
settings-upsell-title = Твоите проекти на секој уред
settings-upsell-free = Бесплатно
settings-upsell-body = Counted работи без сметка. Со бесплатна сметка твоите проекти и претпочитања те следат на телефонот, лаптопот и веб - сè уште шифрирани, сè уште нечитливи за нас.
settings-locked-badge = Сметка
settings-locked-friends = Создај сметка за да додаваш пријатели и да ги покануваш во проект од апликацијата - без линк за споделување.
settings-locked-payment-methods = Зачувај го својот IBAN или апликација за плаќање еднаш и сподели ги со проектите што ќе ги избереш. Кој ти должи, ги гледа покрај твоето име.
settings-friends-hint = Додавај пријатели и покани ги во своите проекти без споделување линк.

account-member-since = Член од
account-logout = Одјава
account-logging-out = Одјавување…
account-delete-title = Избриши ја мојата сметка
account-delete-warning = Веднаш и трајно, без корпа. Трошоците што си ги внел во споделен проект остануваат видливи за другите членови - тие се дел од нивните сметки.
account-delete-confirm-title = Избриши сметка
account-delete-confirm-message = Твојата сметка, сесиите и листата на проекти ќе бидат трајно избришани. Без лозинката шифрираните податоци на споделен проект стануваат нечитливи за тебе - ова не може да се врати.

settings-payment-methods = Податоци за плаќање
settings-payment-methods-hint = Како сакаш да ти се врати. Шифрирано со твојата сметка.
payment-method-kind = Начин
payment-method-kind-other = Друго
payment-method-label = Име
payment-method-label-placeholder = Главна сметка
payment-method-value = Податоци
payment-method-value-placeholder = IBAN, телефонски број, корисничко име…
payment-method-add = Додај
payment-method-remove = Отстрани { $name }
payment-method-empty = Сè уште немаш додадено податоци за плаќање.
payment-method-deleted = Начинот на плаќање е избришан.
payment-method-value-required = Пополни ги податоците на секој начин на плаќање или отстрани го.
payment-method-label-required = Дај име на својот сопствен начин.
payment-method-too-long = Тоа е предолго - скрати го.
payment-method-invalid-characters = Отстрани ги новите редови или невидливите знаци.
payment-method-limit = Можеш да зачуваш до { $max } начини на плаќање.
payment-methods-saved = Податоците за плаќање се зачувани.
payment-methods-offline = Мораш да бидеш онлајн за да ги зачуваш податоците за плаќање.
payment-methods-stale = Твоите податоци за плаќање беа променети на друг уред. Повторно се вчитани — обиди се повторно.
payment-methods-key-missing = Најави се повторно за да управуваш со податоците за плаќање.
settings-payment-methods-share-warning = Споделен начин е видлив за секој член на проектите во кои си го избрал своето име - за секој што има еден од тие линкови.
payment-method-share = Сподели со моите проекти
payment-method-share-hint = Се прикажува покрај твоето име кога некој ти должи пари.
payment-method-copy = Копирај { $name }
payment-method-copied = Копирано.
payment-method-copy-failed = Не можеше да се копира - означи го текстот и копирај го рачно.

verify-email-checking = Потврдување на е-поштата…
verify-email-welcome = Е-поштата е потврдена - добредојде во Counted!
verify-email-back-to-login = Назад кон најава

### Project status

project-close = Затвори
project-archive = Архивирај
project-reopen = Отвори повторно
project-unarchive = Врати од архива

### Dates

date-long = { $day } { $month } { $year }

month-1 = јануари
month-2 = февруари
month-3 = март
month-4 = април
month-5 = мај
month-6 = јуни
month-7 = јули
month-8 = август
month-9 = септември
month-10 = октомври
month-11 = ноември
month-12 = декември

month-short-1 = јан
month-short-2 = фев
month-short-3 = мар
month-short-4 = апр
month-short-5 = мај
month-short-6 = јун
month-short-7 = јул
month-short-8 = авг
month-short-9 = сеп
month-short-10 = окт
month-short-11 = ное
month-short-12 = дек

### Actions

add = Додај
create = Создај
creating = Создавање…
edit = Уреди
leave = Напушти
close = Затвори
paste = Залепи
join = Приклучи се
import = Увези
importing = Увезување…
field-description = Опис
field-date = Датум
date-today = Денес
date-yesterday = Вчера
field-optional = По избор

### Projects

projects-filter-active = Активни
projects-filter-all = Сите
projects-count-label = Проекти
projects-empty = Нема проекти
projects-empty-hint = Создај проект со копчето подолу
projects-offline-banner = Офлајн податоци - поврзи се повторно за освежување.
projects-no-local-data = Нема локални податоци
projects-no-local-data-hint = Најави се за да ги вчиташ проектите за прв пат.
projects-add = Додај проект
projects-create = Создај проект
projects-join = Приклучи се на проект
projects-import-tricount = Увези од Tricount
project-actions = Дејства за проектот

status-ongoing = Во тек
status-closed = Затворен
status-archived = Архивиран

nav-help = Помош
nav-privacy = Политика за приватност
nav-terms = Услови за користење
nav-legal = Правни информации

leave-project-title = Да го напуштиш проектот?
leave-project-message = Ќе го изгубиш пристапот од овој уред. Ако не остане ниту еден член, проектот и сите негови трошоци трајно се бришат.

add-project-title = Нов проект
add-project-name-label = Име на проектот
add-project-name-placeholder = Моето патување, Цимери 2024…
add-project-participants = Учесници
add-project-participant-name = Име на учесникот
add-project-participant-placeholder = Кларк Кент
add-project-remove-participant = Отстрани учесник
add-project-me-badge = Јас
add-project-thats-me = Тоа сум јас!
add-project-offline = Не можеш да создадеш проект офлајн. Поврзи се повторно и обиди се пак.
add-project-name-required = Проектот треба име.
add-project-need-two-participants = Додај најмалку 2 учесници.
add-project-pick-yourself = Кажи ни кој учесник си ти.

join-link-label = Линк за споделување
join-link-hint = Линкот го содржи клучот за дешифрирање - копирај го целиот.
join-invalid-link = Тој линк не е важечки. Залепи го целиот линк за споделување, вклучувајќи го делот по #.
join-wrong-project = Тој линк е за друг проект.

import-tricount-link-label = Tricount линк или клуч
import-tricount-key-required = Внеси Tricount линк или клуч.
import-tricount-encryption-failed = Шифрирањето не успеа.

### Expenses

save = Зачувај
saving = Зачувување…
adding = Додавање…
link-copied = Линкот е копиран
missing-encryption-key = Недостасува клуч за шифрирање.
missing-encryption-key-title = Недостасува клуч за шифрирање
missing-encryption-key-hint = Линкот што го користеше не го содржи клучот потребен за дешифрирање на овој проект. Користи го целосниот линк споделен од оној што го создал.
project-locked-hint = Овој уред го нема клучот на овој проект. Отвори го неговиот линк за споделување за да го отклучиш.
project-unlock = Отклучи
project-no-local-data-hint = Најави се за да ги вчиташ податоците на овој проект за прв пат.
project-gone-title = Овој проект повеќе не постои
project-gone-hint = Беше избришан кога последниот член го напушти. Линкот за споделување повеќе не работи, дури и ако го отвориш повторно.

expense-add = Додај трошок
transfer-add = Додај трансфер
expense-edit-title = Уреди трошок
expense-category = Категорија
expense-category-auto = Авто · { $emoji }
expense-currency = Валута на износот
amount-op-add = Плус
amount-op-subtract = Минус
amount-op-multiply = Помножи
amount-op-divide = Подели
amount-op-equals = Еднакво
amount-op-done = Готово
expense-rate = Девизен курс (по избор)
expense-rate-hint = Остави празно за курсот на Европската комисија (InforEuro) за { $month }: 1 { $from } = { $rate } { $to }.
expense-rate-invalid = Внеси девизен курс поголем од 0.
expense-rate-unavailable = Нема автоматски курс - внеси го рачно.
expense-delete-title = Избриши трошок
expense-delete-message = „{ $name }“ ќе биде трајно избришан. Ова не може да се врати.
expense-inconsistent-amounts = Износите не се совпаѓаат
expenses-empty = Нема трошоци
expenses-empty-hint = Почни со додавање трошоци со копчето подолу
expenses-show-more = Прикажи повеќе (уште { $count })

expense-type-expense = Трошок
expense-type-transfer = Трансфер
expense-type-gain = Приход
expense-paid-by = платено од
expense-sent-by = испратено од
expense-contributed-by = придонесено од

expense-name-required = Името е задолжително.
expense-amount-not-positive = Износот мора да биде поголем од 0.
expense-no-payer = Избери барем еден плаќач.
expense-no-debtor = Избери барем едно лице кое должи.
expense-invalid-date = Тој датум не е важечки.
expense-payers-mismatch = Збирот на плаќачите е { $sum }, што не одговара на износот на трошокот ({ $total }).
expense-debtors-mismatch = Збирот на должниците е { $sum }, што не одговара на износот на трошокот ({ $total }).

participants-none = Никој
participants-everyone = Сите ({ $count })
participants-some = { $count } од { $total }
participants-select-all = Избери сè
participants-by-shares = По удели
split-amounts = Износи
participants-remaining = Преостанува { $amount }
participants-over-by = { $amount } повеќе
participants-who-paid = Кој платил?
participants-who-received = Кој примил?
participants-who-transfers = Кој префрла?
participants-who-receives = Кој прима?
participants-for-whom = За кого?

stats-total-expenses = Вкупни трошоци
stats-my-expenses = Моите трошоци

tab-expenses = Трошоци
tab-balance = Состојба
tab-reimbursements = Порамнување
reimbursements-empty-title = Сè е порамнето!
reimbursements-empty-hint = Предлози за порамнување се појавуваат овде кога сметките не се балансирани
reimbursement-owes = { $debtor } должи на { $creditor }
reimbursement-record = Порамни
reimbursement-pay-with = Плати
reimbursement-pay-shared-by = Споделено од { $name } - провери го името на примачот што го прикажува твојата апликација пред да испратиш.
reimbursement-pay-title = Плати на { $name }
reimbursements-mine-title = Ти должиш
reimbursements-others-title = Други враќања
copy = Копирај

user-selection-title = Кој учесник си ти?
user-selection-hint = Избери го своето име од листата.
user-selection-required = Избери учесник.
identity-claimed = Поврзано со сметка
identity-claimed-by = Сметка на { $name }
identity-taken-repick = Друга сметка го презеде учесникот што го користеше. Избери друг.
participant-gone-repick = Учесникот што го користеше е отстранет од овој проект. Избери друг.

edit-project-title = Уреди проект
edit-project-new-badge = ново
edit-project-deferred-new-members = додавањето нови членови
edit-project-deferred-removals = отстранувањето членови
edit-project-deferred-me = изборот „Тоа сум јас“
edit-project-offline-deferred = Офлајн: { $items } ќе се примени при повторно поврзување.

export-saved = Датотеката е зачувана:
    { $path }
export-failed = Извозот не успеа: { $reason }

history-expense-added = Додаден трошок: { $name }
history-expense-edited = Уреден трошок: { $name }
history-expense-deleted = Избришан трошок: { $name }
history-project-edited = Уреден проект: { $name }
history-name-changed = Име: „{ $from }“ → „{ $to }“
history-description-added = Додаден опис: „{ $value }“
history-description-removed = Отстранет опис: „{ $value }“
history-description-changed = Опис: „{ $from }“ → „{ $to }“

### Sweep

field-amount = Износ
expense-name-placeholder = Ресторан, намирници…
expense-actions = Дејства за трошокот
expense-your-share = Твојот удел
expense-your-share-value = Твојот удел: { $amount } { $currency }
expense-inconsistent-detail = Износите не се совпаѓаат: { $paid } платено, { $owed } должено, за трошок од { $total }. Уреди го трошокот за да го поправиш.
missing-access-key = Недостасува клуч за пристап. Отвори го овој проект преку неговиот линк за споделување.
filter-all = Сите
filter-my-payments = Моите плаќања
filter-my-debts = Што должам
participants-shares-for = Удели за { $name }
participants-amount-for = Износ за { $name }
reimbursement-add = Додај порамнување
project-forget = Отстрани од мојата листа
project-history-title = Историја
history-kind-add = Додадено
history-kind-delete = Избришано
history-kind-edit = Уредено
export = Извези
export-json = Извези JSON
export-csv = Извези CSV
share-link = Сподели
copy-link-failed = Линкот не можеше да се копира
open-in-app = Отвори во апликацијата
not-found-title = Страницата не е пронајдена
not-found-back = Назад кон проектите

### Charts

charts-period = Период
period-all = Сè
period-month = Месец
period-3months = 3 мес.
period-year = Година
period-custom = По избор
charts-tab-categories = Категории
charts-tab-per-person = По лице
charts-tab-trends = Трендови
charts-by-category = Распределба по категории
charts-per-person = Трошоци по лице
charts-categories-by-month = Категории по месеци
charts-total-spent = Вкупно потрошено
charts-avg-per-person = Просек по лице
charts-expense-count =
    { $count ->
        [one] { $count } трошок
       *[other] { $count } трошоци
    }
charts-clear-category-filter = Исчисти го филтерот за категорија
charts-no-expenses = Нема трошоци.
charts-pick-a-project = Избери проект за да ги видиш трошоците по лице.
charts-nothing-to-show = Нема што да се прикаже
charts-my-share-note = Овие износи се твојот удел во секој трошок.
charts-my-share-skipped =
    { $count ->
        [one] 1 проект не е засметан — не е избран учесник или неговите податоци не се вчитаа.
       *[other] { $count } проекти не се засметани — не е избран учесник или нивните податоци не се вчитаа.
    }

### Categories

category-food = Храна
category-transport = Превоз
category-accommodation = Сместување
category-leisure = Слободно време
category-shopping = Шопинг
category-services = Услуги
category-parties-gifts = Забави и подароци
category-other = Друго
charts-person = Лице
charts-project = Проект
charts-all-projects = Сите проекти
charts-whole-project = Целиот проект
charts-date-from = Од
charts-date-to = До
charts-total = Вкупно
charts-payments-per-person-by-month = Плаќања по лице по месеци
history-empty = Нема настани
history-by = Од { $name }
not-found-hint = Оваа страница не постои или е преместена.
payers-title-paid-by = Платено од
payers-title-sender = Испраќач
payers-title-contributors = Придонесувачи
debtors-title-debtors = Должи
debtors-title-recipients = Примачи
debtors-title-beneficiaries = Корисници

### Welcome

welcome-title = Твоите сметки не се ничија работа.
welcome-subtitle = Дели трошоци со пријатели.
welcome-e2ee-title = Сè е шифрирано
welcome-e2ee-body = Имиња, износи, проекти: сè се шифрира на твојот уред. Само ти го имаш клучот. Никој не може да ги чита твоите сметки. Ни ние.
welcome-e2ee-note = Нечитливо дури и за нас (нулти пристап на серверот)
welcome-eu-title = 100 % европски
welcome-eu-body = Сервери во Германија, е-пошта испратена од Франција. Твоите податоци никогаш не ја напуштаат Европската унија.
welcome-noads-title = Без реклами. Без следачи.
welcome-noads-body = Ништо не собираме и не ги продаваме твоите податоци. Тоа не е нашиот модел.
welcome-start = Започни
welcome-how-it-works = Како точно работи?

### Help

help-intro = Често прашање? Допри за да го отвориш одговорот.
help-create-project-q = Како да создадам проект?
help-create-project-a = Од почетниот екран допри го копчето + долу. Дај му име на проектот, избери валута и готово.
help-add-participants-q = Како да додадам учесници?
help-add-participants-a = Отвори го проектот, па додај учесници од листата на членови. Секој учесник може да плати или да должи за трошок.
help-share-project-q = Како да споделам проект?
help-share-project-a = Сподели го URL-то на проектот (тоа во адресната лента). Секој со линкот може да го гледа и уредува проектот.
help-add-expense-q = Како да додадам трошок?
help-add-expense-a = Во проект допри +, внеси го износот, кажи кој платил и меѓу кого да се подели. Можеш и да избереш датум различен од денешниот.
help-types-q = Која е разликата меѓу трошок, трансфер и приход?
help-types-expense = - купување направено од едно лице и поделено меѓу повеќе.
help-types-transfer = - враќање пари од едно лице на друго, без делење.
help-types-gain = - примени пари (повраток, подарок) за делење меѓу повеќе лица.
help-past-date-q = Можам ли да датирам трошок во минатото?
help-past-date-a = Да, полето за датум е слободно. Времето на создавање на записот се чува одделно.
help-who-owes-q = Како Counted пресметува кој што должи?
help-who-owes-a = Counted го пресметува нето салдото на секој учесник (што платил однапред минус што должи), па предлага најкратка низа трансфери што ги порамнува сите.
help-minimal-transfers-q = Зошто бројот на предложени трансфери е минимален?
help-minimal-transfers-a = Алгоритмот прво спарува салда што точно се поништуваат, па го обработува остатокот од најголемиот доверител до најголемиот должник. Резултат: помалку трансфери за да се порамни сè.
help-import-tricount-q = Како да увезам проект од Tricount?
help-import-tricount-a = Од почетниот екран допри го копчето „+“ долу, потоа
help-import-tricount-b = Залепи го линкот за споделување на Tricount што сакаш да го увезеш.
help-encryption-q = Дали моите податоци се шифрирани?
help-encryption-a = Да. Counted комбинира две гаранции:
help-encryption-e2ee-term = Шифрирање од крај до крај
help-encryption-e2ee-def = - сè меѓу тебе и серверот патува шифрирано.
help-encryption-zero-term = Нулти пристап
help-encryption-zero-def = - ти ги шифрираш податоците пред да ги испратиш, а серверот чува само шифриран текст. Немаме начин да го прочитаме.
help-encryption-see = За детали, види ја
help-forgot-password-q = Што се случува ако ја заборавам лозинката?
help-forgot-password-warning = Твоите податоци ќе бидат трајно изгубени.
help-forgot-password-a = Клучот за шифрирање се изведува од твојата лозинка, па ресетирање не е можно: никој - вклучително и ние - не може да ги дешифрира твоите проекти без неа. Чувај ја безбедно, идеално во менаџер за лозинки.
help-archive-delete-q = Како да архивирам или избришам проект?
help-archive-delete-a = Од екранот на проектот отвори го менито и избери
help-archive-delete-b = за да го сокриеш, а да го задржиш. Проектот трајно се брише кога последниот член ќе го напушти.
help-delete-account-q = Како да ја избришам сметката?
help-delete-account-a = Отвори Поставки и користи „Избриши ја мојата сметка“. Веднаш е и не може да се врати.
help-contact = Друго прашање? Пиши ни на

# Receipt scanning (mobile only)
expense-scan = Скенирај сметка
scan-in-progress = Читање на сметката…
scan-error-capture = Не можеше да се направи таа фотографија. Обиди се повторно или внеси го трошокот рачно.
scan-error-unreadable = Ништо читливо на таа сметка. Внеси го трошокот рачно.
scan-check-amount = Провери го вкупниот износ - не беше јасно отпечатен.
scan-take-photo = Направи фотографија
scan-choose-photo = Избери фотографија
expense-converted-from = Платено { $amount } { $from } · 1 { $from } = { $rate } { $to }
project-currency = Валута
project-currency-hint = Секој износ се прикажува во оваа валута. Не може да се промени подоцна.
project-currency-locked = Валутата се определува при создавањето на проектот.

update-required-title = Потребно е ажурирање
update-required-body = Оваа верзија на Counted е престара за да комуницира со серверот. Ажурирај ја за да продолжиш да ја користиш апликацијата.
update-required-button = Ажурирај

notifications-label = Известувања
notifications-title = Известувања
notifications-empty = Ништо ново
notifications-friend-request = Барање за пријателство

friends-title = Пријатели
friends-anonymous-body = Пријателите се чуваат со твојата сметка. Најави се за да додаваш луѓе и да ги покануваш во своите проекти без споделување линк.
friends-add-title = Додај пријател
friends-add-hint = Ќе го види твоето барање кога ќе се најави. Никој од вас не дознава дали другиот има сметка додека барањето не се прифати.
friends-add-button = Додај
friends-add-from-project = Додај како пријател
friends-request-sent = Барањето е испратено
friends-no-account-key = Најави се повторно за да управуваш со пријателите на овој уред.
friends-incoming-title = Барања
friends-accept = Прифати
friends-decline = Одбиј
friends-list-title = Моите пријатели
friends-list-empty = Сè уште нема пријатели. Додај некого преку е-пошта погоре, или од проект што го споделувате.
friends-remove = Отстрани
friends-remove-confirm-title = Отстрани пријател
friends-remove-confirm-message = { $email } веќе нема да биде меѓу вашите пријатели, ниту вие меѓу неговите. Секој од вас може подоцна да испрати ново барање.
friends-no-key = Сè уште не е подготвено
friends-fingerprint = Безбедносен код
friends-fingerprint-hint = Двајца пријатели кои си го читаат истиот безбедносен код знаат дека никој не стои меѓу нив - дури ни нашиот сервер.
friends-outgoing-title = Испратени
friends-outgoing-hint = Чека одговор. Ќе ги видиш меѓу пријателите штом прифатат.
friends-withdraw = Откажи
invite-friends-title = Покани пријатели
invite-friends-hint = Клучот на проектот се шифрира за секој пријател на овој уред. Серверот никогаш не го гледа.
invite-friends-empty = Сè уште нема пријатели за покана.
invite-friends-button = Покани
invite-sent = { $count ->
    [one] Поканата е испратена
   *[other] Испратени се { $count } покани
}
invitation-badge = Покана
invitation-to = Приклучи се на „{ $name }“
invitation-to-unnamed = Приклучи се на проект
invitation-unreadable = Оваа покана не може да се отвори на овој уред
invitation-from = Од { $email }
invitation-accept = Приклучи се
invitation-decline = Одбиј
