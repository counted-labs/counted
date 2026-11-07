# Suomi. Täydellinen lukuun ottamatta juridisia tekstejä (legal-, terms-, privacy-), jotka ovat vain
# englanniksi ja ranskaksi ja palautuvat viesteittäin en.ftl-tiedostoon.

### Common

loading = Ladataan…
cancel = Peruuta
discard-title = Hylätäänkö muutokset?
discard-message = Syöttämäsi tiedot menetetään.
discard = Hylkää
confirm = Vahvista
retry = Yritä uudelleen
delete = Poista
back = Takaisin
language = Kieli
theme = Teema
theme-light = Vaalea
theme-dark = Tumma
theme-system = Järjestelmä
theme-hint-device = Tallennettu vain tälle laitteelle.
theme-hint-system = Seuraa tämän laitteen ulkoasua.

### Navigation

nav-main = Päänavigointi
nav-projects = Projektit
nav-charts = Tilastot
nav-settings = Asetukset

### Connectivity

offline-banner = Offline-tila
offline-pending =
    { $count ->
        [one] { $count } odottaa
       *[other] { $count } odottaa
    }

sync-conflict-edit = Ristiriita: kohteen ”{ $name }” muokkaus epäonnistui (kohde poistettu). Ohitettu.
sync-conflict-delete = Ristiriita: kohteen ”{ $name }” poisto epäonnistui (kohde poistettu). Ohitettu.
sync-conflict-other = Ristiriita: toiminto kohteelle ”{ $name }” epäonnistui (kohde poistettu). Ohitettu.
sync-error = Synkronointivirhe: { $reason }

### Errors

error-network = Palvelimeen ei saada yhteyttä. Tarkista internetyhteytesi.
error-rate-limited = Palvelin on kiireinen. Yritä hetken päästä uudelleen.
error-generic = Jokin meni pieleen. Yritä uudelleen.

error-invalid-email = Sähköpostiosoite ei kelpaa.
error-invalid-password = Salasana ei kelpaa.
error-password-too-short = Salasanassa on oltava vähintään 12 merkkiä.
error-client-outdated = Tämä sovellusversio on vanhentunut. Päivitä se kirjautuaksesi.
error-invalid-link = Linkki ei kelpaa.
error-batch-too-large = Liian monta kohdetta kerralla.
error-payers-required = Valitse vähintään yksi maksaja.
error-debtors-required = Valitse vähintään yksi velallinen.
error-duplicate-participant = Osallistuja esiintyy kahdesti samalla puolella.
error-participant-not-in-project = Osallistuja ei kuulu tähän projektiin.
error-too-many-participants = Liian monta osallistujaa yhdelle kululle.
error-invalid-credentials = Väärä sähköposti tai salasana.
error-unauthenticated = Kirjaudu sisään tehdäksesi tämän.
error-email-not-verified = Sähköpostiosoitettasi ei ole vielä vahvistettu.
error-project-not-found = Projektia ei enää ole.
error-expense-not-found = Kulua ei enää ole.
error-storage-full = Tallennustila on täynnä: tämän projektin avainta ei voitu tallentaa tälle laitteelle. Säilytä jakolinkki.
error-user-not-found = Osallistujaa ei enää ole.
error-tricount-not-found = Tricountia ei löytynyt, tai sen API palautti virheen.
error-too-many-members = Projektin jäsenraja on täynnä.
error-identity-taken = Toinen tili on jo lunastanut tämän osallistujan.
error-claim-proof-invalid = Tällä laitteella ei ole projektin avainta, joten se ei voi lunastaa osallistujaa. Avaa jakolinkki uudelleen.
error-user-has-payments = Osallistujalla on kuluja projektissa, eikä häntä voi poistaa.
error-resend-cooldown = Odota 60 sekuntia ennen uuden sähköpostin pyytämistä.
error-self-friend-request = Et voi lisätä itseäsi kaveriksi.
error-not-a-friend = Voit kutsua vain kaverilistasi henkilöitä.
error-friend-has-no-key = Kaverisi ei ole vielä avannut sovelluksen uusinta versiota. Pyydä häntä kirjautumaan kerran ja yritä sitten uudelleen.
error-friend-request-not-found = Kaveripyyntöä ei enää ole.
error-invitation-not-found = Kutsua ei enää ole.
error-too-many-friend-requests = Liian monta kaveripyyntöä toistaiseksi. Yritä huomenna.
error-too-many-invitations = Liian monta avointa kutsua.
error-invalid-kdf-salt = Salausasetukset eivät kelpaa. Päivitä sovellus ja yritä uudelleen.
error-mixed-project-batch = Nämä osallistujat eivät ole kaikki samassa projektissa.
error-invalid-payload = Tämä sovellusversio lähetti tietoja, joita palvelin ei hyväksy. Päivitä se ja yritä uudelleen.
error-invalid-public-key = Salausavaimesi ei kelpaa. Päivitä sovellus ja yritä uudelleen.
error-payment-methods-stale = Maksutietojasi muutettiin toisella laitteella. Lataa uudelleen ja yritä uudelleen.

### Auth

field-email = Sähköposti
field-email-placeholder = sina@esimerkki.fi
field-password = Salasana
field-name = Nimi
field-name-placeholder = Maija Meikäläinen

login-title = Kirjaudu sisään
login-submit = Kirjaudu sisään
login-submitting = Kirjaudutaan…
login-password-placeholder = Salasanasi
login-no-account = Eikö sinulla ole vielä tiliä?
login-unverified = Sähköpostiosoitettasi ei ole vielä vahvistettu. Tarkista postilaatikkosi tai lähetä linkki uudelleen.
login-maybe-unverified = Rekisteröidyitkö juuri? Vahvista ensin osoitteesi lähettämällämme linkillä tai lähetä se uudelleen.
login-resend = Lähetä vahvistuslinkki uudelleen
login-resending = Lähetetään…
login-resend-sent = Sähköposti lähetetty - tarkista postilaatikkosi.

register-submit = Luo tili
register-submitting = Luodaan…
register-have-account = Onko sinulla jo tili?
register-password-placeholder = Vähintään 12 merkkiä
register-password-warning = Kirjoita salasanasi muistiin: emme näe sitä koskaan emmekä voi antaa sitä sinulle takaisin.
register-check-email-title = Tarkista sähköpostisi
register-email-sent = Sähköposti lähetetty
register-email-sent-hint = Napsauta postilaatikossasi olevaa linkkiä aktivoidaksesi tilisi.
register-not-received-prefix = Eikö tullut? Tarkista roskaposti tai
register-sign-in-link = kirjaudu sisään
register-not-received-suffix = lähettääksesi linkin uudelleen.
register-terms-prefix = Luomalla tilin hyväksyt
register-terms-link = käyttöehtomme
register-terms-and = ja
register-privacy-link = tietosuojakäytäntömme

settings-title = Asetukset
settings-preferences = Asetukset
settings-preferences-local = Tallennettu tälle laitteelle.
settings-preferences-synced = Synkronoitu tilillesi, salattuna.
settings-about = Tietoja
settings-anonymous-title = Et ole kirjautunut sisään
settings-upsell-title = Projektisi kaikilla laitteilla
settings-upsell-free = Ilmainen
settings-upsell-body = Counted toimii ilman tiliä. Ilmaisella tilillä projektisi ja asetuksesi seuraavat sinua puhelimeen, kannettavaan ja selaimeen - edelleen salattuina, edelleen meille lukukelvottomina.
settings-locked-badge = Tili
settings-locked-friends = Luo tili lisätäksesi kavereita ja kutsuaksesi heidät projektiin sovelluksesta - ilman linkin välittämistä.
settings-locked-payment-methods = Tallenna IBAN-tilinumerosi tai maksusovelluksesi kerran ja jaa ne valitsemillesi projekteille. Sinulle velkaa oleva näkee ne nimesi vieressä.
settings-friends-hint = Lisää kavereita ja kutsu heidät projekteihisi jakamatta linkkiä.

account-member-since = Jäsen alkaen
account-logout = Kirjaudu ulos
account-logging-out = Kirjaudutaan ulos…
account-delete-title = Poista tilini
account-delete-warning = Tilisi ja tietojesi välitön ja pysyvä poisto.
account-delete-confirm-title = Poista tili
account-delete-confirm-message = Tilisi, istuntosi ja projektilistasi poistetaan pysyvästi. Ilman salasanaasi jaetun projektin salatut tiedot muuttuvat sinulle lukukelvottomiksi - tätä ei voi perua.
erase-device-open = Kirjaudu ulos ja tyhjennä tämä laite
erase-device-title = Tyhjennetäänkö tämä laite?
erase-device-body = Tämän laitteen projektit, niiden avaimet ja välimuisti poistetaan, minkä jälkeen istuntosi päättyy. Projektisi pysyvät tililläsi: kirjaudu uudelleen sisään saadaksesi ne takaisin.
erase-device-checking = Tarkistetaan, että projektisi ovat tililläsi…
erase-device-pending = Lähettämättömiä muutoksia: { $count }. Palaa verkkoon ja yritä sitten uudelleen.
erase-device-unreachable = Ei voitu tarkistaa, että projektisi ovat tililläsi. Tarkista yhteys ja yritä sitten uudelleen.
erase-device-at-risk = Nämä projektit eivät ole tililläsi, ja ne menetetään. Jos haluat säilyttää ne, peruuta ja kopioi ensin niiden linkki:
erase-device-confirm = Tyhjennä ja kirjaudu ulos
erase-device-confirm-anyway = Tyhjennä silti
erase-device-erasing = Tyhjennetään…

settings-payment-methods = Maksutiedot
settings-payment-methods-hint = Miten haluat saada rahasi takaisin. Salattu tilisi avaimella.
payment-method-kind = Tapa
payment-method-kind-other = Muu
payment-method-label = Nimi
payment-method-label-placeholder = Päätili
payment-method-value = Tiedot
payment-method-value-placeholder = IBAN, puhelinnumero, käyttäjätunnus…
payment-method-add = Lisää
payment-method-remove = Poista { $name }
payment-method-empty = Et ole vielä lisännyt maksutietoja.
payment-method-deleted = Maksutapa poistettu.
payment-method-value-required = Täytä jokaisen maksutavan tiedot tai poista se.
payment-method-label-required = Anna omalle maksutavallesi nimi.
payment-method-too-long = Liian pitkä - lyhennä.
payment-method-invalid-characters = Poista rivinvaihdot tai näkymättömät merkit.
payment-method-limit = Voit tallentaa enintään { $max } maksutapaa.
payment-methods-saved = Maksutiedot tallennettu.
payment-methods-offline = Maksutietojen tallentaminen vaatii verkkoyhteyden.
payment-methods-stale = Maksutietojasi muutettiin toisella laitteella. Ne ladattiin uudelleen - yritä uudelleen.
payment-methods-key-missing = Kirjaudu uudelleen hallitaksesi maksutietojasi.
settings-payment-methods-share-warning = Jaettu maksutapa näkyy kaikille projektiesi jäsenille.
payment-method-share = Jaa projekteilleni
payment-method-share-hint = Näytetään nimesi vieressä, kun joku on sinulle velkaa.
payment-method-copy = Kopioi { $name }
payment-method-copied = Kopioitu.
payment-method-copy-failed = Kopiointi ei onnistunut - valitse teksti ja kopioi se käsin.

verify-email-checking = Vahvistetaan sähköpostiosoitettasi…
verify-email-welcome = Sähköposti vahvistettu - tervetuloa Countediin!
verify-email-back-to-login = Takaisin kirjautumiseen
verify-email-intro = Anna salasanasi vahvistaaksesi, että tämä osoite on sinun.
verify-email-submit = Vahvista osoitteeni

### Project status

project-archive = Arkistoi
project-sheet-invite = Kutsu
project-sheet-recurring = Toistuvat
project-sheet-edit = Muokkaa projektia
project-sheet-close = Sulje projekti
project-sheet-archive = Arkistoi projekti
project-sheet-reopen = Avaa projekti uudelleen
project-sheet-unarchive = Palauta projekti arkistosta
project-sheet-leave = Poistu projektista

### Dates

date-long = { $day }. { $month } { $year }

month-1 = tammikuuta
month-2 = helmikuuta
month-3 = maaliskuuta
month-4 = huhtikuuta
month-5 = toukokuuta
month-6 = kesäkuuta
month-7 = heinäkuuta
month-8 = elokuuta
month-9 = syyskuuta
month-10 = lokakuuta
month-11 = marraskuuta
month-12 = joulukuuta

month-short-1 = tammi
month-short-2 = helmi
month-short-3 = maalis
month-short-4 = huhti
month-short-5 = touko
month-short-6 = kesä
month-short-7 = heinä
month-short-8 = elo
month-short-9 = syys
month-short-10 = loka
month-short-11 = marras
month-short-12 = joulu

### Actions

add = Lisää
create = Luo
creating = Luodaan…
edit = Muokkaa
leave = Poistu
close = Sulje
paste = Liitä
join = Liity
import = Tuo
importing = Tuodaan…
field-description = Kuvaus
field-date = Päivämäärä
date-today = Tänään
date-yesterday = Eilen
field-optional = Valinnainen

### Projects

projects-filter-active = Aktiiviset
projects-filter-all = Kaikki
projects-count-label = Projektit
projects-empty = Ei projekteja
projects-offline-banner = Offline-tiedot - yhdistä uudelleen päivittääksesi.
demo-banner = Esittelyprojekti - vain luku.
demo-start-own = Aloita oma projekti
demo-badge = Demo
demo-bar = Muutoksesi katoavat, kun suljet tämän välilehden.
demo-leave = Aloita tosissaan
demo-preparing = Demoa valmistellaan…
demo-failed = Demoa ei voitu valmistella.
demo-sign-in-blocked = Kirjautuminen on poissa käytöstä demossa. Poistu demosta kirjautuaksesi tai luodaksesi tilin; omat projektisi pysyvät koskemattomina.
projects-no-local-data = Ei paikallisia tietoja
projects-no-local-data-hint = Kirjaudu sisään ladataksesi projektisi ensimmäistä kertaa.
projects-add = Lisää projekti
projects-create = Luo projekti
projects-personal-expense = Oma kulu
personal-expense-banner = Lisätty kohtaan Omat kulut
personal-create-title = Luo oma projektisi
personal-create-hint = Seuraa, mitä kulutat yksin
group-create-title = Luo jaettu projekti
group-create-hint = Jaa kulut ystävien ja perheen kanssa
personal-create-action = Luo
personal-create-signup = Luo tili
personal-sheet-title = Oma projektisi
personal-sheet-intro = Yksin kulutetuille menoille. Se näkyy ryhmiesi kanssa Tilastoissa.
personal-sheet-point-alone = Yksi jäsen: sinä, tilisi nimellä. Ei jakolinkkiä.
personal-sheet-point-encrypted = Salattu laitteillasi, kuten muut projektisi.
personal-sheet-point-stats = Osuutesi ryhmissä lisätään näihin kuluihin Tilastoissa.
personal-default-name = Omat kulut
my-spending-prev-month = Edellinen kuukausi
my-spending-next-month = Seuraava kuukausi
my-spending-spent = Kulutit
my-spending-groups = Ryhmät
my-spending-income = Tulot
my-spending-balance = Kuukauden saldo
my-spending-unconverted = Muun valuutan osuudet jätetään pois, kunnes kurssi tiedetään.
my-spending-empty = Ei mitään tässä kuussa
my-spending-empty-hint = Lisää kulu +-painikkeella. Osuutesi ryhmissä näkyvät täällä automaattisesti.
my-spending-share-of = / { $amount }
my-spending-uncounted = ei laskettu
payer-you = sinä
expense-received-by = vastaanotti
counted-switch = Laske mukaan: Omat kulut
counted-switch-on = Päällä: osuutesi lasketaan mukaan kohteeseen Omat kulut
counted-switch-off = Pois: osuutesi ei näy kohteessa Omat kulut
counted-groups = Lasketut ryhmät
counted-groups-intro = Osuutesi näissä ryhmissä lasketaan mukaan kohteeseen Omat kulut ja budjetteihin.
counted-groups-converted = { $from }, muunnettu valuuttaan { $to }
counted-groups-encrypted = Tämä valinta on salattu tililläsi ja seuraa sinua laitteillasi.
counted-groups-empty = Tällä laitteella ei ole vielä ryhmiä.
counted-groups-done = OK
counted-save-failed = Valintaa ei voitu tallentaa. Yritä uudelleen.
tab-budget = Budjetti
budget-title = Budjetit
budget-manage = Hallitse
budget-left = { $amount } jäljellä
budget-over = Ylitetty { $amount }
budget-empty = Ei budjettia
budget-empty-hint = Aseta katto kullekin luokalle. Osuutesi ryhmissä lasketaan mukaan.
budget-define = Aseta budjetti
budget-sheet-title = Kuukausibudjetit
budget-sheet-intro = Osuudellesi: omat ja ryhmät. Jätä tyhjäksi, jos et seuraa luokkaa.
budget-average = Keskiarvo 3 edellistä kk: { $amount }
budget-for = Budjetti { $category }
budget-alert = Varoita 80 %:ssa
budget-alert-hint = Lasketaan tällä laitteella
budget-invalid = Virheellinen summa: { $category }.
budget-offline = Budjetit voi tallentaa vain verkossa. Yhdistä uudelleen ja yritä uudestaan.
budget-alert-near = { $category }: { $percent } % kuukauden budjetista
budget-alert-over = { $category }: kuukauden budjetti ylitetty
my-spending-search-summary =
    { $count ->
        [one] { $count } tulos
       *[other] { $count } tulosta
    } · { $total } · omat ja ryhmät, kaikki kuukaudet
projects-join = Liity projektiin
projects-import-tricount = Tuo Tricountista
project-actions = Projektin toiminnot

status-ongoing = Käynnissä
status-closed = Suljettu
status-archived = Arkistoitu

nav-help = Ohje
nav-privacy = Tietosuojakäytäntö
nav-terms = Käyttöehdot
nav-legal = Oikeudelliset tiedot

leave-project-title = Poistutaanko projektista?
leave-project-message = Menetät pääsyn tältä laitteelta. Jos jäseniä ei jää, projekti ja kaikki sen kulut poistetaan pysyvästi.

add-project-title = Uusi projekti
add-project-name-label = Projektin nimi
add-project-name-placeholder = Matkani, Kimppakämppä 2024…
add-project-participants = Osallistujat
add-project-participant-name = Osallistujan nimi
add-project-participant-placeholder = Clark Kent
add-project-offline = Projektia ei voi luoda offline-tilassa. Yhdistä uudelleen ja yritä uudelleen.
add-project-name-required = Projekti tarvitsee nimen.

join-link-label = Jakolinkki
join-link-hint = Linkki sisältää salauksen purkuavaimen - kopioi se kokonaan.
join-invalid-link = Linkki ei kelpaa. Liitä koko jakolinkki, myös #-merkin jälkeinen osa.
join-wrong-project = Linkki kuuluu toiseen projektiin.

import-tricount-link-label = Tricount-linkki tai -avain
import-tricount-key-required = Anna Tricount-linkki tai -avain.
import-tricount-encryption-failed = Salaus epäonnistui.
import-tricount-unimportable = Mitään ei tuotu: tässä Tricountissa on jäseniä, joilla on Tricount-tili, tai summia, jotka eivät täsmää (koskee merkintöjä: { $count }).

### Expenses

save = Tallenna
saving = Tallennetaan…
adding = Lisätään…
link-copied = Linkki kopioitu
missing-encryption-key = Salausavain puuttuu.
missing-encryption-key-title = Salausavain puuttuu
missing-encryption-key-hint = Käyttämäsi linkki ei sisällä tämän projektin purkamiseen tarvittavaa avainta. Käytä projektin luojan jakamaa täydellistä linkkiä.
project-locked-hint = Tällä laitteella ei ole tämän projektin avainta. Avaa sen jakolinkki avataksesi lukituksen.
expenses-unreadable = Joitakin merkintöjä ei voi lukea tällä laitteella, eikä niitä lasketa summiin. Päivitä sovellus nähdäksesi ne.
project-unlock = Avaa lukitus
project-no-local-data-hint = Kirjaudu sisään ladataksesi tämän projektin tiedot ensimmäistä kertaa.
project-gone-title = Projektia ei enää ole
project-gone-hint = Se poistettiin, kun viimeinen jäsen poistui. Jakolinkki ei enää toimi, vaikka avaisit sen uudelleen.

expense-add = Lisää kulu
transfer-add = Lisää siirto
expense-edit-title = Muokkaa kulua
expense-category = Luokka
expense-category-auto = Auto · { $emoji }
emoji-picker-label = Emoji
emoji-none = Ei emojia
expense-currency = Summan valuutta
amount-op-add = Plus
amount-op-subtract = Miinus
amount-op-multiply = Kerro
amount-op-divide = Jaa
amount-op-equals = Yhtä kuin
amount-op-done = Valmis
expense-rate = Valuuttakurssi (valinnainen)
expense-rate-hint = Jätä tyhjäksi käyttääksesi Euroopan komission (InforEuro) kurssia kuukaudelle { $month }: 1 { $from } = { $rate } { $to }.
expense-rate-invalid = Anna valuuttakurssi, joka on suurempi kuin 0.
expense-rate-unavailable = Automaattista kurssia ei saatavilla - anna se käsin.
expense-delete-title = Poista kulu
expense-delete-message = ”{ $name }” poistetaan pysyvästi. Tätä ei voi perua.
expense-inconsistent-amounts = Summat eivät täsmää
expenses-empty = Ei kuluja
expenses-empty-hint = Aloita lisäämällä kuluja alla olevalla painikkeella
expenses-show-more = Näytä lisää ({ $count } jäljellä)

expense-type-expense = Kulu
expense-type-transfer = Siirto
expense-type-gain = Tulo
expense-type-group = Tyyppi
expense-paid-by = maksoi
expense-sent-by = lähetti
expense-contributed-by = osallistui

expense-name-required = Nimi vaaditaan.
expense-amount-not-positive = Summan on oltava suurempi kuin 0.
expense-no-payer = Valitse vähintään yksi maksaja.
expense-no-debtor = Valitse vähintään yksi velallinen.
expense-invalid-date = Päivämäärä ei kelpaa.
expense-payers-mismatch = Maksajien summa on { $sum }, mikä ei täsmää kulun summaan ({ $total }).
expense-debtors-mismatch = Velallisten summa on { $sum }, mikä ei täsmää kulun summaan ({ $total }).

participants-none = Ei kukaan
participants-everyone = Kaikki ({ $count })
participants-some = { $count } / { $total }
participants-select-all = Valitse kaikki
participants-by-shares = Osuuksittain
split-amounts = Summat
participants-remaining = { $amount } jäljellä
participants-over-by = { $amount } yli
participants-who-paid = Kuka maksoi?
participants-who-received = Kuka sai?
participants-who-transfers = Kuka siirtää?
participants-who-receives = Kuka vastaanottaa?
participants-for-whom = Kenelle?

stats-total-expenses = Kulut yhteensä
stats-my-expenses = Omat kuluni

tab-expenses = Kulut
tab-balance = Saldo
tab-reimbursements = Tasaus
balance-gets-back = Saa takaisin
balance-owes = Velkaa
balance-settled = Tasoissa
reimbursements-empty-title = Kaikki tasattu!
reimbursements-empty-hint = Tasausehdotukset näkyvät tässä, kun tilit eivät täsmää
reimbursement-owes = { $debtor } on velkaa { $creditor }
reimbursement-record = Tasaa
settlement-recorded = Tasaus kirjattu.
reimbursement-pay-with = Maksa
reimbursement-pay-shared-by = Jakanut { $name } - tarkista sovelluksesi näyttämä vastaanottajan nimi ennen lähettämistä.
reimbursement-pay-title = Maksa { $name }
reimbursements-mine-title = Olet velkaa
reimbursements-others-title = Muut takaisinmaksut
copy = Kopioi

user-selection-title = Kuka osallistujista olet?
user-selection-hint = Valitse nimesi listasta.
user-selection-required = Valitse osallistuja.
identity-claimed = Yhdistetty tiliin
identity-claimed-by = { $name } - tili
identity-taken-repick = Toinen tili on lunastanut käyttämäsi osallistujan. Valitse toinen.
participant-gone-repick = Käyttämäsi osallistuja on poistettu projektista. Valitse toinen.

edit-project-title = Muokkaa projektia
edit-project-new-badge = uusi
edit-project-deferred-new-members = uusien jäsenten lisäys
edit-project-deferred-removals = jäsenten poisto
edit-project-offline-deferred = Offline: { $items } otetaan käyttöön, kun yhteys palaa.

export-failed = Vienti epäonnistui: { $reason }

history-expense-added = Kulu lisätty: { $name }
history-expense-edited = Kulua muokattu: { $name }
history-expense-deleted = Kulu poistettu: { $name }
move-expense = Siirrä projektiin
move-expense-title = Siirrä kohteeseen…
move-current = Nykyinen
move-converted = { $currency }, muunnettu kuukauden kurssilla
move-note-personal = Valitset seuraavassa vaiheessa, kuka maksoi ja kenen puolesta. Kulu poistuu Omista kuluista.
move-note-group = Kulu poistuu projektista { $project } kaikilta jäseniltä, ja ryhmän saldo lasketaan uudelleen. Valitset seuraavassa vaiheessa, kuka maksoi ja kenen puolesta.
move-banner = { $from } → { $to }
move-submit = Siirrä
move-done = Kulu siirretty projektiin { $project }
history-expense-moved = Kulu siirretty toiseen projektiin: { $name }
personal-badge = Oma
move-members =
    { $count ->
        [one] { $count } jäsen
       *[other] { $count } jäsentä
    }
history-project-edited = Projektia muokattu: { $name }
history-name-changed = Nimi: ”{ $from }” → ”{ $to }”
history-description-added = Kuvaus lisätty: ”{ $value }”
history-description-removed = Kuvaus poistettu: ”{ $value }”
history-description-changed = Kuvaus: ”{ $from }” → ”{ $to }”

### Sweep

field-amount = Summa
expense-name-placeholder = Ravintola, ruokaostokset…
expense-actions = Kulun toiminnot
expense-your-share = Osuutesi
expense-your-share-value = Osuutesi: { $amount } { $currency }
expense-share-percent = { $percent } %
expense-added-by = Lisännyt { $name }, { $date }
expense-inconsistent-detail = Summat eivät täsmää: maksettu { $paid }, velkaa { $owed }, kulun summa { $total }. Korjaa muokkaamalla kulua.
missing-access-key = Pääsyavain puuttuu. Avaa projekti sen jakolinkin kautta.
filter-all = Kaikki
filter-my-payments = Omat maksuni
filter-my-debts = Omat velkani
expense-filter-group = Suodatin
expenses-search = Hae kuluja
expenses-no-match = Yksikään kulu ei vastaa.
expenses-search-open = Hae
expenses-search-clear = Tyhjennä haku
expenses-search-summary =
    { $count ->
        [one] { $count } tulos
       *[other] { $count } tulosta
    } · { $total } · kaikki kuukaudet
participants-shares-for = Osuudet: { $name }
participants-amount-for = Summa: { $name }
reimbursement-add = Lisää tasaus
project-forget = Poista listaltani
project-history-title = Historia
history-kind-add = Lisätty
history-kind-delete = Poistettu
history-kind-edit = Muokattu
export = Vie
export-json = Vie JSON
export-csv = Vie CSV
share-link = Jaa
copy-link-failed = Linkin kopiointi epäonnistui
open-in-app = Avaa sovelluksessa
not-found-title = Sivua ei löydy
not-found-back = Takaisin projekteihin

### Charts

charts-period = Ajanjakso
period-all = Kaikki
period-month = Kuukausi
period-3months = 3 kk
period-year = Vuosi
period-custom = Oma
charts-tab-categories = Luokat
charts-tab-trends = Trendit
charts-total-spent = Käytetty yhteensä
charts-avg-per-person = Keskim. / hlö
charts-expense-count =
    { $count ->
        [one] { $count } kulu
       *[other] { $count } kulua
    }
charts-nothing-to-show = Ei näytettävää
charts-my-share-skipped =
    { $count ->
        [one] 1 projektia ei lasketa - osallistujaa ei valittu tai sen tiedot eivät latautuneet.
       *[other] { $count } projektia ei lasketa - osallistujaa ei valittu tai niiden tiedot eivät latautuneet.
    }

### Categories

category-food = Ruoka
category-transport = Liikenne
category-accommodation = Majoitus
category-leisure = Vapaa-aika
category-shopping = Ostokset
category-services = Palvelut
category-parties-gifts = Juhlat ja lahjat
category-other = Muu
categories-manage = Hallitse luokkia
categories-title = Luokat
categories-yours = Omat luokkasi
categories-builtin = Vakioluokat
categories-add = Lisää luokka
categories-in-parent = luokassa { $parent }
categories-renamed-from = nimetty uudelleen · alun perin { $name }
categories-hidden-hint = piilotettu · kulut näkyvät luokassa { $other }
categories-always-shown = näkyy aina
categories-hidden = Piilotettu
categories-new = Uusi luokka
categories-edit = Muokkaa luokkaa
categories-parent = Lasketaan luokkaan
categories-parent-hint = Kaikkien projektiesi tilastoissa { $name } lasketaan luokkaan { $parent }.
categories-create = Luo
categories-hide = Piilota tämä luokka
categories-hide-hint = Sen kulut näkyvät kaikille jäsenille luokassa { $other }.
categories-delete-title = Poistetaanko ”{ $name }”?
categories-delete-body = Siihen kuuluvat kulut ({ $count }) palaavat luokkaan { $parent }. Yhtään kulua ei poisteta.
categories-delete-body-unknown = Sen kulut palaavat luokkaan { $parent }. Yhtään kulua ei poisteta.
categories-delete-warning = Luokka katoaa kaikilta projektin jäseniltä.
categories-summary-created = Luotu: { $count }
categories-summary-renamed = Nimetty uudelleen: { $count }
categories-summary-hidden = Piilotettu: { $count }
categories-summary-default = Ei omia luokkia
categories-offline = Luokkia voi hallita vain verkossa. Yhdistä uudelleen ja yritä uudestaan.
categories-error-too-many = Projektissa voi olla enintään 20 omaa luokkaa.
categories-error-full = Projekti on saavuttanut luokkien enimmäismäärän.
categories-error-name = Anna luokalle nimi.
categories-error-missing = Luokkaa on muutettu sillä välin. Sulje ja yritä uudelleen.
history-category-created = Luokka luotu: { $name }
history-category-edited = Luokkaa muokattu: { $name }
history-category-deleted = Luokka poistettu: { $name }
charts-project = Projekti
charts-all-projects = Kaikki projektit
charts-date-from = Alkaen
charts-date-to = Päättyen
charts-total = Yhteensä
charts-tab-people = Henkilöt
charts-tab-projects = Projektit
charts-scope = Kenen kulut
charts-scope-group = Ryhmä
charts-scope-me = Minä
charts-currency = Valuutta
charts-my-share = Osuuteni
charts-share-of-total = { $pct } % summasta { $total }
charts-i-paid = Maksoin
charts-paid-more = { $amount } enemmän kuin osuuteni
charts-paid-less = { $amount } vähemmän kuin osuuteni
charts-paid-even = Täsmälleen osuuteni
charts-part-title = Osuutesi kustakin luokasta
charts-part-desc = Harmaa on ryhmän kulutus, väri sinun osuutesi.
charts-breakdown-title = Jakauma luokittain
charts-breakdown-desc = Napauta siivua tai riviä nähdäksesi kulut.
charts-of-total = { $amount } / { $total }
charts-show-all = Näytä kaikki ({ $count })
charts-show-less = Näytä vähemmän
charts-spend-title = Kulut ajan mittaan
charts-group-by = Ryhmittele
bucket-day = Päivä
bucket-week = Viikko
bucket-month = Kuukausi
charts-avg = ka.
charts-cat-title-day = { $category }, päivä päivältä
charts-cat-title-week = { $category }, viikko viikolta
charts-cat-title-month = { $category }, kuukausi kuukaudelta
charts-cat-desc = Valitse luokka seurataksesi sitä ajan mittaan.
charts-running-title = Kertymä
charts-running-desc = Alkaen { $date }.
charts-avg-per-day = { $amount } / päivä keskimäärin
charts-avg-per-week = { $amount } / viikko keskimäärin
charts-avg-per-month = { $amount } / kuukausi keskimäärin
charts-people-title = Kuka kantoi ryhmää
charts-people-desc = Mitä kukin maksoi, rinnalla mitä hän kulutti.
charts-paid = Maksettu
charts-fair-share = Reilu osuus
charts-you = (sinä)
charts-net-more = maksoi enemmän
charts-net-less = maksoi vähemmän
charts-balance-title = Saldosi ajan mittaan
charts-balance-desc = Viivan yläpuolella ryhmä on sinulle velkaa. Alapuolella sinä olet velkaa ryhmälle.
charts-owed = Saat
charts-owe = Olet velkaa
charts-projects-title = Osuutesi projekteittain
charts-projects-desc = Summat pidetään valuutoittain.
history-empty = Ei tapahtumia
history-by = { $name }
not-found-hint = Sivua ei ole olemassa, tai se on siirretty.
payers-title-paid-by = Maksaja
payers-title-sender = Lähettäjä
payers-title-contributors = Osallistujat
debtors-title-debtors = Velalliset
debtors-title-recipients = Vastaanottajat
debtors-title-beneficiaries = Edunsaajat
debtors-title-for = Kenelle
debtors-title-recipient = Vastaanottaja
expense-split-between = jaettu { $count } kesken
expense-people =
    { $count ->
        [one] { $count } henkilö
       *[other] { $count } henkilöä
    }

### Welcome

welcome-title = Kirjanpitosi ei kuulu kenellekään muulle.
welcome-subtitle = Jaa kulut kavereiden kesken.
welcome-note = Ilmainen. Tiliä ei tarvita. Ei mainoksia.
welcome-link-title = Yksi linkki, ja kaikki ovat mukana.
welcome-link-body = Kenenkään ei tarvitse luoda tiliä.
welcome-link-account = Tili? Ei koskaan pakollinen. Sillä löydät projektisi toiselta laitteelta, kutsut kavereita sovelluksesta ja jaat maksutietosi.
welcome-demo-project = Viikonloppu Lyonissa
welcome-private-title = Kukaan ei voi lukea tilejäsi. Emme edes me.
welcome-private-body = Nimet, summat, projektit: kaikki salataan laitteellasi. Vain sinulla on avain.
welcome-private-names = Nimet
welcome-private-amounts = Summat
welcome-private-projects = Projektit
welcome-scan-title = Ota kuitista kuva.
welcome-scan-body = Summa, päivämäärä ja luokka täyttyvät itsestään. Kaikki tapahtuu puhelimessasi. Kuvaa ei säilytetä.
welcome-eu-title = 100 % eurooppalainen
welcome-no-ads = Ei mainoksia
welcome-no-trackers = Ei seurantaa
welcome-step = Vaihe { $current }/{ $total }
welcome-next = Seuraava
welcome-skip = Ohita
welcome-start = Aloita
welcome-how-it-works = Miten se tarkalleen toimii?

### Help

help-intro = Yleinen kysymys? Napauta avataksesi vastauksen.
help-create-project-q = Miten luon projektin?
help-create-project-a = Napauta aloitusnäytön alareunan +-painiketta. Anna projektille nimi, valitse valuutta, ja valmista.
help-add-participants-q = Miten lisään osallistujia?
help-add-participants-a = Avaa projekti ja lisää osallistujat jäsenlistasta. Jokainen osallistuja voi maksaa kulun tai olla siitä velkaa.
help-share-project-q = Miten jaan projektin?
help-share-project-a = Jaa projektin URL-osoite (osoitepalkissa oleva). Kuka tahansa linkin saanut voi katsella ja muokata projektia.
help-add-expense-q = Miten lisään kulun?
help-add-expense-a = Napauta projektissa +, anna summa, kerro kuka maksoi ja keiden kesken jaetaan. Voit myös valita muun päivämäärän kuin tämän päivän.
help-types-q = Mitä eroa on kululla, siirrolla ja tulolla?
help-types-expense = - yhden henkilön tekemä osto, joka jaetaan usean kesken.
help-types-transfer = - takaisinmaksu henkilöltä toiselle, ilman jakoa.
help-types-gain = - saatu raha (hyvitys, lahja), joka jaetaan usean henkilön kesken.
help-past-date-q = Voinko päivätä kulun menneisyyteen?
help-past-date-a = Kyllä, päivämääräkenttä on vapaa. Kirjauksen luontiaika tallennetaan erikseen.
help-who-owes-q = Miten Counted laskee, kuka on velkaa ja kenelle?
help-who-owes-a = Counted laskee jokaisen osallistujan nettosaldon (mitä hän maksoi miinus mitä hän on velkaa) ja ehdottaa sitten lyhimmän siirtosarjan, joka tasaa kaikki.
help-minimal-transfers-q = Miksi ehdotettujen siirtojen määrä on minimaalinen?
help-minimal-transfers-a = Algoritmi yhdistää ensin saldot, jotka kumoavat toisensa tarkalleen, ja käy sitten loput läpi suurimmasta velkojasta suurimpaan velalliseen. Tulos: vähemmän siirtoja kaiken tasaamiseen.
help-import-tricount-q = Miten tuon projektin Tricountista?
help-import-tricount-a = Napauta aloitusnäytön alareunan ”+”-painiketta ja sitten
help-import-tricount-b = Liitä tuotavan Tricountin jakolinkki.
help-encryption-q = Onko tietoni salattu?
help-encryption-a = Kyllä. Counted yhdistää kaksi takuuta:
help-encryption-e2ee-term = Päästä päähän -salaus
help-encryption-e2ee-def = - kaikki sinun ja palvelimen välillä kulkee salattuna.
help-encryption-zero-term = Nollapääsy
help-encryption-zero-def = - salaat tiedot ennen lähettämistä, ja palvelin tallentaa vain salattua tekstiä. Meillä ei ole keinoa lukea sitä.
help-encryption-see = Lisätietoja:
help-forgot-password-q = Mitä tapahtuu, jos unohdan salasanani?
help-forgot-password-warning = Kukaan ei voi palauttaa sitä puolestasi.
help-forgot-password-a = Salasanasi suojaa salausavaimiasi, eikä sitä koskaan lähetetä meille: kukaan - emme mekään - ei voi purkaa projektiesi salausta puolestasi. Säilytä se turvassa, mieluiten salasanojen hallintaohjelmassa.
help-archive-delete-q = Miten arkistoin tai poistan projektin?
help-archive-delete-a = Avaa projektin näytöllä valikko ja valitse
help-archive-delete-b = piilottaaksesi sen säilyttäen. Projekti poistetaan lopullisesti, kun sen viimeinen jäsen poistuu.
help-delete-account-q = Miten poistan tilini?
help-delete-account-a = Avaa Asetukset ja käytä ”Poista tilini”. Se tapahtuu heti, eikä sitä voi perua.
help-contact = Muuta kysyttävää? Kirjoita meille osoitteeseen

# Receipt scanning (mobile only)
expense-scan = Skannaa kuitti
scan-in-progress = Luetaan kuittia…
scan-error-capture = Kuvan ottaminen ei onnistunut. Yritä uudelleen tai syötä kulu käsin.
scan-error-unreadable = Kuitista ei löytynyt mitään luettavaa. Syötä kulu käsin.
scan-check-amount = Tarkista loppusumma - se ei ollut selkeästi painettu.
scan-take-photo = Ota kuva
scan-choose-photo = Valitse kuva
expense-converted-from = Maksettu { $amount } { $from } · 1 { $from } = { $rate } { $to }
project-currency = Valuutta
project-currency-locked = Valuutta lukitaan projektia luotaessa.
currency-search = Hae valuuttaa

update-required-title = Päivitys vaaditaan
update-required-body = Tämä Counted-versio on liian vanha keskustellakseen palvelimen kanssa. Päivitä se jatkaaksesi sovelluksen käyttöä.
update-required-button = Päivitä

notifications-label = Ilmoitukset
notifications-title = Ilmoitukset
notifications-empty = Ei uutta
notifications-friend-request = Kaveripyyntö

friends-title = Kaverit
friends-anonymous-body = Kaverit säilytetään tililläsi. Kirjaudu sisään lisätäksesi henkilöitä ja kutsuaksesi heidät projekteihisi jakamatta linkkiä.
friends-add-title = Lisää kaveri
friends-add-hint = Jos hänellä on tili, hän näkee pyyntösi sovelluksessa. Muuten kerro hänelle: hän löytää sen rekisteröityessään tällä osoitteella.
friends-add-button = Lisää
friends-add-from-project = Lisää kaveriksi
friends-request-sent = Pyyntö lähetetty
friends-no-account-key = Kirjaudu uudelleen hallitaksesi kavereitasi tällä laitteella.
friends-incoming-title = Pyynnöt
friends-accept = Hyväksy
friends-decline = Hylkää
friends-list-title = Kaverini
friends-list-empty = Ei vielä kavereita. Lisää joku sähköpostilla yllä tai yhteisestä projektista.
friends-remove = Poista
friends-remove-confirm-title = Poista kaveri
friends-remove-confirm-message = { $email } ei ole enää kavereissasi, etkä sinä hänen. Kumpi tahansa voi lähettää uuden pyynnön myöhemmin.
friends-no-key = Ei vielä valmis
friends-key-changed = Koodi muuttunut
friends-key-changed-hint = Ystävän turvakoodi vaihtui. Niin käy, kun hän suojaa tilinsä, tai jos hänen avaimensa on vaihdettu. Kutsut hänen kanssaan on estetty, kunnes luette uuden koodin toisillenne kasvokkain tai puhelimessa ja napautat ”Koodi täsmää”.
friends-fingerprint = Turvakoodi
friends-fingerprint-hint = Kaksi kaveria, jotka lukevat toisilleen saman turvakoodin, tietävät, ettei kukaan ole heidän välissään - ei edes palvelimemme.
friends-key-mismatch = Palvelin lähetti tilillesi julkisen avaimen, joka ei vastaa yksityistä avaintasi. Turvakoodisi on piilotettu ja kutsut ovat pois päältä tällä laitteella: joku saattaa peukaloida palvelinta.
keypair-mismatch = Palvelimen tilillesi lähettämät avaimet eivät vastaa tämän laitteen avaimia. Kutsut ja avaimen vaihdot ovat täällä pois käytöstä, kunnes ne vastaavat: joku saattaa peukaloida palvelinta.
friends-outgoing-title = Lähetetyt
friends-outgoing-hint = Odottaa vastausta. Näet heidät kavereissasi, kun he hyväksyvät.
friends-withdraw = Peruuta
friends-notify = Kerro viestillä
friends-notify-label = Kerro henkilölle { $email }
friends-notify-message = Siirryn Countediin yhteisiä kulujamme varten. Ilmainen rekisteröityminen osoitteessa https://counted.fr sähköpostiosoitteella { $email }: kaveripyyntöni odottaa siellä.
friends-notify-copied = Viesti kopioitu
friends-notify-copy-failed = Viestin kopiointi epäonnistui
guests-hint-title =
    { $count ->
        [one] { $count } osallistuja ei ole vielä liittynyt projektiin
       *[other] { $count } osallistujaa ei ole vielä liittynyt projektiin
    }
guests-hint-later = Myöhemmin
guests-hint-send = Kutsu heidät
guests-send-title = Lähetä projekti osallistujille
guests-send-friend = Kutsu Counted-kaveri
guests-send-friend-badge = Ilman linkkiä
guests-send-friend-hint = Jos hänellä on jo tili, hän saa salatun kutsun; mitään ei kulje viestisovelluksesi kautta.
guests-send-link = Jaa projektin linkki
guests-send-link-hint = Niille, joilla ei ole tiliä. Projekti aukeaa yhdellä napautuksella.
guests-send-link-warning = Kuka tahansa, jolla on tämä linkki, voi lukea ja muokata projektia. Myös viestisovelluksesi näkee sen.
invite-friends-title = Kutsu kavereita
invite-friends-hint = Projektin avain salataan jokaiselle kaverille tällä laitteella. Palvelin ei koskaan näe sitä.
invite-friends-empty = Ei vielä kutsuttavia kavereita.
invite-friends-button = Kutsu
invite-sent = { $count ->
    [one] Kutsu lähetetty
   *[other] { $count } kutsua lähetetty
}
invitation-badge = Kutsu
invitation-to = Liity projektiin ”{ $name }”
invitation-to-unnamed = Liity projektiin
invitation-unreadable = Tätä kutsua ei voi avata tällä laitteella
invitation-from = Lähettäjä { $email }
invitation-accept = Liity
invitation-decline = Hylkää

# Participants in the create and edit modals, and the "who are you?" picker - see
# docs/plans/friends.md §11.
participants-you-label = Nimesi tässä projektissa
participants-you-badge = Sinä
participants-others = Muut osallistujat
participants-removed = Poistetut
participant-restore = Palauta
participants-empty = Lisää osallistujat alla.
participants-duplicate = ”{ $name }” on jo listalla.
participants-input-label = Lisää kaveri tai kirjoita nimi
participants-input-placeholder = Kaveri tai mikä tahansa nimi
participants-suggest-friend = Kaveri · liittyy nimellä ”{ $name }”, saa kutsun
participants-suggest-not-ready = Kaveri · ei vielä valmis
participants-suggest-guest = Lisää ”{ $text }”
participants-suggest-guest-sub = Vieras
participants-friends = Kaverisi
participants-all-friends = Kaikki kaverit
participants-login-hint = Kirjaudu sisään lisätäksesi ihmisiä suoraan kaverilistaltasi.
participants-invite-badge = Kutsu
participants-guest-badge = Vieras
participants-rename = Nimeä { $name } uudelleen
participants-remove = Poista { $name }
participants-rename-label = Uusi nimi
participants-rename-save = Tallenna nimi
participants-rename-hint = Nimi, jonka kaikki näkevät tässä projektissa. Kutsu menee edelleen osoitteeseen { $email }.
participants-invited-badge = Kutsuttu
participants-invited-sub = { $email } · ei vielä hyväksytty
participants-invited-pending = Kutsua ei ole vielä hyväksytty
participants-unlinked = Ei yhdistetty tiliin
add-project-create-invite = Luo ja kutsu { $count }
edit-project-save-invite = Tallenna ja kutsu { $count }
edit-project-you-are = Tässä projektissa olet { $name }
edit-project-no-identity = Et ole vielä valinnut, kuka olet
edit-project-switch = Vaihda
edit-project-choose = Valitse
invite-failed = Näitä kutsuja ei voitu lähettää: { $emails }
invite-waiting = Näiden ystävien täytyy avata Counted kerran, ennen kuin heidät voi kutsua: { $emails }
key-offer-title = Eri avain projektille { $name }
key-offer-body = Tällä laitteella on tähän projektiin toinen avain, joka ei ole koskaan avannut sitä täällä.
key-offer-choices = Käytä tätä avainta: projekti avautuu tämän linkin tai kutsun avaimella. Peruuta: laite pitää nykyisen avaimensa, eikä mikään muutu.
key-offer-use = Käytä tätä avainta
invite-again = Kutsu uudelleen
friend-picker-title = Lisää kavereita
user-selection-invited-hint = { $email } kutsui sinut projektiin ”{ $project }”.
user-selection-suggested = Ehdotettu
user-selection-suggested-sub = { $email } lisäsi sinut tällä nimellä
user-selection-confirm-as = Olen { $name }
user-selection-missing = Eikö nimeäsi ole tässä? Pyydä osallistujaa lisäämään sinut projektin asetuksissa.

## Toistuvat kulut

repeat-label = Toista
repeat-none = Ei toistu
repeat-weekly = Joka viikko
repeat-biweekly = Joka toinen viikko
repeat-monthly = Joka kuukausi
repeat-quarterly = 3 kuukauden välein
repeat-yearly = Joka vuosi
repeat-every-weeks = { $count } viikon välein
repeat-every-months = { $count } kuukauden välein
repeat-every-years = { $count } vuoden välein
repeat-custom = Mukautettu…
repeat-every = Välein:
repeat-unit-weeks = viikkoa
repeat-unit-months = kuukautta
repeat-unit-years = vuotta
repeat-on-weekday = päivänä { $weekday }
repeat-on-day = kuun { $day }. päivä
repeat-on-day-month = { $day }. { $month }
repeat-month-end = Lyhyempinä kuukausina se osuu kuun viimeiseen päivään.
repeat-ends = Päättyy
repeat-ends-never = Ei koskaan
repeat-ends-on = Päivämääränä
repeat-ends-after = Kertojen jälkeen
repeat-fewer = Vähemmän
repeat-more = Enemmän
repeat-last-on = viimeinen { $date }
repeat-variable = Summa vaihtelee joka kerta
repeat-variable-hint = Jokainen lisätään viimeisimmällä summalla ja merkitään ”vahvistettavaksi”.
repeat-done = Valmis
repeat-no-end = Ei päättymistä
repeat-until = { $date } asti
repeat-occurrences = Kertoja: { $count }
repeat-offline = Vaatii yhteyden. Itse kulun voi silti lisätä.
repeat-foreign = Toistuu summalla { $amount } { $currency }, muunnettuna kerran tämän päivän kurssilla. Saat varoituksen, jos kurssi muuttuu yli 5 %.
repeat-backfill = Alkaa menneisyydestä. Nyt lisättävät kulut: { $count }.
add-and-repeat = Lisää ja toista
weekday-1 = maanantai
weekday-2 = tiistai
weekday-3 = keskiviikko
weekday-4 = torstai
weekday-5 = perjantai
weekday-6 = lauantai
weekday-7 = sunnuntai
recurring-title = Toistuvat kulut
recurring-strip = Toistuvat kulut: { $count }
recurring-next = Seuraava: { $name }, { $date }
recurring-to-confirm = Vahvistettavia: { $count }
recurring-per-month = Kuukaudessa, noin
recurring-your-share = Sinun osuutesi
recurring-active = Aktiiviset
recurring-paused = Keskeytetyt
recurring-finished = Päättyneet
recurring-paid-by = maksaja { $name }
recurring-next-on = Seuraava { $date }
recurring-progress = { $done }/{ $total }
recurring-rate-badge = Kurssi muuttunut { $percent } %
recurring-empty = Mikään ei vielä toistu. Valitse ”Toista”, kun lisäät kulun: vuokra, tilaukset, laskut.
recurring-next-ones = Tulevat
recurring-added-so-far = Lisätty tähän mennessä
recurring-set-up-by = Luonut
recurring-pause = Keskeytä
recurring-resume = Jatka
recurring-stop = Lopeta toisto
recurring-stop-title = Lopetetaanko ”{ $name }”?
recurring-stop-message = Se ei enää toistu. Jo lisätyt kulut säilyvät.
recurring-resume-title = Jatketaanko ”{ $name }”?
recurring-resume-message = Seuraava { $date }. Tauon aikana väliin jääneitä päiviä ei lisätä.
recurring-edit-title = Muokkaa toistuvaa kulua
recurring-edit-banner = Muutokset koskevat { $date } alkaen. Jo lisätyt kulut pysyvät ennallaan.
recurring-next-on-label = Seuraava
recurring-next-too-early = Seuraavan päivän on oltava viimeksi lisätyn kulun jälkeen.
recurring-use-stop = Lopeta se toistuvan kulun kohdasta ”Lopeta toisto”.
recurring-drift = Valuutan { $currency } kurssi on muuttunut { $percent } % luomisen jälkeen. Jokainen lisätään yhä summalla { $amount } { $project_currency } (1 { $currency } = { $rate }). Tämän päivän kurssilla se olisi { $today_amount } { $project_currency }.
recurring-use-rate = Käytä tämän päivän kurssia
recurring-keep = Pidä { $amount } { $currency }
recurring-added = Lisätyt toistuvat kulut: { $count }
recurring-blocks-removal = Osallistujaa { $name } ei voi vielä poistaa: mukana kuluissa { $rules }. Poista { $name } niistä tai lopeta ne ja tallenna uudelleen.
history-recurring-added = Lisätty automaattisesti: { $name } ({ $date })
history-recurring-created = Toistuva kulu luotu: { $name }
history-recurring-edited = Toistuvaa kulua muokattu: { $name }
history-recurring-paused = Toistuva kulu keskeytetty: { $name }
history-recurring-resumed = Toistuvaa kulua jatkettu: { $name }
history-recurring-stopped = Toistuva kulu lopetettu: { $name }
occurrence-recurring = Toistuva kulu
occurrence-auto = Lisätty automaattisesti toistuvasta kulusta.
occurrence-auto-next = Lisätty automaattisesti toistuvasta kulusta. Seuraava { $date }.
occurrence-auto-stopped = Lisätty automaattisesti toistuvasta kulusta, joka on sittemmin lopetettu.
occurrence-manage = Hallitse
estimate-badge = Vahvistettava
expense-pending = Odottaa
estimate-title = Summa vahvistettava.
estimate-body = Lisätty edellisellä summalla. Syötä oikea summa, kun se selviää.
estimate-confirm = Vahvista summa
apply-to = Koskee
apply-this-only = Vain tätä kulua
apply-and-next = Tätä ja seuraavia
apply-and-next-hint = Toistuva kulu muuttuu { $date } alkaen
apply-rule-failed = Kulu tallennettiin, mutta toistuvaa kulua ei muutettu.
occurrence-delete-message = ”{ $name }” ({ $date }) poistetaan pysyvästi. Toisto jatkuu, eikä tämä päivä palaa.
occurrence-delete-one = Poista vain tämä
occurrence-delete-stop = Poista ja lopeta toisto
error-recurring-clock = Tämän laitteen kello on edellä. Tarkista päivämäärä ja aika.
error-recurring-not-found = Tätä toistuvaa kulua ei ole enää olemassa.
error-recurring-stale = Joku muutti tätä toistuvaa kulua sillä välin. Se on ladattu uudelleen: tarkista se ja tallenna uudelleen.
error-too-many-recurring = Projektissa on jo 50 toistuvaa kulua. Lopeta jokin tarpeeton lisätäksesi uuden.
error-user-in-recurring = Osallistuja on mukana toistuvassa kulussa. Poista osallistuja siitä tai lopeta kulu ensin.

home-owed-to-you = Sinulle velkaa
home-you-owe = Olet velkaa
home-details-show = Näytä erittely
home-details-hide = Piilota erittely
home-all-settled = Kaikki on selvitetty
home-pay = Maksa
home-remind = Muistuta
home-remind-message = Pieni muistutus: projektissa { $project } on vielä maksamattomia korvauksia.
home-row-owed = sinulle velkaa
home-row-owe = olet velkaa
home-row-settled = Selvitetty
home-row-quits = Olet sujut
home-row-pending =
    { $count ->
        [one] { $count } takaisinmaksu avoinna
       *[other] { $count } takaisinmaksua avoinna
    }
home-totals-partial = Ilman projekteja, joilla ei ole vaihtokurssia
error-account-key-changed = Tilisi avain vaihtui toisella laitteella. Kirjaudu uudelleen jatkaaksesi.
settings-security = Turvallisuus
settings-change-password = Vaihda salasana
settings-change-password-hint = Projektisi ja tietosi pysyvät ennallaan. Muut laitteesi kirjataan ulos.
change-password-title = Vaihda salasanasi
change-password-current = Nykyinen salasana
change-password-new = Uusi salasana
change-password-confirm = Vahvista uusi salasana
change-password-mismatch = Uudet salasanat eivät täsmää.
change-password-submit = Vaihda salasana
change-password-submitting = Vaihdetaan…
change-password-done = Salasana vaihdettu. Muut laitteesi kirjattiin ulos.
error-rotation-incomplete = Tilisi muuttui tämän aikana. Yritä uudelleen.
error-key-lifecycle-unavailable = Ei vielä käytettävissä: kaikkien täytyy ensin päivittää sovellus uusimpaan versioon.
settings-compromised = Joku saattaa tietää salasanani
settings-compromised-hint = Uusi salasana, uudet avaimet ja kaikki muut laitteet kirjataan ulos.
settings-compromised-open = Suojaa tilini
compromised-title = Suojaa tilisi
compromised-body = Salasanasi ja tilisi avaimet vaihdetaan, ja kaikki muut laitteet kirjataan ulos. Ystäväsi näkevät turvakoodisi vaihtuvan: vertailkaa sitä uudelleen.
compromised-keeps = Salasanasi tiennyt säilyttää sen, minkä on jo avannut: projektit, joita on tähän asti voinut lukea.
compromised-submit = Vaihda salasana ja avaimet
compromised-done = Tilisi on suojattu. Pyydä ystäviäsi vertaamaan uutta turvakoodiasi.
friends-new-fingerprint = Uusi koodi:
friends-code-matches = Koodi täsmää
friends-code-trusted = Koodi vahvistettu.
settings-recovery-key = Palautusavain
settings-recovery-key-none = Ilman sitä unohtunut salasana vie asetuksesi, maksutietosi ja ystävien koodit, ja palautus kestää 3 päivää.
settings-recovery-key-set = Asetettu. Jos unohdat salasanasi, saat sillä tilisi takaisin kaikkineen.
settings-recovery-key-pending = Uusi avain korvaa sen { $when }. Etkö se ollut sinä? Peru se nykyisellä avaimellasi.
recovery-key-create = Luo
recovery-key-replace = Korvaa
recovery-key-cancel-replacement = Peru uusi avain
recovery-key-title = Palautusavaimesi
recovery-key-body = Kirjoita se ylös tai tallenna paikkaan, johon vain sinä pääset. Counted ei voi näyttää sitä uudelleen eikä palauttaa sitä puolestasi.
recovery-key-copy = Kopioi
recovery-key-copied = Kopioitu
recovery-key-save = Tallenna tiedostona
recovery-key-print = Tulosta
recovery-key-next = Jatka
recovery-key-confirm-body = Tarkista kopiosi kirjoittamalla avaimen ryhmä { $n }.
recovery-key-confirm-wrong = Ryhmä ei täsmää. Katso kopiotasi uudelleen.
recovery-key-password-body = Tallenna avain tiliisi syöttämällä salasanasi.
recovery-key-submit = Tallenna avain
recovery-key-done = Palautusavain tallennettu.
recovery-key-replacement-done = Uusi avain tallennettu. Se korvaa nykyisen 72 tunnin kuluttua; siihen asti nykyinen toimii.
recovery-key-field = Palautusavain
recovery-key-file-intro = Countedin palautusavain
recovery-key-cancel-done = Uusi avain on peruttu.
compromised-new-key = Tallenna uusi palautusavaimesi nyt: se korvaa vanhan 72 tunnin kuluttua, ja siihen asti vanha toimii.
pending-reset-title = Tilisi nollausta on pyydetty
pending-reset-body = Se tulee voimaan { $when }: silloin asetuksesi, maksutietosi ja ystävien koodit menetetään, ja laitteidesi täytyy kirjautua uudelleen. Etkö se ollut sinä? Peru se nykyisellä salasanallasi.
pending-reset-cancel = Peru nollaus
pending-reset-cancelled = Nollaus on peruttu.
login-forgot = Unohtuiko salasana?
forgot-title = Unohtunut salasana
forgot-body = Syötä sähköpostiosoitteesi. Jos sillä on tili, lähetämme linkin uuden salasanan valintaan.
forgot-submit = Lähetä linkki
forgot-sent = Jos osoitteella on tili, linkki on matkalla. Se vanhenee 24 tunnissa.
reset-title = Palauta tilisi
reset-with-key-body = Syötä palautusavaimesi ja uusi salasana: kaikki palaa ennalleen.
reset-submit-key = Palauta tilini
reset-no-key = Minulla ei ole palautusavainta
reset-without-key-title = Nollaa ilman palautusavainta
reset-without-key-body = Tilisi alkaa alusta uudella salasanalla 72 tunnin kuluttua. Projektisi palaavat laitteistasi ja jakolinkeistä; asetuksesi, maksutietosi ja ystävien koodit eivät. Siihen asti nykyinen salasanasi toimii ja voi perua nollauksen.
reset-name = Nimesi
reset-submit-request = Nollaa tilini
reset-requested = Valmis. Uusi salasanasi toimii { $when } alkaen. Siihen asti nykyinen salasana toimii ja voi perua sen.
reset-pending-already = Nollaus odottaa jo { $when } asti. Uusi pyyntö aloittaa 72 tuntia alusta.
recovered-done = Tilisi on palautettu. Muut laitteesi kirjattiin ulos.
login-reset-undoable = Tämä salasana korvattiin nollauksella viimeisten 30 päivän aikana. Jos et pyytänyt sitä, peru se: tilisi palaa ennalleen.
login-undo-reset = Peru nollaus
reset-undone = Nollaus on peruttu. Tilisi on ennallaan.
error-reset-pending = Uusi salasana toimii, kun nollaus tulee voimaan, 72 tuntia pyynnön jälkeen.
error-account-disabled = Tili poistettiin pian nollauksen jälkeen ja odottaa pyyhkimistä. Aiempi salasana voi yhä perua nollauksen.
error-recovery-key-invalid = Tämä palautusavain ei avaa tätä tiliä.
error-project-key-required = Projekti tarvitsee avaimensa. Avaa se uudelleen jakolinkistä.
names-or = { $first } tai { $last }
participants-admin-badge = Ylläpitäjä
edit-project-admins = Ylläpitäjiä: { $count }
edit-project-succession = Jos viimeinen ylläpitäjä lähtee projektista, ylläpitäjäksi tulee pisimpään mukana ollut jäsen, jolla on tili.
member-sheet-account = Jäsen, jolla on tili
member-actions = Toiminnot: { $name }
member-make-admin = Tee ylläpitäjäksi
member-make-admin-hint = Voi sitten vaihtaa projektin avaimen ja poistaa jonkun.
member-made-admin = { $name } on nyt ylläpitäjä.
group-security = Projektin turvallisuus
group-security-admin = Projektin turvallisuus · olet ylläpitäjä
group-rotate = Vaihda projektin avain
group-rotate-hint = Jos projektin linkki on vuotanut: vanhan linkin haltijat menettävät pääsyn projektiin.
group-remove = Poista joku projektista
group-remove-hint = Poistaa pääsyn ja vaihtaa avaimen samalla kertaa.
group-admins-only = Vain ylläpitäjille.
group-admins-only-account = Vain ylläpitäjille, joilla on tili.
group-ask-admin = Kysy projektin ylläpitäjältä: { $names }.
group-ask-admins = Kysy projektin ylläpitäjiltä: { $names }.
group-ask-an-admin = Kysy joltakulta projektin ylläpitäjältä.
group-with-account = Tilin kanssa
group-free = Ilmainen
group-sign-in-hint = Projektin ylläpitäjä ({ $names }) voi sitten antaa sinulle roolin.
group-sign-in-hint-any = Projektin ylläpitäjä voi sitten antaa sinulle roolin.
group-no-admin = Projektilla ei ole ylläpitäjää, joten sen avainta ei voi vaihtaa.
group-hide = Piilota joku
group-hide-hint = Piilottaa hänet listoista poistamatta pääsyä: linkki ja avain toimivat yhä.
group-recreate-hint = Projektilla ei ole ylläpitäjää. Jos haluat poistaa jonkun, luo uusi projekti.
group-recreate = Luo uusi projekti
group-no-admin-yet = Projektilla ei ole vielä ylläpitäjää.
group-creator-hint = Vain sinä voit tulla tämän projektin ylläpitäjäksi luomalla tilin.
group-creator-elsewhere-hint = Vain projektin luonut henkilö voi tulla sen ylläpitäjäksi luomalla tilin. Muuten luo uusi projekti, jos haluat poistaa jonkun.
member-revoke-admin = Poista ylläpitäjän rooli
member-revoke-admin-hint = Ei voi enää vaihtaa projektin avainta eikä poistaa ketään.
member-revoked-admin = { $name } ei ole enää ylläpitäjä.
member-step-down = Lopeta ylläpitäjänä
member-step-down-hint = Et voi enää vaihtaa projektin avainta etkä poistaa ketään.
member-stepped-down = Et ole enää ylläpitäjä.
member-remove = Poista projektista
member-remove-hint = Vaihtaa projektin avaimen: { $name } ei voi enää lukea mitään uutta.
member-hide = Piilota listoista
error-user-not-admin = Tämä jäsen ei ole ylläpitäjä.
error-creator-proof-invalid = Tämä laite ei luonut tätä projektia.
error-project-has-admin = Projektilla on jo ylläpitäjä.
error-not-project-admin = Vain projektin ylläpitäjät voivat tehdä tämän.
error-user-not-claimed = Tällä osallistujalla ei ole tiliä, joten hän ei voi olla ylläpitäjä.
error-project-key-rotated = Projektin avain vaihtui. Pyydä uusi linkki joltakulta jäseneltä.
error-project-rotating = Projektin avainta vaihdetaan. Yritä hetken päästä uudelleen.
error-rotation-expired = Avaimen vaihto kesti liian kauan ja peruttiin. Aloita alusta.
error-rotation-rows-changed = Projekti muuttui avaimen vaihdon aikana. Aloita alusta.
project-rotated-title = Sinulla ei ole enää pääsyä tähän projektiin
project-rotated-body = Ylläpitäjä vaihtoi tämän projektin avaimen. Pyydä häneltä uusi linkki.
project-rotated-open-link = Avaa linkki
rotation-remove-title = Poistetaanko { $name } projektista?
rotation-rotate-title = Vaihdetaanko projektin avain?
rotation-remove-body = Jäsenen poistaminen tarkoittaa uuden avaimen luomista. Uusi linkki pitää jakaa niille, joilla ei ole tiliä.
rotation-rotate-body = Vanhan linkin haltijat menettävät pääsyn projektiin. Uusi linkki pitää jakaa niille, joilla ei ole tiliä.
rotation-boxed =
    { $count ->
        [0] Kukaan muu jäsen ei saa avainta automaattisesti
        [one] { $count } jäsen saa avaimen automaattisesti
       *[other] { $count } jäsentä saa avaimen automaattisesti
    }
rotation-by-link-title = Henkilöt, joille sinun pitää lähettää uusi linkki
rotation-by-link-hint = Näillä henkilöillä ei ole tiliä. Lähetät heille linkin heti tämän jälkeen.
rotation-names-and = { $first } ja { $last }
rotation-names-and-others =
    { $names } ja { $count ->
        [one] { $count } muu
       *[other] { $count } muuta
    }
rotation-confirm-remove = Poista ja vaihda avain
rotation-confirm-rotate = Vaihda avain
rotation-done-title = Uusi avain käytössä
rotation-done-removed = { $name } ei ole enää mukana projektissa.
rotation-done-rotated = Projektin avain vaihtui.
rotation-done-boxed =
    { $count ->
        [0] Kukaan muu jäsen ei saanut avainta automaattisesti.
        [one] { $count } jäsen sai avaimen automaattisesti.
       *[other] { $count } jäsentä sai avaimen automaattisesti.
    }
rotation-new-link =
    { $count ->
        [0] Projektin uusi linkki
        [one] Uusi linkki lähetettäväksi · { $count } henkilö ilman tiliä
       *[other] Uusi linkki lähetettäväksi · { $count } henkilöä ilman tiliä
    }
rotation-link-renewed = Linkki uusittiin.
rotation-done = Valmis
history-key-changed = Projektin avain vaihdettu
history-member-removed = { $name } ei ole enää mukana projektissa
project-rotated-row-hint = Avain vaihtui. Pyydä uusi linkki.
