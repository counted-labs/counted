pub mod api;
pub mod balances;
pub mod errors;
pub mod view_models;

pub use errors::{EMAIL_NOT_VERIFIED, PROJECT_NOT_FOUND};

#[cfg(feature = "server")]
use sqlx::FromRow;
use std::collections::HashMap;
use std::fmt;

use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn round_currency(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

/// Every consistency comparison goes through this, so the validator quantises exactly as
/// `distribute` does — otherwise a split the form produced could fail the validation that follows.
/// Saturating: a tampered payload can decrypt to NaN or inf, which must not panic.
pub fn to_cents(amount: f64) -> i64 {
    (amount * 100.0).round() as i64
}

/// True when a side adds up to the total, to the cent. Quantised before summing, not after: a
/// float sum of a hundred 0.01 rows drifts, a sum of a hundred `1i64` cannot.
pub fn sums_to_total(total: f64, amounts: impl IntoIterator<Item = f64>) -> bool {
    to_cents(total) == amounts.into_iter().map(to_cents).sum::<i64>()
}

pub struct Currency {
    pub code: &'static str,
    pub name: &'static str,
}

/// Every active ISO 4217 currency, which is the whole currency picker.
///
/// EUR first (the base and the default), then sorted by code, so the `<select>` needs no sorting
/// of its own. Funds codes (BOV, CHE, CHW, CLF, COU, MXV, USN, UYI, UYW), precious metals, XDR and
/// the test codes are left out: nobody pays a restaurant in them. Names are English on purpose —
/// they are near-universal, and 150 keys in seven locales is not worth the bytes.
///
/// The InforEuro table quotes ~152 codes and covers all but KPW, SVC and ZWG (quoted as the
/// non-ISO `ZIG`); those three simply ask for a rate by hand.
pub const CURRENCIES: &[Currency] = &[
    Currency { code: "EUR", name: "Euro" },
    Currency { code: "AED", name: "UAE Dirham" },
    Currency { code: "AFN", name: "Afghan Afghani" },
    Currency { code: "ALL", name: "Albanian Lek" },
    Currency { code: "AMD", name: "Armenian Dram" },
    Currency { code: "AOA", name: "Angolan Kwanza" },
    Currency { code: "ARS", name: "Argentine Peso" },
    Currency { code: "AUD", name: "Australian Dollar" },
    Currency { code: "AWG", name: "Aruban Florin" },
    Currency { code: "AZN", name: "Azerbaijani Manat" },
    Currency { code: "BAM", name: "Bosnia and Herzegovina Convertible Mark" },
    Currency { code: "BBD", name: "Barbadian Dollar" },
    Currency { code: "BDT", name: "Bangladeshi Taka" },
    Currency { code: "BHD", name: "Bahraini Dinar" },
    Currency { code: "BIF", name: "Burundian Franc" },
    Currency { code: "BMD", name: "Bermudian Dollar" },
    Currency { code: "BND", name: "Brunei Dollar" },
    Currency { code: "BOB", name: "Bolivian Boliviano" },
    Currency { code: "BRL", name: "Brazilian Real" },
    Currency { code: "BSD", name: "Bahamian Dollar" },
    Currency { code: "BTN", name: "Bhutanese Ngultrum" },
    Currency { code: "BWP", name: "Botswana Pula" },
    Currency { code: "BYN", name: "Belarusian Ruble" },
    Currency { code: "BZD", name: "Belize Dollar" },
    Currency { code: "CAD", name: "Canadian Dollar" },
    Currency { code: "CDF", name: "Congolese Franc" },
    Currency { code: "CHF", name: "Swiss Franc" },
    Currency { code: "CLP", name: "Chilean Peso" },
    Currency { code: "CNY", name: "Chinese Yuan" },
    Currency { code: "COP", name: "Colombian Peso" },
    Currency { code: "CRC", name: "Costa Rican Colón" },
    Currency { code: "CUP", name: "Cuban Peso" },
    Currency { code: "CVE", name: "Cape Verdean Escudo" },
    Currency { code: "CZK", name: "Czech Koruna" },
    Currency { code: "DJF", name: "Djiboutian Franc" },
    Currency { code: "DKK", name: "Danish Krone" },
    Currency { code: "DOP", name: "Dominican Peso" },
    Currency { code: "DZD", name: "Algerian Dinar" },
    Currency { code: "EGP", name: "Egyptian Pound" },
    Currency { code: "ERN", name: "Eritrean Nakfa" },
    Currency { code: "ETB", name: "Ethiopian Birr" },
    Currency { code: "FJD", name: "Fijian Dollar" },
    Currency { code: "FKP", name: "Falkland Islands Pound" },
    Currency { code: "GBP", name: "Pound Sterling" },
    Currency { code: "GEL", name: "Georgian Lari" },
    Currency { code: "GHS", name: "Ghanaian Cedi" },
    Currency { code: "GIP", name: "Gibraltar Pound" },
    Currency { code: "GMD", name: "Gambian Dalasi" },
    Currency { code: "GNF", name: "Guinean Franc" },
    Currency { code: "GTQ", name: "Guatemalan Quetzal" },
    Currency { code: "GYD", name: "Guyanese Dollar" },
    Currency { code: "HKD", name: "Hong Kong Dollar" },
    Currency { code: "HNL", name: "Honduran Lempira" },
    Currency { code: "HTG", name: "Haitian Gourde" },
    Currency { code: "HUF", name: "Hungarian Forint" },
    Currency { code: "IDR", name: "Indonesian Rupiah" },
    Currency { code: "ILS", name: "Israeli New Shekel" },
    Currency { code: "INR", name: "Indian Rupee" },
    Currency { code: "IQD", name: "Iraqi Dinar" },
    Currency { code: "IRR", name: "Iranian Rial" },
    Currency { code: "ISK", name: "Icelandic Króna" },
    Currency { code: "JMD", name: "Jamaican Dollar" },
    Currency { code: "JOD", name: "Jordanian Dinar" },
    Currency { code: "JPY", name: "Japanese Yen" },
    Currency { code: "KES", name: "Kenyan Shilling" },
    Currency { code: "KGS", name: "Kyrgyzstani Som" },
    Currency { code: "KHR", name: "Cambodian Riel" },
    Currency { code: "KMF", name: "Comorian Franc" },
    Currency { code: "KPW", name: "North Korean Won" },
    Currency { code: "KRW", name: "South Korean Won" },
    Currency { code: "KWD", name: "Kuwaiti Dinar" },
    Currency { code: "KYD", name: "Cayman Islands Dollar" },
    Currency { code: "KZT", name: "Kazakhstani Tenge" },
    Currency { code: "LAK", name: "Lao Kip" },
    Currency { code: "LBP", name: "Lebanese Pound" },
    Currency { code: "LKR", name: "Sri Lankan Rupee" },
    Currency { code: "LRD", name: "Liberian Dollar" },
    Currency { code: "LSL", name: "Lesotho Loti" },
    Currency { code: "LYD", name: "Libyan Dinar" },
    Currency { code: "MAD", name: "Moroccan Dirham" },
    Currency { code: "MDL", name: "Moldovan Leu" },
    Currency { code: "MGA", name: "Malagasy Ariary" },
    Currency { code: "MKD", name: "Macedonian Denar" },
    Currency { code: "MMK", name: "Myanmar Kyat" },
    Currency { code: "MNT", name: "Mongolian Tögrög" },
    Currency { code: "MOP", name: "Macanese Pataca" },
    Currency { code: "MRU", name: "Mauritanian Ouguiya" },
    Currency { code: "MUR", name: "Mauritian Rupee" },
    Currency { code: "MVR", name: "Maldivian Rufiyaa" },
    Currency { code: "MWK", name: "Malawian Kwacha" },
    Currency { code: "MXN", name: "Mexican Peso" },
    Currency { code: "MYR", name: "Malaysian Ringgit" },
    Currency { code: "MZN", name: "Mozambican Metical" },
    Currency { code: "NAD", name: "Namibian Dollar" },
    Currency { code: "NGN", name: "Nigerian Naira" },
    Currency { code: "NIO", name: "Nicaraguan Córdoba" },
    Currency { code: "NOK", name: "Norwegian Krone" },
    Currency { code: "NPR", name: "Nepalese Rupee" },
    Currency { code: "NZD", name: "New Zealand Dollar" },
    Currency { code: "OMR", name: "Omani Rial" },
    Currency { code: "PAB", name: "Panamanian Balboa" },
    Currency { code: "PEN", name: "Peruvian Sol" },
    Currency { code: "PGK", name: "Papua New Guinean Kina" },
    Currency { code: "PHP", name: "Philippine Peso" },
    Currency { code: "PKR", name: "Pakistani Rupee" },
    Currency { code: "PLN", name: "Polish Złoty" },
    Currency { code: "PYG", name: "Paraguayan Guaraní" },
    Currency { code: "QAR", name: "Qatari Riyal" },
    Currency { code: "RON", name: "Romanian Leu" },
    Currency { code: "RSD", name: "Serbian Dinar" },
    Currency { code: "RUB", name: "Russian Ruble" },
    Currency { code: "RWF", name: "Rwandan Franc" },
    Currency { code: "SAR", name: "Saudi Riyal" },
    Currency { code: "SBD", name: "Solomon Islands Dollar" },
    Currency { code: "SCR", name: "Seychellois Rupee" },
    Currency { code: "SDG", name: "Sudanese Pound" },
    Currency { code: "SEK", name: "Swedish Krona" },
    Currency { code: "SGD", name: "Singapore Dollar" },
    Currency { code: "SHP", name: "Saint Helena Pound" },
    Currency { code: "SLE", name: "Sierra Leonean Leone" },
    Currency { code: "SOS", name: "Somali Shilling" },
    Currency { code: "SRD", name: "Surinamese Dollar" },
    Currency { code: "SSP", name: "South Sudanese Pound" },
    Currency { code: "STN", name: "São Tomé and Príncipe Dobra" },
    Currency { code: "SVC", name: "Salvadoran Colón" },
    Currency { code: "SYP", name: "Syrian Pound" },
    Currency { code: "SZL", name: "Swazi Lilangeni" },
    Currency { code: "THB", name: "Thai Baht" },
    Currency { code: "TJS", name: "Tajikistani Somoni" },
    Currency { code: "TMT", name: "Turkmenistani Manat" },
    Currency { code: "TND", name: "Tunisian Dinar" },
    Currency { code: "TOP", name: "Tongan Paʻanga" },
    Currency { code: "TRY", name: "Turkish Lira" },
    Currency { code: "TTD", name: "Trinidad and Tobago Dollar" },
    Currency { code: "TWD", name: "New Taiwan Dollar" },
    Currency { code: "TZS", name: "Tanzanian Shilling" },
    Currency { code: "UAH", name: "Ukrainian Hryvnia" },
    Currency { code: "UGX", name: "Ugandan Shilling" },
    Currency { code: "USD", name: "US Dollar" },
    Currency { code: "UYU", name: "Uruguayan Peso" },
    Currency { code: "UZS", name: "Uzbekistani Som" },
    Currency { code: "VES", name: "Venezuelan Bolívar" },
    Currency { code: "VND", name: "Vietnamese Dong" },
    Currency { code: "VUV", name: "Vanuatu Vatu" },
    Currency { code: "WST", name: "Samoan Tala" },
    Currency { code: "XAF", name: "Central African CFA Franc" },
    Currency { code: "XCD", name: "East Caribbean Dollar" },
    Currency { code: "XCG", name: "Caribbean Guilder" },
    Currency { code: "XOF", name: "West African CFA Franc" },
    Currency { code: "XPF", name: "CFP Franc" },
    Currency { code: "YER", name: "Yemeni Rial" },
    Currency { code: "ZAR", name: "South African Rand" },
    Currency { code: "ZMW", name: "Zambian Kwacha" },
    Currency { code: "ZWG", name: "Zimbabwe Gold" },
];

/// The largest exchange rate anyone can enter by hand.
///
/// Not a real-world ceiling (the widest ECB pair is ~4 orders of magnitude) but an overflow guard:
/// `amount * rate` must stay finite, and a rate this size already means a typo.
pub const MAX_EXCHANGE_RATE: f64 = 1e9;

/// Converts a source-currency amount into the project currency.
///
/// `rate` is units of project currency per one unit of source currency, so
/// `amount_project = amount_source * rate`.
///
/// **Called once, on the expense total — never per share.** The converted total is what
/// `distribute` then splits into cents, which is what keeps `sum(payers) == total == sum(debtors)`
/// exact. Converting each share separately and rounding each one is precisely where the drift that
/// `sums_to_total` would reject comes from.
pub fn convert_to_project(source_amount: f64, rate: f64) -> f64 {
    round_currency(source_amount * rate)
}

/// True when a rate is usable: finite, positive, and small enough that `amount * rate` cannot
/// overflow to infinity. Applies to a hand-typed rate and to one parsed off the ECB feed alike.
pub fn is_valid_rate(rate: f64) -> bool {
    rate.is_finite() && rate > 0.0 && rate <= MAX_EXCHANGE_RATE
}

/// The rate to get from `from` into `to`, given an InforEuro table of units-per-EUR.
///
/// Everything is quoted against EUR, so any other pair is a quotient of two of its rates.
/// `None` when either currency is absent or the result is not a usable rate.
pub fn cross_rate(rates: &HashMap<String, f64>, from: &str, to: &str) -> Option<f64> {
    let from_rate = *rates.get(from)?;
    let to_rate = *rates.get(to)?;
    if !is_valid_rate(from_rate) || !is_valid_rate(to_rate) {
        return None;
    }
    let rate = to_rate / from_rate;
    is_valid_rate(rate).then_some(rate)
}

/// The European Commission's InforEuro monthly table, as served by `GET /api/v1/fx/rates`.
///
/// `day` is the first day of the month the rates are valid for — InforEuro fixes one rate per
/// currency per calendar month — and the client shows that month next to the rate.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FxRates {
    pub day: String,
    pub base: String,
    pub rates: HashMap<String, f64>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct EncryptedPair {
    pub ct: String,
    pub iv: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[cfg_attr(feature = "server", sqlx(type_name = "project_status", rename_all = "lowercase"))]
pub enum ProjectStatus {
    Ongoing,
    Closed,
    Archived,
}

// Plaintext payloads: serialised to JSON, then encrypted as a single blob.

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HistoryPayload {
    pub summary: String,
    /// Replaces `project_history.actor_user_id` — see docs/plans/participant-links-encryption.md.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_user_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectPayload {
    pub name: String,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Replaces `projects.status`; `None` means the column still holds it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ProjectStatus>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserPayload {
    pub name: String,
    /// Participants are never deleted once the server cannot see their payments: a removed one
    /// leaves the pickers and keeps its name on past expenses.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub removed: bool,
}

/// One payer's or debtor's part of an expense, inside its payload. Replaces a `payments` row,
/// whose `user_id` was in clear.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExpenseShare {
    pub user_id: i32,
    pub amount: f64,
    pub is_debt: bool,
}

/// `amount` is **always** in the project's currency, whatever currency the expense was entered in.
///
/// That is what makes multi-currency additive: every reader — `balances.rs`, the charts, the export,
/// `payments_are_inconsistent` — keeps comparing one currency and needs no change, and a client
/// built before this feature ignores the three `source_*`/`rate` fields and still shows the right
/// number. The conversion happens once, at write time, on the total (see [`convert_to_project`]).
///
/// The three are `Option` + `default` together: all absent means the expense was entered in the
/// project currency and no conversion took place.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpensePayload {
    pub name: String,
    pub amount: f64,
    pub expense_type: String,
    pub date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// The currency the amount was typed in, when it differs from the project's.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_currency: Option<String>,
    /// The amount as typed, before conversion. Display and audit only — never summed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_amount: Option<f64>,
    /// Units of project currency per one unit of `source_currency`, frozen at write time so the
    /// expense keeps the rate it was booked at rather than drifting with the market.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,
    /// The recurring rule this occurrence was materialized from. Inside the ciphertext, so the
    /// server cannot link an expense to its rule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurring_id: Option<Uuid>,
    /// Added with the previous occurrence's amount, awaiting the real figure.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub estimate: bool,
    /// Replaces `expenses.author_id`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author_id: Option<i32>,
    /// Replaces the `payments` rows. `Some` is the format of
    /// docs/plans/participant-links-encryption.md; `None` means the rows still hold them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shares: Option<Vec<ExpenseShare>>,
}

/// Pads serialised JSON with trailing spaces, which every JSON parser skips, to the next power of
/// two from 256 bytes. XChaCha20 keeps `len(ct) == len(plaintext)`, so without it a payload's length
/// would tell the server how many shares it holds.
pub fn pad_json(mut json: String) -> String {
    let target = json.len().max(256).next_power_of_two();
    json.push_str(&" ".repeat(target - json.len()));
    json
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PaymentPayload {
    pub amount: f64,
    pub is_debt: bool,
}

/// Everything the project page reads, in one request.
///
/// `expenses`/`payments` are withheld (`None` together) when the caller's `data_version` already
/// matches. `project`/`users` are always sent — they are `Option` only so a rollback to a server
/// that predates the merge still deserialises. Why the two halves differ:
/// `docs/plans/expenses-tab-performance.md` §5.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProjectSync {
    pub data_version: i64,
    #[serde(default)]
    pub project: Option<ProjectDto>,
    #[serde(default)]
    pub users: Option<Vec<User>>,
    pub expenses: Option<Vec<Expense>>,
    pub payments: Option<Vec<Payment>>,
    /// Always sent, like `users`: a rule's cursor and the expenses it produced come from the same
    /// snapshot.
    #[serde(default)]
    pub recurring: Option<Vec<RecurringExpense>>,
    /// The server's UTC date: the one clock every member's materialization is measured against.
    #[serde(default)]
    pub server_date: Option<NaiveDate>,
}

/// Sent by a client with nothing cached — no real version can be negative, so it never matches.
pub const NO_CACHED_VERSION: i64 = -1;

/// Ceiling on the participants of one project, and on the payers or debtors of one expense. Lives
/// here so the expense and user controllers cannot drift apart on it.
pub const MAX_PARTICIPANTS: usize = 100;

/// Worst case, from `ui::payment_methods`: 20 methods x ((40 kind + 140 value + 60 label) chars x
/// 4 bytes for UTF-8 + ~48 bytes of JSON syntax incl. `"shared":true`) ~= 20,200 bytes, +16 for
/// the Poly1305 tag, x 4/3 for base64 ~= 27,000. Control characters are rejected client-side
/// precisely so JSON escaping cannot inflate that further. 32768 leaves headroom without turning
/// the row into free storage. The same cap bounds every per-project copy, which is a subset.
pub const MAX_PAYMENT_METHODS_LENGTH: usize = 32768;

/// Ceiling on how many projects one account may hold, and so on how many ids a client may ask
/// about in one batch. A membership row keeps a project alive and there is no way for the real
/// members to evict a stranger, so this bounds how much of other people's data one account can pin.
/// Mitigation, not a fix: registration is open, so the hold can be spread across accounts.
pub const MAX_ACCOUNT_PROJECTS: i64 = 500;

/// An expense with the payment rows written for it — the write's own rows, all belonging to
/// `expense`. Returned by create and edit so the client patches locally instead of refetching
/// ~2 MB for a one-row change.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExpenseWithPayments {
    pub expense: Expense,
    pub payments: Vec<Payment>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: Uuid,
    pub email: String,
    pub display_name: EncryptedPair,
    pub created_at: NaiveDateTime,
    pub email_verified: bool,
    pub kdf_salt: Vec<u8>,
    pub kdf_version: i16,
    /// `Preferences`, encrypted with the account key. `None` for an account that has never saved
    /// any — the server stores and returns it without ever being able to read it.
    pub preferences: Option<EncryptedPair>,
    /// `PaymentMethods`, encrypted with the account key. `None` for an account that has never saved
    /// any. Bank details the server holds and cannot read, same contract as `preferences`.
    pub payment_methods: Option<EncryptedPair>,
    /// The account's X25519 public key — what a friend boxes a project key to. `None` until a
    /// build that generates one has logged in. `default` so an older cached `Account` still loads.
    #[serde(default)]
    pub public_key: Option<Vec<u8>>,
    /// The matching private key, encrypted with the account key. Same contract as `preferences`.
    #[serde(default)]
    pub private_key: Option<EncryptedPair>,
}

/// `PUT /auth/keypair`. Write-once: the server keeps the first keypair an account presents.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Keypair {
    pub public_key: Vec<u8>,
    pub private_key: EncryptedPair,
}

pub const PUBLIC_KEY_LENGTH: usize = 32;

/// Ceiling on one account's outgoing friend requests (pending, accepted or rejected — the
/// requester cannot tell them apart) and on how many it may send in a day.
pub const MAX_FRIEND_REQUESTS: i64 = 300;
pub const MAX_FRIEND_REQUESTS_PER_DAY: i64 = 50;
/// Ceiling on the invitations waiting for one account, and on the ones one project has out.
pub const MAX_PENDING_PROJECT_INVITATIONS: i64 = 100;
pub const MAX_FRIEND_LABEL_LENGTH: usize = 1024;

/// `POST /friends/requests`. The address travels in clear for the `accounts` lookup and is stored
/// as a hash; `label` is what the requester calls this person, ciphertext under their account key.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FriendRequestByEmail {
    pub email: String,
    pub label: EncryptedPair,
}

/// `POST /friends/requests/from-project`: the claimed participant `user_id` of a project the caller
/// holds. The server resolves the account; no account id crosses the wire.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FriendRequestFromProject {
    pub project_id: Uuid,
    pub user_id: i32,
    pub label: EncryptedPair,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct Friend {
    pub id: Uuid,
    pub account_id: Uuid,
    pub email: String,
    pub public_key: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct IncomingFriendRequest {
    pub id: Uuid,
    pub requester_email: String,
    pub created_at: NaiveDateTime,
}

/// No status and no account id, by design: rejected, unanswered and "no such account" must be one
/// state to the requester.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OutgoingFriendRequest {
    pub id: Uuid,
    pub label: EncryptedPair,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FriendsView {
    pub friends: Vec<Friend>,
    pub incoming: Vec<IncomingFriendRequest>,
    pub outgoing: Vec<OutgoingFriendRequest>,
}

/// `POST /projects/{id}/invitations`: the project key boxed to `to_account_id`'s public key.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatableProjectInvitation {
    pub to_account_id: Uuid,
    pub key: EncryptedPair,
    /// The participant the sender created for this friend, preselected when they pick who they
    /// are. Must belong to the project.
    #[serde(default)]
    pub user_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProjectInvitation {
    pub id: Uuid,
    pub project_id: Uuid,
    pub from_email: String,
    pub from_public_key: Vec<u8>,
    pub key: EncryptedPair,
    pub created_at: NaiveDateTime,
    #[serde(default)]
    pub user_id: Option<i32>,
}

/// `GET /projects/{id}/invitations/sent`: the caller's own pending invitations that name a
/// participant, so the edit modal can mark that row as invited.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct SentInvitation {
    pub to_account_id: Uuid,
    pub user_id: i32,
}

/// The locales push bodies are hand-written for (`api::server::push::texts`), *not* the shipped
/// locales — those are `UI_LANGS`. A push token's `lang` is validated against this list; a code
/// outside it would render an English body anyway.
pub const SUPPORTED_LANGS: [&str; 7] = ["en", "fr", "es", "de", "it", "pt", "nl"];

/// Every locale the app ships, by code — the same set as `ui::i18n::SUPPORTED`, which pairs them
/// with endonyms and is asserted equal to this list in its tests. Lives here because `api` cannot
/// depend on `ui` and needs a fixed list to bucket the aggregate language counters against.
pub const UI_LANGS: [&str; 32] = [
    "bs", "cs", "da", "de", "et", "en", "es", "fr", "ga", "hr", "is", "it", "lv", "lt", "hu", "mt",
    "nl", "no", "pl", "pt", "ro", "sq", "sk", "sl", "sr", "fi", "sv", "tr", "bg", "mk", "uk", "el",
];

/// APNs tokens are 64 hex characters and Web Push endpoints a URL of ~200; this is an abuse
/// bound, not a format.
pub const MAX_PUSH_TOKEN_LENGTH: usize = 4096;
/// The server's RFC 8292 VAPID public key: uncompressed P-256, base64url without padding. Android
/// hands it to its UnifiedPush distributor at registration, and push services (FCM included)
/// reject pushes not signed by the matching private key (`VAPID_PRIVATE_KEY_BASE64`). Rotating it
/// means a new app release: every device re-registers under the new key.
pub const VAPID_PUBLIC_KEY: &str =
    "BCNBjcs-UB1CqSwnyluLHXAuHisSuQGxmYLS_nUdU1WuK5xbPmqoGMNurR96xukb5annubzpNDDpTTcqWSGQLnA";
/// Devices per account; the oldest rows are dropped beyond it.
pub const MAX_PUSH_TOKENS_PER_ACCOUNT: i64 = 20;
/// Verification pushes an account may cause per rolling hour. Each one is the only request the
/// server sends to an endpoint before its owner proves they receive it, so this bounds what an
/// account can aim at an arbitrary URL.
pub const PUSH_CHALLENGES_PER_HOUR: i32 = 5;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[cfg_attr(feature = "server", sqlx(type_name = "push_platform", rename_all = "lowercase"))]
pub enum PushPlatform {
    Ios,
    Android,
}

/// `PUT /push/token`. `lang` is the two-letter code the device already sends as the
/// `counted_lang` cookie — the server renders the notification sentence from it, so the platform
/// vendor sees a localised sentence and never a name, email or project name.
///
/// iOS: `token` is the APNs device token and `keys` is absent. Android: `token` is the RFC 8030
/// push endpoint URL and `keys` the RFC 8291 receiver keys the payload is encrypted to.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegisterPushToken {
    pub platform: PushPlatform,
    pub token: String,
    pub lang: String,
    #[serde(default)]
    pub keys: Option<WebPushKeys>,
}

/// Base64url, as the UnifiedPush connector's `PublicKeySet` emits them: `p256dh` an uncompressed
/// P-256 point (65 bytes), `auth` a 16-byte secret.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WebPushKeys {
    pub p256dh: String,
    pub auth: String,
}

/// `DELETE /push/token`.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UnregisterPushToken {
    pub token: String,
}

/// `POST /push/verify`: the proof the server pushed to `token`, returned by the device that
/// received it. Until then the endpoint gets nothing but that one push.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VerifyPushToken {
    pub token: String,
    pub proof: String,
}

/// UI preferences that follow the account rather than the device, serialised to JSON and encrypted
/// before it leaves the client. Every field is `Option` + `serde(default)` so a blob written by an
/// older build still deserialises after a field is added.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Preferences {
    #[serde(default)]
    pub language: Option<String>,
}

/// One way the account holder can be paid back.
///
/// `kind` is a **stable id**, not a translated label and not an enum: it is persisted inside the
/// ciphertext, so renaming one orphans every blob already written, and a `MethodKind::Custom(_)`
/// enum would make a kind added by a newer build fail to deserialise on an older one — taking the
/// whole list down with it instead of the single row nobody recognises. The known ids and their
/// brand names live in the client (`ui::payment_methods::KNOWN_KINDS`); anything else, including
/// `other`, is carried as-is.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethod {
    pub kind: String,
    pub value: String,
    /// The holder's own name for it — required when `kind` is `other`, optional otherwise (two
    /// IBANs need telling apart).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Visible to the members of every project this account claimed an identity in: the client
    /// re-encrypts the shared subset under each **project** key. Skipped when false so an unshared
    /// method serialises exactly as it did before the flag existed.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub shared: bool,
}

/// The account's payment details, serialised to JSON and encrypted before they leave the client.
///
/// A struct rather than a bare `Vec` so a later field — an ordering hint, say — can be added
/// without invalidating blobs already written, the same reason `Preferences` is a struct with one
/// field. Also the plaintext of the per-project copy, where it holds only the shared methods.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethods {
    #[serde(default)]
    pub methods: Vec<PaymentMethod>,
}

/// The shared subset of an account's payment methods, encrypted under one project's key.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectPaymentMethods {
    pub project_id: Uuid,
    pub payment_methods: EncryptedPair,
}

/// Body of `PUT /api/v1/auth/payment-methods`. `account` is flattened so the bare `{ct, iv}` an
/// older client sends still deserialises, with `shared` empty — and the server clears every
/// per-project copy it holds for the account before writing the ones listed, so a client that
/// cannot produce them leaves none stale.
///
/// This is the only writer of the per-project copies. `expected_iv` is the `iv` of the account
/// blob the copies were derived from: when set, the server writes nothing unless that is still
/// the stored blob, so a device holding an older list can never put back what a newer save
/// withdrew. A fresh nonce per encryption is what makes the `iv` a version.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePaymentMethods {
    #[serde(flatten)]
    pub account: EncryptedPair,
    #[serde(default)]
    pub shared: Vec<ProjectPaymentMethods>,
    #[serde(default)]
    pub expected_iv: Option<String>,
}

/// Byte lengths the auth endpoints accept. Shared so the client that produces them and the server
/// that checks them cannot drift.
pub const LOGIN_SALT_LENGTH: usize = 16;
pub const LOGIN_PROOF_LENGTH: usize = 32;
/// Password bounds, enforced on the client only: the server never sees the password, so the
/// length is not its to check.
pub const MIN_PASSWORD_LENGTH: usize = 8;
pub const MAX_PASSWORD_LENGTH: usize = 128;

/// How the login proof is derived from the password. The server only ever holds a hash of the
/// proof; the version tells the client which derivation to run.
///
/// - `1`: accounts registered before the proof existed. Their stored Argon2id hash *was* the
///   server's own verification (`m=19456, t=2, p=1`, PHC salt), so the proof is that exact
///   computation done client-side. Upgraded to `2` on the first successful login.
/// - `2`: Argon2id with a client-generated salt and the `counted-login-v2` secret, so the proof is
///   structurally distinct from the account key even under a colliding salt.
pub const AUTH_VERSION_LEGACY: i16 = 1;
pub const AUTH_VERSION_CURRENT: i16 = 2;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegisterPayload {
    pub email: String,
    /// Argon2id of the password, derived on the client. The password itself never leaves it.
    pub proof: Vec<u8>,
    pub login_salt: Vec<u8>,
    pub display_name: EncryptedPair,
    pub kdf_salt: Vec<u8>,
    /// Whether this device already held an anonymous project membership. Feeds an aggregate daily
    /// counter only, never the account row. `default` so app builds predating the field still
    /// register.
    #[serde(default)]
    pub had_anonymous_membership: bool,
    /// Generated alongside the salts. `default` so an older build registers without one and seeds
    /// it at its first login on a newer build.
    #[serde(default)]
    pub keypair: Option<Keypair>,
    /// The interface language at the time of the request. Feeds an aggregate daily counter only,
    /// never the account row. `default` so app builds predating the field still register.
    #[serde(default)]
    pub lang: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoginSaltPayload {
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LoginSalt {
    pub salt: Vec<u8>,
    pub auth_version: i16,
}

/// A legacy account re-deriving under the current scheme, sent alongside its legacy proof so one
/// successful login moves it off `AUTH_VERSION_LEGACY`.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoginUpgrade {
    pub login_salt: Vec<u8>,
    pub proof: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoginPayload {
    pub email: String,
    /// `default` so a build that still sends `password` deserialises and is told it is outdated
    /// instead of failing on shape; the server never reads the field it sent.
    #[serde(default)]
    pub proof: Vec<u8>,
    #[serde(default)]
    pub upgrade: Option<LoginUpgrade>,
    /// The interface language at the time of the request. Feeds an aggregate daily counter only,
    /// never the account row. `default` so app builds predating the field still log in.
    #[serde(default)]
    pub lang: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VerifyEmailPayload {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResendVerificationPayload {
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectDto {
    pub id: Uuid,
    pub payload: EncryptedPair,
    pub status: ProjectStatus,
    pub created_at: NaiveDateTime,
    /// Server-side only. An account id is stable across projects, so returning it to every holder
    /// of a project UUID would let two leaked share links prove the same person is in both — the
    /// exact correlation `anonymous_project_members.member_id` is per-project to prevent. The
    /// server still needs it for the ownership handover in `leave_project`, so the field stays and
    /// only the wire drops it.
    #[serde(skip_serializing, default)]
    pub owner_account_id: Option<Uuid>,
    /// The public demo. Enforced by triggers in the database; the client only hides the controls.
    #[serde(default)]
    pub read_only: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatableProject {
    pub payload: EncryptedPair,
    /// `SHA-256(claim_token)`, so a project is born with its verifier and never has a window in
    /// which anyone holding the UUID could seed one. `default` for builds predating it.
    #[serde(default)]
    pub claim_verifier: Option<Vec<u8>>,
    /// A visitor's throwaway copy from `/demo`: never owned, never a member, swept after 24h.
    #[serde(default)]
    pub demo: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EditableProject {
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<EncryptedPair>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ProjectStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<HistoryContext>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BatchProject {
    pub ids: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct AccountProject {
    pub project_id: Uuid,
    pub user_id: Option<i32>,
    /// The project's E2EE key, encrypted under the account key — see `docs/e2ee.md`, "Key escrow".
    /// `None` for a membership recorded before escrow shipped, or by a device that held the
    /// membership without the key. `default` so a payload from an older client still deserializes.
    #[serde(default)]
    pub key: Option<EncryptedPair>,
    /// Whether `projects.claim_verifier` is set. A key-holding client pushes its `claim_token` when
    /// it is not, closing the pre-verifier window for projects that predate it.
    #[serde(default)]
    pub claim_verifier_seeded: bool,
    /// Whether the server holds this account's shared payment methods under this project's key.
    /// When it does not and the account shares something, a key-holding client re-issues
    /// `PUT /auth/payment-methods` from a fresh `me()` — see `ui::payment_methods::refill`.
    #[serde(default)]
    pub payment_methods_shared: bool,
}

/// Bytes the claim proof is made of. Shared so client and server agree on the lengths.
pub const CLAIM_TOKEN_LENGTH: usize = 32;

/// Base64 lengths the membership write accepts for the blobs it carries. A wrapped key is the
/// 43-character fragment plus a 16-byte tag (80 chars) with a 24-byte nonce (32 chars); a claim
/// label is one display name. Bounded per field so the row cannot be used as free storage.
pub const MAX_WRAPPED_KEY_LENGTH: usize = 128;
pub const MAX_CLAIM_LABEL_LENGTH: usize = 4096;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpsertAccountProject {
    pub project_id: Uuid,
    pub user_id: Option<i32>,
    /// Wrapped project key. `None` means "I do not have it" — never "clear it": both upserts
    /// COALESCE, so a device without the key cannot wipe one another device escrowed.
    #[serde(default)]
    pub key: Option<EncryptedPair>,
    /// This account's `display_name`, re-encrypted under the **project** key so the other members
    /// can read who claimed the identity. Same "never clear it" contract as `key`.
    #[serde(default)]
    pub claim_label: Option<EncryptedPair>,
    /// `SHA-256("counted-claim-v1" || project_key)` — proof of holding the key. Required whenever
    /// `user_id` is set and the project has a verifier; also what seeds the verifier when it has
    /// none. The server compares hashes and stores hashes, never the token.
    #[serde(default)]
    pub claim_token: Option<Vec<u8>>,
}

/// What the login-time reconciliation actually managed to write.
///
/// `identity_rejected` carries the projects whose `user_id` was dropped because another account
/// already claimed that participant. The client clears those locally, which both forces a fresh
/// pick and stops `account_sync::to_push` from re-sending the same rejected claim on every load.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BatchUpsertResult {
    pub accepted: Vec<Uuid>,
    #[serde(default)]
    pub identity_rejected: Vec<Uuid>,
    /// Projects whose `user_id` was dropped because the claim carried no valid `claim_token` or
    /// named a participant of another project. Distinct from `identity_rejected` on purpose: the
    /// client keeps its local identity here — the fix is a key, not a new pick.
    #[serde(default)]
    pub proof_rejected: Vec<Uuid>,
}

/// Registers a holder the server has no identity for. `member_id` is random and per-project, and
/// travels in the request body, never a URL — so it never reaches the access logs.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JoinProject {
    pub project_id: Uuid,
    pub member_id: Uuid,
}

/// `member_id` is set when this device joined anonymously. The account side is read from the
/// session — never from the payload.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeaveProject {
    pub project_id: Uuid,
    pub member_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub payload: EncryptedPair,
    pub created_at: Option<NaiveDateTime>,
    /// An account holds this participant as its identity in the project. Blocks anyone else from
    /// picking it — a coherence guarantee, not an authorisation one: `user_id` grants nothing
    /// server-side. See docs/project-membership.md.
    #[serde(default)]
    pub claimed: bool,
    /// The claimant's account display name, encrypted under the **project** key. `None` when the
    /// claim was made without an account key available, or before this shipped. `default` so the
    /// `User` rows cached in localStorage by an older build still deserialize.
    #[serde(default)]
    pub claim_label: Option<EncryptedPair>,
    /// The claimant's shared payment methods, encrypted under the **project** key. `None` when the
    /// claimant shares nothing, or when there is no claimant.
    #[serde(default)]
    pub payment_methods: Option<EncryptedPair>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreatableUserBatch {
    Single(CreatableUser),
    Multiple(Vec<CreatableUser>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatableUser {
    pub payload: EncryptedPair,
    pub project_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct UserProjects {
    pub project_id: Uuid,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[cfg_attr(feature = "server", sqlx(type_name = "history_action", rename_all = "lowercase"))]
pub enum HistoryAction {
    Created,
    Updated,
    Deleted,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "server", derive(sqlx::Type))]
#[cfg_attr(feature = "server", sqlx(type_name = "history_entity", rename_all = "lowercase"))]
pub enum HistoryEntity {
    Expense,
    Project,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HistoryContext {
    pub actor_user_id: i32,
    pub payload: EncryptedPair,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HistoryEntry {
    pub id: i64,
    pub project_id: Uuid,
    pub actor_user_id: Option<i32>,
    /// Server-side only, for the same reason as `ProjectDto::owner_account_id`: it is a stable
    /// cross-project identifier and every history read is open to any project-UUID holder.
    #[serde(skip_serializing, default)]
    pub actor_account_id: Option<Uuid>,
    pub action: HistoryAction,
    pub entity: HistoryEntity,
    pub entity_id: Option<i32>,
    pub payload: EncryptedPair,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Expense {
    pub id: i32,
    /// `None` once the participant who recorded it has been removed — `expenses.author_id` is
    /// `ON DELETE SET NULL`, so an expense outlives its author rather than pinning them in the
    /// project forever. `default` so blobs written by an older client still deserialize.
    #[serde(default)]
    pub author_id: Option<i32>,
    pub project_id: Uuid,
    pub created_at: NaiveDateTime,
    pub payload: EncryptedPair,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ExpenseType {
    Expense,
    Transfer,
    Gain,
}

impl fmt::Display for ExpenseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ExpenseType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ExpenseType::Expense => "expense",
            ExpenseType::Transfer => "transfer",
            ExpenseType::Gain => "gain",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EncryptedUserAmount {
    pub user_id: i32,
    pub payload: EncryptedPair,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatableExpense {
    pub project_id: Uuid,
    pub author_id: i32,
    pub payload: EncryptedPair,
    pub payers: Vec<EncryptedUserAmount>,
    pub debtors: Vec<EncryptedUserAmount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<HistoryContext>,
    /// Set only by the offline queue: the `QueuedOp.id`, sent on every replay. A replay whose
    /// earlier attempt did commit gets that expense back instead of a duplicate. `None` online.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_op_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EditableExpense {
    pub id: i32,
    pub project_id: Uuid,
    /// Optional, unlike `CreatableExpense::author_id`: creating always has an author, but editing
    /// an expense whose author has since been removed must be able to write the null back.
    #[serde(default)]
    pub author_id: Option<i32>,
    pub payload: EncryptedPair,
    pub payers: Vec<EncryptedUserAmount>,
    pub debtors: Vec<EncryptedUserAmount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<HistoryContext>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeleteExpenseRequest {
    pub id: i32,
    pub project_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<HistoryContext>,
}

/// A recurring expense rule. `payload` holds the rule, the expense template, the cursor and the
/// seed of the occurrences' `client_op_id`s, all under the project key — the server knows neither
/// when it is due nor what it is for. See docs/plans/recurring-expenses.md.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecurringExpense {
    pub id: Uuid,
    pub project_id: Uuid,
    #[serde(default)]
    pub author_id: Option<i32>,
    pub payload: EncryptedPair,
    pub version: i64,
    pub created_at: NaiveDateTime,
}

/// `participant_ids` are the template's payers and debtors. A participant listed here cannot be
/// removed from the project while the rule exists, and every occurrence is checked against them.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatableRecurringExpense {
    pub project_id: Uuid,
    pub author_id: i32,
    pub participant_ids: Vec<i32>,
    pub payload: EncryptedPair,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history: Option<HistoryContext>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EditableRecurringExpense {
    pub id: Uuid,
    pub project_id: Uuid,
    pub participant_ids: Vec<i32>,
    pub payload: EncryptedPair,
    pub expected_version: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history: Option<HistoryContext>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeleteRecurringExpenseRequest {
    pub id: Uuid,
    pub project_id: Uuid,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history: Option<HistoryContext>,
}

/// Claims the occurrences due up to `due_through` and writes them, in one transaction: the rule's
/// cursor (inside `payload`) advances if and only if every occurrence is written. `due_through` is
/// the only date the server sees, and only to refuse one past tomorrow.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MaterializeRecurringRequest {
    pub id: Uuid,
    pub project_id: Uuid,
    pub expected_version: i64,
    pub payload: EncryptedPair,
    pub due_through: NaiveDate,
    pub occurrences: Vec<CreatableExpense>,
}

/// Bounds one materialization's transaction. A longer catch-up is several calls, each advancing
/// the cursor by at most this many occurrences.
pub const MAX_OCCURRENCES_PER_CALL: usize = 50;

pub const MAX_RECURRING_PER_PROJECT: i64 = 50;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    pub id: i32,
    pub expense_id: i32,
    pub user_id: i32,
    pub payload: EncryptedPair,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewPayment {
    pub expense_id: i32,
    pub user_id: i32,
    pub payload: EncryptedPair,
}

// Balance / summary: computed client-side after decryption.

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UserSummary {
    pub reimbursement_suggestions: Vec<ReimbursementSuggestion>,
    pub summary: HashMap<i32, f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserBalance {
    pub amount: f64,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserBalanceComputation {
    pub amount: f64,
    pub remaining_amount: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ReimbursementSuggestion {
    pub amount: f64,
    pub user_id_debtor: i32,
    pub user_id_payer: i32,
}

#[cfg(test)]
mod tests {
    use super::{
        convert_to_project, cross_rate, is_valid_rate, pad_json, sums_to_total, to_cents,
        ExpensePayload, ExpenseShare, HashMap, HistoryAction, HistoryEntity, HistoryPayload,
        ProjectPayload, ProjectStatus, UserPayload, CURRENCIES, MAX_EXCHANGE_RATE,
    };

    #[test]
    fn to_cents_quantises_plain_values() {
        assert_eq!(to_cents(0.0), 0);
        assert_eq!(to_cents(10.0), 1000);
        assert_eq!(to_cents(33.33), 3333);
        assert_eq!(to_cents(99999.99), 9999999);
        assert_eq!(to_cents(1_000_000.99), 100_000_099);
    }

    #[test]
    fn to_cents_absorbs_float_drift() {
        // 0.1 + 0.2 == 0.30000000000000004
        assert_eq!(to_cents(0.1 + 0.2), 30);
        // 33.33 + 33.33 + 33.34 == 100.00000000000001
        assert_eq!(to_cents(33.33 + 33.33 + 33.34), 10000);
    }

    #[test]
    fn to_cents_saturates_instead_of_panicking() {
        // A tampered payload decrypts to whatever it wants — the cast must not be UB or a panic.
        assert_eq!(to_cents(f64::NAN), 0);
        assert_eq!(to_cents(f64::INFINITY), i64::MAX);
        assert_eq!(to_cents(f64::NEG_INFINITY), i64::MIN);
    }

    #[test]
    fn sums_to_total_accepts_an_exact_split() {
        assert!(sums_to_total(100.0, [50.0, 50.0]));
        assert!(sums_to_total(100.0, [100.0]));
        assert!(sums_to_total(100.0, [33.33, 33.33, 33.34]));
    }

    #[test]
    fn sums_to_total_rejects_a_one_cent_gap() {
        assert!(!sums_to_total(100.0, [50.0, 49.99]));
        assert!(!sums_to_total(100.0, [50.0, 50.01]));
        assert!(!sums_to_total(100.0, [33.33, 33.33, 33.33]));
    }

    #[test]
    fn sums_to_total_ignores_sub_cent_noise() {
        assert!(sums_to_total(100.0, [99.999999]));
        assert!(sums_to_total(100.0, [0.1, 0.2, 99.7]));
    }

    #[test]
    fn sums_to_total_rejects_the_reported_scenario() {
        // 100 total, two payers at 60, two debtors at 70.
        assert!(!sums_to_total(100.0, [60.0, 60.0]));
        assert!(!sums_to_total(100.0, [70.0, 70.0]));
    }

    #[test]
    fn sums_to_total_on_an_empty_side() {
        assert!(sums_to_total(0.0, []));
        assert!(!sums_to_total(10.0, []));
    }

    #[test]
    fn sums_to_total_does_not_drift_over_many_rows() {
        let rows = vec![0.01; 100];
        assert!(sums_to_total(1.0, rows));
    }

    #[test]
    fn test_history_action_serde() {
        assert_eq!(serde_json::to_string(&HistoryAction::Created).unwrap(), "\"created\"");
        assert_eq!(serde_json::to_string(&HistoryAction::Updated).unwrap(), "\"updated\"");
        assert_eq!(serde_json::to_string(&HistoryAction::Deleted).unwrap(), "\"deleted\"");
        let rt: HistoryAction = serde_json::from_str("\"created\"").unwrap();
        assert_eq!(rt, HistoryAction::Created);
    }

    #[test]
    fn test_history_entity_serde() {
        assert_eq!(serde_json::to_string(&HistoryEntity::Expense).unwrap(), "\"expense\"");
        assert_eq!(serde_json::to_string(&HistoryEntity::Project).unwrap(), "\"project\"");
        let rt: HistoryEntity = serde_json::from_str("\"project\"").unwrap();
        assert_eq!(rt, HistoryEntity::Project);
    }

    fn ecb_table() -> HashMap<String, f64> {
        // Real values from the 2026-09-04 publication.
        [("EUR", 1.0), ("USD", 1.1622), ("GBP", 0.85898), ("JPY", 181.59)]
            .into_iter()
            .map(|(c, r)| (c.to_string(), r))
            .collect()
    }

    #[test]
    fn convert_rounds_to_the_cent() {
        assert_eq!(convert_to_project(135.0, 0.860437), 116.16);
        assert_eq!(convert_to_project(0.005, 1.0), 0.01);
        assert_eq!(convert_to_project(10.0, 1.0), 10.0);
    }

    /// The whole point of converting the total and not the shares: whatever the rate, the converted
    /// total is a whole number of cents, so `distribute` can always split it exactly.
    #[test]
    fn converted_total_is_always_a_whole_number_of_cents() {
        for rate in [0.85898, 1.1622, 181.59, 0.000001, 1e9] {
            for amount in [1.0, 33.33, 135.0, 999999.99] {
                let converted = convert_to_project(amount, rate);
                assert_eq!(
                    converted,
                    to_cents(converted) as f64 / 100.0,
                    "amount {amount} rate {rate} did not land on a cent"
                );
            }
        }
    }

    #[test]
    fn cross_rate_round_trips_through_eur() {
        let t = ecb_table();
        let there = cross_rate(&t, "EUR", "USD").unwrap();
        let back = cross_rate(&t, "USD", "EUR").unwrap();
        assert_eq!(there, 1.1622);
        assert!((there * back - 1.0).abs() < 1e-12);
    }

    /// Neither leg is EUR — the quotient is the only way to get there.
    #[test]
    fn cross_rate_between_two_non_eur_currencies() {
        let t = ecb_table();
        let rate = cross_rate(&t, "GBP", "JPY").unwrap();
        assert!((rate - (181.59 / 0.85898)).abs() < 1e-9);
    }

    #[test]
    fn cross_rate_rejects_unknown_currency() {
        let t = ecb_table();
        assert!(cross_rate(&t, "EUR", "XXX").is_none());
        assert!(cross_rate(&t, "XXX", "EUR").is_none());
    }

    #[test]
    fn cross_rate_rejects_a_poisoned_table() {
        for bad in [f64::NAN, f64::INFINITY, 0.0, -1.0] {
            let mut t = ecb_table();
            t.insert("USD".to_string(), bad);
            assert!(cross_rate(&t, "EUR", "USD").is_none(), "accepted {bad}");
            assert!(cross_rate(&t, "USD", "EUR").is_none(), "accepted {bad} as the source");
        }
    }

    #[test]
    fn rate_validation_bounds() {
        assert!(is_valid_rate(1.0));
        assert!(is_valid_rate(0.000001));
        assert!(is_valid_rate(MAX_EXCHANGE_RATE));
        assert!(!is_valid_rate(0.0));
        assert!(!is_valid_rate(-1.0));
        assert!(!is_valid_rate(f64::NAN));
        assert!(!is_valid_rate(f64::INFINITY));
        assert!(!is_valid_rate(MAX_EXCHANGE_RATE * 10.0));
    }

    /// A blob written before multi-currency existed must still open, and must read as
    /// "no conversion" rather than failing the whole expense.
    #[test]
    fn expense_payload_without_currency_fields_still_deserialises() {
        let old = r#"{"name":"Pizza","amount":30.0,"expenseType":"expense","date":"2026-09-01"}"#;
        let p: ExpensePayload = serde_json::from_str(old).unwrap();
        assert_eq!(p.amount, 30.0);
        assert!(p.source_currency.is_none());
        assert!(p.source_amount.is_none());
        assert!(p.rate.is_none());
    }

    /// And an expense with no conversion must not grow the three keys — the ciphertext length is
    /// observable, so a same-currency expense has to stay byte-identical to what it was.
    #[test]
    fn expense_payload_omits_currency_fields_when_absent() {
        let p = ExpensePayload {
            name: "Pizza".to_string(),
            amount: 30.0,
            expense_type: "expense".to_string(),
            date: "2026-09-01".to_string(),
            description: None,
            category: None,
            source_currency: None,
            source_amount: None,
            rate: None,
            recurring_id: None,
            estimate: false,
            author_id: None,
            shares: None,
        };
        let json = serde_json::to_string(&p).unwrap();
        assert!(!json.contains("sourceCurrency"), "{json}");
        assert!(!json.contains("sourceAmount"), "{json}");
        assert!(!json.contains("rate"), "{json}");
        assert!(!json.contains("recurringId"), "{json}");
        assert!(!json.contains("estimate"), "{json}");
    }

    #[test]
    fn expense_payload_round_trips_with_currency_fields() {
        let p = ExpensePayload {
            name: "Diner".to_string(),
            amount: 116.16,
            expense_type: "expense".to_string(),
            date: "2026-09-01".to_string(),
            description: None,
            category: None,
            source_currency: Some("USD".to_string()),
            source_amount: Some(135.0),
            rate: Some(0.860437),
            recurring_id: None,
            estimate: false,
            author_id: None,
            shares: None,
        };
        let json = serde_json::to_string(&p).unwrap();
        assert!(json.contains("\"sourceCurrency\":\"USD\""), "{json}");
        let rt: ExpensePayload = serde_json::from_str(&json).unwrap();
        assert_eq!(rt.source_amount, Some(135.0));
        assert_eq!(rt.rate, Some(0.860437));
        assert_eq!(rt.amount, 116.16);
    }

    #[test]
    fn currency_list_is_sane() {
        assert_eq!(CURRENCIES[0].code, "EUR", "EUR must lead — it is the base and the default");
        let codes: Vec<&str> = CURRENCIES.iter().map(|c| c.code).collect();
        let mut sorted = codes[1..].to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted, codes[1..], "must be sorted and unique after EUR");
        assert!(!sorted.contains(&"EUR"));
        assert!(CURRENCIES.iter().all(|c| {
            c.code.len() == 3
                && c.code.chars().all(|ch| ch.is_ascii_uppercase())
                && !c.name.trim().is_empty()
        }));
        assert!(CURRENCIES.len() > 150, "{} currencies", CURRENCIES.len());
    }

    #[test]
    fn pad_json_doubles_from_256() {
        assert_eq!(pad_json("{}".to_string()).len(), 256);
        assert_eq!(pad_json("x".repeat(256)).len(), 256);
        assert_eq!(pad_json("x".repeat(257)).len(), 512);
        assert_eq!(pad_json("x".repeat(3000)).len(), 4096);
    }

    #[test]
    fn a_padded_payload_parses() {
        let json = pad_json(r#"{"summary":"Edited","actorUserId":4}"#.to_string());
        let p: HistoryPayload = serde_json::from_str(&json).unwrap();
        assert_eq!(p.actor_user_id, Some(4));
    }

    /// Phase 1 of docs/plans/participant-links-encryption.md only reads the new fields: what it
    /// writes must stay byte-identical to the format every older client reads.
    #[test]
    fn payloads_without_the_new_fields_serialise_as_before() {
        let user = serde_json::to_string(&UserPayload { name: "Ann".into(), removed: false }).unwrap();
        assert_eq!(user, r#"{"name":"Ann"}"#);
        let history = serde_json::to_string(&HistoryPayload { summary: "s".into(), actor_user_id: None }).unwrap();
        assert_eq!(history, r#"{"summary":"s"}"#);
        let project = ProjectPayload { name: "T".into(), currency: "EUR".into(), description: None, status: None };
        assert_eq!(serde_json::to_string(&project).unwrap(), r#"{"name":"T","currency":"EUR"}"#);
        let expense: ExpensePayload = serde_json::from_str(
            r#"{"name":"P","amount":1.0,"expenseType":"expense","date":"2026-10-01"}"#,
        )
        .unwrap();
        let json = serde_json::to_string(&expense).unwrap();
        assert!(!json.contains("authorId") && !json.contains("shares"), "{json}");
    }

    #[test]
    fn the_new_fields_round_trip() {
        let p: ProjectPayload =
            serde_json::from_str(r#"{"name":"T","currency":"EUR","status":"archived"}"#).unwrap();
        assert_eq!(p.status, Some(ProjectStatus::Archived));
        let u: UserPayload = serde_json::from_str(r#"{"name":"Ann","removed":true}"#).unwrap();
        assert!(u.removed);
        let e: ExpensePayload = serde_json::from_str(
            r#"{"name":"P","amount":2.0,"expenseType":"expense","date":"2026-10-01","authorId":3,
                "shares":[{"userId":3,"amount":2.0,"isDebt":false}]}"#,
        )
        .unwrap();
        assert_eq!(e.author_id, Some(3));
        assert_eq!(e.shares, Some(vec![ExpenseShare { user_id: 3, amount: 2.0, is_debt: false }]));
    }
}
