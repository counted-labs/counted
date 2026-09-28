# Español. Completo salvo los textos legales (legal-, terms-, privacy-), que solo existen en inglés
# y francés y recurren a en.ftl mensaje a mensaje.

### Common

loading = Cargando…
cancel = Cancelar
retry = Reintentar
delete = Eliminar
back = Volver
language = Idioma

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
error-generic = Se ha producido un error. Inténtalo de nuevo.

error-invalid-email = Esta dirección de correo no es válida.
error-invalid-password = Esta contraseña no es válida.
error-password-too-short = La contraseña debe tener al menos 8 caracteres.
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
login-resend = Reenviar el enlace de verificación
login-resending = Enviando…
login-resend-sent = Correo enviado. Revisa tu bandeja de entrada.

register-submit = Crear una cuenta
register-submitting = Creando…
register-have-account = ¿Ya tienes cuenta?
register-password-placeholder = Mínimo 8 caracteres
register-password-warning = Apunta tu contraseña. Si la olvidas, no será posible recuperar tu cuenta.
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
settings-about = Acerca de
settings-anonymous-title = No has iniciado sesión
settings-upsell-title = Tus proyectos, en todos tus dispositivos
settings-upsell-free = Gratis
settings-upsell-body = Counted funciona sin cuenta. Con una cuenta gratuita, tus proyectos y preferencias te siguen en el móvil, el ordenador y la web: siempre cifrados, siempre ilegibles para nosotros.
settings-locked-badge = Cuenta
settings-locked-friends = Crea una cuenta para añadir amigos e invitarlos a un proyecto desde la app, sin enlaces que compartir.
settings-locked-payment-methods = Guarda tu IBAN o tu app de pago una sola vez y compártelos con los proyectos que elijas. Quien te deba dinero los verá junto a tu nombre.
settings-friends-hint = Añade amigos e invítalos a tus proyectos sin compartir ningún enlace.

account-member-since = Miembro desde
account-logout = Cerrar sesión
account-logging-out = Cerrando sesión…
account-delete-title = Eliminar mi cuenta
account-delete-warning = Eliminación inmediata y definitiva, sin papelera. Los gastos que hayas registrado en un proyecto compartido siguen siendo visibles para el resto: forman parte de sus cuentas.
account-delete-confirm-title = Eliminar la cuenta
account-delete-confirm-message = Tu cuenta, tus sesiones y tu lista de proyectos se eliminarán de forma permanente. Sin tu contraseña, los datos cifrados de un proyecto compartido dejarán de ser legibles para ti. Esta acción no se puede deshacer.

settings-payment-methods = Datos de pago
settings-payment-methods-hint = Cómo quieres que te devuelvan el dinero. Cifrados con tu cuenta.
payment-method-kind = Método
payment-method-kind-other = Otro
payment-method-label = Nombre
payment-method-label-placeholder = Cuenta principal
payment-method-value = Datos
payment-method-value-placeholder = IBAN, número de teléfono, usuario…
payment-method-add = Añadir
payment-method-remove = Eliminar { $name }
payment-method-empty = Todavía no has añadido ningún dato de pago.
payment-method-deleted = Método de pago eliminado.
payment-method-value-required = Rellena los datos de cada método de pago, o elimínalo.
payment-method-label-required = Ponle un nombre a tu método personalizado.
payment-method-too-long = Es demasiado largo: acórtalo.
payment-method-invalid-characters = Quita los saltos de línea o los caracteres invisibles.
payment-method-limit = Puedes guardar hasta { $max } métodos de pago.
payment-methods-saved = Datos de pago guardados.
payment-methods-offline = Necesitas estar en línea para guardar tus datos de pago.
payment-methods-stale = Tus datos de pago se modificaron en otro dispositivo. Se han vuelto a cargar — inténtalo de nuevo.
payment-methods-key-missing = Vuelve a iniciar sesión para gestionar tus datos de pago.

verify-email-checking = Verificando tu correo…
verify-email-welcome = Correo verificado. ¡Bienvenido a Counted!
verify-email-back-to-login = Volver al inicio de sesión

### Project status

project-close = Cerrar
project-archive = Archivar
project-reopen = Reabrir
project-unarchive = Desarchivar

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
projects-empty-hint = Crea un proyecto con el botón de abajo
projects-offline-banner = Datos sin conexión: vuelve a conectarte para actualizar.
projects-no-local-data = Sin datos locales
projects-no-local-data-hint = Inicia sesión para cargar tus proyectos por primera vez.
projects-add = Añadir un proyecto
projects-create = Crear un proyecto
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
add-project-remove-participant = Quitar participante
add-project-me-badge = Yo
add-project-thats-me = ¡Soy yo!
add-project-offline = No puedes crear un proyecto sin conexión. Vuelve a conectarte e inténtalo de nuevo.
add-project-name-required = El proyecto necesita un nombre.
add-project-need-two-participants = Añade al menos 2 participantes.
add-project-pick-yourself = Indica qué participante eres.

join-link-label = Enlace para compartir
join-link-hint = El enlace contiene la clave de descifrado: cópialo entero.
join-invalid-link = Ese enlace no es válido. Pega el enlace completo, incluida la parte después del #.
join-wrong-project = Ese enlace corresponde a otro proyecto.

import-tricount-link-label = Enlace o clave de Tricount
import-tricount-key-required = Introduce un enlace o una clave de Tricount.
import-tricount-encryption-failed = Error de cifrado.

### Expenses

save = Guardar
saving = Guardando…
adding = Añadiendo…
link-copied = Enlace copiado
missing-encryption-key = Falta la clave de cifrado.
missing-encryption-key-title = Falta la clave de cifrado
missing-encryption-key-hint = El enlace que has usado no lleva la clave necesaria para descifrar este proyecto. Usa el enlace completo que compartió quien lo creó.
project-locked-hint = Este dispositivo no tiene la clave de este proyecto. Abre su enlace para desbloquearlo.
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
reimbursements-empty-hint = Aquí aparecerán sugerencias de pago cuando las cuentas no cuadren
reimbursement-owes = { $debtor } debe a { $creditor }

user-selection-title = ¿Qué participante eres?
user-selection-hint = Elige tu nombre en la lista.
user-selection-required = Selecciona un participante.

edit-project-deferred-new-members = añadir participantes nuevos
edit-project-deferred-removals = eliminar participantes
edit-project-deferred-me = la selección de «Soy yo»
edit-project-offline-deferred = Sin conexión: { $items } se aplicará cuando vuelvas a conectarte.

export-saved = Archivo guardado:
    { $path }
export-failed = Error al exportar: { $reason }

history-expense-added = Gasto añadido: { $name }
history-expense-edited = Gasto editado: { $name }
history-expense-deleted = Gasto eliminado: { $name }
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
expense-inconsistent-detail = Importes incoherentes: { $paid } pagado, { $owed } debido, para un gasto de { $total }. Edita el gasto para corregirlo.
missing-access-key = Falta la clave de acceso. Abre este proyecto desde su enlace para compartir.
filter-all = Todo
filter-my-payments = Mis pagos
filter-my-debts = Lo que debo
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
charts-my-share-note = Estas cifras son tu parte de cada gasto.
charts-my-share-skipped =
    { $count ->
        [one] 1 proyecto no se cuenta — no has elegido participante, o sus datos no se han cargado.
       *[other] { $count } proyectos no se cuentan — no has elegido participante, o sus datos no se han cargado.
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
history-empty = Sin eventos
not-found-hint = Esta página no existe o se ha movido.
payers-title-paid-by = Pagado por
payers-title-sender = Emisor
payers-title-contributors = Contribuyentes
debtors-title-debtors = Deudores
debtors-title-recipients = Destinatarios
debtors-title-beneficiaries = Beneficiarios

### Welcome

welcome-title = Tus cuentas solo te incumben a ti.
welcome-subtitle = Reparte gastos entre amigos.
welcome-e2ee-title = Datos totalmente cifrados
welcome-e2ee-body = Nombres, importes, proyectos: todo se cifra en tu dispositivo. Solo tú tienes la clave. Nadie puede leer tus cuentas. Ni siquiera nosotros.
welcome-e2ee-note = Indescifrable, incluso para nosotros (cero acceso del servidor)
welcome-eu-title = 100 % europeo
welcome-eu-body = Servidores en Alemania, correos enviados desde Francia. Tus datos nunca salen de la Unión Europea.
welcome-noads-title = Cero anuncios. Cero rastreadores.
welcome-noads-body = No recopilamos nada ni vendemos tus datos. Ese no es nuestro modelo.
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
help-forgot-password-warning = Tus datos se perderán definitivamente.
help-forgot-password-a = La clave de cifrado se deriva de tu contraseña, así que no hay restablecimiento posible: nadie, ni siquiera nosotros, puede descifrar tus proyectos sin ella. Guárdala bien, idealmente en un gestor de contraseñas.
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

settings-payment-methods-share-warning = Un método compartido es visible para todos los miembros de los proyectos en los que has elegido tu nombre - cualquiera que tenga uno de esos enlaces.
payment-method-share = Compartir con mis proyectos
payment-method-share-hint = Se muestra junto a tu nombre cuando alguien te debe dinero.
payment-method-copy = Copiar { $name }
payment-method-copied = Copiado.
payment-method-copy-failed = No se pudo copiar: selecciona el texto y cópialo a mano.

### Expenses (moneda, importe)

expense-category-auto = Auto · { $emoji }
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
project-currency-hint = Todos los importes se muestran en esta moneda. No se podrá cambiar después.
project-currency-locked = La moneda se fija al crear el proyecto.
project-gone-title = Este proyecto ya no existe
participants-by-shares = Por partes
split-amounts = Importes

### Reimbursements

reimbursements-empty-title = ¡Cuentas saldadas!
reimbursement-record = Saldar
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
charts-paid-more = { $amount } más que tu parte
charts-paid-less = { $amount } menos que tu parte
charts-paid-even = Justo tu parte
charts-part-title = Tu parte de cada categoría
charts-part-desc = En gris lo que gastó el grupo, en color lo que consumiste tú.
charts-breakdown-title = Desglose por categoría
charts-breakdown-desc = Toca una porción o una fila para ver sus gastos.
charts-of-total = { $amount } de { $total }
charts-show-all = Ver todo ({ $count })
charts-show-less = Ver menos
charts-spend-title = Gastos en el tiempo
charts-spend-desc = Los periodos cortos van por día, los largos por semana o mes.
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
charts-projects-desc = Los totales se separan por moneda y nunca se suman entre sí.
history-by = Por { $name }

### Notifications

notifications-label = Notificaciones
notifications-title = Notificaciones
notifications-empty = Nada nuevo
notifications-friend-request = Solicitud de amistad

### Friends

friends-title = Amigos
friends-anonymous-body = Los amigos están vinculados a tu cuenta. Inicia sesión para añadir personas e invitarlas a tus proyectos sin compartir un enlace.
friends-add-title = Añadir un amigo
friends-add-hint = Verá tu solicitud cuando inicie sesión. Ninguno de los dos sabrá si el otro tiene cuenta hasta que se acepte la solicitud.
friends-add-button = Añadir
friends-add-from-project = Añadir como amigo
friends-request-sent = Solicitud enviada
friends-no-account-key = Vuelve a iniciar sesión para gestionar tus amigos en este dispositivo.
friends-incoming-title = Solicitudes
friends-accept = Aceptar
friends-decline = Rechazar
friends-list-title = Mis amigos
friends-list-empty = Aún no tienes amigos. Añade a alguien por correo arriba, o desde un proyecto que compartáis.
friends-remove = Quitar
friends-remove-confirm-title = Quitar amigo
friends-remove-confirm-message = { $email } dejará de estar en tus amigos, y tú en los suyos. Cualquiera de los dos podrá enviar una nueva solicitud más tarde.
friends-no-key = Aún no está listo
friends-fingerprint = Código de seguridad
friends-fingerprint-hint = Dos amigos que se leen el mismo código de seguridad saben que nadie se interpone entre ellos, ni siquiera nuestro servidor.
friends-outgoing-title = Enviadas
friends-outgoing-hint = Esperando respuesta. Aparecerán en tus amigos cuando acepten.
friends-withdraw = Cancelar
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
