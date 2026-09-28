# Français - la copie d'origine de l'application.
#
# Doit définir exactement les mêmes clés que en.ftl (garde : `french_and_english_define_the_same_messages`).

### Common

loading = Chargement…
cancel = Annuler
confirm = Confirmer
retry = Réessayer
delete = Supprimer
back = Retour
language = Langue

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
error-generic = Une erreur est survenue. Réessayez.

error-invalid-email = Cette adresse e-mail n'est pas valide.
error-invalid-password = Ce mot de passe n'est pas valide.
error-password-too-short = Le mot de passe doit contenir au moins 8 caractères.
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
login-resend = Renvoyer le lien de vérification
login-resending = Envoi…
login-resend-sent = E-mail envoyé ! Vérifiez votre boîte mail.

register-submit = Créer un compte
register-submitting = Création…
register-have-account = Déjà un compte ?
register-password-placeholder = 8 caractères minimum
register-password-warning = Notez votre mot de passe, il ne sera pas possible de récupérer votre compte si vous l'oubliez.
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
account-delete-warning = Suppression immédiate et définitive, sans corbeille. Les dépenses que vous avez saisies dans un projet partagé restent visibles par les autres membres - elles font partie de leurs comptes.
account-delete-confirm-title = Supprimer le compte
account-delete-confirm-message = Votre compte, vos sessions et la liste de vos projets seront définitivement supprimés. Sans votre mot de passe, les données chiffrées d'un projet partagé deviennent illisibles pour vous - cette action est irréversible.

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
payment-methods-stale = Vos coordonnées de paiement ont été modifiées sur un autre appareil. Elles ont été rechargées — veuillez réessayer.
payment-methods-key-missing = Reconnectez-vous pour gérer vos coordonnées de paiement.
settings-payment-methods-share-warning = Un moyen partagé est visible par tous les membres des projets où vous avez choisi votre nom - toute personne qui détient un de ces liens.
payment-method-share = Partager avec mes projets
payment-method-share-hint = Affiché à côté de votre nom quand quelqu'un vous doit de l'argent.
payment-method-copy = Copier { $name }
payment-method-copied = Copié.
payment-method-copy-failed = Impossible de copier - sélectionnez le texte et copiez-le à la main.

verify-email-checking = Vérification de votre e-mail…
verify-email-welcome = E-mail vérifié - bienvenue sur Counted !
verify-email-back-to-login = Retourner à la connexion

### Project status

project-close = Clôturer
project-archive = Archiver
project-reopen = Réouvrir
project-unarchive = Désarchiver

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
edit = Éditer
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
projects-empty-hint = Créez un projet en cliquant sur le bouton ci-dessous
projects-offline-banner = Données hors ligne - reconnectez-vous pour actualiser.
projects-no-local-data = Aucune donnée locale
projects-no-local-data-hint = Connectez-vous pour charger vos projets pour la première fois.
projects-add = Ajouter un projet
projects-create = Créer un projet
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
add-project-remove-participant = Retirer le participant
add-project-me-badge = Moi
add-project-thats-me = C'est moi !
add-project-offline = Impossible de créer un projet hors ligne. Reconnectez-vous et réessayez.
add-project-name-required = Le nom du projet est requis.
add-project-need-two-participants = Ajoutez au moins 2 participants.
add-project-pick-yourself = Indiquez quel participant vous êtes.

join-link-label = Lien de partage
join-link-hint = Le lien contient la clé de déchiffrement - copiez-le en entier.
join-invalid-link = Lien invalide. Collez le lien de partage complet, avec la partie après le #.
join-wrong-project = Ce lien correspond à un autre projet.

import-tricount-link-label = Lien ou clé Tricount
import-tricount-key-required = Entrez un lien ou une clé Tricount.
import-tricount-encryption-failed = Erreur de chiffrement.

### Expenses

save = Enregistrer
saving = Enregistrement…
adding = Ajout…
link-copied = Lien copié
missing-encryption-key = Clé de chiffrement manquante.
missing-encryption-key-title = Clé de chiffrement manquante
missing-encryption-key-hint = Le lien que vous avez utilisé ne contient pas la clé nécessaire pour déchiffrer ce projet. Utilisez le lien complet partagé par le créateur du projet.
project-locked-hint = Cet appareil n’a pas la clé de ce projet. Ouvrez son lien de partage pour le déverrouiller.
project-unlock = Déverrouiller
project-no-local-data-hint = Connectez-vous pour charger les données de ce projet pour la première fois.
project-gone-title = Ce projet n'existe plus
project-gone-hint = Il a été supprimé lorsque son dernier membre l'a quitté. Le lien de partage ne fonctionne plus, même si vous le rouvrez.

expense-add = Ajouter une dépense
transfer-add = Ajouter un transfert
expense-edit-title = Modifier la dépense
expense-category = Catégorie
expense-category-auto = Auto · { $emoji }
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
reimbursements-empty-title = Les comptes sont bons !
reimbursements-empty-hint = Des suggestions de remboursement seront proposées ici si les comptes ne sont pas équilibrés
reimbursement-owes = { $debtor } doit à { $creditor }
reimbursement-record = Rembourser
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
edit-project-deferred-me = la sélection de « C'est moi »
edit-project-offline-deferred = Hors ligne : { $items } sera appliqué à la reconnexion.

export-saved = Fichier enregistré :
    { $path }
export-failed = Erreur d'export : { $reason }

# Les entrées d'historique sont chiffrées et stockées : elles gardent la langue de leur auteur.
history-expense-added = Dépense ajoutée : { $name }
history-expense-edited = Dépense modifiée : { $name }
history-expense-deleted = Dépense supprimée : { $name }
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
expense-inconsistent-detail = Montants incohérents : { $paid } payé, { $owed } dû, pour une dépense de { $total }. Modifiez la dépense pour la corriger.
missing-access-key = Clé d'accès manquante. Visitez ce projet via son lien de partage.
filter-all = Tout
filter-my-payments = Mes paiements
filter-my-debts = Mes dettes
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
charts-my-share-note = Ces montants correspondent à ta part de chaque dépense.
charts-my-share-skipped =
    { $count ->
        [one] 1 projet n’est pas compté — aucun participant choisi, ou ses données n’ont pas pu être chargées.
       *[other] { $count } projets ne sont pas comptés — aucun participant choisi, ou leurs données n’ont pas pu être chargées.
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
charts-paid-more = { $amount } de plus que ta part
charts-paid-less = { $amount } de moins que ta part
charts-paid-even = Pile ta part
charts-part-title = Ta part dans chaque catégorie
charts-part-desc = En gris ce que le groupe a dépensé, en couleur ce que tu as consommé.
charts-breakdown-title = Répartition par catégorie
charts-breakdown-desc = Touche une part ou une ligne pour voir ses dépenses.
charts-of-total = { $amount } sur { $total }
charts-show-all = Tout afficher ({ $count })
charts-show-less = Afficher moins
charts-spend-title = Dépenses dans le temps
charts-spend-desc = Les courtes périodes vont par jour, les longues par semaine ou par mois.
charts-group-by = Grouper par
bucket-day = Jour
bucket-week = Semaine
bucket-month = Mois
charts-avg = moy.
charts-cat-title-day = { $category }, jour par jour
charts-cat-title-week = { $category }, semaine par semaine
charts-cat-title-month = { $category }, mois par mois
charts-cat-desc = Choisis une catégorie pour la suivre dans le temps.
charts-running-title = Cumul
charts-running-desc = Depuis le { $date }.
charts-avg-per-day = { $amount } / jour en moyenne
charts-avg-per-week = { $amount } / semaine en moyenne
charts-avg-per-month = { $amount } / mois en moyenne
charts-people-title = Qui a porté le groupe
charts-people-desc = Ce que chacun a payé, à côté de ce qu’il a consommé.
charts-paid = Payé
charts-fair-share = Juste part
charts-you = (toi)
charts-net-more = a payé plus
charts-net-less = a payé moins
charts-balance-title = Ton solde dans le temps
charts-balance-desc = Au-dessus de la ligne, le groupe te doit. En dessous, tu dois au groupe.
charts-owed = On te doit
charts-owe = Tu dois
charts-projects-title = Ta part, par projet
charts-projects-desc = Les totaux restent par devise, jamais additionnés entre elles.
history-empty = Aucun événement
history-by = Par { $name }
not-found-hint = Cette page n'existe pas ou a été déplacée.
payers-title-paid-by = Payé par
payers-title-sender = Émetteur
payers-title-contributors = Contributeurs
debtors-title-debtors = Débiteurs
debtors-title-recipients = Destinataires
debtors-title-beneficiaries = Bénéficiaires

### Welcome

welcome-title = Vos comptes ne regardent que vous.
welcome-subtitle = Partagez vos dépenses entre amis.
welcome-e2ee-title = Données entièrement chiffrées
welcome-e2ee-body = Noms, montants, projets : tout est chiffré sur votre appareil. Vous seul avez la clé. Personne ne peut lire vos comptes. Pas même nous.
welcome-e2ee-note = Indéchiffrable, même pour nous (Zéro accès serveur)
welcome-eu-title = 100 % européen
welcome-eu-body = Serveurs en Allemagne, emails envoyés depuis la France. Vos données ne quittent jamais l'Union européenne.
welcome-noads-title = Zéro pub. Zéro traqueur.
welcome-noads-body = Nous ne collectons rien, nous ne vendons pas vos données. Ce n'est pas notre modèle.
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
help-forgot-password-warning = Vos données seront définitivement perdues.
help-forgot-password-a = Comme la clé de chiffrement est dérivée de votre mot de passe, aucune réinitialisation n'est possible : personne (pas même nous) ne peut déchiffrer vos projets sans ce mot de passe. Conservez-le précieusement, idéalement dans un gestionnaire de mots de passe.
help-archive-delete-q = Comment archiver ou supprimer un projet ?
help-archive-delete-a = Depuis l'écran du projet, ouvrez le menu et choisissez
help-archive-delete-b = pour le masquer tout en le conservant. Un projet n'est supprimé définitivement que lorsque son dernier membre le quitte.
help-delete-account-q = Comment supprimer mon compte ?
help-delete-account-a = Ouvrez les Réglages et utilisez « Supprimer mon compte ». La suppression est immédiate et irréversible.
help-contact = Une autre question ? Écrivez-nous à

### Legal

legal-updated = Dernière mise à jour : 27 septembre 2026

legal-publisher-title = 1. Éditeur du site
legal-publisher-body = Counted (« le Service ») est édité à titre non professionnel par une personne physique, Jonathan Bosi.
legal-contact-label = Contact :
legal-publisher-address-note = Conformément à l'article 6-III-2 de la loi n° 2004-575 du 21 juin 2004 pour la confiance dans l'économie numérique, l'éditeur, personne physique éditant à titre non professionnel, ne publie pas son adresse postale. Ces éléments d'identification sont détenus par l'hébergeur, qui peut les communiquer à l'autorité judiciaire.
legal-director-title = 2. Directeur de la publication
legal-host-title = 3. Hébergeur
legal-host-address = Industriestr. 25, 91710 Gunzenhausen, Allemagne
legal-host-phone = Téléphone : +49 (0)9831 505-0
legal-host-email-note = Les emails transactionnels sont acheminés par Scaleway SAS (France). Les serveurs et la base de données sont situés dans l'Union européenne.
legal-ip-title = 4. Propriété intellectuelle
legal-ip-body = La structure du site, ses textes et ses éléments graphiques sont la propriété de l'éditeur, sauf mention contraire. Les données que vous saisissez restent les vôtres : elles sont chiffrées sur votre appareil et l'éditeur ne peut ni les lire, ni les exploiter.
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
terms-account-warning-a = votre clé de chiffrement est dérivée de votre mot de passe et ne quitte jamais votre appareil. En cas d'oubli,
terms-account-warning-em = aucune récupération n'est possible
terms-account-warning-b = - ni par vous, ni par l'éditeur. Il n'existe volontairement aucune procédure de réinitialisation, car elle supposerait un accès à vos données.
terms-acceptable-use-title = 5. Usage acceptable
terms-acceptable-use-intro = Vous vous engagez à ne pas :
terms-acceptable-use-1 = utiliser le Service à des fins illicites ou frauduleuses ;
terms-acceptable-use-2 = tenter d'accéder à des projets qui ne vous ont pas été partagés ;
terms-acceptable-use-3 = porter atteinte au fonctionnement du Service, notamment par des requêtes automatisées massives ;
terms-acceptable-use-4 = contourner les limitations techniques mises en place (quotas, limitation de débit) ;
terms-acceptable-use-5 = utiliser le Service pour envoyer des messages non sollicités à des tiers, y compris via les invitations par email.
terms-your-content-title = 6. Vos contenus
terms-your-content-body = Vous restez propriétaire des données que vous saisissez. Elles sont chiffrées sur votre appareil avant d'être transmises : l'éditeur en héberge une version qu'il ne peut pas lire, et n'exerce donc aucune modération de leur contenu. Vous êtes seul responsable de ce que vous y inscrivez et de votre droit à y faire figurer des informations concernant d'autres personnes.
terms-availability-title = 7. Disponibilité
terms-availability-body = Le Service est fourni « en l'état », sans garantie de disponibilité ni d'absence d'erreur. Il peut être interrompu, modifié ou arrêté à tout moment, notamment pour maintenance. Les calculs de répartition et de remboursement sont fournis à titre indicatif : ils ne constituent ni un service de paiement, ni un conseil financier ou comptable, et aucun mouvement d'argent ne transite par le Service.
terms-deletion-title = 8. Suppression
terms-deletion-account = Vous pouvez supprimer votre compte à tout moment depuis la page « Mon compte ». La suppression est immédiate et définitive.
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
privacy-collected-prefs-def = (langue de l'interface), pour les retrouver sur vos autres appareils. Chiffrées sur votre appareil avec la même clé que le reste : le serveur ne peut pas savoir quelle langue vous utilisez.
privacy-collected-keys-term = Vos clés de projet, scellées
privacy-collected-keys-def = pour qu'une connexion depuis un nouvel appareil vous rende des projets réellement lisibles, et non une liste que vous ne pouvez pas ouvrir. Chacune est chiffrée sur votre appareil avec la clé dérivée de votre mot de passe ; le serveur les conserve sans pouvoir les desceller. La contrepartie est réelle et nous préférons l'énoncer : quelqu'un qui détiendrait à la fois notre base et votre mot de passe atteindrait vos projets. C'est pourquoi le mot de passe n'est jamais transmis ni stocké : ce que nous recevons à la connexion est une preuve distincte dont il ne peut être retrouvé, et nous n'en gardons qu'une empreinte.
privacy-collected-invite-term = Empreinte de l'email d'un participant invité
privacy-collected-invite-def = (SHA-256), lorsque vous invitez quelqu'un par email. Elle sert uniquement à rattacher l'invitation à son compte s'il en crée un, et disparaît avec le participant.
privacy-collected-friends-term = Vos amis
privacy-collected-friends-def = si vous utilisez la liste d'amis : avec quels comptes vous êtes amis, une empreinte (SHA-256) d'une adresse à qui vous avez envoyé une demande avant qu'elle n'ait de compte, et quel ami vous avez invité dans quel projet. Le nom que vous donnez à une demande et la clé de projet qu'une invitation transporte sont chiffrés sur votre appareil - la clé avec la clé publique de votre ami, que nous stockons en clair car elle est publique par nature - et le serveur ne peut lire ni l'un ni l'autre.
privacy-collected-logs-term = Journaux techniques
privacy-collected-logs-def = (adresse IP, user-agent, horodatage) conservés pour la sécurité et la prévention des abus.
privacy-purposes-title = 3. Finalités
privacy-purpose-1 = Authentifier vos sessions et protéger votre compte.
privacy-purpose-2 = Synchroniser vos projets entre vos appareils.
privacy-purpose-3 = Vous envoyer un email de vérification lors de l'inscription.
privacy-purpose-4 = Détecter et prévenir les abus (bruteforce, scraping).
privacy-legal-basis-title = 4. Base légale
privacy-legal-basis-body = Le traitement repose sur l'exécution du contrat qui nous lie (création et fourniture du service) et sur notre intérêt légitime à sécuriser la plateforme.
privacy-e2ee-title = 5. Chiffrement de bout en bout
privacy-e2ee-a = Counted applique un modèle
privacy-e2ee-b = : votre clé de chiffrement est dérivée de votre mot de passe et ne quitte jamais votre appareil. Le serveur stocke uniquement des blobs chiffrés qu'il est incapable de lire.
privacy-e2ee-consequence-label = Conséquence importante :
privacy-e2ee-consequence-a = si vous oubliez votre mot de passe,
privacy-e2ee-consequence-em = personne
privacy-e2ee-consequence-b = - pas même nous - ne peut récupérer vos données. Aucune procédure de réinitialisation n'est possible.
privacy-processors-title = 6. Sous-traitants et hébergement
privacy-processor-hetzner = (Allemagne, UE) - hébergement des serveurs et de la base de données.
privacy-processor-scaleway = (France, UE) - envoi des emails transactionnels (vérification d'email).
privacy-processor-tricount = - contacté uniquement si vous déclenchez vous-même un import depuis Tricount, afin de récupérer le projet que vous souhaitez importer.
privacy-processor-grafana = (région UE) - supervision technique du serveur. Seules des métriques d'infrastructure (processeur, mémoire, disque, état des services) et des totaux d'usage quotidiens (nombre de comptes, de projets, d'adhésions et d'inscriptions, et le nombre d'inscriptions et de connexions effectuées dans chaque langue d'interface) y sont envoyés : ni contenu, ni identifiant, ni adresse IP de visiteur.
privacy-no-transfer-outside-eu = Aucune donnée personnelle n'est transférée hors de l'Union européenne.
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
privacy-shared-expenses-survive = Les dépenses que vous avez saisies dans un projet partagé ne disparaissent pas avec votre compte : elles font partie des comptes des autres membres, qui restent seuls à pouvoir les déchiffrer.
privacy-cookies-title = 8. Cookies et stockage local
privacy-session-cookie-a = Un unique cookie de session
privacy-session-cookie-b = est déposé après connexion pour vous maintenir authentifié. Aucun cookie de traçage, aucun outil d'analytique tiers.
privacy-lang-cookie-a = Un cookie de langue
privacy-lang-cookie-b = conserve la langue d'interface que vous avez choisie, afin que la page vous soit servie dans cette langue dès le premier affichage. Il contient un code de langue à deux lettres et aucun identifiant.
privacy-local-storage-a = L'application conserve par ailleurs des informations dans le
privacy-local-storage-em = stockage local
privacy-local-storage-b = de votre navigateur (ou de l'application mobile). Elles ne sont jamais transmises au serveur :
privacy-ls-keys-term = Vos clés de déchiffrement
privacy-ls-keys-def = - celle de votre compte et celle de chaque projet. Sans elles, l'application ne peut rien afficher.
privacy-ls-projects-term = La liste de vos projets
privacy-ls-projects-def = et l'identifiant aléatoire qui vous représente dans un projet rejoint sans compte.
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
privacy-right-erasure = droit à l'effacement : le bouton « Supprimer mon compte » de la page « Mon compte » supprime tout immédiatement, sans passer par nous ;
privacy-right-portability = droit à la portabilité : chaque projet s'exporte en CSV ou en JSON déchiffrés depuis son menu ;
privacy-right-object = droit d'opposition et de limitation ;
privacy-right-withdraw = droit de retirer votre consentement à tout moment ;
privacy-right-complaint = droit d'introduire une réclamation auprès de la
privacy-rights-contact = Pour exercer ces droits, écrivez à
privacy-security-title = 10. Sécurité
privacy-security-body = Communications TLS 1.2+, mots de passe stockés sous forme d'empreintes argon2id, données utilisateur chiffrées de bout en bout, limitation du débit côté reverse-proxy, suppressions définitives sans corbeille.
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
project-currency-hint = Tous les montants sont affichés dans cette devise. Elle ne pourra pas être modifiée.
project-currency-locked = La devise est fixée à la création du projet.

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
friends-add-hint = Une notification sera envoyée à l’utilisateur au sein de l’application.
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
friends-fingerprint = Code de sécurité
friends-fingerprint-hint = Ce code vous permet de vérifier que vos communications sont chiffrées et de vous protéger contre les tentatives d’attaque de l’homme du milieu.
friends-outgoing-title = Envoyées
friends-outgoing-hint = En attente de réponse. Ils apparaîtront dans vos amis une fois la demande acceptée.
friends-withdraw = Annuler
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
