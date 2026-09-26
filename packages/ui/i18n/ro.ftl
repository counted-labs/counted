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
account-delete-warning = Imediat și definitiv, fără coș de gunoi. Cheltuielile introduse de tine într-un proiect partajat rămân vizibile celorlalți membri - fac parte din socotelile lor.
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
settings-payment-methods-share-warning = O metodă partajată este vizibilă tuturor membrilor proiectelor în care ți-ai ales numele - oricui deține unul din acele linkuri.
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
add-project-remove-participant = Elimină participantul
add-project-me-badge = Eu
add-project-thats-me = Eu sunt!
add-project-offline = Nu poți crea un proiect offline. Reconectează-te și încearcă din nou.
add-project-name-required = Proiectul are nevoie de un nume.
add-project-need-two-participants = Adaugă cel puțin 2 participanți.
add-project-pick-yourself = Spune-ne care participant ești.

join-link-label = Link de partajare
join-link-hint = Linkul conține cheia de decriptare - copiază-l în întregime.
join-invalid-link = Acest link nu este valid. Lipește întregul link de partajare, inclusiv partea de după #.
join-wrong-project = Acest link este pentru alt proiect.

import-tricount-link-label = Link sau cheie Tricount
import-tricount-key-required = Introdu un link sau o cheie Tricount.
import-tricount-encryption-failed = Criptarea a eșuat.

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
edit-project-deferred-me = selecția „Eu sunt”
edit-project-offline-deferred = Offline: { $items } se va aplica la reconectare.

export-saved = Fișier salvat:
    { $path }
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
charts-tab-per-person = Pe persoană
charts-tab-trends = Tendințe
charts-by-category = Defalcare pe categorii
charts-per-person = Cheltuieli pe persoană
charts-categories-by-month = Categorii pe luni
charts-total-spent = Total cheltuit
charts-avg-per-person = Medie pe persoană
charts-expense-count =
    { $count ->
        [one] { $count } cheltuială
        [few] { $count } cheltuieli
       *[other] { $count } de cheltuieli
    }
charts-clear-category-filter = Șterge filtrul de categorie
charts-no-expenses = Nicio cheltuială.
charts-pick-a-project = Alege un proiect pentru a vedea cheltuielile pe persoană.
charts-nothing-to-show = Nimic de afișat
charts-my-share-note = Aceste cifre sunt partea ta din fiecare cheltuială.
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
charts-person = Persoană
charts-project = Proiect
charts-all-projects = Toate proiectele
charts-whole-project = Întregul proiect
charts-date-from = De la
charts-date-to = Până la
charts-total = Total
charts-payments-per-person-by-month = Plăți pe persoană pe luni
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
welcome-e2ee-title = Totul criptat
welcome-e2ee-body = Nume, sume, proiecte: totul este criptat pe dispozitivul tău. Doar tu deții cheia. Nimeni nu îți poate citi socotelile. Nici măcar noi.
welcome-e2ee-note = Ilizibil, chiar și pentru noi (zero acces la server)
welcome-eu-title = 100% european
welcome-eu-body = Servere în Germania, e-mailuri trimise din Franța. Datele tale nu părăsesc niciodată Uniunea Europeană.
welcome-noads-title = Fără reclame. Fără trackere.
welcome-noads-body = Nu colectăm nimic și nu îți vindem datele. Nu acesta e modelul nostru.
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
project-currency-hint = Toate sumele sunt afișate în această monedă. Nu poate fi schimbată ulterior.
project-currency-locked = Moneda este stabilită la crearea proiectului.

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
