use dioxus::prelude::*;

use crate::server_stats::CONTAINER_COUNT;
use crate::server_stats::SERVER_COUNT;

#[component]
pub fn About() -> Element {
    rsx! {
        section {
            id: "about",
            class: "py-16 md:py-20",
            div {
                class: "max-w-[960px] mx-auto px-6",
                h2 {
                    class: "text-xl font-bold text-ink mb-6",
                    "About"
                }
                div {
                    class: "border-t border-hairline",
                    div {
                        class: "py-8 md:py-10 flex flex-col gap-8",
                        ul {
                            class: "flex flex-col gap-3 text-base",
                            BulletItem {
                                label: "Cloud Infrastructure Engineer",
                                desc: format!("Managing {SERVER_COUNT} servers and ~{CONTAINER_COUNT} Docker containers with Ansible, Terraform, Consul, and custom tooling."),
                            }
                            BulletItem {
                                label: "Software Developer",
                                desc: "Rust, Python. Writing Discord bots, TUI tools, CLIs, and scripts.",
                            }
                            BulletItem {
                                label: "Student",
                                desc: "Statistics student in the Mathematics department at University of Indonesia.",
                            }
                        }
                    }
                    div {
                        class: "border-t border-hairline pt-6 flex flex-col md:flex-row gap-3 md:gap-12 text-sm",
                        NavLink {
                            label: "GitHub",
                            desc: "github.com/FAZuH \u{2014} 38 public repos",
                            href: "https://github.com/FAZuH",
                        }
                        NavLink {
                            label: "mail",
                            desc: "mail@fazuh.com",
                            href: "mailto:mail@fazuh.com",
                        }
                        NavLink {
                            label: "Discord",
                            desc: ".fazuh",
                            href: "https://discord.com/channels/@me",
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn BulletItem(label: String, desc: String) -> Element {
    rsx! {
        li {
            class: "gap-2",
            div {
                class: "flex gap-3",
                span {
                    class: "text-mute shrink-0 mr-2",
                    "[+]"
                }
                div {
                    span {
                        class: "font-bold text-ink",
                        "{label}"
                    }
                    span { class: "text-body", " \u{2014} {desc}" }
                }
            }
        }
    }
}

#[component]
fn NavLink(label: &'static str, desc: &'static str, href: &'static str) -> Element {
    rsx! {
        a {
            href,
            target: if href.starts_with("http") { "_blank" } else { "_self" },
            rel: if href.starts_with("http") { "noopener noreferrer" } else { "" },
            class: "flex flex-col gap-1 hover:text-ink transition-colors",
            span {
                class: "text-mute",
                "[{label}]"
            }
            span {
                class: "text-ink",
                "{desc}"
            }
        }
    }
}
