use leptos::prelude::*;
use pulldown_cmark::{Parser, Options, html};

#[component]
pub fn CGMarkdown(#[prop(into)] markdown: &'static str) -> impl IntoView {
    let html_output = Memo::new(move |_| {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);

        let parser = Parser::new_ext(markdown, options);
        let mut html_string = String::new();
        html::push_html(&mut html_string, parser);
        html_string
    });

    view! {
        <div class="
            bg-[var(--color-card)]
            border
            border-[var(--color-border)]
            rounded-2xl
            shadow-xl
            p-6 sm:p-10
        ">
            <article class="
                text-[var(--color-text-secondary)]
                [&_p]:leading-relaxed
                [&_p]:mb-4
                [&_h1]:text-[var(--color-accent)]
                [&_h1]:text-4xl
                [&_h1]:font-extrabold
                [&_h1]:text-center
                [&_h1]:mb-10
                [&_h2]:text-[var(--color-primary)]
                [&_h2]:text-2xl
                [&_h2]:font-bold
                [&_h2]:mt-10
                [&_h2]:mb-4
                [&_h2]:pb-2
                [&_h2]:border-b
                [&_h2]:border-[var(--color-border)]
                [&_h3]:text-[var(--color-text)]
                [&_h3]:text-lg
                [&_h3]:font-semibold
                [&_h3]:mt-6
                [&_h3]:mb-2
                [&_ul]:list-disc
                [&_ul]:pl-6
                [&_ul]:my-4
                [&_li]:my-1
                [&_hr]:my-8
                [&_hr]:border-[var(--color-border)]
                [&_a]:text-[var(--color-primary)]
                [&_a:hover]:text-[var(--color-primary-hover)]
                [&_a]:underline
            "
            inner_html=html_output.get()
            />
        </div>
    }
}