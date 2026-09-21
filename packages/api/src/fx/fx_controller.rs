use dioxus::prelude::*;
use shared::FxRates;

/// The European Commission's InforEuro monthly rates, for converting an expense into its project's
/// currency.
///
/// **It takes no parameters, and it must never take any.** That is the whole zero-knowledge
/// argument for this endpoint existing at all: the body is the entire published table and is byte
/// for byte the same for every caller, so the server learns only that somebody asked for rates —
/// not which currency pair, not which project, not which expense. A `?from=USD&to=EUR` variant
/// would hand the server, request by request, the currency composition of a ledger it is
/// specifically built not to be able to read.
///
/// Unauthenticated for the same reason `get_project` is: it discloses nothing. Anonymous
/// share-link holders create expenses too, and gating this would only break them.
///
/// The outbound call it can trigger is bounded on three levels — nginx `limit_req` per IP, the
/// `fx_refresh` claim globally, and explicit timeouts on the client itself.
#[get("/api/v1/fx/rates")]
pub async fn get_fx_rates() -> Result<FxRates, ServerFnError> {
    crate::server::fx::get_fx_rates().await
}
