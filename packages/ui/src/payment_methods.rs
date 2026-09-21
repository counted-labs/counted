//! How the account holder wants to be paid back.
//!
//! The server stores them as an opaque blob encrypted with the account key, exactly as it does the
//! display name and the preferences, so it holds bank details it can never read. Everything that
//! decides *what* is valid and *what* is stored lives here; the card only calls in.

use api::auth::auth_controller::update_payment_methods;
use dioxus::fullstack::Json;
use shared::{
    Account, EncryptedPair, PaymentMethod, PaymentMethods, ProjectPaymentMethods,
    UpdatePaymentMethods,
};

use crate::{
    common::{key_of, read_from_ls, LocalStorageProject},
    crypto::{decrypt_json, encrypt_json},
};

/// Stable ids and their brand names.
///
/// The id is persisted inside the ciphertext — **never rename one**, or every blob already written
/// points at a kind that no longer exists. The brand names are not translated either: "PayPal" is
/// "PayPal" in every locale. Only `other` gets a translated label, at the render site.
///
/// Adding a kind here is safe in both directions: an older build renders an id it does not know as
/// itself rather than failing the blob.
/// Ordered as it renders: the ones that cross borders first, then the domestic rails
/// alphabetically, then `other` last. Nothing here is regional gatekeeping — a Brazilian can pick
/// Swish and a Swede can pick Pix; the list is a convenience, and `other` covers the rest.
pub const KNOWN_KINDS: &[(&str, &str)] = &[
    ("iban", "IBAN"),
    ("paypal", "PayPal"),
    ("revolut", "Revolut"),
    ("wise", "Wise"),
    ("wero", "Wero"),
    ("alipay", "Alipay"),
    ("bizum", "Bizum"),
    ("blik", "BLIK"),
    ("cashapp", "Cash App"),
    ("gcash", "GCash"),
    ("interac", "Interac e-Transfer"),
    ("kakaopay", "KakaoPay"),
    ("lydia", "Lydia"),
    ("mbway", "MB WAY"),
    ("mobilepay", "MobilePay"),
    ("payid", "PayID"),
    ("paynow", "PayNow"),
    ("pix", "Pix"),
    ("promptpay", "PromptPay"),
    ("satispay", "Satispay"),
    ("swish", "Swish"),
    ("twint", "TWINT"),
    ("upi", "UPI"),
    ("venmo", "Venmo"),
    ("vipps", "Vipps"),
    ("wechatpay", "WeChat Pay"),
    ("zelle", "Zelle"),
    ("other", ""),
];

pub const OTHER_KIND: &str = "other";
pub const DEFAULT_KIND: &str = "iban";

pub const MAX_METHODS: usize = 20;
pub const MAX_VALUE_LEN: usize = 140;
pub const MAX_LABEL_LEN: usize = 60;
/// Bounds the one field the user does not type. Nothing in `KNOWN_KINDS` comes close, but the kind
/// round-trips out of the ciphertext unvalidated so a tampered or future client could put anything
/// there, and the server's byte cap is sized against this.
pub const MAX_KIND_LEN: usize = 40;

/// The account's stored methods, or empty when it never saved any, the blob does not decrypt, or
/// the key is the wrong one — a session restored from the cookie has no account key at all, and
/// the display name already degrades the same way.
pub fn account_payment_methods(account: &Account, key: &[u8; 32]) -> Vec<PaymentMethod> {
    let Some(pair) = account.payment_methods.as_ref() else {
        return Vec::new();
    };
    decrypt_json::<PaymentMethods>(key, pair).map(|p| p.methods).unwrap_or_default()
}

/// True for a row the user added and did not fill in — not an error to shout about.
pub fn is_blank(method: &PaymentMethod) -> bool {
    method.value.trim().is_empty() && method.label.as_deref().unwrap_or("").trim().is_empty()
}

/// Trims one row and checks it, returning what would be stored.
///
/// Returns an i18n key on the first problem rather than a sentence, so it stays testable without a
/// Dioxus runtime. Public because each row on the card saves on its own: one row's mistake must not
/// block the row next to it.
pub fn validate_method(method: &PaymentMethod) -> Result<PaymentMethod, &'static str> {
    let value = method.value.trim();
    let label = method.label.as_deref().unwrap_or("").trim().to_string();
    let kind = method.kind.trim();

    if value.is_empty() {
        return Err("payment-method-value-required");
    }
    if kind == OTHER_KIND && label.is_empty() {
        return Err("payment-method-label-required");
    }
    if value.chars().count() > MAX_VALUE_LEN
        || label.chars().count() > MAX_LABEL_LEN
        || kind.chars().count() > MAX_KIND_LEN
    {
        return Err("payment-method-too-long");
    }
    // Nothing legitimate carries a control character, and they are invisible: a newline pasted
    // into an IBAN would render as a space and copy back as a broken value. Rejecting them also
    // caps how far JSON escaping can inflate the blob, which is what the server's byte cap is
    // sized against.
    if value.chars().chain(label.chars()).chain(kind.chars()).any(char::is_control) {
        return Err("payment-method-invalid-characters");
    }

    Ok(PaymentMethod {
        kind: kind.to_string(),
        value: value.to_string(),
        label: (!label.is_empty()).then_some(label),
        shared: method.shared,
    })
}

/// What the other members of a project get to see: the shared rows, with the flag dropped since
/// it is implied by the copy and would only lengthen the ciphertext.
pub fn shared_methods(methods: &[PaymentMethod]) -> Vec<PaymentMethod> {
    methods
        .iter()
        .filter(|m| m.shared)
        .map(|m| PaymentMethod { shared: false, ..m.clone() })
        .collect()
}

/// One copy per project this device holds a key **and an identity** for — a copy on a row with
/// no `user_id` is served to nobody, and the server refuses it for the same reason. Empty when
/// nothing is shared, which the server turns into "clear every copy".
pub fn project_copies(
    methods: &[PaymentMethod],
    projects: &[LocalStorageProject],
) -> Vec<ProjectPaymentMethods> {
    let methods = shared_methods(methods);
    if methods.is_empty() {
        return Vec::new();
    }
    let plaintext = PaymentMethods { methods };
    projects
        .iter()
        .filter(|p| p.user_id.is_some())
        .filter_map(|p| {
            let key = key_of(p)?;
            let payment_methods = encrypt_json(&key, &plaintext).ok()?;
            Some(ProjectPaymentMethods { project_id: p.project_id, payment_methods })
        })
        .collect()
}

/// Trims every field and drops rows the user started and left blank, then checks what is left.
pub fn validate_methods(methods: &[PaymentMethod]) -> Result<Vec<PaymentMethod>, &'static str> {
    let mut cleaned: Vec<PaymentMethod> = Vec::new();

    for method in methods {
        if is_blank(method) {
            continue;
        }
        cleaned.push(validate_method(method)?);
    }

    if cleaned.len() > MAX_METHODS {
        return Err("payment-method-limit");
    }

    Ok(cleaned)
}

/// Why a save did not land.
#[derive(Debug, Clone, PartialEq)]
pub enum SaveFailure {
    /// Another device saved since the list this one edited was loaded. Reload, then retry.
    Stale,
    Other(String),
}

/// Encrypts and stores the list, returning the pair so the caller can refresh the account in
/// context without refetching.
///
/// Unlike `preferences::push_language` this surfaces its error: a language that failed to sync
/// leaves the local choice driving the UI, but an IBAN that failed to save is simply gone, and the
/// user is the only one who can do anything about it.
///
/// The per-project copies ride along, derived from the same list: the server replaces every copy
/// it holds with these, so a project whose key is not on this device loses its copy until the
/// projects page next runs [`refill_project_copies`] on a device that has it — absent for a while
/// rather than stale.
///
/// `expected_iv` is the nonce of the blob the edited list came from; the server refuses the write
/// when it is no longer the stored one.
pub async fn save_payment_methods(
    key: [u8; 32],
    methods: Vec<PaymentMethod>,
    expected_iv: Option<String>,
) -> Result<EncryptedPair, SaveFailure> {
    let shared = project_copies(&methods, &read_from_ls().projects);
    let pair = encrypt_json(&key, &PaymentMethods { methods }).map_err(SaveFailure::Other)?;
    put(UpdatePaymentMethods { account: pair.clone(), shared, expected_iv }).await?;
    Ok(pair)
}

/// Re-issues the account's current blob with copies for every project this device can encrypt
/// for. Called when the projects page finds a copy missing (`account_sync::copy_missing`).
///
/// `fresh` must come from a `me()` made for this call, not from context: the whole point is that
/// the copies are derived from the blob the server holds *now*, and `expected_iv` makes the
/// server prove it. A `Stale` refusal is not an error — someone saved in between, and the next
/// projects-page load retries from the newer blob.
pub async fn refill_project_copies(fresh: &Account, account_key: &[u8; 32]) -> Result<(), String> {
    let Some(pair) = fresh.payment_methods.clone() else {
        return Ok(());
    };
    let shared = project_copies(&account_payment_methods(fresh, account_key), &read_from_ls().projects);
    if shared.is_empty() {
        return Ok(());
    }
    let expected_iv = Some(pair.iv.clone());
    match put(UpdatePaymentMethods { account: pair, shared, expected_iv }).await {
        Ok(()) | Err(SaveFailure::Stale) => Ok(()),
        Err(SaveFailure::Other(e)) => Err(e),
    }
}

async fn put(payload: UpdatePaymentMethods) -> Result<(), SaveFailure> {
    update_payment_methods(Json(payload)).await.map_err(|e| {
        if crate::common::is_payment_methods_stale_error(&e) {
            SaveFailure::Stale
        } else {
            SaveFailure::Other(crate::common::error_message(&e))
        }
    })
}

/// The rail a debtor pays through — brand for a known kind, raw id for one this build does not
/// know — or `None` for `other`, whose only name is its label.
pub fn method_kind_name(method: &PaymentMethod) -> Option<String> {
    if method.kind == OTHER_KIND {
        return None;
    }
    Some(
        KNOWN_KINDS
            .iter()
            .find(|(id, _)| *id == method.kind)
            .map(|(_, brand)| *brand)
            .unwrap_or(&method.kind)
            .to_string(),
    )
}

/// What to call a row: the holder's own label, else the brand name for a known kind, else the raw
/// id — which is what a kind written by a newer build looks like here.
pub fn method_display_name(method: &PaymentMethod) -> String {
    if let Some(label) = method.label.as_deref().filter(|l| !l.trim().is_empty()) {
        return label.to_string();
    }
    KNOWN_KINDS
        .iter()
        .find(|(id, _)| *id == method.kind)
        .map(|(_, brand)| *brand)
        .filter(|brand| !brand.is_empty())
        .unwrap_or(&method.kind)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use uuid::Uuid;

    const KEY: [u8; 32] = [7u8; 32];

    fn account(payment_methods: Option<EncryptedPair>) -> Account {
        Account {
            id: Uuid::new_v4(),
            email: "a@b.c".to_string(),
            display_name: EncryptedPair::default(),
            created_at: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            email_verified: true,
            kdf_salt: vec![0; 16],
            kdf_version: 1,
            preferences: None,
            payment_methods,
            public_key: None,
            private_key: None,
        }
    }

    fn method(kind: &str, value: &str, label: Option<&str>) -> PaymentMethod {
        PaymentMethod {
            kind: kind.to_string(),
            value: value.to_string(),
            label: label.map(str::to_string),
            shared: false,
        }
    }

    fn shared(kind: &str, value: &str) -> PaymentMethod {
        PaymentMethod { shared: true, ..method(kind, value, None) }
    }

    fn stored(methods: Vec<PaymentMethod>) -> Account {
        account(Some(encrypt_json(&KEY, &PaymentMethods { methods }).unwrap()))
    }

    const PROJECT_KEY: [u8; 32] = [3u8; 32];

    fn project(key: Option<[u8; 32]>) -> LocalStorageProject {
        LocalStorageProject {
            project_id: Uuid::new_v4(),
            user_id: Some(1),
            encryption_key: key.map(|k| crate::crypto::key_to_fragment(&k)),
            ..Default::default()
        }
    }

    #[test]
    fn a_stored_list_round_trips() {
        let methods = vec![
            method("iban", "FR7630001007941234567890185", Some("Main")),
            method("paypal", "me@example.com", None),
        ];

        assert_eq!(account_payment_methods(&stored(methods.clone()), &KEY), methods);
    }

    #[test]
    fn an_account_that_never_saved_any_is_empty() {
        assert_eq!(account_payment_methods(&account(None), &KEY), Vec::new());
    }

    /// The wrong key is the everyday case, not an attack: a session restored from the cookie has no
    /// account key at all.
    #[test]
    fn an_undecryptable_blob_is_empty_rather_than_a_panic() {
        let acc = stored(vec![method("iban", "FR76", None)]);

        assert_eq!(account_payment_methods(&acc, &[9u8; 32]), Vec::new());
    }

    /// Fields are added over time and old blobs must keep deserialising, or a field added later
    /// would silently wipe the payment details of everyone who had not opened the new build.
    #[test]
    fn an_empty_blob_deserialises() {
        let pair = encrypt_json(&KEY, &serde_json::json!({})).unwrap();

        assert_eq!(account_payment_methods(&account(Some(pair)), &KEY), Vec::new());
    }

    /// The whole reason `kind` is a `String` and not an enum: a kind added by a newer build must
    /// come back untouched instead of failing the blob and losing every other method with it.
    /// `futurepay` stands in for a kind a newer build added and this one has never heard of. Do not
    /// swap it for a real id — adding that id to `KNOWN_KINDS` would quietly stop testing this.
    #[test]
    fn an_unknown_kind_survives_the_round_trip() {
        let acc = stored(vec![method("futurepay", "@jb", None)]);

        assert_eq!(account_payment_methods(&acc, &KEY), vec![method("futurepay", "@jb", None)]);
    }

    #[test]
    fn values_are_trimmed() {
        let cleaned = validate_methods(&[method("iban", "  FR76  ", Some("  Main  "))]).unwrap();

        assert_eq!(cleaned, vec![method("iban", "FR76", Some("Main"))]);
    }

    /// A row the user added and left alone is not an error — it is a row they changed their mind
    /// about, and refusing to save would strand them on a form they cannot submit.
    #[test]
    fn a_blank_row_is_dropped_rather_than_rejected() {
        let cleaned =
            validate_methods(&[method("iban", "FR76", None), method("paypal", "  ", None)])
                .unwrap();

        assert_eq!(cleaned, vec![method("iban", "FR76", None)]);
    }

    /// Removing every row and saving has to actually clear the account. An empty list is a real
    /// value to store here, not "nothing to write" — otherwise the last method could never be
    /// deleted, only edited.
    #[test]
    fn an_emptied_list_saves_and_reads_back_as_empty() {
        let cleared = validate_methods(&[]).expect("an empty list is valid");
        assert!(cleared.is_empty());

        assert_eq!(account_payment_methods(&stored(cleared), &KEY), Vec::new());
    }

    /// The same, coming from a list that had entries: what is stored is the draft, so a removed
    /// row must not survive the round trip.
    #[test]
    fn a_removed_row_does_not_survive_the_round_trip() {
        let mut draft = vec![method("iban", "FR76", None), method("paypal", "me@x.c", None)];
        draft.remove(0);

        let cleaned = validate_methods(&draft).unwrap();

        assert_eq!(account_payment_methods(&stored(cleaned), &KEY), vec![
            method("paypal", "me@x.c", None)
        ]);
    }

    #[test]
    fn a_row_with_a_label_and_no_value_is_rejected() {
        assert_eq!(
            validate_methods(&[method("iban", "", Some("Main"))]),
            Err("payment-method-value-required")
        );
    }

    /// Without a name a custom method renders as the bare id `other`, which tells the holder
    /// nothing about which of their wallets it is.
    #[test]
    fn a_custom_method_without_a_name_is_rejected() {
        assert_eq!(
            validate_methods(&[method("other", "@jb", None)]),
            Err("payment-method-label-required")
        );
    }

    #[test]
    fn an_overlong_value_is_rejected() {
        let long = "x".repeat(MAX_VALUE_LEN + 1);

        assert_eq!(
            validate_methods(&[method("iban", &long, None)]),
            Err("payment-method-too-long")
        );
    }

    #[test]
    fn an_overlong_kind_is_rejected() {
        let long = "x".repeat(MAX_KIND_LEN + 1);

        assert_eq!(
            validate_methods(&[method(&long, "FR76", None)]),
            Err("payment-method-too-long")
        );
    }

    /// A newline pasted into an IBAN is invisible in the field and comes back out broken.
    #[test]
    fn control_characters_are_rejected() {
        assert_eq!(
            validate_methods(&[method("iban", "FR76\u{0}0001", None)]),
            Err("payment-method-invalid-characters")
        );
        assert_eq!(
            validate_methods(&[method("iban", "FR76", Some("Main\nAccount"))]),
            Err("payment-method-invalid-characters")
        );
    }

    /// Trimming happens first, so a trailing newline is whitespace to strip, not a rejection.
    #[test]
    fn surrounding_whitespace_is_not_a_control_character_error() {
        let cleaned = validate_methods(&[method("iban", "FR76\n", None)]).unwrap();

        assert_eq!(cleaned, vec![method("iban", "FR76", None)]);
    }

    /// The limits the client enforces must never produce a blob the server refuses — that would be
    /// an opaque 400 on a form the client had just called valid, with nothing the user could do.
    /// Emoji because the client counts characters and the server counts bytes. Every row shared,
    /// because the flag is the one thing that lengthens the JSON — and the per-project copy is
    /// held to the same cap, so it is checked too.
    #[test]
    fn the_worst_list_the_client_accepts_still_fits_the_server_cap() {
        let wide = "\u{1F600}";
        let worst: Vec<PaymentMethod> = (0..MAX_METHODS)
            .map(|_| PaymentMethod {
                kind: wide.repeat(MAX_KIND_LEN),
                value: wide.repeat(MAX_VALUE_LEN),
                label: Some(wide.repeat(MAX_LABEL_LEN)),
                shared: true,
            })
            .collect();

        let cleaned = validate_methods(&worst).expect("exactly at the limits, not over them");
        let cap = shared::MAX_PAYMENT_METHODS_LENGTH;
        let pair = encrypt_json(&KEY, &PaymentMethods { methods: cleaned.clone() }).unwrap();
        assert!(pair.ct.len() <= cap, "ciphertext {} exceeds the server cap {cap}", pair.ct.len());
        assert!(pair.iv.len() <= cap);

        let copies = project_copies(&cleaned, &[project(Some(PROJECT_KEY))]);
        let copy = &copies[0].payment_methods;
        assert!(copy.ct.len() <= cap, "project copy {} exceeds the server cap {cap}", copy.ct.len());
        assert!(copy.iv.len() <= cap);
    }

    #[test]
    fn validate_method_keeps_the_share_flag() {
        assert!(validate_method(&shared("iban", " FR76 ")).unwrap().shared);
        assert!(!validate_method(&method("iban", "FR76", None)).unwrap().shared);
    }

    /// The flag must cost nothing when off: the ciphertext length is visible to the server, and a
    /// blob that grew on the day the flag shipped would date every account that resaved.
    #[test]
    fn an_unshared_method_serialises_byte_identically_to_before() {
        let json = serde_json::to_string(&method("iban", "FR76", None)).unwrap();

        assert_eq!(json, r#"{"kind":"iban","value":"FR76"}"#);
        assert!(serde_json::to_string(&shared("iban", "FR76")).unwrap().contains(r#""shared":true"#));
    }

    #[test]
    fn a_blob_written_before_the_flag_reads_back_unshared() {
        let pair = encrypt_json(&KEY, &serde_json::json!({"methods":[{"kind":"iban","value":"x"}]}))
            .unwrap();

        assert_eq!(account_payment_methods(&account(Some(pair)), &KEY), vec![method("iban", "x", None)]);
    }

    #[test]
    fn project_copies_hold_only_shared_methods_and_strip_the_flag() {
        let copies = project_copies(
            &[method("iban", "FR76", None), shared("wero", "+33600000000")],
            &[project(Some(PROJECT_KEY))],
        );

        let read: PaymentMethods = decrypt_json(&PROJECT_KEY, &copies[0].payment_methods).unwrap();
        assert_eq!(read.methods, vec![method("wero", "+33600000000", None)]);
    }

    #[test]
    fn project_copies_decrypt_with_the_project_key_not_the_account_key() {
        let copies = project_copies(&[shared("wero", "+33600000000")], &[project(Some(PROJECT_KEY))]);
        let copy = &copies[0].payment_methods;

        assert!(decrypt_json::<PaymentMethods>(&KEY, copy).is_err());
        assert!(decrypt_json::<PaymentMethods>(&PROJECT_KEY, copy).is_ok());
    }

    #[test]
    fn project_copies_skips_projects_without_a_key() {
        let with_key = project(Some(PROJECT_KEY));
        let projects = [project(None), with_key.clone()];

        let copies = project_copies(&[shared("iban", "FR76")], &projects);

        assert_eq!(copies.len(), 1);
        assert_eq!(copies[0].project_id, with_key.project_id);
        let read: PaymentMethods = decrypt_json(&PROJECT_KEY, &copies[0].payment_methods).unwrap();
        assert_eq!(read.methods, vec![method("iban", "FR76", None)]);
    }

    /// A copy on a row with no identity is served to nobody today and could be published under a
    /// participant the account never picked tomorrow (`link_invited_account`). Not produced.
    #[test]
    fn project_copies_skips_projects_without_an_identity() {
        let projects = [LocalStorageProject { user_id: None, ..project(Some(PROJECT_KEY)) }];

        assert!(project_copies(&[shared("iban", "FR76")], &projects).is_empty());
    }

    #[test]
    fn project_copies_is_empty_when_nothing_is_shared() {
        let copies = project_copies(&[method("iban", "FR76", None)], &[project(Some(PROJECT_KEY))]);

        assert!(copies.is_empty());
    }

    #[test]
    fn the_method_cap_is_enforced() {
        let many: Vec<PaymentMethod> =
            (0..=MAX_METHODS).map(|i| method("iban", &format!("FR{i}"), None)).collect();

        assert_eq!(validate_methods(&many), Err("payment-method-limit"));
    }

    /// The table is long enough now that a duplicate is easy to add by hand, and it would render
    /// two options sharing a value. An id over `MAX_KIND_LEN` would be refused by the very
    /// validation meant to accept it.
    #[test]
    fn known_kinds_are_unique_and_within_the_limits() {
        let mut ids: Vec<&str> = KNOWN_KINDS.iter().map(|(id, _)| *id).collect();
        let total = ids.len();
        ids.sort_unstable();
        ids.dedup();

        assert_eq!(ids.len(), total, "duplicate id in KNOWN_KINDS");
        assert!(KNOWN_KINDS.iter().all(|(id, _)| id.chars().count() <= MAX_KIND_LEN));
        assert!(KNOWN_KINDS.iter().any(|(id, _)| *id == DEFAULT_KIND), "the default must exist");
        // Rendered in order, and "Other" reads as the fallback only when it comes last.
        assert_eq!(KNOWN_KINDS.last().map(|(id, _)| *id), Some(OTHER_KIND));
        // Only `other` may have a blank brand — everything else renders its own name.
        assert!(KNOWN_KINDS.iter().all(|(id, brand)| *id == OTHER_KIND || !brand.is_empty()));
    }

    #[test]
    fn method_display_name_falls_back_from_label_to_brand_to_id() {
        assert_eq!(method_display_name(&method("iban", "FR76", Some("Main"))), "Main");
        assert_eq!(method_display_name(&method("paypal", "me@x.c", None)), "PayPal");
        assert_eq!(method_display_name(&method("futurepay", "@jb", None)), "futurepay");
    }

    #[test]
    fn method_kind_name_is_the_rail_and_none_for_other() {
        assert_eq!(method_kind_name(&method("lydia", "06", Some("Céline"))), Some("Lydia".into()));
        assert_eq!(method_kind_name(&method("futurepay", "@jb", None)), Some("futurepay".into()));
        assert_eq!(method_kind_name(&method("other", "@jb", Some("Tontine"))), None);
    }

    /// `other` carries no brand name, so a labelless one must not render as the empty string.
    #[test]
    fn an_other_method_without_a_label_falls_back_to_its_id() {
        assert_eq!(method_display_name(&method("other", "@jb", None)), "other");
    }

    #[test]
    fn one_row_is_trimmed_and_returned() {
        let cleaned = validate_method(&method("  iban  ", "  FR76  ", Some("  Main  "))).unwrap();

        assert_eq!(cleaned, method("iban", "FR76", Some("Main")));
    }

    /// An empty label is stored as `None`, not as `Some("")` — the wire format skips it, and
    /// `method_display_name` falls back on the brand only when it is absent.
    #[test]
    fn an_empty_label_becomes_none() {
        assert_eq!(validate_method(&method("iban", "FR76", Some("   "))).unwrap().label, None);
    }

    #[test]
    fn one_row_reports_each_problem_by_key() {
        assert_eq!(
            validate_method(&method("iban", "  ", Some("Main"))),
            Err("payment-method-value-required")
        );
        assert_eq!(
            validate_method(&method("other", "@jb", None)),
            Err("payment-method-label-required")
        );
        assert_eq!(
            validate_method(&method("iban", &"x".repeat(MAX_VALUE_LEN + 1), None)),
            Err("payment-method-too-long")
        );
        assert_eq!(
            validate_method(&method("iban", "FR76\u{0}", None)),
            Err("payment-method-invalid-characters")
        );
    }

    /// Each row saves on its own, so the row being saved must be judged alone — a neighbour the
    /// user has not finished typing cannot be allowed to block it.
    #[test]
    fn a_row_is_validated_without_its_neighbours() {
        let rows = [method("iban", "", None), method("paypal", "me@x.c", None)];

        assert!(validate_method(&rows[1]).is_ok());
    }

    #[test]
    fn a_row_is_blank_only_until_something_is_typed() {
        assert!(is_blank(&method("iban", "  ", None)));
        assert!(is_blank(&method("iban", "", Some("  "))));
        assert!(!is_blank(&method("iban", "FR76", None)));
        assert!(!is_blank(&method("iban", "", Some("Main"))));
    }
}
