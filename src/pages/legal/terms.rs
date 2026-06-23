use leptos::prelude::*;

use crate::i18n::*;
use crate::components::layout::page_container::PageContainer;
use crate::components::markdown::co_markdown::COMarkdown;

#[component]
pub fn Terms() -> impl IntoView {
    let i18n = use_i18n();

    let markdown = move || {
        match i18n.get_locale().as_str() {
            "cs" => include_str!("../../../locales/cs/legal/terms.md"),
            _ => include_str!("../../../locales/en/legal/terms.md"),
        }
    };

    view! {
        <PageContainer>
            {move || {
                view! {
                    <COMarkdown markdown=markdown() />
                }
            }}
        </PageContainer>
    }
}