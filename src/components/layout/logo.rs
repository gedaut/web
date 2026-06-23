use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Logo() -> impl IntoView {
    view! {
        <A href="/" attr:class="flex items-center gap-2">
            <img
                src="/favicon.svg"
                class="h-8 w-8 transition-transform hover:scale-105 duration-200"
            />
            <span class="font-black tracking-wide text-[var(--color-primary)] text-xl">
                "GEDAUT"
            </span>
        </A>
    }
}
