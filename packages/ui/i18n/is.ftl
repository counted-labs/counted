# Íslenska. Heil að undanskildum lagatextum (legal-, terms-, privacy-), sem eru aðeins til á ensku og
# frönsku og falla aftur á en.ftl skilaboð fyrir skilaboð.

### Common

loading = Hleður…
cancel = Hætta við
discard-title = Henda breytingunum?
discard-message = Það sem þú slóst inn glatast.
discard = Henda
confirm = Staðfesta
retry = Reyna aftur
delete = Eyða
back = Til baka
language = Tungumál
theme = Þema
theme-light = Ljóst
theme-dark = Dökkt
theme-system = Kerfi
theme-hint-device = Aðeins vistað á þessu tæki.
theme-hint-system = Fylgir útliti þessa tækis.

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
error-rate-limited = Þjónninn er upptekinn. Reyndu aftur eftir smástund.
error-generic = Eitthvað fór úrskeiðis. Reyndu aftur.

error-invalid-email = Þetta netfang er ekki gilt.
error-invalid-password = Þetta lykilorð er ekki gilt.
error-password-too-short = Lykilorðið verður að vera að minnsta kosti 12 stafir.
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
error-storage-full = Geymslan er full: ekki tókst að vista lykil þessa verkefnis í þessu tæki. Geymdu deilitengilinn.
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
login-maybe-unverified = Varstu að skrá þig? Staðfestu fyrst netfangið með hlekknum sem við sendum þér, eða sendu hann aftur.
login-resend = Senda staðfestingartengil aftur
login-resending = Sendir…
login-resend-sent = Tölvupóstur sendur - athugaðu pósthólfið þitt.

register-submit = Stofna aðgang
register-submitting = Stofnar…
register-have-account = Ertu þegar með aðgang?
register-password-placeholder = Að minnsta kosti 12 stafir
register-password-warning = Skrifaðu lykilorðið þitt niður: við sjáum það aldrei og getum ekki látið þig fá það aftur.
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
account-delete-warning = Tafarlaus og varanleg eyðing aðgangsins þíns og gagnanna þinna.
account-delete-confirm-title = Eyða aðgangi
account-delete-confirm-message = Aðgangi þínum, lotum og verkefnalista verður eytt varanlega. Án lykilorðsins verða dulkóðuð gögn sameiginlegs verkefnis ólæsileg fyrir þig - þetta er ekki hægt að afturkalla.
erase-device-open = Skrá út og hreinsa þetta tæki
erase-device-title = Hreinsa þetta tæki?
erase-device-body = Verkefnum, lyklum þeirra og skyndiminni á þessu tæki verður eytt og síðan verður lotunni lokað. Verkefnin þín verða áfram á aðganginum þínum: skráðu þig inn aftur til að fá þau aftur.
erase-device-checking = Athuga hvort verkefnin þín séu á aðganginum þínum…
erase-device-pending = Breytingar sem bíða sendingar: { $count }. Tengstu netinu aftur og reyndu svo aftur.
erase-device-unreachable = Ekki tókst að athuga hvort verkefnin þín séu á aðganginum þínum. Athugaðu tenginguna og reyndu svo aftur.
erase-device-at-risk = Þessi verkefni eru ekki á aðganginum þínum og munu glatast. Til að halda þeim skaltu hætta við og afrita fyrst tengilinn þeirra:
erase-device-confirm = Hreinsa og skrá út
erase-device-confirm-anyway = Hreinsa samt
erase-device-erasing = Hreinsa…

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
payment-methods-stale = Greiðsluupplýsingum þínum var breytt á öðru tæki. Þær hafa verið endurhlaðnar - reyndu aftur.
payment-methods-key-missing = Skráðu þig inn aftur til að stjórna greiðsluupplýsingum.
settings-payment-methods-share-warning = Deildur greiðslumáti er sýnilegur öllum meðlimum verkefnanna þinna.
payment-method-share = Deila með verkefnunum mínum
payment-method-share-hint = Sýnt við hlið nafnsins þíns þegar einhver skuldar þér.
payment-method-copy = Afrita { $name }
payment-method-copied = Afritað.
payment-method-copy-failed = Tókst ekki að afrita - veldu textann og afritaðu hann handvirkt.

verify-email-checking = Staðfesti netfangið þitt…
verify-email-welcome = Netfang staðfest - velkomin(n) í Counted!
verify-email-back-to-login = Aftur í innskráningu
verify-email-intro = Sláðu inn lykilorðið þitt til að staðfesta að þetta netfang sé þitt.
verify-email-submit = Staðfesta netfangið mitt

### Project status

project-archive = Setja í geymslu
project-sheet-invite = Bjóða
project-sheet-recurring = Endurtekið
project-sheet-edit = Breyta verkefni
project-sheet-close = Loka verkefni
project-sheet-archive = Setja verkefni í geymslu
project-sheet-reopen = Opna verkefni aftur
project-sheet-unarchive = Taka verkefni úr geymslu
project-sheet-leave = Yfirgefa verkefni

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
projects-offline-banner = Gögn án nettengingar - tengstu aftur til að uppfæra.
demo-banner = Sýniverkefni - aðeins til lestrar.
demo-start-own = Stofnaðu eigið verkefni
demo-badge = Prufa
demo-bar = Breytingarnar hverfa þegar þú lokar þessum flipa.
demo-leave = Byrja fyrir alvöru
demo-preparing = Undirbý prufuna þína…
demo-failed = Ekki tókst að undirbúa prufuna.
demo-sign-in-blocked = Innskráning er óvirk í prufunni. Farðu úr henni til að skrá þig inn eða stofna aðgang; þín eigin verkefni eru ósnert.
projects-no-local-data = Engin staðbundin gögn
projects-no-local-data-hint = Skráðu þig inn til að hlaða verkefnunum þínum í fyrsta sinn.
projects-add = Bæta við verkefni
projects-create = Stofna verkefni
projects-personal-expense = Eigin útgjöld
personal-expense-banner = Bætt við Mín útgjöld
personal-create-title = Búðu til þitt eigið verkefni
personal-create-hint = Fylgstu með því sem þú eyðir einn
group-create-title = Stofnaðu sameiginlegt verkefni
group-create-hint = Deildu kostnaði með vinum og fjölskyldu
personal-create-action = Búa til
personal-create-signup = Stofna aðgang
personal-sheet-title = Þitt eigið verkefni
personal-sheet-intro = Til að skrá það sem þú eyðir einn. Það birtist með hópunum þínum í Tölfræði.
personal-sheet-point-alone = Einn meðlimur: þú, með nafni aðgangsins. Enginn deilitengill.
personal-sheet-point-encrypted = Dulkóðað á tækjunum þínum, eins og önnur verkefni.
personal-sheet-point-stats = Hlutir þínir í hópum bætast við þessi útgjöld í Tölfræði.
personal-default-name = Mín útgjöld
my-spending-prev-month = Fyrri mánuður
my-spending-next-month = Næsti mánuður
my-spending-spent = Þú eyddir
my-spending-groups = Hópar
my-spending-income = Tekjur
my-spending-balance = Staða mánaðarins
my-spending-unconverted = Hlutir í öðrum gjaldmiðli eru utan við þar til gengi er þekkt.
my-spending-empty = Ekkert þennan mánuð
my-spending-empty-hint = Bættu við útgjöldum með +. Hlutir þínir í hópum birtast hér sjálfkrafa.
my-spending-share-of = af { $amount }
my-spending-uncounted = ekki talið
payer-you = þér
expense-received-by = móttekið af
counted-switch = Telja með í Mín útgjöld
counted-switch-on = Kveikt: hlutur þinn bætist við Mín útgjöld
counted-switch-off = Slökkt: hlutur þinn birtist ekki í Mín útgjöld
counted-groups = Taldir hópar
counted-groups-intro = Hlutur þinn í þessum hópum bætist við Mín útgjöld og fjárhagsáætlanirnar.
counted-groups-converted = { $from }, umreiknað í { $to }
counted-groups-encrypted = Þetta val er dulkóðað með reikningnum þínum og fylgir tækjunum þínum.
counted-groups-empty = Enginn hópur á þessu tæki enn.
counted-groups-done = Í lagi
counted-save-failed = Ekki tókst að vista þetta val. Reyndu aftur.
tab-budget = Fjárhagsáætlun
budget-title = Fjárhagsáætlanir
budget-manage = Stjórna
budget-left = { $amount } eftir
budget-over = Farið yfir um { $amount }
budget-empty = Engin fjárhagsáætlun
budget-empty-hint = Settu þak á hvern flokk. Hlutir þínir í hópum telja líka.
budget-define = Setja fjárhagsáætlun
budget-sheet-title = Mánaðarlegar áætlanir
budget-sheet-intro = Á þinn hlut: persónuleg og hópar. Skildu eftir autt til að fylgjast ekki með flokki.
budget-average = Meðaltal síðustu 3 mánaða: { $amount }
budget-for = Áætlun { $category }
budget-alert = Láttu mig vita við 80 %
budget-alert-hint = Reiknað á þessu tæki
budget-invalid = Ógild upphæð fyrir { $category }.
budget-offline = Áætlanir vistast aðeins á netinu. Tengstu aftur og reyndu aftur.
budget-alert-near = { $category }: { $percent } % af áætlun mánaðarins
budget-alert-over = { $category }: farið yfir áætlun mánaðarins
personal-card-budget = { $month } · fjárhagsáætlun { $amount }
my-spending-search-summary =
    { $count ->
        [one] { $count } niðurstaða
       *[other] { $count } niðurstöður
    } · { $total } · persónuleg og hópar, allir mánuðir
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
add-project-offline = Ekki er hægt að stofna verkefni án nettengingar. Tengstu aftur og reyndu á ný.
add-project-name-required = Verkefnið þarf heiti.

join-link-label = Deilitengill
join-link-hint = Tengillinn inniheldur afkóðunarlykilinn - afritaðu hann allan.
join-invalid-link = Þessi tengill er ekki gildur. Límdu allan deilitengilinn, þar með talið hlutann á eftir #.
join-wrong-project = Þessi tengill er fyrir annað verkefni.

import-tricount-link-label = Tricount-tengill eða -lykill
import-tricount-key-required = Sláðu inn Tricount-tengil eða -lykil.
import-tricount-encryption-failed = Dulkóðun mistókst.
import-tricount-unimportable = Ekkert var flutt inn: þessi Tricount er með meðlimi með Tricount-aðgang eða upphæðir sem ganga ekki upp (færslur sem um ræðir: { $count }).

### Expenses

save = Vista
saving = Vistar…
adding = Bætir við…
link-copied = Tengill afritaður
missing-encryption-key = Dulkóðunarlykil vantar.
missing-encryption-key-title = Dulkóðunarlykil vantar
missing-encryption-key-hint = Tengillinn sem þú notaðir inniheldur ekki lykilinn sem þarf til að afkóða þetta verkefni. Notaðu allan tengilinn sem sá sem stofnaði það deildi.
project-locked-hint = Þetta tæki hefur ekki lykil þessa verkefnis. Opnaðu deilitengil þess til að aflæsa því.
expenses-unreadable = Ekki er hægt að lesa sumar færslur í þessu tæki og þær eru ekki taldar með í samtölum. Uppfærðu forritið til að sjá þær.
project-unlock = Aflæsa
project-no-local-data-hint = Skráðu þig inn til að hlaða gögnum þessa verkefnis í fyrsta sinn.
project-gone-title = Þetta verkefni er ekki lengur til
project-gone-hint = Því var eytt þegar síðasti meðlimurinn yfirgaf það. Deilitengillinn virkar ekki lengur, jafnvel þótt þú opnir hann aftur.

expense-add = Bæta við útgjöldum
transfer-add = Bæta við millifærslu
expense-edit-title = Breyta útgjöldum
expense-category = Flokkur
expense-category-auto = Sjálfvirkt · { $emoji }
emoji-picker-label = Tjákn
emoji-none = Ekkert tjákn
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
expense-type-group = Tegund
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
balance-gets-back = Fær til baka
balance-owes = Skuldar
balance-settled = Uppgert
reimbursements-empty-title = Allt gert upp!
reimbursements-empty-hint = Uppgjörstillögur birtast hér þegar reikningar stemma ekki
reimbursement-owes = { $debtor } skuldar { $creditor }
reimbursement-record = Gera upp
settlement-recorded = Uppgjör skráð.
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
edit-project-offline-deferred = Án nettengingar: { $items } tekur gildi þegar þú tengist aftur.

export-failed = Útflutningur mistókst: { $reason }

history-expense-added = Útgjöldum bætt við: { $name }
history-expense-edited = Útgjöldum breytt: { $name }
history-expense-deleted = Útgjöldum eytt: { $name }
move-expense = Færa í verkefni
move-expense-title = Færa í…
move-current = Núverandi
move-converted = { $currency }, umreiknað á gengi mánaðarins
move-note-personal = Þú velur í næsta skrefi hver greiddi og fyrir hvern. Útgjöldin fara úr Mín útgjöld.
move-note-group = Útgjöldin fara úr { $project } hjá öllum meðlimum og staða hópsins verður reiknuð aftur. Þú velur í næsta skrefi hver greiddi og fyrir hvern.
move-banner = Frá { $from } í { $to }
move-submit = Færa
move-done = Útgjöld færð í { $project }
history-expense-moved = Útgjöld færð í annað verkefni: { $name }
personal-badge = Eigið
move-members =
    { $count ->
        [one] { $count } meðlimur
       *[other] { $count } meðlimir
    }
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
expense-share-percent = { $percent }%
expense-added-by = Bætt við af { $name }, { $date }
expense-inconsistent-detail = Upphæðir stemma ekki: { $paid } greitt, { $owed } skuldað, fyrir útgjöld upp á { $total }. Breyttu útgjöldunum til að laga það.
missing-access-key = Aðgangslykil vantar. Opnaðu þetta verkefni í gegnum deilitengil þess.
filter-all = Allt
filter-my-payments = Mínar greiðslur
filter-my-debts = Það sem ég skulda
expense-filter-group = Sía
expenses-search = Leita í útgjöldum
expenses-no-match = Engin útgjöld passa.
expenses-search-open = Leita
expenses-search-clear = Hreinsa leit
expenses-search-summary =
    { $count ->
        [one] { $count } niðurstaða
       *[other] { $count } niðurstöður
    } · { $total } · allir mánuðir
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
charts-tab-trends = Þróun
charts-total-spent = Samtals eytt
charts-avg-per-person = Meðaltal á mann
charts-expense-count =
    { $count ->
        [one] { $count } útgjaldaliður
       *[other] { $count } útgjaldaliðir
    }
charts-nothing-to-show = Ekkert að sýna
charts-my-share-skipped =
    { $count ->
        [one] 1 verkefni er ekki talið með - enginn þátttakandi valinn eða gögn þess hlóðust ekki.
       *[other] { $count } verkefni eru ekki talin með - enginn þátttakandi valinn eða gögn þeirra hlóðust ekki.
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
categories-manage = Sýsla með flokka
categories-title = Flokkar
categories-yours = Þínir flokkar
categories-builtin = Sjálfgefnir
categories-add = Bæta við flokki
categories-in-parent = í flokknum { $parent }
categories-renamed-from = endurnefndur · upphaflega { $name }
categories-hidden-hint = falinn · útgjöldin fara í flokkinn { $other }
categories-always-shown = alltaf sýndur
categories-hidden = Falinn
categories-new = Nýr flokkur
categories-edit = Breyta flokki
categories-parent = Talinn með í
categories-parent-hint = Í tölfræði allra verkefna þinna telst { $name } með í flokknum { $parent }.
categories-create = Stofna
categories-hide = Fela þennan flokk
categories-hide-hint = Útgjöld hans birtast öllum meðlimum í flokknum { $other }.
categories-delete-title = Eyða „{ $name }“?
categories-delete-body = Útgjöldin sem um ræðir ({ $count }) fara aftur í flokkinn { $parent }. Engum útgjöldum er eytt.
categories-delete-body-unknown = Útgjöld hans fara aftur í flokkinn { $parent }. Engum útgjöldum er eytt.
categories-delete-warning = Flokkurinn hverfur hjá öllum meðlimum verkefnisins.
categories-summary-created = Stofnaðir: { $count }
categories-summary-renamed = Endurnefndir: { $count }
categories-summary-hidden = Faldir: { $count }
categories-summary-default = Engir eigin flokkar
categories-offline = Aðeins er hægt að sýsla með flokka á netinu. Tengstu aftur og reyndu aftur.
categories-error-too-many = Verkefni getur haft í mesta lagi 20 eigin flokka.
categories-error-full = Þetta verkefni hefur náð hámarksfjölda flokka.
categories-error-name = Gefðu flokknum heiti.
categories-error-missing = Þessum flokki var breytt á meðan. Lokaðu og reyndu aftur.
history-category-created = Flokkur stofnaður: { $name }
history-category-edited = Flokki breytt: { $name }
history-category-deleted = Flokki eytt: { $name }
charts-project = Verkefni
charts-all-projects = Öll verkefni
charts-date-from = Frá
charts-date-to = Til
charts-total = Samtals
charts-tab-people = Fólk
charts-tab-projects = Verkefni
charts-scope = Útgjöld hvers
charts-scope-group = Hópur
charts-scope-me = Ég
charts-currency = Gjaldmiðill
charts-my-share = Minn hluti
charts-share-of-total = { $pct }% af { $total }
charts-i-paid = Ég greiddi
charts-paid-more = { $amount } meira en minn hluti
charts-paid-less = { $amount } minna en minn hluti
charts-paid-even = Nákvæmlega minn hluti
charts-part-title = Þinn hluti í hverjum flokki
charts-part-desc = Grátt er það sem hópurinn eyddi, litur það sem þú notaðir.
charts-breakdown-title = Sundurliðun eftir flokkum
charts-breakdown-desc = Pikkaðu á sneið eða línu til að sjá útgjöldin.
charts-of-total = { $amount } af { $total }
charts-show-all = Sýna allt ({ $count })
charts-show-less = Sýna færri
charts-spend-title = Útgjöld yfir tíma
charts-group-by = Flokka eftir
bucket-day = Dagur
bucket-week = Vika
bucket-month = Mánuður
charts-avg = meðalt.
charts-cat-title-day = { $category }, dag frá degi
charts-cat-title-week = { $category }, viku frá viku
charts-cat-title-month = { $category }, mánuð frá mánuði
charts-cat-desc = Veldu flokk til að fylgjast með honum yfir tíma.
charts-running-title = Uppsöfnuð summa
charts-running-desc = Frá { $date }.
charts-avg-per-day = { $amount } / dag að meðaltali
charts-avg-per-week = { $amount } / viku að meðaltali
charts-avg-per-month = { $amount } / mánuð að meðaltali
charts-people-title = Hver bar hópinn
charts-people-desc = Hvað hver greiddi, við hlið þess sem hann notaði.
charts-paid = Greitt
charts-fair-share = Sanngjarn hluti
charts-you = (þú)
charts-net-more = greiddi meira
charts-net-less = greiddi minna
charts-balance-title = Staðan þín yfir tíma
charts-balance-desc = Fyrir ofan línuna skuldar hópurinn þér. Fyrir neðan skuldar þú hópnum.
charts-owed = Þú átt inni
charts-owe = Þú skuldar
charts-projects-title = Þinn hluti eftir verkefni
charts-projects-desc = Summur eru haldnar eftir gjaldmiðli.
history-empty = Engir atburðir
history-by = Af { $name }
not-found-hint = Þessi síða er ekki til eða hefur verið færð.
payers-title-paid-by = Greitt af
payers-title-sender = Sendandi
payers-title-contributors = Þátttakendur
debtors-title-debtors = Skuldar
debtors-title-recipients = Viðtakendur
debtors-title-beneficiaries = Njótendur
debtors-title-for = Fyrir
debtors-title-recipient = Viðtakandi
expense-split-between = skipt á milli { $count }
expense-people =
    { $count ->
        [one] { $count } manneskja
       *[other] { $count } manneskjur
    }

### Welcome

welcome-title = Bókhaldið þitt kemur engum öðrum við.
welcome-subtitle = Skiptu útgjöldum með vinum.
welcome-note = Ókeypis. Enginn aðgangur nauðsynlegur. Engar auglýsingar.
welcome-link-title = Einn hlekkur og allir taka þátt.
welcome-link-body = Enginn þarf að stofna aðgang.
welcome-link-account = Aðgangur? Aldrei skylda. Hann nýtist til að finna verkefnin þín í öðru tæki, bjóða vinum úr appinu og deila greiðsluupplýsingunum þínum.
welcome-demo-project = Helgi í Lyon
welcome-private-title = Enginn getur lesið reikningana þína. Ekki einu sinni við.
welcome-private-body = Nöfn, upphæðir, verkefni: allt er dulkóðað á tækinu þínu. Þú ein(n) hefur lykilinn.
welcome-private-names = Nöfn
welcome-private-amounts = Upphæðir
welcome-private-projects = Verkefni
welcome-scan-title = Taktu mynd af kvittuninni.
welcome-scan-body = Upphæð, dagsetning og flokkur fyllast út sjálfkrafa. Allt gerist í símanum þínum. Myndin er ekki geymd.
welcome-eu-title = 100% evrópskt
welcome-no-ads = Engar auglýsingar
welcome-no-trackers = Engir rekjarar
welcome-step = Skref { $current } af { $total }
welcome-next = Áfram
welcome-skip = Sleppa
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
help-forgot-password-warning = Enginn getur endurheimt það fyrir þig.
help-forgot-password-a = Lykilorðið þitt verndar dulkóðunarlyklana þína og er aldrei sent til okkar: enginn - ekki einu sinni við - getur afkóðað verkefnin þín fyrir þig. Geymdu það á öruggum stað, helst í lykilorðastjóra.
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
project-currency-locked = Gjaldmiðillinn er festur þegar verkefnið er stofnað.
currency-search = Leita að gjaldmiðli

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
friends-add-hint = Ef viðkomandi er með aðgang sér hann beiðnina þína í appinu. Ef ekki, láttu vita: hann finnur hana þegar hann skráir sig með þessu netfangi.
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
friends-key-changed = Kóði breyttist
friends-key-changed-hint = Öryggiskóði vinar breyttist. Það gerist þegar hann tryggir reikninginn sinn, eða ef skipt var um lykil hans. Boð til og frá honum eru læst þar til þið lesið nýja kóðann hvort fyrir annað, í eigin persónu eða í síma, og þú ýtir á „Kóðinn passar“.
friends-fingerprint = Öryggiskóði
friends-fingerprint-hint = Tveir vinir sem lesa sama öryggiskóðann hvor fyrir annan vita að enginn situr á milli þeirra - ekki einu sinni þjónninn okkar.
friends-key-mismatch = Þjónninn sendi opinberan lykil fyrir aðganginn þinn sem passar ekki við einkalykilinn þinn. Öryggiskóðinn þinn er falinn og boð eru óvirk í þessu tæki: einhver gæti verið að eiga við þjóninn.
keypair-mismatch = Lyklarnir sem þjónninn sendi fyrir reikninginn þinn passa ekki við þá sem eru á þessu tæki. Boð og lyklaskipti eru slökkt hér þar til þeir passa: einhver gæti verið að eiga við þjóninn.
friends-outgoing-title = Sendar
friends-outgoing-hint = Bíður svars. Þú sérð þau meðal vina þinna þegar þau samþykkja.
friends-withdraw = Hætta við
friends-notify = Láta vita með skilaboðum
friends-notify-label = Láta { $email } vita
friends-notify-message = Ég er að skipta yfir í Counted fyrir sameiginleg útgjöld okkar. Ókeypis skráning á https://counted.fr með netfanginu { $email }: vinabeiðnin mín bíður þar.
friends-notify-copied = Skilaboð afrituð
friends-notify-copy-failed = Ekki tókst að afrita skilaboðin
guests-hint-title =
    { $count ->
        [one] { $count } þátttakandi hefur ekki enn gengið í verkefnið
       *[other] { $count } þátttakendur hafa ekki enn gengið í verkefnið
    }
guests-hint-later = Seinna
guests-hint-send = Bjóddu þeim
guests-send-title = Senda verkefnið til þátttakenda
guests-send-friend = Bjóða Counted-vini
guests-send-friend-badge = Án tengils
guests-send-friend-hint = Ef viðkomandi er þegar með aðgang fær hann dulkóðað boð; ekkert fer í gegnum skilaboðaappið þitt.
guests-send-link = Deila tengli verkefnisins
guests-send-link-hint = Fyrir þá sem eru ekki með aðgang. Þeir opna verkefnið með einni snertingu.
guests-send-link-warning = Hver sem er með þennan tengil getur lesið og breytt verkefninu. Skilaboðaappið þitt sér hann líka.
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

# Participants in the create and edit modals, and the "who are you?" picker - see
# docs/plans/friends.md §11.
participants-you-label = Nafnið þitt í þessu verkefni
participants-you-badge = Þú
participants-others = Aðrir þátttakendur
participants-removed = Fjarlægðir
participant-restore = Endurheimta
participants-empty = Bættu við þátttakendum hér fyrir neðan.
participants-duplicate = „{ $name }“ er þegar á listanum.
participants-input-label = Bæta við vini eða skrifa nafn
participants-input-placeholder = Vinur eða hvaða nafn sem er
participants-suggest-friend = Vinur · tekur þátt sem „{ $name }“, fær boð
participants-suggest-not-ready = Vinur · ekki tilbúinn enn
participants-suggest-guest = Bæta „{ $text }“ við
participants-suggest-guest-sub = Gestur
participants-friends = Vinir þínir
participants-all-friends = Allir vinir
participants-login-hint = Skráðu þig inn til að bæta fólki beint við af vinalistanum þínum.
participants-invite-badge = Bjóða
participants-guest-badge = Gestur
participants-rename = Endurnefna { $name }
participants-remove = Fjarlægja { $name }
participants-rename-label = Nýtt nafn
participants-rename-save = Vista nafnið
participants-rename-hint = Nafnið sem allir sjá í þessu verkefni. Boðið fer samt til { $email }.
participants-invited-badge = Boðið
participants-invited-sub = { $email } · ekki samþykkt enn
participants-invited-pending = Boð ekki samþykkt enn
participants-unlinked = Ekki tengt aðgangi
add-project-create-invite = Stofna og bjóða { $count }
edit-project-save-invite = Vista og bjóða { $count }
edit-project-you-are = Í þessu tæki ert þú { $name }
edit-project-no-identity = Þú hefur ekki enn valið hver þú ert
edit-project-switch = Skipta
edit-project-choose = Velja
invite-failed = Ekki tókst að senda þessi boð: { $emails }
invite-waiting = Þessir vinir þurfa að opna Counted einu sinni áður en hægt er að bjóða þeim: { $emails }
key-offer-title = Annar lykill fyrir { $name }
key-offer-body = Þetta tæki hefur annan lykil að þessu verkefni, sem hefur aldrei opnað það hér.
key-offer-choices = Nota þennan lykil: verkefnið opnast með lyklinum úr þessum tengli eða boði. Hætta við: tækið heldur núverandi lykli sínum og ekkert breytist.
key-offer-use = Nota þennan lykil
invite-again = Bjóða aftur
friend-picker-title = Bæta við vinum
user-selection-invited-hint = { $email } bauð þér í „{ $project }“.
user-selection-suggested = Tillaga
user-selection-suggested-sub = { $email } bætti þér við undir þessu nafni
user-selection-confirm-as = Ég er { $name }
user-selection-missing = Er nafnið þitt ekki hér? Biddu þátttakanda að bæta þér við í stillingum verkefnisins.

## Endurteknir kostnaðarliðir

repeat-label = Endurtaka
repeat-none = Endurtekst ekki
repeat-weekly = Vikulega
repeat-biweekly = Aðra hverja viku
repeat-monthly = Mánaðarlega
repeat-quarterly = Á 3 mánaða fresti
repeat-yearly = Árlega
repeat-every-weeks = Á { $count } vikna fresti
repeat-every-months = Á { $count } mánaða fresti
repeat-every-years = Á { $count } ára fresti
repeat-custom = Sérsniðið…
repeat-every = Á fresti:
repeat-unit-weeks = vikur
repeat-unit-months = mánuðir
repeat-unit-years = ár
repeat-on-weekday = á { $weekday }
repeat-on-day = þann { $day }.
repeat-on-day-month = þann { $day }. { $month }
repeat-month-end = Í styttri mánuðum lendir hann á síðasta degi.
repeat-ends = Lýkur
repeat-ends-never = Aldrei
repeat-ends-on = Á dagsetningu
repeat-ends-after = Eftir
repeat-fewer = Færri
repeat-more = Fleiri
repeat-last-on = síðast { $date }
repeat-variable = Upphæðin breytist í hvert sinn
repeat-variable-hint = Hver og einn bætist við með síðustu upphæð og er merktur „á eftir að staðfesta“.
repeat-done = Lokið
repeat-no-end = Engin lok
repeat-until = Til { $date }
repeat-occurrences = Fjöldi skipta: { $count }
repeat-offline = Krefst tengingar. Samt er hægt að bæta kostnaðinum sjálfum við.
repeat-foreign = Endurtekst sem { $amount } { $currency }, umreiknað einu sinni á gengi dagsins. Þú færð viðvörun ef gengið breytist um meira en 5 %.
repeat-backfill = Byrjar í fortíðinni. Kostnaður bættur við núna: { $count }.
add-and-repeat = Bæta við og endurtaka
weekday-1 = mánudegi
weekday-2 = þriðjudegi
weekday-3 = miðvikudegi
weekday-4 = fimmtudegi
weekday-5 = föstudegi
weekday-6 = laugardegi
weekday-7 = sunnudegi
recurring-title = Endurtekinn kostnaður
recurring-strip = Endurtekinn kostnaður: { $count }
recurring-next = Næst: { $name }, { $date }
recurring-to-confirm = Á eftir að staðfesta: { $count }
recurring-per-month = Á mánuði, u.þ.b.
recurring-your-share = Þinn hluti
recurring-active = Virkir
recurring-paused = Í hléi
recurring-finished = Lokið
recurring-paid-by = greitt af { $name }
recurring-next-on = Næst { $date }
recurring-progress = { $done } af { $total }
recurring-rate-badge = Gengi breytt um { $percent } %
recurring-empty = Ekkert endurtekst enn. Veldu „Endurtaka“ þegar þú bætir við kostnaði: leiga, áskriftir, reikningar.
recurring-next-ones = Næstu
recurring-added-so-far = Bætt við hingað til
recurring-set-up-by = Stofnað af
recurring-pause = Gera hlé
recurring-resume = Halda áfram
recurring-stop = Hætta að endurtaka
recurring-stop-title = Stöðva „{ $name }“?
recurring-stop-message = Hann endurtekst ekki lengur. Kostnaður sem þegar hefur verið bætt við helst.
recurring-resume-title = Halda „{ $name }“ áfram?
recurring-resume-message = Næst { $date }. Dagsetningum sem féllu niður í hléinu er ekki bætt við.
recurring-edit-title = Breyta endurteknum kostnaði
recurring-edit-banner = Breytingar gilda frá { $date }. Kostnaður sem þegar hefur verið bætt við helst óbreyttur.
recurring-next-on-label = Næst
recurring-next-too-early = Næsta dagsetning verður að vera á eftir síðasta kostnaði sem bætt var við.
recurring-use-stop = Notaðu „Hætta að endurtaka“ á endurtekna kostnaðinum til að ljúka honum.
recurring-drift = Gengi { $currency } hefur breyst um { $percent } % frá stofnun. Hver og einn bætist enn við sem { $amount } { $project_currency } (1 { $currency } = { $rate }). Á gengi dagsins væri það { $today_amount } { $project_currency }.
recurring-use-rate = Nota gengi dagsins
recurring-keep = Halda { $amount } { $currency }
recurring-added = Endurtekinn kostnaður bættur við: { $count }
recurring-blocks-removal = Ekki er enn hægt að fjarlægja { $name }: hluti af { $rules }. Taktu { $name } út úr þeim eða stöðvaðu þá og vistaðu aftur.
history-recurring-added = Bætt við sjálfkrafa: { $name } ({ $date })
history-recurring-created = Endurtekin útgjöld stofnuð: { $name }
history-recurring-edited = Endurteknum útgjöldum breytt: { $name }
history-recurring-paused = Endurtekin útgjöld sett í hlé: { $name }
history-recurring-resumed = Endurtekin útgjöld hafin á ný: { $name }
history-recurring-stopped = Endurtekin útgjöld stöðvuð: { $name }
occurrence-recurring = Endurtekinn kostnaður
occurrence-auto = Bætt við sjálfkrafa af endurteknum kostnaði.
occurrence-auto-next = Bætt við sjálfkrafa af endurteknum kostnaði. Næst { $date }.
occurrence-auto-stopped = Bætt við sjálfkrafa af endurteknum kostnaði sem hefur síðan verið stöðvaður.
occurrence-manage = Stjórna
estimate-badge = Á eftir að staðfesta
expense-pending = Í bið
estimate-title = Upphæð á eftir að staðfesta.
estimate-body = Bætt við með fyrri upphæð. Sláðu inn rétta upphæð þegar hún liggur fyrir.
estimate-confirm = Staðfesta upphæð
apply-to = Gildir um
apply-this-only = Aðeins þennan kostnað
apply-and-next = Þennan og þá næstu
apply-and-next-hint = Endurtekni kostnaðurinn breytist frá { $date }
apply-rule-failed = Kostnaðurinn var vistaður, en endurtekna kostnaðinum var ekki breytt.
occurrence-delete-message = „{ $name }“ frá { $date } verður eytt varanlega. Hann heldur áfram að endurtakast og þessi dagsetning kemur ekki aftur.
occurrence-delete-one = Eyða aðeins þessum
occurrence-delete-stop = Eyða og hætta að endurtaka
error-recurring-clock = Klukka þessa tækis er of fljót. Athugaðu dagsetningu og tíma.
error-recurring-not-found = Þessi endurtekni kostnaður er ekki lengur til.
error-recurring-stale = Einhver breytti þessum endurtekna kostnaði á meðan. Hann var endurhlaðinn: athugaðu hann og vistaðu aftur.
error-too-many-recurring = Verkefnið hefur þegar 50 endurtekna kostnaðarliði. Stöðvaðu einn sem þú þarft ekki lengur til að bæta við nýjum.
error-user-in-recurring = Þátttakandinn er hluti af endurteknum kostnaði. Fjarlægðu þátttakandann þaðan eða stöðvaðu kostnaðinn fyrst.

home-owed-to-you = Þú átt inni
home-you-owe = Þú skuldar
home-details-show = Sýna nánar
home-details-hide = Fela nánar
home-all-settled = Allt er uppgert
home-pay = Borga
home-remind = Minna á
home-remind-message = Smá áminning: enn eru endurgreiðslur ógerðar í { $project }.
home-row-owed = þú átt inni
home-row-owe = þú skuldar
home-row-settled = Uppgert
home-row-quits = Þitt er uppgert
home-row-pending =
    { $count ->
        [one] { $count } endurgreiðsla í bið
       *[other] { $count } endurgreiðslur í bið
    }
home-totals-partial = Án verkefna án gengis
error-account-key-changed = Lykill reikningsins þíns breyttist í öðru tæki. Skráðu þig inn aftur til að halda áfram.
settings-security = Öryggi
settings-change-password = Breyta lykilorði
settings-change-password-hint = Verkefnin þín og gögnin haldast óbreytt. Önnur tæki þín eru skráð út.
change-password-title = Breyttu lykilorðinu þínu
change-password-current = Núverandi lykilorð
change-password-new = Nýtt lykilorð
change-password-confirm = Staðfestu nýja lykilorðið
change-password-mismatch = Nýju lykilorðin tvö passa ekki saman.
change-password-submit = Breyta lykilorði
change-password-submitting = Breyti…
change-password-done = Lykilorði breytt. Önnur tæki þín voru skráð út.
error-rotation-incomplete = Reikningurinn þinn breyttist á meðan. Reyndu aftur.
error-key-lifecycle-unavailable = Ekki í boði enn: fyrst þurfa allir nýjustu útgáfu appsins.
settings-compromised = Einhver gæti vitað lykilorðið mitt
settings-compromised-hint = Nýtt lykilorð, nýir lyklar og öll önnur tæki skráð út.
settings-compromised-open = Tryggja reikninginn minn
compromised-title = Tryggðu reikninginn þinn
compromised-body = Lykilorðinu þínu og lyklum reikningsins er skipt út og öll önnur tæki eru skráð út. Vinir þínir sjá öryggiskóðann þinn breytast: berðu hann aftur saman við þá.
compromised-keeps = Manneskjan sem hafði lykilorðið þitt heldur því sem þegar hefur verið opnað: verkefnunum sem hægt var að lesa hingað til.
compromised-submit = Skipta um lykilorð og lykla
compromised-done = Reikningurinn þinn er tryggður. Biddu vini þína að bera saman nýja öryggiskóðann þinn.
friends-new-fingerprint = Nýr kóði:
friends-code-matches = Kóðinn passar
friends-code-trusted = Kóði staðfestur.
settings-recovery-key = Endurheimtarlykill
settings-recovery-key-none = Án hans kostar gleymt lykilorð stillingarnar þínar, greiðsluupplýsingar og kóða vina, og það tekur 3 daga.
settings-recovery-key-set = Stofnaður. Ef þú gleymir lykilorðinu færðu reikninginn aftur með öllu.
settings-recovery-key-pending = Nýr lykill kemur í staðinn { $when }. Varstu þetta ekki þú? Hættu við með núverandi lykli.
recovery-key-create = Búa til
recovery-key-replace = Skipta út
recovery-key-cancel-replacement = Hætta við nýja lykilinn
recovery-key-title = Endurheimtarlykillinn þinn
recovery-key-body = Skrifaðu hann niður eða vistaðu þar sem aðeins þú nærð til hans. Counted getur hvorki sýnt hann aftur né sótt hann fyrir þig.
recovery-key-copy = Afrita
recovery-key-copied = Afritað
recovery-key-save = Vista sem skrá
recovery-key-print = Prenta
recovery-key-next = Áfram
recovery-key-confirm-body = Til að athuga afritið skaltu slá inn hóp { $n } úr lyklinum.
recovery-key-confirm-wrong = Hópurinn passar ekki. Skoðaðu afritið aftur.
recovery-key-password-body = Sláðu inn lykilorðið til að vista lykilinn á reikninginn.
recovery-key-submit = Vista lykilinn
recovery-key-done = Endurheimtarlykill vistaður.
recovery-key-replacement-done = Nýr lykill vistaður. Hann kemur í stað núverandi eftir 72 klukkustundir; þangað til virkar núverandi enn.
recovery-key-field = Endurheimtarlykill
recovery-key-file-intro = Endurheimtarlykill Counted
recovery-key-cancel-done = Hætt hefur verið við nýja lykilinn.
compromised-new-key = Vistaðu nýja endurheimtarlykilinn núna: hann kemur í stað þess gamla eftir 72 klukkustundir og þangað til virkar sá gamli.
pending-reset-title = Beðið var um endurstillingu reikningsins þíns
pending-reset-body = Hún tekur gildi { $when }: þá tapast stillingar, greiðsluupplýsingar og kóðar vina, og tækin þín þurfa að skrá sig inn aftur. Varstu þetta ekki þú? Hættu við með núverandi lykilorði.
pending-reset-cancel = Hætta við endurstillingu
pending-reset-cancelled = Hætt hefur verið við endurstillinguna.
login-forgot = Gleymt lykilorð?
forgot-title = Gleymt lykilorð
forgot-body = Sláðu inn netfangið. Ef það er með reikning sendum við tengil til að velja nýtt lykilorð.
forgot-submit = Senda tengilinn
forgot-sent = Ef netfangið er með reikning er tengill á leiðinni. Hann rennur út eftir 24 klukkustundir.
reset-title = Endurheimtu reikninginn þinn
reset-with-key-body = Sláðu inn endurheimtarlykilinn og nýtt lykilorð: allt kemur aftur eins og það var.
reset-submit-key = Endurheimta reikninginn minn
reset-no-key = Ég er ekki með endurheimtarlykilinn
reset-without-key-title = Endurstilla án endurheimtarlykils
reset-without-key-body = Reikningurinn byrjar upp á nýtt með nýju lykilorði eftir 72 klukkustundir. Verkefnin koma aftur úr tækjunum þínum og deilitenglum; stillingar, greiðsluupplýsingar og kóðar vina ekki. Þangað til virkar núverandi lykilorð og getur hætt við endurstillinguna.
reset-name = Nafnið þitt
reset-submit-request = Endurstilla reikninginn minn
reset-requested = Búið. Nýja lykilorðið virkar frá { $when }. Þangað til virkar núverandi lykilorð og getur hætt við.
reset-pending-already = Endurstilling bíður nú þegar til { $when }. Ný beiðni byrjar 72 klukkustundirnar upp á nýtt.
recovered-done = Reikningurinn þinn er kominn aftur. Önnur tæki voru skráð út.
login-reset-undoable = Þessu lykilorði var skipt út með endurstillingu síðustu 30 daga. Ef þú baðst ekki um það skaltu afturkalla það: reikningurinn kemur aftur eins og hann var.
login-undo-reset = Afturkalla endurstillingu
reset-undone = Endurstillingin hefur verið afturkölluð. Reikningurinn er eins og áður.
error-reset-pending = Nýja lykilorðið virkar þegar endurstillingin tekur gildi, 72 klukkustundum eftir beiðnina.
error-account-disabled = Reikningnum var eytt skömmu eftir endurstillingu og bíður þess að verða þurrkaður út. Fyrra lykilorðið getur enn afturkallað endurstillinguna.
error-recovery-key-invalid = Þessi endurheimtarlykill opnar ekki þennan reikning.
error-project-key-required = Verkefnið þarf lykilinn sinn. Opnaðu það aftur úr deilitenglinum.
names-or = { $first } eða { $last }
participants-admin-badge = Stjórnandi
edit-project-admins = Stjórnendur: { $count }
edit-project-succession = Ef síðasti stjórnandinn yfirgefur verkefnið verður sá meðlimur með aðgang sem hefur verið lengst í verkefninu stjórnandi.
member-sheet-account = Meðlimur með aðgang
member-actions = Aðgerðir fyrir { $name }
member-make-admin = Gera að stjórnanda
member-make-admin-hint = Getur þá breytt lykli verkefnisins og fjarlægt einhvern.
member-made-admin = { $name } er nú stjórnandi.
group-security = Öryggi verkefnisins
group-security-admin = Öryggi verkefnisins · þú ert stjórnandi
group-rotate = Breyta lykli verkefnisins
group-rotate-hint = Ef tengill verkefnisins hefur lekið: þau sem eru með gamla tengilinn missa aðgang að verkefninu.
group-remove = Fjarlægja einhvern úr verkefninu
group-remove-hint = Tekur aðganginn af og breytir lyklinum um leið.
group-admins-only = Aðeins fyrir stjórnendur.
group-admins-only-account = Aðeins fyrir stjórnendur, sem eru með aðgang.
group-ask-admin = Spurðu { $names }, stjórnanda verkefnisins.
group-ask-admins = Spurðu { $names }, stjórnendur verkefnisins.
group-ask-an-admin = Spurðu einn af stjórnendum verkefnisins.
group-with-account = Með aðgangi
group-free = Ókeypis
group-sign-in-hint = Stjórnandi verkefnisins, { $names }, getur svo gefið þér hlutverkið.
group-sign-in-hint-any = Stjórnandi verkefnisins getur svo gefið þér hlutverkið.
group-no-admin = Þetta verkefni hefur engan stjórnanda, svo ekki er hægt að breyta lyklinum.
group-hide = Fela einhvern
group-hide-hint = Felur viðkomandi í listunum án þess að taka aðganginn af: tengillinn og lykillinn virka áfram.
group-recreate-hint = Þetta verkefni hefur engan stjórnanda. Til að fjarlægja einhvern skaltu búa til nýtt verkefni.
group-recreate = Búa til nýtt verkefni
group-no-admin-yet = Þetta verkefni hefur engan stjórnanda enn.
group-creator-hint = Aðeins þú getur orðið stjórnandi þessa verkefnis, með því að stofna aðgang.
group-creator-elsewhere-hint = Aðeins manneskjan sem stofnaði verkefnið getur orðið stjórnandi þess, með því að stofna aðgang. Annars skaltu búa til nýtt verkefni til að fjarlægja einhvern.
member-revoke-admin = Taka stjórnandahlutverk af
member-revoke-admin-hint = Getur ekki lengur breytt lykli verkefnisins né fjarlægt neinn.
member-revoked-admin = { $name } er ekki lengur stjórnandi.
member-step-down = Hætta sem stjórnandi
member-step-down-hint = Þú getur ekki lengur breytt lykli verkefnisins né fjarlægt neinn.
member-stepped-down = Þú ert ekki lengur stjórnandi.
member-remove = Fjarlægja úr verkefninu
member-remove-hint = Breytir lykli verkefnisins: { $name } getur ekki lengur lesið neitt nýtt.
member-hide = Fela í listunum
error-user-not-admin = Þessi meðlimur er ekki stjórnandi.
error-creator-proof-invalid = Þetta tæki stofnaði ekki þetta verkefni.
error-project-has-admin = Þetta verkefni hefur nú þegar stjórnanda.
error-not-project-admin = Aðeins stjórnendur verkefnisins geta gert þetta.
error-user-not-claimed = Þessi þátttakandi er ekki með aðgang og getur því ekki verið stjórnandi.
error-project-key-rotated = Lykli verkefnisins var breytt. Biddu meðlim um nýja tengilinn.
error-project-rotating = Verið er að breyta lykli verkefnisins. Reyndu aftur eftir smástund.
error-rotation-expired = Lyklabreytingin tók of langan tíma og var hætt við hana. Byrjaðu aftur.
error-rotation-rows-changed = Verkefnið breyttist meðan lyklinum var breytt. Byrjaðu aftur.
project-rotated-title = Þú hefur ekki lengur aðgang að þessu verkefni
project-rotated-body = Stjórnandi breytti lykli þessa verkefnis. Biddu stjórnandann um nýja tengilinn.
project-rotated-open-link = Opna tengil
rotation-remove-title = Fjarlægja { $name } úr verkefninu?
rotation-rotate-title = Breyta lykli verkefnisins?
rotation-remove-body = Að fjarlægja meðlim þýðir að nýr lykill er búinn til. Það þarf að deila nýja tenglinum með fólki sem er ekki með aðgang.
rotation-rotate-body = Með gamla tenglinum verður ekki lengur hægt að opna verkefnið. Það þarf að deila nýja tenglinum með fólki sem er ekki með aðgang.
rotation-boxed =
    { $count ->
        [0] Enginn annar meðlimur fær lykilinn sjálfkrafa
        [one] { $count } meðlimur fær lykilinn sjálfkrafa
       *[other] { $count } meðlimir fá lykilinn sjálfkrafa
    }
rotation-by-link-title = Fólk sem þú þarft að senda nýja tengilinn
rotation-by-link-hint = Þetta fólk er ekki með aðgang. Þú sendir því tengilinn strax á eftir.
rotation-names-and = { $first } og { $last }
rotation-names-and-others = { $names } og { $count } í viðbót
rotation-confirm-remove = Fjarlægja og breyta lyklinum
rotation-confirm-rotate = Breyta lyklinum
rotation-done-title = Nýr lykill kominn
rotation-done-removed = { $name } er ekki lengur hluti af verkefninu.
rotation-done-rotated = Lykli verkefnisins var breytt.
rotation-done-boxed =
    { $count ->
        [0] Enginn annar meðlimur fékk lykilinn sjálfkrafa.
        [one] { $count } meðlimur fékk lykilinn sjálfkrafa.
       *[other] { $count } meðlimir fengu lykilinn sjálfkrafa.
    }
rotation-new-link =
    { $count ->
        [0] Nýr tengill verkefnisins
       *[other] Nýr tengill til að senda · án aðgangs: { $count }
    }
rotation-link-renewed = Tengillinn var endurnýjaður.
rotation-done = Lokið
history-key-changed = Lykli verkefnisins breytt
history-member-removed = { $name } er ekki lengur hluti af verkefninu
project-rotated-row-hint = Lyklinum var breytt. Biddu um nýja tengilinn.
