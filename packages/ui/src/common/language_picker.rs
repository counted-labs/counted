use dioxus::prelude::*;
use crate::tid;
use shared::Account;

use crate::i18n::{current_lang, set_language, SUPPORTED};
use crate::preferences::push_language;

/// Language selector — a bare labelled `<select>`, with no card or menu chrome of its own, so the
/// settings page can put it inside a `fieldset` without fighting a second container.
///
/// A plain `<select>`, not a `DropdownButton`: the list is long enough that the OS picker is the
/// better control on a phone, and it sidesteps the iOS `:focus-within` dropdown problem entirely.
/// Each option is labelled in its own language — someone looking for German scans for "Deutsch".
/// The daisyUI class goes on the wrapping `<label>`, which is what joins the caption to the control.
#[component]
pub fn LanguagePicker() -> Element {
    let active = current_lang();
    let auth_ctx = use_context::<Signal<Option<Account>>>();
    let account_enc_key_ctx = use_context::<Signal<Option<[u8; 32]>>>();

    let on_change = move |e: FormEvent| {
        let code = e.value();
        set_language(&code);
        // Only a deliberate pick syncs, and only with a key to encrypt it — a session restored from
        // the cookie has none, and the choice then stays on this device like an anonymous one.
        if auth_ctx().is_some() {
            if let Some(key) = account_enc_key_ctx() {
                push_language(key, code);
            }
        }
    };

    rsx! {
        label { class: "select w-full",
            span { class: "label", {tid!("language")} }
            select {
                id: "language-picker",
                value: "{active}",
                onchange: on_change,
                for (code , endonym) in SUPPORTED {
                    option { value: "{code}", selected: *code == active, "{endonym}" }
                }
            }
        }
    }
}
