# Español. Completo salvo los textos legales (legal-, terms-, privacy-), que solo existen en inglés
# y francés y recurren a en.ftl mensaje a mensaje.

### Common

loading = Cargando…
cancel = Cancelar
discard-title = ¿Descartar los cambios?
discard-message = Lo que has escrito se perderá.
discard = Descartar
retry = Reintentar
delete = Eliminar
back = Volver
language = Idioma
theme = Tema
theme-light = Claro
theme-dark = Oscuro
theme-system = Sistema
theme-hint-device = Guardado solo en este dispositivo.
theme-hint-system = Sigue la apariencia de este dispositivo.

### Navigation

nav-main = Navegación principal
nav-projects = Proyectos
nav-charts = Estadísticas
nav-settings = Ajustes

### Connectivity

offline-banner = Sin conexión
offline-pending =
    { $count ->
        [one] { $count } pendiente
       *[other] { $count } pendientes
    }

sync-conflict-edit = Conflicto: no se pudo editar «{ $name }» (elemento eliminado). Omitido.
sync-conflict-delete = Conflicto: no se pudo eliminar «{ $name }» (elemento eliminado). Omitido.
sync-conflict-other = Conflicto: la operación sobre «{ $name }» falló (elemento eliminado). Omitida.
sync-error = Error de sincronización: { $reason }

### Errors

error-network = No se puede contactar con el servidor. Comprueba tu conexión a internet.
error-rate-limited = El servidor está ocupado. Vuelve a intentarlo en un momento.
error-generic = Se ha producido un error. Inténtalo de nuevo.

error-invalid-email = Esta dirección de correo no es válida.
error-invalid-password = Esta contraseña no es válida.
error-password-too-short = La contraseña debe tener al menos 12 caracteres.
error-client-outdated = Esta versión de la aplicación está obsoleta. Actualízala para iniciar sesión.
error-invalid-link = Este enlace no es válido.
error-batch-too-large = Demasiados elementos a la vez.
error-payers-required = Selecciona al menos una persona que pagó.
error-debtors-required = Selecciona al menos una persona que debe.
error-duplicate-participant = Un participante aparece dos veces en el mismo lado.
error-participant-not-in-project = Ese participante no forma parte de este proyecto.
error-too-many-participants = Demasiados participantes para un solo gasto.
error-invalid-credentials = Correo o contraseña incorrectos.
error-unauthenticated = Inicia sesión para hacer eso.
error-email-not-verified = Tu dirección de correo aún no está verificada.
error-claim-proof-invalid = Este dispositivo no tiene la clave del proyecto, así que no puede reclamar un participante. Vuelve a abrir el enlace compartido.
error-project-not-found = Este proyecto ya no existe.
error-expense-not-found = Este gasto ya no existe.
error-storage-full = El almacenamiento está lleno: no se pudo guardar la clave de este proyecto en este dispositivo. Conserva su enlace para compartir.
error-user-not-found = Este participante ya no existe.
error-tricount-not-found = Tricount no encontrado, o su API devolvió un error.
error-too-many-members = Este proyecto ha alcanzado su límite de participantes.
error-user-has-payments = Este participante tiene gastos en el proyecto y no se puede eliminar.
error-resend-cooldown = Espera 60 segundos antes de pedir otro correo.

### Auth

field-email = Correo electrónico
field-email-placeholder = tu@ejemplo.es
field-password = Contraseña
field-name = Nombre
field-name-placeholder = Juan Pérez

login-title = Iniciar sesión
login-submit = Iniciar sesión
login-submitting = Iniciando sesión…
login-password-placeholder = Tu contraseña
login-no-account = ¿Aún no tienes cuenta?
login-unverified = Tu correo aún no está verificado. Revisa tu bandeja de entrada o vuelve a enviar el enlace.
login-maybe-unverified = ¿Acabas de registrarte? Verifica primero tu dirección con el enlace que te enviamos, o vuelve a enviarlo.
login-resend = Reenviar el enlace de verificación
login-resending = Enviando…
login-resend-sent = Correo enviado. Revisa tu bandeja de entrada.

register-submit = Crear una cuenta
register-submitting = Creando…
register-have-account = ¿Ya tienes cuenta?
register-password-placeholder = Mínimo 12 caracteres
register-password-warning = Apunta tu contraseña: nunca la vemos y no podemos devolvértela.
register-check-email-title = Revisa tu correo
register-email-sent = Correo enviado
register-email-sent-hint = Haz clic en el enlace de tu bandeja de entrada para activar tu cuenta.
register-not-received-prefix = ¿No te ha llegado? Revisa el spam o
register-sign-in-link = inicia sesión
register-not-received-suffix = para reenviar el enlace.
register-terms-prefix = Al crear una cuenta aceptas las
register-terms-link = condiciones de uso
register-terms-and = y nuestra
register-privacy-link = política de privacidad

settings-title = Ajustes
settings-preferences = Preferencias
settings-preferences-local = Se guardan en este dispositivo.
settings-preferences-synced = Sincronizadas con tu cuenta, cifradas.
settings-anonymous-title = No has iniciado sesión
settings-upsell-title = Tus proyectos, en todos tus dispositivos
settings-upsell-free = Gratis

account-logout = Cerrar sesión
account-logging-out = Cerrando sesión…
account-delete-title = Eliminar mi cuenta
account-delete-confirm-title = Eliminar la cuenta
account-delete-confirm-message = Tu cuenta, tus sesiones y tu lista de proyectos se eliminarán de forma permanente. Sin tu contraseña, los datos cifrados de un proyecto compartido dejarán de ser legibles para ti. Esta acción no se puede deshacer.
erase-device-title = ¿Borrar este dispositivo?
erase-device-body = Se borrarán los proyectos, sus claves y la caché de este dispositivo, y después se cerrará tu sesión. Tus proyectos siguen en tu cuenta: vuelve a iniciar sesión para recuperarlos.
erase-device-checking = Comprobando que tus proyectos están en tu cuenta…
erase-device-pending = Cambios pendientes de enviar: { $count }. Vuelve a conectarte y luego inténtalo de nuevo.
erase-device-unreachable = No se pudo comprobar que tus proyectos están en tu cuenta. Revisa tu conexión y luego inténtalo de nuevo.
erase-device-at-risk = Estos proyectos no están en tu cuenta y se perderán. Para conservarlos, cancela y copia antes su enlace:
erase-device-confirm = Borrar y cerrar sesión
erase-device-confirm-anyway = Borrar de todos modos
erase-device-erasing = Borrando…

settings-payment-methods = Datos de pago
payment-method-kind = Método
payment-method-kind-other = Otro
payment-method-label = Nombre
payment-method-label-placeholder = Cuenta principal
payment-method-value = Datos
payment-method-value-placeholder = IBAN, número de teléfono, usuario…
payment-method-add = Añadir
payment-method-deleted = Método de pago eliminado.
payment-method-value-required = Rellena los datos de cada método de pago, o elimínalo.
payment-method-label-required = Ponle un nombre a tu método personalizado.
payment-method-too-long = Es demasiado largo: acórtalo.
payment-method-invalid-characters = Quita los saltos de línea o los caracteres invisibles.
payment-method-limit = Puedes guardar hasta { $max } métodos de pago.
payment-methods-saved = Datos de pago guardados.
payment-methods-offline = Necesitas estar en línea para guardar tus datos de pago.
payment-methods-stale = Tus datos de pago se modificaron en otro dispositivo. Se han vuelto a cargar - inténtalo de nuevo.
payment-methods-key-missing = Vuelve a iniciar sesión para gestionar tus datos de pago.

verify-email-checking = Verificando tu correo…
verify-email-welcome = Correo verificado. ¡Bienvenido a Counted!
verify-email-back-to-login = Volver al inicio de sesión
verify-email-intro = Introduce tu contraseña para confirmar que esta dirección es tuya.
verify-email-submit = Verificar mi dirección

### Project status

project-archive = Archivar
project-sheet-invite = Invitar
project-sheet-recurring = Recurrentes
project-sheet-edit = Editar proyecto
project-sheet-close = Cerrar proyecto
project-sheet-archive = Archivar proyecto
project-sheet-reopen = Reabrir proyecto
project-sheet-unarchive = Desarchivar proyecto
project-sheet-leave = Salir del proyecto

### Dates

date-long = { $day } de { $month } de { $year }

month-1 = enero
month-2 = febrero
month-3 = marzo
month-4 = abril
month-5 = mayo
month-6 = junio
month-7 = julio
month-8 = agosto
month-9 = septiembre
month-10 = octubre
month-11 = noviembre
month-12 = diciembre

month-short-1 = Ene
month-short-2 = Feb
month-short-3 = Mar
month-short-4 = Abr
month-short-5 = May
month-short-6 = Jun
month-short-7 = Jul
month-short-8 = Ago
month-short-9 = Sep
month-short-10 = Oct
month-short-11 = Nov
month-short-12 = Dic

### Actions

add = Añadir
create = Crear
creating = Creando…
edit = Editar
leave = Salir
close = Cerrar
paste = Pegar
join = Unirse
import = Importar
importing = Importando…
field-description = Descripción
field-optional = Opcional

### Projects

projects-filter-active = Activos
projects-filter-all = Todos
projects-count-label = Proyectos
projects-empty = Sin proyectos
projects-offline-banner = Datos sin conexión: vuelve a conectarte para actualizar.
demo-banner = Proyecto de demostración - solo lectura.
demo-start-own = Crear mi proyecto
demo-badge = Demo
demo-bar = Tus cambios desaparecen al cerrar esta pestaña.
demo-leave = Empezar de verdad
demo-preparing = Preparando tu demo…
demo-failed = No se ha podido preparar la demo.
demo-sign-in-blocked = El inicio de sesión está desactivado en la demo. Sal de ella para iniciar sesión o crear una cuenta; tus propios proyectos no se tocan.
projects-no-local-data = Sin datos locales
projects-no-local-data-hint = Inicia sesión para cargar tus proyectos por primera vez.
projects-add = Añadir un proyecto
projects-create = Crear un proyecto
projects-personal-expense = Gasto personal
personal-expense-banner = Añadido a Mis gastos
personal-create-title = Crea tu proyecto personal
personal-create-hint = Sigue lo que gastas por tu cuenta
group-create-title = Crea un proyecto compartido
group-create-hint = Comparte gastos con amigos y familia
personal-create-action = Crear
personal-create-signup = Crear una cuenta
personal-sheet-title = Tu proyecto personal
personal-sheet-intro = Para anotar lo que gastas por tu cuenta. Aparece con tus grupos en Estadísticas.
personal-sheet-point-alone = Un solo miembro: tú, con el nombre de tu cuenta. Sin enlace para compartir.
personal-sheet-point-encrypted = Cifrado en tus dispositivos, como tus otros proyectos.
personal-sheet-point-stats = Tus partes en los grupos se suman a estos gastos en Estadísticas.
personal-default-name = Mis gastos
my-spending-prev-month = Mes anterior
my-spending-next-month = Mes siguiente
my-spending-spent = Has gastado
my-spending-groups = Grupos
my-spending-income = Ingresos
my-spending-balance = Saldo del mes
my-spending-unconverted = Las partes en otra moneda quedan fuera hasta que se conozca un tipo de cambio.
my-spending-empty = Nada este mes
my-spending-empty-hint = Añade un gasto con +. Tus partes en los grupos aparecerán aquí automáticamente.
my-spending-share-of = de { $amount }
my-spending-uncounted = no contado
payer-you = ti
expense-received-by = recibido por
counted-switch = Contar en Mis gastos
counted-switch-on = Activado: tu parte se suma a Mis gastos
counted-switch-off = Desactivado: tu parte no aparece en Mis gastos
counted-groups = Grupos contados
counted-groups-intro = Tu parte en estos grupos se suma a Mis gastos y a los presupuestos.
counted-groups-converted = { $from }, convertido a { $to }
counted-groups-encrypted = Esta elección se cifra con tu cuenta y te sigue en tus dispositivos.
counted-groups-empty = Todavía no hay ningún grupo en este dispositivo.
counted-groups-done = OK
counted-save-failed = No se pudo guardar esta elección. Inténtalo de nuevo.
tab-budget = Presupuesto
budget-title = Presupuestos
budget-manage = Gestionar
budget-left = Quedan { $amount }
budget-over = Excedido en { $amount }
budget-empty = Ningún presupuesto
budget-empty-hint = Fija un tope por categoría. Tus partes en los grupos también cuentan.
budget-define = Definir un presupuesto
budget-sheet-title = Presupuestos mensuales
budget-sheet-intro = Sobre tu parte: personal y grupos. Déjalo vacío para no seguir una categoría.
budget-average = Media últimos 3 meses: { $amount }
budget-for = Presupuesto { $category }
budget-alert = Avisarme al 80 %
budget-alert-hint = Calculado en este dispositivo
budget-invalid = Importe no válido para { $category }.
budget-offline = Los presupuestos solo se guardan con conexión. Vuelve a conectarte e inténtalo de nuevo.
budget-alert-near = { $category }: { $percent } % del presupuesto del mes
budget-alert-over = { $category }: presupuesto del mes superado
my-spending-search-summary =
    { $count ->
        [one] { $count } resultado
       *[other] { $count } resultados
    } · { $total } · personal y grupos, todos los meses
projects-join = Unirse a un proyecto
projects-import-tricount = Importar desde Tricount
project-actions = Acciones del proyecto

status-ongoing = En curso
status-closed = Cerrado
status-archived = Archivado

nav-help = Ayuda
nav-privacy = Política de privacidad
nav-terms = Condiciones de uso
nav-legal = Aviso legal

leave-project-title = ¿Salir del proyecto?
leave-project-message = Perderás el acceso desde este dispositivo. Si no queda ningún miembro, el proyecto y todos sus gastos se eliminarán definitivamente.

add-project-title = Nuevo proyecto
add-project-name-label = Nombre del proyecto
add-project-name-placeholder = Mi viaje, Piso 2024…
add-project-participants = Participantes
add-project-participant-name = Nombre del participante
add-project-participant-placeholder = Clark Kent
add-project-offline = No puedes crear un proyecto sin conexión. Vuelve a conectarte e inténtalo de nuevo.
add-project-name-required = El proyecto necesita un nombre.

join-link-label = Enlace para compartir
join-link-hint = El enlace contiene la clave de descifrado: cópialo entero.
join-invalid-link = Ese enlace no es válido. Pega el enlace completo, incluida la parte después del #.
join-wrong-project = Ese enlace corresponde a otro proyecto.

import-tricount-link-label = Enlace o clave de Tricount
import-tricount-key-required = Introduce un enlace o una clave de Tricount.
import-tricount-encryption-failed = Error de cifrado.
import-tricount-unimportable = No se importó nada: este Tricount tiene miembros con cuenta de Tricount o importes que no cuadran (entradas afectadas: { $count }).

### Expenses

save = Guardar
saving = Guardando…
adding = Añadiendo…
link-copied = Enlace copiado
missing-encryption-key = Falta la clave de cifrado.
missing-encryption-key-title = Falta la clave de cifrado
missing-encryption-key-hint = El enlace que has usado no lleva la clave necesaria para descifrar este proyecto. Usa el enlace completo que compartió quien lo creó.
project-locked-hint = Este dispositivo no tiene la clave de este proyecto. Abre su enlace para desbloquearlo.
expenses-unreadable = Algunas entradas no se pueden leer en este dispositivo y no cuentan en los totales. Actualiza la app para verlas.
project-unlock = Desbloquear
project-no-local-data-hint = Inicia sesión para cargar los datos de este proyecto por primera vez.
project-gone-hint = Se eliminó cuando su último miembro salió. El enlace ya no funciona, aunque vuelvas a abrirlo.

expense-add = Añadir un gasto
transfer-add = Añadir una transferencia
expense-edit-title = Editar el gasto
expense-category = Categoría
expense-delete-title = Eliminar el gasto
expense-delete-message = «{ $name }» se eliminará definitivamente. Esta acción no se puede deshacer.
expense-inconsistent-amounts = Importes incoherentes
expenses-empty = Sin gastos
expenses-empty-hint = Empieza añadiendo gastos con el botón de abajo

expense-type-expense = Gasto
expense-type-transfer = Transferencia
expense-type-gain = Ingreso
expense-type-group = Tipo
expense-paid-by = pagado por
expense-sent-by = enviado por
expense-contributed-by = aportado por

expense-name-required = El nombre es obligatorio.
expense-amount-not-positive = El importe debe ser mayor que 0.
expense-no-payer = Selecciona al menos una persona que pagó.
expense-no-debtor = Selecciona al menos una persona que debe.
expense-invalid-date = Esa fecha no es válida.
expense-payers-mismatch = Los pagadores suman { $sum }, que no coincide con el importe del gasto ({ $total }).
expense-debtors-mismatch = Los deudores suman { $sum }, que no coincide con el importe del gasto ({ $total }).

participants-none = Nadie
participants-everyone = Todos ({ $count })
participants-some = { $count } de { $total }
participants-select-all = Seleccionar todo
participants-remaining = Faltan { $amount }
participants-over-by = Sobran { $amount }
participants-who-paid = ¿Quién pagó?
participants-who-received = ¿Quién recibió?
participants-who-transfers = ¿Quién transfiere?
participants-who-receives = ¿Quién recibe?
participants-for-whom = ¿Para quién?

stats-total-expenses = Total de gastos
stats-my-expenses = Mis gastos

tab-expenses = Gastos
tab-balance = Balance
tab-reimbursements = Saldar cuentas
balance-gets-back = Recupera
balance-owes = Debe
balance-settled = Al día
reimbursements-empty-hint = Aquí aparecerán sugerencias de pago cuando las cuentas no cuadren
reimbursement-owes = { $debtor } debe a { $creditor }

user-selection-title = ¿Qué participante eres?
user-selection-hint = Elige tu nombre en la lista.
user-selection-required = Selecciona un participante.

edit-project-deferred-new-members = añadir participantes nuevos
edit-project-deferred-removals = eliminar participantes
edit-project-offline-deferred = Sin conexión: { $items } se aplicará cuando vuelvas a conectarte.

export-failed = Error al exportar: { $reason }

history-expense-added = Gasto añadido: { $name }
history-expense-edited = Gasto editado: { $name }
history-expense-deleted = Gasto eliminado: { $name }
move-expense = Mover a un proyecto
move-expense-title = Mover a…
move-current = Actual
move-converted = { $currency }, convertido al tipo del mes
move-note-personal = Elegirás quién pagó y para quién en el siguiente paso. El gasto saldrá de Mis gastos.
move-note-group = El gasto saldrá de { $project } para todos sus miembros y se recalculará el balance del grupo. Elegirás quién pagó y para quién en el siguiente paso.
move-banner = De { $from } a { $to }
move-submit = Mover
move-done = Gasto movido a { $project }
history-expense-moved = Gasto movido a otro proyecto: { $name }
personal-badge = Personal
move-members =
    { $count ->
        [one] { $count } miembro
       *[other] { $count } miembros
    }
history-project-edited = Proyecto editado: { $name }
history-name-changed = Nombre: «{ $from }» → «{ $to }»
history-description-added = Descripción añadida: «{ $value }»
history-description-removed = Descripción eliminada: «{ $value }»
history-description-changed = Descripción: «{ $from }» → «{ $to }»

### Sweep

field-amount = Importe
expense-name-placeholder = Restaurante, compra…
expense-actions = Acciones del gasto
expense-your-share = Tu parte
expense-your-share-value = Tu parte: { $amount } { $currency }
expense-share-percent = { $percent } %
expense-added-by = Añadido por { $name } el { $date }
expense-inconsistent-detail = Importes incoherentes: { $paid } pagado, { $owed } debido, para un gasto de { $total }. Edita el gasto para corregirlo.
missing-access-key = Falta la clave de acceso. Abre este proyecto desde su enlace para compartir.
filter-all = Todo
filter-my-payments = Mis pagos
filter-my-debts = Lo que debo
expense-filter-group = Filtro
expenses-search = Buscar gastos
expenses-no-match = Ningún gasto coincide.
expenses-search-open = Buscar
expenses-search-clear = Borrar la búsqueda
expenses-search-summary =
    { $count ->
        [one] { $count } resultado
       *[other] { $count } resultados
    } · { $total } · todos los meses
participants-shares-for = Partes de { $name }
participants-amount-for = Importe de { $name }
reimbursement-add = Añadir un pago
project-forget = Quitar de mi lista
project-history-title = Historial
history-kind-add = Añadido
history-kind-delete = Eliminado
history-kind-edit = Editado
export = Exportar
export-json = Exportar JSON
export-csv = Exportar CSV
share-link = Compartir
copy-link-failed = No se pudo copiar el enlace
open-in-app = Abrir en la aplicación
not-found-title = Página no encontrada
not-found-back = Volver a los proyectos

### Charts

charts-period = Periodo
period-all = Todo
period-month = Mes
period-3months = 3 meses
period-year = Año
period-custom = Pers.
charts-tab-categories = Categorías
charts-tab-trends = Tendencias
charts-total-spent = Total gastado
charts-avg-per-person = Media por persona
charts-expense-count =
    { $count ->
        [one] { $count } gasto
       *[other] { $count } gastos
    }
charts-nothing-to-show = Nada que mostrar
charts-my-share-skipped =
    { $count ->
        [one] 1 proyecto no se cuenta - no has elegido participante, o sus datos no se han cargado.
       *[other] { $count } proyectos no se cuentan - no has elegido participante, o sus datos no se han cargado.
    }

### Categories

category-food = Comida
category-transport = Transporte
category-accommodation = Alojamiento
category-leisure = Ocio
category-shopping = Compras
category-services = Servicios
category-parties-gifts = Fiestas y regalos
category-other = Otros
categories-manage = Gestionar categorías
categories-title = Categorías
categories-yours = Tus categorías
categories-builtin = Predeterminadas
categories-add = Añadir una categoría
categories-in-parent = en { $parent }
categories-renamed-from = renombrada · originalmente { $name }
categories-hidden-hint = oculta · sus gastos van a { $other }
categories-always-shown = siempre visible
categories-hidden = Oculta
categories-new = Nueva categoría
categories-edit = Editar categoría
categories-parent = Cuenta en
categories-parent-hint = En las estadísticas de todos tus proyectos, { $name } cuenta en { $parent }.
categories-create = Crear
categories-hide = Ocultar esta categoría
categories-hide-hint = Sus gastos aparecen en { $other } para todos los miembros.
categories-delete-title = ¿Eliminar «{ $name }»?
categories-delete-body = Los gastos afectados ({ $count }) vuelven a { $parent }. No se elimina ningún gasto.
categories-delete-body-unknown = Sus gastos vuelven a { $parent }. No se elimina ningún gasto.
categories-delete-warning = La categoría desaparece para todos los miembros del proyecto.
categories-summary-created = Creadas: { $count }
categories-summary-renamed = Renombradas: { $count }
categories-summary-hidden = Ocultas: { $count }
categories-summary-default = Sin categorías propias
categories-offline = Gestionar categorías requiere conexión. Vuelve a conectarte e inténtalo de nuevo.
categories-error-too-many = Un proyecto tiene como máximo 20 categorías propias.
categories-error-full = Este proyecto ha alcanzado su límite de categorías.
categories-error-name = Ponle un nombre a la categoría.
categories-error-missing = Esta categoría ha cambiado mientras tanto. Cierra e inténtalo de nuevo.
history-category-created = Categoría creada: { $name }
history-category-edited = Categoría editada: { $name }
history-category-deleted = Categoría eliminada: { $name }
history-empty = Sin eventos
not-found-hint = Esta página no existe o se ha movido.
payers-title-paid-by = Pagado por
payers-title-sender = Emisor
payers-title-contributors = Contribuyentes
debtors-title-debtors = Deudores
debtors-title-recipients = Destinatarios
debtors-title-beneficiaries = Beneficiarios
debtors-title-for = Para
debtors-title-recipient = Destinatario
expense-split-between = repartido entre { $count }
expense-people =
    { $count ->
        [one] { $count } persona
       *[other] { $count } personas
    }

### Welcome

welcome-title = Tus cuentas solo te incumben a ti.
welcome-subtitle = Reparte gastos entre amigos.
welcome-note = Gratis. Sin necesidad de cuenta. Sin anuncios.
welcome-link-title = Un enlace, y todos participan.
welcome-link-body = Nadie tiene que crear una cuenta.
welcome-link-account = ¿Una cuenta? Nunca obligatoria. Sirve para recuperar tus proyectos en otro dispositivo, invitar a tus amigos desde la app y compartir tus datos de pago.
welcome-demo-project = Fin de semana en Lyon
welcome-private-title = Nadie puede leer tus cuentas. Ni siquiera nosotros.
welcome-private-body = Nombres, importes, proyectos: todo se cifra en tu dispositivo. Solo tú tienes la clave.
welcome-private-names = Nombres
welcome-private-amounts = Importes
welcome-private-projects = Proyectos
welcome-scan-title = Hazle una foto al recibo.
welcome-scan-body = El importe, la fecha y la categoría se rellenan solos. Todo ocurre en tu teléfono. La foto no se guarda.
welcome-eu-title = 100 % europeo
welcome-no-ads = Cero anuncios
welcome-no-trackers = Cero rastreadores
welcome-step = Paso { $current } de { $total }
welcome-next = Siguiente
welcome-skip = Omitir
welcome-start = Empezar
welcome-how-it-works = ¿Cómo funciona exactamente?

### Help

help-intro = ¿Una duda frecuente? Toca para desplegar la respuesta.
help-create-project-q = ¿Cómo creo un proyecto?
help-create-project-a = Desde la pantalla de inicio, toca el botón + de abajo. Dale un nombre al proyecto, elige la moneda y listo.
help-add-participants-q = ¿Cómo añado participantes?
help-add-participants-a = Abre el proyecto y añade participantes desde la lista de miembros. Cada participante puede pagar o deber en un gasto.
help-share-project-q = ¿Cómo comparto un proyecto?
help-share-project-a = Comparte la URL del proyecto (la de la barra de direcciones). Cualquiera con el enlace puede ver y editar el proyecto.
help-add-expense-q = ¿Cómo añado un gasto?
help-add-expense-a = Dentro de un proyecto, toca +, introduce el importe e indica quién pagó y entre quiénes se reparte. También puedes elegir una fecha distinta de hoy.
help-types-q = ¿Qué diferencia hay entre gasto, transferencia e ingreso?
help-types-expense = - una compra hecha por una persona y repartida entre varias.
help-types-transfer = - una devolución de una persona a otra, sin reparto.
help-types-gain = - dinero recibido (una devolución, un regalo) para repartir entre varias personas.
help-past-date-q = ¿Puedo poner a un gasto una fecha pasada?
help-past-date-a = Sí, el campo de fecha es libre. La fecha de creación del registro se guarda aparte.
help-who-owes-q = ¿Cómo calcula Counted quién debe qué?
help-who-owes-a = Counted calcula el saldo neto de cada participante (lo que adelantó menos lo que debe) y propone la serie de transferencias más corta para saldar todas las cuentas.
help-minimal-transfers-q = ¿Por qué el número de transferencias sugeridas es mínimo?
help-minimal-transfers-a = El algoritmo empareja primero los saldos que se compensan exactamente y luego recorre el resto del mayor acreedor al mayor deudor. Resultado: menos transferencias para saldarlo todo.
help-import-tricount-q = ¿Cómo importo un proyecto desde Tricount?
help-import-tricount-a = Desde la pantalla de inicio, toca el botón «+» de abajo y luego
help-import-tricount-b = Pega el enlace para compartir del Tricount que quieras importar.
help-encryption-q = ¿Están cifrados mis datos?
help-encryption-a = Sí. Counted combina dos garantías:
help-encryption-e2ee-term = Cifrado de extremo a extremo
help-encryption-e2ee-def = - todo lo que va entre tú y el servidor viaja cifrado.
help-encryption-zero-term = Cero acceso
help-encryption-zero-def = - tú cifras los datos antes de enviarlos y el servidor solo guarda texto cifrado. No tenemos forma de leerlo.
help-encryption-see = Para más detalles, consulta la
help-forgot-password-q = ¿Qué pasa si olvido mi contraseña?
help-forgot-password-warning = Nadie puede recuperarla por ti.
help-forgot-password-a = Tu contraseña protege tus claves de cifrado y nunca se nos envía: nadie, ni siquiera nosotros, puede descifrar tus proyectos por ti. Guárdala bien, idealmente en un gestor de contraseñas.
help-archive-delete-q = ¿Cómo archivo o elimino un proyecto?
help-archive-delete-a = Desde la pantalla del proyecto, abre el menú y elige
help-archive-delete-b = para ocultarlo sin perderlo. Un proyecto solo se elimina definitivamente cuando lo abandona su último miembro.
help-delete-account-q = ¿Cómo elimino mi cuenta?
help-delete-account-a = Abre los Ajustes y usa «Eliminar mi cuenta». Es inmediato y no se puede deshacer.
help-contact = ¿Otra duda? Escríbenos a

# Receipt scanning (mobile only)
expense-scan = Escanear un recibo
scan-in-progress = Leyendo el recibo…
scan-error-capture = No se pudo tomar esa foto. Inténtalo de nuevo o introduce el gasto a mano.
scan-error-unreadable = No hay nada legible en ese recibo. Introduce el gasto a mano.
scan-check-amount = Comprueba el total: no estaba impreso con claridad.
scan-take-photo = Tomar una foto
scan-choose-photo = Elegir una foto

update-required-title = Actualización necesaria
update-required-body = Esta versión de Counted es demasiado antigua para comunicarse con el servidor. Actualízala para seguir usando la aplicación.
update-required-button = Actualizar

### Common (añadidos)

confirm = Confirmar
copy = Copiar
field-date = Fecha
date-today = Hoy
date-yesterday = Ayer

### Errors (amigos)

error-identity-taken = Otra cuenta ya ha reclamado a este participante.
error-self-friend-request = No puedes añadirte a ti mismo como amigo.
error-not-a-friend = Solo puedes invitar a personas de tu lista de amigos.
error-friend-has-no-key = Este amigo aún no ha abierto la última versión de la aplicación. Pídele que inicie sesión una vez y vuelve a intentarlo.
error-friend-request-not-found = Esta solicitud de amistad ya no existe.
error-invitation-not-found = Esta invitación ya no existe.
error-too-many-friend-requests = Demasiadas solicitudes de amistad por ahora. Inténtalo mañana.
error-too-many-invitations = Demasiadas invitaciones pendientes.
error-invalid-kdf-salt = La configuración de cifrado no es válida. Actualiza la aplicación e inténtalo de nuevo.
error-mixed-project-batch = Esos participantes no están todos en el mismo proyecto.
error-invalid-payload = Esta versión de la aplicación envió datos que el servidor no acepta. Actualízala e inténtalo de nuevo.
error-invalid-public-key = Tu clave de cifrado no es válida. Actualiza la aplicación e inténtalo de nuevo.
error-payment-methods-stale = Tus datos de pago se han cambiado en otro dispositivo. Recarga e inténtalo de nuevo.

### Payment methods (compartir)

payment-method-copy = Copiar { $name }
payment-method-copied = Copiado.
payment-method-copy-failed = No se pudo copiar: selecciona el texto y cópialo a mano.

### Expenses (moneda, importe)

expense-category-auto = Auto · { $emoji }
emoji-picker-label = Emoji
emoji-none = Sin emoji
expense-currency = Moneda del importe
amount-op-add = Más
amount-op-subtract = Menos
amount-op-multiply = Multiplicar
amount-op-divide = Dividir
amount-op-equals = Igual
amount-op-done = Listo
expense-rate = Tipo de cambio (opcional)
expense-rate-hint = Déjalo vacío para usar el tipo de la Comisión Europea (InforEuro) de { $month }: 1 { $from } = { $rate } { $to }.
expense-rate-invalid = Introduce un tipo de cambio mayor que 0.
expense-rate-unavailable = No hay tipo automático disponible: introdúcelo a mano.
expenses-show-more = Mostrar más ({ $count } restantes)
expense-converted-from = Pagado { $amount } { $from } · 1 { $from } = { $rate } { $to }
project-currency = Moneda
project-currency-locked = La moneda se fija al crear el proyecto.
currency-search = Buscar una moneda
project-gone-title = Este proyecto ya no existe
participants-by-shares = Por partes
split-amounts = Importes

### Reimbursements

reimbursements-empty-title = ¡Cuentas saldadas!
reimbursement-record = Saldar
settlement-recorded = Pago registrado.
reimbursement-pay-with = Pagar
reimbursement-pay-shared-by = Compartido por { $name }: comprueba el nombre del destinatario que muestra tu aplicación antes de enviar.
reimbursement-pay-title = Pagar a { $name }
reimbursements-mine-title = Debes
reimbursements-others-title = Otros reembolsos

### Identity

identity-claimed = Vinculado a una cuenta
identity-claimed-by = Cuenta de { $name }
identity-taken-repick = Otra cuenta ha reclamado al participante que usabas. Elige otro.
participant-gone-repick = El participante que usabas se ha eliminado de este proyecto. Elige otro.

### Edit project

edit-project-title = Editar el proyecto
edit-project-new-badge = nuevo

### Charts (añadidos)

charts-project = Proyecto
charts-all-projects = Todos los proyectos
charts-date-from = Desde
charts-date-to = Hasta
charts-total = Total
charts-tab-people = Personas
charts-tab-projects = Proyectos
charts-scope = Gastos de quién
charts-scope-group = Grupo
charts-scope-me = Yo
charts-currency = Moneda
charts-my-share = Mi parte
charts-share-of-total = { $pct } % de { $total }
charts-i-paid = He pagado
charts-paid-more = { $amount } más que mi parte
charts-paid-less = { $amount } menos que mi parte
charts-paid-even = Justo mi parte
charts-part-title = Tu parte de cada categoría
charts-part-desc = En gris lo que gastó el grupo, en color lo que consumiste tú.
charts-breakdown-title = Desglose por categoría
charts-breakdown-desc = Toca una porción o una fila para ver sus gastos.
charts-of-total = { $amount } de { $total }
charts-show-all = Ver todo ({ $count })
charts-show-less = Ver menos
charts-spend-title = Gastos en el tiempo
charts-group-by = Agrupar por
bucket-day = Día
bucket-week = Semana
bucket-month = Mes
charts-avg = media
charts-cat-title-day = { $category }, día a día
charts-cat-title-week = { $category }, semana a semana
charts-cat-title-month = { $category }, mes a mes
charts-cat-desc = Elige una categoría para seguirla en el tiempo.
charts-running-title = Total acumulado
charts-running-desc = Desde el { $date }.
charts-avg-per-day = { $amount } / día de media
charts-avg-per-week = { $amount } / semana de media
charts-avg-per-month = { $amount } / mes de media
charts-people-title = Quién sostuvo al grupo
charts-people-desc = Lo que pagó cada persona, junto a lo que consumió.
charts-paid = Pagado
charts-fair-share = Parte justa
charts-you = (tú)
charts-net-more = pagó más
charts-net-less = pagó menos
charts-balance-title = Tu saldo en el tiempo
charts-balance-desc = Por encima de la línea el grupo te debe. Por debajo, tú debes al grupo.
charts-owed = Te deben
charts-owe = Debes
charts-projects-title = Tu parte, por proyecto
charts-projects-desc = Los totales se separan por moneda.
history-by = Por { $name }

### Notifications

notifications-label = Notificaciones
notifications-title = Notificaciones
notifications-empty = Nada nuevo

### Friends

friends-title = Amigos
friends-anonymous-body = Los amigos están vinculados a tu cuenta. Inicia sesión para añadir personas e invitarlas a tus proyectos sin compartir un enlace.
friends-add-title = Añadir un amigo
friends-add-from-project = Añadir como amigo
friends-request-sent = Solicitud enviada
friends-no-account-key = Vuelve a iniciar sesión para gestionar tus amigos en este dispositivo.
friends-incoming-title = Solicitudes
friends-accept = Aceptar
friends-decline = Rechazar
friends-list-title = Mis amigos
friends-remove = Quitar
friends-remove-confirm-title = Quitar amigo
friends-remove-confirm-message = { $email } dejará de estar en tus amigos, y tú en los suyos. Cualquiera de los dos podrá enviar una nueva solicitud más tarde.
friends-no-key = Aún no está listo
friends-key-mismatch = El servidor envió para tu cuenta una clave pública que no coincide con tu clave privada. Tu código de seguridad está oculto y las invitaciones están desactivadas en este dispositivo: puede que alguien esté manipulando el servidor.
keypair-mismatch = Las claves que el servidor envió para tu cuenta no coinciden con las de este dispositivo. Las invitaciones y los cambios de clave están desactivados aquí hasta que coincidan: puede que alguien esté manipulando el servidor.
friends-outgoing-title = Enviadas
friends-notify = Avisar por mensaje
friends-notify-message = Me paso a Counted para nuestras cuentas compartidas. Registro gratuito en https://counted.fr con la dirección { $email }: mi solicitud de amistad espera allí.
friends-notify-copied = Mensaje copiado
friends-notify-copy-failed = No se pudo copiar el mensaje
guests-hint-title =
    { $count ->
        [one] { $count } participante aún no se ha unido al proyecto
       *[other] { $count } participantes aún no se han unido al proyecto
    }
guests-hint-later = Más tarde
guests-hint-send = Invítalos
guests-send-title = Enviar el proyecto a los participantes
guests-send-friend = Invitar a un amigo de Counted
guests-send-friend-badge = Sin enlace
guests-send-friend-hint = Si ya tiene cuenta, recibe una invitación cifrada; nada pasa por tu mensajería.
guests-send-link = Compartir el enlace del proyecto
guests-send-link-hint = Para quien no tiene cuenta. Abre el proyecto con un toque.
guests-send-link-warning = Cualquiera que tenga este enlace puede leer y modificar el proyecto. Tu mensajería también lo ve.
invite-friends-title = Invitar amigos
invite-friends-hint = La clave del proyecto se cifra para cada amigo en este dispositivo. El servidor nunca la ve.
invite-friends-empty = Aún no hay amigos a los que invitar.
invite-friends-button = Invitar
invite-sent = { $count ->
    [one] Invitación enviada
   *[other] { $count } invitaciones enviadas
}
invitation-badge = Invitación
invitation-to = Unirse a «{ $name }»
invitation-to-unnamed = Unirse a un proyecto
invitation-unreadable = Esta invitación no se puede abrir en este dispositivo
invitation-from = De { $email }
invitation-accept = Unirse
invitation-decline = Rechazar

# Participants in the create and edit modals, and the "who are you?" picker - see
# docs/plans/friends.md §11.
participants-you-label = Tu nombre en este proyecto
participants-you-badge = Tú
participants-others = Otros participantes
participants-removed = Retirados
participant-restore = Restaurar
participants-empty = Añade participantes abajo.
participants-duplicate = «{ $name }» ya está en la lista.
participants-input-label = Añadir un amigo o escribir un nombre
participants-input-placeholder = Un amigo o cualquier nombre
participants-suggest-friend = Amigo · entra como «{ $name }», recibe una invitación
participants-suggest-not-ready = Amigo · aún no está listo
participants-suggest-guest = Añadir «{ $text }»
participants-suggest-guest-sub = Invitado
participants-friends = Tus amigos
participants-all-friends = Todos los amigos
participants-login-hint = Inicia sesión para añadir personas directamente desde tu lista de amigos.
participants-invite-badge = Invitar
participants-guest-badge = Invitado
participants-rename = Renombrar a { $name }
participants-remove = Quitar a { $name }
participants-rename-label = Nuevo nombre
participants-rename-save = Guardar el nombre
participants-rename-hint = El nombre que todos ven en este proyecto. La invitación sigue yendo a { $email }.
participants-invited-badge = Invitado
participants-invited-sub = { $email } · aún no aceptada
participants-invited-pending = Invitación aún no aceptada
participants-unlinked = No vinculado a ninguna cuenta
add-project-create-invite = Crear e invitar a { $count }
edit-project-save-invite = Guardar e invitar a { $count }
edit-project-you-are = En este proyecto eres { $name }
edit-project-no-identity = Aún no has elegido quién eres
edit-project-switch = Cambiar
edit-project-choose = Elegir
invite-failed = No se pudieron enviar estas invitaciones: { $emails }
invite-waiting = Estos amigos tienen que abrir Counted una vez antes de poder ser invitados: { $emails }
key-offer-title = Otra clave para { $name }
key-offer-body = Este dispositivo tiene otra clave para este proyecto, que nunca lo ha abierto aquí.
key-offer-choices = Usar esta clave: el proyecto se abre con la clave de este enlace o invitación. Cancelar: el dispositivo conserva su clave actual y nada cambia.
key-offer-use = Usar esta clave
invite-again = Invitar de nuevo
friend-picker-title = Añadir amigos
user-selection-invited-hint = { $email } te ha invitado a «{ $project }».
user-selection-suggested = Sugerido
user-selection-suggested-sub = { $email } te añadió con este nombre
user-selection-confirm-as = Soy { $name }
user-selection-missing = ¿No está tu nombre? Pide a un participante que te añada en los ajustes del proyecto.

## Gastos recurrentes

repeat-label = Repetir
repeat-none = No se repite
repeat-weekly = Cada semana
repeat-biweekly = Cada 2 semanas
repeat-monthly = Cada mes
repeat-quarterly = Cada 3 meses
repeat-yearly = Cada año
repeat-every-weeks = Cada { $count } semanas
repeat-every-months = Cada { $count } meses
repeat-every-years = Cada { $count } años
repeat-custom = Personalizado…
repeat-every = Cada
repeat-unit-weeks = semanas
repeat-unit-months = meses
repeat-unit-years = años
repeat-on-weekday = el { $weekday }
repeat-on-day = el día { $day }
repeat-on-day-month = el { $day } de { $month }
repeat-month-end = En los meses más cortos cae el último día.
repeat-ends = Termina
repeat-ends-never = Nunca
repeat-ends-on = En una fecha
repeat-ends-after = Tras
repeat-fewer = Menos
repeat-more = Más
repeat-last-on = el último el { $date }
repeat-variable = El importe cambia cada vez
repeat-variable-hint = Cada uno se añade con el último importe y marcado como «por confirmar».
repeat-done = Listo
repeat-no-end = Sin fin
repeat-until = Hasta el { $date }
repeat-occurrences = Repeticiones: { $count }
repeat-offline = Necesita conexión. El gasto se puede añadir igualmente.
repeat-foreign = Se repite por { $amount } { $currency }, convertido una vez al cambio de hoy. Se te avisará si el cambio varía más de un 5 %.
repeat-backfill = Empieza en el pasado. Gastos añadidos ahora: { $count }.
add-and-repeat = Añadir y repetir
weekday-1 = lunes
weekday-2 = martes
weekday-3 = miércoles
weekday-4 = jueves
weekday-5 = viernes
weekday-6 = sábado
weekday-7 = domingo
recurring-title = Gastos recurrentes
recurring-strip = Gastos recurrentes: { $count }
recurring-next = Próximo: { $name }, { $date }
recurring-to-confirm = Por confirmar: { $count }
recurring-per-month = Al mes, aprox.
recurring-your-share = Tu parte
recurring-active = Activos
recurring-paused = En pausa
recurring-finished = Terminados
recurring-paid-by = pagado por { $name }
recurring-next-on = Próximo el { $date }
recurring-progress = { $done } de { $total }
recurring-rate-badge = Cambio movido un { $percent } %
recurring-empty = Aún no se repite nada. Elige «Repetir» al añadir un gasto: alquiler, suscripciones, facturas.
recurring-next-ones = Próximos
recurring-added-so-far = Añadidos hasta ahora
recurring-set-up-by = Creado por
recurring-pause = Pausar
recurring-resume = Reanudar
recurring-stop = Dejar de repetir
recurring-stop-title = ¿Dejar de repetir «{ $name }»?
recurring-stop-message = Ya no se repetirá. Los gastos ya añadidos se mantienen.
recurring-resume-title = ¿Reanudar «{ $name }»?
recurring-resume-message = Próximo el { $date }. Las fechas perdidas durante la pausa no se añaden.
recurring-edit-title = Editar gasto recurrente
recurring-edit-banner = Los cambios se aplican desde el { $date }. Los gastos ya añadidos se quedan como están.
recurring-next-on-label = Próximo el
recurring-next-too-early = La próxima fecha debe ser posterior al último gasto ya añadido.
recurring-use-stop = Para terminarlo, usa «Dejar de repetir» en el gasto recurrente.
recurring-drift = El cambio de { $currency } se ha movido un { $percent } % desde que se creó. Cada uno se sigue añadiendo por { $amount } { $project_currency } (1 { $currency } = { $rate }). Al cambio de hoy serían { $today_amount } { $project_currency }.
recurring-use-rate = Usar el cambio de hoy
recurring-keep = Mantener { $amount } { $currency }
recurring-added = Gastos recurrentes añadidos: { $count }
recurring-blocks-removal = { $name } aún no se puede quitar: forma parte de { $rules }. Quita a { $name } de ellos o detenlos y vuelve a guardar.
history-recurring-added = Añadido automáticamente: { $name } ({ $date })
history-recurring-created = Gasto recurrente creado: { $name }
history-recurring-edited = Gasto recurrente editado: { $name }
history-recurring-paused = Gasto recurrente pausado: { $name }
history-recurring-resumed = Gasto recurrente reanudado: { $name }
history-recurring-stopped = Gasto recurrente detenido: { $name }
occurrence-recurring = Gasto recurrente
occurrence-auto = Añadido automáticamente por un gasto recurrente.
occurrence-auto-next = Añadido automáticamente por un gasto recurrente. Próximo el { $date }.
occurrence-auto-stopped = Añadido automáticamente por un gasto recurrente que ya se ha detenido.
occurrence-manage = Gestionar
estimate-badge = Por confirmar
expense-pending = Pendiente
estimate-title = Importe por confirmar.
estimate-body = Añadido con el importe anterior. Introduce el real cuando lo tengas.
estimate-confirm = Confirmar importe
apply-to = Aplicar a
apply-this-only = Solo este gasto
apply-and-next = Este y los siguientes
apply-and-next-hint = El gasto recurrente cambia a partir del { $date }
apply-rule-failed = El gasto se ha guardado, pero el gasto recurrente no se ha modificado.
occurrence-delete-message = «{ $name }» del { $date } se eliminará definitivamente. Se sigue repitiendo y esta fecha no volverá.
occurrence-delete-one = Eliminar solo este
occurrence-delete-stop = Eliminar y dejar de repetir
error-recurring-clock = El reloj de este dispositivo va adelantado. Revisa la fecha y la hora.
error-recurring-not-found = Este gasto recurrente ya no existe.
error-recurring-stale = Alguien ha modificado este gasto recurrente mientras tanto. Se ha recargado: revísalo y vuelve a guardar.
error-too-many-recurring = Este proyecto ya tiene 50 gastos recurrentes. Detén uno que ya no necesites para añadir otro.
error-user-in-recurring = Este participante forma parte de un gasto recurrente. Quítalo de él o detenlo primero.

home-owed-to-you = Te deben
home-you-owe = Debes
home-details-show = Ver detalle
home-details-hide = Ocultar detalle
home-all-settled = Todo saldado
home-pay = Pagar
home-remind = Recordar
home-remind-message = Pequeño recordatorio: quedan reembolsos pendientes en { $project }.
home-row-owed = te deben
home-row-owe = debes
home-row-settled = Al día
home-row-quits = Estás al día
home-row-pending =
    { $count ->
        [one] { $count } reembolso pendiente
       *[other] { $count } reembolsos pendientes
    }
home-totals-partial = Sin los proyectos sin tipo de cambio
error-account-key-changed = La clave de tu cuenta cambió en otro dispositivo. Vuelve a iniciar sesión para continuar.
settings-security = Seguridad
settings-change-password-hint = Tus proyectos y tus datos se quedan como están. Se cierra la sesión en tus otros dispositivos.
change-password-title = Cambia tu contraseña
change-password-current = Contraseña actual
change-password-new = Nueva contraseña
change-password-confirm = Confirma la nueva contraseña
change-password-mismatch = Las dos contraseñas nuevas no coinciden.
change-password-submit = Cambiar la contraseña
change-password-submitting = Cambiando…
change-password-done = Contraseña cambiada. Se cerró la sesión en tus otros dispositivos.
error-rotation-incomplete = Tu cuenta cambió mientras se hacía esto. Vuelve a intentarlo.
error-key-lifecycle-unavailable = Aún no está disponible: primero todos necesitan la última versión de la app.
settings-compromised-open = Proteger mi cuenta
compromised-title = Protege tu cuenta
compromised-body = Se reemplazan tu contraseña y las claves de tu cuenta, y se cierra la sesión en los demás dispositivos. Tus amigos verán que tu código de seguridad cambia: compáralo de nuevo con ellos.
compromised-keeps = Quien tenía tu contraseña conserva lo que ya abrió: los proyectos que podía leer hasta ahora.
compromised-submit = Reemplazar contraseña y claves
compromised-done = Tu cuenta está protegida. Pide a tus amigos que comparen tu nuevo código de seguridad.
friends-code-matches = El código coincide
friends-code-trusted = Código confirmado.
settings-recovery-key = Clave de recuperación
settings-recovery-key-pending = Una clave nueva la reemplaza el { $when }. ¿No fuiste tú? Cancélala con tu clave actual.
recovery-key-cancel-replacement = Cancelar la clave nueva
recovery-key-title = Tu clave de recuperación
recovery-key-body = Apúntala o guárdala donde solo tú puedas acceder. Counted no puede volver a mostrarla ni recuperarla por ti.
recovery-key-copy = Copiar
recovery-key-copied = Copiada
recovery-key-save = Guardar como archivo
recovery-key-print = Imprimir
recovery-key-next = Continuar
recovery-key-confirm-body = Para comprobar tu copia, escribe el grupo { $n } de la clave.
recovery-key-confirm-wrong = Ese grupo no coincide. Vuelve a mirar tu copia.
recovery-key-password-body = Escribe tu contraseña para guardar la clave en tu cuenta.
recovery-key-submit = Guardar la clave
recovery-key-done = Clave de recuperación guardada.
recovery-key-replacement-done = Clave nueva guardada. Reemplaza la actual en 72 horas; hasta entonces la actual sigue funcionando.
recovery-key-field = Clave de recuperación
recovery-key-file-intro = Clave de recuperación de Counted
recovery-key-cancel-done = La clave nueva se ha cancelado.
compromised-new-key = Guarda ahora tu nueva clave de recuperación: reemplaza la antigua en 72 horas, y hasta entonces la antigua sigue funcionando.
pending-reset-title = Restablecimiento solicitado
pending-reset-body = Se aplica el { $when }. ¿No fuiste tú? Cancélalo con tu contraseña.
pending-reset-cancel = Cancelar el restablecimiento
pending-reset-cancelled = El restablecimiento se ha cancelado.
login-forgot = ¿Olvidaste la contraseña?
forgot-title = Has olvidado tu contraseña
forgot-body = Escribe tu correo. Si tiene una cuenta, te enviamos un enlace para elegir una nueva contraseña.
forgot-submit = Enviar el enlace
forgot-sent = Si esta dirección tiene una cuenta, el enlace va de camino. Caduca en 24 horas.
reset-title = Recupera tu cuenta
reset-with-key-body = Escribe tu clave de recuperación y una nueva contraseña: todo vuelve como estaba.
reset-submit-key = Recuperar mi cuenta
reset-no-key = No tengo mi clave de recuperación
reset-without-key-title = Restablecer sin la clave de recuperación
reset-without-key-body = Tu cuenta empieza de nuevo con una contraseña nueva en 72 horas. Tus proyectos vuelven desde tus dispositivos y los enlaces compartidos; tus preferencias, datos de pago y códigos de amigos, no. Hasta entonces tu contraseña actual sigue funcionando y puede cancelar el reinicio.
reset-name = Tu nombre
reset-submit-request = Restablecer mi cuenta
reset-requested = Hecho. Tu nueva contraseña funciona a partir del { $when }. Hasta entonces la actual sigue funcionando y puede cancelarlo.
reset-pending-already = Ya hay un restablecimiento pendiente hasta el { $when }. Una nueva solicitud reinicia las 72 horas.
recovered-done = Has recuperado tu cuenta. Se cerró la sesión en tus otros dispositivos.
login-reset-undoable = Esta contraseña fue reemplazada por un restablecimiento en los últimos 30 días. Si no lo pediste, deshazlo: tu cuenta vuelve como estaba.
login-undo-reset = Deshacer el restablecimiento
reset-undone = Se ha deshecho el restablecimiento. Tu cuenta vuelve a estar como antes.
error-reset-pending = Esta contraseña nueva funciona cuando se aplique el restablecimiento, 72 horas después de pedirlo.
error-account-disabled = Esta cuenta se eliminó poco después de un restablecimiento y está pendiente de borrarse. La contraseña anterior todavía puede deshacerlo.
error-recovery-key-invalid = Esta clave de recuperación no abre esta cuenta.
error-project-key-required = Este proyecto necesita su clave. Vuelve a abrirlo desde su enlace compartido.
names-or = { $first } o { $last }
participants-admin-badge = Admin
edit-project-admins = Admins: { $count }
edit-project-succession = Si el proyecto se queda sin admin, pasa a serlo el miembro con cuenta que lleve más tiempo en él.
member-sheet-account = Miembro con cuenta
member-actions = Acciones para { $name }
member-make-admin = Hacer admin
member-make-admin-hint = Podrá cambiar la clave del proyecto y quitar a alguien.
member-made-admin = { $name } ahora es admin.
group-security = Seguridad del proyecto
group-security-admin = Seguridad del proyecto · eres admin
group-rotate = Cambiar la clave del proyecto
group-rotate-hint = Si el enlace del proyecto se ha filtrado: quienes tengan el enlace antiguo perderán el acceso al proyecto.
group-remove = Quitar a alguien del proyecto
group-remove-hint = Le quita el acceso y cambia la clave a la vez.
group-admins-only = Solo para admins.
group-admins-only-account = Solo para admins, que tienen cuenta.
group-ask-admin = Pregunta a { $names }, admin del proyecto.
group-ask-admins = Pregunta a { $names }, que administran el proyecto.
group-ask-an-admin = Pregunta a alguien que administre el proyecto.
group-with-account = Con cuenta
group-free = Gratis
group-sign-in-hint = { $names }, que administra el proyecto, podrá darte el rol después.
group-sign-in-hint-any = Alguien que administre el proyecto podrá darte el rol después.
group-no-admin = Este proyecto no tiene admin, así que su clave no puede cambiar.
group-hide = Ocultar a alguien
group-hide-hint = Oculta a la persona de las listas sin quitarle el acceso: el enlace y la clave siguen valiendo.
group-recreate-hint = Este proyecto no tiene admin. Para quitar a alguien, crea un proyecto nuevo.
group-recreate = Crear un proyecto nuevo
group-no-admin-yet = Este proyecto aún no tiene admin.
group-creator-hint = Solo tú puedes ser admin de este proyecto, creando una cuenta.
group-creator-elsewhere-hint = Solo la persona que creó el proyecto puede ser su admin, creando una cuenta. Si no, crea un proyecto nuevo para quitar a alguien.
member-revoke-admin = Quitar el rol de admin
member-revoke-admin-hint = Ya no podrá cambiar la clave del proyecto ni quitar a nadie.
member-revoked-admin = { $name } ya no es admin.
member-step-down = Dejar de ser admin
member-step-down-hint = Ya no podrás cambiar la clave del proyecto ni quitar a nadie.
member-stepped-down = Ya no eres admin.
member-remove = Quitar del proyecto
member-remove-hint = Cambia la clave del proyecto: { $name } ya no podrá leer nada nuevo.
member-hide = Ocultar de las listas
error-user-not-admin = Este miembro no es admin.
error-creator-proof-invalid = Este dispositivo no creó este proyecto.
error-project-has-admin = Este proyecto ya tiene admin.
error-not-project-admin = Solo quienes administran el proyecto pueden hacer esto.
error-user-not-claimed = Esta persona no tiene cuenta, así que no puede ser admin.
error-project-key-rotated = La clave del proyecto ha cambiado. Pide el enlace nuevo a un miembro.
error-project-rotating = Se está cambiando la clave del proyecto. Vuelve a intentarlo en un momento.
error-rotation-expired = Este cambio de clave tardó demasiado y se canceló. Empieza de nuevo.
error-rotation-rows-changed = El proyecto cambió durante el cambio de clave. Empieza de nuevo.
project-rotated-title = Ya no tienes acceso a este proyecto
project-rotated-body = Se ha cambiado la clave de este proyecto. Pide el enlace nuevo a quien lo administra.
project-rotated-open-link = Abrir un enlace
rotation-remove-title = ¿Quitar a { $name } del proyecto?
rotation-rotate-title = ¿Cambiar la clave del proyecto?
rotation-remove-body = Quitar a un miembro implica generar una clave nueva. Tendrás que compartir el enlace nuevo con las personas que no tienen cuenta.
rotation-rotate-body = Con el enlace anterior ya no se podrá acceder al proyecto. Tendrás que compartir el enlace nuevo con las personas que no tienen cuenta.
rotation-boxed =
    { $count ->
        [0] Ningún otro miembro recibirá la clave automáticamente
        [one] { $count } miembro recibirá la clave automáticamente
       *[other] { $count } miembros recibirán la clave automáticamente
    }
rotation-by-link-title = Personas a las que tendrás que enviar el enlace nuevo
rotation-by-link-hint = Estas personas no tienen cuenta. Les enviarás el enlace justo después.
rotation-names-and = { $first } y { $last }
rotation-names-and-others = { $names } y { $count } más
rotation-confirm-remove = Quitar y cambiar la clave
rotation-confirm-rotate = Cambiar la clave
rotation-done-title = Nueva clave activa
rotation-done-removed = { $name } ya no forma parte del proyecto.
rotation-done-rotated = La clave del proyecto ha cambiado.
rotation-done-boxed =
    { $count ->
        [0] Ningún otro miembro ha recibido la clave automáticamente.
        [one] { $count } miembro ha recibido la clave automáticamente.
       *[other] { $count } miembros han recibido la clave automáticamente.
    }
rotation-new-link =
    { $count ->
        [0] Enlace nuevo del proyecto
        [one] Enlace nuevo para enviar · { $count } persona sin cuenta
       *[other] Enlace nuevo para enviar · { $count } personas sin cuenta
    }
rotation-link-renewed = El enlace se ha renovado correctamente.
rotation-done = Listo
history-key-changed = Clave del proyecto cambiada
history-member-removed = { $name } ya no forma parte del proyecto
project-rotated-row-hint = Su clave ha cambiado. Pide el enlace nuevo.

## Settings and Friends redesign (docs/plans/friends-settings-redesign.md)
settings-group-sharing = Compartir
settings-group-help = Ayuda e información
settings-friends-changed = { $count ->
    [one] 1 código cambiado
   *[other] { $count } códigos cambiados
}
settings-recovery-key-created = Creada
settings-recovery-key-missing = Por crear
settings-recovery-key-replacing = Sustitución pendiente
settings-password = Contraseña
settings-compromised-short = ¿Alguien conoce tu contraseña?
settings-recovery-key-footnote = Sin clave de recuperación, olvidar la contraseña borra tus preferencias, tus datos de pago y los códigos de tus amigos.
erase-device-row = Borrar este dispositivo
erase-device-row-hint = Cierra tu sesión y solo conserva el idioma.
account-delete-hint = Inmediato y definitivo.
settings-open-source = Código abierto, AGPL-3.0
settings-upsell-short = Sin cuenta, tus proyectos se quedan en este dispositivo. Con una, te siguen a todas partes, cifrados.
settings-locked-group = Con una cuenta
settings-locked-friends-short = Invita sin enlace
settings-locked-payment-short = Lo ve quien te debe dinero
payment-methods-title = Métodos de pago
payment-methods-row-hint = Compartidos con tus amigos
payment-methods-intro = Compartidos con tus amigos. Cifrados con tu cuenta.
payment-methods-footnote = Un método de pago se comparte (máx. 20) con los miembros de tus proyectos en la pestaña Saldar cuentas.
payment-method-shared-pill = Compartido
payment-method-share-groups = Compartir en tus grupos
payment-method-label-optional = Nombre (opcional)
payment-method-delete-confirm-title = ¿Eliminar este método de pago?
payment-method-delete-confirm-message = { $name } ya no se mostrará en tus proyectos.
payment-methods-empty-title = Ningún método de pago
payment-methods-empty-body = Añade tu IBAN o tu app de pago: quien te deba dinero lo verá.
payment-method-masked = { $kind } terminado en { $last }
friends-to-check = Por verificar
friends-code-changed-row = Su código cambió
friends-own-code = Tu código de seguridad
friends-own-code-hint = Tus amigos ven este código junto a tu dirección. Si coincide con el suyo, te están invitando a ti y a nadie más.
friends-received = Recibidas
friends-received-on = Recibida el { $date }
friends-pending = Pendiente
friends-from-project = Desde un proyecto
friends-accept-label = Aceptar la solicitud de { $email }
friends-decline-label = Rechazar la solicitud de { $email }
friends-add-hint-short = Esta persona verá tu solicitud si tiene cuenta. Si no, avísale.
friends-send = Enviar
friends-sent-hint = ¿Aún no tiene cuenta? Avísale: el mensaje la invita a registrarse con esta dirección.
friends-done = Listo
friends-their-code = Su código de seguridad
friends-code-compare-hint = Leedlo juntos. Debe ver este mismo código arriba en su página Amigos.
friends-no-key-hint = Aún no está listo: su clave se crea la próxima vez que abra Counted. Hasta entonces, no puedes invitarlo.
friends-remove-row = Quitar de mis amigos
friends-new-code = Código nuevo
friends-key-changed-hint-short = Comprobad este código juntos, fuera de esta aplicación. Hasta entonces, las invitaciones con él están bloqueadas.
friends-kem-changed-title = Su clave fue sustituida
friends-kem-changed-body = La parte poscuántica de su clave cambió, pero no su código. Proteger una cuenta cambia ambos, así que alguien pudo sustituir esta clave. Las invitaciones con él siguen bloqueadas.
friends-kem-changed-hint = Puedes quitarlo y luego enviarle una nueva solicitud: su clave se fijará de nuevo.
friends-pending-sheet = Solicitud enviada, pendiente.
friends-withdraw-row = Cancelar la solicitud
friends-empty-title = Aún no tienes amigos
friends-empty-body = Añade a alguien por correo y luego invítalo a un proyecto sin compartir un enlace.
friends-requests-empty-title = Ninguna solicitud
friends-requests-empty-body = Aquí aparecen las solicitudes que recibes y envías.
friends-code-hidden = Tu código está oculto.
friends-code-hidden-body = Una clave servida para tu cuenta no coincide con este dispositivo. Invitaciones desactivadas aquí.
notifications-manage-friends = Gestionar mis amigos
notifications-friend-request-on = Solicitud de amistad · { $date }

### Link preview

share-preview-title = Nuestros gastos en Counted
share-preview-description = Abre el enlace para ver los gastos y quién debe qué. Sin cuenta, cifrado en tu dispositivo.

### Currency names

currency-EUR = Euro
currency-AED = Dírham de los Emiratos Árabes Unidos
currency-AFN = Afgani afgano
currency-ALL = Lek albanés
currency-AMD = Dram armenio
currency-AOA = Kuanza angoleño
currency-ARS = Peso argentino
currency-AUD = Dólar australiano
currency-AWG = Florín arubeño
currency-AZN = Manat azerbaiyano
currency-BAM = Marco convertible de Bosnia y Herzegovina
currency-BBD = Dólar barbadense
currency-BDT = Taka bangladesí
currency-BHD = Dinar bareiní
currency-BIF = Franco burundés
currency-BMD = Dólar bermudeño
currency-BND = Dólar bruneano
currency-BOB = Boliviano
currency-BRL = Real brasileño
currency-BSD = Dólar bahameño
currency-BTN = Gultrum butanés
currency-BWP = Pula botsuano
currency-BYN = Rublo bielorruso
currency-BZD = Dólar beliceño
currency-CAD = Dólar canadiense
currency-CDF = Franco congoleño
currency-CHF = Franco suizo
currency-CLP = Peso chileno
currency-CNY = Yuan renminbi
currency-COP = Peso colombiano
currency-CRC = Colón costarricense
currency-CUP = Peso cubano
currency-CVE = Escudo de Cabo Verde
currency-CZK = Corona checa
currency-DJF = Franco yibutiano
currency-DKK = Corona danesa
currency-DOP = Peso dominicano
currency-DZD = Dinar argelino
currency-EGP = Libra egipcia
currency-ERN = Nakfa eritreo
currency-ETB = Bir etíope
currency-FJD = Dólar fiyiano
currency-FKP = Libra malvinense
currency-GBP = Libra esterlina
currency-GEL = Lari georgiano
currency-GHS = Cedi ghanés
currency-GIP = Libra gibraltareña
currency-GMD = Dalasi gambiano
currency-GNF = Franco guineano
currency-GTQ = Quetzal guatemalteco
currency-GYD = Dólar guyanés
currency-HKD = Dólar hongkonés
currency-HNL = Lempira hondureño
currency-HTG = Gurde haitiano
currency-HUF = Forinto húngaro
currency-IDR = Rupia indonesia
currency-ILS = Nuevo séquel israelí
currency-INR = Rupia india
currency-IQD = Dinar iraquí
currency-IRR = Rial iraní
currency-ISK = Corona islandesa
currency-JMD = Dólar jamaicano
currency-JOD = Dinar jordano
currency-JPY = Yen japonés
currency-KES = Chelín keniano
currency-KGS = Som kirguís
currency-KHR = Riel camboyano
currency-KMF = Franco comorense
currency-KPW = Won norcoreano
currency-KRW = Won surcoreano
currency-KWD = Dinar kuwaití
currency-KYD = Dólar de las Islas Caimán
currency-KZT = Tengue kazajo
currency-LAK = Kip laosiano
currency-LBP = Libra libanesa
currency-LKR = Rupia esrilanquesa
currency-LRD = Dólar liberiano
currency-LSL = Loti lesotense
currency-LYD = Dinar libio
currency-MAD = Dírham marroquí
currency-MDL = Leu moldavo
currency-MGA = Ariari malgache
currency-MKD = Dinar macedonio
currency-MMK = Kiat de Myanmar
currency-MNT = Tugrik mongol
currency-MOP = Pataca macaense
currency-MRU = Uguiya mauritano
currency-MUR = Rupia mauriciana
currency-MVR = Rufiya maldiva
currency-MWK = Kuacha malauí
currency-MXN = Peso mexicano
currency-MYR = Ringit malasio
currency-MZN = Metical mozambiqueño
currency-NAD = Dólar namibio
currency-NGN = Naira nigeriano
currency-NIO = Córdoba oro
currency-NOK = Corona noruega
currency-NPR = Rupia nepalí
currency-NZD = Dólar neozelandés
currency-OMR = Rial omaní
currency-PAB = Balboa panameño
currency-PEN = Sol peruano
currency-PGK = Kina papú
currency-PHP = Peso filipino
currency-PKR = Rupia pakistaní
currency-PLN = Esloti polaco
currency-PYG = Guaraní paraguayo
currency-QAR = Rial catarí
currency-RON = Leu rumano
currency-RSD = Dinar serbio
currency-RUB = Rublo ruso
currency-RWF = Franco ruandés
currency-SAR = Rial saudí
currency-SBD = Dólar salomonense
currency-SCR = Rupia seychellense
currency-SDG = Libra sudanesa
currency-SEK = Corona sueca
currency-SGD = Dólar singapurense
currency-SHP = Libra de Santa Elena
currency-SLE = Leona sierraleonesa
currency-SOS = Chelín somalí
currency-SRD = Dólar surinamés
currency-SSP = Libra sursudanesa
currency-STN = Dobra santotomense
currency-SVC = Colón salvadoreño
currency-SYP = Libra siria
currency-SZL = Lilangeni esuatiní
currency-THB = Bat tailandés
currency-TJS = Somoni tayiko
currency-TMT = Manat turcomano
currency-TND = Dinar tunecino
currency-TOP = Paanga tongano
currency-TRY = Lira turca
currency-TTD = Dólar de Trinidad y Tobago
currency-TWD = Nuevo dólar taiwanés
currency-TZS = Chelín tanzano
currency-UAH = Grivna ucraniana
currency-UGX = Chelín ugandés
currency-USD = Dólar estadounidense
currency-UYU = Peso uruguayo
currency-UZS = Sum uzbeko
currency-VES = Bolívar venezolano
currency-VND = Dong vietnamita
currency-VUV = Vatu vanuatense
currency-WST = Tala samoano
currency-XAF = Franco CFA de África Central
currency-XCD = Dólar del Caribe Oriental
currency-XCG = Florín caribeño
currency-XOF = Franco CFA de África Occidental
currency-XPF = Franco CFP
currency-YER = Rial yemení
currency-ZAR = Rand sudafricano
currency-ZMW = Kuacha zambiano
currency-ZWG = Zimbabwe Gold
