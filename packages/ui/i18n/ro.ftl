# Română. Complet, cu excepția textelor juridice (legal-, terms-, privacy-), care există doar în
# engleză și franceză și revin la en.ftl mesaj cu mesaj.

### Common

loading = Se încarcă…
cancel = Anulează
confirm = Confirmă
retry = Încearcă din nou
delete = Șterge
back = Înapoi
language = Limbă

### Navigation

nav-main = Navigare principală
nav-projects = Proiecte
nav-charts = Statistici
nav-settings = Setări

### Connectivity

offline-banner = Offline
offline-pending =
    { $count ->
        [one] { $count } în așteptare
        [few] { $count } în așteptare
       *[other] { $count } în așteptare
    }

sync-conflict-edit = Conflict: editarea „{ $name }” a eșuat (element șters). Omis.
sync-conflict-delete = Conflict: ștergerea „{ $name }” a eșuat (element șters). Omis.
sync-conflict-other = Conflict: operațiunea pe „{ $name }” a eșuat (element șters). Omis.
sync-error = Eroare de sincronizare: { $reason }

### Errors

error-network = Serverul nu poate fi contactat. Verifică-ți conexiunea la internet.
error-generic = Ceva n-a mers bine. Încearcă din nou.

error-invalid-email = Această adresă de e-mail nu este validă.
error-invalid-password = Această parolă nu este validă.
error-password-too-short = Parola trebuie să aibă cel puțin 8 caractere.
error-client-outdated = Această versiune a aplicației este învechită. Actualizeaz-o pentru a te conecta.
error-invalid-link = Acest link nu este valid.
error-batch-too-large = Prea multe elemente deodată.
error-payers-required = Selectează cel puțin un plătitor.
error-debtors-required = Selectează cel puțin o persoană care datorează.
error-duplicate-participant = Un participant apare de două ori pe aceeași parte.
error-participant-not-in-project = Acest participant nu face parte din proiect.
error-too-many-participants = Prea mulți participanți pentru o singură cheltuială.
error-invalid-credentials = E-mail sau parolă incorecte.
error-unauthenticated = Conectează-te pentru a face asta.
error-email-not-verified = Adresa ta de e-mail nu este încă verificată.
error-project-not-found = Acest proiect nu mai există.
error-expense-not-found = Această cheltuială nu mai există.
error-storage-full = Spațiul de stocare este plin: cheia acestui proiect nu a putut fi salvată pe acest dispozitiv. Păstrează linkul de partajare.
error-user-not-found = Acest participant nu mai există.
error-tricount-not-found = Tricount negăsit sau API-ul său a returnat o eroare.
error-too-many-members = Acest proiect a atins limita de membri.
error-identity-taken = Un alt cont a revendicat deja acest participant.
error-claim-proof-invalid = Acest dispozitiv nu deține cheia proiectului, deci nu poate revendica un participant. Deschide din nou linkul de partajare.
error-user-has-payments = Acest participant are cheltuieli în proiect și nu poate fi eliminat.
error-resend-cooldown = Așteaptă 60 de secunde înainte de a cere un alt e-mail.
error-self-friend-request = Nu te poți adăuga pe tine ca prieten.
error-not-a-friend = Poți invita doar persoane din lista ta de prieteni.
error-friend-has-no-key = Acest prieten nu a deschis încă ultima versiune a aplicației. Roagă-l să se conecteze o dată, apoi încearcă din nou.
error-friend-request-not-found = Această cerere de prietenie nu mai există.
error-invitation-not-found = Această invitație nu mai există.
error-too-many-friend-requests = Prea multe cereri de prietenie deocamdată. Încearcă mâine.
error-too-many-invitations = Prea multe invitații în așteptare.
error-invalid-kdf-salt = Setările de criptare nu sunt valide. Actualizează aplicația și încearcă din nou.
error-mixed-project-batch = Acești participanți nu sunt toți în același proiect.
error-invalid-payload = Această versiune a aplicației a trimis date pe care serverul nu le acceptă. Actualizeaz-o și încearcă din nou.
error-invalid-public-key = Cheia ta de criptare nu este validă. Actualizează aplicația și încearcă din nou.
error-payment-methods-stale = Datele tale de plată au fost modificate pe alt dispozitiv. Reîncarcă și încearcă din nou.

### Auth

field-email = E-mail
field-email-placeholder = tu@exemplu.ro
field-password = Parolă
field-name = Nume
field-name-placeholder = Ana Popescu

login-title = Conectare
login-submit = Conectează-te
login-submitting = Conectare…
login-password-placeholder = Parola ta
login-no-account = Nu ai încă un cont?
login-unverified = Adresa ta de e-mail nu este încă verificată. Verifică-ți inbox-ul sau trimite linkul din nou.
login-resend = Trimite din nou linkul de verificare
login-resending = Se trimite…
login-resend-sent = E-mail trimis - verifică-ți inbox-ul.

register-submit = Creează un cont
register-submitting = Se creează…
register-have-account = Ai deja un cont?
register-password-placeholder = Cel puțin 8 caractere
register-password-warning = Notează-ți parola. Dacă o uiți, contul nu poate fi recuperat.
register-check-email-title = Verifică-ți e-mailul
register-email-sent = E-mail trimis
register-email-sent-hint = Apasă pe linkul din inbox pentru a-ți activa contul.
register-not-received-prefix = Nu l-ai primit? Verifică folderul spam sau
register-sign-in-link = conectează-te
register-not-received-suffix = pentru a trimite linkul din nou.
register-terms-prefix = Creând un cont, accepți
register-terms-link = termenii de utilizare
register-terms-and = și
register-privacy-link = politica noastră de confidențialitate

settings-title = Setări
settings-preferences = Preferințe
settings-preferences-local = Salvate pe acest dispozitiv.
settings-preferences-synced = Sincronizate cu contul tău, criptate.
settings-about = Despre
settings-anonymous-title = Nu ești conectat
settings-upsell-title = Proiectele tale, pe orice dispozitiv
settings-upsell-free = Gratuit
settings-upsell-body = Counted funcționează fără cont. Cu unul gratuit, proiectele și preferințele te urmează pe telefon, laptop și web - tot criptate, tot ilizibile pentru noi.
settings-locked-badge = Cont
settings-locked-friends = Creează un cont pentru a adăuga prieteni și a-i invita într-un proiect din aplicație - fără link de dat mai departe.
settings-locked-payment-methods = Salvează-ți IBAN-ul sau aplicația de plată o singură dată și partajează-le cu proiectele alese. Cine îți datorează bani le vede lângă numele tău.
settings-friends-hint = Adaugă prieteni și invită-i în proiectele tale fără a partaja un link.

account-member-since = Membru din
account-logout = Deconectare
account-logging-out = Deconectare…
account-delete-title = Șterge-mi contul
account-delete-warning = Ștergerea imediată și definitivă a contului și a datelor tale.
account-delete-confirm-title = Ștergere cont
account-delete-confirm-message = Contul tău, sesiunile și lista de proiecte vor fi șterse definitiv. Fără parola ta, datele criptate ale unui proiect partajat devin ilizibile pentru tine - acest lucru nu poate fi anulat.

settings-payment-methods = Detalii de plată
settings-payment-methods-hint = Cum vrei să fii rambursat. Criptate cu contul tău.
payment-method-kind = Metodă
payment-method-kind-other = Altele
payment-method-label = Nume
payment-method-label-placeholder = Cont principal
payment-method-value = Detalii
payment-method-value-placeholder = IBAN, număr de telefon, nume de utilizator…
payment-method-add = Adaugă
payment-method-remove = Elimină { $name }
payment-method-empty = Nu ai adăugat încă detalii de plată.
payment-method-deleted = Metodă de plată ștearsă.
payment-method-value-required = Completează detaliile fiecărei metode de plată sau elimin-o.
payment-method-label-required = Dă un nume metodei tale personalizate.
payment-method-too-long = Este prea lung - scurtează-l.
payment-method-invalid-characters = Elimină rândurile noi sau caracterele invizibile.
payment-method-limit = Poți salva până la { $max } metode de plată.
payment-methods-saved = Detalii de plată salvate.
payment-methods-offline = Trebuie să fii online pentru a-ți salva detaliile de plată.
payment-methods-stale = Detaliile tale de plată au fost modificate pe alt dispozitiv. Au fost reîncărcate — te rugăm să încerci din nou.
payment-methods-key-missing = Conectează-te din nou pentru a-ți gestiona detaliile de plată.
settings-payment-methods-share-warning = O metodă partajată este vizibilă tuturor membrilor proiectelor tale.
payment-method-share = Partajează cu proiectele mele
payment-method-share-hint = Afișată lângă numele tău când cineva îți datorează bani.
payment-method-copy = Copiază { $name }
payment-method-copied = Copiat.
payment-method-copy-failed = Nu s-a putut copia - selectează textul și copiază-l manual.

verify-email-checking = Se verifică adresa ta de e-mail…
verify-email-welcome = E-mail verificat - bine ai venit în Counted!
verify-email-back-to-login = Înapoi la conectare

### Project status

project-close = Închide
project-archive = Arhivează
project-reopen = Redeschide
project-unarchive = Dezarhivează
project-sheet-invite = Invită
project-sheet-recurring = Recurente
project-sheet-edit = Editează proiectul
project-sheet-close = Închide proiectul
project-sheet-archive = Arhivează proiectul
project-sheet-reopen = Redeschide proiectul
project-sheet-unarchive = Dezarhivează proiectul
project-sheet-leave = Părăsește proiectul

### Dates

date-long = { $day } { $month } { $year }

month-1 = ianuarie
month-2 = februarie
month-3 = martie
month-4 = aprilie
month-5 = mai
month-6 = iunie
month-7 = iulie
month-8 = august
month-9 = septembrie
month-10 = octombrie
month-11 = noiembrie
month-12 = decembrie

month-short-1 = ian
month-short-2 = feb
month-short-3 = mar
month-short-4 = apr
month-short-5 = mai
month-short-6 = iun
month-short-7 = iul
month-short-8 = aug
month-short-9 = sep
month-short-10 = oct
month-short-11 = noi
month-short-12 = dec

### Actions

add = Adaugă
create = Creează
creating = Se creează…
edit = Editează
leave = Părăsește
close = Închide
paste = Lipește
join = Alătură-te
import = Importă
importing = Se importă…
field-description = Descriere
field-date = Dată
date-today = Azi
date-yesterday = Ieri
field-optional = Opțional

### Projects

projects-filter-active = Active
projects-filter-all = Toate
projects-count-label = Proiecte
projects-empty = Niciun proiect
projects-empty-hint = Creează un proiect cu butonul de mai jos
projects-offline-banner = Date offline - reconectează-te pentru a actualiza.
demo-banner = Proiect demonstrativ - doar citire.
demo-start-own = Creează-ți propriul proiect
projects-no-local-data = Nu există date locale
projects-no-local-data-hint = Conectează-te pentru a-ți încărca proiectele pentru prima dată.
projects-add = Adaugă un proiect
projects-create = Creează un proiect
projects-join = Alătură-te unui proiect
projects-import-tricount = Importă din Tricount
project-actions = Acțiuni proiect

status-ongoing = În curs
status-closed = Închis
status-archived = Arhivat

nav-help = Ajutor
nav-privacy = Politica de confidențialitate
nav-terms = Termeni de utilizare
nav-legal = Mențiuni legale

leave-project-title = Părăsești proiectul?
leave-project-message = Vei pierde accesul de pe acest dispozitiv. Dacă nu mai rămâne niciun membru, proiectul și toate cheltuielile sale sunt șterse definitiv.

add-project-title = Proiect nou
add-project-name-label = Numele proiectului
add-project-name-placeholder = Călătoria mea, Colegii de apartament 2024…
add-project-participants = Participanți
add-project-participant-name = Numele participantului
add-project-participant-placeholder = Clark Kent
add-project-offline = Nu poți crea un proiect offline. Reconectează-te și încearcă din nou.
add-project-name-required = Proiectul are nevoie de un nume.

join-link-label = Link de partajare
join-link-hint = Linkul conține cheia de decriptare - copiază-l în întregime.
join-invalid-link = Acest link nu este valid. Lipește întregul link de partajare, inclusiv partea de după #.
join-wrong-project = Acest link este pentru alt proiect.

import-tricount-link-label = Link sau cheie Tricount
import-tricount-key-required = Introdu un link sau o cheie Tricount.
import-tricount-encryption-failed = Criptarea a eșuat.
import-tricount-unimportable = Nu s-a importat nimic: acest Tricount are membri cu cont Tricount sau sume care nu se potrivesc (înregistrări afectate: { $count }).

### Expenses

save = Salvează
saving = Se salvează…
adding = Se adaugă…
link-copied = Link copiat
missing-encryption-key = Lipsește cheia de criptare.
missing-encryption-key-title = Lipsește cheia de criptare
missing-encryption-key-hint = Linkul folosit nu conține cheia necesară pentru a decripta acest proiect. Folosește linkul complet partajat de cel care l-a creat.
project-locked-hint = Acest dispozitiv nu are cheia acestui proiect. Deschide linkul său de partajare pentru a-l debloca.
project-unlock = Deblochează
project-no-local-data-hint = Conectează-te pentru a încărca datele acestui proiect pentru prima dată.
project-gone-title = Acest proiect nu mai există
project-gone-hint = A fost șters când ultimul său membru l-a părăsit. Linkul de partajare nu mai funcționează, chiar dacă îl redeschizi.

expense-add = Adaugă o cheltuială
transfer-add = Adaugă un transfer
expense-edit-title = Editează cheltuiala
expense-category = Categorie
expense-category-auto = Auto · { $emoji }
expense-currency = Moneda sumei
amount-op-add = Plus
amount-op-subtract = Minus
amount-op-multiply = Înmulțit
amount-op-divide = Împărțit
amount-op-equals = Egal
amount-op-done = Gata
expense-rate = Curs de schimb (opțional)
expense-rate-hint = Lasă gol pentru a folosi cursul Comisiei Europene (InforEuro) pentru { $month }: 1 { $from } = { $rate } { $to }.
expense-rate-invalid = Introdu un curs de schimb mai mare decât 0.
expense-rate-unavailable = Niciun curs automat disponibil - introdu-l manual.
expense-delete-title = Șterge cheltuiala
expense-delete-message = „{ $name }” va fi ștearsă definitiv. Acest lucru nu poate fi anulat.
expense-inconsistent-amounts = Sumele nu se potrivesc
expenses-empty = Nicio cheltuială
expenses-empty-hint = Începe prin a adăuga cheltuieli cu butonul de mai jos
expenses-show-more = Arată mai multe ({ $count } rămase)

expense-type-expense = Cheltuială
expense-type-transfer = Transfer
expense-type-gain = Încasare
expense-paid-by = plătită de
expense-sent-by = trimis de
expense-contributed-by = contribuit de

expense-name-required = Numele este obligatoriu.
expense-amount-not-positive = Suma trebuie să fie mai mare decât 0.
expense-no-payer = Selectează cel puțin un plătitor.
expense-no-debtor = Selectează cel puțin o persoană care datorează.
expense-invalid-date = Această dată nu este validă.
expense-payers-mismatch = Plătitorii însumează { $sum }, ceea ce nu corespunde sumei cheltuielii ({ $total }).
expense-debtors-mismatch = Datornicii însumează { $sum }, ceea ce nu corespunde sumei cheltuielii ({ $total }).

participants-none = Nimeni
participants-everyone = Toată lumea ({ $count })
participants-some = { $count } din { $total }
participants-select-all = Selectează tot
participants-by-shares = Pe cote
split-amounts = Sume
participants-remaining = Rămân { $amount }
participants-over-by = { $amount } în plus
participants-who-paid = Cine a plătit?
participants-who-received = Cine a primit?
participants-who-transfers = Cine transferă?
participants-who-receives = Cine primește?
participants-for-whom = Pentru cine?

stats-total-expenses = Total cheltuieli
stats-my-expenses = Cheltuielile mele

tab-expenses = Cheltuieli
tab-balance = Balanță
tab-reimbursements = Regularizare
balance-gets-back = Primește înapoi
balance-owes = Datorează
balance-settled = Achitați
reimbursements-empty-title = Totul e regularizat!
reimbursements-empty-hint = Sugestiile de regularizare apar aici când socotelile nu sunt echilibrate
reimbursement-owes = { $debtor } îi datorează lui { $creditor }
reimbursement-record = Regularizează
reimbursement-pay-with = Plătește
reimbursement-pay-shared-by = Partajat de { $name } - verifică numele destinatarului afișat de aplicația ta înainte de a trimite.
reimbursement-pay-title = Plătește lui { $name }
reimbursements-mine-title = Tu datorezi
reimbursements-others-title = Alte rambursări
copy = Copiază

user-selection-title = Care participant ești?
user-selection-hint = Alege-ți numele din listă.
user-selection-required = Te rugăm să selectezi un participant.
identity-claimed = Asociat unui cont
identity-claimed-by = Contul lui { $name }
identity-taken-repick = Un alt cont a revendicat participantul pe care îl foloseai. Te rugăm să alegi altul.
participant-gone-repick = Participantul pe care îl foloseai a fost eliminat din acest proiect. Te rugăm să alegi altul.

edit-project-title = Editează proiectul
edit-project-new-badge = nou
edit-project-deferred-new-members = adăugarea de membri noi
edit-project-deferred-removals = eliminarea de membri
edit-project-offline-deferred = Offline: { $items } se va aplica la reconectare.

export-failed = Exportul a eșuat: { $reason }

history-expense-added = Cheltuială adăugată: { $name }
history-expense-edited = Cheltuială editată: { $name }
history-expense-deleted = Cheltuială ștearsă: { $name }
history-project-edited = Proiect editat: { $name }
history-name-changed = Nume: „{ $from }” → „{ $to }”
history-description-added = Descriere adăugată: „{ $value }”
history-description-removed = Descriere eliminată: „{ $value }”
history-description-changed = Descriere: „{ $from }” → „{ $to }”

### Sweep

field-amount = Sumă
expense-name-placeholder = Restaurant, cumpărături…
expense-actions = Acțiuni cheltuială
expense-your-share = Partea ta
expense-your-share-value = Partea ta: { $amount } { $currency }
expense-inconsistent-detail = Sumele nu se potrivesc: { $paid } plătit, { $owed } datorat, pentru o cheltuială de { $total }. Editează cheltuiala pentru a corecta.
missing-access-key = Lipsește cheia de acces. Deschide acest proiect prin linkul său de partajare.
filter-all = Toate
filter-my-payments = Plățile mele
filter-my-debts = Ce datorez
participants-shares-for = Cote pentru { $name }
participants-amount-for = Suma pentru { $name }
reimbursement-add = Adaugă o regularizare
project-forget = Elimină din lista mea
project-history-title = Istoric
history-kind-add = Adăugat
history-kind-delete = Șters
history-kind-edit = Editat
export = Exportă
export-json = Exportă JSON
export-csv = Exportă CSV
share-link = Partajează
copy-link-failed = Linkul nu a putut fi copiat
open-in-app = Deschide în aplicație
not-found-title = Pagină negăsită
not-found-back = Înapoi la proiecte

### Charts

charts-period = Perioadă
period-all = Tot
period-month = Lună
period-3months = 3 luni
period-year = An
period-custom = Personalizat
charts-tab-categories = Categorii
charts-tab-trends = Tendințe
charts-total-spent = Total cheltuit
charts-avg-per-person = Medie pe persoană
charts-expense-count =
    { $count ->
        [one] { $count } cheltuială
        [few] { $count } cheltuieli
       *[other] { $count } de cheltuieli
    }
charts-nothing-to-show = Nimic de afișat
charts-my-share-skipped =
    { $count ->
        [one] 1 proiect nu este luat în calcul — niciun participant ales sau datele sale nu s-au încărcat.
        [few] { $count } proiecte nu sunt luate în calcul — niciun participant ales sau datele lor nu s-au încărcat.
       *[other] { $count } de proiecte nu sunt luate în calcul — niciun participant ales sau datele lor nu s-au încărcat.
    }

### Categories

category-food = Mâncare
category-transport = Transport
category-accommodation = Cazare
category-leisure = Timp liber
category-shopping = Cumpărături
category-services = Servicii
category-parties-gifts = Petreceri și cadouri
category-other = Altele
charts-project = Proiect
charts-all-projects = Toate proiectele
charts-date-from = De la
charts-date-to = Până la
charts-total = Total
charts-tab-people = Persoane
charts-tab-projects = Proiecte
charts-scope = Cheltuielile cui
charts-scope-group = Grup
charts-scope-me = Eu
charts-currency = Monedă
charts-my-share = Partea mea
charts-share-of-total = { $pct }% din { $total }
charts-i-paid = Am plătit
charts-paid-more = Cu { $amount } mai mult decât partea mea
charts-paid-less = Cu { $amount } mai puțin decât partea mea
charts-paid-even = Exact partea mea
charts-part-title = Partea ta din fiecare categorie
charts-part-desc = Gri e ce a cheltuit grupul, culoare e ce ai consumat tu.
charts-breakdown-title = Defalcare pe categorii
charts-breakdown-desc = Atinge o felie sau un rând ca să vezi cheltuielile.
charts-of-total = { $amount } din { $total }
charts-show-all = Arată tot ({ $count })
charts-show-less = Arată mai puțin
charts-spend-title = Cheltuieli în timp
charts-group-by = Grupează după
bucket-day = Zi
bucket-week = Săptămână
bucket-month = Lună
charts-avg = med.
charts-cat-title-day = { $category }, zi de zi
charts-cat-title-week = { $category }, săptămână de săptămână
charts-cat-title-month = { $category }, lună de lună
charts-cat-desc = Alege o categorie ca s-o urmărești în timp.
charts-running-title = Total cumulat
charts-running-desc = Din { $date }.
charts-avg-per-day = { $amount } / zi în medie
charts-avg-per-week = { $amount } / săptămână în medie
charts-avg-per-month = { $amount } / lună în medie
charts-people-title = Cine a dus grupul
charts-people-desc = Cât a plătit fiecare, lângă cât a consumat.
charts-paid = Plătit
charts-fair-share = Parte corectă
charts-you = (tu)
charts-net-more = a plătit mai mult
charts-net-less = a plătit mai puțin
charts-balance-title = Soldul tău în timp
charts-balance-desc = Deasupra liniei grupul îți datorează. Dedesubt, tu datorezi grupului.
charts-owed = Ți se datorează
charts-owe = Datorezi
charts-projects-title = Partea ta, pe proiecte
charts-projects-desc = Totalurile se țin pe monede.
history-empty = Niciun eveniment
history-by = De { $name }
not-found-hint = Această pagină nu există sau a fost mutată.
payers-title-paid-by = Plătit de
payers-title-sender = Expeditor
payers-title-contributors = Contribuitori
debtors-title-debtors = Datorează
debtors-title-recipients = Destinatari
debtors-title-beneficiaries = Beneficiari

### Welcome

welcome-title = Socotelile tale nu privesc pe nimeni altcineva.
welcome-subtitle = Împarte cheltuielile cu prietenii.
welcome-note = Gratuit. Fără cont. Fără reclame.
welcome-link-title = Un link și toată lumea participă.
welcome-link-body = Nimeni nu trebuie să își creeze cont.
welcome-link-account = Un cont? Niciodată obligatoriu. Îți folosește ca să-ți regăsești proiectele pe alt dispozitiv, să-ți inviți prietenii din aplicație și să-ți partajezi detaliile de plată.
welcome-demo-project = Weekend la Lyon
welcome-private-title = Nimeni nu îți poate citi socotelile. Nici măcar noi.
welcome-private-body = Nume, sume, proiecte: totul este criptat pe dispozitivul tău. Doar tu deții cheia.
welcome-private-names = Nume
welcome-private-amounts = Sume
welcome-private-projects = Proiecte
welcome-scan-title = Fotografiază bonul.
welcome-scan-body = Suma, data și categoria se completează singure. Totul se petrece pe telefonul tău. Fotografia nu este păstrată.
welcome-eu-title = 100% european
welcome-no-ads = Fără reclame
welcome-no-trackers = Fără trackere
welcome-step = Pasul { $current } din { $total }
welcome-next = Înainte
welcome-skip = Omite
welcome-start = Începe
welcome-how-it-works = Cum funcționează, mai exact?

### Help

help-intro = O întrebare frecventă? Apasă pentru a vedea răspunsul.
help-create-project-q = Cum creez un proiect?
help-create-project-a = Din ecranul principal, apasă butonul + de jos. Dă un nume proiectului, alege moneda și gata.
help-add-participants-q = Cum adaug participanți?
help-add-participants-a = Deschide proiectul, apoi adaugă participanți din lista de membri. Fiecare participant poate plăti sau datora la o cheltuială.
help-share-project-q = Cum partajez un proiect?
help-share-project-a = Partajează URL-ul proiectului (cel din bara de adrese). Oricine are linkul poate vedea și edita proiectul.
help-add-expense-q = Cum adaug o cheltuială?
help-add-expense-a = Într-un proiect, apasă +, introdu suma, spune cine a plătit și între cine se împarte. Poți alege și o altă dată decât cea de azi.
help-types-q = Care e diferența dintre cheltuială, transfer și încasare?
help-types-expense = - o achiziție făcută de o persoană și împărțită între mai multe.
help-types-transfer = - o rambursare de la o persoană la alta, fără împărțire.
help-types-gain = - bani primiți (o rambursare, un cadou) de împărțit între mai multe persoane.
help-past-date-q = Pot data o cheltuială în trecut?
help-past-date-a = Da, câmpul de dată este liber. Momentul creării înregistrării este păstrat separat.
help-who-owes-q = Cum calculează Counted cine ce datorează?
help-who-owes-a = Counted calculează soldul net al fiecărui participant (ce a avansat minus ce datorează), apoi propune cea mai scurtă serie de transferuri care regularizează pe toată lumea.
help-minimal-transfers-q = De ce numărul de transferuri sugerate este minim?
help-minimal-transfers-a = Algoritmul împerechează mai întâi soldurile care se anulează exact, apoi parcurge restul de la cel mai mare creditor la cel mai mare datornic. Rezultatul: mai puține transferuri pentru a regulariza totul.
help-import-tricount-q = Cum import un proiect din Tricount?
help-import-tricount-a = Din ecranul principal, apasă butonul „+” de jos, apoi
help-import-tricount-b = Lipește linkul de partajare al Tricount-ului pe care vrei să-l imporți.
help-encryption-q = Datele mele sunt criptate?
help-encryption-a = Da. Counted combină două garanții:
help-encryption-e2ee-term = Criptare end-to-end
help-encryption-e2ee-def = - tot ce circulă între tine și server este criptat.
help-encryption-zero-term = Zero acces
help-encryption-zero-def = - criptezi datele înainte de a le trimite, iar serverul stochează doar text cifrat. Nu avem cum să-l citim.
help-encryption-see = Pentru detalii, vezi
help-forgot-password-q = Ce se întâmplă dacă îmi uit parola?
help-forgot-password-warning = Datele tale vor fi pierdute definitiv.
help-forgot-password-a = Cheia de criptare derivă din parola ta, deci nicio resetare nu este posibilă: nimeni - nici noi - nu poate decripta proiectele tale fără ea. Păstreaz-o în siguranță, ideal într-un manager de parole.
help-archive-delete-q = Cum arhivez sau șterg un proiect?
help-archive-delete-a = Din ecranul proiectului, deschide meniul și alege
help-archive-delete-b = pentru a-l ascunde păstrându-l. Un proiect este șters definitiv când ultimul său membru îl părăsește.
help-delete-account-q = Cum îmi șterg contul?
help-delete-account-a = Deschide Setări și folosește „Șterge-mi contul”. Este imediat și nu poate fi anulat.
help-contact = Altă întrebare? Scrie-ne la

# Receipt scanning (mobile only)
expense-scan = Scanează un bon
scan-in-progress = Se citește bonul…
scan-error-capture = Nu s-a putut face fotografia. Încearcă din nou sau introdu cheltuiala manual.
scan-error-unreadable = Nimic lizibil pe acest bon. Introdu cheltuiala manual.
scan-check-amount = Verifică totalul - nu era tipărit clar.
scan-take-photo = Fă o fotografie
scan-choose-photo = Alege o fotografie
expense-converted-from = Plătit { $amount } { $from } · 1 { $from } = { $rate } { $to }
project-currency = Monedă
project-currency-locked = Moneda este stabilită la crearea proiectului.
currency-search = Caută o monedă

update-required-title = Actualizare necesară
update-required-body = Această versiune de Counted este prea veche pentru a comunica cu serverul. Actualizeaz-o pentru a continua să folosești aplicația.
update-required-button = Actualizează

notifications-label = Notificări
notifications-title = Notificări
notifications-empty = Nimic nou
notifications-friend-request = Cerere de prietenie

friends-title = Prieteni
friends-anonymous-body = Prietenii sunt păstrați cu contul tău. Conectează-te pentru a adăuga persoane și a le invita în proiectele tale fără a partaja un link.
friends-add-title = Adaugă un prieten
friends-add-hint = Îți va vedea cererea la următoarea conectare. Niciunul dintre voi nu află dacă celălalt are cont până când cererea este acceptată.
friends-add-button = Adaugă
friends-add-from-project = Adaugă ca prieten
friends-request-sent = Cerere trimisă
friends-no-account-key = Conectează-te din nou pentru a-ți gestiona prietenii pe acest dispozitiv.
friends-incoming-title = Cereri
friends-accept = Acceptă
friends-decline = Refuză
friends-list-title = Prietenii mei
friends-list-empty = Niciun prieten încă. Adaugă pe cineva prin e-mail mai sus sau dintr-un proiect partajat.
friends-remove = Elimină
friends-remove-confirm-title = Elimină prietenul
friends-remove-confirm-message = { $email } nu va mai fi printre prietenii tăi, iar tu nu vei mai fi printre ai lui. Oricare dintre voi poate trimite o nouă cerere mai târziu.
friends-no-key = Nu e gata încă
friends-fingerprint = Cod de siguranță
friends-fingerprint-hint = Doi prieteni care își citesc același cod de siguranță știu că nimeni nu stă între ei - nici măcar serverul nostru.
friends-outgoing-title = Trimise
friends-outgoing-hint = În așteptarea unui răspuns. Îi vei vedea printre prieteni odată ce acceptă.
friends-withdraw = Anulează
invite-friends-title = Invită prieteni
invite-friends-hint = Cheia proiectului este criptată pentru fiecare prieten pe acest dispozitiv. Serverul nu o vede niciodată.
invite-friends-empty = Niciun prieten de invitat încă.
invite-friends-button = Invită
invite-sent = { $count ->
    [one] Invitație trimisă
    [few] { $count } invitații trimise
   *[other] { $count } de invitații trimise
}
invitation-badge = Invitație
invitation-to = Alătură-te la „{ $name }”
invitation-to-unnamed = Alătură-te unui proiect
invitation-unreadable = Această invitație nu poate fi deschisă pe acest dispozitiv
invitation-from = De la { $email }
invitation-accept = Alătură-te
invitation-decline = Refuză

# Participants in the create and edit modals, and the "who are you?" picker - see
# docs/plans/friends.md §11.
participants-you-label = Numele tău în acest proiect
participants-you-badge = Tu
participants-others = Alți participanți
participants-empty = Adaugă participanți mai jos.
participants-duplicate = „{ $name }” este deja în listă.
participants-input-label = Adaugă un prieten sau scrie un nume
participants-input-placeholder = Prieten sau orice nume
participants-suggest-friend = Prieten · intră ca „{ $name }”, primește o invitație
participants-suggest-not-ready = Prieten · încă nu e pregătit
participants-suggest-guest = Adaugă „{ $text }”
participants-suggest-guest-sub = Invitat
participants-friends = Prietenii tăi
participants-all-friends = Toți prietenii
participants-login-hint = Conectează-te ca să adaugi oameni direct din lista ta de prieteni.
participants-invite-badge = De invitat
participants-guest-badge = Invitat
participants-rename = Redenumește: { $name }
participants-remove = Elimină: { $name }
participants-rename-label = Nume nou
participants-rename-save = Salvează numele
participants-rename-hint = Numele pe care îl văd toți în acest proiect. Invitația merge în continuare la { $email }.
participants-invited-badge = Invitat
participants-invited-sub = { $email } · încă neacceptată
participants-invited-pending = Invitație încă neacceptată
participants-unlinked = Nelegat de un cont
add-project-create-invite = Creează și invită: { $count }
edit-project-save-invite = Salvează și invită: { $count }
edit-project-you-are = Pe acest dispozitiv ești { $name }
edit-project-no-identity = Încă nu ai ales cine ești
edit-project-switch = Schimbă
edit-project-choose = Alege
invite-failed = Aceste invitații nu au putut fi trimise: { $emails }
invite-again = Invită din nou
friend-picker-title = Adaugă prieteni
user-selection-invited-hint = { $email } te-a invitat în „{ $project }”.
user-selection-suggested = Sugerat
user-selection-suggested-sub = { $email } te-a adăugat cu acest nume
user-selection-confirm-as = Eu sunt { $name }
user-selection-missing = Numele tău nu e aici? Roagă un participant să te adauge din setările proiectului.

## Cheltuieli recurente

repeat-label = Repetă
repeat-none = Nu se repetă
repeat-weekly = În fiecare săptămână
repeat-biweekly = La fiecare 2 săptămâni
repeat-monthly = În fiecare lună
repeat-quarterly = La fiecare 3 luni
repeat-yearly = În fiecare an
repeat-every-weeks = La fiecare { $count } săpt.
repeat-every-months = La fiecare { $count } luni
repeat-every-years = La fiecare { $count } ani
repeat-custom = Personalizat…
repeat-every = La fiecare
repeat-unit-weeks = săpt.
repeat-unit-months = luni
repeat-unit-years = ani
repeat-on-weekday = ziua: { $weekday }
repeat-on-day = pe { $day }
repeat-on-day-month = pe { $day } { $month }
repeat-month-end = În lunile mai scurte cade în ultima zi.
repeat-ends = Se termină
repeat-ends-never = Niciodată
repeat-ends-on = La o dată
repeat-ends-after = După
repeat-fewer = Mai puține
repeat-more = Mai multe
repeat-last-on = ultima pe { $date }
repeat-variable = Suma se schimbă de fiecare dată
repeat-variable-hint = Fiecare e adăugată cu ultima sumă și marcată „de confirmat”.
repeat-done = Gata
repeat-no-end = Fără sfârșit
repeat-until = Până pe { $date }
repeat-occurrences = Număr de repetări: { $count }
repeat-offline = Necesită conexiune. Cheltuiala în sine poate fi adăugată oricum.
repeat-foreign = Se repetă ca { $amount } { $currency }, convertit o dată la cursul de azi. Vei fi avertizat dacă cursul se schimbă cu peste 5 %.
repeat-backfill = Începe în trecut. Cheltuieli adăugate acum: { $count }.
add-and-repeat = Adaugă și repetă
weekday-1 = luni
weekday-2 = marți
weekday-3 = miercuri
weekday-4 = joi
weekday-5 = vineri
weekday-6 = sâmbătă
weekday-7 = duminică
recurring-title = Cheltuieli recurente
recurring-strip = Cheltuieli recurente: { $count }
recurring-next = Următoarea: { $name }, { $date }
recurring-to-confirm = De confirmat: { $count }
recurring-per-month = Pe lună, aprox.
recurring-your-share = Partea ta
recurring-active = Active
recurring-paused = Întrerupte
recurring-finished = Încheiate
recurring-paid-by = plătită de { $name }
recurring-next-on = Următoarea pe { $date }
recurring-progress = { $done } din { $total }
recurring-rate-badge = Curs modificat cu { $percent } %
recurring-empty = Încă nu se repetă nimic. Alege „Repetă” când adaugi o cheltuială: chirie, abonamente, facturi.
recurring-next-ones = Următoarele
recurring-added-so-far = Adăugate până acum
recurring-set-up-by = Creată de
recurring-pause = Întrerupe
recurring-resume = Reia
recurring-stop = Oprește repetarea
recurring-stop-title = Oprești „{ $name }”?
recurring-stop-message = Nu se va mai repeta. Cheltuielile deja adăugate rămân.
recurring-resume-title = Reiei „{ $name }”?
recurring-resume-message = Următoarea pe { $date }. Datele ratate în timpul pauzei nu se adaugă.
recurring-edit-title = Editează cheltuiala recurentă
recurring-edit-banner = Modificările se aplică de pe { $date }. Cheltuielile deja adăugate rămân așa cum sunt.
recurring-next-on-label = Următoarea pe
recurring-next-too-early = Data următoare trebuie să fie după ultima cheltuială deja adăugată.
recurring-use-stop = Pentru a o încheia, folosește „Oprește repetarea” din cheltuiala recurentă.
recurring-drift = Cursul { $currency } s-a schimbat cu { $percent } % de la creare. Fiecare e încă adăugată ca { $amount } { $project_currency } (1 { $currency } = { $rate }). La cursul de azi ar fi { $today_amount } { $project_currency }.
recurring-use-rate = Folosește cursul de azi
recurring-keep = Păstrează { $amount } { $currency }
recurring-added = Cheltuieli recurente adăugate: { $count }
recurring-blocks-removal = { $name } nu poate fi eliminat încă: face parte din { $rules }. Scoate-l pe { $name } din ele sau oprește-le, apoi salvează din nou.
history-recurring-added = Adăugată automat: { $name } ({ $date })
history-recurring-created = Cheltuială recurentă creată: { $name }
history-recurring-edited = Cheltuială recurentă editată: { $name }
history-recurring-paused = Cheltuială recurentă pusă pe pauză: { $name }
history-recurring-resumed = Cheltuială recurentă reluată: { $name }
history-recurring-stopped = Cheltuială recurentă oprită: { $name }
occurrence-recurring = Cheltuială recurentă
occurrence-auto = Adăugată automat de o cheltuială recurentă.
occurrence-auto-next = Adăugată automat de o cheltuială recurentă. Următoarea pe { $date }.
occurrence-auto-stopped = Adăugată automat de o cheltuială recurentă oprită între timp.
occurrence-manage = Gestionează
estimate-badge = De confirmat
estimate-title = Sumă de confirmat.
estimate-body = Adăugată cu suma anterioară. Introdu suma reală când o afli.
estimate-confirm = Confirmă suma
apply-to = Aplică pentru
apply-this-only = Doar această cheltuială
apply-and-next = Aceasta și următoarele
apply-and-next-hint = Cheltuiala recurentă se schimbă de pe { $date }
apply-rule-failed = Cheltuiala a fost salvată, dar cheltuiala recurentă nu a fost modificată.
occurrence-delete-message = „{ $name }” din { $date } va fi ștearsă definitiv. Repetarea continuă, iar această dată nu va reveni.
occurrence-delete-one = Șterge doar aceasta
occurrence-delete-stop = Șterge și oprește repetarea
error-recurring-clock = Ceasul acestui dispozitiv o ia înainte. Verifică data și ora.
error-recurring-not-found = Această cheltuială recurentă nu mai există.
error-recurring-stale = Cineva a modificat între timp această cheltuială recurentă. A fost reîncărcată: verific-o și salvează din nou.
error-too-many-recurring = Acest proiect are deja 50 de cheltuieli recurente. Oprește una de care nu mai ai nevoie ca să adaugi alta.
error-user-in-recurring = Acest participant face parte dintr-o cheltuială recurentă. Scoate-l mai întâi din ea sau oprește-o.
