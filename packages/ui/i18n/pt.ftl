# Português. Completo exceto os textos legais (legal-, terms-, privacy-), que só existem em inglês
# e francês e recorrem a en.ftl mensagem a mensagem.

### Common

loading = A carregar…
cancel = Cancelar
retry = Tentar novamente
delete = Eliminar
back = Voltar
language = Idioma

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
error-generic = Ocorreu um erro. Tenta novamente.

error-invalid-email = Este endereço de e-mail não é válido.
error-invalid-password = Esta palavra-passe não é válida.
error-password-too-short = A palavra-passe tem de ter pelo menos 8 caracteres.
error-client-outdated = Esta versão da aplicação está desatualizada. Atualize-a para iniciar sessão.
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
error-claim-proof-invalid = Este dispositivo não tem a chave do projeto, por isso não pode reivindicar um participante. Abra novamente a ligação de partilha.
error-project-not-found = Este projeto já não existe.
error-expense-not-found = Esta despesa já não existe.
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
login-resend = Enviar novamente o link de verificação
login-resending = A enviar…
login-resend-sent = E-mail enviado - consulta a tua caixa de entrada.

register-submit = Criar uma conta
register-submitting = A criar…
register-have-account = Já tens conta?
register-password-placeholder = Pelo menos 8 caracteres
register-password-warning = Aponta a tua palavra-passe. Se a esqueceres, não será possível recuperar a conta.
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
settings-preferences-synced = Sincronizadas com a sua conta, cifradas.
settings-about = Acerca de
settings-anonymous-title = Não tem sessão iniciada
settings-upsell-title = Os seus projetos, em todos os dispositivos
settings-upsell-free = Grátis
settings-upsell-body = O Counted funciona sem conta. Com uma conta gratuita, os seus projetos e preferências acompanham-no no telemóvel, no computador e na web - sempre cifrados, sempre ilegíveis para nós.
settings-locked-badge = Conta
settings-locked-friends = Crie uma conta para adicionar amigos e convidá-los para um projeto a partir da app - sem links para passar.
settings-locked-payment-methods = Guarde o seu IBAN ou a sua app de pagamento uma vez e partilhe-os com os projetos que escolher. Quem lhe deve dinheiro vê-os ao lado do seu nome.
settings-friends-hint = Adicione amigos e convide-os para os seus projetos sem partilhar nenhum link.

account-member-since = Membro desde
account-logout = Terminar sessão
account-logging-out = A terminar sessão…
account-delete-title = Eliminar a minha conta
account-delete-warning = Eliminação imediata e definitiva, sem reciclagem. As despesas que registaste num projeto partilhado continuam visíveis para os outros membros - fazem parte das contas deles.
account-delete-confirm-title = Eliminar a conta
account-delete-confirm-message = A tua conta, as tuas sessões e a lista dos teus projetos serão eliminadas definitivamente. Sem a tua palavra-passe, os dados cifrados de um projeto partilhado deixam de ser legíveis para ti - esta ação é irreversível.

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
payment-methods-stale = Os teus dados de pagamento foram alterados noutro dispositivo. Foram recarregados — tenta novamente.
payment-methods-key-missing = Inicia sessão novamente para gerir os teus dados de pagamento.

verify-email-checking = A verificar o teu e-mail…
verify-email-welcome = E-mail verificado - bem-vindo ao Counted!
verify-email-back-to-login = Voltar ao início de sessão

### Project status

project-close = Encerrar
project-archive = Arquivar
project-reopen = Reabrir
project-unarchive = Desarquivar

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
projects-empty-hint = Cria um projeto no botão abaixo
projects-offline-banner = Dados offline - liga-te novamente para atualizar.
projects-no-local-data = Sem dados locais
projects-no-local-data-hint = Inicia sessão para carregar os teus projetos pela primeira vez.
projects-add = Adicionar um projeto
projects-create = Criar um projeto
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
add-project-remove-participant = Remover participante
add-project-me-badge = Eu
add-project-thats-me = Sou eu!
add-project-offline = Não podes criar um projeto offline. Liga-te novamente e tenta outra vez.
add-project-name-required = O projeto precisa de um nome.
add-project-need-two-participants = Adiciona pelo menos 2 participantes.
add-project-pick-yourself = Indica qual dos participantes és tu.

join-link-label = Ligação de partilha
join-link-hint = A ligação contém a chave de desencriptação - copia-a por inteiro.
join-invalid-link = Essa ligação não é válida. Cola a ligação de partilha completa, incluindo a parte depois do #.
join-wrong-project = Essa ligação pertence a outro projeto.

import-tricount-link-label = Ligação ou chave Tricount
import-tricount-key-required = Introduz uma ligação ou chave Tricount.
import-tricount-encryption-failed = Erro de encriptação.

### Expenses

save = Guardar
saving = A guardar…
adding = A adicionar…
link-copied = Ligação copiada
missing-encryption-key = Falta a chave de encriptação.
missing-encryption-key-title = Falta a chave de encriptação
missing-encryption-key-hint = A ligação que usaste não contém a chave necessária para desencriptar este projeto. Usa a ligação completa partilhada por quem o criou.
project-locked-hint = Este dispositivo não tem a chave deste projeto. Abre a ligação de partilha para o desbloquear.
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
reimbursements-empty-hint = Aqui aparecem sugestões de reembolso quando as contas não batem certo
reimbursement-owes = { $debtor } deve a { $creditor }

user-selection-title = Qual dos participantes és tu?
user-selection-hint = Escolhe o teu nome na lista.
user-selection-required = Seleciona um participante.

edit-project-deferred-new-members = adicionar novos membros
edit-project-deferred-removals = remover membros
edit-project-deferred-me = a escolha «Sou eu»
edit-project-offline-deferred = Offline: { $items } será aplicado quando voltares a ligar-te.

export-saved = Ficheiro guardado:
    { $path }
export-failed = Falha na exportação: { $reason }

history-expense-added = Despesa adicionada: { $name }
history-expense-edited = Despesa editada: { $name }
history-expense-deleted = Despesa eliminada: { $name }
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
expense-inconsistent-detail = Valores incoerentes: { $paid } pago, { $owed } em dívida, para uma despesa de { $total }. Edita a despesa para corrigir.
missing-access-key = Falta a chave de acesso. Abre este projeto pela ligação de partilha.
filter-all = Tudo
filter-my-payments = Os meus pagamentos
filter-my-debts = O que devo
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
charts-my-share-note = Estes valores são a tua parte de cada despesa.
charts-my-share-skipped =
    { $count ->
        [one] 1 projeto não é contado — nenhum participante escolhido, ou os dados não foram carregados.
       *[other] { $count } projetos não são contados — nenhum participante escolhido, ou os dados não foram carregados.
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
history-empty = Sem eventos
not-found-hint = Esta página não existe ou foi movida.
payers-title-paid-by = Pago por
payers-title-sender = Emissor
payers-title-contributors = Contribuidores
debtors-title-debtors = Deve
debtors-title-recipients = Destinatários
debtors-title-beneficiaries = Beneficiários

### Welcome

welcome-title = As tuas contas só a ti dizem respeito.
welcome-subtitle = Divide despesas entre amigos.
welcome-e2ee-title = Dados totalmente cifrados
welcome-e2ee-body = Nomes, valores, projetos: tudo é cifrado no teu dispositivo. Só tu tens a chave. Ninguém consegue ler as tuas contas. Nem mesmo nós.
welcome-e2ee-note = Indecifrável, mesmo para nós (zero acesso do servidor)
welcome-eu-title = 100 % europeu
welcome-eu-body = Servidores na Alemanha, e-mails enviados de França. Os teus dados nunca saem da União Europeia.
welcome-noads-title = Zero publicidade. Zero rastreadores.
welcome-noads-body = Não recolhemos nada nem vendemos os teus dados. Não é esse o nosso modelo.
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
help-forgot-password-warning = Os teus dados perdem-se definitivamente.
help-forgot-password-a = A chave de cifra deriva da tua palavra-passe, por isso não há reposição possível: ninguém - nós incluídos - consegue decifrar os teus projetos sem ela. Guarda-a bem, de preferência num gestor de palavras-passe.
help-archive-delete-q = Como arquivo ou elimino um projeto?
help-archive-delete-a = No ecrã do projeto, abre o menu e escolhe
help-archive-delete-b = para o ocultar mantendo-o. Um projeto só é eliminado definitivamente quando o último membro sai.
help-delete-account-q = Como elimino a minha conta?
help-delete-account-a = Abre as Definições e usa «Eliminar a minha conta». É imediato e irreversível.
help-contact = Outra dúvida? Escreve-nos para

# Receipt scanning (mobile only)
expense-scan = Digitalizar um recibo
scan-in-progress = A ler o recibo…
scan-error-capture = Não foi possível tirar essa fotografia. Tente de novo, ou introduza a despesa à mão.
scan-error-unreadable = Nada legível nesse recibo. Introduza a despesa à mão.
scan-check-amount = Verifique o total - não estava impresso com clareza.
scan-take-photo = Tirar uma fotografia
scan-choose-photo = Escolher uma fotografia

update-required-title = Atualização necessária
update-required-body = Esta versão do Counted é demasiado antiga para comunicar com o servidor. Atualize-a para continuar a usar a aplicação.
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

settings-payment-methods-share-warning = Um método partilhado é visível para todos os membros dos projetos em que escolheste o teu nome - qualquer pessoa que tenha uma dessas ligações.
payment-method-share = Partilhar com os meus projetos
payment-method-share-hint = Aparece ao lado do teu nome quando alguém te deve dinheiro.
payment-method-copy = Copiar { $name }
payment-method-copied = Copiado.
payment-method-copy-failed = Não foi possível copiar - seleciona o texto e copia-o à mão.

### Expenses (moeda, montante)

expense-category-auto = Auto · { $emoji }
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
project-currency-hint = Todos os montantes são mostrados nesta moeda. Não poderá ser alterada mais tarde.
project-currency-locked = A moeda é fixada na criação do projeto.
project-gone-title = Este projeto já não existe
participants-by-shares = Por partes
split-amounts = Valores

### Reimbursements

reimbursements-empty-title = Contas acertadas!
reimbursement-record = Acertar
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
charts-paid-more = { $amount } a mais do que a tua parte
charts-paid-less = { $amount } a menos do que a tua parte
charts-paid-even = Exatamente a tua parte
charts-part-title = A tua parte em cada categoria
charts-part-desc = A cinzento o que o grupo gastou, a cor o que tu consumiste.
charts-breakdown-title = Repartição por categoria
charts-breakdown-desc = Toca numa fatia ou numa linha para ver as despesas.
charts-of-total = { $amount } de { $total }
charts-show-all = Mostrar tudo ({ $count })
charts-show-less = Mostrar menos
charts-spend-title = Gastos ao longo do tempo
charts-spend-desc = Períodos curtos por dia, os mais longos por semana ou mês.
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
charts-projects-desc = Os totais ficam por moeda e nunca se somam entre si.
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
friends-add-hint = Verá o teu pedido quando iniciar sessão. Nenhum dos dois fica a saber se o outro tem conta até o pedido ser aceite.
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
friends-remove-confirm-message = { $email } deixará de estar nos seus amigos, e você deixará de estar nos dele. Qualquer um dos dois pode enviar um novo pedido mais tarde.
friends-no-key = Ainda não está pronto
friends-fingerprint = Código de segurança
friends-fingerprint-hint = Dois amigos que leem um ao outro o mesmo código de segurança sabem que ninguém está entre eles - nem sequer o nosso servidor.
friends-outgoing-title = Enviados
friends-outgoing-hint = À espera de resposta. Aparecerão nos teus amigos quando aceitarem.
friends-withdraw = Cancelar
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
