use crate::i18n::*;
use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <section class="
            relative
            w-full
            overflow-hidden

            shadow-xl
            
        ">
            <div class="
                h-96
                flex items-center justify-center
                relative

                bg-gradient-to-b
                from-[var(--color-surface-950)]
                via-[var(--color-surface-900)]
                to-[var(--color-surface-900)]
            ">

                <div class="
                    absolute inset-0
                    opacity-40
                    bg-[url('/home/bg.webp')]
                    bg-cover bg-center
                "></div>

                <div class="
                    absolute inset-0
                    flex flex-col
                    items-center justify-center
                    text-center
                    px-6

                    bg-gradient-to-t
                    from-[var(--color-surface-950)]
                    via-transparent
                    to-transparent

                    [text-shadow:0_4px_12px_rgba(0,0,0,0.8)]
                ">
                    <h1 class="
                        text-3xl md:text-5xl
                        font-black
                        uppercase
                        max-w-3xl
                        leading-tight
                    ">
                        {t!(i18n, home.hero.title_1)}
                        <span class="text-[var(--color-primary)]">
                            {t!(i18n, home.hero.title_2)}
                        </span>
                    </h1>

                    <p class="
                        text-[var(--color-accent)]
                        text-lg md:text-xl
                        font-bold
                        mt-3
                    ">
                        {t!(i18n, home.hero.welcome)}
                    </p>

                    <p class="
                        text-[var(--color-text-muted)]
                        text-sm md:text-base
                        mt-2
                    ">
                        {t!(i18n, home.hero.subtitle)}
                    </p>
                </div>
            </div>
        </section>
    }
}