# Português. Completo exceto os textos legais (legal-, terms-, privacy-), que só existem em inglês
# e francês e recorrem a en.ftl mensagem a mensagem.

### Common

loading = A carregar…
cancel = Cancelar
discard-title = Descartar as alterações?
discard-message = O que escreveste vai perder-se.
discard = Descartar
retry = Tentar novamente
delete = Eliminar
back = Voltar
language = Idioma
theme = Tema
theme-light = Claro
theme-dark = Escuro
theme-system = Sistema
theme-hint-device = Guardado apenas neste dispositivo.
theme-hint-system = Segue o aspeto deste dispositivo.

### Navigation

nav-main = Navegação principal
nav-projects = Projetos
nav-charts = Estatísticas
nav-settings = Definições

### Connectivity

offline-banner = Offline
offline-pending =
    { $count ->
        [one] { $count } pendente
       *[other] { $count } pendentes
    }

sync-conflict-edit = Conflito: não foi possível editar «{ $name }» (elemento eliminado). Ignorado.
sync-conflict-delete = Conflito: não foi possível eliminar «{ $name }» (elemento eliminado). Ignorado.
sync-conflict-other = Conflito: a operação sobre «{ $name }» falhou (elemento eliminado). Ignorada.
sync-error = Erro de sincronização: { $reason }

### Errors

error-network = Não foi possível contactar o servidor. Verifica a tua ligação à internet.
error-rate-limited = O servidor está ocupado. Tenta novamente daqui a pouco.
error-generic = Ocorreu um erro. Tenta novamente.

error-invalid-email = Este endereço de e-mail não é válido.
error-invalid-password = Esta palavra-passe não é válida.
error-password-too-short = A palavra-passe tem de ter pelo menos 12 caracteres.
error-client-outdated = Esta versão da aplicação está desatualizada. Atualiza-a para iniciar sessão.
error-invalid-link = Esta ligação não é válida.
error-batch-too-large = Demasiados elementos de uma só vez.
error-payers-required = Seleciona pelo menos uma pessoa que pagou.
error-debtors-required = Seleciona pelo menos uma pessoa que deve.
error-duplicate-participant = Um participante aparece duas vezes do mesmo lado.
error-participant-not-in-project = Este participante não faz parte deste projeto.
error-too-many-participants = Demasiados participantes para uma só despesa.
error-invalid-credentials = E-mail ou palavra-passe incorretos.
error-unauthenticated = Inicia sessão para fazer isso.
error-email-not-verified = O teu endereço de e-mail ainda não foi verificado.
error-claim-proof-invalid = Este dispositivo não tem a chave do projeto, por isso não pode reivindicar um participante. Abre novamente a ligação de partilha.
error-project-not-found = Este projeto já não existe.
error-expense-not-found = Esta despesa já não existe.
error-storage-full = O armazenamento está cheio: não foi possível guardar a chave deste projeto neste dispositivo. Guarda a ligação de partilha.
error-user-not-found = Este participante já não existe.
error-tricount-not-found = Tricount não encontrado, ou a sua API devolveu um erro.
error-too-many-members = Este projeto atingiu o limite de participantes.
error-user-has-payments = Este participante tem despesas no projeto e não pode ser removido.
error-resend-cooldown = Aguarda 60 segundos antes de pedir outro e-mail.

### Auth

field-email = E-mail
field-email-placeholder = tu@exemplo.pt
field-password = Palavra-passe
field-name = Nome
field-name-placeholder = João Silva

login-title = Iniciar sessão
login-submit = Iniciar sessão
login-submitting = A iniciar sessão…
login-password-placeholder = A tua palavra-passe
login-no-account = Ainda não tens conta?
login-unverified = O teu e-mail ainda não foi verificado. Consulta a tua caixa de entrada ou pede um novo link.
login-maybe-unverified = Acabaste de te registar? Verifica primeiro o teu endereço com a ligação que te enviámos, ou envia-a de novo.
login-resend = Enviar novamente o link de verificação
login-resending = A enviar…
login-resend-sent = E-mail enviado - consulta a tua caixa de entrada.

register-submit = Criar uma conta
register-submitting = A criar…
register-have-account = Já tens conta?
register-password-placeholder = Pelo menos 12 caracteres
register-password-warning = Aponta a tua palavra-passe: nunca a vemos e não ta podemos devolver.
register-check-email-title = Consulta o teu e-mail
register-email-sent = E-mail enviado
register-email-sent-hint = Clica no link que recebeste para ativar a tua conta.
register-not-received-prefix = Não chegou? Verifica o spam ou
register-sign-in-link = inicia sessão
register-not-received-suffix = para enviar o link novamente.
register-terms-prefix = Ao criar uma conta aceitas os
register-terms-link = termos de utilização
register-terms-and = e a nossa
register-privacy-link = política de privacidade

settings-title = Definições
settings-preferences = Preferências
settings-preferences-local = Guardadas neste dispositivo.
settings-preferences-synced = Sincronizadas com a tua conta, cifradas.
settings-about = Acerca de
settings-anonymous-title = Não tens sessão iniciada
settings-upsell-title = Os seus projetos, em todos os dispositivos
settings-upsell-free = Grátis
settings-upsell-body = O Counted funciona sem conta. Com uma conta gratuita, os teus projetos e preferências acompanham-te no telemóvel, no computador e na web - sempre cifrados, sempre ilegíveis para nós.
settings-locked-badge = Conta
settings-locked-friends = Cria uma conta para adicionar amigos e convidá-los para um projeto a partir da app - sem links para passar.
settings-locked-payment-methods = Guarda o teu IBAN ou a tua app de pagamento uma vez e partilha-os com os projetos que escolheres. Quem te deve dinheiro vê-os ao lado do teu nome.
settings-friends-hint = Adiciona amigos e convida-os para os teus projetos sem partilhar nenhum link.

account-member-since = Membro desde
account-logout = Terminar sessão
account-logging-out = A terminar sessão…
account-delete-title = Eliminar a minha conta
account-delete-warning = Eliminação imediata e definitiva da tua conta e dos teus dados.
account-delete-confirm-title = Eliminar a conta
account-delete-confirm-message = A tua conta, as tuas sessões e a lista dos teus projetos serão eliminadas definitivamente. Sem a tua palavra-passe, os dados cifrados de um projeto partilhado deixam de ser legíveis para ti - esta ação é irreversível.
erase-device-open = Terminar sessão e apagar este dispositivo
erase-device-title = Apagar este dispositivo?
erase-device-body = Os projetos, as suas chaves e a cache deste dispositivo serão apagados e depois a sessão será terminada. Os teus projetos ficam na tua conta: inicia sessão de novo para os recuperares.
erase-device-checking = A verificar se os teus projetos estão na tua conta…
erase-device-pending = Alterações por enviar: { $count }. Volta a ligar-te à internet e tenta de novo.
erase-device-unreachable = Não foi possível verificar se os teus projetos estão na tua conta. Verifica a ligação à internet e tenta de novo.
erase-device-at-risk = Estes projetos não estão na tua conta e vão perder-se. Para os manter, cancela e copia primeiro a ligação de cada um:
erase-device-confirm = Apagar e terminar sessão
erase-device-confirm-anyway = Apagar mesmo assim
erase-device-erasing = A apagar…

settings-payment-methods = Dados de pagamento
settings-payment-methods-hint = Como preferes ser reembolsado. Cifrados com a tua conta.
payment-method-kind = Método
payment-method-kind-other = Outro
payment-method-label = Nome
payment-method-label-placeholder = Conta principal
payment-method-value = Dados
payment-method-value-placeholder = IBAN, número de telefone, nome de utilizador…
payment-method-add = Adicionar
payment-method-remove = Remover { $name }
payment-method-empty = Ainda não adicionaste dados de pagamento.
payment-method-deleted = Método de pagamento eliminado.
payment-method-value-required = Preenche os dados de cada método de pagamento, ou remove-o.
payment-method-label-required = Dá um nome ao teu método personalizado.
payment-method-too-long = É demasiado longo - encurta-o.
payment-method-invalid-characters = Remove as quebras de linha ou os caracteres invisíveis.
payment-method-limit = Podes guardar até { $max } métodos de pagamento.
payment-methods-saved = Dados de pagamento guardados.
payment-methods-offline = Tens de estar online para guardar os teus dados de pagamento.
payment-methods-stale = Os teus dados de pagamento foram alterados noutro dispositivo. Foram recarregados - tenta novamente.
payment-methods-key-missing = Inicia sessão novamente para gerir os teus dados de pagamento.

verify-email-checking = A verificar o teu e-mail…
verify-email-welcome = E-mail verificado - bem-vindo ao Counted!
verify-email-back-to-login = Voltar ao início de sessão
verify-email-intro = Introduz a tua palavra-passe para confirmares que este endereço é teu.
verify-email-submit = Verificar o meu endereço

### Project status

project-archive = Arquivar
project-sheet-invite = Convidar
project-sheet-recurring = Recorrentes
project-sheet-edit = Editar projeto
project-sheet-close = Encerrar projeto
project-sheet-archive = Arquivar projeto
project-sheet-reopen = Reabrir projeto
project-sheet-unarchive = Desarquivar projeto
project-sheet-leave = Sair do projeto

### Dates

date-long = { $day } de { $month } de { $year }

month-1 = janeiro
month-2 = fevereiro
month-3 = março
month-4 = abril
month-5 = maio
month-6 = junho
month-7 = julho
month-8 = agosto
month-9 = setembro
month-10 = outubro
month-11 = novembro
month-12 = dezembro

month-short-1 = Jan
month-short-2 = Fev
month-short-3 = Mar
month-short-4 = Abr
month-short-5 = Mai
month-short-6 = Jun
month-short-7 = Jul
month-short-8 = Ago
month-short-9 = Set
month-short-10 = Out
month-short-11 = Nov
month-short-12 = Dez

### Actions

add = Adicionar
create = Criar
creating = A criar…
edit = Editar
leave = Sair
close = Fechar
paste = Colar
join = Entrar
import = Importar
importing = A importar…
field-description = Descrição
field-optional = Opcional

### Projects

projects-filter-active = Ativos
projects-filter-all = Todos
projects-count-label = Projetos
projects-empty = Sem projetos
projects-offline-banner = Dados offline - liga-te novamente para atualizar.
demo-banner = Projeto de demonstração - só de leitura.
demo-start-own = Criar o meu projeto
demo-badge = Demo
demo-bar = As suas alterações desaparecem ao fechar este separador.
demo-leave = Começar a sério
demo-preparing = A preparar a tua demo…
demo-failed = Não foi possível preparar a demo.
demo-sign-in-blocked = O início de sessão está desativado na demo. Sai para iniciar sessão ou criar uma conta; os teus projetos ficam intactos.
projects-no-local-data = Sem dados locais
projects-no-local-data-hint = Inicia sessão para carregar os teus projetos pela primeira vez.
projects-add = Adicionar um projeto
projects-create = Criar um projeto
projects-personal-expense = Despesa pessoal
personal-expense-banner = Adicionada a As minhas despesas
personal-create-title = Cria o teu projeto pessoal
personal-create-hint = Acompanha o que gastas sozinho
group-create-title = Cria um projeto partilhado
group-create-hint = Divide as despesas com amigos e família
personal-create-action = Criar
personal-create-signup = Criar uma conta
personal-sheet-title = O teu projeto pessoal
personal-sheet-intro = Para anotares o que gastas sozinho. Aparece com os teus grupos nas Estatísticas.
personal-sheet-point-alone = Um só membro: tu, com o nome da tua conta. Sem link de partilha.
personal-sheet-point-encrypted = Cifrado nos teus dispositivos, como os teus outros projetos.
personal-sheet-point-stats = As tuas partes nos grupos somam-se a estas despesas nas Estatísticas.
personal-default-name = As minhas despesas
my-spending-prev-month = Mês anterior
my-spending-next-month = Mês seguinte
my-spending-spent = Gastou
my-spending-groups = Grupos
my-spending-income = Receitas
my-spending-balance = Saldo do mês
my-spending-unconverted = As partes noutra moeda ficam de fora até se conhecer uma taxa.
my-spending-empty = Nada este mês
my-spending-empty-hint = Adiciona uma despesa com +. As tuas partes nos grupos aparecerão aqui automaticamente.
my-spending-share-of = de { $amount }
my-spending-uncounted = não contada
payer-you = si
expense-received-by = recebido por
counted-switch = Contar em As minhas despesas
counted-switch-on = Ativado: a tua parte soma-se a As minhas despesas
counted-switch-off = Desativado: a tua parte não aparece em As minhas despesas
counted-groups = Grupos contados
counted-groups-intro = A tua parte nestes grupos soma-se a As minhas despesas e aos orçamentos.
counted-groups-converted = { $from }, convertido em { $to }
counted-groups-encrypted = Esta escolha é cifrada com a tua conta e acompanha os teus dispositivos.
counted-groups-empty = Ainda nenhum grupo neste dispositivo.
counted-groups-done = OK
counted-save-failed = Não foi possível guardar esta escolha. Tenta novamente.
tab-budget = Orçamento
budget-title = Orçamentos
budget-manage = Gerir
budget-left = Restam { $amount }
budget-over = Ultrapassado em { $amount }
budget-empty = Nenhum orçamento
budget-empty-hint = Define um limite por categoria. As tuas partes nos grupos também contam.
budget-define = Definir um orçamento
budget-sheet-title = Orçamentos mensais
budget-sheet-intro = Sobre a tua parte: pessoais e grupos. Deixa vazio para não acompanhar uma categoria.
budget-average = Média últimos 3 meses: { $amount }
budget-for = Orçamento { $category }
budget-alert = Avisar-me aos 80 %
budget-alert-hint = Calculado neste dispositivo
budget-invalid = Montante inválido para { $category }.
budget-offline = Os orçamentos só se guardam com ligação. Volta a ligar-te e tenta novamente.
budget-alert-near = { $category }: { $percent } % do orçamento do mês
budget-alert-over = { $category }: orçamento do mês ultrapassado
my-spending-search-summary =
    { $count ->
        [one] { $count } resultado
       *[other] { $count } resultados
    } · { $total } · pessoais e grupos, todos os meses
projects-join = Entrar num projeto
projects-import-tricount = Importar do Tricount
project-actions = Ações do projeto

status-ongoing = Em curso
status-closed = Encerrado
status-archived = Arquivado

nav-help = Ajuda
nav-privacy = Política de privacidade
nav-terms = Termos de utilização
nav-legal = Aviso legal

leave-project-title = Sair do projeto?
leave-project-message = Vais perder o acesso a partir deste dispositivo. Se não ficar nenhum membro, o projeto e todas as suas despesas são eliminados definitivamente.

add-project-title = Novo projeto
add-project-name-label = Nome do projeto
add-project-name-placeholder = A minha viagem, Casa 2024…
add-project-participants = Participantes
add-project-participant-name = Nome do participante
add-project-participant-placeholder = Clark Kent
add-project-offline = Não podes criar um projeto offline. Liga-te novamente e tenta outra vez.
add-project-name-required = O projeto precisa de um nome.

join-link-label = Ligação de partilha
join-link-hint = A ligação contém a chave de desencriptação - copia-a por inteiro.
join-invalid-link = Essa ligação não é válida. Cola a ligação de partilha completa, incluindo a parte depois do #.
join-wrong-project = Essa ligação pertence a outro projeto.

import-tricount-link-label = Ligação ou chave Tricount
import-tricount-key-required = Introduz uma ligação ou chave Tricount.
import-tricount-encryption-failed = Erro de encriptação.
import-tricount-unimportable = Nada foi importado: este Tricount tem membros com conta Tricount ou montantes que não batem certo (entradas afetadas: { $count }).

### Expenses

save = Guardar
saving = A guardar…
adding = A adicionar…
link-copied = Ligação copiada
missing-encryption-key = Falta a chave de encriptação.
missing-encryption-key-title = Falta a chave de encriptação
missing-encryption-key-hint = A ligação que usaste não contém a chave necessária para desencriptar este projeto. Usa a ligação completa partilhada por quem o criou.
project-locked-hint = Este dispositivo não tem a chave deste projeto. Abre a ligação de partilha para o desbloquear.
expenses-unreadable = Algumas entradas não podem ser lidas neste dispositivo e não contam para os totais. Atualiza a app para as veres.
project-unlock = Desbloquear
project-no-local-data-hint = Inicia sessão para carregar os dados deste projeto pela primeira vez.
project-gone-hint = Foi eliminado quando o último membro saiu. A ligação de partilha já não funciona, mesmo que a voltes a abrir.

expense-add = Adicionar uma despesa
transfer-add = Adicionar uma transferência
expense-edit-title = Editar a despesa
expense-category = Categoria
expense-delete-title = Eliminar a despesa
expense-delete-message = «{ $name }» será eliminada definitivamente. Esta ação é irreversível.
expense-inconsistent-amounts = Valores incoerentes
expenses-empty = Sem despesas
expenses-empty-hint = Começa por adicionar despesas no botão abaixo

expense-type-expense = Despesa
expense-type-transfer = Transferência
expense-type-gain = Receita
expense-type-group = Tipo
expense-paid-by = paga por
expense-sent-by = enviada por
expense-contributed-by = contribuída por

expense-name-required = O nome é obrigatório.
expense-amount-not-positive = O valor tem de ser maior do que 0.
expense-no-payer = Seleciona pelo menos uma pessoa que pagou.
expense-no-debtor = Seleciona pelo menos uma pessoa que deve.
expense-invalid-date = Esta data não é válida.
expense-payers-mismatch = Quem pagou soma { $sum }, o que não corresponde ao valor da despesa ({ $total }).
expense-debtors-mismatch = Quem deve soma { $sum }, o que não corresponde ao valor da despesa ({ $total }).

participants-none = Ninguém
participants-everyone = Toda a gente ({ $count })
participants-some = { $count } de { $total }
participants-select-all = Selecionar tudo
participants-remaining = Faltam { $amount }
participants-over-by = Excede em { $amount }
participants-who-paid = Quem pagou?
participants-who-received = Quem recebeu?
participants-who-transfers = Quem transfere?
participants-who-receives = Quem recebe?
participants-for-whom = Para quem?

stats-total-expenses = Total de despesas
stats-my-expenses = As minhas despesas

tab-expenses = Despesas
tab-balance = Saldo
tab-reimbursements = Acertar contas
balance-gets-back = Recebe
balance-owes = Deve
balance-settled = Em dia
reimbursements-empty-hint = Aqui aparecem sugestões de reembolso quando as contas não batem certo
reimbursement-owes = { $debtor } deve a { $creditor }

user-selection-title = Qual dos participantes és tu?
user-selection-hint = Escolhe o teu nome na lista.
user-selection-required = Seleciona um participante.

edit-project-deferred-new-members = adicionar novos membros
edit-project-deferred-removals = remover membros
edit-project-offline-deferred = Offline: { $items } será aplicado quando voltares a ligar-te.

export-failed = Falha na exportação: { $reason }

history-expense-added = Despesa adicionada: { $name }
history-expense-edited = Despesa editada: { $name }
history-expense-deleted = Despesa eliminada: { $name }
move-expense = Mover para um projeto
move-expense-title = Mover para…
move-current = Atual
move-converted = { $currency }, convertido à taxa do mês
move-note-personal = Vais escolher quem pagou e para quem no passo seguinte. A despesa sairá de As minhas despesas.
move-note-group = A despesa sairá de { $project } para todos os seus membros e o saldo do grupo será recalculado. Vais escolher quem pagou e para quem no passo seguinte.
move-banner = De { $from } para { $to }
move-submit = Mover
move-done = Despesa movida para { $project }
history-expense-moved = Despesa movida para outro projeto: { $name }
personal-badge = Pessoal
move-members =
    { $count ->
        [one] { $count } membro
       *[other] { $count } membros
    }
history-project-edited = Projeto editado: { $name }
history-name-changed = Nome: «{ $from }» → «{ $to }»
history-description-added = Descrição adicionada: «{ $value }»
history-description-removed = Descrição removida: «{ $value }»
history-description-changed = Descrição: «{ $from }» → «{ $to }»

### Sweep

field-amount = Valor
expense-name-placeholder = Restaurante, compras…
expense-actions = Ações da despesa
expense-your-share = A tua parte
expense-your-share-value = A tua parte: { $amount } { $currency }
expense-share-percent = { $percent }%
expense-added-by = Adicionado por { $name } em { $date }
expense-inconsistent-detail = Valores incoerentes: { $paid } pago, { $owed } em dívida, para uma despesa de { $total }. Edita a despesa para corrigir.
missing-access-key = Falta a chave de acesso. Abre este projeto pela ligação de partilha.
filter-all = Tudo
filter-my-payments = Os meus pagamentos
filter-my-debts = O que devo
expense-filter-group = Filtro
expenses-search = Pesquisar despesas
expenses-no-match = Nenhuma despesa corresponde.
expenses-search-open = Pesquisar
expenses-search-clear = Limpar pesquisa
expenses-search-summary =
    { $count ->
        [one] { $count } resultado
       *[other] { $count } resultados
    } · { $total } · todos os meses
participants-shares-for = Partes de { $name }
participants-amount-for = Valor de { $name }
reimbursement-add = Adicionar um acerto
project-forget = Remover da minha lista
project-history-title = Histórico
history-kind-add = Adicionado
history-kind-delete = Eliminado
history-kind-edit = Editado
export = Exportar
export-json = Exportar JSON
export-csv = Exportar CSV
share-link = Partilhar
copy-link-failed = Não foi possível copiar a ligação
open-in-app = Abrir na aplicação
not-found-title = Página não encontrada
not-found-back = Voltar aos projetos

### Charts

charts-period = Período
period-all = Tudo
period-month = Mês
period-3months = 3 meses
period-year = Ano
period-custom = Person.
charts-tab-categories = Categorias
charts-tab-trends = Tendências
charts-total-spent = Total gasto
charts-avg-per-person = Média por pessoa
charts-expense-count =
    { $count ->
        [one] { $count } despesa
       *[other] { $count } despesas
    }
charts-nothing-to-show = Nada a mostrar
charts-my-share-skipped =
    { $count ->
        [one] 1 projeto não é contado - nenhum participante escolhido, ou os dados não foram carregados.
       *[other] { $count } projetos não são contados - nenhum participante escolhido, ou os dados não foram carregados.
    }

### Categories

category-food = Alimentação
category-transport = Transportes
category-accommodation = Alojamento
category-leisure = Lazer
category-shopping = Compras
category-services = Serviços
category-parties-gifts = Festas e prendas
category-other = Outros
categories-manage = Gerir categorias
categories-title = Categorias
categories-yours = As tuas categorias
categories-builtin = Predefinidas
categories-add = Adicionar uma categoria
categories-in-parent = em { $parent }
categories-renamed-from = renomeada · originalmente { $name }
categories-hidden-hint = oculta · as suas despesas vão para { $other }
categories-always-shown = sempre visível
categories-hidden = Oculta
categories-new = Nova categoria
categories-edit = Editar categoria
categories-parent = Contada em
categories-parent-hint = Nas estatísticas de todos os teus projetos, { $name } conta em { $parent }.
categories-create = Criar
categories-hide = Ocultar esta categoria
categories-hide-hint = As suas despesas aparecem em { $other } para todos os membros.
categories-delete-title = Eliminar «{ $name }»?
categories-delete-body = As despesas abrangidas ({ $count }) voltam para { $parent }. Nenhuma despesa é eliminada.
categories-delete-body-unknown = As suas despesas voltam para { $parent }. Nenhuma despesa é eliminada.
categories-delete-warning = A categoria desaparece para todos os membros do projeto.
categories-summary-created = Criadas: { $count }
categories-summary-renamed = Renomeadas: { $count }
categories-summary-hidden = Ocultas: { $count }
categories-summary-default = Sem categorias próprias
categories-offline = Gerir categorias exige ligação. Volta a ligar-te e tenta novamente.
categories-error-too-many = Um projeto tem no máximo 20 categorias próprias.
categories-error-full = Este projeto atingiu o limite de categorias.
categories-error-name = Dá um nome à categoria.
categories-error-missing = Esta categoria mudou entretanto. Fecha e tenta novamente.
history-category-created = Categoria criada: { $name }
history-category-edited = Categoria editada: { $name }
history-category-deleted = Categoria eliminada: { $name }
history-empty = Sem eventos
not-found-hint = Esta página não existe ou foi movida.
payers-title-paid-by = Pago por
payers-title-sender = Emissor
payers-title-contributors = Contribuidores
debtors-title-debtors = Deve
debtors-title-recipients = Destinatários
debtors-title-beneficiaries = Beneficiários
debtors-title-for = Para
debtors-title-recipient = Destinatário
expense-split-between = dividido entre { $count }
expense-people =
    { $count ->
        [one] { $count } pessoa
       *[other] { $count } pessoas
    }

### Welcome

welcome-title = As tuas contas só a ti dizem respeito.
welcome-subtitle = Divide despesas entre amigos.
welcome-note = Grátis. Sem necessidade de conta. Sem publicidade.
welcome-link-title = Um link, e todos participam.
welcome-link-body = Ninguém precisa de criar uma conta.
welcome-link-account = Uma conta? Nunca obrigatória. Serve para encontrares os teus projetos noutro dispositivo, convidares amigos a partir da app e partilhares os teus dados de pagamento.
welcome-demo-project = Fim de semana em Lyon
welcome-private-title = Ninguém consegue ler as tuas contas. Nem mesmo nós.
welcome-private-body = Nomes, valores, projetos: tudo é cifrado no teu dispositivo. Só tu tens a chave.
welcome-private-names = Nomes
welcome-private-amounts = Valores
welcome-private-projects = Projetos
welcome-scan-title = Tira uma foto ao recibo.
welcome-scan-body = O valor, a data e a categoria preenchem-se sozinhos. Tudo acontece no teu telemóvel. A foto não é guardada.
welcome-eu-title = 100 % europeu
welcome-no-ads = Zero publicidade
welcome-no-trackers = Zero rastreadores
welcome-step = Passo { $current } de { $total }
welcome-next = Seguinte
welcome-skip = Saltar
welcome-start = Começar
welcome-how-it-works = Como funciona exatamente?

### Help

help-intro = Uma dúvida frequente? Toca para abrir a resposta.
help-create-project-q = Como crio um projeto?
help-create-project-a = No ecrã inicial, toca no botão + em baixo. Dá um nome ao projeto, escolhe a moeda e está feito.
help-add-participants-q = Como adiciono participantes?
help-add-participants-a = Abre o projeto e adiciona participantes a partir da lista de membros. Cada participante pode pagar ou dever numa despesa.
help-share-project-q = Como partilho um projeto?
help-share-project-a = Partilha o URL do projeto (o da barra de endereços). Quem tiver a ligação pode ver e editar o projeto.
help-add-expense-q = Como adiciono uma despesa?
help-add-expense-a = Dentro de um projeto, toca em +, introduz o valor e indica quem pagou e entre quem dividir. Também podes escolher uma data diferente de hoje.
help-types-q = Qual a diferença entre despesa, transferência e receita?
help-types-expense = - uma compra feita por uma pessoa e dividida por várias.
help-types-transfer = - um reembolso de uma pessoa para outra, sem divisão.
help-types-gain = - dinheiro recebido (reembolso, prenda) para dividir por várias pessoas.
help-past-date-q = Posso datar uma despesa no passado?
help-past-date-a = Sim, o campo da data é livre. A data de criação do registo é guardada à parte.
help-who-owes-q = Como calcula o Counted quem deve o quê?
help-who-owes-a = O Counted calcula o saldo líquido de cada participante (o que adiantou menos o que deve) e propõe a série de transferências mais curta para acertar todas as contas.
help-minimal-transfers-q = Porque é mínimo o número de transferências sugeridas?
help-minimal-transfers-a = O algoritmo emparelha primeiro os saldos que se anulam exatamente e depois trata os restantes do maior credor para o maior devedor. Resultado: menos transferências para acertar tudo.
help-import-tricount-q = Como importo um projeto do Tricount?
help-import-tricount-a = No ecrã inicial, toca no botão «+» em baixo e depois em
help-import-tricount-b = Cola a ligação de partilha do Tricount que queres importar.
help-encryption-q = Os meus dados estão cifrados?
help-encryption-a = Sim. O Counted junta duas garantias:
help-encryption-e2ee-term = Cifra ponta a ponta
help-encryption-e2ee-def = - tudo o que passa entre ti e o servidor viaja cifrado.
help-encryption-zero-term = Zero acesso
help-encryption-zero-def = - cifras os dados antes de os enviar e o servidor só guarda texto cifrado. Não temos forma de o ler.
help-encryption-see = Para os detalhes, consulta a
help-forgot-password-q = O que acontece se esquecer a minha palavra-passe?
help-forgot-password-warning = Ninguém a pode recuperar por ti.
help-forgot-password-a = A tua palavra-passe protege as tuas chaves de cifra e nunca nos é enviada: ninguém - nós incluídos - consegue decifrar os teus projetos por ti. Guarda-a bem, de preferência num gestor de palavras-passe.
help-archive-delete-q = Como arquivo ou elimino um projeto?
help-archive-delete-a = No ecrã do projeto, abre o menu e escolhe
help-archive-delete-b = para o ocultar mantendo-o. Um projeto só é eliminado definitivamente quando o último membro sai.
help-delete-account-q = Como elimino a minha conta?
help-delete-account-a = Abre as Definições e usa «Eliminar a minha conta». É imediato e irreversível.
help-contact = Outra dúvida? Escreve-nos para

# Receipt scanning (mobile only)
expense-scan = Digitalizar um recibo
scan-in-progress = A ler o recibo…
scan-error-capture = Não foi possível tirar essa fotografia. Tenta de novo, ou introduz a despesa à mão.
scan-error-unreadable = Nada legível nesse recibo. Introduz a despesa à mão.
scan-check-amount = Verifica o total - não estava impresso com clareza.
scan-take-photo = Tirar uma fotografia
scan-choose-photo = Escolher uma fotografia

update-required-title = Atualização necessária
update-required-body = Esta versão do Counted é demasiado antiga para comunicar com o servidor. Atualiza-a para continuar a usar a aplicação.
update-required-button = Atualizar

### Common (adições)

confirm = Confirmar
copy = Copiar
field-date = Data
date-today = Hoje
date-yesterday = Ontem

### Errors (amigos)

error-identity-taken = Outra conta já reivindicou este participante.
error-self-friend-request = Não podes adicionar-te a ti próprio como amigo.
error-not-a-friend = Só podes convidar pessoas da tua lista de amigos.
error-friend-has-no-key = Este amigo ainda não abriu a versão mais recente da aplicação. Pede-lhe que inicie sessão uma vez e tenta novamente.
error-friend-request-not-found = Este pedido de amizade já não existe.
error-invitation-not-found = Este convite já não existe.
error-too-many-friend-requests = Demasiados pedidos de amizade por agora. Tenta novamente amanhã.
error-too-many-invitations = Demasiados convites pendentes.
error-invalid-kdf-salt = As definições de encriptação não são válidas. Atualiza a aplicação e tenta novamente.
error-mixed-project-batch = Esses participantes não estão todos no mesmo projeto.
error-invalid-payload = Esta versão da aplicação enviou dados que o servidor não aceita. Atualiza-a e tenta novamente.
error-invalid-public-key = A tua chave de encriptação não é válida. Atualiza a aplicação e tenta novamente.
error-payment-methods-stale = Os teus dados de pagamento foram alterados noutro dispositivo. Recarrega e tenta novamente.

### Payment methods (partilha)

settings-payment-methods-share-warning = Um método partilhado é visível para todos os membros dos teus projetos.
payment-method-share = Partilhar com os meus projetos
payment-method-share-hint = Aparece ao lado do teu nome quando alguém te deve dinheiro.
payment-method-copy = Copiar { $name }
payment-method-copied = Copiado.
payment-method-copy-failed = Não foi possível copiar - seleciona o texto e copia-o à mão.

### Expenses (moeda, montante)

expense-category-auto = Auto · { $emoji }
emoji-picker-label = Emoji
emoji-none = Sem emoji
expense-currency = Moeda do montante
amount-op-add = Mais
amount-op-subtract = Menos
amount-op-multiply = Multiplicar
amount-op-divide = Dividir
amount-op-equals = Igual
amount-op-done = Concluído
expense-rate = Taxa de câmbio (opcional)
expense-rate-hint = Deixa em branco para usar a taxa da Comissão Europeia (InforEuro) de { $month }: 1 { $from } = { $rate } { $to }.
expense-rate-invalid = Introduz uma taxa de câmbio superior a 0.
expense-rate-unavailable = Nenhuma taxa automática disponível - introduz uma à mão.
expenses-show-more = Mostrar mais ({ $count } restantes)
expense-converted-from = Pago { $amount } { $from } · 1 { $from } = { $rate } { $to }
project-currency = Moeda
project-currency-locked = A moeda é fixada na criação do projeto.
currency-search = Pesquisar uma moeda
project-gone-title = Este projeto já não existe
participants-by-shares = Por partes
split-amounts = Valores

### Reimbursements

reimbursements-empty-title = Contas acertadas!
reimbursement-record = Acertar
settlement-recorded = Acerto registado.
reimbursement-pay-with = Pagar
reimbursement-pay-shared-by = Partilhado por { $name } - confirma o nome do destinatário que a tua aplicação mostra antes de enviar.
reimbursement-pay-title = Pagar a { $name }
reimbursements-mine-title = Deves
reimbursements-others-title = Outros reembolsos

### Identity

identity-claimed = Associado a uma conta
identity-claimed-by = Conta de { $name }
identity-taken-repick = Outra conta reivindicou o participante que estavas a usar. Escolhe outro.
participant-gone-repick = O participante que estavas a usar foi removido deste projeto. Escolhe outro.

### Edit project

edit-project-title = Editar o projeto
edit-project-new-badge = novo

### Charts (adições)

charts-project = Projeto
charts-all-projects = Todos os projetos
charts-date-from = De
charts-date-to = Até
charts-total = Total
charts-tab-people = Pessoas
charts-tab-projects = Projetos
charts-scope = Gastos de quem
charts-scope-group = Grupo
charts-scope-me = Eu
charts-currency = Moeda
charts-my-share = A minha parte
charts-share-of-total = { $pct }% de { $total }
charts-i-paid = Paguei
charts-paid-more = { $amount } a mais do que a minha parte
charts-paid-less = { $amount } a menos do que a minha parte
charts-paid-even = Exatamente a minha parte
charts-part-title = A tua parte em cada categoria
charts-part-desc = A cinzento o que o grupo gastou, a cor o que tu consumiste.
charts-breakdown-title = Repartição por categoria
charts-breakdown-desc = Toca numa fatia ou numa linha para ver as despesas.
charts-of-total = { $amount } de { $total }
charts-show-all = Mostrar tudo ({ $count })
charts-show-less = Mostrar menos
charts-spend-title = Gastos ao longo do tempo
charts-group-by = Agrupar por
bucket-day = Dia
bucket-week = Semana
bucket-month = Mês
charts-avg = média
charts-cat-title-day = { $category }, dia a dia
charts-cat-title-week = { $category }, semana a semana
charts-cat-title-month = { $category }, mês a mês
charts-cat-desc = Escolhe uma categoria para a seguir ao longo do tempo.
charts-running-title = Total acumulado
charts-running-desc = Desde { $date }.
charts-avg-per-day = { $amount } / dia em média
charts-avg-per-week = { $amount } / semana em média
charts-avg-per-month = { $amount } / mês em média
charts-people-title = Quem suportou o grupo
charts-people-desc = O que cada pessoa pagou, ao lado do que consumiu.
charts-paid = Pago
charts-fair-share = Parte justa
charts-you = (tu)
charts-net-more = pagou mais
charts-net-less = pagou menos
charts-balance-title = O teu saldo ao longo do tempo
charts-balance-desc = Acima da linha o grupo deve-te. Abaixo, deves tu ao grupo.
charts-owed = Devem-te
charts-owe = Deves
charts-projects-title = A tua parte, por projeto
charts-projects-desc = Os totais ficam por moeda.
history-by = Por { $name }

### Notifications

notifications-label = Notificações
notifications-title = Notificações
notifications-empty = Nada de novo
notifications-friend-request = Pedido de amizade

### Friends

friends-title = Amigos
friends-anonymous-body = Os amigos estão associados à tua conta. Inicia sessão para adicionar pessoas e convidá-las para os teus projetos sem partilhar uma ligação.
friends-add-title = Adicionar um amigo
friends-add-hint = Se essa pessoa tiver conta, verá o teu pedido na app. Se não, avisa-a: vai encontrá-lo ao registar-se com este endereço.
friends-add-button = Adicionar
friends-add-from-project = Adicionar como amigo
friends-request-sent = Pedido enviado
friends-no-account-key = Inicia sessão novamente para gerir os teus amigos neste dispositivo.
friends-incoming-title = Pedidos
friends-accept = Aceitar
friends-decline = Recusar
friends-list-title = Os meus amigos
friends-list-empty = Ainda sem amigos. Adiciona alguém por e-mail acima, ou a partir de um projeto que partilhem.
friends-remove = Remover
friends-remove-confirm-title = Remover amigo
friends-remove-confirm-message = { $email } deixará de estar nos teus amigos, e tu deixarás de estar nos dele. Qualquer um dos dois pode enviar um novo pedido mais tarde.
friends-no-key = Ainda não está pronto
friends-key-changed = Código alterado
friends-key-changed-hint = O código de segurança de um amigo mudou. Acontece quando ele protege a conta, ou se a chave dele foi substituída. Os convites com ele ficam bloqueados até lerem um ao outro o novo código, pessoalmente ou por telefone, e tocares em «O código coincide».
friends-fingerprint = Código de segurança
friends-fingerprint-hint = Dois amigos que leem um ao outro o mesmo código de segurança sabem que ninguém está entre eles - nem sequer o nosso servidor.
friends-key-mismatch = O servidor enviou para a tua conta uma chave pública que não corresponde à tua chave privada. O teu código de segurança está oculto e os convites estão desativados neste dispositivo: alguém pode estar a adulterar o servidor.
keypair-mismatch = As chaves que o servidor enviou para a tua conta não correspondem às deste dispositivo. Convites e mudanças de chave estão desligados aqui até corresponderem: alguém pode estar a manipular o servidor.
friends-outgoing-title = Enviados
friends-outgoing-hint = À espera de resposta. Aparecerão nos teus amigos quando aceitarem.
friends-withdraw = Cancelar
friends-notify = Avisar por mensagem
friends-notify-label = Avisar { $email }
friends-notify-message = Vou passar a usar o Counted para as nossas despesas partilhadas. Registo gratuito em https://counted.fr com o endereço { $email }: o meu pedido de amizade está lá à espera.
friends-notify-copied = Mensagem copiada
friends-notify-copy-failed = Não foi possível copiar a mensagem
guests-hint-title =
    { $count ->
        [one] { $count } participante ainda não entrou no projeto
       *[other] { $count } participantes ainda não entraram no projeto
    }
guests-hint-later = Mais tarde
guests-hint-send = Convida-os
guests-send-title = Enviar o projeto aos participantes
guests-send-friend = Convidar um amigo do Counted
guests-send-friend-badge = Sem link
guests-send-friend-hint = Se já tiver conta, recebe um convite cifrado; nada passa pela tua app de mensagens.
guests-send-link = Partilhar o link do projeto
guests-send-link-hint = Para quem não tem conta. Abre o projeto com um toque.
guests-send-link-warning = Qualquer pessoa com este link pode ler e alterar o projeto. A tua app de mensagens também o vê.
invite-friends-title = Convidar amigos
invite-friends-hint = A chave do projeto é cifrada para cada amigo neste dispositivo. O servidor nunca a vê.
invite-friends-empty = Ainda não há amigos para convidar.
invite-friends-button = Convidar
invite-sent = { $count ->
    [one] Convite enviado
   *[other] { $count } convites enviados
}
invitation-badge = Convite
invitation-to = Entrar em «{ $name }»
invitation-to-unnamed = Entrar num projeto
invitation-unreadable = Este convite não pode ser aberto neste dispositivo
invitation-from = De { $email }
invitation-accept = Entrar
invitation-decline = Recusar

# Participants in the create and edit modals, and the "who are you?" picker - see
# docs/plans/friends.md §11.
participants-you-label = O teu nome neste projeto
participants-you-badge = Tu
participants-others = Outros participantes
participants-removed = Removidos
participant-restore = Restaurar
participants-empty = Adiciona participantes abaixo.
participants-duplicate = «{ $name }» já está na lista.
participants-input-label = Adicionar um amigo ou escrever um nome
participants-input-placeholder = Um amigo ou qualquer nome
participants-suggest-friend = Amigo · entra como «{ $name }», recebe um convite
participants-suggest-not-ready = Amigo · ainda não está pronto
participants-suggest-guest = Adicionar «{ $text }»
participants-suggest-guest-sub = Convidado
participants-friends = Os teus amigos
participants-all-friends = Todos os amigos
participants-login-hint = Inicia sessão para adicionar pessoas diretamente da tua lista de amigos.
participants-invite-badge = Convidar
participants-guest-badge = Convidado
participants-rename = Mudar o nome de { $name }
participants-remove = Remover { $name }
participants-rename-label = Novo nome
participants-rename-save = Guardar o nome
participants-rename-hint = O nome que todos veem neste projeto. O convite continua a ir para { $email }.
participants-invited-badge = Convidado
participants-invited-sub = { $email } · ainda não aceite
participants-invited-pending = Convite ainda não aceite
participants-unlinked = Não ligado a nenhuma conta
add-project-create-invite = Criar e convidar { $count }
edit-project-save-invite = Guardar e convidar { $count }
edit-project-you-are = Neste projeto és { $name }
edit-project-no-identity = Ainda não escolheste quem és
edit-project-switch = Mudar
edit-project-choose = Escolher
invite-failed = Não foi possível enviar estes convites: { $emails }
invite-waiting = Estes amigos têm de abrir o Counted uma vez antes de poderem ser convidados: { $emails }
key-offer-title = Outra chave para { $name }
key-offer-body = Este dispositivo tem outra chave para este projeto, que nunca o abriu aqui.
key-offer-choices = Usar esta chave: o projeto abre com a chave desta ligação ou convite. Cancelar: o dispositivo mantém a chave atual e nada muda.
key-offer-use = Usar esta chave
invite-again = Convidar de novo
friend-picker-title = Adicionar amigos
user-selection-invited-hint = { $email } convidou-te para «{ $project }».
user-selection-suggested = Sugerido
user-selection-suggested-sub = { $email } adicionou-te com este nome
user-selection-confirm-as = Sou { $name }
user-selection-missing = O teu nome não está aqui? Pede a um participante que te adicione nas definições do projeto.

## Despesas recorrentes

repeat-label = Repetir
repeat-none = Não se repete
repeat-weekly = Todas as semanas
repeat-biweekly = A cada 2 semanas
repeat-monthly = Todos os meses
repeat-quarterly = A cada 3 meses
repeat-yearly = Todos os anos
repeat-every-weeks = A cada { $count } semanas
repeat-every-months = A cada { $count } meses
repeat-every-years = A cada { $count } anos
repeat-custom = Personalizado…
repeat-every = A cada
repeat-unit-weeks = semanas
repeat-unit-months = meses
repeat-unit-years = anos
repeat-on-weekday = à { $weekday }
repeat-on-day = no dia { $day }
repeat-on-day-month = a { $day } de { $month }
repeat-month-end = Nos meses mais curtos, calha no último dia.
repeat-ends = Termina
repeat-ends-never = Nunca
repeat-ends-on = Numa data
repeat-ends-after = Após
repeat-fewer = Menos
repeat-more = Mais
repeat-last-on = a última a { $date }
repeat-variable = O valor muda de cada vez
repeat-variable-hint = Cada uma é adicionada com o último valor e marcada «a confirmar».
repeat-done = Concluído
repeat-no-end = Sem fim
repeat-until = Até { $date }
repeat-occurrences = Ocorrências: { $count }
repeat-offline = Precisa de ligação. A despesa pode ser adicionada na mesma.
repeat-foreign = Repete-se por { $amount } { $currency }, convertido uma vez à taxa de hoje. Serás avisado se a taxa variar mais de 5%.
repeat-backfill = Começa no passado. Despesas adicionadas agora: { $count }.
add-and-repeat = Adicionar e repetir
weekday-1 = segunda-feira
weekday-2 = terça-feira
weekday-3 = quarta-feira
weekday-4 = quinta-feira
weekday-5 = sexta-feira
weekday-6 = sábado
weekday-7 = domingo
recurring-title = Despesas recorrentes
recurring-strip = Despesas recorrentes: { $count }
recurring-next = Próxima: { $name }, { $date }
recurring-to-confirm = A confirmar: { $count }
recurring-per-month = Por mês, aprox.
recurring-your-share = A tua parte
recurring-active = Ativas
recurring-paused = Em pausa
recurring-finished = Terminadas
recurring-paid-by = paga por { $name }
recurring-next-on = Próxima a { $date }
recurring-progress = { $done } de { $total }
recurring-rate-badge = Taxa variou { $percent }%
recurring-empty = Nada se repete ainda. Escolhe «Repetir» ao adicionar uma despesa: renda, subscrições, faturas.
recurring-next-ones = Próximas
recurring-added-so-far = Adicionadas até agora
recurring-set-up-by = Criada por
recurring-pause = Pausar
recurring-resume = Retomar
recurring-stop = Parar de repetir
recurring-stop-title = Parar «{ $name }»?
recurring-stop-message = Deixa de se repetir. As despesas já adicionadas ficam.
recurring-resume-title = Retomar «{ $name }»?
recurring-resume-message = Próxima a { $date }. As datas perdidas durante a pausa não são adicionadas.
recurring-edit-title = Editar despesa recorrente
recurring-edit-banner = As alterações aplicam-se a partir de { $date }. As despesas já adicionadas ficam como estão.
recurring-next-on-label = Próxima a
recurring-next-too-early = A próxima data tem de ser depois da última despesa já adicionada.
recurring-use-stop = Para a terminar, usa «Parar de repetir» na despesa recorrente.
recurring-drift = A taxa de { $currency } variou { $percent }% desde a criação. Cada uma continua a ser adicionada por { $amount } { $project_currency } (1 { $currency } = { $rate }). À taxa de hoje seriam { $today_amount } { $project_currency }.
recurring-use-rate = Usar a taxa de hoje
recurring-keep = Manter { $amount } { $currency }
recurring-added = Despesas recorrentes adicionadas: { $count }
recurring-blocks-removal = { $name } ainda não pode ser removido: faz parte de { $rules }. Retira { $name } dessas despesas ou para-as e volta a guardar.
history-recurring-added = Adicionada automaticamente: { $name } ({ $date })
history-recurring-created = Despesa recorrente criada: { $name }
history-recurring-edited = Despesa recorrente editada: { $name }
history-recurring-paused = Despesa recorrente pausada: { $name }
history-recurring-resumed = Despesa recorrente retomada: { $name }
history-recurring-stopped = Despesa recorrente parada: { $name }
occurrence-recurring = Despesa recorrente
occurrence-auto = Adicionada automaticamente por uma despesa recorrente.
occurrence-auto-next = Adicionada automaticamente por uma despesa recorrente. Próxima a { $date }.
occurrence-auto-stopped = Adicionada automaticamente por uma despesa recorrente entretanto parada.
occurrence-manage = Gerir
estimate-badge = A confirmar
expense-pending = Pendente
estimate-title = Valor a confirmar.
estimate-body = Adicionada com o valor anterior. Introduz o valor real quando chegar.
estimate-confirm = Confirmar valor
apply-to = Aplicar a
apply-this-only = Só esta despesa
apply-and-next = Esta e as seguintes
apply-and-next-hint = A despesa recorrente muda a partir de { $date }
apply-rule-failed = A despesa foi guardada, mas a despesa recorrente não foi alterada.
occurrence-delete-message = «{ $name }» de { $date } será eliminada definitivamente. Continua a repetir-se e esta data não volta.
occurrence-delete-one = Eliminar só esta
occurrence-delete-stop = Eliminar e parar de repetir
error-recurring-clock = O relógio deste dispositivo está adiantado. Verifica a data e a hora.
error-recurring-not-found = Esta despesa recorrente já não existe.
error-recurring-stale = Alguém alterou esta despesa recorrente entretanto. Foi recarregada: verifica-a e volta a guardar.
error-too-many-recurring = Este projeto já tem 50 despesas recorrentes. Para uma de que já não precises para adicionar outra.
error-user-in-recurring = Este participante faz parte de uma despesa recorrente. Retira-o dela ou para-a primeiro.

home-owed-to-you = Devem-te
home-you-owe = Deves
home-details-show = Ver detalhe
home-details-hide = Ocultar detalhe
home-all-settled = Tudo acertado
home-pay = Pagar
home-remind = Lembrar
home-remind-message = Pequeno lembrete: ainda há reembolsos por acertar em { $project }.
home-row-owed = devem-te
home-row-owe = deves
home-row-settled = Em dia
home-row-quits = Estás em dia
home-row-pending =
    { $count ->
        [one] { $count } reembolso pendente
       *[other] { $count } reembolsos pendentes
    }
home-totals-partial = Sem os projetos sem taxa de câmbio
error-account-key-changed = A chave da tua conta mudou noutro dispositivo. Inicia sessão novamente para continuar.
settings-security = Segurança
settings-change-password = Alterar palavra-passe
settings-change-password-hint = Os teus projetos e dados ficam como estão. A sessão termina nos teus outros dispositivos.
change-password-title = Altera a tua palavra-passe
change-password-current = Palavra-passe atual
change-password-new = Nova palavra-passe
change-password-confirm = Confirma a nova palavra-passe
change-password-mismatch = As duas novas palavras-passe não coincidem.
change-password-submit = Alterar palavra-passe
change-password-submitting = A alterar…
change-password-done = Palavra-passe alterada. A sessão terminou nos teus outros dispositivos.
error-rotation-incomplete = A tua conta mudou entretanto. Tenta novamente.
error-key-lifecycle-unavailable = Ainda não está disponível: primeiro todos precisam da versão mais recente da app.
settings-compromised = Alguém pode saber a minha palavra-passe
settings-compromised-hint = Nova palavra-passe, novas chaves e a sessão termina em todos os outros dispositivos.
settings-compromised-open = Proteger a minha conta
compromised-title = Protege a tua conta
compromised-body = A tua palavra-passe e as chaves da tua conta são substituídas, e a sessão termina em todos os outros dispositivos. Os teus amigos vão ver o teu código de segurança mudar: compara-o de novo com eles.
compromised-keeps = Quem tinha a tua palavra-passe fica com o que já abriu: os projetos que conseguia ler até agora.
compromised-submit = Substituir palavra-passe e chaves
compromised-done = A tua conta está protegida. Pede aos teus amigos que comparem o teu novo código de segurança.
friends-new-fingerprint = Novo código:
friends-code-matches = O código coincide
friends-code-trusted = Código confirmado.
settings-recovery-key = Chave de recuperação
settings-recovery-key-none = Sem ela, esquecer a palavra-passe custa-te as preferências, os dados de pagamento e os códigos dos amigos, e demora 3 dias.
settings-recovery-key-set = Criada. Se esqueceres a palavra-passe, devolve-te a conta com tudo.
settings-recovery-key-pending = Uma nova chave substitui-a em { $when }. Não foste tu? Cancela-a com a tua chave atual.
recovery-key-create = Criar
recovery-key-replace = Substituir
recovery-key-cancel-replacement = Cancelar a nova chave
recovery-key-title = A tua chave de recuperação
recovery-key-body = Escreve-a ou guarda-a num sítio a que só tu tenhas acesso. A Counted não a pode mostrar de novo nem recuperá-la por ti.
recovery-key-copy = Copiar
recovery-key-copied = Copiada
recovery-key-save = Guardar como ficheiro
recovery-key-print = Imprimir
recovery-key-next = Continuar
recovery-key-confirm-body = Para verificares a tua cópia, escreve o grupo { $n } da chave.
recovery-key-confirm-wrong = Esse grupo não coincide. Volta a ver a tua cópia.
recovery-key-password-body = Escreve a tua palavra-passe para guardar a chave na conta.
recovery-key-submit = Guardar a chave
recovery-key-done = Chave de recuperação guardada.
recovery-key-replacement-done = Nova chave guardada. Substitui a atual dentro de 72 horas; até lá a atual continua a funcionar.
recovery-key-field = Chave de recuperação
recovery-key-file-intro = Chave de recuperação da Counted
recovery-key-cancel-done = A nova chave foi cancelada.
compromised-new-key = Guarda já a tua nova chave de recuperação: substitui a antiga dentro de 72 horas, e até lá a antiga continua a funcionar.
pending-reset-title = Foi pedida a reposição da tua conta
pending-reset-body = Entra em vigor em { $when }: as preferências, os dados de pagamento e os códigos dos amigos perdem-se e os teus dispositivos terão de iniciar sessão de novo. Não foste tu? Cancela com a tua palavra-passe atual.
pending-reset-cancel = Cancelar a reposição
pending-reset-cancelled = A reposição foi cancelada.
login-forgot = Esqueceste a palavra-passe?
forgot-title = Esqueceste a palavra-passe
forgot-body = Escreve o teu e-mail. Se tiver uma conta, enviamos uma ligação para escolheres uma nova palavra-passe.
forgot-submit = Enviar a ligação
forgot-sent = Se este endereço tiver uma conta, a ligação vai a caminho. Expira dentro de 24 horas.
reset-title = Recupera a tua conta
reset-with-key-body = Escreve a tua chave de recuperação e uma nova palavra-passe: tudo volta como estava.
reset-submit-key = Recuperar a minha conta
reset-no-key = Não tenho a minha chave de recuperação
reset-without-key-title = Repor sem a chave de recuperação
reset-without-key-body = A tua conta recomeça com uma nova palavra-passe dentro de 72 horas. Os projetos voltam dos teus dispositivos e das ligações de partilha; as preferências, os dados de pagamento e os códigos dos amigos não. Até lá, a palavra-passe atual continua a funcionar e pode cancelar o reinício.
reset-name = O teu nome
reset-submit-request = Repor a minha conta
reset-requested = Feito. A nova palavra-passe funciona a partir de { $when }. Até lá a atual continua a funcionar e pode cancelar.
reset-pending-already = Já há uma reposição pendente até { $when }. Um novo pedido recomeça as 72 horas.
recovered-done = A tua conta está de volta. A sessão terminou nos teus outros dispositivos.
login-reset-undoable = Esta palavra-passe foi substituída por uma reposição nos últimos 30 dias. Se não a pediste, anula-a: a tua conta volta como estava.
login-undo-reset = Anular a reposição
reset-undone = A reposição foi anulada. A tua conta voltou ao que era.
error-reset-pending = Esta nova palavra-passe funciona quando a reposição entrar em vigor, 72 horas depois do pedido.
error-account-disabled = Esta conta foi eliminada pouco depois de uma reposição e aguarda ser apagada. A palavra-passe anterior ainda pode anular a reposição.
error-recovery-key-invalid = Esta chave de recuperação não abre esta conta.
error-project-key-required = Este projeto precisa da sua chave. Abre-o de novo a partir da ligação de partilha.
names-or = { $first } ou { $last }
participants-admin-badge = Admin
edit-project-admins = Admins: { $count }
edit-project-succession = Se o projeto ficar sem admin, passa a sê-lo o membro com conta que está no projeto há mais tempo.
member-sheet-account = Membro com conta
member-actions = Ações para { $name }
member-make-admin = Tornar admin
member-make-admin-hint = Poderá mudar a chave do projeto e remover alguém.
member-made-admin = { $name } é agora admin.
group-security = Segurança do projeto
group-security-admin = Segurança do projeto · és admin
group-rotate = Mudar a chave do projeto
group-rotate-hint = Se a ligação do projeto foi comprometida: quem tiver a ligação antiga perde o acesso ao projeto.
group-remove = Remover alguém do projeto
group-remove-hint = Retira-lhe o acesso e muda a chave ao mesmo tempo.
group-admins-only = Só para admins.
group-admins-only-account = Só para admins, que têm conta.
group-ask-admin = Pede a { $names }, admin do projeto.
group-ask-admins = Pede a { $names }, que administram o projeto.
group-ask-an-admin = Pede a uma das pessoas que administram o projeto.
group-with-account = Com conta
group-free = Grátis
group-sign-in-hint = { $names }, que administra o projeto, poderá depois dar-te o papel.
group-sign-in-hint-any = Quem administra o projeto poderá depois dar-te o papel.
group-no-admin = Este projeto não tem admin, por isso a sua chave não pode mudar.
group-hide = Ocultar alguém
group-hide-hint = Oculta a pessoa das listas sem lhe retirar o acesso: a ligação e a chave continuam válidas.
group-recreate-hint = Este projeto não tem admin. Para remover alguém, cria um projeto novo.
group-recreate = Criar um projeto novo
group-no-admin-yet = Este projeto ainda não tem admin.
group-creator-hint = Só tu podes tornar-te admin deste projeto, criando uma conta.
group-creator-elsewhere-hint = Só a pessoa que criou o projeto pode tornar-se admin dele, criando uma conta. Caso contrário, cria um projeto novo para remover alguém.
member-revoke-admin = Retirar o papel de admin
member-revoke-admin-hint = Deixará de poder mudar a chave do projeto ou remover alguém.
member-revoked-admin = { $name } deixou de ser admin.
member-step-down = Deixar de ser admin
member-step-down-hint = Deixarás de poder mudar a chave do projeto ou remover alguém.
member-stepped-down = Deixaste de ser admin.
member-remove = Remover do projeto
member-remove-hint = Muda a chave do projeto: { $name } não vai poder ler nada de novo.
member-hide = Ocultar das listas
error-user-not-admin = Este membro não é admin.
error-creator-proof-invalid = Este dispositivo não criou este projeto.
error-project-has-admin = Este projeto já tem admin.
error-not-project-admin = Só quem administra o projeto pode fazer isto.
error-user-not-claimed = Esta pessoa não tem conta, por isso não pode ser admin.
error-project-key-rotated = A chave do projeto mudou. Pede a nova ligação a um membro.
error-project-rotating = A chave do projeto está a ser mudada. Tenta de novo daqui a pouco.
error-rotation-expired = Esta mudança de chave demorou demasiado e foi cancelada. Começa de novo.
error-rotation-rows-changed = O projeto mudou durante a mudança de chave. Começa de novo.
project-rotated-title = Já não tens acesso a este projeto
project-rotated-body = A chave deste projeto foi mudada. Pede a nova ligação a quem o administra.
project-rotated-open-link = Abrir uma ligação
rotation-remove-title = Remover { $name } do projeto?
rotation-rotate-title = Mudar a chave do projeto?
rotation-remove-body = Remover um membro implica gerar uma chave nova. Vais ter de partilhar a nova ligação com as pessoas sem conta.
rotation-rotate-body = Com a ligação antiga, deixa de ser possível aceder ao projeto. Vais ter de partilhar a nova ligação com as pessoas sem conta.
rotation-boxed =
    { $count ->
        [0] Nenhum outro membro vai receber a chave automaticamente
        [one] { $count } membro vai receber a chave automaticamente
       *[other] { $count } membros vão receber a chave automaticamente
    }
rotation-by-link-title = Pessoas a quem vais ter de enviar a nova ligação
rotation-by-link-hint = Estas pessoas não têm conta. Envias-lhes a ligação logo a seguir.
rotation-names-and = { $first } e { $last }
rotation-names-and-others = { $names } e mais { $count }
rotation-confirm-remove = Remover e mudar a chave
rotation-confirm-rotate = Mudar a chave
rotation-done-title = Nova chave ativa
rotation-done-removed = { $name } já não faz parte do projeto.
rotation-done-rotated = A chave do projeto mudou.
rotation-done-boxed =
    { $count ->
        [0] Nenhum outro membro recebeu a chave automaticamente.
        [one] { $count } membro recebeu a chave automaticamente.
       *[other] { $count } membros receberam a chave automaticamente.
    }
rotation-new-link =
    { $count ->
        [0] Nova ligação do projeto
        [one] Nova ligação para enviar · { $count } pessoa sem conta
       *[other] Nova ligação para enviar · { $count } pessoas sem conta
    }
rotation-link-renewed = A ligação foi renovada com sucesso.
rotation-done = Concluído
history-key-changed = Chave do projeto mudada
history-member-removed = { $name } já não faz parte do projeto
project-rotated-row-hint = A chave mudou. Pede a nova ligação.
