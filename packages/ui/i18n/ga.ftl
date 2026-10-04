# Gaeilge. Iomlán seachas na téacsanna dlí (legal-, terms-, privacy-), nach bhfuil ann ach i mBéarla
# agus i bhFraincis agus a thiteann siar ar en.ftl teachtaireacht ar theachtaireacht.

### Common

loading = Á lódáil…
cancel = Cealaigh
confirm = Deimhnigh
retry = Bain triail eile as
delete = Scrios
back = Siar
language = Teanga

### Navigation

nav-main = Príomh-nascleanúint
nav-projects = Tionscadail
nav-charts = Staitisticí
nav-settings = Socruithe

### Connectivity

offline-banner = As líne
offline-pending =
    { $count ->
        [one] { $count } ar feitheamh
        [two] { $count } ar feitheamh
        [few] { $count } ar feitheamh
        [many] { $count } ar feitheamh
       *[other] { $count } ar feitheamh
    }

sync-conflict-edit = Coinbhleacht: theip ar “{ $name }” a chur in eagar (mír scriosta). Léimthe thairis.
sync-conflict-delete = Coinbhleacht: theip ar “{ $name }” a scriosadh (mír scriosta). Léimthe thairis.
sync-conflict-other = Coinbhleacht: theip ar an oibríocht ar “{ $name }” (mír scriosta). Léimthe thairis.
sync-error = Earráid sioncronaithe: { $reason }

### Errors

error-network = Ní féidir an freastalaí a shroicheadh. Seiceáil do nasc idirlín.
error-generic = Chuaigh rud éigin amú. Bain triail eile as.

error-invalid-email = Níl an seoladh ríomhphoist sin bailí.
error-invalid-password = Níl an pasfhocal sin bailí.
error-password-too-short = Caithfidh 8 gcarachtar ar a laghad a bheith i do phasfhocal.
error-client-outdated = Tá an leagan seo den aip as dáta. Nuashonraigh é chun logáil isteach.
error-invalid-link = Níl an nasc seo bailí.
error-batch-too-large = An iomarca míreanna in éineacht.
error-payers-required = Roghnaigh íocóir amháin ar a laghad.
error-debtors-required = Roghnaigh duine amháin ar a laghad a bhfuil fiacha air.
error-duplicate-participant = Tá rannpháirtí le feiceáil faoi dhó ar an taobh céanna.
error-participant-not-in-project = Níl an rannpháirtí sin mar chuid den tionscadal seo.
error-too-many-participants = An iomarca rannpháirtithe do chostas amháin.
error-invalid-credentials = Ríomhphost nó pasfhocal mícheart.
error-unauthenticated = Logáil isteach chun é sin a dhéanamh.
error-email-not-verified = Níl do sheoladh ríomhphoist deimhnithe fós.
error-project-not-found = Níl an tionscadal seo ann a thuilleadh.
error-expense-not-found = Níl an costas seo ann a thuilleadh.
error-storage-full = Tá an stóras lán: níorbh fhéidir eochair an tionscadail seo a shábháil ar an ngléas seo. Coinnigh an nasc comhroinnte.
error-user-not-found = Níl an rannpháirtí seo ann a thuilleadh.
error-tricount-not-found = Níor aimsíodh Tricount, nó chuir a API earráid ar ais.
error-too-many-members = Tá teorainn na mball sroichte ag an tionscadal seo.
error-identity-taken = Tá cuntas eile tar éis an rannpháirtí seo a éileamh cheana.
error-claim-proof-invalid = Níl eochair an tionscadail ag an ngléas seo, mar sin ní féidir leis rannpháirtí a éileamh. Oscail an nasc comhroinnte arís.
error-user-has-payments = Tá costais ag an rannpháirtí seo sa tionscadal agus ní féidir é a bhaint.
error-resend-cooldown = Fan 60 soicind sula n-iarrann tú ríomhphost eile.
error-self-friend-request = Ní féidir leat tú féin a chur leis mar chara.
error-not-a-friend = Ní féidir leat ach daoine ó do liosta cairde a thabhairt cuireadh dóibh.
error-friend-has-no-key = Níor oscail an cara seo an leagan is déanaí den aip fós. Iarr air logáil isteach uair amháin, ansin bain triail eile as.
error-friend-request-not-found = Níl an iarraidh chairdis seo ann a thuilleadh.
error-invitation-not-found = Níl an cuireadh seo ann a thuilleadh.
error-too-many-friend-requests = An iomarca iarratas cairdis faoi láthair. Bain triail eile as amárach.
error-too-many-invitations = An iomarca cuirí ar feitheamh.
error-invalid-kdf-salt = Níl na socruithe criptithe bailí. Nuashonraigh an aip agus bain triail eile as.
error-mixed-project-batch = Níl na rannpháirtithe sin go léir sa tionscadal céanna.
error-invalid-payload = Sheol an leagan seo den aip sonraí nach nglacann an freastalaí leo. Nuashonraigh é agus bain triail eile as.
error-invalid-public-key = Níl d'eochair chriptithe bailí. Nuashonraigh an aip agus bain triail eile as.
error-payment-methods-stale = Athraíodh do shonraí íocaíochta ar ghléas eile. Athlódáil agus bain triail eile as.

### Auth

field-email = Ríomhphost
field-email-placeholder = tusa@sampla.ie
field-password = Pasfhocal
field-name = Ainm
field-name-placeholder = Áine Ní Bhriain

login-title = Logáil isteach
login-submit = Logáil isteach
login-submitting = Ag logáil isteach…
login-password-placeholder = Do phasfhocal
login-no-account = Níl cuntas agat fós?
login-unverified = Níl do sheoladh ríomhphoist deimhnithe fós. Seiceáil do bhosca isteach, nó seol an nasc arís.
login-resend = Seol an nasc deimhnithe arís
login-resending = Á sheoladh…
login-resend-sent = Ríomhphost seolta - seiceáil do bhosca isteach.

register-submit = Cruthaigh cuntas
register-submitting = Á chruthú…
register-have-account = An bhfuil cuntas agat cheana?
register-password-placeholder = 8 gcarachtar ar a laghad
register-password-warning = Scríobh síos do phasfhocal. Má dhéanann tú dearmad air, ní féidir do chuntas a aisghabháil.
register-check-email-title = Seiceáil do ríomhphost
register-email-sent = Ríomhphost seolta
register-email-sent-hint = Cliceáil an nasc i do bhosca isteach chun do chuntas a ghníomhachtú.
register-not-received-prefix = Ní bhfuair tú é? Seiceáil d'fhillteán turscair, nó
register-sign-in-link = logáil isteach
register-not-received-suffix = chun an nasc a sheoladh arís.
register-terms-prefix = Trí chuntas a chruthú glacann tú lenár
register-terms-link = dtéarmaí úsáide
register-terms-and = agus lenár
register-privacy-link = bpolasaí príobháideachais

settings-title = Socruithe
settings-preferences = Sainroghanna
settings-preferences-local = Stóráilte ar an ngléas seo.
settings-preferences-synced = Sioncronaithe le do chuntas, criptithe.
settings-about = Maidir leis
settings-anonymous-title = Níl tú logáilte isteach
settings-upsell-title = Do thionscadail, ar gach gléas
settings-upsell-free = Saor in aisce
settings-upsell-body = Oibríonn Counted gan chuntas. Le ceann saor in aisce, leanann do thionscadail agus do shainroghanna thú chuig do ghuthán, do ríomhaire glúine agus an gréasán - fós criptithe, fós doléite againne.
settings-locked-badge = Cuntas
settings-locked-friends = Cruthaigh cuntas chun cairde a chur leis agus cuireadh a thabhairt dóibh chuig tionscadal ón aip - gan nasc le cur timpeall.
settings-locked-payment-methods = Sábháil do IBAN nó d'aip íocaíochta uair amháin agus comhroinn é leis na tionscadail a roghnaíonn tú. Feiceann an té a bhfuil fiacha ort aige é in aice le d'ainm.
settings-friends-hint = Cuir cairde leis agus tabhair cuireadh dóibh chuig do thionscadail gan nasc a chomhroinnt.

account-member-since = Ball ó
account-logout = Logáil amach
account-logging-out = Ag logáil amach…
account-delete-title = Scrios mo chuntas
account-delete-warning = Scriosadh láithreach agus buan ar do chuntas agus ar do shonraí.
account-delete-confirm-title = Scrios cuntas
account-delete-confirm-message = Scriosfar do chuntas, do sheisiúin agus do liosta tionscadal go buan. Gan do phasfhocal, éiríonn sonraí criptithe tionscadail chomhroinnte doléite duit - ní féidir é seo a chealú.

settings-payment-methods = Sonraí íocaíochta
settings-payment-methods-hint = Conas ba mhaith leat go n-aisíocfaí thú. Criptithe le do chuntas.
payment-method-kind = Modh
payment-method-kind-other = Eile
payment-method-label = Ainm
payment-method-label-placeholder = Príomhchuntas
payment-method-value = Sonraí
payment-method-value-placeholder = IBAN, uimhir ghutháin, ainm úsáideora…
payment-method-add = Cuir leis
payment-method-remove = Bain { $name }
payment-method-empty = Níor chuir tú aon sonraí íocaíochta leis fós.
payment-method-deleted = Modh íocaíochta scriosta.
payment-method-value-required = Líon isteach sonraí gach modh íocaíochta, nó bain é.
payment-method-label-required = Tabhair ainm do do mhodh saincheaptha.
payment-method-too-long = Tá sé sin rófhada - giorraigh é.
payment-method-invalid-characters = Bain aon bhristeacha líne nó carachtair dhofheicthe.
payment-method-limit = Is féidir leat suas le { $max } modh íocaíochta a shábháil.
payment-methods-saved = Sonraí íocaíochta sábháilte.
payment-methods-offline = Caithfidh tú a bheith ar líne chun do shonraí íocaíochta a shábháil.
payment-methods-stale = Athraíodh do shonraí íocaíochta ar ghléas eile. Athlódáladh iad — bain triail eile as.
payment-methods-key-missing = Logáil isteach arís chun do shonraí íocaíochta a bhainistiú.
settings-payment-methods-share-warning = Tá modh comhroinnte le feiceáil ag gach ball de do thionscadail.
payment-method-share = Comhroinn le mo thionscadail
payment-method-share-hint = Léirithe in aice le d'ainm nuair atá airgead ag duine ort.
payment-method-copy = Cóipeáil { $name }
payment-method-copied = Cóipeáilte.
payment-method-copy-failed = Níorbh fhéidir cóipeáil - roghnaigh an téacs agus cóipeáil de láimh é.

verify-email-checking = Do sheoladh ríomhphoist á dheimhniú…
verify-email-welcome = Ríomhphost deimhnithe - fáilte go Counted!
verify-email-back-to-login = Ar ais go logáil isteach

### Project status

project-close = Dún
project-archive = Cartlannaigh
project-reopen = Athoscail
project-unarchive = Bain as an gcartlann
project-sheet-invite = Tabhair cuireadh
project-sheet-recurring = Athfhillteach
project-sheet-edit = Cuir an tionscadal in eagar
project-sheet-close = Dún an tionscadal
project-sheet-archive = Cartlannaigh an tionscadal
project-sheet-reopen = Athoscail an tionscadal
project-sheet-unarchive = Bain an tionscadal as an gcartlann
project-sheet-leave = Fág an tionscadal

### Dates

date-long = { $day } { $month } { $year }

month-1 = Eanáir
month-2 = Feabhra
month-3 = Márta
month-4 = Aibreán
month-5 = Bealtaine
month-6 = Meitheamh
month-7 = Iúil
month-8 = Lúnasa
month-9 = Meán Fómhair
month-10 = Deireadh Fómhair
month-11 = Samhain
month-12 = Nollaig

month-short-1 = Ean
month-short-2 = Feabh
month-short-3 = Márta
month-short-4 = Aib
month-short-5 = Beal
month-short-6 = Meith
month-short-7 = Iúil
month-short-8 = Lún
month-short-9 = MFómh
month-short-10 = DFómh
month-short-11 = Samh
month-short-12 = Noll

### Actions

add = Cuir leis
create = Cruthaigh
creating = Á chruthú…
edit = Cuir in eagar
leave = Fág
close = Dún
paste = Greamaigh
join = Glac páirt
import = Iompórtáil
importing = Á iompórtáil…
field-description = Cur síos
field-date = Dáta
date-today = Inniu
date-yesterday = Inné
field-optional = Roghnach

### Projects

projects-filter-active = Gníomhach
projects-filter-all = Uile
projects-count-label = Tionscadail
projects-empty = Gan tionscadail
projects-empty-hint = Cruthaigh tionscadal leis an gcnaipe thíos
projects-offline-banner = Sonraí as líne - athnasc chun athnuachan.
demo-banner = Tionscadal taispeána - inléite amháin.
demo-start-own = Cruthaigh do thionscadal féin
projects-no-local-data = Gan sonraí áitiúla
projects-no-local-data-hint = Logáil isteach chun do thionscadail a lódáil den chéad uair.
projects-add = Cuir tionscadal leis
projects-create = Cruthaigh tionscadal
projects-join = Glac páirt i dtionscadal
projects-import-tricount = Iompórtáil ó Tricount
project-actions = Gníomhartha tionscadail

status-ongoing = Ar siúl
status-closed = Dúnta
status-archived = Cartlannaithe

nav-help = Cabhair
nav-privacy = Polasaí príobháideachais
nav-terms = Téarmaí úsáide
nav-legal = Fógra dlíthiúil

leave-project-title = An tionscadal a fhágáil?
leave-project-message = Caillfidh tú rochtain ón ngléas seo. Mura bhfuil ball ar bith fágtha, scriostar an tionscadal agus a chostais go léir go buan.

add-project-title = Tionscadal nua
add-project-name-label = Ainm an tionscadail
add-project-name-placeholder = Mo thuras, Comhthithe 2024…
add-project-participants = Rannpháirtithe
add-project-participant-name = Ainm an rannpháirtí
add-project-participant-placeholder = Clark Kent
add-project-offline = Ní féidir leat tionscadal a chruthú as líne. Athnasc agus bain triail eile as.
add-project-name-required = Tá ainm ag teastáil ón tionscadal.

join-link-label = Nasc comhroinnte
join-link-hint = Iompraíonn an nasc an eochair dhíchriptithe - cóipeáil é ar fad.
join-invalid-link = Níl an nasc sin bailí. Greamaigh an nasc comhroinnte iomlán, an chuid i ndiaidh # san áireamh.
join-wrong-project = Is do thionscadal eile an nasc sin.

import-tricount-link-label = Nasc nó eochair Tricount
import-tricount-key-required = Cuir isteach nasc nó eochair Tricount.
import-tricount-encryption-failed = Theip ar an gcriptiú.
import-tricount-unimportable = Níor iompórtáladh aon rud: tá baill le cuntas Tricount sa Tricount seo, nó méideanna nach dtagann le chéile (iontrálacha i gceist: { $count }).

### Expenses

save = Sábháil
saving = Á shábháil…
adding = Á chur leis…
link-copied = Nasc cóipeáilte
missing-encryption-key = Eochair chriptithe ar iarraidh.
missing-encryption-key-title = Eochair chriptithe ar iarraidh
missing-encryption-key-hint = Ní iompraíonn an nasc a d'úsáid tú an eochair atá ag teastáil chun an tionscadal seo a dhíchriptiú. Úsáid an nasc iomlán a chomhroinn an té a chruthaigh é.
project-locked-hint = Níl eochair an tionscadail seo ag an ngléas seo. Oscail a nasc comhroinnte chun é a dhíghlasáil.
project-unlock = Díghlasáil
project-no-local-data-hint = Logáil isteach chun sonraí an tionscadail seo a lódáil den chéad uair.
project-gone-title = Níl an tionscadal seo ann a thuilleadh
project-gone-hint = Scriosadh é nuair a d'fhág an ball deireanach. Ní oibríonn an nasc comhroinnte a thuilleadh, fiú má athosclaíonn tú é.

expense-add = Cuir costas leis
transfer-add = Cuir aistriú leis
expense-edit-title = Cuir an costas in eagar
expense-category = Catagóir
expense-category-auto = Uath · { $emoji }
expense-currency = Airgeadra na suime
amount-op-add = Móide
amount-op-subtract = Lúide
amount-op-multiply = Iolraigh
amount-op-divide = Roinn
amount-op-equals = Cothrom le
amount-op-done = Déanta
expense-rate = Ráta malairte (roghnach)
expense-rate-hint = Fág folamh chun ráta an Choimisiúin Eorpaigh (InforEuro) do { $month } a úsáid: 1 { $from } = { $rate } { $to }.
expense-rate-invalid = Cuir isteach ráta malairte níos mó ná 0.
expense-rate-unavailable = Níl ráta uathoibríoch ar fáil - cuir isteach ceann de láimh.
expense-delete-title = Scrios an costas
expense-delete-message = Scriosfar “{ $name }” go buan. Ní féidir é seo a chealú.
expense-inconsistent-amounts = Ní thagann na suimeanna le chéile
expenses-empty = Gan costais
expenses-empty-hint = Tosaigh trí chostais a chur leis leis an gcnaipe thíos
expenses-show-more = Taispeáin tuilleadh ({ $count } fágtha)

expense-type-expense = Costas
expense-type-transfer = Aistriú
expense-type-gain = Ioncam
expense-paid-by = íoctha ag
expense-sent-by = seolta ag
expense-contributed-by = ranníoctha ag

expense-name-required = Tá ainm ag teastáil.
expense-amount-not-positive = Caithfidh an tsuim a bheith níos mó ná 0.
expense-no-payer = Roghnaigh íocóir amháin ar a laghad.
expense-no-debtor = Roghnaigh duine amháin ar a laghad a bhfuil fiacha air.
expense-invalid-date = Níl an dáta sin bailí.
expense-payers-mismatch = Is é { $sum } iomlán na n-íocóirí, rud nach dtagann le suim an chostais ({ $total }).
expense-debtors-mismatch = Is é { $sum } iomlán na bhféichiúnaithe, rud nach dtagann le suim an chostais ({ $total }).

participants-none = Duine ar bith
participants-everyone = Gach duine ({ $count })
participants-some = { $count } as { $total }
participants-select-all = Roghnaigh uile
participants-by-shares = De réir scaireanna
split-amounts = Méideanna
participants-remaining = { $amount } fágtha
participants-over-by = { $amount } thar
participants-who-paid = Cé a d'íoc?
participants-who-received = Cé a fuair?
participants-who-transfers = Cé atá ag aistriú?
participants-who-receives = Cé a fhaigheann?
participants-for-whom = Cé dó?

stats-total-expenses = Costais iomlána
stats-my-expenses = Mo chostais

tab-expenses = Costais
tab-balance = Iarmhéid
tab-reimbursements = Socraigh suas
balance-gets-back = Faigheann ar ais
balance-owes = I bhfiacha
balance-settled = Socraithe
reimbursements-empty-title = Gach rud socraithe!
reimbursements-empty-hint = Feictear moltaí socraithe anseo nuair nach mbíonn na cuntais cothrom
reimbursement-owes = Tá fiacha ag { $debtor } ar { $creditor }
reimbursement-record = Socraigh
reimbursement-pay-with = Íoc
reimbursement-pay-shared-by = Comhroinnte ag { $name } - seiceáil ainm an fhaighteora a thaispeánann d'aip sula seolann tú.
reimbursement-pay-title = Íoc { $name }
reimbursements-mine-title = Tá fiacha ort
reimbursements-others-title = Aisíocaíochtaí eile
copy = Cóipeáil

user-selection-title = Cén rannpháirtí thú?
user-selection-hint = Roghnaigh d'ainm ón liosta.
user-selection-required = Roghnaigh rannpháirtí, le do thoil.
identity-claimed = Nasctha le cuntas
identity-claimed-by = Cuntas { $name }
identity-taken-repick = D'éiligh cuntas eile an rannpháirtí a bhí in úsáid agat. Roghnaigh ceann eile, le do thoil.
participant-gone-repick = Baineadh an rannpháirtí a bhí in úsáid agat den tionscadal seo. Roghnaigh ceann eile, le do thoil.

edit-project-title = Cuir an tionscadal in eagar
edit-project-new-badge = nua
edit-project-deferred-new-members = baill nua a chur leis
edit-project-deferred-removals = baill a bhaint
edit-project-offline-deferred = As líne: cuirfear { $items } i bhfeidhm nuair a athnascann tú.

export-failed = Theip ar an easpórtáil: { $reason }

history-expense-added = Costas curtha leis: { $name }
history-expense-edited = Costas curtha in eagar: { $name }
history-expense-deleted = Costas scriosta: { $name }
history-project-edited = Tionscadal curtha in eagar: { $name }
history-name-changed = Ainm: “{ $from }” → “{ $to }”
history-description-added = Cur síos curtha leis: “{ $value }”
history-description-removed = Cur síos bainte: “{ $value }”
history-description-changed = Cur síos: “{ $from }” → “{ $to }”

### Sweep

field-amount = Suim
expense-name-placeholder = Bialann, earraí grósaera…
expense-actions = Gníomhartha costais
expense-your-share = Do sciar
expense-your-share-value = Do sciar: { $amount } { $currency }
expense-inconsistent-detail = Ní thagann na suimeanna le chéile: { $paid } íoctha, { $owed } dlite, do chostas { $total }. Cuir an costas in eagar chun é a cheartú.
missing-access-key = Eochair rochtana ar iarraidh. Oscail an tionscadal seo trína nasc comhroinnte.
filter-all = Uile
filter-my-payments = M'íocaíochtaí
filter-my-debts = Mo chuid fiacha
participants-shares-for = Scaireanna do { $name }
participants-amount-for = Suim do { $name }
reimbursement-add = Cuir socrú leis
project-forget = Bain de mo liosta
project-history-title = Stair
history-kind-add = Curtha leis
history-kind-delete = Scriosta
history-kind-edit = Curtha in eagar
export = Easpórtáil
export-json = Easpórtáil JSON
export-csv = Easpórtáil CSV
share-link = Comhroinn
copy-link-failed = Níorbh fhéidir an nasc a chóipeáil
open-in-app = Oscail san aip
not-found-title = Leathanach gan aimsiú
not-found-back = Ar ais chuig tionscadail

### Charts

charts-period = Tréimhse
period-all = Uile
period-month = Mí
period-3months = 3 mhí
period-year = Bliain
period-custom = Saincheaptha
charts-tab-categories = Catagóirí
charts-tab-trends = Treochtaí
charts-total-spent = Iomlán caite
charts-avg-per-person = Meán an duine
charts-expense-count =
    { $count ->
        [one] { $count } chostas
        [two] { $count } chostas
        [few] { $count } chostas
        [many] { $count } gcostas
       *[other] { $count } costas
    }
charts-nothing-to-show = Faic le taispeáint
charts-my-share-skipped =
    { $count ->
        [one] Níl { $count } tionscadal amháin san áireamh — gan rannpháirtí roghnaithe, nó níor lódáladh a shonraí.
        [two] Níl { $count } thionscadal san áireamh — gan rannpháirtí roghnaithe, nó níor lódáladh a sonraí.
        [few] Níl { $count } thionscadal san áireamh — gan rannpháirtí roghnaithe, nó níor lódáladh a sonraí.
        [many] Níl { $count } dtionscadal san áireamh — gan rannpháirtí roghnaithe, nó níor lódáladh a sonraí.
       *[other] Níl { $count } tionscadal san áireamh — gan rannpháirtí roghnaithe, nó níor lódáladh a sonraí.
    }

### Categories

category-food = Bia
category-transport = Iompar
category-accommodation = Lóistín
category-leisure = Fóillíocht
category-shopping = Siopadóireacht
category-services = Seirbhísí
category-parties-gifts = Cóisirí & bronntanais
category-other = Eile
charts-project = Tionscadal
charts-all-projects = Gach tionscadal
charts-date-from = Ó
charts-date-to = Go
charts-total = Iomlán
charts-tab-people = Daoine
charts-tab-projects = Tionscadail
charts-scope = Caiteachas cé
charts-scope-group = Grúpa
charts-scope-me = Mise
charts-currency = Airgeadra
charts-my-share = Mo sciar
charts-share-of-total = { $pct }% de { $total }
charts-i-paid = D’íoc mé
charts-paid-more = { $amount } níos mó ná mo sciar
charts-paid-less = { $amount } níos lú ná mo sciar
charts-paid-even = Mo sciar go díreach
charts-part-title = Do chuid de gach catagóir
charts-part-desc = Liath: a chaith an grúpa. Dath: a d’úsáid tusa.
charts-breakdown-title = Miondealú de réir catagóire
charts-breakdown-desc = Tapáil slis nó sraith chun a costais a fheiceáil.
charts-of-total = { $amount } as { $total }
charts-show-all = Taispeáin gach ceann ({ $count })
charts-show-less = Taispeáin níos lú
charts-spend-title = Caiteachas thar am
charts-group-by = Grúpáil de réir
bucket-day = Lá
bucket-week = Seachtain
bucket-month = Mí
charts-avg = meán
charts-cat-title-day = { $category }, lá i ndiaidh lae
charts-cat-title-week = { $category }, seachtain i ndiaidh seachtaine
charts-cat-title-month = { $category }, mí i ndiaidh míosa
charts-cat-desc = Roghnaigh catagóir chun í a leanúint thar am.
charts-running-title = Iomlán reatha
charts-running-desc = Ó { $date }.
charts-avg-per-day = { $amount } / lá ar an meán
charts-avg-per-week = { $amount } / seachtain ar an meán
charts-avg-per-month = { $amount } / mí ar an meán
charts-people-title = Cé a d’iompair an grúpa
charts-people-desc = An méid a d’íoc gach duine, taobh leis an méid a d’úsáid siad.
charts-paid = Íoctha
charts-fair-share = Sciar cothrom
charts-you = (tusa)
charts-net-more = d’íoc níos mó
charts-net-less = d’íoc níos lú
charts-balance-title = Do chothromaíocht thar am
charts-balance-desc = Os cionn na líne tá an grúpa i bhfiacha leat. Faoina bun tá tusa i bhfiacha leis an ngrúpa.
charts-owed = Tá airgead ag dul duit
charts-owe = Tá tú i bhfiacha
charts-projects-title = Do sciar, de réir tionscadail
charts-projects-desc = Coinnítear iomláin de réir airgeadra.
history-empty = Gan imeachtaí
history-by = Ag { $name }
not-found-hint = Níl an leathanach seo ann, nó bogadh é.
payers-title-paid-by = Íoctha ag
payers-title-sender = Seoltóir
payers-title-contributors = Ranníocóirí
debtors-title-debtors = Fiacha ar
debtors-title-recipients = Faighteoirí
debtors-title-beneficiaries = Tairbhithe

### Welcome

welcome-title = Ní gnó do dhuine ar bith eile do chuntais.
welcome-subtitle = Roinn costais le cairde.
welcome-note = Saor in aisce. Níl cuntas ag teastáil. Gan fógraí.
welcome-link-title = Nasc amháin, agus glacann gach duine páirt.
welcome-link-body = Níl ar dhuine ar bith cuntas a chruthú.
welcome-link-account = Cuntas? Ní bhíonn gá leis riamh. Is le do thionscadail a aimsiú ar ghléas eile, cairde a thabhairt isteach ón aip agus do shonraí íocaíochta a roinnt é.
welcome-demo-project = Deireadh seachtaine in Lyon
welcome-private-title = Ní féidir le duine ar bith do chuntais a léamh. Fiú muidne.
welcome-private-body = Ainmneacha, suimeanna, tionscadail: criptítear iad go léir ar do ghléas. Is agatsa amháin atá an eochair.
welcome-private-names = Ainmneacha
welcome-private-amounts = Suimeanna
welcome-private-projects = Tionscadail
welcome-scan-title = Tóg grianghraf den admháil.
welcome-scan-body = Líontar an tsuim, an dáta agus an chatagóir leo féin. Tarlaíonn gach rud ar do ghuthán. Ní choinnítear an grianghraf.
welcome-eu-title = 100% Eorpach
welcome-no-ads = Gan fógraí
welcome-no-trackers = Gan rianairí
welcome-step = Céim { $current } as { $total }
welcome-next = Ar aghaidh
welcome-skip = Scipeáil
welcome-start = Tosaigh
welcome-how-it-works = Conas a oibríonn sé, go díreach?

### Help

help-intro = Ceist choitianta? Tapáil chun an freagra a leathnú.
help-create-project-q = Conas a chruthaím tionscadal?
help-create-project-a = Ón scáileán baile, tapáil an cnaipe + ag an mbun. Tabhair ainm don tionscadal, roghnaigh a airgeadra, agus tá tú réidh.
help-add-participants-q = Conas a chuirim rannpháirtithe leis?
help-add-participants-a = Oscail an tionscadal, ansin cuir rannpháirtithe leis ón liosta ball. Is féidir le gach rannpháirtí íoc as costas nó a bheith faoi fhiacha air.
help-share-project-q = Conas a chomhroinnim tionscadal?
help-share-project-a = Comhroinn URL an tionscadail (an ceann i do bharra seoltaí). Is féidir le duine ar bith a bhfuil an nasc aige an tionscadal a fheiceáil agus a chur in eagar.
help-add-expense-q = Conas a chuirim costas leis?
help-add-expense-a = Laistigh de thionscadal, tapáil +, cuir isteach an tsuim, abair cé a d'íoc agus cé eatarthu a roinntear é. Is féidir leat dáta seachas an lá inniu a roghnú freisin.
help-types-q = Cad é an difríocht idir costas, aistriú agus ioncam?
help-types-expense = - ceannachán a rinne duine amháin agus a roinntear idir roinnt daoine.
help-types-transfer = - aisíocaíocht ó dhuine amháin go duine eile, gan roinnt.
help-types-gain = - airgead a fuarthas (aisíoc, bronntanas) le roinnt idir roinnt daoine.
help-past-date-q = An féidir liom dáta san am atá thart a chur ar chostas?
help-past-date-a = Is féidir, tá réimse an dáta saor. Coinnítear am cruthaithe an taifid ar leith.
help-who-owes-q = Conas a oibríonn Counted amach cé a bhfuil fiacha air?
help-who-owes-a = Ríomhann Counted glan-iarmhéid gach rannpháirtí (an méid a chuir sé amach lúide an méid atá dlite uaidh), ansin molann sé an tsraith is giorra d'aistrithe a shocraíonn gach duine.
help-minimal-transfers-q = Cén fáth a bhfuil líon na n-aistrithe molta íosta?
help-minimal-transfers-a = Péireálann an t-algartam ar dtús iarmhéideanna a chealaíonn a chéile go díreach, ansin oibríonn sé tríd an gcuid eile ón gcreidiúnaí is mó go dtí an féichiúnaí is mó. An toradh: níos lú aistrithe chun gach rud a shocrú.
help-import-tricount-q = Conas a iompórtálaim tionscadal ó Tricount?
help-import-tricount-a = Ón scáileán baile, tapáil an cnaipe “+” ag an mbun, ansin
help-import-tricount-b = Greamaigh nasc comhroinnte an Tricount is mian leat a iompórtáil.
help-encryption-q = An bhfuil mo shonraí criptithe?
help-encryption-a = Tá. Cuireann Counted dhá ráthaíocht le chéile:
help-encryption-e2ee-term = Criptiú ceann go ceann
help-encryption-e2ee-def = - taistealaíonn gach rud idir tusa agus an freastalaí criptithe.
help-encryption-zero-term = Rochtain nialasach
help-encryption-zero-def = - criptíonn tú na sonraí sula seolann tú iad, agus ní stórálann an freastalaí ach cifearthéacs. Níl aon bhealach againn é a léamh.
help-encryption-see = Le haghaidh na sonraí, féach an
help-forgot-password-q = Cad a tharlaíonn má dhéanaim dearmad ar mo phasfhocal?
help-forgot-password-warning = Caillfear do shonraí go buan.
help-forgot-password-a = Díorthaítear an eochair chriptithe ó do phasfhocal, mar sin níl athshocrú ar bith indéanta: ní féidir le duine ar bith - muidne san áireamh - do thionscadail a dhíchriptiú gan é. Coinnigh slán é, i mbainisteoir pasfhocal más féidir.
help-archive-delete-q = Conas a chartlannaím nó a scriosaim tionscadal?
help-archive-delete-a = Ó scáileán an tionscadail, oscail an roghchlár agus roghnaigh
help-archive-delete-b = chun é a cheilt agus é a choinneáil. Scriostar tionscadal go deo nuair a fhágann a bhall deireanach é.
help-delete-account-q = Conas a scriosaim mo chuntas?
help-delete-account-a = Oscail Socruithe agus úsáid “Scrios mo chuntas”. Tá sé láithreach agus ní féidir é a chealú.
help-contact = Ceist eile? Scríobh chugainn ag

# Receipt scanning (mobile only)
expense-scan = Scan admháil
scan-in-progress = An admháil á léamh…
scan-error-capture = Níorbh fhéidir an grianghraf sin a thógáil. Bain triail eile as, nó cuir isteach an costas de láimh.
scan-error-unreadable = Faic inléite ar an admháil sin. Cuir isteach an costas de láimh.
scan-check-amount = Seiceáil an t-iomlán - ní raibh sé priontáilte go soiléir.
scan-take-photo = Tóg grianghraf
scan-choose-photo = Roghnaigh grianghraf
expense-converted-from = Íoctha { $amount } { $from } · 1 { $from } = { $rate } { $to }
project-currency = Airgeadra
project-currency-locked = Socraítear an t-airgeadra nuair a chruthaítear an tionscadal.
currency-search = Cuardaigh airgeadra

update-required-title = Nuashonrú ag teastáil
update-required-body = Tá an leagan seo de Counted róshean chun labhairt leis an bhfreastalaí. Nuashonraigh é chun leanúint ar aghaidh ag úsáid na haipe.
update-required-button = Nuashonraigh

notifications-label = Fógraí
notifications-title = Fógraí
notifications-empty = Faic nua
notifications-friend-request = Iarraidh chairdis

friends-title = Cairde
friends-anonymous-body = Coinnítear cairde le do chuntas. Logáil isteach chun daoine a chur leis agus cuireadh a thabhairt dóibh chuig do thionscadail gan nasc a chomhroinnt.
friends-add-title = Cuir cara leis
friends-add-hint = Feicfidh sé d'iarraidh nuair a logálann sé isteach. Ní insítear do cheachtar agaibh an bhfuil cuntas ag an duine eile go dtí go nglactar leis an iarraidh.
friends-add-button = Cuir leis
friends-add-from-project = Cuir leis mar chara
friends-request-sent = Iarraidh seolta
friends-no-account-key = Logáil isteach arís chun do chairde a bhainistiú ar an ngléas seo.
friends-incoming-title = Iarratais
friends-accept = Glac leis
friends-decline = Diúltaigh
friends-list-title = Mo chairde
friends-list-empty = Gan cairde fós. Cuir duine leis trí ríomhphost thuas, nó ó thionscadal a chomhroinneann sibh.
friends-remove = Bain
friends-remove-confirm-title = Bain cara
friends-remove-confirm-message = Ní bheidh { $email } i measc do chairde a thuilleadh, ná tusa i measc a chairde siúd. Is féidir le ceachtar agaibh iarratas nua a sheoladh níos déanaí.
friends-no-key = Níl sé réidh fós
friends-fingerprint = Cód sábháilteachta
friends-fingerprint-hint = Tá a fhios ag beirt chairde a léann an cód sábháilteachta céanna dá chéile nach bhfuil duine ar bith eatarthu - fiú ár bhfreastalaí.
friends-outgoing-title = Seolta
friends-outgoing-hint = Ag fanacht le freagra. Feicfidh tú iad i measc do chairde nuair a ghlacann siad leis.
friends-withdraw = Cealaigh
invite-friends-title = Tabhair cuireadh do chairde
invite-friends-hint = Criptítear eochair an tionscadail do gach cara ar an ngléas seo. Ní fheiceann an freastalaí choíche í.
invite-friends-empty = Gan cairde le cuireadh a thabhairt dóibh fós.
invite-friends-button = Tabhair cuireadh
invite-sent = { $count ->
    [one] Cuireadh seolta
    [two] { $count } chuireadh seolta
    [few] { $count } chuireadh seolta
    [many] { $count } gcuireadh seolta
   *[other] { $count } cuireadh seolta
}
invitation-badge = Cuireadh
invitation-to = Glac páirt i “{ $name }”
invitation-to-unnamed = Glac páirt i dtionscadal
invitation-unreadable = Ní féidir an cuireadh seo a oscailt ar an ngléas seo
invitation-from = Ó { $email }
invitation-accept = Glac páirt
invitation-decline = Diúltaigh

# Participants in the create and edit modals, and the "who are you?" picker - see
# docs/plans/friends.md §11.
participants-you-label = D'ainm sa tionscadal seo
participants-you-badge = Tusa
participants-others = Rannpháirtithe eile
participants-empty = Cuir rannpháirtithe leis thíos.
participants-duplicate = Tá “{ $name }” ar an liosta cheana.
participants-input-label = Cuir cara leis nó clóscríobh ainm
participants-input-placeholder = Cara nó ainm ar bith
participants-suggest-friend = Cara · glacfaidh páirt mar “{ $name }”, gheobhaidh cuireadh
participants-suggest-not-ready = Cara · níl sé réidh fós
participants-suggest-guest = Cuir “{ $text }” leis
participants-suggest-guest-sub = Aoi
participants-friends = Do chairde
participants-all-friends = Gach cara
participants-login-hint = Sínigh isteach chun daoine a chur leis díreach ó do liosta cairde.
participants-invite-badge = Tabhair cuireadh
participants-guest-badge = Aoi
participants-rename = Athainmnigh { $name }
participants-remove = Bain { $name }
participants-rename-label = Ainm nua
participants-rename-save = Sábháil an t-ainm
participants-rename-hint = An t-ainm a fheiceann gach duine sa tionscadal seo. Téann an cuireadh fós chuig { $email }.
participants-invited-badge = Cuireadh tugtha
participants-invited-sub = { $email } · níor glacadh leis fós
participants-invited-pending = Níor glacadh leis an gcuireadh fós
participants-unlinked = Níl sé nasctha le cuntas
add-project-create-invite = Cruthaigh agus tabhair cuireadh do { $count }
edit-project-save-invite = Sábháil agus tabhair cuireadh do { $count }
edit-project-you-are = Ar an ngléas seo is tusa { $name }
edit-project-no-identity = Níor roghnaigh tú fós cé thú féin
edit-project-switch = Athraigh
edit-project-choose = Roghnaigh
invite-failed = Níorbh fhéidir na cuirí seo a sheoladh: { $emails }
invite-again = Tabhair cuireadh arís
friend-picker-title = Cuir cairde leis
user-selection-invited-hint = Thug { $email } cuireadh duit chuig “{ $project }”.
user-selection-suggested = Molta
user-selection-suggested-sub = Chuir { $email } leis thú faoin ainm seo
user-selection-confirm-as = Is mise { $name }
user-selection-missing = Níl d'ainm anseo? Iarr ar rannpháirtí thú a chur leis i socruithe an tionscadail.

## Costais athfhillteacha

repeat-label = Athdhéan
repeat-none = Ní athdhéantar
repeat-weekly = Gach seachtain
repeat-biweekly = Gach 2 sheachtain
repeat-monthly = Gach mí
repeat-quarterly = Gach 3 mhí
repeat-yearly = Gach bliain
repeat-every-weeks = Gach { $count } seachtain
repeat-every-months = Gach { $count } mí
repeat-every-years = Gach { $count } bliain
repeat-custom = Saincheaptha…
repeat-every = Gach
repeat-unit-weeks = seachtain
repeat-unit-months = mí
repeat-unit-years = bliain
repeat-on-weekday = lá: { $weekday }
repeat-on-day = ar lá { $day } den mhí
repeat-on-day-month = ar { $day } { $month }
repeat-month-end = I míonna níos giorra, titeann sé ar an lá deireanach.
repeat-ends = Críochnaíonn
repeat-ends-never = Riamh
repeat-ends-on = Ar dháta
repeat-ends-after = Tar éis
repeat-fewer = Níos lú
repeat-more = Níos mó
repeat-last-on = an ceann deireanach ar { $date }
repeat-variable = Athraíonn an tsuim gach uair
repeat-variable-hint = Cuirtear gach ceann leis leis an tsuim dheireanach agus marcáiltear “le deimhniú” é.
repeat-done = Déanta
repeat-no-end = Gan deireadh
repeat-until = Go dtí { $date }
repeat-occurrences = Líon uaireanta: { $count }
repeat-offline = Tá nasc ag teastáil. Is féidir an costas féin a chur leis mar sin féin.
repeat-foreign = Athdhéantar mar { $amount } { $currency }, arna thiontú uair amháin ag ráta an lae inniu. Tabharfar rabhadh duit má athraíonn an ráta níos mó ná 5 %.
repeat-backfill = Tosaíonn san am atá thart. Costais curtha leis anois: { $count }.
add-and-repeat = Cuir leis agus athdhéan
weekday-1 = Dé Luain
weekday-2 = Dé Máirt
weekday-3 = Dé Céadaoin
weekday-4 = Déardaoin
weekday-5 = Dé hAoine
weekday-6 = Dé Sathairn
weekday-7 = Dé Domhnaigh
recurring-title = Costais athfhillteacha
recurring-strip = Costais athfhillteacha: { $count }
recurring-next = An chéad cheann eile: { $name }, { $date }
recurring-to-confirm = Le deimhniú: { $count }
recurring-per-month = In aghaidh na míosa, thart ar
recurring-your-share = Do sciar
recurring-active = Gníomhach
recurring-paused = Ar sos
recurring-finished = Críochnaithe
recurring-paid-by = íoctha ag { $name }
recurring-next-on = An chéad cheann eile ar { $date }
recurring-progress = { $done } as { $total }
recurring-rate-badge = D’athraigh an ráta { $percent } %
recurring-empty = Níl aon rud á athdhéanamh fós. Roghnaigh “Athdhéan” nuair a chuireann tú costas leis: cíos, síntiúis, billí.
recurring-next-ones = Na cinn eile
recurring-added-so-far = Curtha leis go dtí seo
recurring-set-up-by = Cruthaithe ag
recurring-pause = Cuir ar sos
recurring-resume = Lean ar aghaidh
recurring-stop = Stop an t-athdhéanamh
recurring-stop-title = Stop “{ $name }”?
recurring-stop-message = Ní athdhéanfar é a thuilleadh. Fanann na costais atá curtha leis cheana.
recurring-resume-title = Lean ar aghaidh le “{ $name }”?
recurring-resume-message = An chéad cheann eile ar { $date }. Ní chuirtear leis na dátaí a cailleadh le linn an tsosa.
recurring-edit-title = Cuir costas athfhillteach in eagar
recurring-edit-banner = Tá feidhm ag na hathruithe ó { $date }. Fanann na costais atá curtha leis cheana mar atá siad.
recurring-next-on-label = An chéad cheann eile ar
recurring-next-too-early = Caithfidh an chéad dáta eile a bheith i ndiaidh an chostais dheireanaigh a cuireadh leis.
recurring-use-stop = Chun é a chríochnú, úsáid “Stop an t-athdhéanamh” ar an gcostas athfhillteach.
recurring-drift = D’athraigh ráta { $currency } { $percent } % ó cruthaíodh é. Cuirtear gach ceann leis fós mar { $amount } { $project_currency } (1 { $currency } = { $rate }). Ag ráta an lae inniu bheadh sé { $today_amount } { $project_currency }.
recurring-use-rate = Úsáid ráta an lae inniu
recurring-keep = Coinnigh { $amount } { $currency }
recurring-added = Costais athfhillteacha curtha leis: { $count }
recurring-blocks-removal = Ní féidir { $name } a bhaint fós: cuid de { $rules }. Bain { $name } astu nó stop iad, ansin sábháil arís.
history-recurring-added = Curtha leis go huathoibríoch: { $name } ({ $date })
history-recurring-created = Costas athfhillteach socraithe: { $name }
history-recurring-edited = Costas athfhillteach curtha in eagar: { $name }
history-recurring-paused = Costas athfhillteach curtha ar sos: { $name }
history-recurring-resumed = Costas athfhillteach atosaithe: { $name }
history-recurring-stopped = Costas athfhillteach stoptha: { $name }
occurrence-recurring = Costas athfhillteach
occurrence-auto = Curtha leis go huathoibríoch ó chostas athfhillteach.
occurrence-auto-next = Curtha leis go huathoibríoch ó chostas athfhillteach. An chéad cheann eile ar { $date }.
occurrence-auto-stopped = Curtha leis go huathoibríoch ó chostas athfhillteach atá stoptha ó shin.
occurrence-manage = Bainistigh
estimate-badge = Le deimhniú
estimate-title = Suim le deimhniú.
estimate-body = Curtha leis leis an tsuim roimhe seo. Cuir isteach an fíorshuim nuair a bheidh sí agat.
estimate-confirm = Deimhnigh an tsuim
apply-to = Cuir i bhfeidhm ar
apply-this-only = An costas seo amháin
apply-and-next = An ceann seo agus na cinn eile
apply-and-next-hint = Athraíonn an costas athfhillteach ó { $date }
apply-rule-failed = Sábháladh an costas, ach níor athraíodh an costas athfhillteach.
occurrence-delete-message = Scriosfar “{ $name }” ó { $date } go buan. Leanann an t-athdhéanamh ar aghaidh agus ní thiocfaidh an dáta seo ar ais.
occurrence-delete-one = Scrios an ceann seo amháin
occurrence-delete-stop = Scrios agus stop an t-athdhéanamh
error-recurring-clock = Tá clog an ghléis seo chun tosaigh. Seiceáil an dáta agus an t-am.
error-recurring-not-found = Níl an costas athfhillteach seo ann a thuilleadh.
error-recurring-stale = D’athraigh duine éigin an costas athfhillteach seo idir an dá linn. Athlódáladh é: seiceáil é agus sábháil arís.
error-too-many-recurring = Tá 50 costas athfhillteach sa tionscadal seo cheana. Stop ceann nach bhfuil uait a thuilleadh chun ceann nua a chur leis.
error-user-in-recurring = Tá an rannpháirtí seo i gcostas athfhillteach. Bain as é ar dtús nó stop an costas.
