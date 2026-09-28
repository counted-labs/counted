# Italiano. Completo tranne i testi legali (legal-, terms-, privacy-), che esistono solo in inglese
# e francese e ricadono su en.ftl messaggio per messaggio.

### Common

loading = Caricamento…
cancel = Annulla
retry = Riprova
delete = Elimina
back = Indietro
language = Lingua

### Navigation

nav-main = Navigazione principale
nav-projects = Progetti
nav-charts = Statistiche
nav-settings = Impostazioni

### Connectivity

offline-banner = Offline
offline-pending =
    { $count ->
        [one] { $count } in sospeso
       *[other] { $count } in sospeso
    }

sync-conflict-edit = Conflitto: modifica di «{ $name }» non riuscita (elemento eliminato). Ignorata.
sync-conflict-delete = Conflitto: eliminazione di «{ $name }» non riuscita (elemento eliminato). Ignorata.
sync-conflict-other = Conflitto: operazione su «{ $name }» non riuscita (elemento eliminato). Ignorata.
sync-error = Errore di sincronizzazione: { $reason }

### Errors

error-network = Impossibile contattare il server. Controlla la connessione a internet.
error-generic = Si è verificato un errore. Riprova.

error-invalid-email = Questo indirizzo e-mail non è valido.
error-invalid-password = Questa password non è valida.
error-password-too-short = La password deve contenere almeno 8 caratteri.
error-client-outdated = Questa versione dell'app non è aggiornata. Aggiornala per accedere.
error-invalid-link = Questo link non è valido.
error-batch-too-large = Troppi elementi in una volta.
error-payers-required = Seleziona almeno una persona che ha pagato.
error-debtors-required = Seleziona almeno una persona che deve.
error-duplicate-participant = Un partecipante compare due volte sullo stesso lato.
error-participant-not-in-project = Questo partecipante non fa parte del progetto.
error-too-many-participants = Troppi partecipanti per una sola spesa.
error-invalid-credentials = E-mail o password non corretti.
error-unauthenticated = Accedi per farlo.
error-email-not-verified = Il tuo indirizzo e-mail non è ancora verificato.
error-claim-proof-invalid = Questo dispositivo non possiede la chiave del progetto e non può quindi rivendicare un partecipante. Riapri il link di condivisione.
error-project-not-found = Questo progetto non esiste più.
error-expense-not-found = Questa spesa non esiste più.
error-user-not-found = Questo partecipante non esiste più.
error-tricount-not-found = Tricount non trovato, o la sua API ha restituito un errore.
error-too-many-members = Questo progetto ha raggiunto il limite di partecipanti.
error-user-has-payments = Questo partecipante ha spese nel progetto e non può essere rimosso.
error-resend-cooldown = Attendi 60 secondi prima di richiedere un'altra e-mail.

### Auth

field-email = E-mail
field-email-placeholder = tu@esempio.it
field-password = Password
field-name = Nome
field-name-placeholder = Mario Rossi

login-title = Accedi
login-submit = Accedi
login-submitting = Accesso in corso…
login-password-placeholder = La tua password
login-no-account = Non hai ancora un account?
login-unverified = Il tuo indirizzo e-mail non è ancora verificato. Controlla la casella o richiedi un nuovo link.
login-resend = Invia di nuovo il link di verifica
login-resending = Invio…
login-resend-sent = E-mail inviata: controlla la tua casella.

register-submit = Crea un account
register-submitting = Creazione…
register-have-account = Hai già un account?
register-password-placeholder = Almeno 8 caratteri
register-password-warning = Annota la password. Se la dimentichi, non sarà possibile recuperare l'account.
register-check-email-title = Controlla la tua e-mail
register-email-sent = E-mail inviata
register-email-sent-hint = Fai clic sul link ricevuto per attivare il tuo account.
register-not-received-prefix = Non è arrivata? Controlla lo spam oppure
register-sign-in-link = accedi
register-not-received-suffix = per inviare di nuovo il link.
register-terms-prefix = Creando un account accetti le
register-terms-link = condizioni d'uso
register-terms-and = e la nostra
register-privacy-link = informativa sulla privacy

settings-title = Impostazioni
settings-preferences = Preferenze
settings-preferences-local = Salvate su questo dispositivo.
settings-preferences-synced = Sincronizzate con il tuo account, cifrate.
settings-about = Informazioni
settings-anonymous-title = Non hai effettuato l'accesso
settings-upsell-title = I tuoi progetti, su ogni dispositivo
settings-upsell-free = Gratis
settings-upsell-body = Counted funziona senza account. Con un account gratuito, i tuoi progetti e le tue preferenze ti seguono su telefono, computer e web: sempre cifrati, sempre illeggibili per noi.
settings-locked-badge = Account
settings-locked-friends = Crea un account per aggiungere amici e invitarli in un progetto dall'app, senza link da far girare.
settings-locked-payment-methods = Salva una volta il tuo IBAN o la tua app di pagamento e condividili con i progetti che scegli. Chi ti deve dei soldi li vede accanto al tuo nome.
settings-friends-hint = Aggiungi amici e invitali nei tuoi progetti senza condividere alcun link.

account-member-since = Membro dal
account-logout = Esci
account-logging-out = Disconnessione…
account-delete-title = Elimina il mio account
account-delete-warning = Eliminazione immediata e definitiva, senza cestino. Le spese che hai inserito in un progetto condiviso restano visibili agli altri partecipanti: fanno parte dei loro conti.
account-delete-confirm-title = Elimina l'account
account-delete-confirm-message = Il tuo account, le tue sessioni e l'elenco dei tuoi progetti saranno eliminati definitivamente. Senza la tua password, i dati cifrati di un progetto condiviso diventano illeggibili per te: l'operazione è irreversibile.

settings-payment-methods = Dati di pagamento
settings-payment-methods-hint = Come preferisci essere rimborsato. Cifrati con il tuo account.
payment-method-kind = Metodo
payment-method-kind-other = Altro
payment-method-label = Nome
payment-method-label-placeholder = Conto principale
payment-method-value = Dati
payment-method-value-placeholder = IBAN, numero di telefono, nome utente…
payment-method-add = Aggiungi
payment-method-remove = Rimuovi { $name }
payment-method-empty = Non hai ancora aggiunto dati di pagamento.
payment-method-deleted = Metodo di pagamento eliminato.
payment-method-value-required = Compila i dati di ogni metodo di pagamento, oppure rimuovilo.
payment-method-label-required = Dai un nome al tuo metodo personalizzato.
payment-method-too-long = È troppo lungo: accorcialo.
payment-method-invalid-characters = Rimuovi le interruzioni di riga o i caratteri invisibili.
payment-method-limit = Puoi salvare fino a { $max } metodi di pagamento.
payment-methods-saved = Dati di pagamento salvati.
payment-methods-offline = Devi essere online per salvare i tuoi dati di pagamento.
payment-methods-stale = I tuoi dati di pagamento sono stati modificati su un altro dispositivo. Sono stati ricaricati — riprova.
payment-methods-key-missing = Accedi di nuovo per gestire i tuoi dati di pagamento.

verify-email-checking = Verifica della tua e-mail…
verify-email-welcome = E-mail verificata - benvenuto su Counted!
verify-email-back-to-login = Torna all'accesso

### Project status

project-close = Chiudi
project-archive = Archivia
project-reopen = Riapri
project-unarchive = Rimuovi dall’archivio

### Dates

date-long = { $day } { $month } { $year }

month-1 = gennaio
month-2 = febbraio
month-3 = marzo
month-4 = aprile
month-5 = maggio
month-6 = giugno
month-7 = luglio
month-8 = agosto
month-9 = settembre
month-10 = ottobre
month-11 = novembre
month-12 = dicembre

month-short-1 = Gen
month-short-2 = Feb
month-short-3 = Mar
month-short-4 = Apr
month-short-5 = Mag
month-short-6 = Giu
month-short-7 = Lug
month-short-8 = Ago
month-short-9 = Set
month-short-10 = Ott
month-short-11 = Nov
month-short-12 = Dic

### Actions

add = Aggiungi
create = Crea
creating = Creazione…
edit = Modifica
leave = Esci
close = Chiudi
paste = Incolla
join = Partecipa
import = Importa
importing = Importazione…
field-description = Descrizione
field-optional = Facoltativo

### Projects

projects-filter-active = Attivi
projects-filter-all = Tutti
projects-count-label = Progetti
projects-empty = Nessun progetto
projects-empty-hint = Crea un progetto con il pulsante qui sotto
projects-offline-banner = Dati offline - riconnettiti per aggiornare.
projects-no-local-data = Nessun dato locale
projects-no-local-data-hint = Accedi per caricare i tuoi progetti per la prima volta.
projects-add = Aggiungi un progetto
projects-create = Crea un progetto
projects-join = Partecipa a un progetto
projects-import-tricount = Importa da Tricount
project-actions = Azioni sul progetto

status-ongoing = In corso
status-closed = Chiuso
status-archived = Archiviato

nav-help = Aiuto
nav-privacy = Informativa sulla privacy
nav-terms = Condizioni d'uso
nav-legal = Note legali

leave-project-title = Uscire dal progetto?
leave-project-message = Perderai l'accesso da questo dispositivo. Se non resta nessun partecipante, il progetto e tutte le sue spese vengono eliminati definitivamente.

add-project-title = Nuovo progetto
add-project-name-label = Nome del progetto
add-project-name-placeholder = Il mio viaggio, Coinquilini 2024…
add-project-participants = Partecipanti
add-project-participant-name = Nome del partecipante
add-project-participant-placeholder = Clark Kent
add-project-remove-participant = Rimuovi partecipante
add-project-me-badge = Io
add-project-thats-me = Sono io!
add-project-offline = Non puoi creare un progetto offline. Riconnettiti e riprova.
add-project-name-required = Il progetto ha bisogno di un nome.
add-project-need-two-participants = Aggiungi almeno 2 partecipanti.
add-project-pick-yourself = Indica quale partecipante sei.

join-link-label = Link di condivisione
join-link-hint = Il link contiene la chiave di decifratura: copialo per intero.
join-invalid-link = Questo link non è valido. Incolla il link di condivisione completo, compresa la parte dopo il #.
join-wrong-project = Questo link appartiene a un altro progetto.

import-tricount-link-label = Link o chiave Tricount
import-tricount-key-required = Inserisci un link o una chiave Tricount.
import-tricount-encryption-failed = Errore di cifratura.

### Expenses

save = Salva
saving = Salvataggio…
adding = Aggiunta…
link-copied = Link copiato
missing-encryption-key = Chiave di cifratura mancante.
missing-encryption-key-title = Chiave di cifratura mancante
missing-encryption-key-hint = Il link che hai usato non contiene la chiave necessaria per decifrare questo progetto. Usa il link completo condiviso da chi lo ha creato.
project-locked-hint = Questo dispositivo non ha la chiave di questo progetto. Apri il link di condivisione per sbloccarlo.
project-unlock = Sblocca
project-no-local-data-hint = Accedi per caricare i dati di questo progetto per la prima volta.
project-gone-hint = È stato eliminato quando l'ultimo partecipante è uscito. Il link di condivisione non funziona più, anche se lo riapri.

expense-add = Aggiungi una spesa
transfer-add = Aggiungi un trasferimento
expense-edit-title = Modifica la spesa
expense-category = Categoria
expense-delete-title = Elimina la spesa
expense-delete-message = «{ $name }» sarà eliminata definitivamente. L'operazione è irreversibile.
expense-inconsistent-amounts = Importi incoerenti
expenses-empty = Nessuna spesa
expenses-empty-hint = Inizia aggiungendo una spesa con il pulsante qui sotto

expense-type-expense = Spesa
expense-type-transfer = Trasferimento
expense-type-gain = Entrata
expense-paid-by = pagata da
expense-sent-by = inviato da
expense-contributed-by = contribuito da

expense-name-required = Il nome è obbligatorio.
expense-amount-not-positive = L'importo deve essere maggiore di 0.
expense-no-payer = Seleziona almeno una persona che ha pagato.
expense-no-debtor = Seleziona almeno una persona che deve.
expense-invalid-date = Questa data non è valida.
expense-payers-mismatch = Il totale di chi ha pagato è { $sum }, non corrisponde all'importo della spesa ({ $total }).
expense-debtors-mismatch = Il totale di chi deve è { $sum }, non corrisponde all'importo della spesa ({ $total }).

participants-none = Nessuno
participants-everyone = Tutti ({ $count })
participants-some = { $count } su { $total }
participants-select-all = Seleziona tutti
participants-remaining = Mancano { $amount }
participants-over-by = Eccede di { $amount }
participants-who-paid = Chi ha pagato?
participants-who-received = Chi ha ricevuto?
participants-who-transfers = Chi trasferisce?
participants-who-receives = Chi riceve?
participants-for-whom = Per chi?

stats-total-expenses = Totale spese
stats-my-expenses = Le mie spese

tab-expenses = Spese
tab-balance = Bilancio
tab-reimbursements = Rimborsi
reimbursements-empty-hint = Qui compariranno i rimborsi suggeriti quando i conti non tornano
reimbursement-owes = { $debtor } deve a { $creditor }

user-selection-title = Quale partecipante sei?
user-selection-hint = Scegli il tuo nome dall'elenco.
user-selection-required = Seleziona un partecipante.

edit-project-deferred-new-members = l'aggiunta di nuovi partecipanti
edit-project-deferred-removals = la rimozione di partecipanti
edit-project-deferred-me = la scelta «Sono io»
edit-project-offline-deferred = Offline: { $items } verrà applicato alla riconnessione.

export-saved = File salvato:
    { $path }
export-failed = Esportazione non riuscita: { $reason }

history-expense-added = Spesa aggiunta: { $name }
history-expense-edited = Spesa modificata: { $name }
history-expense-deleted = Spesa eliminata: { $name }
history-project-edited = Progetto modificato: { $name }
history-name-changed = Nome: «{ $from }» → «{ $to }»
history-description-added = Descrizione aggiunta: «{ $value }»
history-description-removed = Descrizione rimossa: «{ $value }»
history-description-changed = Descrizione: «{ $from }» → «{ $to }»

### Sweep

field-amount = Importo
expense-name-placeholder = Ristorante, spesa…
expense-actions = Azioni sulla spesa
expense-your-share = La tua quota
expense-your-share-value = La tua quota: { $amount } { $currency }
expense-inconsistent-detail = Importi incoerenti: { $paid } pagato, { $owed } dovuto, per una spesa di { $total }. Modifica la spesa per correggerla.
missing-access-key = Chiave di accesso mancante. Apri questo progetto dal suo link di condivisione.
filter-all = Tutto
filter-my-payments = I miei pagamenti
filter-my-debts = Quanto devo
participants-shares-for = Quote di { $name }
participants-amount-for = Importo per { $name }
reimbursement-add = Aggiungi un rimborso
project-forget = Rimuovi dal mio elenco
project-history-title = Cronologia
history-kind-add = Aggiunta
history-kind-delete = Eliminazione
history-kind-edit = Modifica
export = Esporta
export-json = Esporta JSON
export-csv = Esporta CSV
share-link = Condividi
copy-link-failed = Impossibile copiare il link
open-in-app = Apri nell'app
not-found-title = Pagina non trovata
not-found-back = Torna ai progetti

### Charts

charts-period = Periodo
period-all = Tutto
period-month = Mese
period-3months = 3 mesi
period-year = Anno
period-custom = Person.
charts-tab-categories = Categorie
charts-tab-trends = Andamento
charts-total-spent = Totale speso
charts-avg-per-person = Media per persona
charts-expense-count =
    { $count ->
        [one] { $count } spesa
       *[other] { $count } spese
    }
charts-nothing-to-show = Niente da mostrare
charts-my-share-note = Questi importi sono la tua parte di ogni spesa.
charts-my-share-skipped =
    { $count ->
        [one] 1 progetto non è conteggiato — nessun partecipante scelto, o i dati non sono stati caricati.
       *[other] { $count } progetti non sono conteggiati — nessun partecipante scelto, o i dati non sono stati caricati.
    }

### Categories

category-food = Cibo
category-transport = Trasporti
category-accommodation = Alloggio
category-leisure = Tempo libero
category-shopping = Shopping
category-services = Servizi
category-parties-gifts = Feste e regali
category-other = Altro
history-empty = Nessun evento
not-found-hint = Questa pagina non esiste o è stata spostata.
payers-title-paid-by = Pagato da
payers-title-sender = Mittente
payers-title-contributors = Contributori
debtors-title-debtors = Deve
debtors-title-recipients = Destinatari
debtors-title-beneficiaries = Beneficiari

### Welcome

welcome-title = I tuoi conti riguardano solo te.
welcome-subtitle = Dividi le spese tra amici.
welcome-e2ee-title = Dati interamente cifrati
welcome-e2ee-body = Nomi, importi, progetti: tutto viene cifrato sul tuo dispositivo. Solo tu hai la chiave. Nessuno può leggere i tuoi conti. Nemmeno noi.
welcome-e2ee-note = Indecifrabile, anche per noi (zero accesso dal server)
welcome-eu-title = 100 % europeo
welcome-eu-body = Server in Germania, email inviate dalla Francia. I tuoi dati non lasciano mai l'Unione europea.
welcome-noads-title = Zero pubblicità. Zero tracciatori.
welcome-noads-body = Non raccogliamo nulla e non vendiamo i tuoi dati. Non è il nostro modello.
welcome-start = Inizia
welcome-how-it-works = Come funziona esattamente?

### Help

help-intro = Una domanda frequente? Tocca per aprire la risposta.
help-create-project-q = Come creo un progetto?
help-create-project-a = Dalla schermata iniziale tocca il pulsante + in basso. Dai un nome al progetto, scegli la valuta e sei pronto.
help-add-participants-q = Come aggiungo partecipanti?
help-add-participants-a = Apri il progetto e aggiungi i partecipanti dall'elenco dei membri. Ogni partecipante può pagare o dovere in una spesa.
help-share-project-q = Come condivido un progetto?
help-share-project-a = Condividi l'URL del progetto (quello nella barra degli indirizzi). Chiunque abbia il link può vedere e modificare il progetto.
help-add-expense-q = Come aggiungo una spesa?
help-add-expense-a = Dentro un progetto tocca +, inserisci l'importo e indica chi ha pagato e tra chi dividere. Puoi anche scegliere una data diversa da oggi.
help-types-q = Che differenza c'è tra spesa, trasferimento ed entrata?
help-types-expense = - un acquisto fatto da una persona e diviso tra più persone.
help-types-transfer = - un rimborso da una persona a un'altra, senza divisione.
help-types-gain = - una somma ricevuta (rimborso, regalo) da dividere tra più persone.
help-past-date-q = Posso datare una spesa nel passato?
help-past-date-a = Sì, il campo data è libero. La data di creazione del record è conservata a parte.
help-who-owes-q = Come fa Counted a calcolare chi deve cosa?
help-who-owes-a = Counted calcola il saldo netto di ogni partecipante (quanto ha anticipato meno quanto deve), poi propone la serie di bonifici più breve per chiudere tutti i conti.
help-minimal-transfers-q = Perché il numero di bonifici suggeriti è minimo?
help-minimal-transfers-a = L'algoritmo abbina prima i saldi che si compensano esattamente, poi tratta gli altri dal creditore più grande al debitore più grande. Risultato: meno trasferimenti per sistemare tutto.
help-import-tricount-q = Come importo un progetto da Tricount?
help-import-tricount-a = Dalla schermata iniziale tocca il pulsante «+» in basso, poi
help-import-tricount-b = Incolla il link di condivisione del Tricount da importare.
help-encryption-q = I miei dati sono cifrati?
help-encryption-a = Sì. Counted unisce due garanzie:
help-encryption-e2ee-term = Cifratura end-to-end
help-encryption-e2ee-def = - tutto ciò che passa tra te e il server viaggia cifrato.
help-encryption-zero-term = Zero accesso
help-encryption-zero-def = - cifri i dati prima di inviarli e il server conserva solo testo cifrato. Non abbiamo modo di leggerlo.
help-encryption-see = Per i dettagli vedi l'
help-forgot-password-q = Cosa succede se dimentico la password?
help-forgot-password-warning = I tuoi dati andranno persi definitivamente.
help-forgot-password-a = La chiave di cifratura deriva dalla tua password, quindi non è possibile alcun reset: nessuno - noi compresi - può decifrare i tuoi progetti senza di essa. Conservala bene, meglio in un gestore di password.
help-archive-delete-q = Come archivio o elimino un progetto?
help-archive-delete-a = Dalla schermata del progetto apri il menu e scegli
help-archive-delete-b = per nasconderlo conservandolo. Un progetto viene eliminato definitivamente solo quando l'ultimo partecipante lo lascia.
help-delete-account-q = Come elimino il mio account?
help-delete-account-a = Apri le Impostazioni e usa «Elimina il mio account». È immediato e irreversibile.
help-contact = Un'altra domanda? Scrivici a

# Receipt scanning (mobile only)
expense-scan = Scansiona uno scontrino
scan-in-progress = Lettura dello scontrino…
scan-error-capture = Impossibile scattare la foto. Riprova, oppure inserisci la spesa a mano.
scan-error-unreadable = Niente di leggibile su questo scontrino. Inserisci la spesa a mano.
scan-check-amount = Controlla il totale: non era stampato chiaramente.
scan-take-photo = Scatta una foto
scan-choose-photo = Scegli una foto

update-required-title = Aggiornamento necessario
update-required-body = Questa versione di Counted è troppo vecchia per comunicare con il server. Aggiornala per continuare a usare l'app.
update-required-button = Aggiorna

### Common (aggiunte)

confirm = Conferma
copy = Copia
field-date = Data
date-today = Oggi
date-yesterday = Ieri

### Errors (amici)

error-identity-taken = Un altro account ha già rivendicato questo partecipante.
error-self-friend-request = Non puoi aggiungere te stesso come amico.
error-not-a-friend = Puoi invitare solo persone dalla tua lista di amici.
error-friend-has-no-key = Questo amico non ha ancora aperto l'ultima versione dell'app. Chiedigli di accedere una volta, poi riprova.
error-friend-request-not-found = Questa richiesta di amicizia non esiste più.
error-invitation-not-found = Questo invito non esiste più.
error-too-many-friend-requests = Troppe richieste di amicizia per ora. Riprova domani.
error-too-many-invitations = Troppi inviti in sospeso.
error-invalid-kdf-salt = Le impostazioni di crittografia non sono valide. Aggiorna l'app e riprova.
error-mixed-project-batch = Questi partecipanti non fanno tutti parte dello stesso progetto.
error-invalid-payload = Questa versione dell'app ha inviato dati che il server non accetta. Aggiornala e riprova.
error-invalid-public-key = La tua chiave di crittografia non è valida. Aggiorna l'app e riprova.
error-payment-methods-stale = I tuoi dati di pagamento sono stati modificati su un altro dispositivo. Ricarica e riprova.

### Payment methods (condivisione)

settings-payment-methods-share-warning = Un metodo condiviso è visibile a tutti i membri dei progetti in cui hai scelto il tuo nome - chiunque abbia uno di quei link.
payment-method-share = Condividi con i miei progetti
payment-method-share-hint = Mostrato accanto al tuo nome quando qualcuno ti deve dei soldi.
payment-method-copy = Copia { $name }
payment-method-copied = Copiato.
payment-method-copy-failed = Impossibile copiare: seleziona il testo e copialo a mano.

### Expenses (valuta, importo)

expense-category-auto = Auto · { $emoji }
expense-currency = Valuta dell'importo
amount-op-add = Più
amount-op-subtract = Meno
amount-op-multiply = Per
amount-op-divide = Diviso
amount-op-equals = Uguale
amount-op-done = Fine
expense-rate = Tasso di cambio (facoltativo)
expense-rate-hint = Lascia vuoto per usare il tasso della Commissione europea (InforEuro) di { $month }: 1 { $from } = { $rate } { $to }.
expense-rate-invalid = Inserisci un tasso di cambio maggiore di 0.
expense-rate-unavailable = Nessun tasso automatico disponibile: inseriscilo a mano.
expenses-show-more = Mostra altre ({ $count } rimanenti)
expense-converted-from = Pagato { $amount } { $from } · 1 { $from } = { $rate } { $to }
project-currency = Valuta
project-currency-hint = Tutti gli importi sono mostrati in questa valuta. Non potrà essere cambiata in seguito.
project-currency-locked = La valuta viene fissata alla creazione del progetto.
project-gone-title = Questo progetto non esiste più
participants-by-shares = Per quote
split-amounts = Importi

### Reimbursements

reimbursements-empty-title = Conti in pari!
reimbursement-record = Salda
reimbursement-pay-with = Paga
reimbursement-pay-shared-by = Condiviso da { $name }: controlla il nome del destinatario mostrato dalla tua app prima di inviare.
reimbursement-pay-title = Paga { $name }
reimbursements-mine-title = Devi
reimbursements-others-title = Altri rimborsi

### Identity

identity-claimed = Collegato a un account
identity-claimed-by = Account di { $name }
identity-taken-repick = Un altro account ha rivendicato il partecipante che stavi usando. Scegline un altro.
participant-gone-repick = Il partecipante che stavi usando è stato rimosso da questo progetto. Scegline un altro.

### Edit project

edit-project-title = Modifica il progetto
edit-project-new-badge = nuovo

### Charts (aggiunte)

charts-project = Progetto
charts-all-projects = Tutti i progetti
charts-date-from = Dal
charts-date-to = Al
charts-total = Totale
charts-tab-people = Persone
charts-tab-projects = Progetti
charts-scope = Spese di chi
charts-scope-group = Gruppo
charts-scope-me = Io
charts-currency = Valuta
charts-my-share = La mia quota
charts-share-of-total = { $pct }% di { $total }
charts-i-paid = Ho pagato
charts-paid-more = { $amount } più della tua quota
charts-paid-less = { $amount } meno della tua quota
charts-paid-even = Esattamente la tua quota
charts-part-title = La tua parte di ogni categoria
charts-part-desc = In grigio quanto ha speso il gruppo, a colori quanto hai consumato tu.
charts-breakdown-title = Ripartizione per categoria
charts-breakdown-desc = Tocca una fetta o una riga per vederne le spese.
charts-of-total = { $amount } su { $total }
charts-show-all = Mostra tutto ({ $count })
charts-show-less = Mostra meno
charts-spend-title = Spese nel tempo
charts-spend-desc = I periodi brevi vanno per giorno, quelli lunghi per settimana o mese.
charts-group-by = Raggruppa per
bucket-day = Giorno
bucket-week = Settimana
bucket-month = Mese
charts-avg = media
charts-cat-title-day = { $category }, giorno per giorno
charts-cat-title-week = { $category }, settimana per settimana
charts-cat-title-month = { $category }, mese per mese
charts-cat-desc = Scegli una categoria per seguirla nel tempo.
charts-running-title = Totale progressivo
charts-running-desc = Dal { $date }.
charts-avg-per-day = { $amount } / giorno in media
charts-avg-per-week = { $amount } / settimana in media
charts-avg-per-month = { $amount } / mese in media
charts-people-title = Chi ha sostenuto il gruppo
charts-people-desc = Quanto ha pagato ciascuno, accanto a quanto ha consumato.
charts-paid = Pagato
charts-fair-share = Quota giusta
charts-you = (tu)
charts-net-more = ha pagato di più
charts-net-less = ha pagato di meno
charts-balance-title = Il tuo saldo nel tempo
charts-balance-desc = Sopra la linea il gruppo ti deve. Sotto, sei tu a dovere al gruppo.
charts-owed = Ti devono
charts-owe = Devi
charts-projects-title = La tua quota, per progetto
charts-projects-desc = I totali restano per valuta e non vengono mai sommati tra loro.
history-by = Di { $name }

### Notifications

notifications-label = Notifiche
notifications-title = Notifiche
notifications-empty = Niente di nuovo
notifications-friend-request = Richiesta di amicizia

### Friends

friends-title = Amici
friends-anonymous-body = Gli amici sono legati al tuo account. Accedi per aggiungere persone e invitarle nei tuoi progetti senza condividere un link.
friends-add-title = Aggiungi un amico
friends-add-hint = Vedrà la tua richiesta al prossimo accesso. Nessuno dei due saprà se l'altro ha un account finché la richiesta non viene accettata.
friends-add-button = Aggiungi
friends-add-from-project = Aggiungi come amico
friends-request-sent = Richiesta inviata
friends-no-account-key = Accedi di nuovo per gestire i tuoi amici su questo dispositivo.
friends-incoming-title = Richieste
friends-accept = Accetta
friends-decline = Rifiuta
friends-list-title = I miei amici
friends-list-empty = Ancora nessun amico. Aggiungi qualcuno via e-mail qui sopra, o da un progetto che condividete.
friends-remove = Rimuovi
friends-remove-confirm-title = Rimuovi amico
friends-remove-confirm-message = { $email } non sarà più tra i tuoi amici, e tu non sarai più tra i suoi. Ognuno di voi potrà inviare una nuova richiesta in seguito.
friends-no-key = Non ancora pronto
friends-fingerprint = Codice di sicurezza
friends-fingerprint-hint = Due amici che si leggono lo stesso codice di sicurezza sanno che nessuno si è messo in mezzo, nemmeno il nostro server.
friends-outgoing-title = Inviate
friends-outgoing-hint = In attesa di risposta. Compariranno tra i tuoi amici quando accetteranno.
friends-withdraw = Annulla
invite-friends-title = Invita amici
invite-friends-hint = La chiave del progetto viene cifrata per ogni amico su questo dispositivo. Il server non la vede mai.
invite-friends-empty = Ancora nessun amico da invitare.
invite-friends-button = Invita
invite-sent = { $count ->
    [one] Invito inviato
   *[other] { $count } inviti inviati
}
invitation-badge = Invito
invitation-to = Unisciti a «{ $name }»
invitation-to-unnamed = Unisciti a un progetto
invitation-unreadable = Questo invito non può essere aperto su questo dispositivo
invitation-from = Da { $email }
invitation-accept = Unisciti
invitation-decline = Rifiuta
