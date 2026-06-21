use leptos::prelude::*;

#[component]
pub fn Card(
    icon: &'static str,
    title: impl IntoView,
    desc: impl IntoView,
) -> impl IntoView {
    view! {
        <div class="
            flex flex-col items-center
            p-6
            bg-[var(--color-card)]
            border border-[var(--color-border)]
            rounded-2xl
            shadow-lg
        ">
            <div class="text-4xl mb-4">{icon}</div>

            <h3 class="
                text-lg font-bold uppercase
                text-[var(--color-primary)]
            ">
                {title}
            </h3>

            <p class="
                text-sm mt-2
                text-[var(--color-text-muted)]
            ">
                {desc}
            </p>
        </div>
    }
}