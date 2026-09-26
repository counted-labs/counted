# Íslenska. Heil að undanskildum lagatextum (legal-, terms-, privacy-), sem eru aðeins til á ensku og
# frönsku og falla aftur á en.ftl skilaboð fyrir skilaboð.

### Common

loading = Hleður…
cancel = Hætta við
confirm = Staðfesta
retry = Reyna aftur
delete = Eyða
back = Til baka
language = Tungumál

### Navigation

nav-main = Aðalleiðsögn
nav-projects = Verkefni
nav-charts = Tölfræði
nav-settings = Stillingar

### Connectivity

offline-banner = Án nettengingar
offline-pending =
    { $count ->
        [one] { $count } í bið
       *[other] { $count } í bið
    }

sync-conflict-edit = Árekstur: breyting á „{ $name }“ mistókst (atriði eytt). Sleppt.
sync-conflict-delete = Árekstur: eyðing á „{ $name }“ mistókst (atriði eytt). Sleppt.
sync-conflict-other = Árekstur: aðgerð á „{ $name }“ mistókst (atriði eytt). Sleppt.
sync-error = Samstillingarvilla: { $reason }

### Errors

error-network = Næ ekki sambandi við þjóninn. Athugaðu nettenginguna þína.
error-generic = Eitthvað fór úrskeiðis. Reyndu aftur.

error-invalid-email = Þetta netfang er ekki gilt.
error-invalid-password = Þetta lykilorð er ekki gilt.
error-password-too-short = Lykilorðið verður að vera að minnsta kosti 8 stafir.
error-client-outdated = Þessi útgáfa forritsins er úrelt. Uppfærðu hana til að skrá þig inn.
error-invalid-link = Þessi tengill er ekki gildur.
error-batch-too-large = Of mörg atriði í einu.
error-payers-required = Veldu að minnsta kosti einn greiðanda.
error-debtors-required = Veldu að minnsta kosti einn sem skuldar.
error-duplicate-participant = Þátttakandi kemur tvisvar fyrir sömu megin.
error-participant-not-in-project = Þessi þátttakandi er ekki í verkefninu.
error-too-many-participants = Of margir þátttakendur fyrir einn útgjaldalið.
error-invalid-credentials = Rangt netfang eða lykilorð.
error-unauthenticated = Skráðu þig inn til að gera þetta.
error-email-not-verified = Netfangið þitt hefur ekki enn verið staðfest.
error-project-not-found = Þetta verkefni er ekki lengur til.
error-expense-not-found = Þessi útgjöld eru ekki lengur til.
error-user-not-found = Þessi þátttakandi er ekki lengur til.
error-tricount-not-found = Tricount fannst ekki, eða API þess skilaði villu.
error-too-many-members = Þetta verkefni hefur náð hámarksfjölda meðlima.
error-identity-taken = Annar aðgangur hefur þegar gert tilkall til þessa þátttakanda.
error-claim-proof-invalid = Þetta tæki hefur ekki lykil verkefnisins og getur því ekki gert tilkall til þátttakanda. Opnaðu deilitengilinn aftur.
error-user-has-payments = Þessi þátttakandi á útgjöld í verkefninu og ekki er hægt að fjarlægja hann.
error-resend-cooldown = Bíddu í 60 sekúndur áður en þú biður um annan tölvupóst.
error-self-friend-request = Þú getur ekki bætt sjálfum þér við sem vini.
error-not-a-friend = Þú getur aðeins boðið fólki af vinalistanum þínum.
error-friend-has-no-key = Þessi vinur hefur ekki enn opnað nýjustu útgáfu forritsins. Biddu hann að skrá sig inn einu sinni og reyndu svo aftur.
error-friend-request-not-found = Þessi vinabeiðni er ekki lengur til.
error-invitation-not-found = Þetta boð er ekki lengur til.
error-too-many-friend-requests = Of margar vinabeiðnir í bili. Reyndu aftur á morgun.
error-too-many-invitations = Of mörg boð í bið.
error-invalid-kdf-salt = Dulkóðunarstillingarnar eru ógildar. Uppfærðu forritið og reyndu aftur.
error-mixed-project-batch = Þessir þátttakendur eru ekki allir í sama verkefni.
error-invalid-payload = Þessi útgáfa forritsins sendi gögn sem netþjónninn tekur ekki við. Uppfærðu hana og reyndu aftur.
error-invalid-public-key = Dulkóðunarlykillinn þinn er ógildur. Uppfærðu forritið og reyndu aftur.
error-payment-methods-stale = Greiðsluupplýsingunum þínum var breytt í öðru tæki. Endurhlaðu og reyndu aftur.

### Auth

field-email = Netfang
field-email-placeholder = thu@daemi.is
field-password = Lykilorð
field-name = Nafn
field-name-placeholder = Anna Jónsdóttir

login-title = Skrá inn
login-submit = Skrá inn
login-submitting = Skráir inn…
login-password-placeholder = Lykilorðið þitt
login-no-account = Ertu ekki með aðgang?
login-unverified = Netfangið þitt hefur ekki enn verið staðfest. Athugaðu pósthólfið eða sendu tengilinn aftur.
login-resend = Senda staðfestingartengil aftur
login-resending = Sendir…
login-resend-sent = Tölvupóstur sendur - athugaðu pósthólfið þitt.

register-submit = Stofna aðgang
register-submitting = Stofnar…
register-have-account = Ertu þegar með aðgang?
register-password-placeholder = Að minnsta kosti 8 stafir
register-password-warning = Skrifaðu lykilorðið þitt niður. Ef þú gleymir því er ekki hægt að endurheimta aðganginn.
register-check-email-title = Athugaðu tölvupóstinn þinn
register-email-sent = Tölvupóstur sendur
register-email-sent-hint = Smelltu á tengilinn í pósthólfinu til að virkja aðganginn.
register-not-received-prefix = Fékkstu hann ekki? Athugaðu ruslpóstinn eða
register-sign-in-link = skráðu þig inn
register-not-received-suffix = til að senda tengilinn aftur.
register-terms-prefix = Með því að stofna aðgang samþykkir þú
register-terms-link = notkunarskilmála
register-terms-and = og
register-privacy-link = persónuverndarstefnu okkar

settings-title = Stillingar
settings-preferences = Kjörstillingar
settings-preferences-local = Vistað á þessu tæki.
settings-preferences-synced = Samstillt við aðganginn þinn, dulkóðað.
settings-about = Um
settings-anonymous-title = Þú ert ekki skráð(ur) inn
settings-upsell-title = Verkefnin þín, á öllum tækjum
settings-upsell-free = Ókeypis
settings-upsell-body = Counted virkar án aðgangs. Með ókeypis aðgangi fylgja verkefnin þín og kjörstillingar þér á símann, tölvuna og vefinn - enn dulkóðað, enn ólæsilegt fyrir okkur.
settings-locked-badge = Aðgangur
settings-locked-friends = Stofnaðu aðgang til að bæta við vinum og bjóða þeim í verkefni úr forritinu - enginn tengill til að senda.
settings-locked-payment-methods = Vistaðu IBAN-númerið þitt eða greiðsluforrit einu sinni og deildu því með þeim verkefnum sem þú velur. Sá sem skuldar þér sér það við hlið nafnsins þíns.
settings-friends-hint = Bættu við vinum og bjóddu þeim í verkefnin þín án þess að deila tengli.

account-member-since = Meðlimur síðan
account-logout = Skrá út
account-logging-out = Skráir út…
account-delete-title = Eyða aðganginum mínum
account-delete-warning = Strax og varanlega, engin ruslafata. Útgjöld sem þú skráðir í sameiginlegt verkefni verða áfram sýnileg öðrum meðlimum - þau eru hluti af þeirra bókhaldi.
account-delete-confirm-title = Eyða aðgangi
account-delete-confirm-message = Aðgangi þínum, lotum og verkefnalista verður eytt varanlega. Án lykilorðsins verða dulkóðuð gögn sameiginlegs verkefnis ólæsileg fyrir þig - þetta er ekki hægt að afturkalla.

settings-payment-methods = Greiðsluupplýsingar
settings-payment-methods-hint = Hvernig þú vilt fá endurgreitt. Dulkóðað með aðganginum þínum.
payment-method-kind = Aðferð
payment-method-kind-other = Annað
payment-method-label = Nafn
payment-method-label-placeholder = Aðalreikningur
payment-method-value = Upplýsingar
payment-method-value-placeholder = IBAN, símanúmer, notandanafn…
payment-method-add = Bæta við
payment-method-remove = Fjarlægja { $name }
payment-method-empty = Þú hefur ekki enn bætt við greiðsluupplýsingum.
payment-method-deleted = Greiðslumáta eytt.
payment-method-value-required = Fylltu út upplýsingar hvers greiðslumáta, eða fjarlægðu hann.
payment-method-label-required = Gefðu sérsniðna greiðslumátanum nafn.
payment-method-too-long = Þetta er of langt - styttu það.
payment-method-invalid-characters = Fjarlægðu línubil eða ósýnilega stafi.
payment-method-limit = Þú getur vistað allt að { $max } greiðslumáta.
payment-methods-saved = Greiðsluupplýsingar vistaðar.
payment-methods-offline = Þú þarft að vera nettengd(ur) til að vista greiðsluupplýsingar.
payment-methods-stale = Greiðsluupplýsingum þínum var breytt á öðru tæki. Þær hafa verið endurhlaðnar — reyndu aftur.
payment-methods-key-missing = Skráðu þig inn aftur til að stjórna greiðsluupplýsingum.
settings-payment-methods-share-warning = Deildur greiðslumáti er sýnilegur öllum meðlimum þeirra verkefna þar sem þú hefur valið nafnið þitt - öllum sem hafa einn af þessum tenglum.
payment-method-share = Deila með verkefnunum mínum
payment-method-share-hint = Sýnt við hlið nafnsins þíns þegar einhver skuldar þér.
payment-method-copy = Afrita { $name }
payment-method-copied = Afritað.
payment-method-copy-failed = Tókst ekki að afrita - veldu textann og afritaðu hann handvirkt.

verify-email-checking = Staðfesti netfangið þitt…
verify-email-welcome = Netfang staðfest - velkomin(n) í Counted!
verify-email-back-to-login = Aftur í innskráningu

### Project status

project-close = Loka
project-archive = Setja í geymslu
project-reopen = Opna aftur
project-unarchive = Taka úr geymslu

### Dates

date-long = { $day }. { $month } { $year }

month-1 = janúar
month-2 = febrúar
month-3 = mars
month-4 = apríl
month-5 = maí
month-6 = júní
month-7 = júlí
month-8 = ágúst
month-9 = september
month-10 = október
month-11 = nóvember
month-12 = desember

month-short-1 = jan
month-short-2 = feb
month-short-3 = mar
month-short-4 = apr
month-short-5 = maí
month-short-6 = jún
month-short-7 = júl
month-short-8 = ágú
month-short-9 = sep
month-short-10 = okt
month-short-11 = nóv
month-short-12 = des

### Actions

add = Bæta við
create = Stofna
creating = Stofnar…
edit = Breyta
leave = Yfirgefa
close = Loka
paste = Líma
join = Taka þátt
import = Flytja inn
importing = Flytur inn…
field-description = Lýsing
field-date = Dagsetning
date-today = Í dag
date-yesterday = Í gær
field-optional = Valfrjálst

### Projects

projects-filter-active = Virk
projects-filter-all = Öll
projects-count-label = Verkefni
projects-empty = Engin verkefni
projects-empty-hint = Stofnaðu verkefni með hnappinum hér að neðan
projects-offline-banner = Gögn án nettengingar - tengstu aftur til að uppfæra.
projects-no-local-data = Engin staðbundin gögn
projects-no-local-data-hint = Skráðu þig inn til að hlaða verkefnunum þínum í fyrsta sinn.
projects-add = Bæta við verkefni
projects-create = Stofna verkefni
projects-join = Taka þátt í verkefni
projects-import-tricount = Flytja inn úr Tricount
project-actions = Aðgerðir verkefnis

status-ongoing = Í gangi
status-closed = Lokað
status-archived = Í geymslu

nav-help = Hjálp
nav-privacy = Persónuverndarstefna
nav-terms = Notkunarskilmálar
nav-legal = Lagalegar upplýsingar

leave-project-title = Yfirgefa verkefnið?
leave-project-message = Þú missir aðgang frá þessu tæki. Ef enginn meðlimur er eftir er verkefninu og öllum útgjöldum þess eytt varanlega.

add-project-title = Nýtt verkefni
add-project-name-label = Heiti verkefnis
add-project-name-placeholder = Ferðin mín, Sambýlið 2024…
add-project-participants = Þátttakendur
add-project-participant-name = Nafn þátttakanda
add-project-participant-placeholder = Clark Kent
add-project-remove-participant = Fjarlægja þátttakanda
add-project-me-badge = Ég
add-project-thats-me = Þetta er ég!
add-project-offline = Ekki er hægt að stofna verkefni án nettengingar. Tengstu aftur og reyndu á ný.
add-project-name-required = Verkefnið þarf heiti.
add-project-need-two-participants = Bættu við að minnsta kosti 2 þátttakendum.
add-project-pick-yourself = Segðu okkur hvaða þátttakandi þú ert.

join-link-label = Deilitengill
join-link-hint = Tengillinn inniheldur afkóðunarlykilinn - afritaðu hann allan.
join-invalid-link = Þessi tengill er ekki gildur. Límdu allan deilitengilinn, þar með talið hlutann á eftir #.
join-wrong-project = Þessi tengill er fyrir annað verkefni.

import-tricount-link-label = Tricount-tengill eða -lykill
import-tricount-key-required = Sláðu inn Tricount-tengil eða -lykil.
import-tricount-encryption-failed = Dulkóðun mistókst.

### Expenses

save = Vista
saving = Vistar…
adding = Bætir við…
link-copied = Tengill afritaður
missing-encryption-key = Dulkóðunarlykil vantar.
missing-encryption-key-title = Dulkóðunarlykil vantar
missing-encryption-key-hint = Tengillinn sem þú notaðir inniheldur ekki lykilinn sem þarf til að afkóða þetta verkefni. Notaðu allan tengilinn sem sá sem stofnaði það deildi.
project-locked-hint = Þetta tæki hefur ekki lykil þessa verkefnis. Opnaðu deilitengil þess til að aflæsa því.
project-unlock = Aflæsa
project-no-local-data-hint = Skráðu þig inn til að hlaða gögnum þessa verkefnis í fyrsta sinn.
project-gone-title = Þetta verkefni er ekki lengur til
project-gone-hint = Því var eytt þegar síðasti meðlimurinn yfirgaf það. Deilitengillinn virkar ekki lengur, jafnvel þótt þú opnir hann aftur.

expense-add = Bæta við útgjöldum
transfer-add = Bæta við millifærslu
expense-edit-title = Breyta útgjöldum
expense-category = Flokkur
expense-category-auto = Sjálfvirkt · { $emoji }
expense-currency = Gjaldmiðill upphæðar
amount-op-add = Plús
amount-op-subtract = Mínus
amount-op-multiply = Margfalda
amount-op-divide = Deila
amount-op-equals = Jafnt og
amount-op-done = Lokið
expense-rate = Gengi (valfrjálst)
expense-rate-hint = Skildu eftir autt til að nota gengi Evrópusambandsins (InforEuro) fyrir { $month }: 1 { $from } = { $rate } { $to }.
expense-rate-invalid = Sláðu inn gengi hærra en 0.
expense-rate-unavailable = Ekkert sjálfvirkt gengi í boði - sláðu það inn handvirkt.
expense-delete-title = Eyða útgjöldum
expense-delete-message = „{ $name }“ verður eytt varanlega. Þetta er ekki hægt að afturkalla.
expense-inconsistent-amounts = Upphæðir stemma ekki
expenses-empty = Engin útgjöld
expenses-empty-hint = Byrjaðu á að bæta við útgjöldum með hnappinum hér að neðan
expenses-show-more = Sýna meira ({ $count } eftir)

expense-type-expense = Útgjöld
expense-type-transfer = Millifærsla
expense-type-gain = Innkoma
expense-paid-by = greitt af
expense-sent-by = sent af
expense-contributed-by = lagt til af

expense-name-required = Heiti er nauðsynlegt.
expense-amount-not-positive = Upphæðin verður að vera hærri en 0.
expense-no-payer = Veldu að minnsta kosti einn greiðanda.
expense-no-debtor = Veldu að minnsta kosti einn sem skuldar.
expense-invalid-date = Þessi dagsetning er ekki gild.
expense-payers-mismatch = Greiðendur nema samtals { $sum }, sem passar ekki við upphæð útgjaldanna ({ $total }).
expense-debtors-mismatch = Skuldarar nema samtals { $sum }, sem passar ekki við upphæð útgjaldanna ({ $total }).

participants-none = Enginn
participants-everyone = Allir ({ $count })
participants-some = { $count } af { $total }
participants-select-all = Velja alla
participants-by-shares = Eftir hlutum
split-amounts = Upphæðir
participants-remaining = { $amount } eftir
participants-over-by = { $amount } umfram
participants-who-paid = Hver borgaði?
participants-who-received = Hver fékk?
participants-who-transfers = Hver millifærir?
participants-who-receives = Hver tekur við?
participants-for-whom = Fyrir hvern?

stats-total-expenses = Heildarútgjöld
stats-my-expenses = Mín útgjöld

tab-expenses = Útgjöld
tab-balance = Staða
tab-reimbursements = Gera upp
reimbursements-empty-title = Allt gert upp!
reimbursements-empty-hint = Uppgjörstillögur birtast hér þegar reikningar stemma ekki
reimbursement-owes = { $debtor } skuldar { $creditor }
reimbursement-record = Gera upp
reimbursement-pay-with = Borga
reimbursement-pay-shared-by = Deilt af { $name } - athugaðu nafn viðtakanda sem forritið þitt sýnir áður en þú sendir.
reimbursement-pay-title = Borga { $name }
reimbursements-mine-title = Þú skuldar
reimbursements-others-title = Aðrar endurgreiðslur
copy = Afrita

user-selection-title = Hvaða þátttakandi ert þú?
user-selection-hint = Veldu nafnið þitt af listanum.
user-selection-required = Veldu þátttakanda.
identity-claimed = Tengt við aðgang
identity-claimed-by = Aðgangur { $name }
identity-taken-repick = Annar aðgangur hefur gert tilkall til þátttakandans sem þú notaðir. Veldu annan.
participant-gone-repick = Þátttakandinn sem þú notaðir hefur verið fjarlægður úr þessu verkefni. Veldu annan.

edit-project-title = Breyta verkefni
edit-project-new-badge = nýtt
edit-project-deferred-new-members = að bæta við nýjum meðlimum
edit-project-deferred-removals = að fjarlægja meðlimi
edit-project-deferred-me = valið „Þetta er ég“
edit-project-offline-deferred = Án nettengingar: { $items } tekur gildi þegar þú tengist aftur.

export-saved = Skrá vistuð:
    { $path }
export-failed = Útflutningur mistókst: { $reason }

history-expense-added = Útgjöldum bætt við: { $name }
history-expense-edited = Útgjöldum breytt: { $name }
history-expense-deleted = Útgjöldum eytt: { $name }
history-project-edited = Verkefni breytt: { $name }
history-name-changed = Heiti: „{ $from }“ → „{ $to }“
history-description-added = Lýsingu bætt við: „{ $value }“
history-description-removed = Lýsing fjarlægð: „{ $value }“
history-description-changed = Lýsing: „{ $from }“ → „{ $to }“

### Sweep

field-amount = Upphæð
expense-name-placeholder = Veitingastaður, matvörur…
expense-actions = Aðgerðir útgjalda
expense-your-share = Þinn hluti
expense-your-share-value = Þinn hluti: { $amount } { $currency }
expense-inconsistent-detail = Upphæðir stemma ekki: { $paid } greitt, { $owed } skuldað, fyrir útgjöld upp á { $total }. Breyttu útgjöldunum til að laga það.
missing-access-key = Aðgangslykil vantar. Opnaðu þetta verkefni í gegnum deilitengil þess.
filter-all = Allt
filter-my-payments = Mínar greiðslur
filter-my-debts = Það sem ég skulda
participants-shares-for = Hlutir fyrir { $name }
participants-amount-for = Upphæð fyrir { $name }
reimbursement-add = Bæta við uppgjöri
project-forget = Fjarlægja af listanum mínum
project-history-title = Saga
history-kind-add = Bætt við
history-kind-delete = Eytt
history-kind-edit = Breytt
export = Flytja út
export-json = Flytja út JSON
export-csv = Flytja út CSV
share-link = Deila
copy-link-failed = Tókst ekki að afrita tengilinn
open-in-app = Opna í forritinu
not-found-title = Síða fannst ekki
not-found-back = Aftur í verkefni

### Charts

charts-period = Tímabil
period-all = Allt
period-month = Mánuður
period-3months = 3 mán.
period-year = Ár
period-custom = Sérsniðið
charts-tab-categories = Flokkar
charts-tab-per-person = Á mann
charts-tab-trends = Þróun
charts-by-category = Sundurliðun eftir flokkum
charts-per-person = Útgjöld á mann
charts-categories-by-month = Flokkar eftir mánuðum
charts-total-spent = Samtals eytt
charts-avg-per-person = Meðaltal á mann
charts-expense-count =
    { $count ->
        [one] { $count } útgjaldaliður
       *[other] { $count } útgjaldaliðir
    }
charts-clear-category-filter = Hreinsa flokkasíu
charts-no-expenses = Engin útgjöld.
charts-pick-a-project = Veldu verkefni til að sjá útgjöld á mann.
charts-nothing-to-show = Ekkert að sýna
charts-my-share-note = Þessar tölur eru þinn hluti af hverjum útgjaldalið.
charts-my-share-skipped =
    { $count ->
        [one] 1 verkefni er ekki talið með — enginn þátttakandi valinn eða gögn þess hlóðust ekki.
       *[other] { $count } verkefni eru ekki talin með — enginn þátttakandi valinn eða gögn þeirra hlóðust ekki.
    }

### Categories

category-food = Matur
category-transport = Samgöngur
category-accommodation = Gisting
category-leisure = Afþreying
category-shopping = Verslun
category-services = Þjónusta
category-parties-gifts = Veislur og gjafir
category-other = Annað
charts-person = Manneskja
charts-project = Verkefni
charts-all-projects = Öll verkefni
charts-whole-project = Allt verkefnið
charts-date-from = Frá
charts-date-to = Til
charts-total = Samtals
charts-payments-per-person-by-month = Greiðslur á mann eftir mánuðum
history-empty = Engir atburðir
history-by = Af { $name }
not-found-hint = Þessi síða er ekki til eða hefur verið færð.
payers-title-paid-by = Greitt af
payers-title-sender = Sendandi
payers-title-contributors = Þátttakendur
debtors-title-debtors = Skuldar
debtors-title-recipients = Viðtakendur
debtors-title-beneficiaries = Njótendur

### Welcome

welcome-title = Bókhaldið þitt kemur engum öðrum við.
welcome-subtitle = Skiptu útgjöldum með vinum.
welcome-e2ee-title = Allt dulkóðað
welcome-e2ee-body = Nöfn, upphæðir, verkefni: allt er dulkóðað á tækinu þínu. Þú ein(n) hefur lykilinn. Enginn getur lesið reikningana þína. Ekki einu sinni við.
welcome-e2ee-note = Ólæsilegt, jafnvel fyrir okkur (enginn aðgangur þjóns)
welcome-eu-title = 100% evrópskt
welcome-eu-body = Þjónar í Þýskalandi, tölvupóstur sendur frá Frakklandi. Gögnin þín fara aldrei út fyrir Evrópusambandið.
welcome-noads-title = Engar auglýsingar. Engir rekjarar.
welcome-noads-body = Við söfnum engu og seljum ekki gögnin þín. Það er ekki okkar viðskiptamódel.
welcome-start = Byrja
welcome-how-it-works = Hvernig virkar þetta nákvæmlega?

### Help

help-intro = Algeng spurning? Ýttu til að sjá svarið.
help-create-project-q = Hvernig stofna ég verkefni?
help-create-project-a = Á heimaskjánum ýtirðu á +-hnappinn neðst. Gefðu verkefninu heiti, veldu gjaldmiðil og þá ertu til.
help-add-participants-q = Hvernig bæti ég við þátttakendum?
help-add-participants-a = Opnaðu verkefnið og bættu við þátttakendum af meðlimalistanum. Hver þátttakandi getur borgað fyrir eða skuldað vegna útgjalda.
help-share-project-q = Hvernig deili ég verkefni?
help-share-project-a = Deildu vefslóð verkefnisins (þeirri í veffangastikunni). Allir sem hafa tengilinn geta skoðað og breytt verkefninu.
help-add-expense-q = Hvernig bæti ég við útgjöldum?
help-add-expense-a = Inni í verkefni ýtirðu á +, slærð inn upphæðina, hver borgaði og á milli hverra á að skipta. Þú getur líka valið aðra dagsetningu en daginn í dag.
help-types-q = Hver er munurinn á útgjöldum, millifærslu og innkomu?
help-types-expense = - kaup sem ein manneskja gerði og skipt er á milli nokkurra.
help-types-transfer = - endurgreiðsla frá einni manneskju til annarrar, án skiptingar.
help-types-gain = - peningar sem fengust (endurgreiðsla, gjöf) til að skipta á milli nokkurra.
help-past-date-q = Get ég dagsett útgjöld aftur í tímann?
help-past-date-a = Já, dagsetningarreiturinn er frjáls. Stofntími færslunnar er geymdur sér.
help-who-owes-q = Hvernig reiknar Counted út hver skuldar hvað?
help-who-owes-a = Counted reiknar nettóstöðu hvers þátttakanda (það sem hann lagði út mínus það sem hann skuldar) og leggur svo til stystu röð millifærslna sem gerir upp við alla.
help-minimal-transfers-q = Af hverju er fjöldi tillagðra millifærslna í lágmarki?
help-minimal-transfers-a = Reikniritið parar fyrst saman stöður sem jafnast nákvæmlega út og fer svo yfir afganginn frá stærsta kröfuhafa til stærsta skuldara. Niðurstaðan: færri millifærslur til að gera allt upp.
help-import-tricount-q = Hvernig flyt ég inn verkefni úr Tricount?
help-import-tricount-a = Á heimaskjánum ýtirðu á „+“-hnappinn neðst og svo
help-import-tricount-b = Límdu deilitengil Tricount-verkefnisins sem þú vilt flytja inn.
help-encryption-q = Eru gögnin mín dulkóðuð?
help-encryption-a = Já. Counted sameinar tvær tryggingar:
help-encryption-e2ee-term = Enda-í-enda dulkóðun
help-encryption-e2ee-def = - allt á milli þín og þjónsins ferðast dulkóðað.
help-encryption-zero-term = Enginn aðgangur
help-encryption-zero-def = - þú dulkóðar gögnin áður en þú sendir þau og þjónninn geymir aðeins dulritaðan texta. Við höfum enga leið til að lesa hann.
help-encryption-see = Nánar má lesa í
help-forgot-password-q = Hvað gerist ef ég gleymi lykilorðinu mínu?
help-forgot-password-warning = Gögnin þín glatast varanlega.
help-forgot-password-a = Dulkóðunarlykillinn er leiddur af lykilorðinu þínu, svo engin endurstilling er möguleg: enginn - ekki einu sinni við - getur afkóðað verkefnin þín án þess. Geymdu það á öruggum stað, helst í lykilorðastjóra.
help-archive-delete-q = Hvernig set ég verkefni í geymslu eða eyði því?
help-archive-delete-a = Á verkefnaskjánum opnarðu valmyndina og velur
help-archive-delete-b = til að fela það en halda því. Verkefni er eytt fyrir fullt og allt þegar síðasti meðlimurinn yfirgefur það.
help-delete-account-q = Hvernig eyði ég aðganginum mínum?
help-delete-account-a = Opnaðu Stillingar og notaðu „Eyða aðganginum mínum“. Það gerist strax og er ekki hægt að afturkalla.
help-contact = Önnur spurning? Skrifaðu okkur á

# Receipt scanning (mobile only)
expense-scan = Skanna kvittun
scan-in-progress = Les kvittunina…
scan-error-capture = Tókst ekki að taka myndina. Reyndu aftur eða sláðu útgjöldin inn handvirkt.
scan-error-unreadable = Ekkert læsilegt á þessari kvittun. Sláðu útgjöldin inn handvirkt.
scan-check-amount = Athugaðu heildarupphæðina - hún var ekki skýrt prentuð.
scan-take-photo = Taka mynd
scan-choose-photo = Velja mynd
expense-converted-from = Greitt { $amount } { $from } · 1 { $from } = { $rate } { $to }
project-currency = Gjaldmiðill
project-currency-hint = Allar upphæðir eru sýndar í þessum gjaldmiðli. Honum er ekki hægt að breyta síðar.
project-currency-locked = Gjaldmiðillinn er festur þegar verkefnið er stofnað.

update-required-title = Uppfærslu krafist
update-required-body = Þessi útgáfa af Counted er of gömul til að tala við þjóninn. Uppfærðu hana til að halda áfram að nota forritið.
update-required-button = Uppfæra

notifications-label = Tilkynningar
notifications-title = Tilkynningar
notifications-empty = Ekkert nýtt
notifications-friend-request = Vinabeiðni

friends-title = Vinir
friends-anonymous-body = Vinir eru geymdir með aðganginum þínum. Skráðu þig inn til að bæta við fólki og bjóða því í verkefnin þín án þess að deila tengli.
friends-add-title = Bæta við vini
friends-add-hint = Viðkomandi sér beiðnina þína við innskráningu. Hvorugt ykkar fær að vita hvort hitt er með aðgang fyrr en beiðnin hefur verið samþykkt.
friends-add-button = Bæta við
friends-add-from-project = Bæta við sem vini
friends-request-sent = Beiðni send
friends-no-account-key = Skráðu þig inn aftur til að stjórna vinum þínum á þessu tæki.
friends-incoming-title = Beiðnir
friends-accept = Samþykkja
friends-decline = Hafna
friends-list-title = Vinir mínir
friends-list-empty = Engir vinir enn. Bættu við einhverjum með netfangi hér að ofan, eða úr verkefni sem þið deilið.
friends-remove = Fjarlægja
friends-remove-confirm-title = Fjarlægja vin
friends-remove-confirm-message = { $email } verður ekki lengur meðal vina þinna, og þú ekki meðal þeirra. Hvort ykkar sem er getur sent nýja beiðni síðar.
friends-no-key = Ekki tilbúið enn
friends-fingerprint = Öryggiskóði
friends-fingerprint-hint = Tveir vinir sem lesa sama öryggiskóðann hvor fyrir annan vita að enginn situr á milli þeirra - ekki einu sinni þjónninn okkar.
friends-outgoing-title = Sendar
friends-outgoing-hint = Bíður svars. Þú sérð þau meðal vina þinna þegar þau samþykkja.
friends-withdraw = Hætta við
invite-friends-title = Bjóða vinum
invite-friends-hint = Lykill verkefnisins er dulkóðaður fyrir hvern vin á þessu tæki. Þjónninn sér hann aldrei.
invite-friends-empty = Engir vinir til að bjóða enn.
invite-friends-button = Bjóða
invite-sent = { $count ->
    [one] Boð sent
   *[other] { $count } boð send
}
invitation-badge = Boð
invitation-to = Taka þátt í „{ $name }“
invitation-to-unnamed = Taka þátt í verkefni
invitation-unreadable = Ekki er hægt að opna þetta boð á þessu tæki
invitation-from = Frá { $email }
invitation-accept = Taka þátt
invitation-decline = Hafna
