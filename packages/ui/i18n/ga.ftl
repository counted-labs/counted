# Gaeilge. Iomlán seachas na téacsanna dlí (legal-, terms-, privacy-), nach bhfuil ann ach i mBéarla
# agus i bhFraincis agus a thiteann siar ar en.ftl teachtaireacht ar theachtaireacht.

### Common

loading = Á lódáil…
cancel = Cealaigh
discard-title = Na hathruithe a chaitheamh uait?
discard-message = Caillfear a bhfuil curtha isteach agat.
discard = Caith uait
confirm = Deimhnigh
retry = Bain triail eile as
delete = Scrios
back = Siar
language = Teanga
theme = Téama
theme-light = Geal
theme-dark = Dorcha
theme-system = Córas
theme-hint-device = Sábháilte ar an ngléas seo amháin.
theme-hint-system = Leanann sé cuma an ghléis seo.

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
error-rate-limited = Tá an freastalaí gnóthach. Bain triail eile as i gceann nóiméid.
error-generic = Chuaigh rud éigin amú. Bain triail eile as.

error-invalid-email = Níl an seoladh ríomhphoist sin bailí.
error-invalid-password = Níl an pasfhocal sin bailí.
error-password-too-short = Caithfidh 12 charachtar ar a laghad a bheith i do phasfhocal.
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
login-maybe-unverified = Díreach cláraithe? Fíoraigh do sheoladh ar dtús leis an nasc a sheolamar chugat, nó seol arís é.
login-resend = Seol an nasc deimhnithe arís
login-resending = Á sheoladh…
login-resend-sent = Ríomhphost seolta - seiceáil do bhosca isteach.

register-submit = Cruthaigh cuntas
register-submitting = Á chruthú…
register-have-account = An bhfuil cuntas agat cheana?
register-password-placeholder = 12 charachtar ar a laghad
register-password-warning = Scríobh síos do phasfhocal: ní fheicimid riamh é agus ní féidir linn é a thabhairt ar ais duit.
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
erase-device-open = Logáil amach agus glan an gléas seo
erase-device-title = An gléas seo a ghlanadh?
erase-device-body = Scriosfar na tionscadail, a n-eochracha agus an taisce ar an ngléas seo, agus ansin cuirfear deireadh leis an seisiún. Fanann do thionscadail i do chuntas: logáil isteach arís chun iad a fháil ar ais.
erase-device-checking = Ag seiceáil go bhfuil do thionscadail i do chuntas…
erase-device-pending = Athruithe atá ag fanacht le seoladh: { $count }. Téigh ar líne arís, agus ansin bain triail eile as.
erase-device-unreachable = Níorbh fhéidir a sheiceáil go bhfuil do thionscadail i do chuntas. Seiceáil do nasc, agus ansin bain triail eile as.
erase-device-at-risk = Níl na tionscadail seo i do chuntas agus caillfear iad. Chun iad a choinneáil, cealaigh agus cóipeáil a nasc ar dtús:
erase-device-confirm = Glan agus logáil amach
erase-device-confirm-anyway = Glan mar sin féin
erase-device-erasing = Á ghlanadh…

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
payment-methods-stale = Athraíodh do shonraí íocaíochta ar ghléas eile. Athlódáladh iad - bain triail eile as.
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
verify-email-intro = Cuir isteach do phasfhocal le deimhniú gur leatsa an seoladh seo.
verify-email-submit = Fíoraigh mo sheoladh

### Project status

project-archive = Cartlannaigh
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
projects-offline-banner = Sonraí as líne - athnasc chun athnuachan.
demo-banner = Tionscadal taispeána - inléite amháin.
demo-start-own = Cruthaigh do thionscadal féin
demo-badge = Taispeántas
demo-bar = Imíonn d’athruithe nuair a dhúnann tú an cluaisín seo.
demo-leave = Tosaigh i ndáiríre
demo-preparing = Do thaispeántas á ullmhú…
demo-failed = Níorbh fhéidir an taispeántas a ullmhú.
demo-sign-in-blocked = Tá síniú isteach múchta sa taispeántas. Fág é chun síniú isteach nó cuntas a chruthú; níl do thionscadail féin athraithe.
projects-no-local-data = Gan sonraí áitiúla
projects-no-local-data-hint = Logáil isteach chun do thionscadail a lódáil den chéad uair.
projects-add = Cuir tionscadal leis
projects-create = Cruthaigh tionscadal
projects-personal-expense = Costas pearsanta
personal-expense-banner = Curtha le Mo chostais
personal-create-title = Cruthaigh do thionscadal pearsanta
personal-create-hint = Lean a gcaitheann tú i d’aonar
group-create-title = Cruthaigh tionscadal comhroinnte
group-create-hint = Roinn costais le cairde agus le do mhuintir
personal-create-action = Cruthaigh
personal-create-signup = Cruthaigh cuntas
personal-sheet-title = Do thionscadal pearsanta
personal-sheet-intro = Chun a gcaitheann tú i d’aonar a bhreacadh. Taispeántar é le do ghrúpaí sna Staitisticí.
personal-sheet-point-alone = Ball amháin: tusa, le hainm do chuntais. Gan nasc comhroinnte.
personal-sheet-point-encrypted = Criptithe ar do ghléasanna, cosúil le do thionscadail eile.
personal-sheet-point-stats = Cuirtear do chodanna sna grúpaí leis na costais seo sna Staitisticí.
personal-default-name = Mo chostais
my-spending-prev-month = An mhí roimhe
my-spending-next-month = An chéad mhí eile
my-spending-spent = Chaith tú
my-spending-groups = Grúpaí
my-spending-income = Ioncam
my-spending-balance = Iarmhéid na míosa
my-spending-unconverted = Fágtar scaireanna in airgeadra eile as go dtí go mbeidh ráta ar eolas.
my-spending-empty = Faic an mhí seo
my-spending-empty-hint = Cuir costas leis le +. Taispeánfar do scaireanna sna grúpaí anseo go huathoibríoch.
my-spending-share-of = as { $amount }
my-spending-uncounted = nár comhaireadh
payer-you = tú
expense-received-by = faighte ag
counted-switch = Comhair in Mo chostais
counted-switch-on = Ar siúl: cuirtear do scair le Mo chostais
counted-switch-off = As: ní thaispeántar do scair in Mo chostais
counted-groups = Grúpaí a chomhairtear
counted-groups-intro = Cuirtear do scair sna grúpaí seo le Mo chostais agus leis na buiséid.
counted-groups-converted = { $from }, tiontaithe go { $to }
counted-groups-encrypted = Tá an rogha seo criptithe le do chuntas agus leanann sé thú ar do ghléasanna.
counted-groups-empty = Níl aon ghrúpa ar an ngléas seo fós.
counted-groups-done = OK
counted-save-failed = Níorbh fhéidir an rogha seo a shábháil. Bain triail eile as.
tab-budget = Buiséad
budget-title = Buiséid
budget-manage = Bainistigh
budget-left = { $amount } fágtha
budget-over = Sáraithe faoi { $amount }
budget-empty = Gan buiséad
budget-empty-hint = Socraigh uasteorainn do gach catagóir. Comhairtear do scaireanna sna grúpaí freisin.
budget-define = Socraigh buiséad
budget-sheet-title = Buiséid mhíosúla
budget-sheet-intro = Ar do scair: pearsanta agus grúpaí. Fág folamh gan catagóir a leanúint.
budget-average = Meán 3 mhí dheireanacha: { $amount }
budget-for = Buiséad { $category }
budget-alert = Tabhair rabhadh dom ag 80 %
budget-alert-hint = Ríomhtha ar an ngléas seo
budget-invalid = Ní suim bhailí é do { $category }.
budget-offline = Ní féidir buiséid a shábháil ach ar líne. Athcheangail agus bain triail eile as.
budget-alert-near = { $category }: { $percent } % de bhuiséad na míosa
budget-alert-over = { $category }: buiséad na míosa sáraithe
personal-card-budget = { $month } · buiséad { $amount }
my-spending-search-summary =
    { $count ->
        [one] { $count } toradh
        [two] { $count } thoradh
        [few] { $count } thoradh
        [many] { $count } dtoradh
       *[other] { $count } toradh
    } · { $total } · pearsanta agus grúpaí, gach mí
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
expenses-unreadable = Ní féidir roinnt iontrálacha a léamh ar an ngléas seo agus níl siad sna hiomláin. Nuashonraigh an aip chun iad a fheiceáil.
project-unlock = Díghlasáil
project-no-local-data-hint = Logáil isteach chun sonraí an tionscadail seo a lódáil den chéad uair.
project-gone-title = Níl an tionscadal seo ann a thuilleadh
project-gone-hint = Scriosadh é nuair a d'fhág an ball deireanach. Ní oibríonn an nasc comhroinnte a thuilleadh, fiú má athosclaíonn tú é.

expense-add = Cuir costas leis
transfer-add = Cuir aistriú leis
expense-edit-title = Cuir an costas in eagar
expense-category = Catagóir
expense-category-auto = Uath · { $emoji }
emoji-picker-label = Emoji
emoji-none = Gan emoji
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
expense-type-group = Cineál
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
settlement-recorded = Socrú taifeadta.
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
move-expense = Bog go tionscadal
move-expense-title = Bog go…
move-current = Reatha
move-converted = { $currency }, tiontaithe ag ráta na míosa
move-note-personal = Roghnóidh tú cé a d’íoc agus cé dó sa chéad chéim eile. Fágfaidh an costas Mo chostais.
move-note-group = Fágfaidh an costas { $project } dá bhaill go léir, agus déanfar iarmhéid an ghrúpa a athríomh. Roghnóidh tú cé a d’íoc agus cé dó sa chéad chéim eile.
move-banner = Ó { $from } go { $to }
move-submit = Bog
move-done = Bogadh an costas go { $project }
history-expense-moved = Bogadh an costas go tionscadal eile: { $name }
personal-badge = Pearsanta
move-members =
    { $count ->
        [one] { $count } ball
        [two] { $count } bhall
        [few] { $count } bhall
        [many] { $count } mball
       *[other] { $count } ball
    }
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
expense-share-percent = { $percent }%
expense-added-by = Curtha leis ag { $name } ar { $date }
expense-inconsistent-detail = Ní thagann na suimeanna le chéile: { $paid } íoctha, { $owed } dlite, do chostas { $total }. Cuir an costas in eagar chun é a cheartú.
missing-access-key = Eochair rochtana ar iarraidh. Oscail an tionscadal seo trína nasc comhroinnte.
filter-all = Uile
filter-my-payments = M'íocaíochtaí
filter-my-debts = Mo chuid fiacha
expense-filter-group = Scagaire
expenses-search = Cuardaigh costais
expenses-no-match = Níl aon chostas ag teacht leis.
expenses-search-open = Cuardaigh
expenses-search-clear = Glan an cuardach
expenses-search-summary =
    { $count ->
        [one] { $count } toradh
        [two] { $count } thoradh
        [few] { $count } thoradh
        [many] { $count } dtoradh
       *[other] { $count } toradh
    } · { $total } · gach mí
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
        [one] Níl { $count } tionscadal amháin san áireamh - gan rannpháirtí roghnaithe, nó níor lódáladh a shonraí.
        [two] Níl { $count } thionscadal san áireamh - gan rannpháirtí roghnaithe, nó níor lódáladh a sonraí.
        [few] Níl { $count } thionscadal san áireamh - gan rannpháirtí roghnaithe, nó níor lódáladh a sonraí.
        [many] Níl { $count } dtionscadal san áireamh - gan rannpháirtí roghnaithe, nó níor lódáladh a sonraí.
       *[other] Níl { $count } tionscadal san áireamh - gan rannpháirtí roghnaithe, nó níor lódáladh a sonraí.
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
categories-manage = Bainistigh catagóirí
categories-title = Catagóirí
categories-yours = Do chatagóirí
categories-builtin = Réamhshocraithe
categories-add = Cuir catagóir leis
categories-in-parent = sa chatagóir { $parent }
categories-renamed-from = athainmnithe · { $name } ar dtús
categories-hidden-hint = i bhfolach · téann a costais faoi { $other }
categories-always-shown = taispeántar i gcónaí
categories-hidden = I bhfolach
categories-new = Catagóir nua
categories-edit = Cuir an chatagóir in eagar
categories-parent = Comhairtear faoi
categories-parent-hint = I staitisticí do thionscadal go léir, comhairtear { $name } faoi { $parent }.
categories-create = Cruthaigh
categories-hide = Folaigh an chatagóir seo
categories-hide-hint = Taispeántar a costais faoi { $other } do gach ball.
categories-delete-title = Scrios “{ $name }”?
categories-delete-body = Filleann na costais lena mbaineann ({ $count }) ar { $parent }. Ní scriostar aon chostas.
categories-delete-body-unknown = Filleann a costais ar { $parent }. Ní scriostar aon chostas.
categories-delete-warning = Imíonn an chatagóir do gach ball den tionscadal.
categories-summary-created = Cruthaithe: { $count }
categories-summary-renamed = Athainmnithe: { $count }
categories-summary-hidden = I bhfolach: { $count }
categories-summary-default = Gan catagóirí saincheaptha
categories-offline = Ní féidir catagóirí a bhainistiú ach ar líne. Athcheangail agus bain triail eile as.
categories-error-too-many = Bíonn 20 catagóir dá chuid féin ar a mhéad ag tionscadal.
categories-error-full = Tá teorainn na gcatagóirí bainte amach ag an tionscadal seo.
categories-error-name = Tabhair ainm don chatagóir.
categories-error-missing = Athraíodh an chatagóir seo idir an dá linn. Dún agus bain triail eile as.
history-category-created = Catagóir cruthaithe: { $name }
history-category-edited = Catagóir curtha in eagar: { $name }
history-category-deleted = Catagóir scriosta: { $name }
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
debtors-title-for = Do
debtors-title-recipient = Faighteoir
expense-split-between = roinnte ar { $count }
expense-people =
    { $count ->
        [one] { $count } duine
        [two] { $count } dhuine
        [few] { $count } dhuine
        [many] { $count } nduine
       *[other] { $count } duine
    }

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
help-forgot-password-warning = Ní féidir le duine ar bith é a aisghabháil duit.
help-forgot-password-a = Cosnaíonn do phasfhocal d'eochracha criptithe agus ní sheoltar chugainn riamh é: ní féidir le duine ar bith - muidne san áireamh - do thionscadail a dhíchriptiú ar do shon. Coinnigh slán é, i mbainisteoir pasfhocal más féidir.
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
friends-add-hint = Má tá cuntas ag an duine seo, feicfidh sé d'iarraidh san aip. Mura bhfuil, cuir in iúl dó é: gheobhaidh sé í nuair a chláraíonn sé leis an seoladh seo.
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
friends-key-changed = Athraíodh an cód
friends-key-changed-hint = D’athraigh cód slándála carad. Tarlaíonn sé sin nuair a dhaingníonn sé a chuntas, nó má cuireadh eochair eile in áit a eochrach. Tá cuirí leis bactha go léifidh sibh an cód nua dá chéile, duine le duine nó ar an bhfón, agus go mbrúfaidh tú “Tá an cód mar an gcéanna”.
friends-fingerprint = Cód sábháilteachta
friends-fingerprint-hint = Tá a fhios ag beirt chairde a léann an cód sábháilteachta céanna dá chéile nach bhfuil duine ar bith eatarthu - fiú ár bhfreastalaí.
friends-key-mismatch = Sheol an freastalaí eochair phoiblí do do chuntas nach dtagann le d'eochair phríobháideach. Tá do chód sábháilteachta i bhfolach agus tá cuirí múchta ar an ngléas seo: b'fhéidir go bhfuil duine éigin ag cur isteach ar an bhfreastalaí.
keypair-mismatch = Ní thagann na heochracha a sheol an freastalaí do do chuntas leis na cinn ar an ngléas seo. Tá cuirí agus athruithe eochrach múchta anseo go dtí go dtagann siad: b'fhéidir go bhfuil duine ag cur isteach ar an bhfreastalaí.
friends-outgoing-title = Seolta
friends-outgoing-hint = Ag fanacht le freagra. Feicfidh tú iad i measc do chairde nuair a ghlacann siad leis.
friends-withdraw = Cealaigh
friends-notify = Cuir in iúl le teachtaireacht
friends-notify-label = Cuir in iúl do { $email }
friends-notify-message = Táim ag aistriú go Counted dár gcostais roinnte. Clárú saor in aisce ar https://counted.fr leis an seoladh { $email }: tá m'iarratas cairdis ag fanacht ansin.
friends-notify-copied = Teachtaireacht cóipeáilte
friends-notify-copy-failed = Níorbh fhéidir an teachtaireacht a chóipeáil
guests-hint-title =
    { $count ->
        [one] Níl { $count } rannpháirtí tar éis teacht isteach sa tionscadal fós
        [two] Níl { $count } rannpháirtí tar éis teacht isteach sa tionscadal fós
        [few] Níl { $count } rannpháirtí tar éis teacht isteach sa tionscadal fós
        [many] Níl { $count } rannpháirtí tar éis teacht isteach sa tionscadal fós
       *[other] Níl { $count } rannpháirtí tar éis teacht isteach sa tionscadal fós
    }
guests-hint-later = Ar ball
guests-hint-send = Tabhair cuireadh dóibh
guests-send-title = Seol an tionscadal chuig na rannpháirtithe
guests-send-friend = Tabhair cuireadh do chara Counted
guests-send-friend-badge = Gan nasc
guests-send-friend-hint = Má tá cuntas aige cheana, faigheann sé cuireadh criptithe; ní théann aon rud trí d'aip teachtaireachtaí.
guests-send-link = Roinn nasc an tionscadail
guests-send-link-hint = Dóibh siúd gan chuntas. Osclaíonn siad an tionscadal le tapáil amháin.
guests-send-link-warning = Is féidir le duine ar bith a bhfuil an nasc seo aige an tionscadal a léamh agus a athrú. Feiceann d'aip teachtaireachtaí é freisin.
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
participants-removed = Bainte
participant-restore = Athchóirigh
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
invite-waiting = Caithfidh na cairde seo Counted a oscailt uair amháin sular féidir cuireadh a thabhairt dóibh: { $emails }
key-offer-title = Eochair eile do { $name }
key-offer-body = Tá eochair eile ag an ngléas seo don tionscadal seo, ceann nár oscail riamh anseo é.
key-offer-choices = Úsáid an eochair seo: osclaítear an tionscadal leis an eochair ón nasc nó ón gcuireadh seo. Cealaigh: coinníonn an gléas a eochair reatha, agus ní athraíonn aon rud.
key-offer-use = Úsáid an eochair seo
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
expense-pending = Ar feitheamh
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

home-owed-to-you = Atá ag dul duit
home-you-owe = Atá ort
home-details-show = Taispeáin sonraí
home-details-hide = Folaigh sonraí
home-all-settled = Tá gach rud socraithe
home-pay = Íoc
home-remind = Meabhraigh
home-remind-message = Meabhrúchán beag: tá aisíocaíochtaí fós le socrú in { $project }.
home-row-owed = ag dul duit
home-row-owe = ort
home-row-settled = Socraithe
home-row-quits = Tá tú réidh
home-row-pending =
    { $count ->
        [one] { $count } aisíocaíocht ar feitheamh
        [two] { $count } aisíocaíocht ar feitheamh
        [few] { $count } aisíocaíocht ar feitheamh
        [many] { $count } aisíocaíocht ar feitheamh
       *[other] { $count } aisíocaíocht ar feitheamh
    }
home-totals-partial = Gan tionscadail gan ráta malairte
error-account-key-changed = Athraíodh eochair do chuntais ar ghléas eile. Sínigh isteach arís chun leanúint ar aghaidh.
settings-security = Slándáil
settings-change-password = Athraigh an pasfhocal
settings-change-password-hint = Fanann do thionscadail agus do shonraí mar atá siad. Sínítear amach do ghléasanna eile.
change-password-title = Athraigh do phasfhocal
change-password-current = Pasfhocal reatha
change-password-new = Pasfhocal nua
change-password-confirm = Deimhnigh an pasfhocal nua
change-password-mismatch = Ní hionann an dá phasfhocal nua.
change-password-submit = Athraigh an pasfhocal
change-password-submitting = Á athrú…
change-password-done = Athraíodh an pasfhocal. Síníodh amach do ghléasanna eile.
error-rotation-incomplete = D’athraigh do chuntas idir an dá linn. Bain triail eile as.
error-key-lifecycle-unavailable = Níl sé ar fáil fós: ar dtús teastaíonn an leagan is déanaí den aip ó gach duine.
settings-compromised = B’fhéidir go bhfuil mo phasfhocal ag duine éigin
settings-compromised-hint = Pasfhocal nua, eochracha nua, agus sínítear amach gach gléas eile.
settings-compromised-open = Daingnigh mo chuntas
compromised-title = Daingnigh do chuntas
compromised-body = Cuirtear pasfhocal agus eochracha nua ar do chuntas, agus sínítear amach gach gléas eile. Feicfidh do chairde go n-athraíonn do chód slándála: cuir i gcomparáid arís é.
compromised-keeps = Ní féidir an méid a osclaíodh le do phasfhocal cheana a bhaint siar: na tionscadail a bhí inléite go dtí seo.
compromised-submit = Athraigh pasfhocal agus eochracha
compromised-done = Tá do chuntas daingnithe. Iarr ar do chairde do chód slándála nua a chur i gcomparáid.
friends-new-fingerprint = Cód nua:
friends-code-matches = Tá an cód mar an gcéanna
friends-code-trusted = Cód deimhnithe.
settings-recovery-key = Eochair athshlánaithe
settings-recovery-key-none = Gan í, cosnaíonn pasfhocal dearmadta do roghanna, do shonraí íocaíochta agus cóid do chairde, agus tógann sé 3 lá.
settings-recovery-key-set = Socraithe. Má dhéanann tú dearmad ar do phasfhocal, tugann sí do chuntas ar ais agus gach rud ann.
settings-recovery-key-pending = Cuirfear eochair nua ina háit ar { $when }. Nárbh tú a bhí ann? Cealaigh í le d’eochair reatha.
recovery-key-create = Cruthaigh
recovery-key-replace = Athchuir
recovery-key-cancel-replacement = Cealaigh an eochair nua
recovery-key-title = D’eochair athshlánaithe
recovery-key-body = Scríobh síos í nó sábháil áit nach féidir le duine ar bith eile teacht uirthi. Ní féidir le Counted í a thaispeáint arís ná í a fháil ar ais duit.
recovery-key-copy = Cóipeáil
recovery-key-copied = Cóipeáilte
recovery-key-save = Sábháil mar chomhad
recovery-key-print = Priontáil
recovery-key-next = Ar aghaidh
recovery-key-confirm-body = Chun do chóip a sheiceáil, clóscríobh grúpa { $n } den eochair.
recovery-key-confirm-wrong = Ní hionann an grúpa sin. Féach ar do chóip arís.
recovery-key-password-body = Cuir isteach do phasfhocal chun an eochair a shábháil ar do chuntas.
recovery-key-submit = Sábháil an eochair
recovery-key-done = Sábháladh an eochair athshlánaithe.
recovery-key-replacement-done = Sábháladh an eochair nua. Cuirfear in áit na heochrach reatha í i gceann 72 uair; go dtí sin oibríonn an eochair reatha fós.
recovery-key-field = Eochair athshlánaithe
recovery-key-file-intro = Eochair athshlánaithe Counted
recovery-key-cancel-done = Cealaíodh an eochair nua.
compromised-new-key = Sábháil d’eochair athshlánaithe nua anois: cuirfear in áit na seaneochrach í i gceann 72 uair, agus go dtí sin oibríonn an tseaneochair fós.
pending-reset-title = Iarradh d’athshocrú do chuntais
pending-reset-body = Tiocfaidh sé i bhfeidhm ar { $when }: caillfear do roghanna, do shonraí íocaíochta agus cóid do chairde, agus beidh ar do ghléasanna síniú isteach arís. Nárbh tú a bhí ann? Cealaigh é le do phasfhocal reatha.
pending-reset-cancel = Cealaigh an t-athshocrú
pending-reset-cancelled = Cealaíodh an t-athshocrú.
login-forgot = Pasfhocal dearmadta?
forgot-title = Pasfhocal dearmadta
forgot-body = Cuir isteach do ríomhphost. Má tá cuntas aige, seolfaimid nasc chun pasfhocal nua a roghnú.
forgot-submit = Seol an nasc
forgot-sent = Má tá cuntas ag an seoladh seo, tá nasc ar an mbealach. Rachaidh sé in éag i gceann 24 uair.
reset-title = Faigh do chuntas ar ais
reset-with-key-body = Cuir isteach d’eochair athshlánaithe agus pasfhocal nua: tagann gach rud ar ais mar a bhí.
reset-submit-key = Athshlánaigh mo chuntas
reset-no-key = Níl m’eochair athshlánaithe agam
reset-without-key-title = Athshocraigh gan an eochair athshlánaithe
reset-without-key-body = Tosóidh do chuntas as an nua le pasfhocal nua i gceann 72 uair. Tagann do thionscadail ar ais ó do ghléasanna agus ó naisc chomhroinnte; ní thagann do roghanna, do shonraí íocaíochta ná cóid do chairde. Go dtí sin oibríonn do phasfhocal reatha agus is féidir leis an t-athshocrú a chealú.
reset-name = D’ainm
reset-submit-request = Athshocraigh mo chuntas
reset-requested = Déanta. Oibreoidh do phasfhocal nua ó { $when }. Go dtí sin oibríonn do phasfhocal reatha agus is féidir leis é a chealú.
reset-pending-already = Tá athshocrú ar feitheamh cheana go dtí { $when }. Tosaíonn iarratas nua na 72 uair arís.
recovered-done = Tá do chuntas ar ais. Síníodh amach do ghléasanna eile.
login-reset-undoable = Cuireadh athshocrú in áit an phasfhocail seo le 30 lá anuas. Murar iarr tú é, cealaigh é: tagann do chuntas ar ais mar a bhí.
login-undo-reset = Cealaigh an t-athshocrú
reset-undone = Cealaíodh an t-athshocrú. Tá do chuntas mar a bhí.
error-reset-pending = Oibreoidh an pasfhocal nua seo nuair a thiocfaidh an t-athshocrú i bhfeidhm, 72 uair tar éis an iarratais.
error-account-disabled = Scriosadh an cuntas seo go gairid tar éis athshocraithe agus tá sé ag fanacht le léirscriosadh. Is féidir leis an seanphasfhocal an t-athshocrú a chealú fós.
error-recovery-key-invalid = Ní osclaíonn an eochair athshlánaithe seo an cuntas seo.
error-project-key-required = Teastaíonn a eochair ón tionscadal seo. Oscail arís é óna nasc comhroinnte.
names-or = { $first } nó { $last }
participants-admin-badge = Riarthóir
edit-project-admins = Riarthóirí: { $count }
edit-project-succession = Má fhágann an riarthóir deireanach an tionscadal, déantar riarthóir den bhall le cuntas is faide sa tionscadal.
member-sheet-account = Ball le cuntas
member-actions = Gníomhartha: { $name }
member-make-admin = Tabhair ról an riarthóra
member-make-admin-hint = Is féidir eochair an tionscadail a athrú agus duine a bhaint ansin.
member-made-admin = Tá ról an riarthóra ag { $name } anois.
group-security = Slándáil an tionscadail
group-security-admin = Slándáil an tionscadail · tá ról an riarthóra agat
group-rotate = Athraigh eochair an tionscadail
group-rotate-hint = Má sceitheadh nasc an tionscadail: caillfidh lucht an tseannaisc rochtain ar an tionscadal.
group-remove = Bain duine den tionscadal
group-remove-hint = Baineann sé an rochtain agus athraíonn sé an eochair ag an am céanna.
group-admins-only = Do riarthóirí amháin.
group-admins-only-account = Do riarthóirí amháin, a bhfuil cuntas acu.
group-ask-admin = Iarr ar { $names }, riarthóir an tionscadail.
group-ask-admins = Iarr ar { $names }, riarthóirí an tionscadail.
group-ask-an-admin = Iarr ar dhuine de riarthóirí an tionscadail.
group-with-account = Le cuntas
group-free = Saor in aisce
group-sign-in-hint = Is féidir le riarthóir an tionscadail, { $names }, an ról a thabhairt duit ansin.
group-sign-in-hint-any = Is féidir le riarthóir an tionscadail an ról a thabhairt duit ansin.
group-no-admin = Níl riarthóir ag an tionscadal seo, mar sin ní féidir a eochair a athrú.
group-hide = Folaigh duine
group-hide-hint = Folaíonn sé an duine sna liostaí gan an rochtain a bhaint: oibríonn an nasc agus an eochair fós.
group-recreate-hint = Níl riarthóir ag an tionscadal seo. Chun duine a bhaint, cruthaigh tionscadal nua.
group-recreate = Cruthaigh tionscadal nua
group-no-admin-yet = Níl riarthóir ag an tionscadal seo fós.
group-creator-hint = Is tusa amháin atá in ann a bheith i do riarthóir ar an tionscadal seo, trí chuntas a chruthú.
group-creator-elsewhere-hint = Níl ach an duine a chruthaigh an tionscadal in ann a bheith ina riarthóir air, trí chuntas a chruthú. Seachas sin, cruthaigh tionscadal nua chun duine a bhaint.
member-revoke-admin = Bain ról an riarthóra
member-revoke-admin-hint = Ní féidir eochair an tionscadail a athrú ná duine a bhaint a thuilleadh.
member-revoked-admin = Níl ról an riarthóra ag { $name } a thuilleadh.
member-step-down = Éirigh as mar riarthóir
member-step-down-hint = Ní bheidh tú in ann eochair an tionscadail a athrú ná duine a bhaint a thuilleadh.
member-stepped-down = Níl ról an riarthóra agat a thuilleadh.
member-remove = Bain den tionscadal
member-remove-hint = Athraíonn sé eochair an tionscadail: ní bheidh { $name } in ann aon rud nua a léamh.
member-hide = Folaigh sna liostaí
error-user-not-admin = Ní riarthóir é an ball seo.
error-creator-proof-invalid = Níor chruthaigh an gléas seo an tionscadal seo.
error-project-has-admin = Tá riarthóir ag an tionscadal seo cheana.
error-not-project-admin = Ní féidir ach le riarthóirí an tionscadail é seo a dhéanamh.
error-user-not-claimed = Níl cuntas ag an rannpháirtí seo, agus ní féidir ach le cuntas a bheith ina riarthóir.
error-project-key-rotated = Athraíodh eochair an tionscadail. Iarr an nasc nua ar bhall.
error-project-rotating = Tá eochair an tionscadail á hathrú. Bain triail eile as i gceann nóiméid.
error-rotation-expired = Thóg an t-athrú eochrach rófhada agus cuireadh ar ceal é. Tosaigh arís.
error-rotation-rows-changed = Athraíodh an tionscadal le linn an athraithe eochrach. Tosaigh arís.
project-rotated-title = Níl rochtain agat ar an tionscadal seo a thuilleadh
project-rotated-body = Athraíodh eochair an tionscadail seo. Iarr an nasc nua ar riarthóir an tionscadail.
project-rotated-open-link = Oscail nasc
rotation-remove-title = { $name } a bhaint den tionscadal?
rotation-rotate-title = Eochair an tionscadail a athrú?
rotation-remove-body = Nuair a bhaintear ball, cruthaítear eochair nua. Beidh ort an nasc nua a roinnt leis na daoine nach bhfuil cuntas acu.
rotation-rotate-body = Ní bheidh rochtain ar an tionscadal leis an seannasc a thuilleadh. Beidh ort an nasc nua a roinnt leis na daoine nach bhfuil cuntas acu.
rotation-boxed =
    { $count ->
        [0] Ní bhfaighidh aon bhall eile an eochair go huathoibríoch
       *[other] Baill a gheobhaidh an eochair go huathoibríoch: { $count }
    }
rotation-by-link-title = Daoine a mbeidh ort an nasc nua a sheoladh chucu
rotation-by-link-hint = Níl cuntas ag na daoine seo. Seolfaidh tú an nasc chucu díreach ina dhiaidh sin.
rotation-names-and = { $first } agus { $last }
rotation-names-and-others = { $names } agus { $count } eile
rotation-confirm-remove = Bain agus athraigh an eochair
rotation-confirm-rotate = Athraigh an eochair
rotation-done-title = Tá an eochair nua i bhfeidhm
rotation-done-removed = Níl { $name } páirteach sa tionscadal a thuilleadh.
rotation-done-rotated = Athraíodh eochair an tionscadail.
rotation-done-boxed =
    { $count ->
        [0] Ní bhfuair aon bhall eile an eochair go huathoibríoch.
       *[other] Baill a fuair an eochair go huathoibríoch: { $count }.
    }
rotation-new-link =
    { $count ->
        [0] Nasc nua an tionscadail
       *[other] Nasc nua le seoladh · daoine gan chuntas: { $count }
    }
rotation-link-renewed = Athnuadh an nasc.
rotation-done = Déanta
history-key-changed = Athraíodh eochair an tionscadail
history-member-removed = Níl { $name } páirteach sa tionscadal a thuilleadh
project-rotated-row-hint = Athraíodh an eochair. Iarr an nasc nua.
