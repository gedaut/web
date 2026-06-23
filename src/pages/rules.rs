use leptos::prelude::*;

use crate::i18n::*;
use crate::components::layout::page_container::PageContainer;
use crate::components::markdown::co_markdown::COMarkdown;

#[component]
pub fn Rules() -> impl IntoView {
    let i18n = use_i18n();

    let markdown = move || {
        match i18n.get_locale().as_str() {
            "cs" => include_str!("../../locales/cs/rules.md"),
            _ => include_str!("../../locales/en/rules.md"),
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