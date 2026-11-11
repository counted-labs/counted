use dioxus::prelude::*;

use crate::common::APP_BASE_URL;
use crate::i18n::current_lang;
use crate::tid;

/// What a messaging app shows for a pasted project link. Identical for every project: the key is
/// in the fragment, which a link preview never sends, and the path it fetches is the one it
/// already fetched without these tags.
#[component]
pub fn SharePreview() -> Element {
    let image = if current_lang() == "fr" { "fr" } else { "en" };
    rsx! {
        document::Meta { property: "og:type", content: "website" }
        document::Meta { property: "og:site_name", content: "Counted" }
        document::Meta { property: "og:title", content: tid!("share-preview-title") }
        document::Meta { property: "og:description", content: tid!("share-preview-description") }
        document::Meta { property: "og:image", content: format!("{APP_BASE_URL}/static/og/{image}.jpg") }
        document::Meta { property: "og:image:width", content: "1200" }
        document::Meta { property: "og:image:height", content: "630" }
        document::Meta { name: "twitter:card", content: "summary_large_image" }
    }
}
