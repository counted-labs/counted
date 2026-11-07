# Ελληνικά. Πλήρες, εκτός από τα νομικά κείμενα (legal-, terms-, privacy-), που υπάρχουν μόνο στα
# αγγλικά και τα γαλλικά και για κάθε μήνυμα ξεχωριστά επιστρέφουν στο en.ftl.

### Common

loading = Φόρτωση…
cancel = Ακύρωση
discard-title = Απόρριψη αλλαγών;
discard-message = Ό,τι έγραψες θα χαθεί.
discard = Απόρριψη
confirm = Επιβεβαίωση
retry = Δοκίμασε ξανά
delete = Διαγραφή
back = Πίσω
language = Γλώσσα
theme = Θέμα
theme-light = Φωτεινό
theme-dark = Σκοτεινό
theme-system = Συστήματος
theme-hint-device = Αποθηκεύεται μόνο σε αυτή τη συσκευή.
theme-hint-system = Ακολουθεί την εμφάνιση αυτής της συσκευής.

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
error-rate-limited = Ο διακομιστής είναι απασχολημένος. Δοκίμασε ξανά σε λίγο.
error-generic = Κάτι πήγε στραβά. Δοκίμασε ξανά.

error-invalid-email = Αυτή η διεύθυνση email δεν είναι έγκυρη.
error-invalid-password = Αυτός ο κωδικός δεν είναι έγκυρος.
error-password-too-short = Ο κωδικός πρέπει να έχει τουλάχιστον 12 χαρακτήρες.
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
error-storage-full = Ο χώρος αποθήκευσης είναι γεμάτος: το κλειδί αυτού του έργου δεν αποθηκεύτηκε σε αυτή τη συσκευή. Κράτα τον σύνδεσμο κοινοποίησης.
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
login-maybe-unverified = Μόλις έκανες εγγραφή; Επιβεβαίωσε πρώτα τη διεύθυνσή σου με τον σύνδεσμο που σου στείλαμε ή στείλ' τον ξανά.
login-resend = Αποστολή του συνδέσμου επαλήθευσης ξανά
login-resending = Αποστολή…
login-resend-sent = Το email στάλθηκε - έλεγξε τα εισερχόμενά σου.

register-submit = Δημιουργία λογαριασμού
register-submitting = Δημιουργία…
register-have-account = Έχεις ήδη λογαριασμό;
register-password-placeholder = Τουλάχιστον 12 χαρακτήρες
register-password-warning = Σημείωσε τον κωδικό σου: δεν τον βλέπουμε ποτέ και δεν μπορούμε να σου τον επιστρέψουμε.
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
account-delete-warning = Άμεση και οριστική διαγραφή του λογαριασμού σου και των δεδομένων σου.
account-delete-confirm-title = Διαγραφή λογαριασμού
account-delete-confirm-message = Ο λογαριασμός σου, οι συνεδρίες σου και η λίστα έργων σου θα διαγραφούν οριστικά. Χωρίς τον κωδικό σου, τα κρυπτογραφημένα δεδομένα ενός κοινού έργου γίνονται αδιάβαστα για σένα - αυτό δεν αναιρείται.
erase-device-open = Αποσύνδεση και διαγραφή αυτής της συσκευής
erase-device-title = Διαγραφή αυτής της συσκευής;
erase-device-body = Τα έργα, τα κλειδιά τους και η προσωρινή μνήμη αυτής της συσκευής θα διαγραφούν και μετά η συνεδρία θα τερματιστεί. Τα έργα σου παραμένουν στον λογαριασμό σου: συνδέσου ξανά για να τα πάρεις πίσω.
erase-device-checking = Ελέγχω αν τα έργα σου είναι στον λογαριασμό σου…
erase-device-pending = Αλλαγές που περιμένουν αποστολή: { $count }. Συνδέσου ξανά στο διαδίκτυο και δοκίμασε πάλι.
erase-device-unreachable = Δεν ήταν δυνατό να ελεγχθεί αν τα έργα σου είναι στον λογαριασμό σου. Έλεγξε τη σύνδεσή σου και δοκίμασε πάλι.
erase-device-at-risk = Αυτά τα έργα δεν είναι στον λογαριασμό σου και θα χαθούν. Για να τα κρατήσεις, ακύρωσε και αντίγραψε πρώτα τον σύνδεσμό τους:
erase-device-confirm = Διαγραφή και αποσύνδεση
erase-device-confirm-anyway = Διαγραφή ούτως ή άλλως
erase-device-erasing = Διαγραφή…

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
payment-methods-stale = Τα στοιχεία πληρωμής σου άλλαξαν σε άλλη συσκευή. Φορτώθηκαν ξανά - δοκίμασε ξανά.
payment-methods-key-missing = Συνδέσου ξανά για να διαχειριστείς τα στοιχεία πληρωμής σου.
settings-payment-methods-share-warning = Μια κοινόχρηστη μέθοδος είναι ορατή σε όλα τα μέλη των έργων σου.
payment-method-share = Κοινή χρήση με τα έργα μου
payment-method-share-hint = Εμφανίζεται δίπλα στο όνομά σου όταν κάποιος σου χρωστάει.
payment-method-copy = Αντιγραφή { $name }
payment-method-copied = Αντιγράφηκε.
payment-method-copy-failed = Δεν ήταν δυνατή η αντιγραφή - επίλεξε το κείμενο και αντέγραψέ το με το χέρι.

verify-email-checking = Επαλήθευση της διεύθυνσης email σου…
verify-email-welcome = Το email επαληθεύτηκε - καλώς ήρθες στο Counted!
verify-email-back-to-login = Πίσω στη σύνδεση
verify-email-intro = Πληκτρολόγησε τον κωδικό σου για να επιβεβαιώσεις ότι αυτή η διεύθυνση είναι δική σου.
verify-email-submit = Επαλήθευση της διεύθυνσής μου

### Project status

project-archive = Αρχειοθέτηση
project-sheet-invite = Πρόσκληση
project-sheet-recurring = Επαναλαμβανόμενες
project-sheet-edit = Επεξεργασία έργου
project-sheet-close = Κλείσιμο έργου
project-sheet-archive = Αρχειοθέτηση έργου
project-sheet-reopen = Άνοιγμα έργου ξανά
project-sheet-unarchive = Επαναφορά έργου από αρχείο
project-sheet-leave = Αποχώρηση από το έργο

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
projects-offline-banner = Δεδομένα εκτός σύνδεσης - συνδέσου ξανά για ανανέωση.
demo-banner = Έργο επίδειξης - μόνο για ανάγνωση.
demo-start-own = Δημιούργησε το δικό σου έργο
demo-badge = Επίδειξη
demo-bar = Οι αλλαγές σου χάνονται όταν κλείσεις αυτή την καρτέλα.
demo-leave = Ξεκίνα κανονικά
demo-preparing = Προετοιμασία της επίδειξης…
demo-failed = Δεν ήταν δυνατή η προετοιμασία της επίδειξης.
demo-sign-in-blocked = Η σύνδεση είναι απενεργοποιημένη στην επίδειξη. Βγες για να συνδεθείς ή να δημιουργήσεις λογαριασμό· τα δικά σου έργα μένουν ανέπαφα.
projects-no-local-data = Δεν υπάρχουν τοπικά δεδομένα
projects-no-local-data-hint = Συνδέσου για να φορτώσεις τα έργα σου για πρώτη φορά.
projects-add = Προσθήκη έργου
projects-create = Δημιουργία έργου
projects-personal-expense = Προσωπικό έξοδο
personal-expense-banner = Προστέθηκε στα Έξοδά μου
personal-create-title = Δημιούργησε το προσωπικό σου έργο
personal-create-hint = Παρακολούθησε τι ξοδεύεις μόνος σου
group-create-title = Δημιούργησε κοινό έργο
group-create-hint = Μοιράσου τα έξοδα με φίλους και οικογένεια
personal-create-action = Δημιουργία
personal-create-signup = Δημιουργία λογαριασμού
personal-sheet-title = Το προσωπικό σου έργο
personal-sheet-intro = Για να σημειώνεις ό,τι ξοδεύεις μόνος σου. Εμφανίζεται με τις ομάδες σου στα Στατιστικά.
personal-sheet-point-alone = Ένα μέλος: εσύ, με το όνομα του λογαριασμού σου. Χωρίς σύνδεσμο κοινοποίησης.
personal-sheet-point-encrypted = Κρυπτογραφημένο στις συσκευές σου, όπως τα άλλα έργα σου.
personal-sheet-point-stats = Τα μερίδιά σου στις ομάδες προστίθενται σε αυτά τα έξοδα στα Στατιστικά.
personal-default-name = Τα έξοδά μου
my-spending-prev-month = Προηγούμενος μήνας
my-spending-next-month = Επόμενος μήνας
my-spending-spent = Ξόδεψες
my-spending-groups = Ομάδες
my-spending-income = Έσοδα
my-spending-balance = Υπόλοιπο μήνα
my-spending-unconverted = Τα μερίδια σε άλλο νόμισμα μένουν εκτός μέχρι να είναι γνωστή μια ισοτιμία.
my-spending-empty = Τίποτα αυτόν τον μήνα
my-spending-empty-hint = Πρόσθεσε ένα έξοδο με +. Τα μερίδιά σου στις ομάδες θα εμφανίζονται εδώ αυτόματα.
my-spending-share-of = από { $amount }
my-spending-uncounted = δεν υπολογίζεται
payer-you = εσένα
expense-received-by = ελήφθη από
counted-switch = Υπολογισμός στα Τα έξοδά μου
counted-switch-on = Ενεργό: το μερίδιό σου προστίθεται στα Τα έξοδά μου
counted-switch-off = Ανενεργό: το μερίδιό σου δεν εμφανίζεται στα Τα έξοδά μου
counted-groups = Ομάδες που υπολογίζονται
counted-groups-intro = Το μερίδιό σου σε αυτές τις ομάδες προστίθεται στα Τα έξοδά μου και στους προϋπολογισμούς.
counted-groups-converted = { $from }, μετατροπή σε { $to }
counted-groups-encrypted = Αυτή η επιλογή κρυπτογραφείται με τον λογαριασμό σου και σε ακολουθεί στις συσκευές σου.
counted-groups-empty = Δεν υπάρχει ακόμη ομάδα σε αυτή τη συσκευή.
counted-groups-done = OK
counted-save-failed = Δεν ήταν δυνατή η αποθήκευση αυτής της επιλογής. Δοκίμασε ξανά.
tab-budget = Προϋπολογισμός
budget-title = Προϋπολογισμοί
budget-manage = Διαχείριση
budget-left = Απομένουν { $amount }
budget-over = Υπέρβαση κατά { $amount }
budget-empty = Κανένας προϋπολογισμός
budget-empty-hint = Όρισε ένα ανώτατο όριο ανά κατηγορία. Τα μερίδιά σου στις ομάδες μετράνε κι αυτά.
budget-define = Ορισμός προϋπολογισμού
budget-sheet-title = Μηνιαίοι προϋπολογισμοί
budget-sheet-intro = Στο μερίδιό σου: προσωπικά και ομάδες. Άφησε κενό για να μην παρακολουθείς μια κατηγορία.
budget-average = Μ.ό. τελευταίων 3 μηνών: { $amount }
budget-for = Προϋπολογισμός { $category }
budget-alert = Ειδοποίησέ με στο 80 %
budget-alert-hint = Υπολογίζεται σε αυτή τη συσκευή
budget-invalid = Μη έγκυρο ποσό για { $category }.
budget-offline = Οι προϋπολογισμοί αποθηκεύονται μόνο online. Συνδέσου ξανά και δοκίμασε πάλι.
budget-alert-near = { $category }: { $percent } % του προϋπολογισμού του μήνα
budget-alert-over = { $category }: υπέρβαση προϋπολογισμού του μήνα
personal-card-budget = { $month } · προϋπολογισμός { $amount }
my-spending-search-summary =
    { $count ->
        [one] { $count } αποτέλεσμα
       *[other] { $count } αποτελέσματα
    } · { $total } · προσωπικά και ομάδες, όλοι οι μήνες
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
add-project-offline = Δεν μπορείς να δημιουργήσεις έργο εκτός σύνδεσης. Συνδέσου ξανά και δοκίμασε πάλι.
add-project-name-required = Το έργο χρειάζεται όνομα.

join-link-label = Σύνδεσμος κοινής χρήσης
join-link-hint = Ο σύνδεσμος περιέχει το κλειδί αποκρυπτογράφησης - αντέγραψέ τον ολόκληρο.
join-invalid-link = Αυτός ο σύνδεσμος δεν είναι έγκυρος. Επικόλλησε ολόκληρο τον σύνδεσμο κοινής χρήσης, μαζί με το τμήμα μετά το #.
join-wrong-project = Αυτός ο σύνδεσμος αφορά άλλο έργο.

import-tricount-link-label = Σύνδεσμος ή κλειδί Tricount
import-tricount-key-required = Εισήγαγε έναν σύνδεσμο ή κλειδί Tricount.
import-tricount-encryption-failed = Η κρυπτογράφηση απέτυχε.
import-tricount-unimportable = Δεν εισήχθη τίποτα: αυτό το Tricount έχει μέλη με λογαριασμό Tricount ή ποσά που δεν συμφωνούν (επηρεαζόμενες εγγραφές: { $count }).

### Expenses

save = Αποθήκευση
saving = Αποθήκευση…
adding = Προσθήκη…
link-copied = Ο σύνδεσμος αντιγράφηκε
missing-encryption-key = Λείπει το κλειδί κρυπτογράφησης.
missing-encryption-key-title = Λείπει το κλειδί κρυπτογράφησης
missing-encryption-key-hint = Ο σύνδεσμος που χρησιμοποίησες δεν περιέχει το κλειδί που χρειάζεται για την αποκρυπτογράφηση αυτού του έργου. Χρησιμοποίησε τον πλήρη σύνδεσμο που μοιράστηκε όποιος το δημιούργησε.
project-locked-hint = Αυτή η συσκευή δεν έχει το κλειδί αυτού του έργου. Άνοιξε τον σύνδεσμο κοινής χρήσης του για να το ξεκλειδώσεις.
expenses-unreadable = Ορισμένες καταχωρίσεις δεν μπορούν να διαβαστούν σε αυτή τη συσκευή και δεν μετράνε στα σύνολα. Ενημέρωσε την εφαρμογή για να τις δεις.
project-unlock = Ξεκλείδωμα
project-no-local-data-hint = Συνδέσου για να φορτώσεις τα δεδομένα αυτού του έργου για πρώτη φορά.
project-gone-title = Αυτό το έργο δεν υπάρχει πια
project-gone-hint = Διαγράφηκε όταν αποχώρησε το τελευταίο του μέλος. Ο σύνδεσμος κοινής χρήσης δεν λειτουργεί πια, ακόμα κι αν τον ανοίξεις ξανά.

expense-add = Προσθήκη δαπάνης
transfer-add = Προσθήκη μεταφοράς
expense-edit-title = Επεξεργασία δαπάνης
expense-category = Κατηγορία
expense-category-auto = Αυτόματα · { $emoji }
emoji-picker-label = Emoji
emoji-none = Χωρίς emoji
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
expense-type-group = Τύπος
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
balance-gets-back = Παίρνει πίσω
balance-owes = Χρωστάει
balance-settled = Εξοφλημένοι
reimbursements-empty-title = Όλα εξοφλημένα!
reimbursements-empty-hint = Οι προτάσεις εξόφλησης εμφανίζονται εδώ όταν οι λογαριασμοί δεν ισορροπούν
reimbursement-owes = { $debtor } χρωστάει σε { $creditor }
reimbursement-record = Εξόφληση
settlement-recorded = Η εξόφληση καταγράφηκε.
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
edit-project-offline-deferred = Εκτός σύνδεσης: { $items } θα εφαρμοστεί όταν συνδεθείς ξανά.

export-failed = Η εξαγωγή απέτυχε: { $reason }

history-expense-added = Προστέθηκε δαπάνη: { $name }
history-expense-edited = Επεξεργάστηκε δαπάνη: { $name }
history-expense-deleted = Διαγράφηκε δαπάνη: { $name }
move-expense = Μετακίνηση σε έργο
move-expense-title = Μετακίνηση σε…
move-current = Τρέχον
move-converted = { $currency }, με την ισοτιμία του μήνα
move-note-personal = Στο επόμενο βήμα θα επιλέξεις ποιος πλήρωσε και για ποιον. Το έξοδο θα φύγει από τα Έξοδά μου.
move-note-group = Το έξοδο θα φύγει από το { $project } για όλα τα μέλη του και το υπόλοιπο της ομάδας θα υπολογιστεί ξανά. Στο επόμενο βήμα θα επιλέξεις ποιος πλήρωσε και για ποιον.
move-banner = Από { $from } σε { $to }
move-submit = Μετακίνηση
move-done = Το έξοδο μετακινήθηκε στο { $project }
history-expense-moved = Το έξοδο μετακινήθηκε σε άλλο έργο: { $name }
personal-badge = Προσωπικό
move-members =
    { $count ->
        [one] { $count } μέλος
       *[other] { $count } μέλη
    }
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
expense-share-percent = { $percent }%
expense-added-by = Προστέθηκε από { $name } στις { $date }
expense-inconsistent-detail = Τα ποσά δεν συμφωνούν: { $paid } πληρώθηκαν, { $owed } οφείλονται, για δαπάνη { $total }. Επεξεργάσου τη δαπάνη για να το διορθώσεις.
missing-access-key = Λείπει το κλειδί πρόσβασης. Άνοιξε αυτό το έργο μέσω του συνδέσμου κοινής χρήσης του.
filter-all = Όλα
filter-my-payments = Οι πληρωμές μου
filter-my-debts = Τι χρωστάω
expense-filter-group = Φίλτρο
expenses-search = Αναζήτηση εξόδων
expenses-no-match = Κανένα έξοδο δεν ταιριάζει.
expenses-search-open = Αναζήτηση
expenses-search-clear = Καθαρισμός αναζήτησης
expenses-search-summary =
    { $count ->
        [one] { $count } αποτέλεσμα
       *[other] { $count } αποτελέσματα
    } · { $total } · όλοι οι μήνες
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
charts-my-share-skipped =
    { $count ->
        [one] 1 έργο δεν υπολογίζεται - δεν επιλέχθηκε συμμετέχων ή τα δεδομένα του δεν φορτώθηκαν.
       *[other] { $count } έργα δεν υπολογίζονται - δεν επιλέχθηκε συμμετέχων ή τα δεδομένα τους δεν φορτώθηκαν.
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
categories-manage = Διαχείριση κατηγοριών
categories-title = Κατηγορίες
categories-yours = Οι κατηγορίες σου
categories-builtin = Προεπιλεγμένες
categories-add = Πρόσθεσε κατηγορία
categories-in-parent = στην κατηγορία { $parent }
categories-renamed-from = μετονομάστηκε · αρχικά { $name }
categories-hidden-hint = κρυφή · τα έξοδά της πάνε στην κατηγορία { $other }
categories-always-shown = εμφανίζεται πάντα
categories-hidden = Κρυφή
categories-new = Νέα κατηγορία
categories-edit = Επεξεργασία κατηγορίας
categories-parent = Μετράει στην κατηγορία
categories-parent-hint = Στα στατιστικά όλων των έργων σου, το «{ $name }» μετράει στην κατηγορία { $parent }.
categories-create = Δημιουργία
categories-hide = Απόκρυψη αυτής της κατηγορίας
categories-hide-hint = Τα έξοδά της εμφανίζονται στην κατηγορία { $other } για όλα τα μέλη.
categories-delete-title = Διαγραφή «{ $name }»;
categories-delete-body = Τα σχετικά έξοδα ({ $count }) επιστρέφουν στην κατηγορία { $parent }. Κανένα έξοδο δεν διαγράφεται.
categories-delete-body-unknown = Τα έξοδά της επιστρέφουν στην κατηγορία { $parent }. Κανένα έξοδο δεν διαγράφεται.
categories-delete-warning = Η κατηγορία εξαφανίζεται για όλα τα μέλη του έργου.
categories-summary-created = Δημιουργήθηκαν: { $count }
categories-summary-renamed = Μετονομάστηκαν: { $count }
categories-summary-hidden = Κρυφές: { $count }
categories-summary-default = Καμία δική σου κατηγορία
categories-offline = Η διαχείριση κατηγοριών γίνεται μόνο online. Συνδέσου ξανά και δοκίμασε πάλι.
categories-error-too-many = Ένα έργο έχει το πολύ 20 δικές του κατηγορίες.
categories-error-full = Αυτό το έργο έφτασε το όριο κατηγοριών.
categories-error-name = Δώσε ένα όνομα στην κατηγορία.
categories-error-missing = Αυτή η κατηγορία άλλαξε στο μεταξύ. Κλείσε και δοκίμασε ξανά.
history-category-created = Δημιουργήθηκε κατηγορία: { $name }
history-category-edited = Επεξεργάστηκε κατηγορία: { $name }
history-category-deleted = Διαγράφηκε κατηγορία: { $name }
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
charts-paid-more = { $amount } περισσότερα από το μερίδιό μου
charts-paid-less = { $amount } λιγότερα από το μερίδιό μου
charts-paid-even = Ακριβώς το μερίδιό μου
charts-part-title = Το μερίδιό σου σε κάθε κατηγορία
charts-part-desc = Γκρι όσα ξόδεψε η ομάδα, χρώμα όσα κατανάλωσες εσύ.
charts-breakdown-title = Ανάλυση ανά κατηγορία
charts-breakdown-desc = Πάτησε ένα κομμάτι ή μια γραμμή για να δεις τα έξοδα.
charts-of-total = { $amount } από { $total }
charts-show-all = Εμφάνιση όλων ({ $count })
charts-show-less = Εμφάνιση λιγότερων
charts-spend-title = Έξοδα στον χρόνο
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
charts-projects-desc = Τα σύνολα κρατιούνται ανά νόμισμα.
history-empty = Δεν υπάρχουν συμβάντα
history-by = Από { $name }
not-found-hint = Αυτή η σελίδα δεν υπάρχει ή μετακινήθηκε.
payers-title-paid-by = Πληρώθηκε από
payers-title-sender = Αποστολέας
payers-title-contributors = Συνεισφέροντες
debtors-title-debtors = Χρωστούν
debtors-title-recipients = Παραλήπτες
debtors-title-beneficiaries = Δικαιούχοι
debtors-title-for = Για
debtors-title-recipient = Παραλήπτης
expense-split-between = μοιράστηκε σε { $count }
expense-people =
    { $count ->
        [one] { $count } άτομο
       *[other] { $count } άτομα
    }

### Welcome

welcome-title = Οι λογαριασμοί σου δεν αφορούν κανέναν άλλο.
welcome-subtitle = Μοιράσου έξοδα με φίλους.
welcome-note = Δωρεάν. Χωρίς λογαριασμό. Χωρίς διαφημίσεις.
welcome-link-title = Ένας σύνδεσμος, και συμμετέχουν όλοι.
welcome-link-body = Κανείς δεν χρειάζεται να δημιουργήσει λογαριασμό.
welcome-link-account = Λογαριασμός; Ποτέ υποχρεωτικός. Χρησιμεύει για να βρίσκεις τα έργα σου σε άλλη συσκευή, να προσκαλείς φίλους από την εφαρμογή και να μοιράζεσαι τα στοιχεία πληρωμής σου.
welcome-demo-project = Σαββατοκύριακο στη Λιόν
welcome-private-title = Κανείς δεν μπορεί να διαβάσει τους λογαριασμούς σου. Ούτε εμείς.
welcome-private-body = Ονόματα, ποσά, έργα: όλα κρυπτογραφούνται στη συσκευή σου. Μόνο εσύ έχεις το κλειδί.
welcome-private-names = Ονόματα
welcome-private-amounts = Ποσά
welcome-private-projects = Έργα
welcome-scan-title = Φωτογράφισε την απόδειξη.
welcome-scan-body = Το ποσό, η ημερομηνία και η κατηγορία συμπληρώνονται μόνα τους. Όλα γίνονται στο τηλέφωνό σου. Η φωτογραφία δεν αποθηκεύεται.
welcome-eu-title = 100% ευρωπαϊκό
welcome-no-ads = Χωρίς διαφημίσεις
welcome-no-trackers = Χωρίς ιχνηλάτες
welcome-step = Βήμα { $current } από { $total }
welcome-next = Επόμενο
welcome-skip = Παράλειψη
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
help-forgot-password-warning = Κανείς δεν μπορεί να τον ανακτήσει για σένα.
help-forgot-password-a = Ο κωδικός σου προστατεύει τα κλειδιά κρυπτογράφησής σου και δεν μας αποστέλλεται ποτέ: κανείς - ούτε εμείς - δεν μπορεί να αποκρυπτογραφήσει τα έργα σου στη θέση σου. Φύλαξέ τον με ασφάλεια, ιδανικά σε διαχειριστή κωδικών.
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
project-currency-locked = Το νόμισμα ορίζεται κατά τη δημιουργία του έργου.
currency-search = Αναζήτηση νομίσματος

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
friends-add-hint = Αν αυτό το άτομο έχει λογαριασμό, θα δει το αίτημά σου στην εφαρμογή. Αλλιώς, ενημέρωσέ το: θα το βρει όταν εγγραφεί με αυτή τη διεύθυνση.
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
friends-remove-confirm-message = Ο/Η { $email } δεν θα είναι πια στους φίλους σου, ούτε εσύ στους δικούς του. Οποιοσδήποτε από τους δύο μπορεί να στείλει νέο αίτημα αργότερα.
friends-no-key = Δεν είναι έτοιμο ακόμα
friends-key-changed = Ο κωδικός άλλαξε
friends-key-changed-hint = Ο κωδικός ασφαλείας ενός φίλου άλλαξε. Συμβαίνει όταν ασφαλίζει τον λογαριασμό του, ή αν το κλειδί του αντικαταστάθηκε. Οι προσκλήσεις μαζί του είναι μπλοκαρισμένες μέχρι να διαβάσετε ο ένας στον άλλον τον νέο κωδικό, από κοντά ή στο τηλέφωνο, και να πατήσεις «Ο κωδικός ταιριάζει».
friends-fingerprint = Κωδικός ασφαλείας
friends-fingerprint-hint = Δύο φίλοι που διαβάζουν ο ένας στον άλλο τον ίδιο κωδικό ασφαλείας ξέρουν ότι κανείς δεν βρίσκεται ανάμεσά τους - ούτε καν ο διακομιστής μας.
friends-key-mismatch = Ο διακομιστής έστειλε για τον λογαριασμό σου δημόσιο κλειδί που δεν ταιριάζει με το ιδιωτικό σου κλειδί. Ο κωδικός ασφαλείας σου κρύφτηκε και οι προσκλήσεις είναι απενεργοποιημένες σε αυτή τη συσκευή: ίσως κάποιος παραποιεί τον διακομιστή.
keypair-mismatch = Τα κλειδιά που έστειλε ο διακομιστής για τον λογαριασμό σου δεν ταιριάζουν με αυτά της συσκευής. Οι προσκλήσεις και οι αλλαγές κλειδιού είναι απενεργοποιημένες εδώ μέχρι να ταιριάξουν: κάποιος ίσως παραποιεί τον διακομιστή.
friends-outgoing-title = Απεσταλμένα
friends-outgoing-hint = Σε αναμονή απάντησης. Θα τους δεις στους φίλους σου μόλις αποδεχτούν.
friends-withdraw = Ακύρωση
friends-notify = Ενημέρωση με μήνυμα
friends-notify-label = Ενημέρωσε: { $email }
friends-notify-message = Περνάω στο Counted για τα κοινά μας έξοδα. Δωρεάν εγγραφή στο https://counted.fr με τη διεύθυνση { $email }: εκεί περιμένει το αίτημα φιλίας μου.
friends-notify-copied = Το μήνυμα αντιγράφηκε
friends-notify-copy-failed = Δεν ήταν δυνατή η αντιγραφή του μηνύματος
guests-hint-title =
    { $count ->
        [one] { $count } συμμετέχων δεν έχει μπει ακόμη στο έργο
       *[other] { $count } συμμετέχοντες δεν έχουν μπει ακόμη στο έργο
    }
guests-hint-later = Αργότερα
guests-hint-send = Προσκάλεσέ τους
guests-send-title = Αποστολή του έργου στους συμμετέχοντες
guests-send-friend = Πρόσκληση φίλου στο Counted
guests-send-friend-badge = Χωρίς σύνδεσμο
guests-send-friend-hint = Αν έχει ήδη λογαριασμό, λαμβάνει κρυπτογραφημένη πρόσκληση· τίποτα δεν περνά από την εφαρμογή μηνυμάτων σου.
guests-send-link = Κοινοποίηση συνδέσμου έργου
guests-send-link-hint = Για όσους δεν έχουν λογαριασμό. Ανοίγουν το έργο με ένα πάτημα.
guests-send-link-warning = Όποιος έχει αυτόν τον σύνδεσμο μπορεί να διαβάσει και να αλλάξει το έργο. Το βλέπει και η εφαρμογή μηνυμάτων σου.
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

# Participants in the create and edit modals, and the "who are you?" picker - see
# docs/plans/friends.md §11.
participants-you-label = Το όνομά σου σε αυτό το έργο
participants-you-badge = Εσύ
participants-others = Άλλοι συμμετέχοντες
participants-removed = Αφαιρέθηκαν
participant-restore = Επαναφορά
participants-empty = Πρόσθεσε συμμετέχοντες παρακάτω.
participants-duplicate = Το «{ $name }» υπάρχει ήδη στη λίστα.
participants-input-label = Πρόσθεσε φίλο ή γράψε όνομα
participants-input-placeholder = Φίλος ή οποιοδήποτε όνομα
participants-suggest-friend = Φίλος · μπαίνει ως «{ $name }», παίρνει πρόσκληση
participants-suggest-not-ready = Φίλος · δεν είναι ακόμα έτοιμος
participants-suggest-guest = Προσθήκη «{ $text }»
participants-suggest-guest-sub = Επισκέπτης
participants-friends = Οι φίλοι σου
participants-all-friends = Όλοι οι φίλοι
participants-login-hint = Συνδέσου για να προσθέτεις άτομα κατευθείαν από τη λίστα φίλων σου.
participants-invite-badge = Πρόσκληση
participants-guest-badge = Επισκέπτης
participants-rename = Μετονομασία: { $name }
participants-remove = Αφαίρεση: { $name }
participants-rename-label = Νέο όνομα
participants-rename-save = Αποθήκευση ονόματος
participants-rename-hint = Το όνομα που βλέπουν όλοι σε αυτό το έργο. Η πρόσκληση πηγαίνει ακόμα στο { $email }.
participants-invited-badge = Προσκλήθηκε
participants-invited-sub = { $email } · δεν έγινε ακόμα αποδεκτή
participants-invited-pending = Η πρόσκληση δεν έγινε ακόμα αποδεκτή
participants-unlinked = Δεν συνδέεται με λογαριασμό
add-project-create-invite = Δημιουργία και πρόσκληση: { $count }
edit-project-save-invite = Αποθήκευση και πρόσκληση: { $count }
edit-project-you-are = Σε αυτή τη συσκευή είσαι ο/η { $name }
edit-project-no-identity = Δεν έχεις διαλέξει ακόμα ποιος είσαι
edit-project-switch = Αλλαγή
edit-project-choose = Επιλογή
invite-failed = Δεν ήταν δυνατή η αποστολή αυτών των προσκλήσεων: { $emails }
invite-waiting = Αυτοί οι φίλοι πρέπει να ανοίξουν το Counted μία φορά πριν μπορέσουν να προσκληθούν: { $emails }
key-offer-title = Διαφορετικό κλειδί για { $name }
key-offer-body = Αυτή η συσκευή έχει άλλο κλειδί για αυτό το έργο, που δεν το έχει ανοίξει ποτέ εδώ.
key-offer-choices = Χρήση αυτού του κλειδιού: το έργο ανοίγει με το κλειδί αυτού του συνδέσμου ή της πρόσκλησης. Ακύρωση: η συσκευή κρατά το τρέχον κλειδί της και τίποτα δεν αλλάζει.
key-offer-use = Χρήση αυτού του κλειδιού
invite-again = Νέα πρόσκληση
friend-picker-title = Προσθήκη φίλων
user-selection-invited-hint = Ο/Η { $email } σε προσκάλεσε στο «{ $project }».
user-selection-suggested = Προτείνεται
user-selection-suggested-sub = Ο/Η { $email } σε πρόσθεσε με αυτό το όνομα
user-selection-confirm-as = Είμαι ο/η { $name }
user-selection-missing = Δεν βρίσκεις το όνομά σου; Ζήτα από έναν συμμετέχοντα να σε προσθέσει στις ρυθμίσεις του έργου.

## Επαναλαμβανόμενες δαπάνες

repeat-label = Επανάληψη
repeat-none = Δεν επαναλαμβάνεται
repeat-weekly = Κάθε εβδομάδα
repeat-biweekly = Κάθε 2 εβδομάδες
repeat-monthly = Κάθε μήνα
repeat-quarterly = Κάθε 3 μήνες
repeat-yearly = Κάθε χρόνο
repeat-every-weeks = Κάθε { $count } εβδ.
repeat-every-months = Κάθε { $count } μήνες
repeat-every-years = Κάθε { $count } έτη
repeat-custom = Προσαρμογή…
repeat-every = Κάθε
repeat-unit-weeks = εβδ.
repeat-unit-months = μήνες
repeat-unit-years = έτη
repeat-on-weekday = ημέρα: { $weekday }
repeat-on-day = στις { $day } του μήνα
repeat-on-day-month = στις { $day } { $month }
repeat-month-end = Στους μικρότερους μήνες πέφτει την τελευταία ημέρα.
repeat-ends = Λήγει
repeat-ends-never = Ποτέ
repeat-ends-on = Σε ημερομηνία
repeat-ends-after = Μετά από
repeat-fewer = Λιγότερες
repeat-more = Περισσότερες
repeat-last-on = τελευταία στις { $date }
repeat-variable = Το ποσό αλλάζει κάθε φορά
repeat-variable-hint = Κάθε μία προστίθεται με το τελευταίο ποσό και σημειώνεται «προς επιβεβαίωση».
repeat-done = Τέλος
repeat-no-end = Χωρίς λήξη
repeat-until = Έως { $date }
repeat-occurrences = Επαναλήψεις: { $count }
repeat-offline = Χρειάζεται σύνδεση. Η ίδια η δαπάνη μπορεί να προστεθεί κανονικά.
repeat-foreign = Επαναλαμβάνεται ως { $amount } { $currency }, μετατρεμμένο μία φορά με τη σημερινή ισοτιμία. Θα ειδοποιηθείς αν η ισοτιμία αλλάξει πάνω από 5 %.
repeat-backfill = Ξεκινά στο παρελθόν. Δαπάνες που προστίθενται τώρα: { $count }.
add-and-repeat = Προσθήκη και επανάληψη
weekday-1 = Δευτέρα
weekday-2 = Τρίτη
weekday-3 = Τετάρτη
weekday-4 = Πέμπτη
weekday-5 = Παρασκευή
weekday-6 = Σάββατο
weekday-7 = Κυριακή
recurring-title = Επαναλαμβανόμενες δαπάνες
recurring-strip = Επαναλαμβανόμενες δαπάνες: { $count }
recurring-next = Επόμενη: { $name }, { $date }
recurring-to-confirm = Προς επιβεβαίωση: { $count }
recurring-per-month = Ανά μήνα, περίπου
recurring-your-share = Το μερίδιό σου
recurring-active = Ενεργές
recurring-paused = Σε παύση
recurring-finished = Ολοκληρωμένες
recurring-paid-by = πληρώνει { $name }
recurring-next-on = Επόμενη στις { $date }
recurring-progress = { $done } από { $total }
recurring-rate-badge = Η ισοτιμία άλλαξε { $percent } %
recurring-empty = Τίποτα δεν επαναλαμβάνεται ακόμα. Επίλεξε «Επανάληψη» όταν προσθέτεις δαπάνη: ενοίκιο, συνδρομές, λογαριασμοί.
recurring-next-ones = Επόμενες
recurring-added-so-far = Προστέθηκαν μέχρι τώρα
recurring-set-up-by = Δημιουργήθηκε από
recurring-pause = Παύση
recurring-resume = Συνέχιση
recurring-stop = Διακοπή επανάληψης
recurring-stop-title = Διακοπή του «{ $name }»;
recurring-stop-message = Δεν θα επαναλαμβάνεται πια. Οι δαπάνες που έχουν ήδη προστεθεί μένουν.
recurring-resume-title = Συνέχιση του «{ $name }»;
recurring-resume-message = Επόμενη στις { $date }. Οι ημερομηνίες που χάθηκαν κατά την παύση δεν προστίθενται.
recurring-edit-title = Επεξεργασία επαναλαμβανόμενης δαπάνης
recurring-edit-banner = Οι αλλαγές ισχύουν από { $date }. Οι δαπάνες που έχουν ήδη προστεθεί μένουν ως έχουν.
recurring-next-on-label = Επόμενη στις
recurring-next-too-early = Η επόμενη ημερομηνία πρέπει να είναι μετά την τελευταία δαπάνη που προστέθηκε.
recurring-use-stop = Για να τη σταματήσεις, χρησιμοποίησε «Διακοπή επανάληψης» στην επαναλαμβανόμενη δαπάνη.
recurring-drift = Η ισοτιμία του { $currency } άλλαξε { $percent } % από τη δημιουργία. Κάθε μία προστίθεται ακόμα ως { $amount } { $project_currency } (1 { $currency } = { $rate }). Με τη σημερινή ισοτιμία θα ήταν { $today_amount } { $project_currency }.
recurring-use-rate = Χρήση σημερινής ισοτιμίας
recurring-keep = Διατήρηση { $amount } { $currency }
recurring-added = Προστέθηκαν επαναλαμβανόμενες δαπάνες: { $count }
recurring-blocks-removal = Ο/Η { $name } δεν μπορεί ακόμα να αφαιρεθεί: συμμετέχει σε { $rules }. Αφαίρεσε { $name } από αυτές ή σταμάτησέ τες και αποθήκευσε ξανά.
history-recurring-added = Προστέθηκε αυτόματα: { $name } ({ $date })
history-recurring-created = Δημιουργήθηκε επαναλαμβανόμενη δαπάνη: { $name }
history-recurring-edited = Επεξεργάστηκε επαναλαμβανόμενη δαπάνη: { $name }
history-recurring-paused = Τέθηκε σε παύση επαναλαμβανόμενη δαπάνη: { $name }
history-recurring-resumed = Συνεχίστηκε επαναλαμβανόμενη δαπάνη: { $name }
history-recurring-stopped = Διακόπηκε επαναλαμβανόμενη δαπάνη: { $name }
occurrence-recurring = Επαναλαμβανόμενη δαπάνη
occurrence-auto = Προστέθηκε αυτόματα από επαναλαμβανόμενη δαπάνη.
occurrence-auto-next = Προστέθηκε αυτόματα από επαναλαμβανόμενη δαπάνη. Επόμενη στις { $date }.
occurrence-auto-stopped = Προστέθηκε αυτόματα από επαναλαμβανόμενη δαπάνη που έχει πλέον σταματήσει.
occurrence-manage = Διαχείριση
estimate-badge = Προς επιβεβαίωση
expense-pending = Σε αναμονή
estimate-title = Ποσό προς επιβεβαίωση.
estimate-body = Προστέθηκε με το προηγούμενο ποσό. Βάλε το πραγματικό όταν το μάθεις.
estimate-confirm = Επιβεβαίωση ποσού
apply-to = Εφαρμογή σε
apply-this-only = Μόνο αυτή τη δαπάνη
apply-and-next = Αυτή και τις επόμενες
apply-and-next-hint = Η επαναλαμβανόμενη δαπάνη αλλάζει από { $date }
apply-rule-failed = Η δαπάνη αποθηκεύτηκε, αλλά η επαναλαμβανόμενη δαπάνη δεν άλλαξε.
occurrence-delete-message = Η «{ $name }» της { $date } θα διαγραφεί οριστικά. Η επανάληψη συνεχίζεται και αυτή η ημερομηνία δεν θα επιστρέψει.
occurrence-delete-one = Διαγραφή μόνο αυτής
occurrence-delete-stop = Διαγραφή και διακοπή επανάληψης
error-recurring-clock = Το ρολόι αυτής της συσκευής πάει μπροστά. Έλεγξε την ημερομηνία και την ώρα.
error-recurring-not-found = Αυτή η επαναλαμβανόμενη δαπάνη δεν υπάρχει πια.
error-recurring-stale = Κάποιος άλλαξε στο μεταξύ αυτή την επαναλαμβανόμενη δαπάνη. Φορτώθηκε ξανά: έλεγξέ την και αποθήκευσε ξανά.
error-too-many-recurring = Αυτό το έργο έχει ήδη 50 επαναλαμβανόμενες δαπάνες. Σταμάτησε μία που δεν χρειάζεσαι για να προσθέσεις νέα.
error-user-in-recurring = Αυτός ο συμμετέχων ανήκει σε επαναλαμβανόμενη δαπάνη. Αφαίρεσέ τον πρώτα από αυτή ή σταμάτησέ την.

home-owed-to-you = Σου χρωστούν
home-you-owe = Χρωστάς
home-details-show = Λεπτομέρειες
home-details-hide = Απόκρυψη λεπτομερειών
home-all-settled = Όλα έχουν εξοφληθεί
home-pay = Πληρωμή
home-remind = Υπενθύμιση
home-remind-message = Μικρή υπενθύμιση: στο { $project } εκκρεμούν ακόμη αποπληρωμές.
home-row-owed = σου χρωστούν
home-row-owe = χρωστάς
home-row-settled = Εξοφλημένο
home-row-quits = Είσαι εντάξει
home-row-pending =
    { $count ->
        [one] { $count } εκκρεμής αποπληρωμή
       *[other] { $count } εκκρεμείς αποπληρωμές
    }
home-totals-partial = Χωρίς τα έργα χωρίς συναλλαγματική ισοτιμία
error-account-key-changed = Το κλειδί του λογαριασμού σου άλλαξε σε άλλη συσκευή. Συνδέσου ξανά για να συνεχίσεις.
settings-security = Ασφάλεια
settings-change-password = Αλλαγή κωδικού
settings-change-password-hint = Τα έργα και τα δεδομένα σου μένουν ως έχουν. Οι άλλες συσκευές σου αποσυνδέονται.
change-password-title = Άλλαξε τον κωδικό σου
change-password-current = Τρέχων κωδικός
change-password-new = Νέος κωδικός
change-password-confirm = Επιβεβαίωσε τον νέο κωδικό
change-password-mismatch = Οι δύο νέοι κωδικοί δεν ταιριάζουν.
change-password-submit = Αλλαγή κωδικού
change-password-submitting = Αλλαγή…
change-password-done = Ο κωδικός άλλαξε. Οι άλλες συσκευές σου αποσυνδέθηκαν.
error-rotation-incomplete = Ο λογαριασμός σου άλλαξε στο μεταξύ. Δοκίμασε ξανά.
error-key-lifecycle-unavailable = Δεν είναι ακόμη διαθέσιμο: πρώτα όλοι χρειάζονται την πιο πρόσφατη έκδοση της εφαρμογής.
settings-compromised = Κάποιος ίσως ξέρει τον κωδικό μου
settings-compromised-hint = Νέος κωδικός, νέα κλειδιά και αποσύνδεση όλων των άλλων συσκευών.
settings-compromised-open = Ασφάλισε τον λογαριασμό μου
compromised-title = Ασφάλισε τον λογαριασμό σου
compromised-body = Ο κωδικός σου και τα κλειδιά του λογαριασμού σου αντικαθίστανται και όλες οι άλλες συσκευές αποσυνδέονται. Οι φίλοι σου θα δουν τον κωδικό ασφαλείας σου να αλλάζει: συγκρίνετέ τον ξανά.
compromised-keeps = Το άτομο που είχε τον κωδικό σου κρατά ό,τι έχει ήδη ανοίξει: τα έργα που μπορούσε να διαβάσει μέχρι τώρα.
compromised-submit = Αντικατάσταση κωδικού και κλειδιών
compromised-done = Ο λογαριασμός σου είναι ασφαλής. Ζήτα από τους φίλους σου να συγκρίνουν τον νέο κωδικό ασφαλείας σου.
friends-new-fingerprint = Νέος κωδικός:
friends-code-matches = Ο κωδικός ταιριάζει
friends-code-trusted = Ο κωδικός επιβεβαιώθηκε.
settings-recovery-key = Κλειδί ανάκτησης
settings-recovery-key-none = Χωρίς αυτό, ένας ξεχασμένος κωδικός σού κοστίζει τις προτιμήσεις, τα στοιχεία πληρωμής και τους κωδικούς των φίλων, και διαρκεί 3 μέρες.
settings-recovery-key-set = Ορίστηκε. Αν ξεχάσεις τον κωδικό σου, σου επιστρέφει τον λογαριασμό με όλο το περιεχόμενο.
settings-recovery-key-pending = Ένα νέο κλειδί το αντικαθιστά στις { $when }. Δεν ήσουν εσύ; Ακύρωσέ το με το τρέχον κλειδί σου.
recovery-key-create = Δημιουργία
recovery-key-replace = Αντικατάσταση
recovery-key-cancel-replacement = Ακύρωση του νέου κλειδιού
recovery-key-title = Το κλειδί ανάκτησής σου
recovery-key-body = Γράψ’ το ή αποθήκευσέ το κάπου που μόνο εσύ έχεις πρόσβαση. Το Counted δεν μπορεί να το ξαναδείξει ούτε να το ανακτήσει για σένα.
recovery-key-copy = Αντιγραφή
recovery-key-copied = Αντιγράφηκε
recovery-key-save = Αποθήκευση ως αρχείο
recovery-key-print = Εκτύπωση
recovery-key-next = Συνέχεια
recovery-key-confirm-body = Για να ελέγξεις το αντίγραφό σου, πληκτρολόγησε την ομάδα { $n } του κλειδιού.
recovery-key-confirm-wrong = Η ομάδα δεν ταιριάζει. Κοίτα ξανά το αντίγραφό σου.
recovery-key-password-body = Πληκτρολόγησε τον κωδικό σου για να αποθηκεύσεις το κλειδί στον λογαριασμό.
recovery-key-submit = Αποθήκευση κλειδιού
recovery-key-done = Το κλειδί ανάκτησης αποθηκεύτηκε.
recovery-key-replacement-done = Το νέο κλειδί αποθηκεύτηκε. Αντικαθιστά το τρέχον σε 72 ώρες· μέχρι τότε το τρέχον λειτουργεί ακόμα.
recovery-key-field = Κλειδί ανάκτησης
recovery-key-file-intro = Κλειδί ανάκτησης Counted
recovery-key-cancel-done = Το νέο κλειδί ακυρώθηκε.
compromised-new-key = Αποθήκευσε τώρα το νέο κλειδί ανάκτησης: αντικαθιστά το παλιό σε 72 ώρες, και μέχρι τότε το παλιό λειτουργεί ακόμα.
pending-reset-title = Ζητήθηκε επαναφορά του λογαριασμού σου
pending-reset-body = Ισχύει από { $when }: τότε χάνονται οι προτιμήσεις, τα στοιχεία πληρωμής και οι κωδικοί των φίλων, και οι συσκευές σου πρέπει να συνδεθούν ξανά. Δεν ήσουν εσύ; Ακύρωσέ το με τον τρέχοντα κωδικό σου.
pending-reset-cancel = Ακύρωση επαναφοράς
pending-reset-cancelled = Η επαναφορά ακυρώθηκε.
login-forgot = Ξέχασες τον κωδικό;
forgot-title = Ξέχασες τον κωδικό
forgot-body = Πληκτρολόγησε το email σου. Αν έχει λογαριασμό, στέλνουμε σύνδεσμο για νέο κωδικό.
forgot-submit = Αποστολή συνδέσμου
forgot-sent = Αν η διεύθυνση έχει λογαριασμό, ο σύνδεσμος είναι καθ’ οδόν. Λήγει σε 24 ώρες.
reset-title = Ανάκτησε τον λογαριασμό σου
reset-with-key-body = Πληκτρολόγησε το κλειδί ανάκτησης και νέο κωδικό: όλα επιστρέφουν όπως ήταν.
reset-submit-key = Ανάκτηση του λογαριασμού μου
reset-no-key = Δεν έχω το κλειδί ανάκτησης
reset-without-key-title = Επαναφορά χωρίς κλειδί ανάκτησης
reset-without-key-body = Ο λογαριασμός σου ξεκινά από την αρχή με νέο κωδικό σε 72 ώρες. Τα έργα επιστρέφουν από τις συσκευές σου και τους συνδέσμους κοινοποίησης· οι προτιμήσεις, τα στοιχεία πληρωμής και οι κωδικοί των φίλων όχι. Μέχρι τότε ο τρέχων κωδικός λειτουργεί και μπορεί να ακυρώσει την επαναφορά.
reset-name = Το όνομά σου
reset-submit-request = Επαναφορά του λογαριασμού μου
reset-requested = Έγινε. Ο νέος κωδικός λειτουργεί από { $when }. Μέχρι τότε ο τρέχων λειτουργεί και μπορεί να το ακυρώσει.
reset-pending-already = Μια επαναφορά εκκρεμεί ήδη έως { $when }. Ένα νέο αίτημα ξεκινά ξανά τις 72 ώρες.
recovered-done = Ο λογαριασμός σου επέστρεψε. Οι άλλες συσκευές αποσυνδέθηκαν.
login-reset-undoable = Αυτός ο κωδικός αντικαταστάθηκε από επαναφορά τις τελευταίες 30 ημέρες. Αν δεν τη ζήτησες, αναίρεσέ τη: ο λογαριασμός επιστρέφει όπως ήταν.
login-undo-reset = Αναίρεση επαναφοράς
reset-undone = Η επαναφορά αναιρέθηκε. Ο λογαριασμός σου είναι όπως πριν.
error-reset-pending = Ο νέος κωδικός λειτουργεί όταν η επαναφορά τεθεί σε ισχύ, 72 ώρες μετά το αίτημα.
error-account-disabled = Ο λογαριασμός διαγράφηκε λίγο μετά από επαναφορά και περιμένει να σβηστεί. Ο προηγούμενος κωδικός μπορεί ακόμη να αναιρέσει την επαναφορά.
error-recovery-key-invalid = Αυτό το κλειδί ανάκτησης δεν ανοίγει αυτόν τον λογαριασμό.
error-project-key-required = Αυτό το έργο χρειάζεται το κλειδί του. Άνοιξέ το ξανά από τον σύνδεσμο κοινοποίησης.
names-or = { $first } ή { $last }
participants-admin-badge = Διαχειριστής
edit-project-admins = Διαχειριστές: { $count }
edit-project-succession = Αν το έργο μείνει χωρίς διαχείριση, τον ρόλο αναλαμβάνει το μέλος με λογαριασμό που είναι στο έργο περισσότερο καιρό.
member-sheet-account = Μέλος με λογαριασμό
member-actions = Ενέργειες: { $name }
member-make-admin = Ανάθεση ρόλου διαχειριστή
member-make-admin-hint = Θα μπορεί να αλλάζει το κλειδί του έργου και να αφαιρεί μέλη.
member-made-admin = { $name }: έχει πλέον τον ρόλο διαχειριστή.
group-security = Ασφάλεια έργου
group-security-admin = Ασφάλεια έργου · έχεις τον ρόλο διαχειριστή
group-rotate = Αλλαγή κλειδιού έργου
group-rotate-hint = Αν ο σύνδεσμος του έργου διέρρευσε: όσοι έχουν τον παλιό σύνδεσμο χάνουν την πρόσβαση στο έργο.
group-remove = Αφαίρεση κάποιου από το έργο
group-remove-hint = Αφαιρεί την πρόσβαση και αλλάζει ταυτόχρονα το κλειδί.
group-admins-only = Μόνο για διαχειριστές.
group-admins-only-account = Μόνο για διαχειριστές, που έχουν λογαριασμό.
group-ask-admin = Ρώτα το άτομο που διαχειρίζεται το έργο: { $names }.
group-ask-admins = Ρώτα τα άτομα που διαχειρίζονται το έργο: { $names }.
group-ask-an-admin = Ρώτα ένα από τα άτομα που διαχειρίζονται το έργο.
group-with-account = Με λογαριασμό
group-free = Δωρεάν
group-sign-in-hint = Ένα άτομο που διαχειρίζεται το έργο ({ $names }) μπορεί μετά να σου δώσει τον ρόλο.
group-sign-in-hint-any = Ένα άτομο που διαχειρίζεται το έργο μπορεί μετά να σου δώσει τον ρόλο.
group-no-admin = Αυτό το έργο δεν έχει διαχειριστή, οπότε το κλειδί του δεν μπορεί να αλλάξει.
group-hide = Απόκρυψη κάποιου
group-hide-hint = Κρύβει το άτομο από τις λίστες χωρίς να αφαιρεί την πρόσβαση: ο σύνδεσμος και το κλειδί εξακολουθούν να λειτουργούν.
group-recreate-hint = Αυτό το έργο δεν έχει διαχειριστή. Για να αφαιρέσεις ένα μέλος, δημιούργησε νέο έργο.
group-recreate = Δημιουργία νέου έργου
group-no-admin-yet = Αυτό το έργο δεν έχει ακόμη διαχειριστή.
group-creator-hint = Μόνο εσύ μπορείς να αναλάβεις τη διαχείριση αυτού του έργου, δημιουργώντας λογαριασμό.
group-creator-elsewhere-hint = Μόνο το άτομο που δημιούργησε το έργο μπορεί να αναλάβει τη διαχείρισή του, δημιουργώντας λογαριασμό. Αλλιώς, δημιούργησε νέο έργο για να αφαιρέσεις ένα μέλος.
member-revoke-admin = Αφαίρεση ρόλου διαχειριστή
member-revoke-admin-hint = Δεν θα μπορεί πλέον να αλλάζει το κλειδί του έργου ούτε να αφαιρεί μέλη.
member-revoked-admin = { $name }: δεν έχει πλέον τον ρόλο διαχειριστή.
member-step-down = Παραίτηση από τον ρόλο διαχειριστή
member-step-down-hint = Δεν θα μπορείς πλέον να αλλάζεις το κλειδί του έργου ούτε να αφαιρείς μέλη.
member-stepped-down = Δεν έχεις πλέον τον ρόλο διαχειριστή.
member-remove = Αφαίρεση από το έργο
member-remove-hint = Αλλάζει το κλειδί του έργου. { $name }: δεν θα διαβάζει πια τίποτα νέο.
member-hide = Απόκρυψη από τις λίστες
error-user-not-admin = Αυτό το μέλος δεν έχει τον ρόλο διαχειριστή.
error-creator-proof-invalid = Αυτή η συσκευή δεν δημιούργησε αυτό το έργο.
error-project-has-admin = Αυτό το έργο έχει ήδη διαχειριστή.
error-not-project-admin = Μόνο η διαχείριση του έργου μπορεί να το κάνει αυτό.
error-user-not-claimed = Αυτό το άτομο δεν έχει λογαριασμό, οπότε δεν μπορεί να έχει τον ρόλο διαχειριστή.
error-project-key-rotated = Το κλειδί του έργου άλλαξε. Ζήτα τον νέο σύνδεσμο από ένα μέλος.
error-project-rotating = Το κλειδί του έργου αλλάζει. Δοκίμασε ξανά σε λίγο.
error-rotation-expired = Η αλλαγή κλειδιού κράτησε πολύ και ακυρώθηκε. Ξεκίνα από την αρχή.
error-rotation-rows-changed = Το έργο άλλαξε κατά την αλλαγή κλειδιού. Ξεκίνα από την αρχή.
project-rotated-title = Δεν έχεις πια πρόσβαση σε αυτό το έργο
project-rotated-body = Το κλειδί αυτού του έργου άλλαξε. Ζήτα τον νέο σύνδεσμο από τη διαχείριση του έργου.
project-rotated-open-link = Άνοιγμα συνδέσμου
rotation-remove-title = Αφαίρεση από το έργο: { $name };
rotation-rotate-title = Αλλαγή κλειδιού του έργου;
rotation-remove-body = Η αφαίρεση μέλους σημαίνει δημιουργία νέου κλειδιού. Θα πρέπει να μοιραστείς τον νέο σύνδεσμο με τα άτομα χωρίς λογαριασμό.
rotation-rotate-body = Με τον παλιό σύνδεσμο δεν θα υπάρχει πια πρόσβαση στο έργο. Θα πρέπει να μοιραστείς τον νέο σύνδεσμο με τα άτομα χωρίς λογαριασμό.
rotation-boxed =
    { $count ->
        [0] Κανένα άλλο μέλος δεν θα πάρει το κλειδί αυτόματα
        [one] { $count } μέλος θα πάρει το κλειδί αυτόματα
       *[other] { $count } μέλη θα πάρουν το κλειδί αυτόματα
    }
rotation-by-link-title = Άτομα στα οποία θα πρέπει να στείλεις τον νέο σύνδεσμο
rotation-by-link-hint = Αυτά τα άτομα δεν έχουν λογαριασμό. Θα τους στείλεις τον σύνδεσμο αμέσως μετά.
rotation-names-and = { $first } και { $last }
rotation-names-and-others = { $names } και { $count } ακόμη
rotation-confirm-remove = Αφαίρεση και αλλαγή κλειδιού
rotation-confirm-rotate = Αλλαγή κλειδιού
rotation-done-title = Το νέο κλειδί ισχύει
rotation-done-removed = { $name } δεν συμμετέχει πια στο έργο.
rotation-done-rotated = Το κλειδί του έργου άλλαξε.
rotation-done-boxed =
    { $count ->
        [0] Κανένα άλλο μέλος δεν πήρε το κλειδί αυτόματα.
        [one] { $count } μέλος πήρε το κλειδί αυτόματα.
       *[other] { $count } μέλη πήραν το κλειδί αυτόματα.
    }
rotation-new-link =
    { $count ->
        [0] Νέος σύνδεσμος του έργου
        [one] Νέος σύνδεσμος για αποστολή · { $count } άτομο χωρίς λογαριασμό
       *[other] Νέος σύνδεσμος για αποστολή · { $count } άτομα χωρίς λογαριασμό
    }
rotation-link-renewed = Ο σύνδεσμος ανανεώθηκε με επιτυχία.
rotation-done = Τέλος
history-key-changed = Το κλειδί του έργου άλλαξε
history-member-removed = { $name } δεν συμμετέχει πια στο έργο
project-rotated-row-hint = Το κλειδί άλλαξε. Ζήτα τον νέο σύνδεσμο.
