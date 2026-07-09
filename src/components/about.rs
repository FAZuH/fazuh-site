use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        section {
            id: "about",
            class: "py-16 md:py-20",
            div {
                class: "max-w-[960px] mx-auto px-6",
                h2 {
                    class: "text-base font-bold text-ink mb-6",
                    "[+] about"
                }
                div {
                    class: "border-t border-hairline",
                    div {
                        class: "py-8 md:py-10 flex flex-col gap-8",
                        p {
                            class: "text-base text-body leading-relaxed max-w-[65ch]",
                            "Infrastructure engineer, software developer, and Statistics student in the Mathematics department at University of Indonesia. Building things for the terminal, Discord, and the homelab \u{2014} sometimes all three."
                        }
                        ul {
                            class: "flex flex-col gap-3 text-base",
                            BulletItem {
                                label: "infrastructure engineer",
                                desc: "4 servers, ~20 Docker stacks, homelab automation with Ansible, Terraform, Consul, and custom tooling.",
                            }
                            BulletItem {
                                label: "software developer",
                                desc: "Rust, Python. Writing Discord bots, TUI tools, CLIs, and scripts.",
                            }
                            BulletItem {
                                label: "student",
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
                    }
                }
            }
        }
    }
}

#[component]
fn BulletItem(label: &'static str, desc: &'static str) -> Element {
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
