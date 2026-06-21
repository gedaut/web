use leptos::prelude::*;

#[component]
pub fn PageContainer(children: Children) -> impl IntoView {
    view! {
        <div class="
            min-h-screen
            bg-[var(--color-bg)]
            text-[var(--color-text)]
            py-12 px-4 sm:px-6 lg:px-8
        ">
            <div class="
                max-w-4xl
                mx-auto
            ">
                {children()}
            </div>
        </div>
    }
}