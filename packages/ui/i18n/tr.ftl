# Türkçe. Yalnızca İngilizce ve Fransızca olan ve her ileti için ayrı ayrı en.ftl'ye dönen yasal
# metinler (legal-, terms-, privacy-) dışında eksiksizdir.

### Common

loading = Yükleniyor…
cancel = İptal
confirm = Onayla
retry = Tekrar dene
delete = Sil
back = Geri
language = Dil

### Navigation

nav-main = Ana gezinme
nav-projects = Projeler
nav-charts = İstatistikler
nav-settings = Ayarlar

### Connectivity

offline-banner = Çevrimdışı
offline-pending =
    { $count ->
        [one] { $count } bekliyor
       *[other] { $count } bekliyor
    }

sync-conflict-edit = Çakışma: “{ $name }” düzenlenemedi (öğe silinmiş). Atlandı.
sync-conflict-delete = Çakışma: “{ $name }” silinemedi (öğe silinmiş). Atlandı.
sync-conflict-other = Çakışma: “{ $name }” üzerindeki işlem başarısız oldu (öğe silinmiş). Atlandı.
sync-error = Eşitleme hatası: { $reason }

### Errors

error-network = Sunucuya ulaşılamıyor. İnternet bağlantını kontrol et.
error-generic = Bir şeyler ters gitti. Lütfen tekrar dene.

error-invalid-email = Bu e-posta adresi geçerli değil.
error-invalid-password = Bu şifre geçerli değil.
error-password-too-short = Şifren en az 8 karakter olmalı.
error-client-outdated = Uygulamanın bu sürümü güncel değil. Giriş yapmak için güncelle.
error-invalid-link = Bu bağlantı geçerli değil.
error-batch-too-large = Bir seferde çok fazla öğe.
error-payers-required = En az bir ödeyen seç.
error-debtors-required = Borçlu en az bir kişi seç.
error-duplicate-participant = Bir katılımcı aynı tarafta iki kez görünüyor.
error-participant-not-in-project = Bu katılımcı bu projenin parçası değil.
error-too-many-participants = Tek bir harcama için çok fazla katılımcı.
error-invalid-credentials = Yanlış e-posta veya şifre.
error-unauthenticated = Bunu yapmak için giriş yap.
error-email-not-verified = E-posta adresin henüz doğrulanmadı.
error-project-not-found = Bu proje artık yok.
error-expense-not-found = Bu harcama artık yok.
error-storage-full = Depolama alanı dolu: bu projenin anahtarı bu cihaza kaydedilemedi. Paylaşım bağlantısını sakla.
error-user-not-found = Bu katılımcı artık yok.
error-tricount-not-found = Tricount bulunamadı veya API'si hata döndürdü.
error-too-many-members = Bu proje üye sınırına ulaştı.
error-identity-taken = Başka bir hesap bu katılımcıyı zaten üstlendi.
error-claim-proof-invalid = Bu cihazda proje anahtarı yok, bu yüzden bir katılımcı üstlenemez. Paylaşım bağlantısını yeniden aç.
error-user-has-payments = Bu katılımcının projede harcamaları var ve kaldırılamaz.
error-resend-cooldown = Yeni bir e-posta istemeden önce 60 saniye bekle.
error-self-friend-request = Kendini arkadaş olarak ekleyemezsin.
error-not-a-friend = Yalnızca arkadaş listendekileri davet edebilirsin.
error-friend-has-no-key = Bu arkadaşın uygulamanın en son sürümünü henüz açmadı. Bir kez giriş yapmasını iste, sonra tekrar dene.
error-friend-request-not-found = Bu arkadaşlık isteği artık yok.
error-invitation-not-found = Bu davet artık yok.
error-too-many-friend-requests = Şimdilik çok fazla arkadaşlık isteği. Yarın tekrar dene.
error-too-many-invitations = Çok fazla bekleyen davet.
error-invalid-kdf-salt = Şifreleme ayarları geçerli değil. Uygulamayı güncelle ve tekrar dene.
error-mixed-project-batch = Bu katılımcıların hepsi aynı projede değil.
error-invalid-payload = Uygulamanın bu sürümü sunucunun kabul etmediği veri gönderdi. Güncelle ve tekrar dene.
error-invalid-public-key = Şifreleme anahtarın geçerli değil. Uygulamayı güncelle ve tekrar dene.
error-payment-methods-stale = Ödeme bilgilerin başka bir cihazda değiştirildi. Yeniden yükle ve tekrar dene.

### Auth

field-email = E-posta
field-email-placeholder = sen@ornek.com
field-password = Şifre
field-name = Ad
field-name-placeholder = Ayşe Yılmaz

login-title = Giriş yap
login-submit = Giriş yap
login-submitting = Giriş yapılıyor…
login-password-placeholder = Şifren
login-no-account = Henüz hesabın yok mu?
login-unverified = E-posta adresin henüz doğrulanmadı. Gelen kutunu kontrol et veya bağlantıyı yeniden gönder.
login-resend = Doğrulama bağlantısını yeniden gönder
login-resending = Gönderiliyor…
login-resend-sent = E-posta gönderildi - gelen kutunu kontrol et.

register-submit = Hesap oluştur
register-submitting = Oluşturuluyor…
register-have-account = Zaten hesabın var mı?
register-password-placeholder = En az 8 karakter
register-password-warning = Şifreni bir yere not et. Unutursan hesabın kurtarılamaz.
register-check-email-title = E-postanı kontrol et
register-email-sent = E-posta gönderildi
register-email-sent-hint = Hesabını etkinleştirmek için gelen kutundaki bağlantıya tıkla.
register-not-received-prefix = Gelmedi mi? Spam klasörünü kontrol et veya
register-sign-in-link = giriş yap
register-not-received-suffix = ve bağlantıyı yeniden gönder.
register-terms-prefix = Hesap oluşturarak
register-terms-link = kullanım koşullarımızı
register-terms-and = ve
register-privacy-link = gizlilik politikamızı kabul etmiş olursun

settings-title = Ayarlar
settings-preferences = Tercihler
settings-preferences-local = Bu cihazda saklanır.
settings-preferences-synced = Hesabınla eşitlenir, şifrelenir.
settings-about = Hakkında
settings-anonymous-title = Giriş yapmadın
settings-upsell-title = Projelerin, her cihazda
settings-upsell-free = Ücretsiz
settings-upsell-body = Counted hesapsız çalışır. Ücretsiz bir hesapla projelerin ve tercihlerin telefonuna, dizüstüne ve web'e seninle gelir - yine şifreli, yine bizim için okunamaz.
settings-locked-badge = Hesap
settings-locked-friends = Arkadaş eklemek ve onları uygulamadan bir projeye davet etmek için hesap oluştur - dolaştırılacak bağlantı yok.
settings-locked-payment-methods = IBAN'ını veya ödeme uygulamanı bir kez kaydet ve seçtiğin projelerle paylaş. Sana borcu olan, adının yanında görür.
settings-friends-hint = Arkadaş ekle ve bağlantı paylaşmadan onları projelerine davet et.

account-member-since = Üyelik tarihi
account-logout = Çıkış yap
account-logging-out = Çıkış yapılıyor…
account-delete-title = Hesabımı sil
account-delete-warning = Anında ve kalıcı, geri dönüşüm kutusu yok. Paylaşılan bir projeye girdiğin harcamalar diğer üyelere görünür kalır - onların hesaplarının parçasıdır.
account-delete-confirm-title = Hesabı sil
account-delete-confirm-message = Hesabın, oturumların ve proje listen kalıcı olarak silinecek. Şifren olmadan paylaşılan bir projenin şifreli verileri senin için okunamaz hale gelir - bu geri alınamaz.

settings-payment-methods = Ödeme bilgileri
settings-payment-methods-hint = Paranı nasıl geri almak istediğin. Hesabınla şifrelenir.
payment-method-kind = Yöntem
payment-method-kind-other = Diğer
payment-method-label = Ad
payment-method-label-placeholder = Ana hesap
payment-method-value = Bilgiler
payment-method-value-placeholder = IBAN, telefon numarası, kullanıcı adı…
payment-method-add = Ekle
payment-method-remove = { $name } kaldır
payment-method-empty = Henüz ödeme bilgisi eklemedin.
payment-method-deleted = Ödeme yöntemi silindi.
payment-method-value-required = Her ödeme yönteminin bilgilerini doldur veya yöntemi kaldır.
payment-method-label-required = Özel yöntemine bir ad ver.
payment-method-too-long = Bu çok uzun - kısalt.
payment-method-invalid-characters = Satır sonlarını veya görünmez karakterleri kaldır.
payment-method-limit = En fazla { $max } ödeme yöntemi kaydedebilirsin.
payment-methods-saved = Ödeme bilgileri kaydedildi.
payment-methods-offline = Ödeme bilgilerini kaydetmek için çevrimiçi olmalısın.
payment-methods-stale = Ödeme bilgilerin başka bir cihazda değiştirildi. Yeniden yüklendi — lütfen tekrar dene.
payment-methods-key-missing = Ödeme bilgilerini yönetmek için yeniden giriş yap.
settings-payment-methods-share-warning = Paylaşılan bir yöntem, adını seçtiğin projelerin her üyesine görünür - o proje bağlantılarından birine sahip herkese.
payment-method-share = Projelerimle paylaş
payment-method-share-hint = Biri sana borçluyken adının yanında gösterilir.
payment-method-copy = { $name } kopyala
payment-method-copied = Kopyalandı.
payment-method-copy-failed = Kopyalanamadı - metni seç ve elle kopyala.

verify-email-checking = E-posta adresin doğrulanıyor…
verify-email-welcome = E-posta doğrulandı - Counted'a hoş geldin!
verify-email-back-to-login = Girişe dön

### Project status

project-close = Kapat
project-archive = Arşivle
project-reopen = Yeniden aç
project-unarchive = Arşivden çıkar

### Dates

date-long = { $day } { $month } { $year }

month-1 = Ocak
month-2 = Şubat
month-3 = Mart
month-4 = Nisan
month-5 = Mayıs
month-6 = Haziran
month-7 = Temmuz
month-8 = Ağustos
month-9 = Eylül
month-10 = Ekim
month-11 = Kasım
month-12 = Aralık

month-short-1 = Oca
month-short-2 = Şub
month-short-3 = Mar
month-short-4 = Nis
month-short-5 = May
month-short-6 = Haz
month-short-7 = Tem
month-short-8 = Ağu
month-short-9 = Eyl
month-short-10 = Eki
month-short-11 = Kas
month-short-12 = Ara

### Actions

add = Ekle
create = Oluştur
creating = Oluşturuluyor…
edit = Düzenle
leave = Ayrıl
close = Kapat
paste = Yapıştır
join = Katıl
import = İçe aktar
importing = İçe aktarılıyor…
field-description = Açıklama
field-date = Tarih
date-today = Bugün
date-yesterday = Dün
field-optional = İsteğe bağlı

### Projects

projects-filter-active = Etkin
projects-filter-all = Tümü
projects-count-label = Projeler
projects-empty = Proje yok
projects-empty-hint = Aşağıdaki düğmeyle bir proje oluştur
projects-offline-banner = Çevrimdışı veriler - yenilemek için yeniden bağlan.
projects-no-local-data = Yerel veri yok
projects-no-local-data-hint = Projelerini ilk kez yüklemek için giriş yap.
projects-add = Proje ekle
projects-create = Proje oluştur
projects-join = Bir projeye katıl
projects-import-tricount = Tricount'tan içe aktar
project-actions = Proje işlemleri

status-ongoing = Devam ediyor
status-closed = Kapalı
status-archived = Arşivlendi

nav-help = Yardım
nav-privacy = Gizlilik politikası
nav-terms = Kullanım koşulları
nav-legal = Yasal bildirim

leave-project-title = Projeden ayrılınsın mı?
leave-project-message = Bu cihazdan erişimini kaybedeceksin. Hiç üye kalmazsa proje ve tüm harcamaları kalıcı olarak silinir.

add-project-title = Yeni proje
add-project-name-label = Proje adı
add-project-name-placeholder = Seyahatim, Ev arkadaşları 2024…
add-project-participants = Katılımcılar
add-project-participant-name = Katılımcı adı
add-project-participant-placeholder = Clark Kent
add-project-offline = Çevrimdışıyken proje oluşturamazsın. Yeniden bağlan ve tekrar dene.
add-project-name-required = Projenin bir adı olmalı.

join-link-label = Paylaşım bağlantısı
join-link-hint = Bağlantı, şifre çözme anahtarını taşır - tamamını kopyala.
join-invalid-link = Bu bağlantı geçerli değil. # sonrası dahil paylaşım bağlantısının tamamını yapıştır.
join-wrong-project = Bu bağlantı başka bir projeye ait.

import-tricount-link-label = Tricount bağlantısı veya anahtarı
import-tricount-key-required = Bir Tricount bağlantısı veya anahtarı gir.
import-tricount-encryption-failed = Şifreleme başarısız oldu.
import-tricount-unimportable = Hiçbir şey içe aktarılmadı: bu Tricount'ta Tricount hesabı olan üyeler ya da tutmayan tutarlar var (etkilenen kayıtlar: { $count }).

### Expenses

save = Kaydet
saving = Kaydediliyor…
adding = Ekleniyor…
link-copied = Bağlantı kopyalandı
missing-encryption-key = Şifreleme anahtarı eksik.
missing-encryption-key-title = Şifreleme anahtarı eksik
missing-encryption-key-hint = Kullandığın bağlantı, bu projenin şifresini çözmek için gereken anahtarı taşımıyor. Projeyi oluşturan kişinin paylaştığı tam bağlantıyı kullan.
project-locked-hint = Bu cihazda bu projenin anahtarı yok. Kilidini açmak için paylaşım bağlantısını aç.
project-unlock = Kilidi aç
project-no-local-data-hint = Bu projenin verilerini ilk kez yüklemek için giriş yap.
project-gone-title = Bu proje artık yok
project-gone-hint = Son üyesi ayrıldığında silindi. Paylaşım bağlantısı, yeniden açsan bile artık çalışmıyor.

expense-add = Harcama ekle
transfer-add = Transfer ekle
expense-edit-title = Harcamayı düzenle
expense-category = Kategori
expense-category-auto = Otomatik · { $emoji }
expense-currency = Tutarın para birimi
amount-op-add = Artı
amount-op-subtract = Eksi
amount-op-multiply = Çarp
amount-op-divide = Böl
amount-op-equals = Eşittir
amount-op-done = Tamam
expense-rate = Döviz kuru (isteğe bağlı)
expense-rate-hint = { $month } için Avrupa Komisyonu (InforEuro) kurunu kullanmak üzere boş bırak: 1 { $from } = { $rate } { $to }.
expense-rate-invalid = 0'dan büyük bir döviz kuru gir.
expense-rate-unavailable = Otomatik kur yok - elle gir.
expense-delete-title = Harcamayı sil
expense-delete-message = “{ $name }” kalıcı olarak silinecek. Bu geri alınamaz.
expense-inconsistent-amounts = Tutarlar tutmuyor
expenses-empty = Harcama yok
expenses-empty-hint = Aşağıdaki düğmeyle harcama ekleyerek başla
expenses-show-more = Daha fazla göster ({ $count } kaldı)

expense-type-expense = Harcama
expense-type-transfer = Transfer
expense-type-gain = Gelir
expense-paid-by = ödeyen:
expense-sent-by = gönderen:
expense-contributed-by = katkıda bulunan:

expense-name-required = Bir ad gerekli.
expense-amount-not-positive = Tutar 0'dan büyük olmalı.
expense-no-payer = En az bir ödeyen seç.
expense-no-debtor = Borçlu en az bir kişi seç.
expense-invalid-date = Bu tarih geçerli değil.
expense-payers-mismatch = Ödeyenlerin toplamı { $sum }, bu harcama tutarıyla ({ $total }) uyuşmuyor.
expense-debtors-mismatch = Borçluların toplamı { $sum }, bu harcama tutarıyla ({ $total }) uyuşmuyor.

participants-none = Hiç kimse
participants-everyone = Herkes ({ $count })
participants-some = { $count } / { $total }
participants-select-all = Tümünü seç
participants-by-shares = Paylara göre
split-amounts = Tutarlar
participants-remaining = { $amount } kaldı
participants-over-by = { $amount } fazla
participants-who-paid = Kim ödedi?
participants-who-received = Kim aldı?
participants-who-transfers = Kim gönderiyor?
participants-who-receives = Kim alıyor?
participants-for-whom = Kimin için?

stats-total-expenses = Toplam harcama
stats-my-expenses = Harcamalarım

tab-expenses = Harcamalar
tab-balance = Bakiye
tab-reimbursements = Hesaplaşma
reimbursements-empty-title = Hesaplar kapandı!
reimbursements-empty-hint = Hesaplar denk olmadığında hesaplaşma önerileri burada görünür
reimbursement-owes = { $debtor }, { $creditor } kişisine borçlu
reimbursement-record = Hesaplaş
reimbursement-pay-with = Öde
reimbursement-pay-shared-by = { $name } tarafından paylaşıldı - göndermeden önce uygulamanın gösterdiği alıcı adını kontrol et.
reimbursement-pay-title = { $name } kişisine öde
reimbursements-mine-title = Borcun
reimbursements-others-title = Diğer geri ödemeler
copy = Kopyala

user-selection-title = Hangi katılımcısın?
user-selection-hint = Listeden adını seç.
user-selection-required = Lütfen bir katılımcı seç.
identity-claimed = Bir hesaba bağlı
identity-claimed-by = { $name } hesabı
identity-taken-repick = Başka bir hesap kullandığın katılımcıyı üstlendi. Lütfen başka birini seç.
participant-gone-repick = Kullandığın katılımcı bu projeden kaldırıldı. Lütfen başka birini seç.

edit-project-title = Projeyi düzenle
edit-project-new-badge = yeni
edit-project-deferred-new-members = yeni üye ekleme
edit-project-deferred-removals = üye kaldırma
edit-project-offline-deferred = Çevrimdışı: { $items } yeniden bağlandığında uygulanacak.

export-failed = Dışa aktarma başarısız: { $reason }

history-expense-added = Harcama eklendi: { $name }
history-expense-edited = Harcama düzenlendi: { $name }
history-expense-deleted = Harcama silindi: { $name }
history-project-edited = Proje düzenlendi: { $name }
history-name-changed = Ad: “{ $from }” → “{ $to }”
history-description-added = Açıklama eklendi: “{ $value }”
history-description-removed = Açıklama kaldırıldı: “{ $value }”
history-description-changed = Açıklama: “{ $from }” → “{ $to }”

### Sweep

field-amount = Tutar
expense-name-placeholder = Restoran, market…
expense-actions = Harcama işlemleri
expense-your-share = Senin payın
expense-your-share-value = Senin payın: { $amount } { $currency }
expense-inconsistent-detail = Tutarlar tutmuyor: { $total } tutarındaki harcama için { $paid } ödendi, { $owed } borçlanıldı. Düzeltmek için harcamayı düzenle.
missing-access-key = Erişim anahtarı eksik. Bu projeyi paylaşım bağlantısından aç.
filter-all = Tümü
filter-my-payments = Ödemelerim
filter-my-debts = Borçlarım
participants-shares-for = { $name } payları
participants-amount-for = { $name } tutarı
reimbursement-add = Hesaplaşma ekle
project-forget = Listemden kaldır
project-history-title = Geçmiş
history-kind-add = Eklendi
history-kind-delete = Silindi
history-kind-edit = Düzenlendi
export = Dışa aktar
export-json = JSON olarak dışa aktar
export-csv = CSV olarak dışa aktar
share-link = Paylaş
copy-link-failed = Bağlantı kopyalanamadı
open-in-app = Uygulamada aç
not-found-title = Sayfa bulunamadı
not-found-back = Projelere dön

### Charts

charts-period = Dönem
period-all = Tümü
period-month = Ay
period-3months = 3 ay
period-year = Yıl
period-custom = Özel
charts-tab-categories = Kategoriler
charts-tab-trends = Eğilimler
charts-total-spent = Toplam harcanan
charts-avg-per-person = Kişi başı ort.
charts-expense-count =
    { $count ->
        [one] { $count } harcama
       *[other] { $count } harcama
    }
charts-nothing-to-show = Gösterilecek bir şey yok
charts-my-share-note = Bu rakamlar her harcamadaki senin payın.
charts-my-share-skipped =
    { $count ->
        [one] { $count } proje sayılmadı — katılımcı seçilmemiş veya verileri yüklenmemiş.
       *[other] { $count } proje sayılmadı — katılımcı seçilmemiş veya verileri yüklenmemiş.
    }

### Categories

category-food = Yemek
category-transport = Ulaşım
category-accommodation = Konaklama
category-leisure = Eğlence
category-shopping = Alışveriş
category-services = Hizmetler
category-parties-gifts = Partiler ve hediyeler
category-other = Diğer
charts-project = Proje
charts-all-projects = Tüm projeler
charts-date-from = Başlangıç
charts-date-to = Bitiş
charts-total = Toplam
charts-tab-people = Kişiler
charts-tab-projects = Projeler
charts-scope = Kimin harcaması
charts-scope-group = Grup
charts-scope-me = Ben
charts-currency = Para birimi
charts-my-share = Payım
charts-share-of-total = { $total } içinde %{ $pct }
charts-i-paid = Ödediğim
charts-paid-more = Payından { $amount } fazla
charts-paid-less = Payından { $amount } az
charts-paid-even = Tam payın kadar
charts-part-title = Her kategorideki payın
charts-part-desc = Gri grubun harcadığı, renkli senin tükettiğin.
charts-breakdown-title = Kategoriye göre dağılım
charts-breakdown-desc = Harcamaları görmek için bir dilime veya satıra dokun.
charts-of-total = { $amount } / { $total }
charts-show-all = Tümünü göster ({ $count })
charts-show-less = Daha az göster
charts-spend-title = Zaman içinde harcama
charts-spend-desc = Kısa dönemler günlük, uzun dönemler haftalık veya aylık.
charts-group-by = Gruplandır
bucket-day = Gün
bucket-week = Hafta
bucket-month = Ay
charts-avg = ort.
charts-cat-title-day = { $category }, gün gün
charts-cat-title-week = { $category }, hafta hafta
charts-cat-title-month = { $category }, ay ay
charts-cat-desc = Zaman içinde izlemek için bir kategori seç.
charts-running-title = Birikimli toplam
charts-running-desc = { $date } tarihinden beri.
charts-avg-per-day = Günde ortalama { $amount }
charts-avg-per-week = Haftada ortalama { $amount }
charts-avg-per-month = Ayda ortalama { $amount }
charts-people-title = Grubu kim taşıdı
charts-people-desc = Herkesin ödediği, tükettiğinin yanında.
charts-paid = Ödenen
charts-fair-share = Adil pay
charts-you = (sen)
charts-net-more = fazla ödedi
charts-net-less = az ödedi
charts-balance-title = Zaman içinde bakiyen
charts-balance-desc = Çizginin üstünde grup sana borçlu. Altında sen gruba borçlusun.
charts-owed = Alacaklısın
charts-owe = Borçlusun
charts-projects-title = Proje bazında payın
charts-projects-desc = Toplamlar para birimine göre tutulur, asla birbirine eklenmez.
history-empty = Olay yok
history-by = { $name }
not-found-hint = Bu sayfa yok veya taşınmış.
payers-title-paid-by = Ödeyen
payers-title-sender = Gönderen
payers-title-contributors = Katkıda bulunanlar
debtors-title-debtors = Borçlular
debtors-title-recipients = Alıcılar
debtors-title-beneficiaries = Yararlananlar

### Welcome

welcome-title = Hesapların kimseyi ilgilendirmez.
welcome-subtitle = Arkadaşlarınla harcamaları paylaş.
welcome-note = Ücretsiz. Hesap gerekmez. Reklam yok.
welcome-link-title = Tek bir bağlantı, herkes katılır.
welcome-link-body = Kimsenin hesap oluşturması gerekmez.
welcome-link-account = Hesap mı? Asla zorunlu değil. Projelerini başka bir cihazda bulmana, arkadaşlarını uygulamadan davet etmene ve ödeme bilgilerini paylaşmana yarar.
welcome-demo-project = Lyon'da hafta sonu
welcome-private-title = Hesaplarını kimse okuyamaz. Biz bile.
welcome-private-body = Adlar, tutarlar, projeler: hepsi cihazında şifrelenir. Anahtar yalnızca sende.
welcome-private-names = Adlar
welcome-private-amounts = Tutarlar
welcome-private-projects = Projeler
welcome-scan-title = Fişin fotoğrafını çek.
welcome-scan-body = Tutar, tarih ve kategori kendiliğinden dolar. Her şey telefonunda gerçekleşir. Fotoğraf saklanmaz.
welcome-eu-title = %100 Avrupa
welcome-no-ads = Reklam yok
welcome-no-trackers = İzleyici yok
welcome-step = Adım { $current } / { $total }
welcome-next = İleri
welcome-skip = Atla
welcome-start = Başla
welcome-how-it-works = Tam olarak nasıl çalışıyor?

### Help

help-intro = Sık sorulan bir soru mu? Yanıtı açmak için dokun.
help-create-project-q = Nasıl proje oluştururum?
help-create-project-a = Ana ekranda alttaki + düğmesine dokun. Projeye bir ad ver, para birimini seç, hazırsın.
help-add-participants-q = Nasıl katılımcı eklerim?
help-add-participants-a = Projeyi aç, sonra üye listesinden katılımcı ekle. Her katılımcı bir harcamayı ödeyebilir veya ona borçlanabilir.
help-share-project-q = Bir projeyi nasıl paylaşırım?
help-share-project-a = Projenin URL'sini (adres çubuğundakini) paylaş. Bağlantıya sahip herkes projeyi görüntüleyebilir ve düzenleyebilir.
help-add-expense-q = Nasıl harcama eklerim?
help-add-expense-a = Bir projede + düğmesine dokun, tutarı gir, kimin ödediğini ve kimler arasında bölüneceğini belirt. Bugünden farklı bir tarih de seçebilirsin.
help-types-q = Harcama, transfer ve gelir arasındaki fark nedir?
help-types-expense = - bir kişinin yaptığı ve birkaç kişi arasında bölünen bir alışveriş.
help-types-transfer = - bir kişiden diğerine geri ödeme, bölünme yok.
help-types-gain = - birkaç kişi arasında bölünecek alınan para (iade, hediye).
help-past-date-q = Bir harcamaya geçmiş tarih verebilir miyim?
help-past-date-a = Evet, tarih alanı serbesttir. Kaydın oluşturulma zamanı ayrıca tutulur.
help-who-owes-q = Counted kimin ne borçlu olduğunu nasıl hesaplar?
help-who-owes-a = Counted her katılımcının net bakiyesini (ödediği eksi borçlandığı) hesaplar, sonra herkesi hesaplaştıran en kısa transfer dizisini önerir.
help-minimal-transfers-q = Önerilen transfer sayısı neden en az?
help-minimal-transfers-a = Algoritma önce birbirini tam olarak götüren bakiyeleri eşleştirir, sonra kalanları en büyük alacaklıdan en büyük borçluya doğru işler. Sonuç: her şeyi kapatmak için daha az transfer.
help-import-tricount-q = Tricount'tan bir projeyi nasıl içe aktarırım?
help-import-tricount-a = Ana ekranda alttaki “+” düğmesine dokun, ardından
help-import-tricount-b = İçe aktarmak istediğin Tricount'un paylaşım bağlantısını yapıştır.
help-encryption-q = Verilerim şifreli mi?
help-encryption-a = Evet. Counted iki güvenceyi birleştirir:
help-encryption-e2ee-term = Uçtan uca şifreleme
help-encryption-e2ee-def = - seninle sunucu arasındaki her şey şifreli yolculuk eder.
help-encryption-zero-term = Sıfır erişim
help-encryption-zero-def = - verileri göndermeden önce şifrelersin ve sunucu yalnızca şifreli metin saklar. Onu okumanın hiçbir yolu yok.
help-encryption-see = Ayrıntılar için bkz.
help-forgot-password-q = Şifremi unutursam ne olur?
help-forgot-password-warning = Verilerin kalıcı olarak kaybolur.
help-forgot-password-a = Şifreleme anahtarı şifrenden türetilir, bu yüzden sıfırlama mümkün değildir: kimse - biz dahil - onsuz projelerinin şifresini çözemez. Güvende tut, tercihen bir şifre yöneticisinde.
help-archive-delete-q = Bir projeyi nasıl arşivler veya silerim?
help-archive-delete-a = Proje ekranından menüyü aç ve
help-archive-delete-b = seçeneğini seçerek projeyi saklayarak gizle. Bir proje, son üyesi ayrıldığında kalıcı olarak silinir.
help-delete-account-q = Hesabımı nasıl silerim?
help-delete-account-a = Ayarlar'ı aç ve “Hesabımı sil”i kullan. Anında olur ve geri alınamaz.
help-contact = Başka bir soru mu? Bize yaz:

# Receipt scanning (mobile only)
expense-scan = Fiş tara
scan-in-progress = Fiş okunuyor…
scan-error-capture = Fotoğraf çekilemedi. Tekrar dene veya harcamayı elle gir.
scan-error-unreadable = Bu fişte okunabilir bir şey yok. Harcamayı elle gir.
scan-check-amount = Toplamı kontrol et - net basılmamıştı.
scan-take-photo = Fotoğraf çek
scan-choose-photo = Fotoğraf seç
expense-converted-from = { $amount } { $from } ödendi · 1 { $from } = { $rate } { $to }
project-currency = Para birimi
project-currency-hint = Her tutar bu para biriminde gösterilir. Daha sonra değiştirilemez.
project-currency-locked = Para birimi proje oluşturulurken sabitlenir.

update-required-title = Güncelleme gerekli
update-required-body = Counted'ın bu sürümü sunucuyla konuşamayacak kadar eski. Uygulamayı kullanmaya devam etmek için güncelle.
update-required-button = Güncelle

notifications-label = Bildirimler
notifications-title = Bildirimler
notifications-empty = Yeni bir şey yok
notifications-friend-request = Arkadaşlık isteği

friends-title = Arkadaşlar
friends-anonymous-body = Arkadaşlar hesabınla saklanır. Bağlantı paylaşmadan kişi eklemek ve onları projelerine davet etmek için giriş yap.
friends-add-title = Arkadaş ekle
friends-add-hint = İsteğini giriş yaptığında görecek. İstek kabul edilene kadar ikiniz de diğerinin hesabı olup olmadığını öğrenmezsiniz.
friends-add-button = Ekle
friends-add-from-project = Arkadaş olarak ekle
friends-request-sent = İstek gönderildi
friends-no-account-key = Bu cihazda arkadaşlarını yönetmek için yeniden giriş yap.
friends-incoming-title = İstekler
friends-accept = Kabul et
friends-decline = Reddet
friends-list-title = Arkadaşlarım
friends-list-empty = Henüz arkadaş yok. Yukarıdan e-postayla veya paylaştığın bir projeden birini ekle.
friends-remove = Kaldır
friends-remove-confirm-title = Arkadaşı kaldır
friends-remove-confirm-message = { $email } artık arkadaşlarınız arasında olmayacak, siz de onunkiler arasında. İkinizden biri daha sonra yeni bir istek gönderebilir.
friends-no-key = Henüz hazır değil
friends-fingerprint = Güvenlik kodu
friends-fingerprint-hint = Birbirine aynı güvenlik kodunu okuyan iki arkadaş, aralarına kimsenin girmediğini bilir - sunucumuzun bile.
friends-outgoing-title = Gönderilen
friends-outgoing-hint = Yanıt bekleniyor. Kabul ettiklerinde arkadaşların arasında göreceksin.
friends-withdraw = İptal
invite-friends-title = Arkadaş davet et
invite-friends-hint = Proje anahtarı bu cihazda her arkadaş için şifrelenir. Sunucu onu asla görmez.
invite-friends-empty = Henüz davet edilecek arkadaş yok.
invite-friends-button = Davet et
invite-sent = { $count ->
    [one] Davet gönderildi
   *[other] { $count } davet gönderildi
}
invitation-badge = Davet
invitation-to = “{ $name }” projesine katıl
invitation-to-unnamed = Bir projeye katıl
invitation-unreadable = Bu davet bu cihazda açılamıyor
invitation-from = Gönderen: { $email }
invitation-accept = Katıl
invitation-decline = Reddet

# Participants in the create and edit modals, and the "who are you?" picker - see
# docs/plans/friends.md §11.
participants-you-label = Bu projedeki adın
participants-you-badge = Sen
participants-you-from-account = Hesap adından alındı. Yalnızca bu proje için burada değiştir.
participants-you-required = Zorunlu. Diğerleri seni böyle görecek.
participants-others = Diğer katılımcılar
participants-empty = Henüz kimse yok. Aşağıdan bir arkadaş seç ya da herhangi bir ad yaz.
participants-empty-signed-out = Henüz kimse yok. Birini eklemek için bir ad yaz.
participants-duplicate = “{ $name }” zaten listede.
participants-input-label = Arkadaş ekle ya da ad yaz
participants-input-placeholder = Arkadaş ya da herhangi bir ad
participants-suggest-friend = Arkadaş · “{ $name }” olarak katılır, davet alır
participants-suggest-not-ready = Arkadaş · henüz hazır değil
participants-suggest-guest = “{ $text }” adını hesapsız ekle
participants-suggest-guest-sub = Hesap yok, yalnızca bir ad
participants-friends = Arkadaşların
participants-all-friends = Tüm arkadaşlar
participants-login-hint = Arkadaş listenden doğrudan kişi eklemek için giriş yap.
participants-invite-badge = Davet et
participants-guest-badge = Hesapsız
participants-guest-sub = Hesap yok, yalnızca bir ad
participants-rename = Yeniden adlandır: { $name }
participants-remove = Kaldır: { $name }
participants-rename-label = Yeni ad
participants-rename-save = Adı kaydet
participants-rename-hint = Bu projede herkesin gördüğü ad. Davet yine { $email } adresine gider.
participants-invited-badge = Davet edildi
participants-invited-sub = { $email } · henüz kabul edilmedi
participants-invited-pending = Davet henüz kabul edilmedi
participants-unlinked = Bir hesaba bağlı değil
add-project-create-invite = Oluştur ve davet et: { $count }
edit-project-save-invite = Kaydet ve davet et: { $count }
edit-project-you-are = Bu cihazda sen { $name } olarak görünüyorsun
edit-project-no-identity = Henüz kim olduğunu seçmedin
edit-project-switch = Değiştir
edit-project-choose = Seç
invite-failed = Bu davetler gönderilemedi: { $emails }
invite-again = Yeniden davet et
friend-picker-title = Arkadaş ekle
user-selection-invited-hint = { $email } seni “{ $project }” projesine davet etti.
user-selection-suggested = Önerilen
user-selection-suggested-sub = { $email } seni bu adla ekledi
user-selection-confirm-as = Ben { $name }
user-selection-missing = Adın burada yok mu? Bir katılımcıdan seni proje ayarlarından eklemesini iste.
