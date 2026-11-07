# Français - la copie d'origine de l'application.
#
# Doit définir exactement les mêmes clés que en.ftl (garde : `french_and_english_define_the_same_messages`).

### Common

loading = Chargement…
cancel = Annuler
discard-title = Abandonner les modifications ?
discard-message = Ce que vous avez saisi sera perdu.
discard = Abandonner
confirm = Confirmer
retry = Réessayer
delete = Supprimer
back = Retour
language = Langue
theme = Thème
theme-light = Clair
theme-dark = Sombre
theme-system = Système
theme-hint-device = Enregistré sur cet appareil uniquement.
theme-hint-system = Suit l'apparence de cet appareil.

### Navigation

nav-main = Navigation principale
nav-projects = Projets
nav-charts = Statistiques
nav-settings = Réglages

### Connectivity

offline-banner = Hors ligne
offline-pending =
    { $count ->
        [one] { $count } en attente
       *[other] { $count } en attente
    }

# Une clé par verbe plutôt qu'un nom interpolé : « la opération » n'élide pas.
sync-conflict-edit = Conflit : la modification de « { $name } » a échoué (élément supprimé). Ignorée.
sync-conflict-delete = Conflit : la suppression de « { $name } » a échoué (élément supprimé). Ignorée.
sync-conflict-other = Conflit : l'opération sur « { $name } » a échoué (élément supprimé). Ignorée.
sync-error = Erreur de synchronisation : { $reason }

### Errors

error-network = Impossible de contacter le serveur. Vérifiez votre connexion internet.
error-rate-limited = Le serveur est très sollicité. Réessayez dans un instant.
error-generic = Une erreur est survenue. Réessayez.

error-invalid-email = Cette adresse e-mail n'est pas valide.
error-invalid-password = Ce mot de passe n'est pas valide.
error-password-too-short = Le mot de passe doit contenir au moins 12 caractères.
error-client-outdated = Cette version de l'application est obsolète. Mettez-la à jour pour vous connecter.
error-invalid-link = Ce lien n'est pas valide.
error-batch-too-large = Trop d'éléments à la fois.
error-payers-required = Sélectionnez au moins un payeur.
error-debtors-required = Sélectionnez au moins un débiteur.
error-duplicate-participant = Un participant apparaît deux fois du même côté.
error-participant-not-in-project = Ce participant ne fait pas partie du projet.
error-too-many-participants = Trop de participants pour une seule dépense.
error-invalid-credentials = E-mail ou mot de passe incorrect.
error-unauthenticated = Connectez-vous pour effectuer cette action.
error-email-not-verified = Votre adresse e-mail n'est pas encore vérifiée.
error-project-not-found = Ce projet n'existe plus.
error-expense-not-found = Cette dépense n'existe plus.
error-storage-full = Le stockage est plein : la clé de ce projet n'a pas pu être enregistrée sur cet appareil. Conservez son lien de partage.
error-user-not-found = Ce participant n'existe plus.
error-tricount-not-found = Tricount introuvable ou erreur de son API.
error-too-many-members = Ce projet a atteint sa limite de participants.
error-identity-taken = Un autre compte a déjà revendiqué ce participant.
error-claim-proof-invalid = Cet appareil ne détient pas la clé du projet et ne peut donc pas revendiquer un participant. Rouvrez le lien de partage.
error-user-has-payments = Ce participant a des dépenses dans le projet et ne peut pas être retiré.
error-resend-cooldown = Attendez 60 secondes avant de redemander un e-mail.
error-self-friend-request = Vous ne pouvez pas vous ajouter vous-même en ami.
error-not-a-friend = Vous ne pouvez inviter que des personnes de votre liste d’amis.
error-friend-has-no-key = Cet ami n’a pas encore ouvert la dernière version de l’application. Demandez-lui de se connecter une fois, puis réessayez.
error-friend-request-not-found = Cette demande d’ami n’existe plus.
error-invitation-not-found = Cette invitation n’existe plus.
error-too-many-friend-requests = Trop de demandes d’amis pour le moment. Réessayez demain.
error-too-many-invitations = Trop d’invitations en attente.
error-invalid-kdf-salt = Les paramètres de chiffrement ne sont pas valides. Mettez l'application à jour et réessayez.
error-mixed-project-batch = Ces participants ne font pas tous partie du même projet.
error-invalid-payload = Cette version de l'application a envoyé des données que le serveur refuse. Mettez-la à jour et réessayez.
error-invalid-public-key = Votre clé de chiffrement n'est pas valide. Mettez l'application à jour et réessayez.
error-payment-methods-stale = Vos informations de paiement ont été modifiées sur un autre appareil. Rechargez et réessayez.

### Auth

field-email = E-mail
field-email-placeholder = vous@exemple.fr
field-password = Mot de passe
field-name = Nom
field-name-placeholder = Jean Dupont

login-title = Se connecter
login-submit = Se connecter
login-submitting = Connexion…
login-password-placeholder = Votre mot de passe
login-no-account = Pas encore de compte ?
login-unverified = Votre e-mail n'est pas encore vérifié. Vérifiez votre boîte mail ou renvoyez le lien.
login-maybe-unverified = Vous venez de vous inscrire ? Vérifiez d'abord votre adresse avec le lien reçu par e-mail, ou renvoyez-le.
login-resend = Renvoyer le lien de vérification
login-resending = Envoi…
login-resend-sent = E-mail envoyé ! Vérifiez votre boîte mail.

register-submit = Créer un compte
register-submitting = Création…
register-have-account = Déjà un compte ?
register-password-placeholder = 12 caractères minimum
register-password-warning = Notez votre mot de passe : nous ne le voyons jamais et ne pouvons pas vous le redonner.
register-check-email-title = Vérifiez votre e-mail
register-email-sent = E-mail envoyé !
register-email-sent-hint = Cliquez sur le lien dans votre boîte mail pour activer votre compte.
# Découpé autour d'un lien : Fluent ne peut pas contenir de composant.
register-not-received-prefix = Pas reçu ? Vérifiez vos spams ou
register-sign-in-link = connectez-vous
register-not-received-suffix = pour renvoyer le lien.
register-terms-prefix = En créant un compte, vous acceptez les
register-terms-link = conditions d'utilisation
register-terms-and = et notre
register-privacy-link = politique de confidentialité

settings-title = Réglages
settings-preferences = Préférences
settings-preferences-local = Enregistrées sur cet appareil.
settings-preferences-synced = Synchronisées avec votre compte, chiffrées.
settings-about = À propos
settings-anonymous-title = Vous n'êtes pas connecté
settings-upsell-title = Vos projets, sur tous vos appareils
settings-upsell-free = Gratuit
settings-upsell-body = Counted fonctionne sans compte. Avec un compte gratuit, vos projets et vos préférences vous suivent sur votre téléphone, votre ordinateur et le web - toujours chiffrés, toujours illisibles pour nous.
settings-locked-badge = Compte
settings-locked-friends = Créez un compte pour ajouter des amis et les inviter dans un projet depuis l'application - sans lien à faire circuler.
settings-locked-payment-methods = Enregistrez votre IBAN ou votre appli de paiement une fois et partagez-les avec les projets de votre choix. Ceux qui vous doivent de l'argent les voient à côté de votre nom.
settings-friends-hint = Ajoutez des amis et invitez-les dans vos projets sans partager de lien.

account-member-since = Membre depuis
account-logout = Se déconnecter
account-logging-out = Déconnexion…
account-delete-title = Supprimer mon compte
account-delete-warning = Suppression immédiate et définitive de votre compte et de vos informations.
account-delete-confirm-title = Supprimer le compte
account-delete-confirm-message = Votre compte, vos sessions et la liste de vos projets seront définitivement supprimés. Sans votre mot de passe, les données chiffrées d'un projet partagé deviennent illisibles pour vous - cette action est irréversible.
erase-device-open = Se déconnecter et effacer cet appareil
erase-device-title = Effacer cet appareil ?
erase-device-body = Les projets, leurs clés et le cache de cet appareil seront effacés, puis votre session sera fermée. Vos projets restent sur votre compte : reconnectez-vous pour les retrouver.
erase-device-checking = Vérification que vos projets sont sur votre compte…
erase-device-pending = Modifications en attente d'envoi : { $count }. Reconnectez-vous à internet, puis réessayez.
erase-device-unreachable = Impossible de vérifier que vos projets sont sur votre compte. Vérifiez votre connexion, puis réessayez.
erase-device-at-risk = Ces projets ne sont pas sur votre compte et seront perdus. Pour les garder, annulez et copiez d'abord leur lien :
erase-device-confirm = Effacer et se déconnecter
erase-device-confirm-anyway = Effacer quand même
erase-device-erasing = Effacement…

settings-payment-methods = Coordonnées de paiement
settings-payment-methods-hint = Comment vous souhaitez être remboursé. Chiffrées avec votre compte.
payment-method-kind = Moyen
payment-method-kind-other = Autre
payment-method-label = Nom
payment-method-label-placeholder = Compte principal
payment-method-value = Coordonnées
payment-method-value-placeholder = IBAN, numéro de téléphone, identifiant…
payment-method-add = Ajouter
payment-method-remove = Supprimer { $name }
payment-method-empty = Vous n'avez pas encore ajouté de coordonnées de paiement.
payment-method-deleted = Moyen de paiement supprimé.
payment-method-value-required = Renseignez les coordonnées de chaque moyen de paiement, ou supprimez-le.
payment-method-label-required = Donnez un nom à votre moyen personnalisé.
payment-method-too-long = C'est trop long - raccourcissez.
payment-method-invalid-characters = Retirez les retours à la ligne ou les caractères invisibles.
payment-method-limit = Vous pouvez enregistrer jusqu'à { $max } moyens de paiement.
payment-methods-saved = Coordonnées de paiement enregistrées.
payment-methods-offline = Vous devez être en ligne pour enregistrer vos coordonnées de paiement.
payment-methods-stale = Vos coordonnées de paiement ont été modifiées sur un autre appareil. Elles ont été rechargées - veuillez réessayer.
payment-methods-key-missing = Reconnectez-vous pour gérer vos coordonnées de paiement.
settings-payment-methods-share-warning = Un moyen partagé est visible par tous les membres de vos projets.
payment-method-share = Partager avec mes projets
payment-method-share-hint = Affiché à côté de votre nom quand quelqu'un vous doit de l'argent.
payment-method-copy = Copier { $name }
payment-method-copied = Copié.
payment-method-copy-failed = Impossible de copier - sélectionnez le texte et copiez-le à la main.

verify-email-checking = Vérification de votre e-mail…
verify-email-welcome = E-mail vérifié - bienvenue sur Counted !
verify-email-back-to-login = Retourner à la connexion
verify-email-intro = Saisissez votre mot de passe pour confirmer que cette adresse est bien la vôtre.
verify-email-submit = Vérifier mon adresse

### Project status

project-archive = Archiver
project-sheet-invite = Inviter
project-sheet-recurring = Récurrentes
project-sheet-edit = Modifier le projet
project-sheet-close = Clôturer le projet
project-sheet-archive = Archiver le projet
project-sheet-reopen = Réouvrir le projet
project-sheet-unarchive = Désarchiver le projet
project-sheet-leave = Quitter le projet

### Dates

date-long = { $day } { $month } { $year }

month-1 = janvier
month-2 = février
month-3 = mars
month-4 = avril
month-5 = mai
month-6 = juin
month-7 = juillet
month-8 = août
month-9 = septembre
month-10 = octobre
month-11 = novembre
month-12 = décembre

month-short-1 = Jan
month-short-2 = Fév
month-short-3 = Mar
month-short-4 = Avr
month-short-5 = Mai
month-short-6 = Juin
month-short-7 = Juil
month-short-8 = Aoû
month-short-9 = Sep
month-short-10 = Oct
month-short-11 = Nov
month-short-12 = Déc

### Actions

add = Ajouter
create = Créer
creating = Création…
edit = Modifier
leave = Quitter
close = Fermer
paste = Coller
join = Rejoindre
import = Importer
importing = Importation…
field-description = Description
field-date = Date
date-today = Aujourd'hui
date-yesterday = Hier
field-optional = Optionnel

### Projects

projects-filter-active = Actifs
projects-filter-all = Tous
projects-count-label = Projets
projects-empty = Aucun projet
projects-offline-banner = Données hors ligne - reconnectez-vous pour actualiser.
demo-banner = Projet de démonstration - lecture seule.
demo-start-own = Créer mon projet
demo-badge = Démo
demo-bar = Tout est effacé à la fermeture de l’onglet.
demo-leave = Quitter la démo
demo-preparing = Préparation de votre démo…
demo-failed = La démo n’a pas pu être préparée.
demo-sign-in-blocked = La connexion est désactivée dans la démo. Quittez-la pour vous connecter ou créer un compte ; vos propres projets sont intacts.
projects-no-local-data = Aucune donnée locale
projects-no-local-data-hint = Connectez-vous pour charger vos projets pour la première fois.
projects-add = Ajouter un projet
projects-create = Créer un projet
projects-personal-expense = Dépense perso
personal-expense-banner = Ajoutée à Mes dépenses
personal-create-title = Créer votre projet personnel
personal-create-hint = Suivez ce que vous dépensez seul
group-create-title = Créer un projet partagé
group-create-hint = Partagez les dépenses avec vos proches
personal-create-action = Créer
personal-create-signup = Créer un compte
personal-sheet-title = Votre projet personnel
personal-sheet-intro = Pour noter ce que vous dépensez seul. Il apparaît avec vos groupes dans les statistiques.
personal-sheet-point-alone = Un seul membre : vous, avec le nom de votre compte. Pas de lien de partage.
personal-sheet-point-encrypted = Chiffré sur vos appareils, comme vos autres projets.
personal-sheet-point-stats = Vos parts dans les groupes s’ajoutent à ces dépenses dans Statistiques.
personal-default-name = Mes dépenses
my-spending-prev-month = Mois précédent
my-spending-next-month = Mois suivant
my-spending-spent = Vous avez dépensé
my-spending-groups = Groupes
my-spending-income = Revenus
my-spending-balance = Solde du mois
my-spending-unconverted = Les parts dans une autre devise restent de côté tant qu’aucun taux n’est connu.
my-spending-empty = Rien ce mois-ci
my-spending-empty-hint = Ajoutez une dépense avec +. Vos parts dans les groupes apparaîtront ici automatiquement.
my-spending-share-of = sur { $amount }
my-spending-uncounted = non comptée
payer-you = vous
expense-received-by = reçu par
counted-switch = Compter dans mes dépenses
counted-switch-on = Activé : votre part s’ajoute à Mes dépenses
counted-switch-off = Désactivé : votre part n’apparaît pas dans Mes dépenses
counted-groups = Groupes comptés
counted-groups-intro = Votre part dans ces groupes s’ajoute à Mes dépenses et aux budgets.
counted-groups-converted = { $from }, converti en { $to }
counted-groups-encrypted = Ce choix est chiffré avec votre compte et suit vos appareils.
counted-groups-empty = Aucun groupe sur cet appareil pour l’instant.
counted-groups-done = OK
counted-save-failed = Ce choix n’a pas pu être enregistré. Réessayez.
tab-budget = Budget
budget-title = Budgets
budget-manage = Gérer
budget-left = Reste { $amount }
budget-over = Dépassé de { $amount }
budget-empty = Aucun budget
budget-empty-hint = Fixez un plafond par catégorie. Vos parts dans les groupes comptent aussi.
budget-define = Définir un budget
budget-sheet-title = Budgets mensuels
budget-sheet-intro = Sur votre part : perso et groupes. Laissez vide pour ne pas suivre une catégorie.
budget-average = Moy. 3 derniers mois : { $amount }
budget-for = Budget { $category }
budget-alert = Me prévenir à 80 %
budget-alert-hint = Calculé sur cet appareil
budget-invalid = Montant invalide pour { $category }.
budget-offline = Les budgets ne s’enregistrent qu’en ligne. Reconnectez-vous et réessayez.
budget-alert-near = { $category } : { $percent } % du budget du mois
budget-alert-over = { $category } : budget du mois dépassé
my-spending-search-summary =
    { $count ->
        [one] { $count } résultat
       *[other] { $count } résultats
    } · { $total } · perso et groupes, tous les mois
projects-join = Rejoindre un projet
projects-import-tricount = Importer depuis Tricount
project-actions = Actions sur le projet

status-ongoing = En cours
status-closed = Clôturé
status-archived = Archivé

nav-help = Aide
nav-privacy = Politique de confidentialité
nav-terms = Conditions d'utilisation
nav-legal = Mentions légales

leave-project-title = Quitter le projet ?
leave-project-message = Vous n'y aurez plus accès depuis cet appareil. S'il ne reste plus aucun membre, le projet et toutes ses dépenses seront définitivement supprimés.

add-project-title = Nouveau projet
add-project-name-label = Nom du projet
add-project-name-placeholder = Mon voyage, Coloc 2024…
add-project-participants = Liste des utilisateurs
add-project-participant-name = Nom du participant
add-project-participant-placeholder = Clark Kent
add-project-offline = Impossible de créer un projet hors ligne. Reconnectez-vous et réessayez.
add-project-name-required = Le nom du projet est requis.

join-link-label = Lien de partage
join-link-hint = Le lien contient la clé de déchiffrement - copiez-le en entier.
join-invalid-link = Lien invalide. Collez le lien de partage complet, avec la partie après le #.
join-wrong-project = Ce lien correspond à un autre projet.

import-tricount-link-label = Lien ou clé Tricount
import-tricount-key-required = Entrez un lien ou une clé Tricount.
import-tricount-encryption-failed = Erreur de chiffrement.
import-tricount-unimportable = Rien n'a été importé : ce Tricount contient des membres ayant un compte Tricount ou des montants qui ne tombent pas juste (entrées concernées : { $count }).

### Expenses

save = Enregistrer
saving = Enregistrement…
adding = Ajout…
link-copied = Lien copié
missing-encryption-key = Clé de chiffrement manquante.
missing-encryption-key-title = Clé de chiffrement manquante
missing-encryption-key-hint = Le lien que vous avez utilisé ne contient pas la clé nécessaire pour déchiffrer ce projet. Utilisez le lien complet partagé par le créateur du projet.
project-locked-hint = Cet appareil n’a pas la clé de ce projet. Ouvrez son lien de partage pour le déverrouiller.
expenses-unreadable = Certaines entrées sont illisibles sur cet appareil et ne sont pas comptées dans les totaux. Mettez l’application à jour pour les voir.
project-unlock = Déverrouiller
project-no-local-data-hint = Connectez-vous pour charger les données de ce projet pour la première fois.
project-gone-title = Ce projet n'existe plus
project-gone-hint = Il a été supprimé lorsque son dernier membre l'a quitté. Le lien de partage ne fonctionne plus, même si vous le rouvrez.

expense-add = Ajouter une dépense
transfer-add = Ajouter un transfert
expense-edit-title = Modifier la dépense
expense-category = Catégorie
expense-category-auto = Auto · { $emoji }
emoji-picker-label = Emoji
emoji-none = Sans emoji
expense-currency = Devise du montant
amount-op-add = Plus
amount-op-subtract = Moins
amount-op-multiply = Multiplier
amount-op-divide = Diviser
amount-op-equals = Égal
amount-op-done = Terminé
expense-rate = Taux de change (facultatif)
expense-rate-hint = Laissez vide pour appliquer le taux de la Commission européenne (InforEuro) de { $month } : 1 { $from } = { $rate } { $to }.
expense-rate-invalid = Saisissez un taux de change supérieur à 0.
expense-rate-unavailable = Aucun taux automatique disponible - saisissez-le à la main.
expense-delete-title = Supprimer la dépense
expense-delete-message = « { $name } » sera définitivement supprimée. Cette action est irréversible.
expense-inconsistent-amounts = Montants incohérents
expenses-empty = Aucune dépense
expenses-empty-hint = Commencez par ajouter des dépenses en cliquant sur le bouton ci-dessous
expenses-show-more = Afficher plus ({ $count } restantes)

expense-type-expense = Dépense
expense-type-transfer = Transfert
expense-type-gain = Gain
expense-type-group = Type
expense-paid-by = payée par
expense-sent-by = envoyé par
expense-contributed-by = contribué par

expense-name-required = Le nom est requis.
expense-amount-not-positive = Le montant doit être supérieur à 0.
expense-no-payer = Sélectionnez au moins un payeur.
expense-no-debtor = Sélectionnez au moins un débiteur.
expense-invalid-date = Date invalide.
# Une clé par côté : l'accord grammatical diffère (« des payeurs » / « des débiteurs »).
expense-payers-mismatch = Le total des payeurs ({ $sum }) ne correspond pas au montant de la dépense ({ $total }).
expense-debtors-mismatch = Le total des débiteurs ({ $sum }) ne correspond pas au montant de la dépense ({ $total }).

participants-none = Personne
participants-everyone = Tout le monde ({ $count })
participants-some = { $count } sur { $total }
participants-select-all = Tout sélectionner
participants-by-shares = Par parts
split-amounts = Montants
participants-remaining = Reste { $amount }
participants-over-by = Dépasse de { $amount }
participants-who-paid = Qui a payé ?
participants-who-received = Qui a reçu ?
participants-who-transfers = Qui transfère ?
participants-who-receives = Qui reçoit ?
participants-for-whom = Pour qui ?

stats-total-expenses = Total des dépenses
stats-my-expenses = Mes dépenses

tab-expenses = Dépenses
tab-balance = Équilibre
tab-reimbursements = Remboursements
balance-gets-back = En positif
balance-owes = En négatif
balance-settled = À l’équilibre
reimbursements-empty-title = Les comptes sont bons !
reimbursements-empty-hint = Des suggestions de remboursement seront proposées ici si les comptes ne sont pas équilibrés
reimbursement-owes = { $debtor } doit à { $creditor }
reimbursement-record = Rembourser
settlement-recorded = Remboursement enregistré.
reimbursement-pay-with = Payer
reimbursement-pay-shared-by = Partagé par { $name } - vérifiez le nom du destinataire affiché par votre application avant d'envoyer.
reimbursement-pay-title = Payer { $name }
reimbursements-mine-title = À régler
reimbursements-others-title = Autres remboursements
copy = Copier

user-selection-title = Quel utilisateur êtes-vous ?
user-selection-hint = Sélectionnez votre nom dans la liste.
user-selection-required = Veuillez sélectionner un utilisateur.
identity-claimed = Rattaché à un compte
identity-claimed-by = Compte de { $name }
identity-taken-repick = Un autre compte a revendiqué le participant que vous utilisiez. Veuillez en choisir un autre.
participant-gone-repick = Le participant que vous utilisiez a été retiré de ce projet. Veuillez en choisir un autre.

edit-project-title = Modifier le projet
edit-project-new-badge = nouveau
edit-project-deferred-new-members = l'ajout de nouveaux membres
edit-project-deferred-removals = la suppression de membres
edit-project-offline-deferred = Hors ligne : { $items } sera appliqué à la reconnexion.

export-failed = Erreur d'export : { $reason }

# Les entrées d'historique sont chiffrées et stockées : elles gardent la langue de leur auteur.
history-expense-added = Dépense ajoutée : { $name }
history-expense-edited = Dépense modifiée : { $name }
history-expense-deleted = Dépense supprimée : { $name }
move-expense = Déplacer vers un projet
move-expense-title = Déplacer vers…
move-current = Actuel
move-converted = { $currency }, converti au taux du mois
move-note-personal = Vous choisirez qui a payé et pour qui à l’étape suivante. La dépense quittera Mes dépenses.
move-note-group = La dépense quittera { $project } pour tous ses membres, et l’équilibre du groupe sera recalculé. Vous choisirez qui a payé et pour qui à l’étape suivante.
move-banner = Depuis { $from } vers { $to }
move-submit = Déplacer
move-done = Dépense déplacée vers { $project }
history-expense-moved = Dépense déplacée vers un autre projet : { $name }
personal-badge = Perso
move-members =
    { $count ->
        [one] { $count } membre
       *[other] { $count } membres
    }
history-project-edited = Projet modifié : { $name }
history-name-changed = Nom : « { $from } » → « { $to } »
history-description-added = Description ajoutée : « { $value } »
history-description-removed = Description supprimée : « { $value } »
history-description-changed = Description : « { $from } » → « { $to } »

### Sweep

field-amount = Montant
expense-name-placeholder = Restaurant, courses…
expense-actions = Actions sur la dépense
expense-your-share = Votre part
expense-your-share-value = Votre part : { $amount } { $currency }
expense-share-percent = { $percent } %
expense-added-by = Ajouté par { $name } le { $date }
expense-inconsistent-detail = Montants incohérents : { $paid } payé, { $owed } dû, pour une dépense de { $total }. Modifiez la dépense pour la corriger.
missing-access-key = Clé d'accès manquante. Visitez ce projet via son lien de partage.
filter-all = Tous
filter-my-payments = Mes paiements
filter-my-debts = Mes dettes
expense-filter-group = Filtre
expenses-search = Rechercher une dépense
expenses-no-match = Aucune dépense ne correspond.
expenses-search-open = Rechercher
expenses-search-clear = Effacer la recherche
expenses-search-summary =
    { $count ->
        [one] { $count } résultat
       *[other] { $count } résultats
    } · { $total } · tous les mois
participants-shares-for = Parts de { $name }
participants-amount-for = Montant pour { $name }
reimbursement-add = Ajouter un remboursement
project-forget = Retirer de ma liste
project-history-title = Historique
history-kind-add = Ajout
history-kind-delete = Suppression
history-kind-edit = Modification
export = Exporter
export-json = Exporter JSON
export-csv = Exporter CSV
share-link = Partager
copy-link-failed = Impossible de copier le lien
open-in-app = Ouvrir dans l'application
not-found-title = Page introuvable
not-found-back = Retour aux projets

### Charts

charts-period = Période
# À garder court : cinq segments égaux doivent tenir en 360px.
period-all = Tout
period-month = Mois
period-3months = 3 mois
period-year = Année
period-custom = Perso
charts-tab-categories = Catégories
charts-tab-trends = Tendances
charts-total-spent = Total dépensé
charts-avg-per-person = Moy. par personne
charts-expense-count =
    { $count ->
        [one] { $count } dépense
       *[other] { $count } dépenses
    }
charts-nothing-to-show = Aucune dépense à afficher
charts-my-share-skipped =
    { $count ->
        [one] 1 projet n’est pas compté - aucun participant choisi, ou ses données n’ont pas pu être chargées.
       *[other] { $count } projets ne sont pas comptés - aucun participant choisi, ou leurs données n’ont pas pu être chargées.
    }

### Categories

category-food = Nourriture
category-transport = Transport
category-accommodation = Hébergement
category-leisure = Loisirs
category-shopping = Shopping
category-services = Services
category-parties-gifts = Fêtes & Cadeaux
category-other = Autres
categories-manage = Gérer les catégories
categories-title = Catégories
categories-yours = Vos catégories
categories-builtin = De base
categories-add = Ajouter une catégorie
categories-in-parent = dans { $parent }
categories-renamed-from = renommée · { $name } à l’origine
categories-hidden-hint = masquée · ses dépenses vont dans { $other }
categories-always-shown = toujours affichée
categories-hidden = Masquée
categories-new = Nouvelle catégorie
categories-edit = Modifier la catégorie
categories-parent = Comptée dans
categories-parent-hint = Dans les statistiques de tous vos projets, { $name } compte dans { $parent }.
categories-create = Créer
categories-hide = Masquer cette catégorie
categories-hide-hint = Ses dépenses s’affichent dans { $other } pour tous les membres.
categories-delete-title = Supprimer « { $name } » ?
categories-delete-body = Les dépenses concernées ({ $count }) reviennent dans { $parent }. Aucune dépense n’est supprimée.
categories-delete-body-unknown = Ses dépenses reviennent dans { $parent }. Aucune dépense n’est supprimée.
categories-delete-warning = La catégorie disparaît pour tous les membres du projet.
categories-summary-created = Créées : { $count }
categories-summary-renamed = Renommées : { $count }
categories-summary-hidden = Masquées : { $count }
categories-summary-default = Pas de catégories personnalisées
categories-offline = Gérer les catégories demande une connexion. Reconnectez-vous et réessayez.
categories-error-too-many = Un projet compte au plus 20 catégories créées.
categories-error-full = Ce projet a atteint sa limite de catégories.
categories-error-name = Donnez un nom à la catégorie.
categories-error-missing = Cette catégorie a changé entre-temps. Fermez et réessayez.
history-category-created = Catégorie créée : { $name }
history-category-edited = Catégorie modifiée : { $name }
history-category-deleted = Catégorie supprimée : { $name }
charts-project = Projet
charts-all-projects = Tous les projets
charts-date-from = Du
charts-date-to = Au
charts-total = Total
charts-tab-people = Personnes
charts-tab-projects = Projets
charts-scope = Dépenses de qui
charts-scope-group = Groupe
charts-scope-me = Moi
charts-currency = Devise
charts-my-share = Ma part
charts-share-of-total = { $pct } % de { $total }
charts-i-paid = J’ai payé
charts-paid-more = { $amount } de plus que ma part
charts-paid-less = { $amount } de moins que ma part
charts-paid-even = Pile ma part
charts-part-title = Votre part dans chaque catégorie
charts-part-desc = En gris ce que le groupe a dépensé, en couleur ce que vous avez consommé.
charts-breakdown-title = Répartition par catégorie
charts-breakdown-desc = Touchez une part ou une ligne pour voir ses dépenses.
charts-of-total = { $amount } sur { $total }
charts-show-all = Tout afficher ({ $count })
charts-show-less = Afficher moins
charts-spend-title = Dépenses dans le temps
charts-group-by = Grouper par
bucket-day = Jour
bucket-week = Semaine
bucket-month = Mois
charts-avg = moy.
charts-cat-title-day = { $category }, jour par jour
charts-cat-title-week = { $category }, semaine par semaine
charts-cat-title-month = { $category }, mois par mois
charts-cat-desc = Choisissez une catégorie pour la suivre dans le temps.
charts-running-title = Cumul
charts-running-desc = Depuis le { $date }.
charts-avg-per-day = { $amount } / jour en moyenne
charts-avg-per-week = { $amount } / semaine en moyenne
charts-avg-per-month = { $amount } / mois en moyenne
charts-people-title = Qui a porté le groupe
charts-people-desc = Ce que chacun a payé, à côté de ce qu’il a consommé.
charts-paid = Payé
charts-fair-share = Juste part
charts-you = (vous)
charts-net-more = a payé plus
charts-net-less = a payé moins
charts-balance-title = Votre solde dans le temps
charts-balance-desc = Au-dessus de la ligne, le groupe vous doit. En dessous, vous devez au groupe.
charts-owed = On vous doit
charts-owe = Vous devez
charts-projects-title = Votre part, par projet
charts-projects-desc = Les totaux restent par devise.
history-empty = Aucun événement
history-by = Par { $name }
not-found-hint = Cette page n'existe pas ou a été déplacée.
payers-title-paid-by = Payé par
payers-title-sender = Émetteur
payers-title-contributors = Contributeurs
debtors-title-debtors = Débiteurs
debtors-title-recipients = Destinataires
debtors-title-beneficiaries = Bénéficiaires
debtors-title-for = Pour
debtors-title-recipient = Destinataire
expense-split-between = partagé entre { $count }
expense-people =
    { $count ->
        [one] { $count } personne
       *[other] { $count } personnes
    }

### Welcome

welcome-title = Vos comptes ne regardent que vous.
welcome-subtitle = Partagez vos dépenses entre amis.
welcome-note = Gratuit. Pas besoin de compte. Pas de pub.
welcome-link-title = Un lien, et tout le monde participe.
welcome-link-body = Aucun compte à créer, pour personne.
welcome-link-account = Un compte ? Jamais obligatoire. Il sert à retrouver vos projets sur un autre appareil, inviter vos amis depuis l'app et partager vos coordonnées de paiement.
welcome-demo-project = Week-end à Lyon
welcome-private-title = Personne ne peut lire vos comptes. Pas même nous.
welcome-private-body = Noms, montants, projets : tout est chiffré sur votre appareil. Vous seul avez la clé.
welcome-private-names = Noms
welcome-private-amounts = Montants
welcome-private-projects = Projets
welcome-scan-title = Prenez le ticket en photo.
welcome-scan-body = Montant, date et catégorie se remplissent automatiquement. Tout se passe sur votre téléphone. La photo n'est pas conservée.
welcome-eu-title = 100 % européen
welcome-no-ads = Zéro pub
welcome-no-trackers = Zéro traqueur
welcome-step = Étape { $current } sur { $total }
welcome-next = Suivant
welcome-skip = Passer
welcome-start = Commencer
welcome-how-it-works = Comment ça marche exactement ?

### Help

help-intro = Une question fréquente ? Cliquez pour développer la réponse.
help-create-project-q = Comment créer un projet ?
help-create-project-a = Depuis l'écran d'accueil, appuyez sur le bouton + en bas de l'écran. Donnez un nom au projet, choisissez sa devise et c'est parti.
help-add-participants-q = Comment ajouter des participants ?
help-add-participants-a = Ouvrez le projet puis ajoutez les participants depuis la liste des membres. Chaque participant peut payer ou devoir lors d'une dépense.
help-share-project-q = Comment partager un projet ?
help-share-project-a = Partagez simplement l'URL du projet (visible dans la barre d'adresse). Toute personne disposant du lien peut consulter et modifier le projet.
help-add-expense-q = Comment ajouter une dépense ?
help-add-expense-a = Dans un projet, appuyez sur +, saisissez le montant, indiquez qui a payé et entre qui répartir la dépense. Vous pouvez aussi choisir une date différente de celle d'aujourd'hui.
help-types-q = Quelle est la différence entre dépense, transfert et gain ?
help-types-expense = - un achat fait par une personne et réparti entre plusieurs.
help-types-transfer = - un remboursement d'une personne à une autre, sans répartition.
help-types-gain = - une somme reçue (remboursement, cadeau) à répartir entre plusieurs personnes.
help-past-date-q = Puis-je dater une dépense dans le passé ?
help-past-date-a = Oui, le champ « date » est libre. La date de création de l'enregistrement est conservée séparément.
help-who-owes-q = Comment Counted calcule-t-il qui doit quoi ?
help-who-owes-a = Counted calcule le solde net de chaque participant (somme avancée moins somme due), puis propose la série de virements la plus courte pour solder tous les comptes.
help-minimal-transfers-q = Pourquoi le nombre de virements suggérés est-il minimal ?
help-minimal-transfers-a = L'algorithme apparie d'abord les soldes qui se compensent exactement, puis traite les autres du plus gros créditeur au plus gros débiteur. Résultat : moins de transferts à effectuer pour tout solder.
help-import-tricount-q = Comment importer un projet depuis Tricount ?
help-import-tricount-a = Depuis l'écran d'accueil, appuyez sur le bouton « + » en bas de l'écran puis
help-import-tricount-b = Collez le lien de partage du Tricount à importer.
help-encryption-q = Mes données sont-elles chiffrées ?
help-encryption-a = Oui. Counted combine deux garanties :
help-encryption-e2ee-term = Chiffrement de bout en bout
help-encryption-e2ee-def = - toutes les communications entre le serveur et vous sont chiffrées.
help-encryption-zero-term = Zéro accès
help-encryption-zero-def = - vous chiffrez les données avant de nous les envoyer, le serveur ne stocke que des données chiffrées. Nous n'avons donc pas la possibilité de les lire.
help-encryption-see = Voir la
help-forgot-password-q = Que se passe-t-il si j'oublie mon mot de passe ?
help-forgot-password-warning = Personne ne peut le retrouver pour vous.
help-forgot-password-a = Votre mot de passe protège vos clés de chiffrement et ne nous est jamais transmis : personne (pas même nous) ne peut déchiffrer vos projets à votre place. Conservez-le précieusement, idéalement dans un gestionnaire de mots de passe.
help-archive-delete-q = Comment archiver ou supprimer un projet ?
help-archive-delete-a = Depuis l'écran du projet, ouvrez le menu et choisissez
help-archive-delete-b = pour le masquer tout en le conservant. Un projet n'est supprimé définitivement que lorsque son dernier membre le quitte.
help-delete-account-q = Comment supprimer mon compte ?
help-delete-account-a = Ouvrez les Réglages et utilisez « Supprimer mon compte ». La suppression est immédiate et irréversible.
help-contact = Une autre question ? Écrivez-nous à

### Legal

legal-updated = Dernière mise à jour : 21 octobre 2026

legal-publisher-title = 1. Éditeur du site
legal-publisher-body = Counted (« le Service ») est édité à titre non professionnel par une personne physique, Jonathan Bosi.
legal-contact-label = Contact :
legal-publisher-address-note = Conformément au II de l'article 1-1 de la loi n° 2004-575 du 21 juin 2004 pour la confiance dans l'économie numérique, l'éditeur, personne physique éditant à titre non professionnel, ne publie pas son adresse postale. Ces éléments d'identification sont détenus par l'hébergeur, qui peut les communiquer à l'autorité judiciaire.
legal-director-title = 2. Directeur de la publication
legal-host-title = 3. Hébergeur
legal-host-address = Industriestr. 25, 91710 Gunzenhausen, Allemagne
legal-host-phone = Téléphone : +49 (0)9831 505-0
legal-host-email-note = Les emails transactionnels sont acheminés par Scaleway SAS (France). Les serveurs et la base de données sont situés dans l'Union européenne.
legal-ip-title = 4. Propriété intellectuelle
legal-ip-body = La structure du site, ses textes et ses éléments graphiques sont la propriété de l'éditeur, sauf mention contraire. Les données que vous saisissez restent les vôtres : elles sont chiffrées sur votre appareil et l'éditeur ne peut ni les lire, ni les exploiter. La politique de confidentialité nomme les seules données qui traversent notre serveur en clair.
legal-ip-source = Le code source des applications web et mobiles est un logiciel libre, publié sous la licence GNU Affero General Public License, version 3 uniquement, à l'adresse
legal-ip-brand = Le nom « Counted », son logo et le domaine counted.fr ne sont pas couverts par cette licence.
legal-ip-third-party = Les applications intègrent des logiciels tiers, chacun sous sa propre licence :
legal-licenses-title = Licences des logiciels tiers
legal-licenses-unavailable = La liste des licences n'a pas pu être chargée. Elle figure aussi dans le code source, dans packages/ui/licenses/third-party.txt.
legal-personal-data-title = 5. Données personnelles
legal-personal-data-body = Le traitement des données personnelles est décrit dans la
legal-terms-title = 6. Conditions d'utilisation
legal-terms-body = L'usage du Service est régi par les
legal-report-title = 7. Signalement
legal-report-body-a = Tout contenu ou comportement susceptible de contrevenir à la loi peut être signalé à
legal-report-body-b = Le contenu des projets étant chiffré de bout en bout, l'éditeur n'y a pas accès et ne peut agir que sur les comptes concernés.

terms-intro = Counted est un service gratuit de partage de dépenses entre proches. Ces conditions décrivent ce que le Service fait, ce qu'il ne fait pas, et ce que chacun s'engage à faire.
terms-purpose-title = 1. Objet
terms-purpose-body = Les présentes conditions régissent l'utilisation du service Counted, accessible sur counted.fr et via ses applications mobiles. L'éditeur du Service est identifié dans les
terms-acceptance-title = 2. Acceptation
terms-acceptance-body = L'utilisation du Service vaut acceptation des présentes conditions. Si vous ne les acceptez pas, n'utilisez pas le Service.
terms-access-title = 3. Accès au Service
terms-access-body = Le Service est gratuit. Un projet peut être créé et partagé sans compte : toute personne disposant du lien de partage peut y accéder. La création d'un compte est facultative et sert à retrouver vos projets d'un appareil à l'autre.
terms-account-title = 4. Compte et mot de passe
terms-account-body = Vous êtes responsable de la confidentialité de votre mot de passe et des actions effectuées depuis votre compte.
terms-account-key-point = Point essentiel :
terms-account-warning-a = vos clés de chiffrement sont protégées par votre mot de passe et ne quittent jamais votre appareil en clair. En cas d'oubli,
terms-account-warning-em = l'éditeur ne peut rien récupérer pour vous
terms-account-warning-b = - aucune procédure ne lui donne accès à vos données ni à votre mot de passe.
terms-acceptable-use-title = 5. Usage acceptable
terms-acceptable-use-intro = Vous vous engagez à ne pas :
terms-acceptable-use-1 = utiliser le Service à des fins illicites ou frauduleuses ;
terms-acceptable-use-2 = tenter d'accéder à des projets qui ne vous ont pas été partagés ;
terms-acceptable-use-3 = porter atteinte au fonctionnement du Service, notamment par des requêtes automatisées massives ;
terms-acceptable-use-4 = contourner les limitations techniques mises en place (quotas, limitation de débit) ;
terms-acceptable-use-5 = utiliser le Service pour envoyer des messages non sollicités à des tiers, y compris via les demandes d'ami ou les invitations à un projet.
terms-your-content-title = 6. Vos contenus
terms-your-content-body = Vous restez propriétaire des données que vous saisissez. Elles sont chiffrées sur votre appareil avant d'être transmises : l'éditeur en héberge une version qu'il ne peut pas lire, et n'exerce donc aucune modération de leur contenu. Vous êtes seul responsable de ce que vous y inscrivez et de votre droit à y faire figurer des informations concernant d'autres personnes.
terms-availability-title = 7. Disponibilité
terms-availability-body = Le Service est fourni « en l'état », sans garantie de disponibilité ni d'absence d'erreur. Il peut être interrompu, modifié ou arrêté à tout moment, notamment pour maintenance. Les calculs de répartition et de remboursement sont fournis à titre indicatif : ils ne constituent ni un service de paiement, ni un conseil financier ou comptable, et aucun mouvement d'argent ne transite par le Service.
terms-deletion-title = 8. Suppression
terms-deletion-account = Vous pouvez supprimer votre compte à tout moment depuis la page « Réglages ». La suppression est immédiate et définitive.
terms-deletion-project = Un projet vit tant qu'un membre le conserve : lorsque le dernier membre le quitte, il est supprimé automatiquement, ainsi que les dépenses qu'il contient. Les dépenses que vous avez saisies dans un projet partagé restent en revanche visibles par les autres membres après votre départ - elles font partie de leurs comptes.
terms-backup-title = 9. Sauvegarde
terms-backup-body = Des sauvegardes techniques sont réalisées pour la continuité du Service, mais elles ne constituent pas un service d'archivage à votre bénéfice. Exportez vos projets (CSV ou JSON) si vous souhaitez en conserver une copie.
terms-liability-title = 10. Responsabilité
terms-liability-body = Le Service étant fourni gratuitement et à titre non professionnel, la responsabilité de l'éditeur ne saurait être engagée pour les dommages indirects résultant de son utilisation, notamment la perte de données consécutive à l'oubli d'un mot de passe. Les dispositions légales impératives protégeant les consommateurs restent applicables.
terms-liability-suspension = L'éditeur peut suspendre ou clôturer un compte en cas de manquement caractérisé à l'article 5.
terms-personal-data-title = 11. Données personnelles
terms-changes-title = 12. Modification des conditions
terms-changes-body = Ces conditions peuvent évoluer. Toute modification substantielle sera signalée dans l'application ou par email.
terms-law-title = 13. Droit applicable
terms-law-body-a = Les présentes conditions sont soumises au droit français. En cas de différend, une solution amiable sera recherchée en priorité en écrivant à
terms-law-body-b = À défaut, les tribunaux français sont compétents.

privacy-intro-a = Counted est un service de partage de dépenses entre amis, conçu autour d'un principe simple :
privacy-intro-em = le serveur ne voit jamais vos données en clair.
privacy-intro-b = Cette politique décrit quelles informations nous collectons, pourquoi, et quels droits vous avez.
privacy-controller-title = 1. Éditeur et responsable du traitement
privacy-controller-a = Counted est édité par Jonathan Bosi - voir les
privacy-controller-b = Pour toute question relative à vos données personnelles, vous pouvez écrire à
privacy-collected-title = 2. Données collectées
privacy-collected-email-term = Adresse email
privacy-collected-email-def = (obligatoire pour la création de compte).
privacy-collected-hash-term = Empreinte du mot de passe
privacy-collected-hash-def = votre appareil transforme le mot de passe en une preuve de connexion avec argon2id et n'envoie que cette preuve ; nous en stockons une empreinte. Le mot de passe lui-même n'est jamais transmis ni stocké.
privacy-collected-salt-term = Sel de dérivation de clé
privacy-collected-salt-def = utilisé pour générer votre clé de chiffrement côté client.
privacy-collected-content-term = Contenu chiffré des projets
privacy-collected-content-def = (noms, dépenses, participants, montants). Ces données sont chiffrées sur votre appareil avant d'être envoyées au serveur, qui n'en stocke qu'une version inintelligible.
privacy-collected-prefs-term = Vos préférences chiffrées
privacy-collected-prefs-def = (langue de l'interface, groupes exclus de « Mes dépenses »), pour les retrouver sur vos autres appareils. Chiffrées sur votre appareil avec la même clé que le reste : le serveur ne peut pas les lire.
privacy-collected-keys-term = Vos clés de projet, scellées
privacy-collected-keys-def = pour qu'une connexion depuis un nouvel appareil vous rende des projets réellement lisibles, et non une liste que vous ne pouvez pas ouvrir. Chacune est chiffrée sur votre appareil avec la clé de votre compte, elle-même protégée par votre mot de passe ; le serveur les conserve sans pouvoir les desceller. La contrepartie est réelle et nous préférons l'énoncer : quelqu'un qui détiendrait à la fois notre base et votre mot de passe atteindrait vos projets. C'est pourquoi le mot de passe n'est jamais transmis ni stocké : ce que nous recevons à la connexion est une preuve distincte dont il ne peut être retrouvé, et nous n'en gardons qu'une empreinte.
privacy-collected-payment-term = Vos coordonnées de paiement
privacy-collected-payment-def = (IBAN, identifiants de paiement), uniquement si vous les saisissez, pour que les personnes avec qui vous partagez des dépenses sachent comment vous rembourser. Chiffrées sur votre appareil avec la clé de votre compte ; un moyen que vous choisissez de partager dans un projet est aussi chiffré avec la clé de ce projet, afin que seuls ses membres puissent le lire. Le serveur ne peut lire aucune des deux copies.
privacy-collected-anon-term = Vos adhésions sans compte
privacy-collected-anon-def = lorsque vous ouvrez un projet sans être connecté : un identifiant aléatoire, tiré à nouveau pour chaque projet, enregistré auprès de ce projet afin qu'il ne soit pas supprimé tant que quelqu'un l'utilise. Il ne vous identifie pas et ne relie pas à lui seul vos projets entre eux.
privacy-collected-invite-term = Empreinte de l'email d'un participant invité
privacy-collected-invite-def = (SHA-256), enregistrée par les versions précédentes de l'application lorsque vous invitiez quelqu'un par email. Elle n'est plus collectée ; les empreintes déjà enregistrées disparaissent avec leur participant.
privacy-collected-friends-term = Vos amis
privacy-collected-friends-def = si vous utilisez la liste d'amis : avec quels comptes vous êtes amis, une empreinte à clé secrète (HMAC-SHA256) d'une adresse à qui vous avez envoyé une demande avant qu'elle n'ait de compte - une demande non acceptée sous 30 jours est supprimée - et quel ami vous avez invité dans quel projet, et en tant que quel participant. Le nom que vous donnez à une demande et la clé de projet qu'une invitation transporte sont chiffrés sur votre appareil - la clé avec la clé publique de votre ami, que nous stockons en clair car elle est publique par nature - et le serveur ne peut lire ni l'un ni l'autre. La moitié privée de votre propre paire de clés est conservée chiffrée avec la clé de votre compte.
privacy-collected-push-term = Jeton de notification
privacy-collected-push-def = si vous autorisez les notifications dans l'application mobile : l'adresse que le service de notification de votre téléphone lui attribue (sur Android, avec deux clés publiques de chiffrement), rattachée à votre compte, ainsi que la langue de votre téléphone pour rédiger la notification dans cette langue. Supprimé à la déconnexion, lorsque le service de notification le signale comme invalide, et avec votre compte.
privacy-collected-logs-term = Journaux techniques
privacy-collected-logs-def = (adresse IP, user-agent, horodatage) conservés pour la sécurité et la prévention des abus.
privacy-purposes-title = 3. Finalités
privacy-purpose-1 = Authentifier vos sessions et protéger votre compte.
privacy-purpose-2 = Synchroniser vos projets entre vos appareils.
privacy-purpose-3 = Vous envoyer un email de vérification lors de l'inscription.
privacy-purpose-4 = Détecter et prévenir les abus (bruteforce, scraping).
privacy-purpose-5 = Si vous avez autorisé les notifications, prévenir votre téléphone d'une demande d'ami ou de son acceptation, d'une invitation à un projet ou de la clôture d'un projet.
privacy-purpose-6 = Connaître la portée du service, par des totaux quotidiens qui ne nomment personne : les inscriptions, et la langue d'interface des inscriptions et des connexions.
privacy-legal-basis-title = 4. Base légale
privacy-legal-basis-body = Le traitement repose sur l'exécution du contrat qui nous lie (création et fourniture du service), sur notre intérêt légitime à sécuriser la plateforme, et sur notre intérêt légitime à connaître sa portée, par des totaux qui ne nomment personne.
privacy-e2ee-title = 5. Chiffrement de bout en bout
privacy-e2ee-a = Counted applique un modèle
privacy-e2ee-b = : vos clés de chiffrement sont protégées par votre mot de passe et ne quittent jamais votre appareil en clair. Le serveur stocke uniquement des blobs chiffrés qu'il est incapable de lire.
privacy-e2ee-metadata = Ce que le serveur voit en clair, c'est ce dont il a besoin pour faire fonctionner le service : votre adresse email, les projets auxquels votre compte appartient et le participant que vous êtes dans chacun, le nombre de dépenses d'un projet, la date d'ajout et la taille approximative de chacune, et le moment où un projet change. Il ne voit ni nom, ni montant, ni catégorie, ni note, ni qui a payé ou qui doit dans une dépense. Une réserve, le temps de la transition : dans un projet enregistré par une version antérieure de l'application et qu'aucun membre n'a rouvert depuis avec une version à jour, le serveur voit encore quels participants figurent dans chaque dépense, lequel l'a saisie et lequel a fait chaque modification de l'historique - jamais qui a payé ni qui doit, ni les montants. La première ouverture avec une version à jour efface ces liens ; les sauvegardes les conservent au plus 30 jours de plus. Une exception : un projet que vous importez depuis Tricount transite en clair par notre serveur jusqu'à votre appareil, qui le chiffre ensuite. Il n'est ni conservé ni journalisé.
privacy-e2ee-consequence-label = Conséquence importante :
privacy-e2ee-consequence-a = si vous oubliez votre mot de passe,
privacy-e2ee-consequence-em = personne
privacy-e2ee-consequence-b = - pas même nous - ne peut déchiffrer vos données à votre place.
privacy-processors-title = 6. Sous-traitants et hébergement
privacy-processor-hetzner = (Allemagne, UE) - hébergement des serveurs et de la base de données.
privacy-processor-scaleway = (France, UE) - envoi des emails transactionnels (vérification d'email).
privacy-processor-tricount = - contacté uniquement si vous déclenchez vous-même un import depuis Tricount, afin de récupérer le projet que vous souhaitez importer. Notre serveur effectue la requête et transmet le projet en clair à votre appareil, sans le conserver ni le journaliser ; votre appareil le chiffre ensuite comme tous les autres.
privacy-processor-grafana = (région UE) - supervision technique du serveur. Seules des métriques d'infrastructure (processeur, mémoire, disque, état des services) et des totaux d'usage quotidiens (nombre de comptes, de projets, d'adhésions et d'inscriptions, et le nombre d'inscriptions et de connexions effectuées dans chaque langue d'interface) y sont envoyés : ni contenu, ni identifiant, ni adresse IP de visiteur.
privacy-processor-proton = (Suisse) - stockage de la copie externe des sauvegardes de notre base de données. La copie est chiffrée avant de quitter notre serveur, avec une clé que Proton ne détient pas.
privacy-processor-apple = (Apple Push Notification service) - uniquement si vous autorisez les notifications sur iPhone : reçoit le jeton de votre téléphone et une phrase générique comme « Nouvelle demande d'ami », jamais un nom, un montant ou un projet.
privacy-processor-android-push-term = Le service de notification de votre téléphone Android
privacy-processor-android-push-def = (Google Firebase Cloud Messaging sur la plupart des téléphones, ou le distributeur UnifiedPush que vous avez installé) - uniquement si vous autorisez les notifications sur Android : reçoit un message chiffré pour votre seul téléphone et complété à une taille fixe, si bien qu'il ne peut même pas savoir de quel type de notification il s'agit.
privacy-no-transfer-outside-eu = Ces deux services de notification peuvent traiter des données hors de l'Union européenne : Google y applique le cadre de protection des données UE - États-Unis (Data Privacy Framework), reconnu adéquat par la Commission européenne, et Apple les clauses contractuelles types de la Commission. Le distributeur UnifiedPush est celui que vous avez choisi. La copie de sauvegarde est conservée en Suisse, que la Commission européenne reconnaît comme offrant une protection adéquate. En dehors d'eux, aucune donnée personnelle n'est transférée hors de l'Union européenne.
privacy-retention-title = 7. Durée de conservation
privacy-retention-a = Vos données sont conservées tant que votre compte est actif. La suppression du compte entraîne une suppression
privacy-retention-em = immédiate et définitive
privacy-retention-b = des enregistrements correspondants en base (suppression en cascade, sans corbeille).
privacy-sweep-intro = Un nettoyage automatique quotidien supprime par ailleurs :
privacy-sweep-1 = les sessions et les liens de vérification expirés ;
privacy-sweep-2 = les comptes créés mais jamais vérifiés au bout de 24 heures ;
privacy-sweep-3 = les projets dont plus aucun membre ne fait partie, au bout de 24 heures.
privacy-logs-a = Les journaux techniques sont conservés
privacy-logs-duration = 30 jours au maximum
privacy-logs-b = puis supprimés automatiquement. Ils ne sont ni archivés, ni exportés, ni analysés à d'autres fins que la sécurité.
privacy-retention-backups = La base de données est sauvegardée chaque jour. Les sauvegardes sont conservées 14 jours sur notre serveur et 30 jours dans la copie externe chiffrée : une donnée supprimée en disparaît donc au plus tard 30 jours après sa suppression. Elles servent uniquement à restaurer le service et à vérifier que la restauration fonctionne.
privacy-shared-expenses-survive = Les dépenses que vous avez saisies dans un projet partagé ne disparaissent pas avec votre compte : elles font partie des comptes des autres membres, qui restent seuls à pouvoir les déchiffrer.
privacy-cookies-title = 8. Cookies et stockage local
privacy-session-cookie-a = Un unique cookie de session
privacy-session-cookie-b = est déposé après connexion pour vous maintenir authentifié. Aucun cookie de traçage, aucun outil d'analytique tiers.
privacy-lang-cookie-a = Un cookie de langue
privacy-lang-cookie-b = conserve la langue d'interface que vous avez choisie, afin que la page vous soit servie dans cette langue dès le premier affichage. Il contient un code de langue à deux lettres et aucun identifiant.
privacy-local-storage-a = L'application conserve par ailleurs des informations dans le
privacy-local-storage-em = stockage local
privacy-local-storage-b = de votre navigateur (ou de l'application mobile). Elles restent sur votre appareil, en dehors de ce que les fonctions décrites à la section 2 envoient au serveur :
privacy-ls-keys-term = Vos clés de déchiffrement
privacy-ls-keys-def = - celle de votre compte et celle de chaque projet. Sans elles, l'application ne peut rien afficher.
privacy-ls-projects-term = La liste de vos projets
privacy-ls-projects-def = et l'identifiant aléatoire qui vous représente dans un projet rejoint sans compte, que le serveur détient aussi (voir la section 2).
privacy-ls-cache-term = Un cache des projets
privacy-ls-cache-def = (dépenses, participants) pour l'affichage hors connexion et pour éviter de retélécharger ce qui n'a pas changé.
privacy-ls-queue-term = Une file d'attente
privacy-ls-queue-def = des modifications faites hors connexion, envoyées au serveur au retour du réseau.
privacy-ls-prefs-term = Vos préférences d'affichage
privacy-ls-prefs-def = (projets archivés, écran d'accueil déjà vu, langue de l'interface).
privacy-ls-necessary-a = Ces éléments sont strictement nécessaires au fonctionnement du service et ne requièrent donc pas de consentement. Vous pouvez les effacer à tout moment en vidant les données du site -
privacy-ls-warning-label = attention :
privacy-ls-necessary-b = effacer vos clés sans connaître votre mot de passe rend vos projets illisibles.
privacy-rights-title = 9. Vos droits
privacy-rights-intro = Conformément au RGPD, vous disposez des droits suivants :
privacy-right-access = droit d'accès et de rectification ;
privacy-right-erasure = droit à l'effacement : le bouton « Supprimer mon compte » de la page « Réglages » supprime tout immédiatement, sans passer par nous ;
privacy-right-portability = droit à la portabilité : chaque projet s'exporte en CSV ou en JSON déchiffrés depuis son menu ;
privacy-right-object = droit d'opposition et de limitation ;
privacy-right-post-mortem = droit de définir des directives relatives au sort de vos données après votre décès ;
privacy-right-withdraw = droit de retirer à tout moment l'autorisation donnée aux notifications, depuis les réglages de votre téléphone ;
privacy-right-complaint = droit d'introduire une réclamation auprès de la
privacy-rights-contact = Pour exercer ces droits, écrivez à
privacy-security-title = 10. Sécurité
privacy-security-body = Communications TLS 1.2+, mots de passe ni transmis ni stockés (nous ne gardons qu'une empreinte d'une preuve de connexion argon2id), données utilisateur chiffrées de bout en bout, limitation du débit côté reverse-proxy, suppressions définitives sans corbeille.
privacy-changes-title = 12. Modifications
privacy-changes-body = Cette politique peut évoluer. Toute modification substantielle sera notifiée via l'application ou par email.

# Receipt scanning (mobile only)
expense-scan = Scanner un ticket
scan-in-progress = Lecture du ticket…
scan-error-capture = Impossible de prendre cette photo. Réessayez, ou saisissez la dépense à la main.
scan-error-unreadable = Rien de lisible sur ce ticket. Saisissez la dépense à la main.
scan-check-amount = Vérifiez le total - il n’était pas clairement imprimé.
scan-take-photo = Prendre une photo
scan-choose-photo = Choisir une photo
privacy-scan-title = 11. Scan de tickets
privacy-scan-body = Sur l'application mobile, vous pouvez photographier un ticket de caisse, ou choisir une photo déjà présente sur votre téléphone, pour pré-remplir une dépense. Le modèle de reconnaissance est embarqué dans l'application et s'exécute sur votre téléphone : l'image est gardée en mémoire, lue, puis supprimée. Aucune photo et aucun texte reconnu n'est envoyé à nos serveurs ni à qui que ce soit, et aucune requête réseau n'est effectuée pour cela.
privacy-scan-retention = La copie de la photo que votre téléphone crée au moment de la prise de vue, ou de la sélection dans votre galerie, est supprimée dès la fin du scan - qu'il ait réussi, échoué, ou que vous soyez revenu en arrière. Votre photo d'origine, elle, n'est jamais modifiée ni supprimée. Seuls les champs que vous validez sont enregistrés, comme une dépense ordinaire, chiffrée sur votre appareil comme toutes les autres.
expense-converted-from = Payé { $amount } { $from } · 1 { $from } = { $rate } { $to }
project-currency = Devise
project-currency-locked = La devise est fixée à la création du projet.
currency-search = Rechercher une devise

update-required-title = Mise à jour requise
update-required-body = Cette version de Counted est trop ancienne pour communiquer avec le serveur. Mettez-la à jour pour continuer à utiliser l'application.
update-required-button = Mettre à jour

# Cloche de notifications sur la page des projets - pour l’instant les demandes d’ami reçues.
notifications-label = Notifications
notifications-title = Notifications
notifications-empty = Rien de nouveau
notifications-friend-request = Demande d’ami

# Amis et invitations - voir docs/plans/friends.md. « Demande envoyée » s’affiche que l’adresse ait
# un compte ou non ; rien ici ne doit laisser deviner lequel.
friends-title = Amis
friends-anonymous-body = Les amis sont liés à votre compte. Connectez-vous pour ajouter des personnes et les inviter dans vos projets sans partager de lien.
friends-add-title = Ajouter un ami
friends-add-hint = Si cette personne a un compte, elle verra votre demande dans l’app. Sinon, prévenez-la : elle la trouvera en s’inscrivant avec cette adresse.
friends-add-button = Ajouter
friends-add-from-project = Ajouter en ami
friends-request-sent = Demande envoyée
friends-no-account-key = Reconnectez-vous pour gérer vos amis sur cet appareil.
friends-incoming-title = Demandes
friends-accept = Accepter
friends-decline = Refuser
friends-list-title = Mes amis
friends-list-empty = Pas encore d’amis. Ajoutez quelqu’un par e-mail ci-dessus, ou depuis un projet que vous partagez.
friends-remove = Retirer
friends-remove-confirm-title = Retirer cet ami
friends-remove-confirm-message = { $email } ne sera plus dans vos amis, et vous ne serez plus dans les siens. L’un de vous pourra renvoyer une demande plus tard.
friends-no-key = Pas encore prêt
friends-key-changed = Code modifié
friends-key-changed-hint = Le code de sécurité d’un ami a changé. Cela arrive quand il sécurise son compte, ou si sa clé a été substituée. Les invitations avec cet ami sont bloquées jusqu’à ce que vous lisiez ensemble le nouveau code, de vive voix ou par téléphone, puis touchiez « Le code correspond ».
friends-fingerprint = Code de sécurité
friends-fingerprint-hint = Ce code vous permet de vérifier que vos communications sont chiffrées et de vous protéger contre les tentatives d’attaque de l’homme du milieu.
friends-key-mismatch = Le serveur a envoyé pour votre compte une clé publique qui ne correspond pas à votre clé privée. Votre code de sécurité est masqué et les invitations sont désactivées sur cet appareil : quelqu'un modifie peut-être le serveur.
keypair-mismatch = Les clés que le serveur a envoyées pour votre compte ne correspondent pas à celles de cet appareil. Les invitations et les changements de clé sont désactivés ici tant que cela dure : quelqu'un modifie peut-être le serveur.
friends-outgoing-title = Envoyées
friends-outgoing-hint = En attente de réponse. Ils apparaîtront dans vos amis une fois la demande acceptée.
friends-withdraw = Annuler
friends-notify = Prévenir par message
friends-notify-label = Prévenir { $email }
friends-notify-message = Je passe sur Counted pour nos comptes partagés. Inscription gratuite sur https://counted.fr avec l’adresse { $email } : ma demande d’ami y attend.
friends-notify-copied = Message copié
friends-notify-copy-failed = Impossible de copier le message
guests-hint-title =
    { $count ->
        [one] { $count } participant n’a pas encore rejoint le projet
       *[other] { $count } participants n’ont pas encore rejoint le projet
    }
guests-hint-later = Plus tard
guests-hint-send = Les inviter
guests-send-title = Envoyer le projet aux participants
guests-send-friend = Inviter un ami Counted
guests-send-friend-badge = Sans lien
guests-send-friend-hint = S’il a déjà un compte : il reçoit une invitation chiffrée, rien ne passe par votre messagerie.
guests-send-link = Partager le lien du projet
guests-send-link-hint = Pour ceux qui n’ont pas de compte. Ils ouvrent le projet en un geste.
guests-send-link-warning = Toute personne qui a ce lien peut lire et modifier le projet. Votre messagerie le voit aussi.
invite-friends-title = Inviter des amis
invite-friends-hint = La clé du projet est chiffrée pour chaque ami sur cet appareil. Le serveur ne la voit jamais.
invite-friends-empty = Pas encore d’amis à inviter.
invite-friends-button = Inviter
invite-sent = { $count ->
    [one] Invitation envoyée
   *[other] { $count } invitations envoyées
}
invitation-badge = Invitation
invitation-to = Rejoindre « { $name } »
invitation-to-unnamed = Rejoindre un projet
invitation-unreadable = Cette invitation ne peut pas être ouverte sur cet appareil
invitation-from = De { $email }
invitation-accept = Rejoindre
invitation-decline = Refuser

# Participants in the create and edit modals, and the "who are you?" picker - see
# docs/plans/friends.md §11.
participants-you-label = Votre nom dans ce projet
participants-you-badge = Vous
participants-others = Autres participants
participants-removed = Retirés
participant-restore = Rétablir
participants-empty = Ajoutez des participants ci-dessous.
participants-duplicate = « { $name } » est déjà dans la liste.
participants-input-label = Ajouter un ami ou taper un nom
participants-input-placeholder = Un ami ou n’importe quel nom
participants-suggest-friend = Ami · rejoint le projet en tant que « { $name } », reçoit une invitation
participants-suggest-not-ready = Ami · pas encore prêt
participants-suggest-guest = Ajouter « { $text } »
participants-suggest-guest-sub = Invité
participants-friends = Vos amis
participants-all-friends = Tous les amis
participants-login-hint = Connectez-vous pour ajouter des personnes directement depuis votre liste d’amis.
participants-invite-badge = À inviter
participants-guest-badge = Invité
participants-rename = Renommer { $name }
participants-remove = Retirer { $name }
participants-rename-label = Nouveau nom
participants-rename-save = Enregistrer le nom
participants-rename-hint = Le nom que tout le monde voit dans ce projet. L’invitation part toujours vers { $email }.
participants-invited-badge = Invité
participants-invited-sub = { $email } · pas encore acceptée
participants-invited-pending = Invitation pas encore acceptée
participants-unlinked = Non lié à un compte
add-project-create-invite = Créer et inviter { $count }
edit-project-save-invite = Enregistrer et inviter { $count }
edit-project-you-are = Dans ce projet, vous êtes { $name }
edit-project-no-identity = Vous n’avez pas encore indiqué qui vous êtes
edit-project-switch = Changer
edit-project-choose = Choisir
invite-failed = Ces invitations n’ont pas pu être envoyées : { $emails }
invite-waiting = Ces amis doivent ouvrir Counted une fois avant de pouvoir être invités : { $emails }
key-offer-title = Une autre clé pour { $name }
key-offer-body = Cet appareil détient une autre clé pour ce projet, qui ne l’a jamais ouvert ici.
key-offer-choices = Utiliser cette clé : le projet s’ouvre avec la clé de ce lien ou de cette invitation. Annuler : cet appareil garde sa clé actuelle, et rien ne change.
key-offer-use = Utiliser cette clé
invite-again = Inviter à nouveau
friend-picker-title = Ajouter des amis
user-selection-invited-hint = { $email } vous a invité dans « { $project } ».
user-selection-suggested = Suggéré
user-selection-suggested-sub = { $email } vous a ajouté sous ce nom
user-selection-confirm-as = Je suis { $name }
user-selection-missing = Votre nom n’y est pas ? Demandez à un participant de vous ajouter dans les paramètres du projet.

## Dépenses récurrentes

repeat-label = Répéter
repeat-none = Ne se répète pas
repeat-weekly = Chaque semaine
repeat-biweekly = Toutes les 2 semaines
repeat-monthly = Chaque mois
repeat-quarterly = Tous les 3 mois
repeat-yearly = Chaque année
repeat-every-weeks = Toutes les { $count } semaines
repeat-every-months = Tous les { $count } mois
repeat-every-years = Tous les { $count } ans
repeat-custom = Personnalisé…
repeat-every = Tous les
repeat-unit-weeks = semaines
repeat-unit-months = mois
repeat-unit-years = ans
repeat-on-weekday = le { $weekday }
repeat-on-day = le { $day }
repeat-on-day-month = le { $day } { $month }
repeat-month-end = Les mois plus courts, elle tombe le dernier jour.
repeat-ends = Fin
repeat-ends-never = Jamais
repeat-ends-on = À une date
repeat-ends-after = Après
repeat-fewer = Moins
repeat-more = Plus
repeat-last-on = dernière le { $date }
repeat-variable = Le montant change à chaque fois
repeat-variable-hint = Chacune est ajoutée avec le dernier montant et marquée « à confirmer ».
repeat-done = OK
repeat-no-end = Sans fin
repeat-until = Jusqu’au { $date }
repeat-occurrences = Occurrences : { $count }
repeat-offline = Nécessite une connexion. La dépense peut quand même être ajoutée.
repeat-foreign = Répétée pour { $amount } { $currency }, convertis une fois au taux du jour. Vous serez prévenu si le taux bouge de plus de 5 %.
repeat-backfill = Commence dans le passé. Dépenses ajoutées maintenant : { $count }.
add-and-repeat = Ajouter et répéter
weekday-1 = lundi
weekday-2 = mardi
weekday-3 = mercredi
weekday-4 = jeudi
weekday-5 = vendredi
weekday-6 = samedi
weekday-7 = dimanche
recurring-title = Dépenses récurrentes
recurring-strip = Dépenses récurrentes : { $count }
recurring-next = Prochaine : { $name }, { $date }
recurring-to-confirm = À confirmer : { $count }
recurring-per-month = Par mois, environ
recurring-your-share = Votre part
recurring-active = En cours
recurring-paused = En pause
recurring-finished = Terminées
recurring-paid-by = payée par { $name }
recurring-next-on = Prochaine le { $date }
recurring-progress = { $done } sur { $total }
recurring-rate-badge = Taux : { $percent } % d’écart
recurring-empty = Rien ne se répète encore. Choisissez « Répéter » en ajoutant une dépense : loyer, abonnements, factures.
recurring-next-ones = Prochaines
recurring-added-so-far = Ajoutées jusqu’ici
recurring-set-up-by = Créée par
recurring-pause = Mettre en pause
recurring-resume = Reprendre
recurring-stop = Arrêter la répétition
recurring-stop-title = Arrêter « { $name } » ?
recurring-stop-message = Elle ne se répétera plus. Les dépenses déjà ajoutées restent.
recurring-resume-title = Reprendre « { $name } » ?
recurring-resume-message = Prochaine le { $date }. Les dates manquées pendant la pause ne sont pas ajoutées.
recurring-edit-title = Modifier la dépense récurrente
recurring-edit-banner = Les changements s’appliquent à partir du { $date }. Les dépenses déjà ajoutées restent telles quelles.
recurring-next-on-label = Prochaine le
recurring-next-too-early = La prochaine date doit être après la dernière dépense déjà ajoutée.
recurring-use-stop = Pour y mettre fin, utilisez « Arrêter la répétition » sur la dépense récurrente.
recurring-drift = Le taux { $currency } a bougé de { $percent } % depuis la création. Chacune est encore ajoutée pour { $amount } { $project_currency } (1 { $currency } = { $rate }). Au taux du jour, ce serait { $today_amount } { $project_currency }.
recurring-use-rate = Utiliser le taux du jour
recurring-keep = Garder { $amount } { $currency }
recurring-added = Dépenses récurrentes ajoutées : { $count }
recurring-blocks-removal = { $name } ne peut pas encore être retiré : fait partie de { $rules }. Retirez { $name } de ces dépenses ou arrêtez-les, puis enregistrez à nouveau.
history-recurring-added = Ajoutée automatiquement : { $name } ({ $date })
history-recurring-created = Dépense récurrente créée : { $name }
history-recurring-edited = Dépense récurrente modifiée : { $name }
history-recurring-paused = Dépense récurrente mise en pause : { $name }
history-recurring-resumed = Dépense récurrente reprise : { $name }
history-recurring-stopped = Dépense récurrente arrêtée : { $name }
occurrence-recurring = Dépense récurrente
occurrence-auto = Ajoutée automatiquement par une dépense récurrente.
occurrence-auto-next = Ajoutée automatiquement par une dépense récurrente. Prochaine le { $date }.
occurrence-auto-stopped = Ajoutée automatiquement par une dépense récurrente arrêtée depuis.
occurrence-manage = Gérer
estimate-badge = À confirmer
expense-pending = En attente
estimate-title = Montant à confirmer.
estimate-body = Ajoutée avec le montant précédent. Saisissez le vrai montant quand il arrive.
estimate-confirm = Confirmer le montant
apply-to = Appliquer à
apply-this-only = Cette dépense seulement
apply-and-next = Celle-ci et les suivantes
apply-and-next-hint = La dépense récurrente change à partir du { $date }
apply-rule-failed = La dépense est enregistrée, mais la dépense récurrente n’a pas été modifiée.
occurrence-delete-message = « { $name } » du { $date } sera définitivement supprimée. La répétition continue et cette date ne reviendra pas.
occurrence-delete-one = Supprimer celle-ci
occurrence-delete-stop = Supprimer et arrêter la répétition
error-recurring-clock = L’horloge de cet appareil est en avance. Vérifiez sa date et son heure.
error-recurring-not-found = Cette dépense récurrente n’existe plus.
error-recurring-stale = Quelqu’un a modifié cette dépense récurrente entre-temps. Elle a été rechargée : vérifiez-la et enregistrez à nouveau.
error-too-many-recurring = Ce projet a déjà 50 dépenses récurrentes. Arrêtez-en une dont vous n’avez plus besoin pour en ajouter une autre.
error-user-in-recurring = Ce participant fait partie d’une dépense récurrente. Retirez-le de celle-ci ou arrêtez-la d’abord.

home-owed-to-you = On vous doit
home-you-owe = Vous devez
home-details-show = Voir le détail
home-details-hide = Masquer le détail
home-all-settled = Tout est réglé
home-pay = Payer
home-remind = Relancer
home-remind-message = Petit rappel : il reste des remboursements à régler sur { $project }.
home-row-owed = on vous doit
home-row-owe = vous devez
home-row-settled = À l’équilibre
home-row-quits = Vous êtes à l’équilibre
home-row-pending =
    { $count ->
        [one] { $count } remboursement en attente
       *[other] { $count } remboursements en attente
    }
home-totals-partial = Hors projets sans taux de change
error-account-key-changed = La clé de votre compte a changé sur un autre appareil. Reconnectez-vous pour continuer.
settings-security = Sécurité
settings-change-password = Changer le mot de passe
settings-change-password-hint = Vos projets et vos données restent tels quels. Vos autres appareils sont déconnectés.
change-password-title = Changer votre mot de passe
change-password-current = Mot de passe actuel
change-password-new = Nouveau mot de passe
change-password-confirm = Confirmez le nouveau mot de passe
change-password-mismatch = Les deux nouveaux mots de passe ne correspondent pas.
change-password-submit = Changer le mot de passe
change-password-submitting = Modification…
change-password-done = Mot de passe modifié. Vos autres appareils ont été déconnectés.
error-rotation-incomplete = Votre compte a changé pendant l’opération. Réessayez.
error-key-lifecycle-unavailable = Pas encore disponible : tout le monde doit d’abord avoir la dernière version de l’application.
settings-compromised = Quelqu’un connaît peut-être mon mot de passe
settings-compromised-hint = Nouveau mot de passe, nouvelles clés, et tous les autres appareils déconnectés.
settings-compromised-open = Sécuriser mon compte
compromised-title = Sécuriser votre compte
compromised-body = Votre mot de passe et les clés de votre compte sont remplacés, et tous vos autres appareils sont déconnectés. Vos amis verront votre code de sécurité changer : comparez-le à nouveau avec eux.
compromised-keeps = La personne qui avait votre mot de passe garde ce qu’elle a déjà ouvert : les projets qu’elle pouvait lire jusqu’ici.
compromised-submit = Remplacer mot de passe et clés
compromised-done = Votre compte est sécurisé. Demandez à vos amis de comparer votre nouveau code de sécurité.
friends-new-fingerprint = Nouveau code :
friends-code-matches = Le code correspond
friends-code-trusted = Code confirmé.
settings-recovery-key = Clé de récupération
settings-recovery-key-none = Sans elle, un mot de passe oublié vous coûte vos préférences, vos coordonnées de paiement et les codes de vos amis, et prend 3 jours.
settings-recovery-key-set = Créée. Si vous oubliez votre mot de passe, elle vous rend votre compte avec tout son contenu.
settings-recovery-key-pending = Une nouvelle clé la remplace le { $when }. Ce n’était pas vous ? Annulez-la avec votre clé actuelle.
recovery-key-create = Créer
recovery-key-replace = Remplacer
recovery-key-cancel-replacement = Annuler la nouvelle clé
recovery-key-title = Votre clé de récupération
recovery-key-body = Notez-la ou enregistrez-la dans un endroit que vous seul pouvez atteindre. Counted ne peut ni la réafficher ni la retrouver pour vous.
recovery-key-copy = Copier
recovery-key-copied = Copiée
recovery-key-save = Enregistrer en fichier
recovery-key-print = Imprimer
recovery-key-next = Continuer
recovery-key-confirm-body = Pour vérifier votre copie, saisissez le groupe { $n } de la clé.
recovery-key-confirm-wrong = Ce groupe ne correspond pas. Regardez à nouveau votre copie.
recovery-key-password-body = Saisissez votre mot de passe pour enregistrer la clé sur votre compte.
recovery-key-submit = Enregistrer la clé
recovery-key-done = Clé de récupération enregistrée.
recovery-key-replacement-done = Nouvelle clé enregistrée. Elle remplace l’actuelle dans 72 heures ; d’ici là, l’actuelle fonctionne encore.
recovery-key-field = Clé de récupération
recovery-key-file-intro = Clé de récupération Counted
recovery-key-cancel-done = La nouvelle clé est annulée.
compromised-new-key = Enregistrez votre nouvelle clé de récupération maintenant : elle remplace l’ancienne dans 72 heures, et d’ici là l’ancienne fonctionne encore.
pending-reset-title = Une réinitialisation de votre compte a été demandée
pending-reset-body = Elle prend effet le { $when } : vos préférences, coordonnées de paiement et codes d’amis seront alors perdus, et vos appareils devront se reconnecter. Ce n’était pas vous ? Annulez-la avec votre mot de passe actuel.
pending-reset-cancel = Annuler la réinitialisation
pending-reset-cancelled = La réinitialisation est annulée.
login-forgot = Mot de passe oublié ?
forgot-title = Mot de passe oublié
forgot-body = Saisissez votre adresse e-mail. Si elle a un compte, nous envoyons un lien pour choisir un nouveau mot de passe.
forgot-submit = Envoyer le lien
forgot-sent = Si cette adresse a un compte, un lien est en route. Il expire dans 24 heures.
reset-title = Récupérer votre compte
reset-with-key-body = Saisissez votre clé de récupération et un nouveau mot de passe : tout revient comme avant.
reset-submit-key = Récupérer mon compte
reset-no-key = Je n’ai pas ma clé de récupération
reset-without-key-title = Réinitialiser sans la clé de récupération
reset-without-key-body = Votre compte repart avec un nouveau mot de passe dans 72 heures. Vos projets reviennent depuis vos appareils et les liens de partage ; vos préférences, coordonnées de paiement et codes d’amis, non. D’ici là, votre mot de passe actuel fonctionne encore et peut l’annuler.
reset-name = Votre nom
reset-submit-request = Réinitialiser mon compte
reset-requested = C’est fait. Votre nouveau mot de passe fonctionnera à partir du { $when }. D’ici là, votre mot de passe actuel fonctionne encore et peut l’annuler.
reset-pending-already = Une réinitialisation est déjà en attente jusqu’au { $when }. Une nouvelle demande relance les 72 heures.
recovered-done = Votre compte est récupéré. Vos autres appareils ont été déconnectés.
login-reset-undoable = Ce mot de passe a été remplacé par une réinitialisation ces 30 derniers jours. Si vous ne l’avez pas demandée, annulez-la : votre compte revient comme avant.
login-undo-reset = Annuler la réinitialisation
reset-undone = La réinitialisation est annulée. Votre compte est revenu comme avant.
error-reset-pending = Ce nouveau mot de passe fonctionnera quand la réinitialisation prendra effet, 72 heures après la demande.
error-account-disabled = Ce compte a été supprimé peu après une réinitialisation et attend d’être effacé. L’ancien mot de passe peut encore annuler la réinitialisation.
error-recovery-key-invalid = Cette clé de récupération n’ouvre pas ce compte.
error-project-key-required = Ce projet a besoin de sa clé. Rouvrez-le depuis son lien de partage.
names-or = { $first } ou { $last }
participants-admin-badge = Admin
edit-project-admins = Administrateurs : { $count }
edit-project-succession = Si le dernier administrateur quitte le projet, le membre avec un compte présent depuis le plus longtemps le devient.
member-sheet-account = Membre avec un compte
member-actions = Actions pour { $name }
member-make-admin = Donner le rôle d’administrateur
member-make-admin-hint = Pourra changer la clé du projet et retirer quelqu’un.
member-made-admin = { $name } a maintenant le rôle d’administrateur.
group-security = Sécurité du projet
group-security-admin = Sécurité du projet · vous êtes administrateur
group-rotate = Changer la clé du projet
group-rotate-hint = Si le lien du projet a été compromis : les anciens détenteurs du lien n’auront plus accès au projet.
group-remove = Retirer quelqu’un du projet
group-remove-hint = Retire son accès et change la clé en même temps.
group-admins-only = Réservé aux administrateurs.
group-admins-only-account = Réservé aux administrateurs, qui ont un compte.
group-ask-admin = Demandez à { $names }, qui administre le projet.
group-ask-admins = Demandez à { $names }, les administrateurs du projet.
group-ask-an-admin = Demandez à un administrateur du projet.
group-with-account = Avec un compte
group-free = Gratuit
group-sign-in-hint = Un administrateur du projet, { $names }, pourra ensuite vous donner ce rôle.
group-sign-in-hint-any = Un administrateur du projet pourra ensuite vous donner ce rôle.
group-no-admin = Ce projet n’a pas d’administrateur : sa clé ne peut pas changer.
group-hide = Masquer quelqu’un
group-hide-hint = Le masque des listes, sans lui retirer l’accès : le lien et la clé restent valables.
group-recreate-hint = Ce projet n’a pas d’administrateur. Pour retirer quelqu’un, créez un nouveau projet.
group-recreate = Créer un nouveau projet
group-no-admin-yet = Ce projet n’a pas encore d’administrateur.
group-creator-hint = Vous seul pouvez devenir administrateur de ce projet, en créant un compte.
group-creator-elsewhere-hint = Seule la personne qui a créé le projet peut en devenir administrateur, en créant un compte. Sinon, créez un nouveau projet pour retirer quelqu’un.
member-revoke-admin = Retirer le rôle d’administrateur
member-revoke-admin-hint = Ne pourra plus changer la clé du projet ni retirer quelqu’un.
member-revoked-admin = { $name } n’a plus le rôle d’administrateur.
member-step-down = Ne plus être administrateur
member-step-down-hint = Vous ne pourrez plus changer la clé du projet ni retirer quelqu’un.
member-stepped-down = Vous n’avez plus le rôle d’administrateur.
member-remove = Retirer du projet
member-remove-hint = Change la clé du projet : { $name } ne pourra plus rien lire de nouveau.
member-hide = Masquer des listes
error-user-not-admin = Ce membre n’a pas le rôle d’administrateur.
error-creator-proof-invalid = Cet appareil n’a pas créé ce projet.
error-project-has-admin = Ce projet a déjà un administrateur.
error-not-project-admin = Seuls les administrateurs du projet peuvent faire cela.
error-user-not-claimed = Cette personne n’a pas de compte, et seul un compte peut être administrateur.
error-project-key-rotated = La clé du projet a changé. Demandez le nouveau lien à un membre.
error-project-rotating = La clé du projet est en cours de changement. Réessayez dans un instant.
error-rotation-expired = Ce changement de clé a pris trop de temps et a été annulé. Recommencez.
error-rotation-rows-changed = Le projet a changé pendant le changement de clé. Recommencez.
project-rotated-title = Vous n’avez plus accès à ce projet
project-rotated-body = La clé de ce projet a été changée par l’administrateur. Demandez-lui le nouveau lien.
project-rotated-open-link = Ouvrir un lien
rotation-remove-title = Retirer { $name } du projet ?
rotation-rotate-title = Changer la clé du projet ?
rotation-remove-body = Retirer un membre implique de générer une nouvelle clé. Le nouveau lien devra être partagé avec les utilisateurs n’ayant pas de compte.
rotation-rotate-body = Les anciens détenteurs du lien n’auront plus accès au projet. Le nouveau lien devra être partagé avec les utilisateurs n’ayant pas de compte.
rotation-boxed =
    { $count ->
        [0] Aucun autre membre ne recevra la clé automatiquement
        [one] { $count } membre recevra la clé automatiquement
       *[other] { $count } membres recevront la clé automatiquement
    }
rotation-by-link-title = Utilisateurs à qui vous devrez envoyer le nouveau lien
rotation-by-link-hint = Ces utilisateurs n’ont pas de compte. Vous leur enverrez le lien juste après.
rotation-names-and = { $first } et { $last }
rotation-names-and-others =
    { $names } et { $count ->
        [one] { $count } autre
       *[other] { $count } autres
    }
rotation-confirm-remove = Retirer et changer la clé
rotation-confirm-rotate = Changer la clé
rotation-done-title = Nouvelle clé en place
rotation-done-removed = { $name } ne fait plus partie du projet.
rotation-done-rotated = La clé du projet a changé.
rotation-done-boxed =
    { $count ->
        [0] Aucun autre membre n’a reçu la clé automatiquement.
        [one] { $count } membre a reçu la clé automatiquement.
       *[other] { $count } membres ont reçu la clé automatiquement.
    }
rotation-new-link =
    { $count ->
        [0] Nouveau lien du projet
        [one] Nouveau lien à envoyer · { $count } utilisateur sans compte
       *[other] Nouveau lien à envoyer · { $count } utilisateurs sans compte
    }
rotation-link-renewed = Le lien a été renouvelé avec succès.
rotation-done = Terminé
history-key-changed = Clé du projet changée
history-member-removed = { $name } ne fait plus partie du projet
project-rotated-row-hint = Sa clé a changé. Demandez le nouveau lien.
