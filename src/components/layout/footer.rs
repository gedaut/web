use leptos::prelude::*;
use crate::components::layout::logo::Logo;
use crate::i18n::*;

#[component]
pub fn Footer() -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <footer class="
            mt-auto
            bg-[var(--color-navbar)]
            border-t border-[var(--color-border)]
            text-[var(--color-text)]
        ">
            <div class="max-w-6xl mx-auto px-4 py-8">
                
                // ================= TOP: LOGO & SOCIAL NETWORKS =================
                <div class="flex flex-col sm:flex-row items-center justify-between gap-4 pb-8">
                    
                    // Logo
                    <Logo />

                    // Ikony sociálních sítí
                    <div class="flex items-center gap-5 text-lg opacity-60">
                        <a href="https://www.youtube.com/@gedaut" target="_blank" rel="noopener noreferrer" class="hover:text-[var(--color-primary)] transition-colors">"YouTube"</a>
                    </div>
                </div>

                // ================= BOTTOM: LEGISLATION & COPYRIGHT =================
                <div class="pt-6 border-t border-[var(--color-border)] flex flex-col md:flex-row items-center justify-between gap-4 text-xs opacity-60">
                    
                    // Legal links
                    <div class="flex flex-wrap justify-center gap-x-6 gap-y-2">
                        <a href="#" class="hover:underline">
                            {t!(i18n, layout.footer.privacy_policy)}
                        </a>
                        <a href="#" class="hover:underline">
                            {t!(i18n, layout.footer.terms_of_service)}
                        </a>
                        <a href="#" class="hover:underline">
                            {t!(i18n, layout.footer.cookies)}
                        </a>
                    </div>

                    // Copyright
                    <div class="text-center md:text-right">
                        "Copyright © GEDAUT. " {t!(i18n, layout.footer.rights_reserved)}
                    </div>
                </div>

            </div>
        </footer>
    }
}