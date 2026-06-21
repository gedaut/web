use crate::i18n::*;
use leptos::prelude::*;
use crate::components::ui::card::Card;

#[component]
pub fn Info() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <section class="
            w-full
            max-w-5xl
            mx-auto
            mt-16
            px-4
            text-center
        ">
            <h2 class="
                text-2xl
                font-black
                uppercase
                mb-10
                text-[var(--color-text)]
                tracking-wide
            ">
                {t!(i18n, home.info.title)}
            </h2>

            <div class="
                grid grid-cols-1 md:grid-cols-3
                gap-8
            ">
                <Card
                    icon="🏆"
                    title={t!(i18n, home.info.game_goal_title)}
                    desc={t!(i18n, home.info.game_goal_desc)}
                />

                <Card
                    icon="🏐"
                    title={t!(i18n, home.info.equipment_title)}
                    desc={t!(i18n, home.info.equipment_desc)}
                />

                <Card
                    icon="⏱️"
                    title={t!(i18n, home.info.fouls_title)}
                    desc={t!(i18n, home.info.fouls_desc)}
                />
            </div>
        </section>
    }
}