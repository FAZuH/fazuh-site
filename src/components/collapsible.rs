use dioxus::prelude::*;

#[component]
pub fn Collapsible(
    label: &'static str,
    default_open: bool,
    always_open: Option<bool>,
    children: Element,
) -> Element {
    let mut open = use_signal(|| default_open);
    let locked = always_open.unwrap_or(false);

    rsx! {
        if locked {
            div {
                class: "text-base font-bold text-ink flex items-center gap-2 py-3 px-2",
                "[-]"
                span { "{label}" }
            }
        } else {
            button {
                class: "text-base font-bold text-ink flex items-center gap-2 py-3 w-full text-left hover:bg-surface-soft transition-colors rounded-sm px-2",
                onclick: move |_| open.set(!open()),
                if open() { "[-]" } else { "[+]" }
                span { "{label}" }
            }
        }
        if open() {
            {children}
        }
    }
}
