//! The `/demo` sandbox: a throwaway project per visitor, in a tab whose stores live in
//! sessionStorage (`common::persist::is_demo`). See DOCUMENTATION.md, "Demo projects".

pub mod dataset;
mod demo_page;
mod seed;

pub use demo_page::{DemoBar, DemoPage, DemoSignInBlocked};

use dioxus::prelude::*;

/// Post-mount, like every store-derived flag: SSR cannot see sessionStorage.
pub fn use_demo_mode() -> bool {
    let mut demo = use_signal(|| false);
    use_effect(move || demo.set(crate::common::persist::is_demo()));
    demo()
}

/// Back to the real app, with nothing of the demo left behind.
pub fn leave_demo() {
    #[cfg(target_arch = "wasm32")]
    {
        crate::common::persist::clear_demo();
        crate::common::web_dom::assign_location("/projects");
    }
}
