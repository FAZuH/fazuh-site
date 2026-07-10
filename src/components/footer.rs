use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "border-t border-hairline mt-24",
            div {
                class: "max-w-[960px] mx-auto px-6 py-8",
                div {
                    class: "flex flex-col md:flex-row items-center justify-between gap-3 sm:gap-4 text-xs text-mute",
                    div {
                        class: "flex items-center gap-4",
                        a {
                            href: "https://github.com/FAZuH",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "hover:text-ink transition-colors",
                            "github.com/FAZuH"
                        }
                        span { class: "hidden md:inline text-ash", "·" }
                        a {
                            href: "mailto:mail@fazuh.com",
                            class: "hover:text-ink transition-colors",
                            "mail@fazuh.com"
                        }
                    }
                    div {
                        "\u{00A9} 2026 FAZuH"
                    }
                }
            }
        }
    }
}
