use crate::i18n::*;
use leptos::prelude::*;
use crate::components::ui::button::*;

#[component]
pub fn LangToggle() -> impl IntoView {
    let i18n = use_i18n();

    let on_click = move |_| {
        let new_lang = match i18n.get_locale() {
            Locale::cs => Locale::en,
            Locale::en => Locale::cs,
        };
        i18n.set_locale(new_lang);
    };

    view! {
        <Button 
            on:click=on_click
            kind=ButtonKind::Solid(Color::Cyan)
        >
            {move || match i18n.get_locale() {
                Locale::cs => "English 🇬🇧",
                Locale::en => "Čeština 🇨🇿",
            }}
        </Button>
    }
}