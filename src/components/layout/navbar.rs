use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::layout::lang_toggler::LangToggle;
use crate::components::ui::button::{Button, ButtonKind, Color};
use crate::i18n::*;

#[component]
pub fn Navbar() -> impl IntoView {
    let i18n = use_i18n();
    let open = RwSignal::new(false);

    let toggle = move |_| open.update(|v| *v = !*v);
    let close = move |_| open.set(false);

    view! {
        <nav class="
            sticky top-0 z-50
            backdrop-blur-md
            bg-[var(--color-navbar)]
            border-b border-[var(--color-border)]
        ">

            // ================= TOP BAR =================
            <div class="max-w-6xl mx-auto flex items-center justify-between px-4 py-3">

                // LOGO
                <A href="/" attr:class="flex items-center gap-2">
                    <img
                        src="/favicon.svg"
                        class="h-9 w-9 transition-transform hover:scale-105 duration-200"
                    />
                    <span class="font-black tracking-wide text-[var(--color-primary)]">
                        GEDAUT
                    </span>
                </A>

                // ================= DESKTOP =================
                <div class="hidden md:flex items-center gap-2">

                    <A href="/">
                        <Button kind=ButtonKind::Transparent(Color::Cyan)>
                            {t!(i18n, layout.nav.home)}
                        </Button>
                    </A>

                    <A href="/rules">
                        <Button kind=ButtonKind::Transparent(Color::Cyan)>
                            {t!(i18n, layout.nav.rules)}
                        </Button>
                    </A>

                    <div class="ml-2">
                        <LangToggle/>
                    </div>
                </div>

                // ================= HAMBURGER =================
                <button
                    class="
                        md:hidden
                        text-[var(--color-text)]
                        text-2xl
                        transition-transform duration-200
                        active:scale-90
                    "
                    on:click=toggle
                >
                    {move || if open.get() { "✕" } else { "☰" }}
                </button>

            </div>

            // ================= MOBILE MENU =================
            <div
                class="
                    md:hidden
                    overflow-hidden
                    border-t border-[var(--color-border)]
                    bg-[var(--color-navbar)]
                    transition-all duration-300
                "
                class:max-h-96=move || open.get()
                class:opacity-100=move || open.get()
                class:max-h-0=move || !open.get()
                class:opacity-0=move || !open.get()
                class:pointer-events-none=move || !open.get()
            >

                <div class="flex flex-col gap-2 px-4 py-3">

                    <A href="/" on:click=close>
                        <Button kind=ButtonKind::Transparent(Color::Cyan)>
                            {t!(i18n, layout.nav.home)}
                        </Button>
                    </A>

                    <A href="/rules" on:click=close>
                        <Button kind=ButtonKind::Transparent(Color::Cyan)>
                            {t!(i18n, layout.nav.rules)}
                        </Button>
                    </A>

                    <div class="pt-2">
                        <LangToggle/>
                    </div>

                </div>
            </div>
        </nav>
    }
}