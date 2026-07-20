use std::sync::LazyLock;

use dioxus::prelude::*;

use crate::components::Collapsible;

#[derive(PartialEq)]
struct Project {
    name: String,
    desc: String,
    link: String,
}

macro_rules! proj {
    ($name:expr, $desc:expr, $link:expr) => {
        Project {
            name: $name.into(),
            desc: $desc.into(),
            link: $link.into(),
        }
    };
}

#[rustfmt::skip]
static NOTABLE: LazyLock<Vec<Project>> = LazyLock::new(|| vec![
    proj!("pwr-bot", "Discord bot \u{2014} anime & manga tracking, voice call stats", "https://github.com/FAZuH/pwr-bot"),
    proj!("notes.fazuh.com", "My Obsidian notes built with Quartz", "https://notes.fazuh.com"),
    proj!("mail.fazuh.com", "Mail server for @fazuh.com emails", "https://mail.fazuh.com"),
    proj!("tomo", "TUI-based Pomodoro timer", "https://github.com/FAZuH/tomo"),
    proj!("lab-ops", "Utility tools for my homelab", "https://github.com/FAZuH/lab-ops"),
    proj!("bimbel-bci.com", "Landing page and mail server for Bintang Cemerlang Insani", "https://bimbel-bci.com"),
    proj!("arthaenergi.id", "Landing page and mail server for PT. Mandala Artha Energi", "https://arthaenergi.id"),
    proj!("krosanevasi.com", "Landing page and mail server for PT. Kros Anevasi", "https://krosanevasi.com"),
]);

#[rustfmt::skip]
static SMALLER: LazyLock<Vec<Project>> = LazyLock::new(|| vec![
    proj!("design-patterns-rust", "Learn software design patterns in Rust", "https://github.com/FAZuH/design-patterns-rust"),
    proj!("warlock", "UI course registration & schedule tracking bot (SIAKNG)", "https://github.com/FAZuH/warlock"),
    proj!("contribution-grid", "Rust crate \u{2014} GitHub-style contribution heatmap generation", "https://crates.io/crates/contribution-grid"),
    proj!("elm-architecture", "PoC of The Elm Architecture across different UI backends in Rust", "https://github.com/FAZuH/elm-architecture"),
    proj!("symlist", "Simple CLI to manage and sync a list of symbolic links", "https://github.com/FAZuH/symlist"),
    proj!("hyper-clear", "CLI tool to control window transparency & blur on Hyprland", "https://github.com/FAZuH/hyper-clear"),
    proj!("ratatui-toaster", "Fork of ratatui-toaster \u{2014} added toast deduplication and stacking", "https://crates.io/crates/ratatui-toaster"),
    proj!("vigilance", "Watch and notify usage of disk, memory, battery, wifi", "https://github.com/FAZuH/vigilance"),
]);

#[rustfmt::skip]
static COURSE_WORK: LazyLock<Vec<Project>> = LazyLock::new(|| vec![
    proj!("Perbankan", "Banking app", "https://github.com/FAZuH/Perbankan"),
]);

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
                                    rsx! { ProjectRow { project: p, is_last } }
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
                                    rsx! { ProjectRow { project: p, is_last } }
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
                                    rsx! { ProjectRow { project: p, is_last } }
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
fn ProjectRow(project: &'static Project, is_last: bool) -> Element {
    let branch = if is_last { "└──" } else { "├──" };

    rsx! {
        a {
            href: &project.link,
            target: if project.link.starts_with("http") { "_blank" } else { "_self" },
            rel: if project.link.starts_with("http") { "noopener noreferrer" } else { "" },
            class: "flex items-baseline gap-3 py-1.5 hover:bg-surface-soft transition-colors rounded-sm \
                group cursor-pointer flex-wrap",
            span { class: "text-mute ml-3 shrink-0", "{branch}" }
            span {
                class: "text-ink font-medium shrink-0 sm:min-w-[8rem] group-hover:text-accent transition-colors",
                "{project.name}"
            }
            span {
                class: "text-mute min-w-0",
                "{project.desc}"
            }
        }
    }
}
