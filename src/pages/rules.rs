use leptos::prelude::*;
use pulldown_cmark::{Parser, Options, html};

use crate::i18n::*;
use crate::components::layout::page_container::PageContainer;
use crate::components::markdown::rules_markdown::RulesMarkdown;

#[component]
pub fn Rules() -> impl IntoView {
    let i18n = use_i18n();

    // Reaktivní zdroj markdown textu
    let markdown = move || {
        match i18n.get_locale().as_str() {
            "cs" => include_str!("../../locales/cs/rules.md"),
            _ => include_str!("../../locales/en/rules.md"),
        }
    };

    let html_output = Memo::new(move |_| {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);

        let parser = Parser::new_ext(markdown(), options);

        let mut html_string = String::new();
        html::push_html(&mut html_string, parser);

        html_string
    });

    view! {
        <PageContainer>
            {move || {
                view! {
                    <RulesMarkdown html=html_output.get() />
                }
            }}
        </PageContainer>
    }
}