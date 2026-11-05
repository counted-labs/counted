# Italiano. Completo tranne i testi legali (legal-, terms-, privacy-), che esistono solo in inglese
# e francese e ricadono su en.ftl messaggio per messaggio.

### Common

loading = Caricamento…
cancel = Annulla
retry = Riprova
delete = Elimina
back = Indietro
language = Lingua
theme = Tema
theme-light = Chiaro
theme-dark = Scuro
theme-system = Sistema
theme-hint-device = Salvato solo su questo dispositivo.
theme-hint-system = Segue l'aspetto di questo dispositivo.

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
error-rate-limited = Il server è occupato. Riprova tra un attimo.
error-generic = Si è verificato un errore. Riprova.

error-invalid-email = Questo indirizzo e-mail non è valido.
error-invalid-password = Questa password non è valida.
error-password-too-short = La password deve contenere almeno 12 caratteri.
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
error-storage-full = Lo spazio di archiviazione è pieno: non è stato possibile salvare la chiave di questo progetto su questo dispositivo. Conserva il link di condivisione.
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
login-maybe-unverified = Nuovo account? Verifica prima il tuo indirizzo con il link che ti abbiamo inviato, o invialo di nuovo.
login-resend = Invia di nuovo il link di verifica
login-resending = Invio…
login-resend-sent = E-mail inviata: controlla la tua casella.

register-submit = Crea un account
register-submitting = Creazione…
register-have-account = Hai già un account?
register-password-placeholder = Almeno 12 caratteri
register-password-warning = Annota la password: non la vediamo mai e non possiamo restituirtela.
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
account-delete-warning = Eliminazione immediata e definitiva del tuo account e dei tuoi dati.
account-delete-confirm-title = Elimina l'account
account-delete-confirm-message = Il tuo account, le tue sessioni e l'elenco dei tuoi progetti saranno eliminati definitivamente. Senza la tua password, i dati cifrati di un progetto condiviso diventano illeggibili per te: l'operazione è irreversibile.
erase-device-open = Esci e cancella questo dispositivo
erase-device-title = Cancellare questo dispositivo?
erase-device-body = I progetti, le loro chiavi e la cache su questo dispositivo verranno cancellati, poi la sessione verrà chiusa. I tuoi progetti restano nel tuo account: accedi di nuovo per ritrovarli.
erase-device-checking = Verifico che i tuoi progetti siano nel tuo account…
erase-device-pending = Modifiche in attesa di invio: { $count }. Torna online, poi riprova.
erase-device-unreachable = Impossibile verificare che i tuoi progetti siano nel tuo account. Controlla la connessione, poi riprova.
erase-device-at-risk = Questi progetti non sono nel tuo account e andranno persi. Per tenerli, annulla e copia prima il loro link:
erase-device-confirm = Cancella ed esci
erase-device-confirm-anyway = Cancella comunque
erase-device-erasing = Cancellazione…

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
payment-methods-stale = I tuoi dati di pagamento sono stati modificati su un altro dispositivo. Sono stati ricaricati - riprova.
payment-methods-key-missing = Accedi di nuovo per gestire i tuoi dati di pagamento.

verify-email-checking = Verifica della tua e-mail…
verify-email-welcome = E-mail verificata - benvenuto su Counted!
verify-email-back-to-login = Torna all'accesso
verify-email-intro = Inserisci la tua password per confermare che questo indirizzo è tuo.
verify-email-submit = Verifica il mio indirizzo

### Project status

project-archive = Archivia
project-sheet-invite = Invita
project-sheet-recurring = Ricorrenti
project-sheet-edit = Modifica progetto
project-sheet-close = Chiudi progetto
project-sheet-archive = Archivia progetto
project-sheet-reopen = Riapri progetto
project-sheet-unarchive = Rimuovi progetto dall’archivio
project-sheet-leave = Esci dal progetto

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
projects-offline-banner = Dati offline - riconnettiti per aggiornare.
demo-banner = Progetto dimostrativo - sola lettura.
demo-start-own = Crea il tuo progetto
demo-badge = Demo
demo-bar = Le modifiche spariscono quando chiudi questa scheda.
demo-leave = Inizia davvero
demo-preparing = Preparazione della demo…
demo-failed = Impossibile preparare la demo.
demo-sign-in-blocked = L’accesso è disattivato nella demo. Esci per accedere o creare un account; i tuoi progetti restano intatti.
projects-no-local-data = Nessun dato locale
projects-no-local-data-hint = Accedi per caricare i tuoi progetti per la prima volta.
projects-add = Aggiungi un progetto
projects-create = Crea un progetto
projects-personal-expense = Spesa personale
personal-expense-banner = Aggiunta a Le mie spese
personal-create-title = Crea il tuo progetto personale
personal-create-hint = Segui quello che spendi da solo
group-create-title = Crea un progetto condiviso
group-create-hint = Dividi le spese con amici e famiglia
personal-create-action = Crea
personal-create-signup = Crea un account
personal-sheet-title = Il tuo progetto personale
personal-sheet-intro = Per annotare quello che spendi da solo. Compare con i tuoi gruppi nelle Statistiche.
personal-sheet-point-alone = Un solo membro: tu, con il nome del tuo account. Nessun link di condivisione.
personal-sheet-point-encrypted = Cifrato sui tuoi dispositivi, come gli altri progetti.
personal-sheet-point-stats = Le tue quote nei gruppi si sommano a queste spese nelle Statistiche.
personal-default-name = Le mie spese
my-spending-prev-month = Mese precedente
my-spending-next-month = Mese successivo
my-spending-spent = Hai speso
my-spending-groups = Gruppi
my-spending-income = Entrate
my-spending-balance = Saldo del mese
my-spending-unconverted = Le quote in un’altra valuta restano escluse finché non è noto un tasso.
my-spending-empty = Niente questo mese
my-spending-empty-hint = Aggiungi una spesa con +. Le tue quote nei gruppi compariranno qui automaticamente.
my-spending-share-of = su { $amount }
my-spending-uncounted = non conteggiata
payer-you = te
expense-received-by = ricevuto da
counted-switch = Conta in Le mie spese
counted-switch-on = Attivo: la tua quota si aggiunge a Le mie spese
counted-switch-off = Disattivo: la tua quota non compare in Le mie spese
counted-groups = Gruppi conteggiati
counted-groups-intro = La tua quota in questi gruppi si aggiunge a Le mie spese e ai budget.
counted-groups-converted = { $from }, convertito in { $to }
counted-groups-encrypted = Questa scelta è cifrata con il tuo account e ti segue sui tuoi dispositivi.
counted-groups-empty = Ancora nessun gruppo su questo dispositivo.
counted-groups-done = OK
counted-save-failed = Impossibile salvare questa scelta. Riprova.
tab-budget = Budget
budget-title = Budget
budget-manage = Gestisci
budget-left = Restano { $amount }
budget-over = Superato di { $amount }
budget-empty = Nessun budget
budget-empty-hint = Fissa un tetto per categoria. Contano anche le tue quote nei gruppi.
budget-define = Definisci un budget
budget-sheet-title = Budget mensili
budget-sheet-intro = Sulla tua quota: personali e gruppi. Lascia vuoto per non seguire una categoria.
budget-average = Media ultimi 3 mesi: { $amount }
budget-for = Budget { $category }
budget-alert = Avvisami all’80 %
budget-alert-hint = Calcolato su questo dispositivo
budget-invalid = Importo non valido per { $category }.
budget-offline = I budget si salvano solo online. Riconnettiti e riprova.
budget-alert-near = { $category }: { $percent } % del budget del mese
budget-alert-over = { $category }: budget del mese superato
personal-card-budget = { $month } · budget { $amount }
my-spending-search-summary =
    { $count ->
        [one] { $count } risultato
       *[other] { $count } risultati
    } · { $total } · personali e gruppi, tutti i mesi
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
add-project-offline = Non puoi creare un progetto offline. Riconnettiti e riprova.
add-project-name-required = Il progetto ha bisogno di un nome.

join-link-label = Link di condivisione
join-link-hint = Il link contiene la chiave di decifratura: copialo per intero.
join-invalid-link = Questo link non è valido. Incolla il link di condivisione completo, compresa la parte dopo il #.
join-wrong-project = Questo link appartiene a un altro progetto.

import-tricount-link-label = Link o chiave Tricount
import-tricount-key-required = Inserisci un link o una chiave Tricount.
import-tricount-encryption-failed = Errore di cifratura.
import-tricount-unimportable = Non è stato importato nulla: questo Tricount ha membri con un account Tricount o importi che non tornano (voci interessate: { $count }).

### Expenses

save = Salva
saving = Salvataggio…
adding = Aggiunta…
link-copied = Link copiato
missing-encryption-key = Chiave di cifratura mancante.
missing-encryption-key-title = Chiave di cifratura mancante
missing-encryption-key-hint = Il link che hai usato non contiene la chiave necessaria per decifrare questo progetto. Usa il link completo condiviso da chi lo ha creato.
project-locked-hint = Questo dispositivo non ha la chiave di questo progetto. Apri il link di condivisione per sbloccarlo.
expenses-unreadable = Alcune voci non sono leggibili su questo dispositivo e sono escluse dai totali. Aggiorna l’app per vederle.
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
expense-type-group = Tipo
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
balance-gets-back = Riceve
balance-owes = Deve
balance-settled = In pari
reimbursements-empty-hint = Qui compariranno i rimborsi suggeriti quando i conti non tornano
reimbursement-owes = { $debtor } deve a { $creditor }

user-selection-title = Quale partecipante sei?
user-selection-hint = Scegli il tuo nome dall'elenco.
user-selection-required = Seleziona un partecipante.

edit-project-deferred-new-members = l'aggiunta di nuovi partecipanti
edit-project-deferred-removals = la rimozione di partecipanti
edit-project-offline-deferred = Offline: { $items } verrà applicato alla riconnessione.

export-failed = Esportazione non riuscita: { $reason }

history-expense-added = Spesa aggiunta: { $name }
history-expense-edited = Spesa modificata: { $name }
history-expense-deleted = Spesa eliminata: { $name }
move-expense = Sposta in un progetto
move-expense-title = Sposta in…
move-current = Attuale
move-converted = { $currency }, convertito al tasso del mese
move-note-personal = Sceglierai chi ha pagato e per chi al passaggio successivo. La spesa lascerà Le mie spese.
move-note-group = La spesa lascerà { $project } per tutti i suoi membri e il saldo del gruppo verrà ricalcolato. Sceglierai chi ha pagato e per chi al passaggio successivo.
move-banner = Da { $from } a { $to }
move-submit = Sposta
move-done = Spesa spostata in { $project }
move-delete-failed = La spesa è stata aggiunta all’altro progetto ma è ancora in { $project }: eliminala lì.
history-expense-moved = Spesa spostata in un altro progetto: { $name }
personal-badge = Personale
move-members =
    { $count ->
        [one] { $count } membro
       *[other] { $count } membri
    }
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
expense-share-percent = { $percent }%
expense-added-by = Aggiunto da { $name } il { $date }
expense-inconsistent-detail = Importi incoerenti: { $paid } pagato, { $owed } dovuto, per una spesa di { $total }. Modifica la spesa per correggerla.
missing-access-key = Chiave di accesso mancante. Apri questo progetto dal suo link di condivisione.
filter-all = Tutto
filter-my-payments = I miei pagamenti
filter-my-debts = Quanto devo
expense-filter-group = Filtro
expenses-search = Cerca spese
expenses-no-match = Nessuna spesa corrisponde.
expenses-search-open = Cerca
expenses-search-clear = Cancella la ricerca
expenses-search-summary =
    { $count ->
        [one] { $count } risultato
       *[other] { $count } risultati
    } · { $total } · tutti i mesi
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
charts-my-share-skipped =
    { $count ->
        [one] 1 progetto non è conteggiato - nessun partecipante scelto, o i dati non sono stati caricati.
       *[other] { $count } progetti non sono conteggiati - nessun partecipante scelto, o i dati non sono stati caricati.
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
categories-manage = Gestisci categorie
categories-title = Categorie
categories-yours = Le tue categorie
categories-builtin = Predefinite
categories-add = Aggiungi una categoria
categories-in-parent = in { $parent }
categories-renamed-from = rinominata · in origine { $name }
categories-hidden-hint = nascosta · le sue spese vanno in { $other }
categories-always-shown = sempre visibile
categories-hidden = Nascosta
categories-new = Nuova categoria
categories-edit = Modifica categoria
categories-parent = Conteggiata in
categories-parent-hint = Nelle statistiche di tutti i tuoi progetti, { $name } conta in { $parent }.
categories-create = Crea
categories-hide = Nascondi questa categoria
categories-hide-hint = Le sue spese appaiono in { $other } per tutti i membri.
categories-delete-title = Eliminare «{ $name }»?
categories-delete-body = Le spese interessate ({ $count }) tornano in { $parent }. Nessuna spesa viene eliminata.
categories-delete-body-unknown = Le sue spese tornano in { $parent }. Nessuna spesa viene eliminata.
categories-delete-warning = La categoria scompare per tutti i membri del progetto.
categories-summary-created = Create: { $count }
categories-summary-renamed = Rinominate: { $count }
categories-summary-hidden = Nascoste: { $count }
categories-summary-default = Nessuna categoria personalizzata
categories-offline = Per gestire le categorie serve la connessione. Riconnettiti e riprova.
categories-error-too-many = Un progetto ha al massimo 20 categorie proprie.
categories-error-full = Questo progetto ha raggiunto il limite di categorie.
categories-error-name = Dai un nome alla categoria.
categories-error-missing = Questa categoria è cambiata nel frattempo. Chiudi e riprova.
history-category-created = Categoria creata: { $name }
history-category-edited = Categoria modificata: { $name }
history-category-deleted = Categoria eliminata: { $name }
history-empty = Nessun evento
not-found-hint = Questa pagina non esiste o è stata spostata.
payers-title-paid-by = Pagato da
payers-title-sender = Mittente
payers-title-contributors = Contributori
debtors-title-debtors = Deve
debtors-title-recipients = Destinatari
debtors-title-beneficiaries = Beneficiari
debtors-title-for = Per
debtors-title-recipient = Destinatario
expense-split-between = diviso tra { $count }
expense-people =
    { $count ->
        [one] { $count } persona
       *[other] { $count } persone
    }

### Welcome

welcome-title = I tuoi conti riguardano solo te.
welcome-subtitle = Dividi le spese tra amici.
welcome-note = Gratis. Nessun account necessario. Niente pubblicità.
welcome-link-title = Un link, e partecipano tutti.
welcome-link-body = Nessuno deve creare un account.
welcome-link-account = Un account? Mai obbligatorio. Serve a ritrovare i tuoi progetti su un altro dispositivo, invitare gli amici dall'app e condividere i tuoi dati di pagamento.
welcome-demo-project = Weekend a Lione
welcome-private-title = Nessuno può leggere i tuoi conti. Nemmeno noi.
welcome-private-body = Nomi, importi, progetti: tutto viene cifrato sul tuo dispositivo. Solo tu hai la chiave.
welcome-private-names = Nomi
welcome-private-amounts = Importi
welcome-private-projects = Progetti
welcome-scan-title = Fotografa lo scontrino.
welcome-scan-body = Importo, data e categoria si compilano da soli. Tutto avviene sul tuo telefono. La foto non viene conservata.
welcome-eu-title = 100 % europeo
welcome-no-ads = Zero pubblicità
welcome-no-trackers = Zero tracciatori
welcome-step = Passo { $current } di { $total }
welcome-next = Avanti
welcome-skip = Salta
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
help-forgot-password-warning = Nessuno può recuperarla al posto tuo.
help-forgot-password-a = La tua password protegge le tue chiavi di cifratura e non ci viene mai inviata: nessuno - noi compresi - può decifrare i tuoi progetti al posto tuo. Conservala bene, meglio in un gestore di password.
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

settings-payment-methods-share-warning = Un metodo condiviso è visibile a tutti i membri dei tuoi progetti.
payment-method-share = Condividi con i miei progetti
payment-method-share-hint = Mostrato accanto al tuo nome quando qualcuno ti deve dei soldi.
payment-method-copy = Copia { $name }
payment-method-copied = Copiato.
payment-method-copy-failed = Impossibile copiare: seleziona il testo e copialo a mano.

### Expenses (valuta, importo)

expense-category-auto = Auto · { $emoji }
emoji-picker-label = Emoji
emoji-none = Nessuna emoji
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
project-currency-locked = La valuta viene fissata alla creazione del progetto.
currency-search = Cerca una valuta
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
charts-paid-more = { $amount } più della mia quota
charts-paid-less = { $amount } meno della mia quota
charts-paid-even = Esattamente la mia quota
charts-part-title = La tua parte di ogni categoria
charts-part-desc = In grigio quanto ha speso il gruppo, a colori quanto hai consumato tu.
charts-breakdown-title = Ripartizione per categoria
charts-breakdown-desc = Tocca una fetta o una riga per vederne le spese.
charts-of-total = { $amount } su { $total }
charts-show-all = Mostra tutto ({ $count })
charts-show-less = Mostra meno
charts-spend-title = Spese nel tempo
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
charts-projects-desc = I totali restano per valuta.
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
friends-add-hint = Se questa persona ha un account, vedrà la tua richiesta nell'app. Altrimenti avvisala: la troverà registrandosi con questo indirizzo.
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
friends-key-changed = Codice cambiato
friends-key-changed-hint = Il codice di sicurezza di un amico è cambiato. Succede quando protegge il suo account, o se la sua chiave è stata sostituita. Gli inviti con lui sono bloccati finché non vi leggete il nuovo codice, di persona o al telefono, e tocchi «Il codice corrisponde».
friends-fingerprint = Codice di sicurezza
friends-fingerprint-hint = Due amici che si leggono lo stesso codice di sicurezza sanno che nessuno si è messo in mezzo, nemmeno il nostro server.
friends-key-mismatch = Il server ha inviato per il tuo account una chiave pubblica che non corrisponde alla tua chiave privata. Il tuo codice di sicurezza è nascosto e gli inviti sono disattivati su questo dispositivo: qualcuno potrebbe manomettere il server.
keypair-mismatch = Le chiavi che il server ha inviato per il tuo account non corrispondono a quelle di questo dispositivo. Inviti e cambi di chiave sono disattivati qui finché non corrispondono: qualcuno potrebbe manomettere il server.
friends-outgoing-title = Inviate
friends-outgoing-hint = In attesa di risposta. Compariranno tra i tuoi amici quando accetteranno.
friends-withdraw = Annulla
friends-notify = Avvisa con un messaggio
friends-notify-label = Avvisa { $email }
friends-notify-message = Passo a Counted per le nostre spese condivise. Iscrizione gratuita su https://counted.fr con l'indirizzo { $email }: la mia richiesta di amicizia aspetta lì.
friends-notify-copied = Messaggio copiato
friends-notify-copy-failed = Impossibile copiare il messaggio
guests-hint-title =
    { $count ->
        [one] { $count } partecipante non si è ancora unito al progetto
       *[other] { $count } partecipanti non si sono ancora uniti al progetto
    }
guests-hint-later = Più tardi
guests-hint-send = Invitali
guests-send-title = Invia il progetto ai partecipanti
guests-send-friend = Invita un amico Counted
guests-send-friend-badge = Senza link
guests-send-friend-hint = Se ha già un account, riceve un invito cifrato: niente passa dalla tua app di messaggi.
guests-send-link = Condividi il link del progetto
guests-send-link-hint = Per chi non ha un account. Apre il progetto con un tocco.
guests-send-link-warning = Chiunque abbia questo link può leggere e modificare il progetto. Anche la tua app di messaggi lo vede.
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

# Participants in the create and edit modals, and the "who are you?" picker - see
# docs/plans/friends.md §11.
participants-you-label = Il tuo nome in questo progetto
participants-you-badge = Tu
participants-others = Altri partecipanti
participants-removed = Rimossi
participant-restore = Ripristina
participants-empty = Aggiungi i partecipanti qui sotto.
participants-duplicate = «{ $name }» è già nell'elenco.
participants-input-label = Aggiungi un amico o scrivi un nome
participants-input-placeholder = Un amico o un nome qualsiasi
participants-suggest-friend = Amico · entra come «{ $name }», riceve un invito
participants-suggest-not-ready = Amico · non ancora pronto
participants-suggest-guest = Aggiungi «{ $text }»
participants-suggest-guest-sub = Ospite
participants-friends = I tuoi amici
participants-all-friends = Tutti gli amici
participants-login-hint = Accedi per aggiungere persone direttamente dalla tua lista di amici.
participants-invite-badge = Da invitare
participants-guest-badge = Ospite
participants-rename = Rinomina { $name }
participants-remove = Rimuovi { $name }
participants-rename-label = Nuovo nome
participants-rename-save = Salva il nome
participants-rename-hint = Il nome che tutti vedono in questo progetto. L'invito va comunque a { $email }.
participants-invited-badge = Invitato
participants-invited-sub = { $email } · non ancora accettato
participants-invited-pending = Invito non ancora accettato
participants-unlinked = Non collegato a un account
add-project-create-invite = Crea e invita { $count }
edit-project-save-invite = Salva e invita { $count }
edit-project-you-are = Su questo dispositivo sei { $name }
edit-project-no-identity = Non hai ancora scelto chi sei
edit-project-switch = Cambia
edit-project-choose = Scegli
invite-failed = Questi inviti non sono stati inviati: { $emails }
invite-again = Invita di nuovo
friend-picker-title = Aggiungi amici
user-selection-invited-hint = { $email } ti ha invitato in «{ $project }».
user-selection-suggested = Suggerito
user-selection-suggested-sub = { $email } ti ha aggiunto con questo nome
user-selection-confirm-as = Sono { $name }
user-selection-missing = Il tuo nome non c'è? Chiedi a un partecipante di aggiungerti nelle impostazioni del progetto.

## Spese ricorrenti

repeat-label = Ripeti
repeat-none = Non si ripete
repeat-weekly = Ogni settimana
repeat-biweekly = Ogni 2 settimane
repeat-monthly = Ogni mese
repeat-quarterly = Ogni 3 mesi
repeat-yearly = Ogni anno
repeat-every-weeks = Ogni { $count } settimane
repeat-every-months = Ogni { $count } mesi
repeat-every-years = Ogni { $count } anni
repeat-custom = Personalizzato…
repeat-every = Ogni
repeat-unit-weeks = settimane
repeat-unit-months = mesi
repeat-unit-years = anni
repeat-on-weekday = di { $weekday }
repeat-on-day = il giorno { $day }
repeat-on-day-month = il { $day } { $month }
repeat-month-end = Nei mesi più corti cade l’ultimo giorno.
repeat-ends = Termina
repeat-ends-never = Mai
repeat-ends-on = In una data
repeat-ends-after = Dopo
repeat-fewer = Meno
repeat-more = Più
repeat-last-on = l’ultima il { $date }
repeat-variable = L’importo cambia ogni volta
repeat-variable-hint = Ognuna viene aggiunta con l’ultimo importo e segnata «da confermare».
repeat-done = Fatto
repeat-no-end = Senza fine
repeat-until = Fino al { $date }
repeat-occurrences = Ripetizioni: { $count }
repeat-offline = Serve una connessione. La spesa può comunque essere aggiunta.
repeat-foreign = Si ripete per { $amount } { $currency }, convertiti una volta al cambio di oggi. Riceverai un avviso se il cambio varia di più del 5%.
repeat-backfill = Inizia nel passato. Spese aggiunte ora: { $count }.
add-and-repeat = Aggiungi e ripeti
weekday-1 = lunedì
weekday-2 = martedì
weekday-3 = mercoledì
weekday-4 = giovedì
weekday-5 = venerdì
weekday-6 = sabato
weekday-7 = domenica
recurring-title = Spese ricorrenti
recurring-strip = Spese ricorrenti: { $count }
recurring-next = Prossima: { $name }, { $date }
recurring-to-confirm = Da confermare: { $count }
recurring-per-month = Al mese, circa
recurring-your-share = La tua quota
recurring-active = Attive
recurring-paused = In pausa
recurring-finished = Terminate
recurring-paid-by = pagata da { $name }
recurring-next-on = Prossima il { $date }
recurring-progress = { $done } di { $total }
recurring-rate-badge = Cambio variato del { $percent }%
recurring-empty = Ancora nulla si ripete. Scegli «Ripeti» quando aggiungi una spesa: affitto, abbonamenti, bollette.
recurring-next-ones = Prossime
recurring-added-so-far = Aggiunte finora
recurring-set-up-by = Creata da
recurring-pause = Metti in pausa
recurring-resume = Riprendi
recurring-stop = Interrompi la ripetizione
recurring-stop-title = Interrompere «{ $name }»?
recurring-stop-message = Non si ripeterà più. Le spese già aggiunte restano.
recurring-resume-title = Riprendere «{ $name }»?
recurring-resume-message = Prossima il { $date }. Le date saltate durante la pausa non vengono aggiunte.
recurring-edit-title = Modifica spesa ricorrente
recurring-edit-banner = Le modifiche valgono dal { $date }. Le spese già aggiunte restano come sono.
recurring-next-on-label = Prossima il
recurring-next-too-early = La prossima data deve venire dopo l’ultima spesa già aggiunta.
recurring-use-stop = Per terminarla, usa «Interrompi la ripetizione» sulla spesa ricorrente.
recurring-drift = Il cambio { $currency } è variato del { $percent }% dalla creazione. Ognuna viene ancora aggiunta per { $amount } { $project_currency } (1 { $currency } = { $rate }). Al cambio di oggi sarebbero { $today_amount } { $project_currency }.
recurring-use-rate = Usa il cambio di oggi
recurring-keep = Mantieni { $amount } { $currency }
recurring-added = Spese ricorrenti aggiunte: { $count }
recurring-blocks-removal = { $name } non può ancora essere rimosso: fa parte di { $rules }. Togli { $name } da queste spese o interrompile, poi salva di nuovo.
history-recurring-added = Aggiunta automaticamente: { $name } ({ $date })
history-recurring-created = Spesa ricorrente creata: { $name }
history-recurring-edited = Spesa ricorrente modificata: { $name }
history-recurring-paused = Spesa ricorrente messa in pausa: { $name }
history-recurring-resumed = Spesa ricorrente ripresa: { $name }
history-recurring-stopped = Spesa ricorrente interrotta: { $name }
occurrence-recurring = Spesa ricorrente
occurrence-auto = Aggiunta automaticamente da una spesa ricorrente.
occurrence-auto-next = Aggiunta automaticamente da una spesa ricorrente. Prossima il { $date }.
occurrence-auto-stopped = Aggiunta automaticamente da una spesa ricorrente poi interrotta.
occurrence-manage = Gestisci
estimate-badge = Da confermare
expense-pending = In attesa
estimate-title = Importo da confermare.
estimate-body = Aggiunta con l’importo precedente. Inserisci quello reale quando arriva.
estimate-confirm = Conferma importo
apply-to = Applica a
apply-this-only = Solo questa spesa
apply-and-next = Questa e le successive
apply-and-next-hint = La spesa ricorrente cambia dal { $date }
apply-rule-failed = La spesa è stata salvata, ma la spesa ricorrente non è stata modificata.
occurrence-delete-message = «{ $name }» del { $date } sarà eliminata definitivamente. La ripetizione continua e questa data non tornerà.
occurrence-delete-one = Elimina solo questa
occurrence-delete-stop = Elimina e interrompi la ripetizione
error-recurring-clock = L’orologio di questo dispositivo è avanti. Controlla data e ora.
error-recurring-not-found = Questa spesa ricorrente non esiste più.
error-recurring-stale = Qualcuno ha modificato questa spesa ricorrente nel frattempo. È stata ricaricata: controllala e salva di nuovo.
error-too-many-recurring = Questo progetto ha già 50 spese ricorrenti. Interrompine una che non ti serve più per aggiungerne un’altra.
error-user-in-recurring = Questo partecipante fa parte di una spesa ricorrente. Toglilo da essa o interrompila prima.

home-owed-to-you = Ti devono
home-you-owe = Devi
home-details-show = Vedi dettaglio
home-details-hide = Nascondi dettaglio
home-all-settled = Tutto saldato
home-pay = Paga
home-remind = Ricorda
home-remind-message = Piccolo promemoria: ci sono ancora rimborsi da saldare in { $project }.
home-row-owed = ti devono
home-row-owe = devi
home-row-settled = In pari
home-totals-partial = Esclusi i progetti senza tasso di cambio
error-account-key-changed = La chiave del tuo account è cambiata su un altro dispositivo. Accedi di nuovo per continuare.
settings-security = Sicurezza
settings-change-password = Cambia password
settings-change-password-hint = I tuoi progetti e i tuoi dati restano come sono. Gli altri tuoi dispositivi vengono disconnessi.
change-password-title = Cambia la tua password
change-password-current = Password attuale
change-password-new = Nuova password
change-password-confirm = Conferma la nuova password
change-password-mismatch = Le due nuove password non coincidono.
change-password-submit = Cambia password
change-password-submitting = Modifica in corso…
change-password-done = Password cambiata. Gli altri tuoi dispositivi sono stati disconnessi.
error-rotation-incomplete = Il tuo account è cambiato nel frattempo. Riprova.
error-key-lifecycle-unavailable = Non ancora disponibile: prima tutti devono avere l’ultima versione dell’app.
settings-compromised = Qualcuno potrebbe conoscere la mia password
settings-compromised-hint = Nuova password, nuove chiavi e tutti gli altri dispositivi disconnessi.
settings-compromised-open = Proteggi il mio account
compromised-title = Proteggi il tuo account
compromised-body = La tua password e le chiavi del tuo account vengono sostituite e tutti gli altri dispositivi vengono disconnessi. I tuoi amici vedranno cambiare il tuo codice di sicurezza: confrontalo di nuovo con loro.
compromised-keeps = Chi aveva la tua password conserva ciò che ha già aperto: i progetti che poteva leggere finora.
compromised-submit = Sostituisci password e chiavi
compromised-done = Il tuo account è protetto. Chiedi ai tuoi amici di confrontare il tuo nuovo codice di sicurezza.
friends-new-fingerprint = Nuovo codice:
friends-code-matches = Il codice corrisponde
friends-code-trusted = Codice confermato.
settings-recovery-key = Chiave di recupero
settings-recovery-key-none = Senza, una password dimenticata ti costa preferenze, dati di pagamento e codici degli amici, e richiede 3 giorni.
settings-recovery-key-set = Creata. Se dimentichi la password, ti restituisce l’account con tutto il suo contenuto.
settings-recovery-key-pending = Una nuova chiave la sostituisce il { $when }. Non sei stato tu? Annullala con la tua chiave attuale.
recovery-key-create = Crea
recovery-key-replace = Sostituisci
recovery-key-cancel-replacement = Annulla la nuova chiave
recovery-key-title = La tua chiave di recupero
recovery-key-body = Scrivila o salvala dove solo tu puoi arrivare. Counted non può mostrarla di nuovo né recuperarla per te.
recovery-key-copy = Copia
recovery-key-copied = Copiata
recovery-key-save = Salva come file
recovery-key-print = Stampa
recovery-key-next = Continua
recovery-key-confirm-body = Per controllare la tua copia, scrivi il gruppo { $n } della chiave.
recovery-key-confirm-wrong = Questo gruppo non corrisponde. Ricontrolla la tua copia.
recovery-key-password-body = Inserisci la password per salvare la chiave nel tuo account.
recovery-key-submit = Salva la chiave
recovery-key-done = Chiave di recupero salvata.
recovery-key-replacement-done = Nuova chiave salvata. Sostituisce l’attuale tra 72 ore; fino ad allora l’attuale funziona ancora.
recovery-key-field = Chiave di recupero
recovery-key-file-intro = Chiave di recupero di Counted
recovery-key-cancel-done = La nuova chiave è stata annullata.
compromised-new-key = Salva ora la tua nuova chiave di recupero: sostituisce la vecchia tra 72 ore, e fino ad allora la vecchia funziona ancora.
pending-reset-title = È stato richiesto il ripristino del tuo account
pending-reset-body = Ha effetto il { $when }: allora preferenze, dati di pagamento e codici degli amici andranno persi e i tuoi dispositivi dovranno accedere di nuovo. Non sei stato tu? Annullalo con la tua password attuale.
pending-reset-cancel = Annulla il ripristino
pending-reset-cancelled = Il ripristino è stato annullato.
login-forgot = Password dimenticata?
forgot-title = Password dimenticata
forgot-body = Inserisci la tua email. Se ha un account, inviamo un link per scegliere una nuova password.
forgot-submit = Invia il link
forgot-sent = Se questo indirizzo ha un account, un link è in arrivo. Scade tra 24 ore.
reset-title = Recupera il tuo account
reset-with-key-body = Inserisci la chiave di recupero e una nuova password: tutto torna com’era.
reset-submit-key = Recupera il mio account
reset-no-key = Non ho la mia chiave di recupero
reset-without-key-title = Ripristina senza la chiave di recupero
reset-without-key-body = Il tuo account riparte con una nuova password tra 72 ore. I progetti tornano dai tuoi dispositivi e dai link di condivisione; preferenze, dati di pagamento e codici degli amici no. Fino ad allora la password attuale funziona ancora e può annullare il ripristino.
reset-name = Il tuo nome
reset-submit-request = Ripristina il mio account
reset-requested = Fatto. La nuova password funziona dal { $when }. Fino ad allora quella attuale funziona ancora e può annullarlo.
reset-pending-already = Un ripristino è già in attesa fino al { $when }. Una nuova richiesta fa ripartire le 72 ore.
recovered-done = Il tuo account è tornato. Gli altri tuoi dispositivi sono stati disconnessi.
login-reset-undoable = Questa password è stata sostituita da un ripristino negli ultimi 30 giorni. Se non l’hai chiesto, annullalo: il tuo account torna com’era.
login-undo-reset = Annulla il ripristino
reset-undone = Il ripristino è annullato. Il tuo account è tornato com’era.
error-reset-pending = Questa nuova password funziona quando il ripristino diventa effettivo, 72 ore dopo la richiesta.
error-account-disabled = Questo account è stato eliminato poco dopo un ripristino e attende la cancellazione. La password precedente può ancora annullare il ripristino.
error-recovery-key-invalid = Questa chiave di recupero non apre questo account.
error-project-key-required = Questo progetto richiede la sua chiave. Riaprilo dal suo link di condivisione.
names-or = { $first } o { $last }
participants-admin-badge = Admin
edit-project-admins = Admin: { $count }
edit-project-succession = Se il progetto resta senza admin, lo diventa il membro con account presente da più tempo.
member-sheet-account = Membro con account
member-actions = Azioni per { $name }
member-make-admin = Rendi admin
member-make-admin-hint = Potrà cambiare la chiave del progetto e rimuovere qualcuno.
member-made-admin = { $name } ora è admin.
group-security = Sicurezza del progetto
group-security-admin = Sicurezza del progetto · sei admin
group-rotate = Cambia la chiave del progetto
group-rotate-hint = Se il link del progetto è stato compromesso: chi ha il vecchio link non avrà più accesso al progetto.
group-remove = Rimuovi qualcuno dal progetto
group-remove-hint = Toglie l’accesso alla persona e cambia la chiave nello stesso momento.
group-admins-only = Riservato agli admin.
group-admins-only-account = Riservato agli admin, che hanno un account.
group-ask-admin = Chiedi a { $names }, admin del progetto.
group-ask-admins = Chiedi a { $names }, che amministrano il progetto.
group-ask-an-admin = Chiedi a una delle persone che amministrano il progetto.
group-with-account = Con un account
group-free = Gratis
group-sign-in-hint = { $names }, che amministra il progetto, potrà poi darti il ruolo.
group-sign-in-hint-any = Chi amministra il progetto potrà poi darti il ruolo.
group-no-admin = Questo progetto non ha admin, quindi la sua chiave non può cambiare.
group-hide = Nascondi qualcuno
group-hide-hint = Nasconde la persona dagli elenchi senza toglierle l’accesso: il link e la chiave restano validi.
group-recreate-hint = Questo progetto non ha admin. Per rimuovere qualcuno, crea un nuovo progetto.
group-recreate = Crea un nuovo progetto
group-no-admin-yet = Questo progetto non ha ancora admin.
group-creator-hint = Solo tu puoi diventare admin di questo progetto, creando un account.
group-creator-elsewhere-hint = Solo la persona che ha creato il progetto può diventarne admin, creando un account. Altrimenti, crea un nuovo progetto per rimuovere qualcuno.
member-revoke-admin = Togli il ruolo di admin
member-revoke-admin-hint = Non potrà più cambiare la chiave del progetto né rimuovere nessuno.
member-revoked-admin = { $name } non è più admin.
member-step-down = Smetti di essere admin
member-step-down-hint = Non potrai più cambiare la chiave del progetto né rimuovere nessuno.
member-stepped-down = Non sei più admin.
member-remove = Rimuovi dal progetto
member-remove-hint = Cambia la chiave del progetto: { $name } non potrà leggere niente di nuovo.
member-hide = Nascondi dagli elenchi
error-user-not-admin = Questo membro non è admin.
error-creator-proof-invalid = Questo dispositivo non ha creato questo progetto.
error-project-has-admin = Questo progetto ha già un admin.
error-not-project-admin = Solo chi amministra il progetto può farlo.
error-user-not-claimed = Questa persona non ha un account, quindi non può essere admin.
error-project-key-rotated = La chiave del progetto è cambiata. Chiedi il nuovo link a un membro.
error-project-rotating = La chiave del progetto sta cambiando. Riprova tra un momento.
error-rotation-expired = Questo cambio di chiave ha richiesto troppo tempo ed è stato annullato. Ricomincia.
error-rotation-rows-changed = Il progetto è cambiato durante il cambio di chiave. Ricomincia.
project-rotated-title = Non hai più accesso a questo progetto
project-rotated-body = La chiave di questo progetto è stata cambiata. Chiedi il nuovo link a chi lo amministra.
project-rotated-open-link = Apri un link
rotation-remove-title = Rimuovere { $name } dal progetto?
rotation-rotate-title = Cambiare la chiave del progetto?
rotation-remove-body = Rimuovere un membro comporta la creazione di una nuova chiave. Dovrai condividere il nuovo link con le persone senza account.
rotation-rotate-body = Con il vecchio link non si potrà più accedere al progetto. Dovrai condividere il nuovo link con le persone senza account.
rotation-boxed =
    { $count ->
        [0] Nessun altro membro riceverà la chiave automaticamente
        [one] { $count } membro riceverà la chiave automaticamente
       *[other] { $count } membri riceveranno la chiave automaticamente
    }
rotation-by-link-title = Persone a cui dovrai inviare il nuovo link
rotation-by-link-hint = Queste persone non hanno un account. Invierai loro il link subito dopo.
rotation-names-and = { $first } e { $last }
rotation-names-and-others =
    { $names } e { $count ->
        [one] un’altra persona
       *[other] altre { $count } persone
    }
rotation-confirm-remove = Rimuovi e cambia la chiave
rotation-confirm-rotate = Cambia la chiave
rotation-done-title = Nuova chiave attiva
rotation-done-removed = { $name } non fa più parte del progetto.
rotation-done-rotated = La chiave del progetto è cambiata.
rotation-done-boxed =
    { $count ->
        [0] Nessun altro membro ha ricevuto la chiave automaticamente.
        [one] { $count } membro ha ricevuto la chiave automaticamente.
       *[other] { $count } membri hanno ricevuto la chiave automaticamente.
    }
rotation-new-link =
    { $count ->
        [0] Nuovo link del progetto
        [one] Nuovo link da inviare · { $count } persona senza account
       *[other] Nuovo link da inviare · { $count } persone senza account
    }
rotation-link-renewed = Il link è stato rinnovato.
rotation-done = Fatto
history-key-changed = Chiave del progetto cambiata
history-member-removed = { $name } non fa più parte del progetto
project-rotated-row-hint = La chiave è cambiata. Chiedi il nuovo link.
