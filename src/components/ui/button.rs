use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Cyan,
    Orange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonKind {
    Solid(Color),
    Transparent(Color),
    Danger,
}

fn color_solid(color: Color) -> &'static str {
    match color {
        Color::Cyan =>
            "bg-[var(--color-primary)] hover:bg-[var(--color-primary-hover)] text-black font-bold",

        Color::Orange =>
            "bg-[var(--color-accent)] hover:bg-[var(--color-accent-hover)] text-white font-bold",
    }
}

fn color_transparent(color: Color) -> &'static str {
    match color {
        Color::Cyan =>
            "text-[var(--color-primary)] after:bg-[var(--color-primary)]",

        Color::Orange =>
            "text-[var(--color-accent)] after:bg-[var(--color-accent)]",
    }
}

#[component]
pub fn Button(
    children: Children,
    kind: ButtonKind,
    #[prop(optional)] on_click: Option<Box<dyn Fn()>>,
) -> impl IntoView {

    let base_solid =
        "px-6 py-2.5 rounded-full text-sm tracking-wide transition active:scale-95 shadow-md";

    let base_transparent =
        "px-4 py-2 text-sm tracking-wide transition active:scale-95 rounded-md relative overflow-hidden
         after:absolute after:bottom-0 after:left-0 after:h-0.5
         after:w-0 hover:after:w-full after:transition-all after:duration-300";

    let (base, variant) = match kind {
        ButtonKind::Solid(color) => (
            base_solid,
            color_solid(color),
        ),

        ButtonKind::Transparent(color) => (
            base_transparent,
            color_transparent(color),
        ),

        ButtonKind::Danger => (
            base_solid,
            "bg-red-600 hover:bg-red-700 text-white font-bold",
        ),
    };

    let class = format!("{base} {variant}");

    view! {
        <button
            class=class
            on:click=move |_| {
                if let Some(f) = &on_click {
                    f();
                }
            }
        >
            {children()}
        </button>
    }
}