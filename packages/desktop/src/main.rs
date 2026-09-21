use api::auth::auth_controller::me;
use dioxus::prelude::*;
use shared::Account;
use ui::common::{is_mobile, read_from_ls, LocalStorageState};
use ui::route::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(app);
}

#[component]
fn app() -> Element {
    let auth: Signal<Option<Account>> = use_context_provider(|| Signal::new(None));
    let _account_enc_key: Signal<Option<[u8; 32]>> = use_context_provider(|| Signal::new(None));
    let _flash: Signal<Option<String>> = use_context_provider(|| Signal::new(None));
    let _ls: Signal<LocalStorageState> = use_context_provider(|| Signal::new(read_from_ls()));
    let _is_mobile: Signal<bool> = use_context_provider(|| Signal::new(is_mobile()));

    use_effect(move || {
        let mut auth = auth;
        spawn(async move {
            if let Ok(account) = me().await {
                auth.set(account);
            }
        });
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        main {
            "data-theme": "counted",
            class: "min-h-screen flex flex-col items-center",
            Router::<Route> {}
        }
    }
}
