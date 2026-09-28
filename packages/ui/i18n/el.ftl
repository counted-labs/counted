# Ελληνικά. Πλήρες, εκτός από τα νομικά κείμενα (legal-, terms-, privacy-), που υπάρχουν μόνο στα
# αγγλικά και τα γαλλικά και για κάθε μήνυμα ξεχωριστά επιστρέφουν στο en.ftl.

### Common

loading = Φόρτωση…
cancel = Ακύρωση
confirm = Επιβεβαίωση
retry = Δοκίμασε ξανά
delete = Διαγραφή
back = Πίσω
language = Γλώσσα

### Navigation

nav-main = Κύρια πλοήγηση
nav-projects = Έργα
nav-charts = Στατιστικά
nav-settings = Ρυθμίσεις

### Connectivity

offline-banner = Εκτός σύνδεσης
offline-pending =
    { $count ->
        [one] { $count } σε εκκρεμότητα
       *[other] { $count } σε εκκρεμότητα
    }

sync-conflict-edit = Σύγκρουση: η επεξεργασία του «{ $name }» απέτυχε (το στοιχείο διαγράφηκε). Παραλείφθηκε.
sync-conflict-delete = Σύγκρουση: η διαγραφή του «{ $name }» απέτυχε (το στοιχείο διαγράφηκε). Παραλείφθηκε.
sync-conflict-other = Σύγκρουση: η ενέργεια στο «{ $name }» απέτυχε (το στοιχείο διαγράφηκε). Παραλείφθηκε.
sync-error = Σφάλμα συγχρονισμού: { $reason }

### Errors

error-network = Δεν είναι δυνατή η σύνδεση με τον διακομιστή. Έλεγξε τη σύνδεσή σου στο διαδίκτυο.
error-generic = Κάτι πήγε στραβά. Δοκίμασε ξανά.

error-invalid-email = Αυτή η διεύθυνση email δεν είναι έγκυρη.
error-invalid-password = Αυτός ο κωδικός δεν είναι έγκυρος.
error-password-too-short = Ο κωδικός πρέπει να έχει τουλάχιστον 8 χαρακτήρες.
error-client-outdated = Αυτή η έκδοση της εφαρμογής είναι παλιά. Ενημέρωσέ την για να συνδεθείς.
error-invalid-link = Αυτός ο σύνδεσμος δεν είναι έγκυρος.
error-batch-too-large = Πάρα πολλά στοιχεία ταυτόχρονα.
error-payers-required = Επίλεξε τουλάχιστον έναν πληρωτή.
error-debtors-required = Επίλεξε τουλάχιστον ένα άτομο που χρωστάει.
error-duplicate-participant = Ένας συμμετέχων εμφανίζεται δύο φορές στην ίδια πλευρά.
error-participant-not-in-project = Αυτός ο συμμετέχων δεν ανήκει σε αυτό το έργο.
error-too-many-participants = Πάρα πολλοί συμμετέχοντες για μία δαπάνη.
error-invalid-credentials = Λάθος email ή κωδικός.
error-unauthenticated = Συνδέσου για να το κάνεις αυτό.
error-email-not-verified = Η διεύθυνση email σου δεν έχει επαληθευτεί ακόμα.
error-project-not-found = Αυτό το έργο δεν υπάρχει πια.
error-expense-not-found = Αυτή η δαπάνη δεν υπάρχει πια.
error-user-not-found = Αυτός ο συμμετέχων δεν υπάρχει πια.
error-tricount-not-found = Το Tricount δεν βρέθηκε ή το API του επέστρεψε σφάλμα.
error-too-many-members = Αυτό το έργο έφτασε το όριο μελών.
error-identity-taken = Ένας άλλος λογαριασμός έχει ήδη διεκδικήσει αυτόν τον συμμετέχοντα.
error-claim-proof-invalid = Αυτή η συσκευή δεν έχει το κλειδί του έργου, οπότε δεν μπορεί να διεκδικήσει συμμετέχοντα. Άνοιξε ξανά τον σύνδεσμο κοινής χρήσης.
error-user-has-payments = Αυτός ο συμμετέχων έχει δαπάνες στο έργο και δεν μπορεί να αφαιρεθεί.
error-resend-cooldown = Περίμενε 60 δευτερόλεπτα πριν ζητήσεις νέο email.
error-self-friend-request = Δεν μπορείς να προσθέσεις τον εαυτό σου ως φίλο.
error-not-a-friend = Μπορείς να προσκαλέσεις μόνο άτομα από τη λίστα φίλων σου.
error-friend-has-no-key = Αυτός ο φίλος δεν έχει ανοίξει ακόμα την τελευταία έκδοση της εφαρμογής. Ζήτησέ του να συνδεθεί μία φορά και δοκίμασε ξανά.
error-friend-request-not-found = Αυτό το αίτημα φιλίας δεν υπάρχει πια.
error-invitation-not-found = Αυτή η πρόσκληση δεν υπάρχει πια.
error-too-many-friend-requests = Πάρα πολλά αιτήματα φιλίας προς το παρόν. Δοκίμασε αύριο.
error-too-many-invitations = Πάρα πολλές προσκλήσεις σε εκκρεμότητα.
error-invalid-kdf-salt = Οι ρυθμίσεις κρυπτογράφησης δεν είναι έγκυρες. Ενημέρωσε την εφαρμογή και δοκίμασε ξανά.
error-mixed-project-batch = Αυτά τα άτομα δεν ανήκουν όλα στο ίδιο έργο.
error-invalid-payload = Αυτή η έκδοση της εφαρμογής έστειλε δεδομένα που ο διακομιστής δεν δέχεται. Ενημέρωσέ την και δοκίμασε ξανά.
error-invalid-public-key = Το κλειδί κρυπτογράφησής σου δεν είναι έγκυρο. Ενημέρωσε την εφαρμογή και δοκίμασε ξανά.
error-payment-methods-stale = Τα στοιχεία πληρωμής σου άλλαξαν σε άλλη συσκευή. Κάνε ανανέωση και δοκίμασε ξανά.

### Auth

field-email = Email
field-email-placeholder = esy@paradeigma.gr
field-password = Κωδικός
field-name = Όνομα
field-name-placeholder = Μαρία Παπαδοπούλου

login-title = Σύνδεση
login-submit = Σύνδεση
login-submitting = Σύνδεση…
login-password-placeholder = Ο κωδικός σου
login-no-account = Δεν έχεις λογαριασμό ακόμα;
login-unverified = Η διεύθυνση email σου δεν έχει επαληθευτεί ακόμα. Έλεγξε τα εισερχόμενά σου ή στείλε ξανά τον σύνδεσμο.
login-resend = Αποστολή του συνδέσμου επαλήθευσης ξανά
login-resending = Αποστολή…
login-resend-sent = Το email στάλθηκε - έλεγξε τα εισερχόμενά σου.

register-submit = Δημιουργία λογαριασμού
register-submitting = Δημιουργία…
register-have-account = Έχεις ήδη λογαριασμό;
register-password-placeholder = Τουλάχιστον 8 χαρακτήρες
register-password-warning = Σημείωσε τον κωδικό σου. Αν τον ξεχάσεις, ο λογαριασμός σου δεν μπορεί να ανακτηθεί.
register-check-email-title = Έλεγξε το email σου
register-email-sent = Το email στάλθηκε
register-email-sent-hint = Πάτησε τον σύνδεσμο στα εισερχόμενά σου για να ενεργοποιήσεις τον λογαριασμό σου.
register-not-received-prefix = Δεν το έλαβες; Έλεγξε τον φάκελο ανεπιθύμητων ή
register-sign-in-link = συνδέσου
register-not-received-suffix = για να σταλεί ξανά ο σύνδεσμος.
register-terms-prefix = Δημιουργώντας λογαριασμό αποδέχεσαι τους
register-terms-link = όρους χρήσης
register-terms-and = και την
register-privacy-link = πολιτική απορρήτου μας

settings-title = Ρυθμίσεις
settings-preferences = Προτιμήσεις
settings-preferences-local = Αποθηκεύονται σε αυτήν τη συσκευή.
settings-preferences-synced = Συγχρονισμένες με τον λογαριασμό σου, κρυπτογραφημένες.
settings-about = Σχετικά
settings-anonymous-title = Δεν είσαι συνδεδεμένος
settings-upsell-title = Τα έργα σου, σε κάθε συσκευή
settings-upsell-free = Δωρεάν
settings-upsell-body = Το Counted δουλεύει χωρίς λογαριασμό. Με έναν δωρεάν λογαριασμό, τα έργα και οι προτιμήσεις σου σε ακολουθούν στο κινητό, στο laptop και στον ιστό - πάντα κρυπτογραφημένα, πάντα αδιάβαστα για εμάς.
settings-locked-badge = Λογαριασμός
settings-locked-friends = Δημιούργησε λογαριασμό για να προσθέτεις φίλους και να τους προσκαλείς σε ένα έργο μέσα από την εφαρμογή - χωρίς σύνδεσμο για μοίρασμα.
settings-locked-payment-methods = Αποθήκευσε το IBAN ή την εφαρμογή πληρωμών σου μία φορά και μοιράσου τα με τα έργα που επιλέγεις. Όποιος σου χρωστάει τα βλέπει δίπλα στο όνομά σου.
settings-friends-hint = Πρόσθεσε φίλους και προσκάλεσέ τους στα έργα σου χωρίς να μοιράζεσαι σύνδεσμο.

account-member-since = Μέλος από
account-logout = Αποσύνδεση
account-logging-out = Αποσύνδεση…
account-delete-title = Διαγραφή του λογαριασμού μου
account-delete-warning = Άμεση και οριστική, χωρίς κάδο ανακύκλωσης. Οι δαπάνες που καταχώρισες σε κοινό έργο παραμένουν ορατές στα άλλα μέλη - είναι μέρος των λογαριασμών τους.
account-delete-confirm-title = Διαγραφή λογαριασμού
account-delete-confirm-message = Ο λογαριασμός σου, οι συνεδρίες σου και η λίστα έργων σου θα διαγραφούν οριστικά. Χωρίς τον κωδικό σου, τα κρυπτογραφημένα δεδομένα ενός κοινού έργου γίνονται αδιάβαστα για σένα - αυτό δεν αναιρείται.

settings-payment-methods = Στοιχεία πληρωμής
settings-payment-methods-hint = Πώς θέλεις να σε αποπληρώνουν. Κρυπτογραφημένα με τον λογαριασμό σου.
payment-method-kind = Μέθοδος
payment-method-kind-other = Άλλο
payment-method-label = Όνομα
payment-method-label-placeholder = Κύριος λογαριασμός
payment-method-value = Στοιχεία
payment-method-value-placeholder = IBAN, αριθμός τηλεφώνου, όνομα χρήστη…
payment-method-add = Προσθήκη
payment-method-remove = Αφαίρεση { $name }
payment-method-empty = Δεν έχεις προσθέσει στοιχεία πληρωμής ακόμα.
payment-method-deleted = Η μέθοδος πληρωμής διαγράφηκε.
payment-method-value-required = Συμπλήρωσε τα στοιχεία κάθε μεθόδου πληρωμής ή αφαίρεσέ την.
payment-method-label-required = Δώσε όνομα στην προσαρμοσμένη μέθοδό σου.
payment-method-too-long = Είναι πολύ μεγάλο - συντόμευσέ το.
payment-method-invalid-characters = Αφαίρεσε αλλαγές γραμμής ή αόρατους χαρακτήρες.
payment-method-limit = Μπορείς να αποθηκεύσεις έως { $max } μεθόδους πληρωμής.
payment-methods-saved = Τα στοιχεία πληρωμής αποθηκεύτηκαν.
payment-methods-offline = Πρέπει να είσαι συνδεδεμένος για να αποθηκεύσεις τα στοιχεία πληρωμής.
payment-methods-stale = Τα στοιχεία πληρωμής σου άλλαξαν σε άλλη συσκευή. Φορτώθηκαν ξανά — δοκίμασε ξανά.
payment-methods-key-missing = Συνδέσου ξανά για να διαχειριστείς τα στοιχεία πληρωμής σου.
settings-payment-methods-share-warning = Μια κοινόχρηστη μέθοδος είναι ορατή σε κάθε μέλος των έργων όπου έχεις επιλέξει το όνομά σου - σε όποιον έχει έναν από αυτούς τους συνδέσμους.
payment-method-share = Κοινή χρήση με τα έργα μου
payment-method-share-hint = Εμφανίζεται δίπλα στο όνομά σου όταν κάποιος σου χρωστάει.
payment-method-copy = Αντιγραφή { $name }
payment-method-copied = Αντιγράφηκε.
payment-method-copy-failed = Δεν ήταν δυνατή η αντιγραφή - επίλεξε το κείμενο και αντέγραψέ το με το χέρι.

verify-email-checking = Επαλήθευση της διεύθυνσης email σου…
verify-email-welcome = Το email επαληθεύτηκε - καλώς ήρθες στο Counted!
verify-email-back-to-login = Πίσω στη σύνδεση

### Project status

project-close = Κλείσιμο
project-archive = Αρχειοθέτηση
project-reopen = Άνοιγμα ξανά
project-unarchive = Επαναφορά από αρχείο

### Dates

date-long = { $day } { $month } { $year }

month-1 = Ιανουαρίου
month-2 = Φεβρουαρίου
month-3 = Μαρτίου
month-4 = Απριλίου
month-5 = Μαΐου
month-6 = Ιουνίου
month-7 = Ιουλίου
month-8 = Αυγούστου
month-9 = Σεπτεμβρίου
month-10 = Οκτωβρίου
month-11 = Νοεμβρίου
month-12 = Δεκεμβρίου

month-short-1 = Ιαν
month-short-2 = Φεβ
month-short-3 = Μαρ
month-short-4 = Απρ
month-short-5 = Μάι
month-short-6 = Ιουν
month-short-7 = Ιουλ
month-short-8 = Αυγ
month-short-9 = Σεπ
month-short-10 = Οκτ
month-short-11 = Νοε
month-short-12 = Δεκ

### Actions

add = Προσθήκη
create = Δημιουργία
creating = Δημιουργία…
edit = Επεξεργασία
leave = Αποχώρηση
close = Κλείσιμο
paste = Επικόλληση
join = Συμμετοχή
import = Εισαγωγή
importing = Εισαγωγή…
field-description = Περιγραφή
field-date = Ημερομηνία
date-today = Σήμερα
date-yesterday = Χθες
field-optional = Προαιρετικό

### Projects

projects-filter-active = Ενεργά
projects-filter-all = Όλα
projects-count-label = Έργα
projects-empty = Δεν υπάρχουν έργα
projects-empty-hint = Δημιούργησε ένα έργο με το κουμπί παρακάτω
projects-offline-banner = Δεδομένα εκτός σύνδεσης - συνδέσου ξανά για ανανέωση.
projects-no-local-data = Δεν υπάρχουν τοπικά δεδομένα
projects-no-local-data-hint = Συνδέσου για να φορτώσεις τα έργα σου για πρώτη φορά.
projects-add = Προσθήκη έργου
projects-create = Δημιουργία έργου
projects-join = Συμμετοχή σε έργο
projects-import-tricount = Εισαγωγή από Tricount
project-actions = Ενέργειες έργου

status-ongoing = Σε εξέλιξη
status-closed = Κλειστό
status-archived = Αρχειοθετημένο

nav-help = Βοήθεια
nav-privacy = Πολιτική απορρήτου
nav-terms = Όροι χρήσης
nav-legal = Νομική σημείωση

leave-project-title = Αποχώρηση από το έργο;
leave-project-message = Θα χάσεις την πρόσβαση από αυτήν τη συσκευή. Αν δεν μείνει κανένα μέλος, το έργο και όλες οι δαπάνες του διαγράφονται οριστικά.

add-project-title = Νέο έργο
add-project-name-label = Όνομα έργου
add-project-name-placeholder = Το ταξίδι μου, Συγκάτοικοι 2024…
add-project-participants = Συμμετέχοντες
add-project-participant-name = Όνομα συμμετέχοντα
add-project-participant-placeholder = Κλαρκ Κεντ
add-project-remove-participant = Αφαίρεση συμμετέχοντα
add-project-me-badge = Εγώ
add-project-thats-me = Εγώ είμαι!
add-project-offline = Δεν μπορείς να δημιουργήσεις έργο εκτός σύνδεσης. Συνδέσου ξανά και δοκίμασε πάλι.
add-project-name-required = Το έργο χρειάζεται όνομα.
add-project-need-two-participants = Πρόσθεσε τουλάχιστον 2 συμμετέχοντες.
add-project-pick-yourself = Πες μας ποιος συμμετέχων είσαι.

join-link-label = Σύνδεσμος κοινής χρήσης
join-link-hint = Ο σύνδεσμος περιέχει το κλειδί αποκρυπτογράφησης - αντέγραψέ τον ολόκληρο.
join-invalid-link = Αυτός ο σύνδεσμος δεν είναι έγκυρος. Επικόλλησε ολόκληρο τον σύνδεσμο κοινής χρήσης, μαζί με το τμήμα μετά το #.
join-wrong-project = Αυτός ο σύνδεσμος αφορά άλλο έργο.

import-tricount-link-label = Σύνδεσμος ή κλειδί Tricount
import-tricount-key-required = Εισήγαγε έναν σύνδεσμο ή κλειδί Tricount.
import-tricount-encryption-failed = Η κρυπτογράφηση απέτυχε.

### Expenses

save = Αποθήκευση
saving = Αποθήκευση…
adding = Προσθήκη…
link-copied = Ο σύνδεσμος αντιγράφηκε
missing-encryption-key = Λείπει το κλειδί κρυπτογράφησης.
missing-encryption-key-title = Λείπει το κλειδί κρυπτογράφησης
missing-encryption-key-hint = Ο σύνδεσμος που χρησιμοποίησες δεν περιέχει το κλειδί που χρειάζεται για την αποκρυπτογράφηση αυτού του έργου. Χρησιμοποίησε τον πλήρη σύνδεσμο που μοιράστηκε όποιος το δημιούργησε.
project-locked-hint = Αυτή η συσκευή δεν έχει το κλειδί αυτού του έργου. Άνοιξε τον σύνδεσμο κοινής χρήσης του για να το ξεκλειδώσεις.
project-unlock = Ξεκλείδωμα
project-no-local-data-hint = Συνδέσου για να φορτώσεις τα δεδομένα αυτού του έργου για πρώτη φορά.
project-gone-title = Αυτό το έργο δεν υπάρχει πια
project-gone-hint = Διαγράφηκε όταν αποχώρησε το τελευταίο του μέλος. Ο σύνδεσμος κοινής χρήσης δεν λειτουργεί πια, ακόμα κι αν τον ανοίξεις ξανά.

expense-add = Προσθήκη δαπάνης
transfer-add = Προσθήκη μεταφοράς
expense-edit-title = Επεξεργασία δαπάνης
expense-category = Κατηγορία
expense-category-auto = Αυτόματα · { $emoji }
expense-currency = Νόμισμα του ποσού
amount-op-add = Συν
amount-op-subtract = Πλην
amount-op-multiply = Επί
amount-op-divide = Διά
amount-op-equals = Ίσον
amount-op-done = Τέλος
expense-rate = Ισοτιμία (προαιρετικό)
expense-rate-hint = Άφησέ το κενό για την ισοτιμία της Ευρωπαϊκής Επιτροπής (InforEuro) για { $month }: 1 { $from } = { $rate } { $to }.
expense-rate-invalid = Εισήγαγε ισοτιμία μεγαλύτερη από 0.
expense-rate-unavailable = Δεν υπάρχει αυτόματη ισοτιμία - εισήγαγέ την με το χέρι.
expense-delete-title = Διαγραφή δαπάνης
expense-delete-message = Το «{ $name }» θα διαγραφεί οριστικά. Αυτό δεν αναιρείται.
expense-inconsistent-amounts = Τα ποσά δεν συμφωνούν
expenses-empty = Δεν υπάρχουν δαπάνες
expenses-empty-hint = Ξεκίνα προσθέτοντας δαπάνες με το κουμπί παρακάτω
expenses-show-more = Εμφάνιση περισσότερων ({ $count } ακόμα)

expense-type-expense = Δαπάνη
expense-type-transfer = Μεταφορά
expense-type-gain = Έσοδο
expense-paid-by = πληρώθηκε από
expense-sent-by = στάλθηκε από
expense-contributed-by = συνεισφορά από

expense-name-required = Το όνομα είναι υποχρεωτικό.
expense-amount-not-positive = Το ποσό πρέπει να είναι μεγαλύτερο από 0.
expense-no-payer = Επίλεξε τουλάχιστον έναν πληρωτή.
expense-no-debtor = Επίλεξε τουλάχιστον ένα άτομο που χρωστάει.
expense-invalid-date = Αυτή η ημερομηνία δεν είναι έγκυρη.
expense-payers-mismatch = Το σύνολο των πληρωτών είναι { $sum }, που δεν συμφωνεί με το ποσό της δαπάνης ({ $total }).
expense-debtors-mismatch = Το σύνολο των οφειλετών είναι { $sum }, που δεν συμφωνεί με το ποσό της δαπάνης ({ $total }).

participants-none = Κανείς
participants-everyone = Όλοι ({ $count })
participants-some = { $count } από { $total }
participants-select-all = Επιλογή όλων
participants-by-shares = Ανά μερίδια
split-amounts = Ποσά
participants-remaining = Απομένουν { $amount }
participants-over-by = { $amount } επιπλέον
participants-who-paid = Ποιος πλήρωσε;
participants-who-received = Ποιος έλαβε;
participants-who-transfers = Ποιος μεταφέρει;
participants-who-receives = Ποιος λαμβάνει;
participants-for-whom = Για ποιον;

stats-total-expenses = Σύνολο δαπανών
stats-my-expenses = Οι δαπάνες μου

tab-expenses = Δαπάνες
tab-balance = Υπόλοιπο
tab-reimbursements = Εξόφληση
reimbursements-empty-title = Όλα εξοφλημένα!
reimbursements-empty-hint = Οι προτάσεις εξόφλησης εμφανίζονται εδώ όταν οι λογαριασμοί δεν ισορροπούν
reimbursement-owes = { $debtor } χρωστάει σε { $creditor }
reimbursement-record = Εξόφληση
reimbursement-pay-with = Πληρωμή
reimbursement-pay-shared-by = Κοινοποιήθηκε από { $name } - έλεγξε το όνομα παραλήπτη που δείχνει η εφαρμογή σου πριν στείλεις.
reimbursement-pay-title = Πληρωμή σε { $name }
reimbursements-mine-title = Χρωστάς
reimbursements-others-title = Άλλες αποπληρωμές
copy = Αντιγραφή

user-selection-title = Ποιος συμμετέχων είσαι;
user-selection-hint = Διάλεξε το όνομά σου από τη λίστα.
user-selection-required = Επίλεξε έναν συμμετέχοντα.
identity-claimed = Συνδεδεμένο με λογαριασμό
identity-claimed-by = Λογαριασμός του { $name }
identity-taken-repick = Ένας άλλος λογαριασμός διεκδίκησε τον συμμετέχοντα που χρησιμοποιούσες. Διάλεξε άλλον.
participant-gone-repick = Ο συμμετέχων που χρησιμοποιούσες αφαιρέθηκε από αυτό το έργο. Διάλεξε άλλον.

edit-project-title = Επεξεργασία έργου
edit-project-new-badge = νέο
edit-project-deferred-new-members = η προσθήκη νέων μελών
edit-project-deferred-removals = η αφαίρεση μελών
edit-project-deferred-me = η επιλογή «Εγώ είμαι»
edit-project-offline-deferred = Εκτός σύνδεσης: { $items } θα εφαρμοστεί όταν συνδεθείς ξανά.

export-saved = Το αρχείο αποθηκεύτηκε:
    { $path }
export-failed = Η εξαγωγή απέτυχε: { $reason }

history-expense-added = Προστέθηκε δαπάνη: { $name }
history-expense-edited = Επεξεργάστηκε δαπάνη: { $name }
history-expense-deleted = Διαγράφηκε δαπάνη: { $name }
history-project-edited = Επεξεργάστηκε έργο: { $name }
history-name-changed = Όνομα: «{ $from }» → «{ $to }»
history-description-added = Προστέθηκε περιγραφή: «{ $value }»
history-description-removed = Αφαιρέθηκε περιγραφή: «{ $value }»
history-description-changed = Περιγραφή: «{ $from }» → «{ $to }»

### Sweep

field-amount = Ποσό
expense-name-placeholder = Εστιατόριο, ψώνια…
expense-actions = Ενέργειες δαπάνης
expense-your-share = Το μερίδιό σου
expense-your-share-value = Το μερίδιό σου: { $amount } { $currency }
expense-inconsistent-detail = Τα ποσά δεν συμφωνούν: { $paid } πληρώθηκαν, { $owed } οφείλονται, για δαπάνη { $total }. Επεξεργάσου τη δαπάνη για να το διορθώσεις.
missing-access-key = Λείπει το κλειδί πρόσβασης. Άνοιξε αυτό το έργο μέσω του συνδέσμου κοινής χρήσης του.
filter-all = Όλα
filter-my-payments = Οι πληρωμές μου
filter-my-debts = Τι χρωστάω
participants-shares-for = Μερίδια για { $name }
participants-amount-for = Ποσό για { $name }
reimbursement-add = Προσθήκη εξόφλησης
project-forget = Αφαίρεση από τη λίστα μου
project-history-title = Ιστορικό
history-kind-add = Προσθήκη
history-kind-delete = Διαγραφή
history-kind-edit = Επεξεργασία
export = Εξαγωγή
export-json = Εξαγωγή JSON
export-csv = Εξαγωγή CSV
share-link = Κοινοποίηση
copy-link-failed = Δεν ήταν δυνατή η αντιγραφή του συνδέσμου
open-in-app = Άνοιγμα στην εφαρμογή
not-found-title = Η σελίδα δεν βρέθηκε
not-found-back = Πίσω στα έργα

### Charts

charts-period = Περίοδος
period-all = Όλα
period-month = Μήνας
period-3months = 3 μήνες
period-year = Έτος
period-custom = Προσαρμογή
charts-tab-categories = Κατηγορίες
charts-tab-trends = Τάσεις
charts-total-spent = Συνολικά έξοδα
charts-avg-per-person = Μ.Ο. ανά άτομο
charts-expense-count =
    { $count ->
        [one] { $count } δαπάνη
       *[other] { $count } δαπάνες
    }
charts-nothing-to-show = Τίποτα προς εμφάνιση
charts-my-share-note = Αυτά τα ποσά είναι το μερίδιό σου σε κάθε δαπάνη.
charts-my-share-skipped =
    { $count ->
        [one] 1 έργο δεν υπολογίζεται — δεν επιλέχθηκε συμμετέχων ή τα δεδομένα του δεν φορτώθηκαν.
       *[other] { $count } έργα δεν υπολογίζονται — δεν επιλέχθηκε συμμετέχων ή τα δεδομένα τους δεν φορτώθηκαν.
    }

### Categories

category-food = Φαγητό
category-transport = Μεταφορές
category-accommodation = Διαμονή
category-leisure = Ψυχαγωγία
category-shopping = Ψώνια
category-services = Υπηρεσίες
category-parties-gifts = Πάρτι & δώρα
category-other = Άλλο
charts-project = Έργο
charts-all-projects = Όλα τα έργα
charts-date-from = Από
charts-date-to = Έως
charts-total = Σύνολο
charts-tab-people = Άτομα
charts-tab-projects = Έργα
charts-scope = Ποιανού έξοδα
charts-scope-group = Ομάδα
charts-scope-me = Εγώ
charts-currency = Νόμισμα
charts-my-share = Το μερίδιό μου
charts-share-of-total = { $pct }% του { $total }
charts-i-paid = Πλήρωσα
charts-paid-more = { $amount } περισσότερα από το μερίδιό σου
charts-paid-less = { $amount } λιγότερα από το μερίδιό σου
charts-paid-even = Ακριβώς το μερίδιό σου
charts-part-title = Το μερίδιό σου σε κάθε κατηγορία
charts-part-desc = Γκρι όσα ξόδεψε η ομάδα, χρώμα όσα κατανάλωσες εσύ.
charts-breakdown-title = Ανάλυση ανά κατηγορία
charts-breakdown-desc = Πάτησε ένα κομμάτι ή μια γραμμή για να δεις τα έξοδα.
charts-of-total = { $amount } από { $total }
charts-show-all = Εμφάνιση όλων ({ $count })
charts-show-less = Εμφάνιση λιγότερων
charts-spend-title = Έξοδα στον χρόνο
charts-spend-desc = Οι σύντομες περίοδοι ανά ημέρα, οι μεγαλύτερες ανά εβδομάδα ή μήνα.
charts-group-by = Ομαδοποίηση ανά
bucket-day = Ημέρα
bucket-week = Εβδομάδα
bucket-month = Μήνας
charts-avg = μ.ό.
charts-cat-title-day = { $category }, μέρα με τη μέρα
charts-cat-title-week = { $category }, εβδομάδα με την εβδομάδα
charts-cat-title-month = { $category }, μήνα με τον μήνα
charts-cat-desc = Διάλεξε μια κατηγορία για να την παρακολουθείς στον χρόνο.
charts-running-title = Σωρευτικό σύνολο
charts-running-desc = Από { $date }.
charts-avg-per-day = { $amount } / ημέρα κατά μέσο όρο
charts-avg-per-week = { $amount } / εβδομάδα κατά μέσο όρο
charts-avg-per-month = { $amount } / μήνα κατά μέσο όρο
charts-people-title = Ποιος σήκωσε την ομάδα
charts-people-desc = Τι πλήρωσε ο καθένας, δίπλα σε ό,τι κατανάλωσε.
charts-paid = Πληρωμένα
charts-fair-share = Δίκαιο μερίδιο
charts-you = (εσύ)
charts-net-more = πλήρωσε περισσότερα
charts-net-less = πλήρωσε λιγότερα
charts-balance-title = Το υπόλοιπό σου στον χρόνο
charts-balance-desc = Πάνω από τη γραμμή η ομάδα σού χρωστά. Κάτω από αυτήν χρωστάς εσύ στην ομάδα.
charts-owed = Σου χρωστούν
charts-owe = Χρωστάς
charts-projects-title = Το μερίδιό σου ανά έργο
charts-projects-desc = Τα σύνολα κρατιούνται ανά νόμισμα και δεν αθροίζονται ποτέ μεταξύ τους.
history-empty = Δεν υπάρχουν συμβάντα
history-by = Από { $name }
not-found-hint = Αυτή η σελίδα δεν υπάρχει ή μετακινήθηκε.
payers-title-paid-by = Πληρώθηκε από
payers-title-sender = Αποστολέας
payers-title-contributors = Συνεισφέροντες
debtors-title-debtors = Χρωστούν
debtors-title-recipients = Παραλήπτες
debtors-title-beneficiaries = Δικαιούχοι

### Welcome

welcome-title = Οι λογαριασμοί σου δεν αφορούν κανέναν άλλο.
welcome-subtitle = Μοιράσου έξοδα με φίλους.
welcome-e2ee-title = Όλα κρυπτογραφημένα
welcome-e2ee-body = Ονόματα, ποσά, έργα: όλα κρυπτογραφούνται στη συσκευή σου. Μόνο εσύ έχεις το κλειδί. Κανείς δεν μπορεί να διαβάσει τους λογαριασμούς σου. Ούτε εμείς.
welcome-e2ee-note = Αδιάβαστα ακόμα και για εμάς (μηδενική πρόσβαση διακομιστή)
welcome-eu-title = 100% ευρωπαϊκό
welcome-eu-body = Διακομιστές στη Γερμανία, email από τη Γαλλία. Τα δεδομένα σου δεν φεύγουν ποτέ από την Ευρωπαϊκή Ένωση.
welcome-noads-title = Χωρίς διαφημίσεις. Χωρίς ιχνηλάτες.
welcome-noads-body = Δεν συλλέγουμε τίποτα και δεν πουλάμε τα δεδομένα σου. Δεν είναι αυτό το μοντέλο μας.
welcome-start = Ξεκίνα
welcome-how-it-works = Πώς ακριβώς λειτουργεί;

### Help

help-intro = Συχνή ερώτηση; Πάτησε για να δεις την απάντηση.
help-create-project-q = Πώς δημιουργώ ένα έργο;
help-create-project-a = Από την αρχική οθόνη, πάτησε το κουμπί + στο κάτω μέρος. Δώσε όνομα στο έργο, διάλεξε νόμισμα και είσαι έτοιμος.
help-add-participants-q = Πώς προσθέτω συμμετέχοντες;
help-add-participants-a = Άνοιξε το έργο και πρόσθεσε συμμετέχοντες από τη λίστα μελών. Κάθε συμμετέχων μπορεί να πληρώσει ή να χρωστάει σε μια δαπάνη.
help-share-project-q = Πώς μοιράζομαι ένα έργο;
help-share-project-a = Μοιράσου το URL του έργου (αυτό στη γραμμή διευθύνσεων). Όποιος έχει τον σύνδεσμο μπορεί να δει και να επεξεργαστεί το έργο.
help-add-expense-q = Πώς προσθέτω μια δαπάνη;
help-add-expense-a = Μέσα σε ένα έργο, πάτησε +, βάλε το ποσό, πες ποιος πλήρωσε και μεταξύ ποιων μοιράζεται. Μπορείς επίσης να διαλέξεις άλλη ημερομηνία από τη σημερινή.
help-types-q = Ποια είναι η διαφορά μεταξύ δαπάνης, μεταφοράς και εσόδου;
help-types-expense = - μια αγορά από ένα άτομο που μοιράζεται σε πολλά.
help-types-transfer = - μια αποπληρωμή από ένα άτομο σε άλλο, χωρίς μοιρασιά.
help-types-gain = - χρήματα που ελήφθησαν (επιστροφή, δώρο) για να μοιραστούν σε πολλά άτομα.
help-past-date-q = Μπορώ να βάλω παλαιότερη ημερομηνία σε μια δαπάνη;
help-past-date-a = Ναι, το πεδίο ημερομηνίας είναι ελεύθερο. Η ώρα δημιουργίας της εγγραφής κρατιέται ξεχωριστά.
help-who-owes-q = Πώς υπολογίζει το Counted ποιος χρωστάει τι;
help-who-owes-a = Το Counted υπολογίζει το καθαρό υπόλοιπο κάθε συμμετέχοντα (όσα προκατέβαλε μείον όσα χρωστάει) και προτείνει τη συντομότερη σειρά μεταφορών που εξοφλεί τους πάντες.
help-minimal-transfers-q = Γιατί ο αριθμός των προτεινόμενων μεταφορών είναι ελάχιστος;
help-minimal-transfers-a = Ο αλγόριθμος ταιριάζει πρώτα υπόλοιπα που αλληλοεξουδετερώνονται ακριβώς και μετά επεξεργάζεται τα υπόλοιπα από τον μεγαλύτερο πιστωτή προς τον μεγαλύτερο οφειλέτη. Αποτέλεσμα: λιγότερες μεταφορές για να εξοφληθούν όλα.
help-import-tricount-q = Πώς εισάγω ένα έργο από το Tricount;
help-import-tricount-a = Από την αρχική οθόνη, πάτησε το κουμπί «+» στο κάτω μέρος και μετά
help-import-tricount-b = Επικόλλησε τον σύνδεσμο κοινής χρήσης του Tricount που θέλεις να εισάγεις.
help-encryption-q = Είναι κρυπτογραφημένα τα δεδομένα μου;
help-encryption-a = Ναι. Το Counted συνδυάζει δύο εγγυήσεις:
help-encryption-e2ee-term = Κρυπτογράφηση από άκρο σε άκρο
help-encryption-e2ee-def = - όλα ανάμεσα σε σένα και τον διακομιστή ταξιδεύουν κρυπτογραφημένα.
help-encryption-zero-term = Μηδενική πρόσβαση
help-encryption-zero-def = - κρυπτογραφείς τα δεδομένα πριν τα στείλεις και ο διακομιστής αποθηκεύει μόνο κρυπτογραφημένο κείμενο. Δεν έχουμε τρόπο να το διαβάσουμε.
help-encryption-see = Για λεπτομέρειες, δες την
help-forgot-password-q = Τι γίνεται αν ξεχάσω τον κωδικό μου;
help-forgot-password-warning = Τα δεδομένα σου θα χαθούν οριστικά.
help-forgot-password-a = Το κλειδί κρυπτογράφησης προκύπτει από τον κωδικό σου, οπότε δεν είναι δυνατή επαναφορά: κανείς - ούτε εμείς - δεν μπορεί να αποκρυπτογραφήσει τα έργα σου χωρίς αυτόν. Φύλαξέ τον με ασφάλεια, ιδανικά σε διαχειριστή κωδικών.
help-archive-delete-q = Πώς αρχειοθετώ ή διαγράφω ένα έργο;
help-archive-delete-a = Από την οθόνη του έργου, άνοιξε το μενού και διάλεξε
help-archive-delete-b = για να το κρύψεις χωρίς να το χάσεις. Ένα έργο διαγράφεται οριστικά όταν αποχωρήσει το τελευταίο μέλος του.
help-delete-account-q = Πώς διαγράφω τον λογαριασμό μου;
help-delete-account-a = Άνοιξε τις Ρυθμίσεις και χρησιμοποίησε το «Διαγραφή του λογαριασμού μου». Είναι άμεσο και δεν αναιρείται.
help-contact = Άλλη ερώτηση; Γράψε μας στο

# Receipt scanning (mobile only)
expense-scan = Σάρωση απόδειξης
scan-in-progress = Ανάγνωση της απόδειξης…
scan-error-capture = Δεν ήταν δυνατή η λήψη της φωτογραφίας. Δοκίμασε ξανά ή καταχώρισε τη δαπάνη με το χέρι.
scan-error-unreadable = Τίποτα αναγνώσιμο σε αυτήν την απόδειξη. Καταχώρισε τη δαπάνη με το χέρι.
scan-check-amount = Έλεγξε το σύνολο - δεν ήταν καθαρά τυπωμένο.
scan-take-photo = Λήψη φωτογραφίας
scan-choose-photo = Επιλογή φωτογραφίας
expense-converted-from = Πληρώθηκαν { $amount } { $from } · 1 { $from } = { $rate } { $to }
project-currency = Νόμισμα
project-currency-hint = Κάθε ποσό εμφανίζεται σε αυτό το νόμισμα. Δεν μπορεί να αλλάξει αργότερα.
project-currency-locked = Το νόμισμα ορίζεται κατά τη δημιουργία του έργου.

update-required-title = Απαιτείται ενημέρωση
update-required-body = Αυτή η έκδοση του Counted είναι πολύ παλιά για να επικοινωνήσει με τον διακομιστή. Ενημέρωσέ την για να συνεχίσεις να χρησιμοποιείς την εφαρμογή.
update-required-button = Ενημέρωση

notifications-label = Ειδοποιήσεις
notifications-title = Ειδοποιήσεις
notifications-empty = Τίποτα νέο
notifications-friend-request = Αίτημα φιλίας

friends-title = Φίλοι
friends-anonymous-body = Οι φίλοι φυλάσσονται με τον λογαριασμό σου. Συνδέσου για να προσθέσεις άτομα και να τα προσκαλέσεις στα έργα σου χωρίς να μοιραστείς σύνδεσμο.
friends-add-title = Προσθήκη φίλου
friends-add-hint = Θα δει το αίτημά σου μόλις συνδεθεί. Κανείς από τους δύο δεν μαθαίνει αν ο άλλος έχει λογαριασμό μέχρι να γίνει αποδεκτό το αίτημα.
friends-add-button = Προσθήκη
friends-add-from-project = Προσθήκη ως φίλου
friends-request-sent = Το αίτημα στάλθηκε
friends-no-account-key = Συνδέσου ξανά για να διαχειριστείς τους φίλους σου σε αυτήν τη συσκευή.
friends-incoming-title = Αιτήματα
friends-accept = Αποδοχή
friends-decline = Απόρριψη
friends-list-title = Οι φίλοι μου
friends-list-empty = Δεν υπάρχουν φίλοι ακόμα. Πρόσθεσε κάποιον με email παραπάνω ή από ένα κοινό έργο.
friends-remove = Αφαίρεση
friends-remove-confirm-title = Αφαίρεση φίλου
friends-remove-confirm-message = Ο/Η { $email } δεν θα είναι πια στους φίλους σας, ούτε εσείς στους δικούς του. Οποιοσδήποτε από τους δύο μπορεί να στείλει νέο αίτημα αργότερα.
friends-no-key = Δεν είναι έτοιμο ακόμα
friends-fingerprint = Κωδικός ασφαλείας
friends-fingerprint-hint = Δύο φίλοι που διαβάζουν ο ένας στον άλλο τον ίδιο κωδικό ασφαλείας ξέρουν ότι κανείς δεν βρίσκεται ανάμεσά τους - ούτε καν ο διακομιστής μας.
friends-outgoing-title = Απεσταλμένα
friends-outgoing-hint = Σε αναμονή απάντησης. Θα τους δεις στους φίλους σου μόλις αποδεχτούν.
friends-withdraw = Ακύρωση
invite-friends-title = Πρόσκληση φίλων
invite-friends-hint = Το κλειδί του έργου κρυπτογραφείται για κάθε φίλο σε αυτήν τη συσκευή. Ο διακομιστής δεν το βλέπει ποτέ.
invite-friends-empty = Δεν υπάρχουν φίλοι για πρόσκληση ακόμα.
invite-friends-button = Πρόσκληση
invite-sent = { $count ->
    [one] Η πρόσκληση στάλθηκε
   *[other] Στάλθηκαν { $count } προσκλήσεις
}
invitation-badge = Πρόσκληση
invitation-to = Συμμετοχή στο «{ $name }»
invitation-to-unnamed = Συμμετοχή σε έργο
invitation-unreadable = Αυτή η πρόσκληση δεν μπορεί να ανοίξει σε αυτήν τη συσκευή
invitation-from = Από { $email }
invitation-accept = Συμμετοχή
invitation-decline = Απόρριψη
