use leptos::prelude::*;

use crate::i18n::*;
use crate::components::layout::page_container::PageContainer;
use crate::components::markdown::cg_markdown::CGMarkdown;

#[component]
pub fn Privacy() -> impl IntoView {
    let i18n = use_i18n();

    let markdown = move || {
        match i18n.get_locale().as_str() {
            "cs" => include_str!("../../../locales/cs/legal/privacy.md"),
            _ => include_str!("../../../locales/en/legal/privacy.md"),
        }
    };

    view! {
        <PageContainer>
            {move || {
                view! {
                    <CGMarkdown markdown=markdown() />
                }
            }}
        </PageContainer>
    }
}