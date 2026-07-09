use dioxus::prelude::*;

use crate::components::Collapsible;

#[derive(PartialEq)]
struct Project {
    name: &'static str,
    desc: &'static str,
    link: &'static str,
}

const NOTABLE: &[Project] = &[
    Project {
        name: "pwr-bot",
        desc: "Discord bot \u{2014} anime & manga tracking, voice call stats",
        link: "https://github.com/FAZuH/pwr-bot",
    },
    Project {
        name: "tomo",
        desc: "TUI-based Pomodoro timer",
        link: "https://github.com/FAZuH/tomo",
    },
    Project {
        name: "lab-ops",
        desc: "Utility tools for my homelab",
        link: "https://github.com/FAZuH/lab-ops",
    },
    Project {
        name: "notes.fazuh.com",
        desc: "My Obsidian notes built with Quartz",
        link: "https://notes.fazuh.com",
    },
    Project {
        name: "mail.fazuh.com",
        desc: "Mail server for @fazuh.com emails (Mailcow)",
        link: "https://mail.fazuh.com",
    },
    Project {
        name: "warlock",
        desc: "UI course registration & schedule tracking bot (SIAKNG)",
        link: "https://github.com/FAZuH/warlock",
    },
    Project {
        name: "arthaenergi.id",
        desc: "Landing page (Dioxus) and mail server (Mailcow) for PT. Mandala Artha Energi",
        link: "https://arthaenergi.id",
    },
    Project {
        name: "krosanevasi.com",
        desc: "Landing page (Dioxus) and mail server (Mailcow) for PT. Kros Anevasi ",
        link: "https://krosanevasi.com",
    },
];

const SMALLER: &[Project] = &[
    Project {
        name: "design-patterns-rust",
        desc: "Learn software design patterns in Rust",
        link: "https://github.com/FAZuH/design-patterns-rust",
    },
    Project {
        name: "contribution-grid",
        desc: "Rust crate \u{2014} GitHub-style contribution heatmap generation",
        link: "https://crates.io/crates/contribution-grid",
    },
    Project {
        name: "elm-architecture",
        desc: "PoC of The Elm Architecture across different UI backends in Rust",
        link: "https://github.com/FAZuH/elm-architecture",
    },
    Project {
        name: "symlist",
        desc: "Simple CLI to manage and sync a list of symbolic links",
        link: "https://github.com/FAZuH/symlist",
    },
    Project {
        name: "hyper-clear",
        desc: "CLI tool to control window transparency & blur on Hyprland",
        link: "https://github.com/FAZuH/hyper-clear",
    },
    Project {
        name: "ratatui-toaster",
        desc: "Fork of ratatui-toaster \u{2014} added toast deduplication and stacking",
        link: "https://crates.io/crates/ratatui-toaster",
    },
    Project {
        name: "vigilance",
        desc: "Watch and notify usage of disk, memory, battery, wifi",
        link: "https://github.com/FAZuH/vigilance",
    },
];

const COURSE_WORK: &[Project] = &[Project {
    name: "Perbankan",
    desc: "Banking app",
    link: "https://github.com/FAZuH/Perbankan",
}];

#[component]
pub fn Projects() -> Element {
    rsx! {
        section {
            id: "projects",
            class: "py-16 md:py-20",
            div {
                class: "max-w-[960px] mx-auto px-6",
                h2 {
                    class: "text-xl font-bold text-ink mb-2",
                    "Projects"
                }
                p {
                    class: "text-sm text-mute mb-8 max-w-[65ch]",
                    "Selected projects."
                }

                div {
                    class: "border-t border-hairline pt-6 flex flex-col",
                    Collapsible {
                        label: "notable/",
                        default_open: true,
                        always_open: true,
                        children: rsx! {
                            div { class: "pl-3 mb-4",
                                {NOTABLE.iter().enumerate().map(|(i, p)| {
                                    let is_last = i == NOTABLE.len() - 1;
                                    rsx! { ProjectRow { project: p, is_last, indent: true } }
                                })}
                            }
                        }
                    }
                    Collapsible {
                        label: "smaller-projects/",
                        default_open: false,
                        children: rsx! {
                            div { class: "pl-3 mb-4",
                                {SMALLER.iter().enumerate().map(|(i, p)| {
                                    let is_last = i == SMALLER.len() - 1;
                                    rsx! { ProjectRow { project: p, is_last, indent: true } }
                                })}
                            }
                        }
                    }
                    Collapsible {
                        label: "course-work/",
                        default_open: false,
                        children: rsx! {
                            div { class: "pl-3",
                                {COURSE_WORK.iter().enumerate().map(|(i, p)| {
                                    let is_last = i == COURSE_WORK.len() - 1;
                                    rsx! { ProjectRow { project: p, is_last, indent: true } }
                                })}
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ProjectRow(project: &'static Project, is_last: bool, indent: bool) -> Element {
    let branch = if is_last { "└──" } else { "├──" };

    rsx! {
        a {
            href: project.link,
            target: if project.link.starts_with("http") { "_blank" } else { "_self" },
            rel: if project.link.starts_with("http") { "noopener noreferrer" } else { "" },
            class: "flex items-baseline gap-3 py-1.5 hover:bg-surface-soft transition-colors rounded-sm \
                group cursor-pointer",
            if indent {
                span { class: "text-mute ml-3", "{branch}" }
            } else {
                span { class: "text-mute ml-2", "{branch}" }
            }
            span {
                class: "text-ink font-medium min-w-[8rem] group-hover:text-accent transition-colors",
                "{project.name}"
            }
            span {
                class: "text-mute",
                "{project.desc}"
            }
        }
    }
}
