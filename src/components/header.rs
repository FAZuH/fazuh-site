use dioxus::prelude::*;

const NAV_LINKS: &[(&str, &str)] = &[
    ("about", "#about"),
    ("skills", "#skills"),
    ("projects", "#projects"),
    ("contact", "#contact"),
];

#[component]
pub fn Header() -> Element {
    let mut is_menu_open = use_signal(|| false);

    rsx! {
        header {
            class: "fixed top-0 left-0 right-0 z-50 bg-canvas/95 backdrop-blur-sm border-b border-hairline",
            div {
                class: "max-w-[960px] mx-auto px-6",
                div {
                    class: "flex justify-between items-center h-14",
                    a {
                        href: "#top",
                        class: "text-ink text-sm font-bold tracking-tight",
                        "FAZuH"
                    }

                    nav {
                        class: if is_menu_open() {
                            "flex flex-col absolute top-full left-0 right-0 bg-canvas border-t border-hairline md:static md:flex-row md:border-0 md:gap-1"
                        } else {
                            "hidden md:flex md:flex-row"
                        },
                        ul {
                            class: "flex flex-col md:flex-row md:items-center md:gap-1 p-4 md:p-0",
                            {NAV_LINKS.iter().map(|(label, href)| {
                                rsx! {
                                    li {
                                        a {
                                            href: *href,
                                            class: "block py-2 md:py-0 md:px-3 text-sm text-body hover:text-ink transition-colors",
                                            onclick: move |_| is_menu_open.set(false),
                                            "[+] {label}"
                                        }
                                    }
                                }
                            })}
                            li {
                                a {
                                    href: "https://github.com/FAZuH",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    class: "block py-2 md:py-0 md:px-3 text-sm text-body hover:text-ink transition-colors group relative",
                                    title: "github.com/FAZuH — 38 public repos",
                                    GitHubIcon {}
                                    span {
                                        class: "hidden md:block absolute top-full left-1/2 -translate-x-1/2 mt-1 px-3 py-2 bg-surface-dark text-on-dark text-xs whitespace-nowrap rounded-sm opacity-0 group-hover:opacity-100 pointer-events-none transition-opacity z-50",
                                        "github.com/FAZuH
{'\u{00A0}'}
38 public repos"
                                    }
                                }
                            }
                        }
                    }

                    button {
                        class: "md:hidden p-2 text-ink",
                        onclick: move |_| is_menu_open.set(!is_menu_open()),
                        aria_label: "Toggle menu",
                        if is_menu_open() {
                            "[-]"
                        } else {
                            "[+]"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn GitHubIcon() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "20",
            height: "20",
            view_box: "0 0 24 24",
            fill: "currentColor",
            path {
                d: "M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.05-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23. 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0 0 24 12c0-6.63-5.37-12-12-12z"
            }
        }
    }
}
