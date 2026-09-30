# English - the fallback locale.
#
# This file must be COMPLETE. Every other locale falls back to it per message, so a key missing
# here is the only kind a user ever sees as a raw key. Keys are kebab-case and page-prefixed.

### Common

loading = Loading…
cancel = Cancel
confirm = Confirm
retry = Try again
delete = Delete
back = Back
language = Language

### Navigation

nav-main = Main navigation
nav-projects = Projects
nav-charts = Statistics
nav-settings = Settings

### Connectivity

offline-banner = Offline
offline-pending =
    { $count ->
        [one] { $count } pending
       *[other] { $count } pending
    }

# The queued write could not be replayed because what it targets is gone. One message per verb
# rather than an interpolated noun: languages inflect the surrounding words differently.
sync-conflict-edit = Conflict: editing “{ $name }” failed (item deleted). Skipped.
sync-conflict-delete = Conflict: deleting “{ $name }” failed (item deleted). Skipped.
sync-conflict-other = Conflict: the operation on “{ $name }” failed (item deleted). Skipped.
sync-error = Sync error: { $reason }

### Errors
#
# Keyed off the constants in `shared::errors`, which the server returns verbatim. Anything
# unrecognised renders `error-generic`, so raw server text never reaches the DOM.

error-network = Can’t reach the server. Check your internet connection.
error-generic = Something went wrong. Please try again.

error-invalid-email = That email address isn’t valid.
error-invalid-password = That password isn’t valid.
error-password-too-short = Your password must be at least 8 characters.
error-client-outdated = This version of the app is outdated. Please update it to sign in.
error-invalid-link = This link isn’t valid.
error-batch-too-large = Too many items at once.
error-payers-required = Select at least one payer.
error-debtors-required = Select at least one person who owes.
error-duplicate-participant = A participant appears twice on the same side.
error-participant-not-in-project = That participant isn’t part of this project.
error-too-many-participants = Too many participants for one expense.
error-invalid-credentials = Incorrect email or password.
error-unauthenticated = Sign in to do that.
error-email-not-verified = Your email address isn’t verified yet.
error-project-not-found = This project no longer exists.
error-expense-not-found = This expense no longer exists.
error-storage-full = Storage is full: this project's key could not be saved on this device. Keep its share link.
error-user-not-found = This participant no longer exists.
error-tricount-not-found = Tricount not found, or its API returned an error.
error-too-many-members = This project has reached its limit of members.
error-identity-taken = Another account has already claimed this participant.
error-claim-proof-invalid = This device does not hold the project key, so it cannot claim a participant. Open the share link again.
error-user-has-payments = This participant has expenses in the project and can’t be removed.
error-resend-cooldown = Wait 60 seconds before asking for another email.
error-self-friend-request = You can’t add yourself as a friend.
error-not-a-friend = You can only invite people from your friends list.
error-friend-has-no-key = This friend hasn’t opened the latest version of the app yet. Ask them to sign in once, then try again.
error-friend-request-not-found = This friend request no longer exists.
error-invitation-not-found = This invitation no longer exists.
error-too-many-friend-requests = Too many friend requests for now. Try again tomorrow.
error-too-many-invitations = Too many pending invitations.
error-invalid-kdf-salt = Your encryption settings aren’t valid. Update the app and try again.
error-mixed-project-batch = Those participants aren’t all in the same project.
error-invalid-payload = This version of the app sent data the server couldn’t accept. Update it and try again.
error-invalid-public-key = Your encryption key isn’t valid. Update the app and try again.
error-payment-methods-stale = Your payment details were changed on another device. Reload and try again.

### Auth

field-email = Email
field-email-placeholder = you@example.com
field-password = Password
field-name = Name
field-name-placeholder = Jane Doe

login-title = Sign in
login-submit = Sign in
login-submitting = Signing in…
login-password-placeholder = Your password
login-no-account = Don’t have an account yet?
login-unverified = Your email address isn’t verified yet. Check your inbox, or send the link again.
login-resend = Send the verification link again
login-resending = Sending…
login-resend-sent = Email sent - check your inbox.

register-submit = Create an account
register-submitting = Creating…
register-have-account = Already have an account?
register-password-placeholder = At least 8 characters
register-password-warning = Write your password down. If you forget it, your account can’t be recovered.
register-check-email-title = Check your email
register-email-sent = Email sent
register-email-sent-hint = Click the link in your inbox to activate your account.
# Split around an inline link - Fluent can’t embed a component, so each half is its own message.
register-not-received-prefix = Didn’t get it? Check your spam folder, or
register-sign-in-link = sign in
register-not-received-suffix = to send the link again.
register-terms-prefix = By creating an account you accept our
register-terms-link = terms of use
register-terms-and = and our
register-privacy-link = privacy policy

settings-title = Settings
settings-preferences = Preferences
settings-preferences-local = Stored on this device.
settings-preferences-synced = Synced with your account, encrypted.
settings-about = About
settings-anonymous-title = You are not signed in
settings-upsell-title = Your projects, on every device
settings-upsell-free = Free
settings-upsell-body = Counted works without an account. With a free one, your projects and preferences follow you to your phone, your laptop and the web - still encrypted, still unreadable to us.
settings-locked-badge = Account
settings-locked-friends = Create an account to add friends and invite them into a project from the app - no link to pass around.
settings-locked-payment-methods = Save your IBAN or payment app once and share it with the projects you choose. Whoever owes you sees it next to your name.
settings-friends-hint = Add friends and invite them into your projects without sharing a link.

account-member-since = Member since
account-logout = Sign out
account-logging-out = Signing out…
account-delete-title = Delete my account
account-delete-warning = Immediate and permanent, with no recycle bin. Expenses you entered in a shared project stay visible to the other members - they are part of their books.
account-delete-confirm-title = Delete account
account-delete-confirm-message = Your account, your sessions and your list of projects will be permanently deleted. Without your password, the encrypted data of a shared project becomes unreadable to you - this cannot be undone.

settings-payment-methods = Payment details
settings-payment-methods-hint = How you would like to be paid back. Encrypted with your account.
payment-method-kind = Method
payment-method-kind-other = Other
payment-method-label = Name
payment-method-label-placeholder = Main account
payment-method-value = Details
payment-method-value-placeholder = IBAN, phone number, username…
payment-method-add = Add
payment-method-remove = Remove { $name }
payment-method-empty = You have not added any payment details yet.
payment-method-deleted = Payment method deleted.
payment-method-value-required = Fill in the details of each payment method, or remove it.
payment-method-label-required = Give your custom method a name.
payment-method-too-long = That is too long - shorten it.
payment-method-invalid-characters = Remove any line breaks or invisible characters.
payment-method-limit = You can save up to { $max } payment methods.
payment-methods-saved = Payment details saved.
payment-methods-offline = You need to be online to save your payment details.
payment-methods-stale = Your payment details were changed on another device. They have been reloaded — please try again.
payment-methods-key-missing = Sign in again to manage your payment details.
settings-payment-methods-share-warning = A shared method is visible to every member of the projects you have picked your name in - anyone holding one of those project links.
payment-method-share = Share with my projects
payment-method-share-hint = Shown next to your name when someone owes you money.
payment-method-copy = Copy { $name }
payment-method-copied = Copied.
payment-method-copy-failed = Could not copy - select the text and copy it by hand.

verify-email-checking = Verifying your email address…
verify-email-welcome = Email verified - welcome to Counted!
verify-email-back-to-login = Back to sign in

### Project status

project-close = Close
project-archive = Archive
project-reopen = Reopen
project-unarchive = Unarchive

### Dates
#
# `date-long` owns the field ORDER, so each language can put the month where it belongs.
# fluent-rs ships no CLDR data, so its DATETIME builtin cannot do this.

date-long = { $month } { $day }, { $year }

month-1 = January
month-2 = February
month-3 = March
month-4 = April
month-5 = May
month-6 = June
month-7 = July
month-8 = August
month-9 = September
month-10 = October
month-11 = November
month-12 = December

month-short-1 = Jan
month-short-2 = Feb
month-short-3 = Mar
month-short-4 = Apr
month-short-5 = May
month-short-6 = Jun
month-short-7 = Jul
month-short-8 = Aug
month-short-9 = Sep
month-short-10 = Oct
month-short-11 = Nov
month-short-12 = Dec

### Actions

add = Add
create = Create
creating = Creating…
edit = Edit
leave = Leave
close = Close
paste = Paste
join = Join
import = Import
importing = Importing…
field-description = Description
field-date = Date
date-today = Today
date-yesterday = Yesterday
field-optional = Optional

### Projects

projects-filter-active = Active
projects-filter-all = All
projects-count-label = Projects
projects-empty = No projects
projects-empty-hint = Create a project with the button below
projects-offline-banner = Offline data - reconnect to refresh.
projects-no-local-data = No local data
projects-no-local-data-hint = Sign in to load your projects for the first time.
projects-add = Add a project
projects-create = Create a project
projects-join = Join a project
projects-import-tricount = Import from Tricount
project-actions = Project actions

status-ongoing = Ongoing
status-closed = Closed
status-archived = Archived

nav-help = Help
nav-privacy = Privacy policy
nav-terms = Terms of use
nav-legal = Legal notice

leave-project-title = Leave the project?
leave-project-message = You will lose access from this device. If no member is left, the project and all its expenses are permanently deleted.

add-project-title = New project
add-project-name-label = Project name
add-project-name-placeholder = My trip, Flatshare 2024…
add-project-participants = Participants
add-project-participant-name = Participant name
add-project-participant-placeholder = Clark Kent
add-project-offline = You can’t create a project offline. Reconnect and try again.
add-project-name-required = The project needs a name.

join-link-label = Share link
join-link-hint = The link carries the decryption key - copy all of it.
join-invalid-link = That link isn’t valid. Paste the whole share link, including the part after the #.
join-wrong-project = That link is for a different project.

import-tricount-link-label = Tricount link or key
import-tricount-key-required = Enter a Tricount link or key.
import-tricount-encryption-failed = Encryption failed.
import-tricount-unimportable = Nothing was imported: this Tricount has members with a Tricount account, or amounts that don't add up (entries affected: { $count }).

### Expenses

save = Save
saving = Saving…
adding = Adding…
link-copied = Link copied
missing-encryption-key = Missing encryption key.
missing-encryption-key-title = Missing encryption key
missing-encryption-key-hint = The link you used does not carry the key needed to decrypt this project. Use the full link shared by whoever created it.
project-locked-hint = This device doesn’t have this project’s key. Open its share link to unlock it.
project-unlock = Unlock
project-no-local-data-hint = Sign in to load this project's data for the first time.
project-gone-title = This project no longer exists
project-gone-hint = It was deleted when its last member left. The share link no longer works, even if you reopen it.

expense-add = Add an expense
transfer-add = Add a transfer
expense-edit-title = Edit the expense
expense-category = Category
expense-category-auto = Auto · { $emoji }
expense-currency = Currency of the amount
amount-op-add = Plus
amount-op-subtract = Minus
amount-op-multiply = Multiply
amount-op-divide = Divide
amount-op-equals = Equals
amount-op-done = Done
expense-rate = Exchange rate (optional)
expense-rate-hint = Leave empty to use the European Commission (InforEuro) rate for { $month }: 1 { $from } = { $rate } { $to }.
expense-rate-invalid = Enter an exchange rate greater than 0.
expense-rate-unavailable = No automatic rate available - enter one by hand.
expense-delete-title = Delete the expense
expense-delete-message = “{ $name }” will be permanently deleted. This cannot be undone.
expense-inconsistent-amounts = Amounts don’t add up
expenses-empty = No expenses
expenses-empty-hint = Start by adding expenses with the button below
expenses-show-more = Show more ({ $count } left)

expense-type-expense = Expense
expense-type-transfer = Transfer
expense-type-gain = Gain
expense-paid-by = paid by
expense-sent-by = sent by
expense-contributed-by = contributed by

expense-name-required = A name is required.
expense-amount-not-positive = The amount must be greater than 0.
expense-no-payer = Select at least one payer.
expense-no-debtor = Select at least one person who owes.
expense-invalid-date = That date isn’t valid.
# One message per side: the surrounding words inflect differently for each in most languages.
expense-payers-mismatch = The payers add up to { $sum }, which doesn’t match the expense amount ({ $total }).
expense-debtors-mismatch = The debtors add up to { $sum }, which doesn’t match the expense amount ({ $total }).

participants-none = Nobody
participants-everyone = Everyone ({ $count })
participants-some = { $count } of { $total }
participants-select-all = Select all
participants-by-shares = By shares
split-amounts = Amounts
participants-remaining = { $amount } left
participants-over-by = { $amount } over
participants-who-paid = Who paid?
participants-who-received = Who received?
participants-who-transfers = Who is transferring?
participants-who-receives = Who receives?
participants-for-whom = For whom?

stats-total-expenses = Total expenses
stats-my-expenses = My expenses

tab-expenses = Expenses
tab-balance = Balance
tab-reimbursements = Settle up
reimbursements-empty-title = All settled up!
reimbursements-empty-hint = Settlement suggestions appear here when the accounts don’t balance
reimbursement-owes = { $debtor } owes { $creditor }
reimbursement-record = Settle
reimbursement-pay-with = Pay
reimbursement-pay-shared-by = Shared by { $name } - check the recipient name your app shows before sending.
reimbursement-pay-title = Pay { $name }
reimbursements-mine-title = You owe
reimbursements-others-title = Other reimbursements
copy = Copy

user-selection-title = Which participant are you?
user-selection-hint = Pick your name from the list.
user-selection-required = Please select a participant.
identity-claimed = Linked to an account
identity-claimed-by = { $name }'s account
identity-taken-repick = Another account has claimed the participant you were using. Please pick another one.
participant-gone-repick = The participant you were using has been removed from this project. Please pick another one.

edit-project-title = Edit the project
edit-project-new-badge = new
edit-project-deferred-new-members = adding new members
edit-project-deferred-removals = removing members
edit-project-offline-deferred = Offline: { $items } will be applied when you reconnect.

export-failed = Export failed: { $reason }

# History entries are encrypted and stored, so they keep the language of whoever wrote them.
history-expense-added = Expense added: { $name }
history-expense-edited = Expense edited: { $name }
history-expense-deleted = Expense deleted: { $name }
history-project-edited = Project edited: { $name }
history-name-changed = Name: “{ $from }” → “{ $to }”
history-description-added = Description added: “{ $value }”
history-description-removed = Description removed: “{ $value }”
history-description-changed = Description: “{ $from }” → “{ $to }”

### Sweep

field-amount = Amount
expense-name-placeholder = Restaurant, groceries…
expense-actions = Expense actions
expense-your-share = Your share
expense-your-share-value = Your share: { $amount } { $currency }
expense-inconsistent-detail = Amounts don’t add up: { $paid } paid, { $owed } owed, for an expense of { $total }. Edit the expense to fix it.
missing-access-key = Access key missing. Open this project through its share link.
filter-all = All
filter-my-payments = My payments
filter-my-debts = What I owe
participants-shares-for = Shares for { $name }
participants-amount-for = Amount for { $name }
reimbursement-add = Add a settlement
project-forget = Remove from my list
project-history-title = History
history-kind-add = Added
history-kind-delete = Deleted
history-kind-edit = Edited
export = Export
export-json = Export JSON
export-csv = Export CSV
share-link = Share
copy-link-failed = Could not copy the link
open-in-app = Open in the app
not-found-title = Page not found
not-found-back = Back to projects

### Charts

charts-period = Period
# Keep these short: five equal segments have to fit at 360px.
period-all = All
period-month = Month
period-3months = 3 mo
period-year = Year
period-custom = Custom
charts-tab-categories = Categories
charts-tab-trends = Trends
charts-total-spent = Total spent
charts-avg-per-person = Avg. per person
charts-expense-count =
    { $count ->
        [one] { $count } expense
       *[other] { $count } expenses
    }
charts-nothing-to-show = Nothing to show
charts-my-share-note = These figures are your share of each expense.
charts-my-share-skipped =
    { $count ->
        [one] 1 project isn’t counted — no participant picked, or its data didn’t load.
       *[other] { $count } projects aren’t counted — no participant picked, or their data didn’t load.
    }

### Categories
#
# The ids these label are French strings persisted inside encrypted payloads - see
# `categories::category_label`. Never translate the id, only this label.

category-food = Food
category-transport = Transport
category-accommodation = Accommodation
category-leisure = Leisure
category-shopping = Shopping
category-services = Services
category-parties-gifts = Parties & gifts
category-other = Other
charts-project = Project
charts-all-projects = All projects
charts-date-from = From
charts-date-to = To
charts-total = Total
charts-tab-people = People
charts-tab-projects = Projects
charts-scope = Whose spending
charts-scope-group = Group
charts-scope-me = Me
charts-currency = Currency
charts-my-share = My share
charts-share-of-total = { $pct }% of { $total }
charts-i-paid = I paid
charts-paid-more = { $amount } more than your share
charts-paid-less = { $amount } less than your share
charts-paid-even = Exactly your share
charts-part-title = Your part of each category
charts-part-desc = Grey is what the group spent, colour is what you consumed.
charts-breakdown-title = Breakdown by category
charts-breakdown-desc = Tap a slice or a row to list its expenses.
charts-of-total = { $amount } of { $total }
charts-show-all = Show all ({ $count })
charts-show-less = Show less
charts-spend-title = Spending over time
charts-spend-desc = Short periods go by day, longer ones by week or month.
charts-group-by = Group by
bucket-day = Day
bucket-week = Week
bucket-month = Month
charts-avg = avg
charts-cat-title-day = { $category }, day by day
charts-cat-title-week = { $category }, week by week
charts-cat-title-month = { $category }, month by month
charts-cat-desc = Pick a category to follow it over time.
charts-running-title = Running total
charts-running-desc = Since { $date }.
charts-avg-per-day = { $amount } / day on average
charts-avg-per-week = { $amount } / week on average
charts-avg-per-month = { $amount } / month on average
charts-people-title = Who carried the group
charts-people-desc = What each person paid, next to what they consumed.
charts-paid = Paid
charts-fair-share = Fair share
charts-you = (you)
charts-net-more = paid more
charts-net-less = paid less
charts-balance-title = Your balance over time
charts-balance-desc = Above the line the group owes you. Below it, you owe the group.
charts-owed = You’re owed
charts-owe = You owe
charts-projects-title = Your share, by project
charts-projects-desc = Totals are kept per currency, never added across them.
history-empty = No events
history-by = By { $name }
not-found-hint = This page doesn’t exist, or it moved.
payers-title-paid-by = Paid by
payers-title-sender = Sender
payers-title-contributors = Contributors
debtors-title-debtors = Owes
debtors-title-recipients = Recipients
debtors-title-beneficiaries = Beneficiaries

### Welcome
#
# Onboarding copy. The EU-stack claims are specific and load-bearing - see docs/welcome-copy.md
# before rewording any of them in any language.

welcome-title = Your books are nobody else’s business.
welcome-subtitle = Split expenses with friends.
welcome-note = Free. No account required. No ads.
welcome-link-title = One link, and everyone joins in.
welcome-link-body = Nobody has to create an account.
welcome-link-account = An account? Never required. It is for finding your projects on another device, inviting friends from the app and sharing your payment details.
welcome-demo-project = Weekend in Lyon
welcome-private-title = Nobody can read your books. Not even us.
welcome-private-body = Names, amounts, projects: everything is encrypted on your device. Only you hold the key.
welcome-private-names = Names
welcome-private-amounts = Amounts
welcome-private-projects = Projects
welcome-scan-title = Take a photo of the receipt.
welcome-scan-body = Amount, date and category fill in on their own. Everything happens on your phone. The photo is not kept.
welcome-eu-title = 100% European
welcome-no-ads = No ads
welcome-no-trackers = No trackers
welcome-step = Step { $current } of { $total }
welcome-next = Next
welcome-skip = Skip
welcome-start = Get started
welcome-how-it-works = How does it work, exactly?

### Help

help-intro = A common question? Tap to expand the answer.
help-create-project-q = How do I create a project?
help-create-project-a = From the home screen, tap the + button at the bottom. Give the project a name, pick its currency, and you’re set.
help-add-participants-q = How do I add participants?
help-add-participants-a = Open the project, then add participants from the member list. Every participant can pay for or owe on an expense.
help-share-project-q = How do I share a project?
help-share-project-a = Share the project URL (the one in your address bar). Anyone with the link can view and edit the project.
help-add-expense-q = How do I add an expense?
help-add-expense-a = Inside a project, tap +, enter the amount, say who paid and who to split it between. You can also pick a date other than today.
help-types-q = What’s the difference between an expense, a transfer and a gain?
help-types-expense = - a purchase made by one person and split between several.
help-types-transfer = - a repayment from one person to another, with no split.
help-types-gain = - money received (a refund, a gift) to split between several people.
help-past-date-q = Can I date an expense in the past?
help-past-date-a = Yes, the date field is free. The record’s creation time is kept separately.
help-who-owes-q = How does Counted work out who owes what?
help-who-owes-a = Counted computes each participant’s net balance (what they fronted minus what they owe), then proposes the shortest series of transfers that settles everyone.
help-minimal-transfers-q = Why is the number of suggested transfers minimal?
help-minimal-transfers-a = The algorithm first pairs balances that cancel out exactly, then works through the rest from the largest creditor to the largest debtor. The result: fewer transfers to settle everything.
help-import-tricount-q = How do I import a project from Tricount?
help-import-tricount-a = From the home screen, tap the “+” button at the bottom, then
help-import-tricount-b = Paste the share link of the Tricount you want to import.
help-encryption-q = Is my data encrypted?
help-encryption-a = Yes. Counted combines two guarantees:
help-encryption-e2ee-term = End-to-end encryption
help-encryption-e2ee-def = - everything between you and the server travels encrypted.
help-encryption-zero-term = Zero access
help-encryption-zero-def = - you encrypt the data before sending it, and the server stores nothing but ciphertext. We have no way to read it.
help-encryption-see = For the details, see the
help-forgot-password-q = What happens if I forget my password?
help-forgot-password-warning = Your data will be permanently lost.
help-forgot-password-a = The encryption key is derived from your password, so no reset is possible: nobody - us included - can decrypt your projects without it. Keep it safe, ideally in a password manager.
help-archive-delete-q = How do I archive or delete a project?
help-archive-delete-a = From the project screen, open the menu and choose
help-archive-delete-b = to hide it while keeping it. A project is deleted for good once its last member leaves it.
help-delete-account-q = How do I delete my account?
help-delete-account-a = Open Settings and use “Delete my account”. It is immediate and cannot be undone.
help-contact = Another question? Write to us at

### Legal
#
# Legal and policy text. Translated into en and fr only, on purpose: every other locale falls back
# to English per message until a translation has actually been reviewed. Do not machine-translate
# this section.

legal-updated = Last updated: 28 September 2026

legal-publisher-title = 1. Publisher
legal-publisher-body = Counted (“the Service”) is published in a non-professional capacity by an individual, Jonathan Bosi.
legal-contact-label = Contact:
legal-publisher-address-note = Under article 6-III-2 of French law no. 2004-575 of 21 June 2004 on confidence in the digital economy, a publisher who is an individual acting in a non-professional capacity does not publish their postal address. Those identifying details are held by the host, who may disclose them to the judicial authority.
legal-director-title = 2. Publication director
legal-host-title = 3. Host
legal-host-address = Industriestr. 25, 91710 Gunzenhausen, Germany
legal-host-phone = Phone: +49 (0)9831 505-0
legal-host-email-note = Transactional email is delivered by Scaleway SAS (France). The servers and the database are located in the European Union.
legal-ip-title = 4. Intellectual property
legal-ip-body = The structure of the site, its text and its graphics belong to the publisher unless stated otherwise. The data you enter stays yours: it is encrypted on your device, and the publisher can neither read nor use it.
legal-ip-source = The source code of the web and mobile apps is free software, published under the GNU Affero General Public License, version 3 only, at
legal-ip-brand = The name “Counted”, its logo and the counted.fr domain are not covered by that licence.
legal-ip-third-party = The apps include third-party software, each under its own licence:
legal-licenses-title = Third-party licences
legal-licenses-unavailable = The licence list could not be loaded. It is also in the source code, in packages/ui/licenses/third-party.txt.
legal-personal-data-title = 5. Personal data
legal-personal-data-body = The processing of personal data is described in the
legal-terms-title = 6. Terms of use
legal-terms-body = Use of the Service is governed by the
legal-report-title = 7. Reporting
legal-report-body-a = Any content or behaviour that may break the law can be reported to
legal-report-body-b = Project contents are end-to-end encrypted, so the publisher has no access to them and can only act on the accounts involved.

terms-intro = Counted is a free service for splitting expenses among people who know each other. These terms describe what the Service does, what it does not do, and what each side undertakes to do.
terms-purpose-title = 1. Purpose
terms-purpose-body = These terms govern the use of the Counted service, available at counted.fr and through its mobile apps. The publisher of the Service is identified in the
terms-acceptance-title = 2. Acceptance
terms-acceptance-body = Using the Service means accepting these terms. If you do not accept them, do not use the Service.
terms-access-title = 3. Access to the Service
terms-access-body = The Service is free. A project can be created and shared without an account: anyone holding the share link can open it. Creating an account is optional and serves to find your projects again from another device.
terms-account-title = 4. Account and password
terms-account-body = You are responsible for keeping your password confidential and for what is done from your account.
terms-account-key-point = The essential point:
terms-account-warning-a = your encryption key is derived from your password and never leaves your device. If you forget it,
terms-account-warning-em = no recovery is possible
terms-account-warning-b = - neither by you nor by the publisher. There is deliberately no reset procedure, because one would require access to your data.
terms-acceptable-use-title = 5. Acceptable use
terms-acceptable-use-intro = You undertake not to:
terms-acceptable-use-1 = use the Service for unlawful or fraudulent purposes;
terms-acceptable-use-2 = attempt to reach projects that have not been shared with you;
terms-acceptable-use-3 = interfere with the operation of the Service, in particular through massive automated requests;
terms-acceptable-use-4 = circumvent the technical limits in place (quotas, rate limiting);
terms-acceptable-use-5 = use the Service to send unsolicited messages to third parties, including through email invitations.
terms-your-content-title = 6. Your content
terms-your-content-body = You remain the owner of the data you enter. It is encrypted on your device before being transmitted: the publisher hosts a version it cannot read, and therefore performs no moderation of its content. You alone are responsible for what you record there and for your right to include information about other people.
terms-availability-title = 7. Availability
terms-availability-body = The Service is provided “as is”, with no guarantee of availability or of being error-free. It may be interrupted, changed or discontinued at any time, in particular for maintenance. Split and settlement calculations are indicative: they are neither a payment service nor financial or accounting advice, and no money moves through the Service.
terms-deletion-title = 8. Deletion
terms-deletion-account = You can delete your account at any time from the “My account” page. Deletion is immediate and permanent.
terms-deletion-project = A project lives as long as one member keeps it: when the last member leaves, it is deleted automatically, along with the expenses it contains. Expenses you entered in a shared project do remain visible to the other members after you leave - they are part of their books.
terms-backup-title = 9. Backups
terms-backup-body = Technical backups are taken for the continuity of the Service, but they are not an archiving service for your benefit. Export your projects (CSV or JSON) if you want to keep a copy.
terms-liability-title = 10. Liability
terms-liability-body = As the Service is provided free of charge and in a non-professional capacity, the publisher cannot be held liable for indirect damage resulting from its use, in particular loss of data following a forgotten password. Mandatory legal provisions protecting consumers still apply.
terms-liability-suspension = The publisher may suspend or close an account in the event of a clear breach of article 5.
terms-personal-data-title = 11. Personal data
terms-changes-title = 12. Changes to these terms
terms-changes-body = These terms may change. Any substantial change will be announced in the app or by email.
terms-law-title = 13. Governing law
terms-law-body-a = These terms are governed by French law. In the event of a dispute, an amicable solution will be sought first by writing to
terms-law-body-b = Failing that, the French courts have jurisdiction.

privacy-intro-a = Counted is a service for splitting expenses among friends, built around one simple principle:
privacy-intro-em = the server never sees your data in the clear.
privacy-intro-b = This policy describes what information we collect, why, and what rights you have.
privacy-controller-title = 1. Publisher and data controller
privacy-controller-a = Counted is published by Jonathan Bosi - see the
privacy-controller-b = For any question about your personal data, you can write to
privacy-collected-title = 2. Data collected
privacy-collected-email-term = Email address
privacy-collected-email-def = (required to create an account).
privacy-collected-hash-term = Password hash
privacy-collected-hash-def = your device turns the password into a login proof with argon2id and sends only that proof; we store a hash of it. The password itself is never transmitted or stored.
privacy-collected-salt-term = Key derivation salt
privacy-collected-salt-def = used to generate your encryption key on the client.
privacy-collected-content-term = Encrypted project content
privacy-collected-content-def = (names, expenses, participants, amounts). This data is encrypted on your device before being sent to the server, which stores nothing but an unintelligible version of it.
privacy-collected-prefs-term = Your encrypted preferences
privacy-collected-prefs-def = (interface language), so you find them again on your other devices. Encrypted on your device with the same key as the rest: the server cannot read them.
privacy-collected-keys-term = Your project keys, wrapped
privacy-collected-keys-def = so that signing in on a new device gives you back projects you can actually read, instead of a list you cannot open. Each one is encrypted on your device with the key derived from your password; the server stores them and cannot unwrap them. The trade-off is real and we would rather state it: someone holding both our database and your password could reach your projects. That is why the password is never transmitted and never stored: what we receive at sign-in is a separate proof it cannot be recovered from, and we keep only a hash of that.
privacy-collected-invite-term = Hash of an invited participant’s email
privacy-collected-invite-def = (SHA-256), when you invite someone by email. It serves only to attach the invitation to their account should they create one, and disappears with the participant.
privacy-collected-friends-term = Your friends
privacy-collected-friends-def = when you use the friends list: which accounts you are friends with, a hash (SHA-256) of an address you sent a request to before it had an account, and which friend you invited into which project, and as which participant of it. The name you give a request and the project key an invitation carries are encrypted on your device - the key with your friend’s public key, which we store in the clear because it is public by nature - and the server cannot read either.
privacy-collected-push-term = Notification token
privacy-collected-push-def = if you allow notifications in the mobile app: the address your phone's push service gives it (on Android, with two public encryption keys), linked to your account, along with your phone's language so the notification is written in it. Deleted when you sign out, when the push service reports it is no longer valid, and with your account.
privacy-collected-logs-term = Technical logs
privacy-collected-logs-def = (IP address, user agent, timestamp) kept for security and abuse prevention.
privacy-purposes-title = 3. Purposes
privacy-purpose-1 = Authenticate your sessions and protect your account.
privacy-purpose-2 = Sync your projects between your devices.
privacy-purpose-3 = Send you a verification email when you sign up.
privacy-purpose-4 = Detect and prevent abuse (brute force, scraping).
privacy-purpose-5 = If you allowed notifications, tell your phone about a friend request or its acceptance, a project invitation or a project being closed.
privacy-legal-basis-title = 4. Legal basis
privacy-legal-basis-body = Processing rests on performance of the contract between us (creating and providing the service) and on our legitimate interest in securing the platform.
privacy-e2ee-title = 5. End-to-end encryption
privacy-e2ee-a = Counted applies a
privacy-e2ee-b = model: your encryption key is derived from your password and never leaves your device. The server stores only encrypted blobs it is unable to read.
privacy-e2ee-consequence-label = An important consequence:
privacy-e2ee-consequence-a = if you forget your password,
privacy-e2ee-consequence-em = nobody
privacy-e2ee-consequence-b = - not even us - can recover your data. No reset procedure is possible.
privacy-processors-title = 6. Processors and hosting
privacy-processor-hetzner = (Germany, EU) - hosting of the servers and the database.
privacy-processor-scaleway = (France, EU) - sending transactional email (email verification).
privacy-processor-tricount = - contacted only if you trigger an import from Tricount yourself, in order to fetch the project you want to import.
privacy-processor-grafana = (EU region) - technical monitoring of the server. Only infrastructure metrics (CPU, memory, disk, service state) and daily aggregate usage counts (numbers of accounts, projects, memberships and sign-ups, and how many sign-ups and logins were made in each interface language) are sent there: no content, no identifier, no visitor IP address.
privacy-processor-apple = (Apple Push Notification service) - only if you allow notifications on iPhone: receives your phone's token and a generic sentence such as “New friend request”, never a name, an amount or a project.
privacy-processor-android-push-term = Your Android phone's push service
privacy-processor-android-push-def = (Google Firebase Cloud Messaging on most phones, or the UnifiedPush distributor you installed) - only if you allow notifications on Android: receives a message encrypted for your phone alone and padded to a fixed size, so it cannot tell even what kind of notification it is.
privacy-no-transfer-outside-eu = These two push services may process data outside the European Union. Apart from them, no personal data is transferred outside the European Union.
privacy-retention-title = 7. Retention
privacy-retention-a = Your data is kept for as long as your account is active. Deleting the account causes an
privacy-retention-em = immediate and permanent
privacy-retention-b = deletion of the corresponding database records (cascading deletion, with no recycle bin).
privacy-sweep-intro = A daily automatic sweep additionally deletes:
privacy-sweep-1 = expired sessions and verification links;
privacy-sweep-2 = accounts created but never verified, after 24 hours;
privacy-sweep-3 = projects no member belongs to any more, after 24 hours.
privacy-logs-a = Technical logs are kept for
privacy-logs-duration = 30 days at most
privacy-logs-b = and then deleted automatically. They are neither archived, nor exported, nor analysed for any purpose other than security.
privacy-shared-expenses-survive = Expenses you entered in a shared project do not disappear with your account: they are part of the other members’ books, and they alone can still decrypt them.
privacy-cookies-title = 8. Cookies and local storage
privacy-session-cookie-a = A single session cookie
privacy-session-cookie-b = is set after signing in to keep you authenticated. No tracking cookie, no third-party analytics.
privacy-lang-cookie-a = A language cookie
privacy-lang-cookie-b = stores the interface language you picked, so the page is served in it from the first render. It carries a two-letter language code and no identifier.
privacy-local-storage-a = The app also keeps information in your browser’s (or the mobile app’s)
privacy-local-storage-em = local storage.
privacy-local-storage-b = It is never sent to the server:
privacy-ls-keys-term = Your decryption keys
privacy-ls-keys-def = - your account’s and each project’s. Without them, the app can display nothing.
privacy-ls-projects-term = The list of your projects
privacy-ls-projects-def = and the random identifier representing you in a project joined without an account.
privacy-ls-cache-term = A cache of the projects
privacy-ls-cache-def = (expenses, participants) for offline display and to avoid re-downloading what has not changed.
privacy-ls-queue-term = A queue
privacy-ls-queue-def = of the changes made offline, sent to the server when the network comes back.
privacy-ls-prefs-term = Your display preferences
privacy-ls-prefs-def = (archived projects, onboarding already seen, interface language).
privacy-ls-necessary-a = These items are strictly necessary for the service to work and therefore require no consent. You can erase them at any time by clearing the site data -
privacy-ls-warning-label = careful:
privacy-ls-necessary-b = erasing your keys without knowing your password makes your projects unreadable.
privacy-rights-title = 9. Your rights
privacy-rights-intro = Under the GDPR, you have the following rights:
privacy-right-access = right of access and rectification;
privacy-right-erasure = right to erasure: the “Delete my account” button on the “My account” page removes everything immediately, without going through us;
privacy-right-portability = right to portability: every project exports to decrypted CSV or JSON from its own menu;
privacy-right-object = right to object and to restriction;
privacy-right-withdraw = right to withdraw your consent at any time;
privacy-right-complaint = right to lodge a complaint with the French data protection authority, the
privacy-rights-contact = To exercise these rights, write to
privacy-security-title = 10. Security
privacy-security-body = TLS 1.2+ communications, passwords stored as argon2id hashes, user data encrypted end to end, rate limiting at the reverse proxy, permanent deletion with no recycle bin.
privacy-changes-title = 12. Changes
privacy-changes-body = This policy may change. Any substantial change will be notified through the app or by email.

# Receipt scanning (mobile only)
expense-scan = Scan a receipt
scan-in-progress = Reading the receipt…
scan-error-capture = Couldn’t take that photo. Try again, or enter the expense by hand.
scan-error-unreadable = Nothing readable on that receipt. Enter the expense by hand.
scan-check-amount = Check the total - it wasn’t clearly printed.
scan-take-photo = Take a photo
scan-choose-photo = Choose a photo
privacy-scan-title = 11. Receipt scanning
privacy-scan-body = On the mobile app you can photograph a receipt, or pick a photo already on your phone, to pre-fill an expense. The recognition model ships inside the app and runs on your phone: the image is held in memory, read, and discarded. No photograph and no recognised text is ever sent to our servers or to anyone else, and no network request is made to perform it.
privacy-scan-retention = The copy your phone makes of the photo, whether you take it or pick it from your gallery, is deleted as soon as the scan ends - whether it succeeded, failed, or you backed out. Your original photo is never altered or removed. Only the fields you confirm are saved, as an ordinary expense, encrypted on your device like every other.
expense-converted-from = Paid { $amount } { $from } · 1 { $from } = { $rate } { $to }
project-currency = Currency
project-currency-hint = Every amount is shown in this currency. It cannot be changed later.
project-currency-locked = The currency is fixed when the project is created.

update-required-title = Update required
update-required-body = This version of Counted is too old to talk to the server. Update it to keep using the app.
update-required-button = Update

# Notification bell on the projects page - today only incoming friend requests.
notifications-label = Notifications
notifications-title = Notifications
notifications-empty = Nothing new
notifications-friend-request = Friend request

# Friends and invitations - see docs/plans/friends.md. "Request sent" is shown whether or not the
# address has an account; nothing here may hint at which.
friends-title = Friends
friends-anonymous-body = Friends are kept with your account. Sign in to add people and invite them into your projects without sharing a link.
friends-add-title = Add a friend
friends-add-hint = They’ll see your request once they sign in. Neither of you is told whether the other has an account until the request is accepted.
friends-add-button = Add
friends-add-from-project = Add as friend
friends-request-sent = Request sent
friends-no-account-key = Sign in again to manage your friends on this device.
friends-incoming-title = Requests
friends-accept = Accept
friends-decline = Decline
friends-list-title = My friends
friends-list-empty = No friends yet. Add someone by email above, or from a project you share.
friends-remove = Remove
friends-remove-confirm-title = Remove friend
friends-remove-confirm-message = { $email } will no longer be in your friends, and you will no longer be in theirs. Either of you can send a new request later.
friends-no-key = Not ready yet
friends-fingerprint = Safety code
friends-fingerprint-hint = Two friends who read each other the same safety code know nobody sits between them - not even our server.
friends-outgoing-title = Sent
friends-outgoing-hint = Waiting for an answer. You’ll see them in your friends once they accept.
friends-withdraw = Cancel
invite-friends-title = Invite friends
invite-friends-hint = The project key is encrypted for each friend on this device. The server never sees it.
invite-friends-empty = No friends to invite yet.
invite-friends-button = Invite
invite-sent = { $count ->
    [one] Invitation sent
   *[other] { $count } invitations sent
}
invitation-badge = Invitation
invitation-to = Join “{ $name }”
invitation-to-unnamed = Join a project
invitation-unreadable = This invitation can’t be opened on this device
invitation-from = From { $email }
invitation-accept = Join
invitation-decline = Decline

# Participants in the create and edit modals, and the "who are you?" picker - see
# docs/plans/friends.md §11.
participants-you-label = Your name in this project
participants-you-badge = You
participants-you-from-account = Filled from your account name. Change it here for this project only.
participants-you-required = Required. This is how the others will see you.
participants-others = Other participants
participants-empty = Nobody yet. Pick a friend below or type any name.
participants-empty-signed-out = Nobody yet. Type a name to add someone.
participants-duplicate = “{ $name }” is already in the list.
participants-input-label = Add a friend or type a name
participants-input-placeholder = Friend or any name
participants-suggest-friend = Friend · joins as “{ $name }”, gets an invitation
participants-suggest-not-ready = Friend · not ready yet
participants-suggest-guest = Add “{ $text }” as a guest
participants-suggest-guest-sub = No account, just a name
participants-friends = Your friends
participants-all-friends = All friends
participants-login-hint = Sign in to add people straight from your friends list.
participants-invite-badge = Invite
participants-guest-badge = Guest
participants-guest-sub = No account, just a name
participants-rename = Rename { $name }
participants-remove = Remove { $name }
participants-rename-label = New name
participants-rename-save = Save the name
participants-rename-hint = The name everyone sees in this project. The invitation still goes to { $email }.
participants-invited-badge = Invited
participants-invited-sub = { $email } · not accepted yet
participants-invited-pending = Invitation not accepted yet
participants-unlinked = Not linked to an account
add-project-create-invite = Create and invite { $count }
edit-project-save-invite = Save and invite { $count }
edit-project-you-are = On this device you are { $name }
edit-project-no-identity = You haven’t picked who you are yet
edit-project-switch = Switch
edit-project-choose = Choose
invite-failed = These invitations couldn’t be sent: { $emails }
invite-again = Invite again
friend-picker-title = Add friends
user-selection-invited-hint = { $email } invited you to “{ $project }”.
user-selection-suggested = Suggested
user-selection-suggested-sub = { $email } added you under this name
user-selection-confirm-as = I’m { $name }
user-selection-missing = Your name isn’t here? Ask a participant to add you in the project settings.
