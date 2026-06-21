use leptos::prelude::*;
use crate::components::home::hero::Hero;
use crate::components::home::info::Info;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="space-y-12 pb-16">
            <Hero />
            <Info />
        </div>
    }
}