# Suomi. Täydellinen lukuun ottamatta juridisia tekstejä (legal-, terms-, privacy-), jotka ovat vain
# englanniksi ja ranskaksi ja palautuvat viesteittäin en.ftl-tiedostoon.

### Common

loading = Ladataan…
cancel = Peruuta
confirm = Vahvista
retry = Yritä uudelleen
delete = Poista
back = Takaisin
language = Kieli

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
error-generic = Jokin meni pieleen. Yritä uudelleen.

error-invalid-email = Sähköpostiosoite ei kelpaa.
error-invalid-password = Salasana ei kelpaa.
error-password-too-short = Salasanassa on oltava vähintään 8 merkkiä.
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
login-resend = Lähetä vahvistuslinkki uudelleen
login-resending = Lähetetään…
login-resend-sent = Sähköposti lähetetty - tarkista postilaatikkosi.

register-submit = Luo tili
register-submitting = Luodaan…
register-have-account = Onko sinulla jo tili?
register-password-placeholder = Vähintään 8 merkkiä
register-password-warning = Kirjoita salasanasi muistiin. Jos unohdat sen, tiliäsi ei voi palauttaa.
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
account-delete-warning = Välitön ja pysyvä, ilman roskakoria. Jaettuun projektiin kirjaamasi kulut jäävät muiden jäsenten nähtäville - ne ovat osa heidän kirjanpitoaan.
account-delete-confirm-title = Poista tili
account-delete-confirm-message = Tilisi, istuntosi ja projektilistasi poistetaan pysyvästi. Ilman salasanaasi jaetun projektin salatut tiedot muuttuvat sinulle lukukelvottomiksi - tätä ei voi perua.

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
payment-methods-stale = Maksutietojasi muutettiin toisella laitteella. Ne ladattiin uudelleen — yritä uudelleen.
payment-methods-key-missing = Kirjaudu uudelleen hallitaksesi maksutietojasi.
settings-payment-methods-share-warning = Jaettu maksutapa näkyy kaikille niiden projektien jäsenille, joissa olet valinnut nimesi - kenelle tahansa, jolla on jokin näistä linkeistä.
payment-method-share = Jaa projekteilleni
payment-method-share-hint = Näytetään nimesi vieressä, kun joku on sinulle velkaa.
payment-method-copy = Kopioi { $name }
payment-method-copied = Kopioitu.
payment-method-copy-failed = Kopiointi ei onnistunut - valitse teksti ja kopioi se käsin.

verify-email-checking = Vahvistetaan sähköpostiosoitettasi…
verify-email-welcome = Sähköposti vahvistettu - tervetuloa Countediin!
verify-email-back-to-login = Takaisin kirjautumiseen

### Project status

project-close = Sulje
project-archive = Arkistoi
project-reopen = Avaa uudelleen
project-unarchive = Palauta arkistosta

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
projects-empty-hint = Luo projekti alla olevalla painikkeella
projects-offline-banner = Offline-tiedot - yhdistä uudelleen päivittääksesi.
projects-no-local-data = Ei paikallisia tietoja
projects-no-local-data-hint = Kirjaudu sisään ladataksesi projektisi ensimmäistä kertaa.
projects-add = Lisää projekti
projects-create = Luo projekti
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
project-unlock = Avaa lukitus
project-no-local-data-hint = Kirjaudu sisään ladataksesi tämän projektin tiedot ensimmäistä kertaa.
project-gone-title = Projektia ei enää ole
project-gone-hint = Se poistettiin, kun viimeinen jäsen poistui. Jakolinkki ei enää toimi, vaikka avaisit sen uudelleen.

expense-add = Lisää kulu
transfer-add = Lisää siirto
expense-edit-title = Muokkaa kulua
expense-category = Luokka
expense-category-auto = Auto · { $emoji }
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
reimbursements-empty-title = Kaikki tasattu!
reimbursements-empty-hint = Tasausehdotukset näkyvät tässä, kun tilit eivät täsmää
reimbursement-owes = { $debtor } on velkaa { $creditor }
reimbursement-record = Tasaa
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
expense-inconsistent-detail = Summat eivät täsmää: maksettu { $paid }, velkaa { $owed }, kulun summa { $total }. Korjaa muokkaamalla kulua.
missing-access-key = Pääsyavain puuttuu. Avaa projekti sen jakolinkin kautta.
filter-all = Kaikki
filter-my-payments = Omat maksuni
filter-my-debts = Omat velkani
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
charts-my-share-note = Nämä luvut ovat osuutesi kustakin kulusta.
charts-my-share-skipped =
    { $count ->
        [one] 1 projektia ei lasketa — osallistujaa ei valittu tai sen tiedot eivät latautuneet.
       *[other] { $count } projektia ei lasketa — osallistujaa ei valittu tai niiden tiedot eivät latautuneet.
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
charts-paid-more = { $amount } enemmän kuin osuutesi
charts-paid-less = { $amount } vähemmän kuin osuutesi
charts-paid-even = Täsmälleen osuutesi
charts-part-title = Osuutesi kustakin luokasta
charts-part-desc = Harmaa on ryhmän kulutus, väri sinun osuutesi.
charts-breakdown-title = Jakauma luokittain
charts-breakdown-desc = Napauta siivua tai riviä nähdäksesi kulut.
charts-of-total = { $amount } / { $total }
charts-show-all = Näytä kaikki ({ $count })
charts-show-less = Näytä vähemmän
charts-spend-title = Kulut ajan mittaan
charts-spend-desc = Lyhyet jaksot päivittäin, pidemmät viikoittain tai kuukausittain.
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
charts-projects-desc = Summat pidetään valuutoittain eikä niitä koskaan lasketa yhteen.
history-empty = Ei tapahtumia
history-by = { $name }
not-found-hint = Sivua ei ole olemassa, tai se on siirretty.
payers-title-paid-by = Maksaja
payers-title-sender = Lähettäjä
payers-title-contributors = Osallistujat
debtors-title-debtors = Velalliset
debtors-title-recipients = Vastaanottajat
debtors-title-beneficiaries = Edunsaajat

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
help-forgot-password-warning = Tietosi menetetään pysyvästi.
help-forgot-password-a = Salausavain johdetaan salasanastasi, joten nollaus ei ole mahdollista: kukaan - emme mekään - ei voi purkaa projektiesi salausta ilman sitä. Säilytä se turvassa, mieluiten salasanojen hallintaohjelmassa.
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
project-currency-hint = Kaikki summat näytetään tässä valuutassa. Sitä ei voi muuttaa myöhemmin.
project-currency-locked = Valuutta lukitaan projektia luotaessa.

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
friends-add-hint = Hän näkee pyyntösi kirjautuessaan. Kumpikaan ei saa tietää, onko toisella tiliä, ennen kuin pyyntö on hyväksytty.
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
friends-fingerprint = Turvakoodi
friends-fingerprint-hint = Kaksi kaveria, jotka lukevat toisilleen saman turvakoodin, tietävät, ettei kukaan ole heidän välissään - ei edes palvelimemme.
friends-outgoing-title = Lähetetyt
friends-outgoing-hint = Odottaa vastausta. Näet heidät kavereissasi, kun he hyväksyvät.
friends-withdraw = Peruuta
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
participants-you-from-account = Otettu tilisi nimestä. Muuta sitä tässä vain tätä projektia varten.
participants-you-required = Pakollinen. Näin muut näkevät sinut.
participants-others = Muut osallistujat
participants-empty = Ei vielä ketään. Valitse kaveri alta tai kirjoita mikä tahansa nimi.
participants-empty-signed-out = Ei vielä ketään. Lisää joku kirjoittamalla nimi.
participants-duplicate = ”{ $name }” on jo listalla.
participants-input-label = Lisää kaveri tai kirjoita nimi
participants-input-placeholder = Kaveri tai mikä tahansa nimi
participants-suggest-friend = Kaveri · liittyy nimellä ”{ $name }”, saa kutsun
participants-suggest-not-ready = Kaveri · ei vielä valmis
participants-suggest-guest = Lisää ”{ $text }” ilman tiliä
participants-suggest-guest-sub = Ei tiliä, pelkkä nimi
participants-friends = Kaverisi
participants-all-friends = Kaikki kaverit
participants-login-hint = Kirjaudu sisään lisätäksesi ihmisiä suoraan kaverilistaltasi.
participants-invite-badge = Kutsu
participants-guest-badge = Ilman tiliä
participants-guest-sub = Ei tiliä, pelkkä nimi
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
edit-project-you-are = Tällä laitteella olet { $name }
edit-project-no-identity = Et ole vielä valinnut, kuka olet
edit-project-switch = Vaihda
edit-project-choose = Valitse
invite-failed = Näitä kutsuja ei voitu lähettää: { $emails }
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
