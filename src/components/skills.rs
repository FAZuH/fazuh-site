use std::sync::LazyLock;

use dioxus::prelude::*;

use crate::components::Collapsible;
use crate::server_stats::CONTAINER_COUNT;
use crate::server_stats::SERVER_COUNT;
use crate::server_stats::STACK_COUNT;

struct Framework {
    name: String,
    desc: String,
}

struct Lang {
    name: String,
    badge: String,
    children: Vec<Framework>,
}

struct Tool {
    badge: String,
    desc: String,
}

macro_rules! fw {
    ($name:expr, $desc:expr) => {
        Framework {
            name: $name.into(),
            desc: $desc.into(),
        }
    };
}

macro_rules! lang {
    ($name:expr, $badge:expr, [$($child:expr),* $(,)?]) => {
        Lang {
            name: $name.into(),
            badge: $badge.into(),
            children: vec![$($child),*]
        }
    };
    ($name:expr, $badge:expr) => {
        Lang {
            name: $name.into(),
            badge: $badge.into(),
            children: Vec::new()
        }
    };
}

macro_rules! tool {
    ($desc:expr, $badge:expr) => {
        Tool {
            badge: $badge.into(),
            desc: $desc.into(),
        }
    };
}

#[rustfmt::skip]
static LANGS: LazyLock<Vec<Lang>> = LazyLock::new(|| vec![
    lang!("Rust", "https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust&logoColor=white", [
        fw!("Poise", "Discord bot"),
        fw!("ratatui", "TUI"),
        fw!("Diesel", "ORM"),
        fw!("sqlx", "SQLite"),
        fw!("Axum", "Web backend"),
    ]),
    lang!("Python", "https://img.shields.io/badge/Python-3776AB?style=flat-square&logo=python&logoColor=white", [
        fw!("Nextcord", "Discord bot"),
        fw!("SQLAlchemy", "Database ORM"),
        fw!("Pandas, NumPy", "Data analysis"),
        fw!("Matplotlib", "Plotting"),
        fw!("Playwright", "Web automation"),
        fw!("Streamlit", "Rapid web prototyping"),
    ]),
    lang!("MySQL", "https://img.shields.io/badge/MySQL-4479A1?style=flat-square&logo=mysql&logoColor=white"),
]);

#[rustfmt::skip]
static DEVOPS: LazyLock<Vec<Tool>> = LazyLock::new(|| vec![
    tool!("Declarative service management", "https://img.shields.io/badge/Docker-2496ED?style=flat-square&logo=docker&logoColor=white"),
    tool!("Configuration management and node provisioning", "https://img.shields.io/badge/Ansible-EE0000?style=flat-square&logo=ansible&logoColor=white"),
    tool!("Secure connectivity for private services, gaming, exit nodes", "https://img.shields.io/badge/Tailscale-242424?style=flat-square&logo=tailscale&logoColor=white"),
    tool!("Reverse proxy", "https://img.shields.io/badge/NGINX-009639?style=flat-square&logo=nginx&logoColor=white"),
    tool!("Service configs for auto-discovery and routing with lab-ops", "https://img.shields.io/badge/Consul-F24C53?style=flat-square&logo=consul&logoColor=white"),
    tool!("Automatic backups", "https://img.shields.io/badge/Restic-00ADEE?style=flat-square"),
    tool!("Automatic backups (wrapper)", "https://img.shields.io/badge/Autorestic-000000?style=flat-square"),
    tool!("Simple scripting & automation", "https://img.shields.io/badge/Bash-4EAA25?style=flat-square&logo=gnu-bash&logoColor=white"),
    tool!("Secrets encryption", "https://img.shields.io/badge/SOPS-5E6772?style=flat-square"),
    tool!("Secrets encryption", "https://img.shields.io/badge/age-000000?style=flat-square"),
    tool!("Server monitoring", "https://img.shields.io/badge/Grafana-F46800?style=flat-square&logo=grafana&logoColor=white"),
    tool!("Server monitoring", "https://img.shields.io/badge/Prometheus-E6522C?style=flat-square&logo=prometheus&logoColor=white"),
    tool!("Server monitoring", "https://img.shields.io/badge/Uptime_Kuma-607D8B?style=flat-square&logo=uptime-kuma&logoColor=white"),
    tool!("Security monitoring (SIEM / XDR)", "https://img.shields.io/badge/Wazuh-3585F8?style=flat-square&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCA0OCA0OCI+PHBhdGggZmlsbD0iIzM1ODVmOCIgZD0iTTI0IDBDMTAuOTcxIDAgMCAxMC45NzEgMCAyNHMxMC42MjkgMjQgMjQgMjQgMjQtMTAuNjI5IDI0LTI0UzM3LjAyOSAwIDI0IDBtMS4zNzEgMzIuOTE0LTQuMTE0LTEzLjAyOS00LjExNCAxMy4wMjloLTMuMDg2TDguNTcxIDE0Ljc0M0gxMmwzLjc3MSAxMi4zNDMgMy43NzItMTIuMzQzaDMuMDg2bDMuNzcgMTIuMzQzIDMuNzcyLTEyLjM0M0gzMy42bC01LjE0MyAxOC4xNzF6bTEwLjk3MS4zNDNjLTEuNzE0IDAtMi43NDMtMS4zNzItMi43NDMtMi43NDMgMC0xLjcxNCAxLjM3Mi0yLjc0MyAyLjc0My0yLjc0MyAxLjM3MiAwIDIuNzQzIDEuMzcxIDIuNzQzIDIuNzQzcy0xLjAyOCAyLjc0My0yLjc0MyAyLjc0MyIvPjwvc3ZnPg=="),
    tool!("Infrastructure as Code for cloud infrastructure and VM provisioning", "https://img.shields.io/badge/Terraform-7B42BC?style=flat-square&logo=terraform&logoColor=white"),
    tool!(format!("Cloud provider — managing {SERVER_COUNT} nodes"), "https://img.shields.io/badge/OVH-0050D7?style=flat-square&logo=ovh&logoColor=white"),
    tool!("Virtualization in dedicated server for VMs and resource allocation", "https://img.shields.io/badge/Proxmox-E57000?style=flat-square&logo=proxmox&logoColor=white"),
]);

#[component]
pub fn Skills() -> Element {
    rsx! {
        section {
            id: "skills",
            class: "py-16 md:py-20",
            div {
                class: "max-w-[960px] mx-auto px-6",
                h2 {
                    class: "text-xl font-bold text-ink mb-2",
                    "Skills"
                }
                p {
                    class: "text-sm text-mute mb-8 max-w-[65ch]",
                    "List of tools and what I've used them for in my projects."
                }

                div {
                    class: "border-t border-hairline",

                    Collapsible {
                        label: "software-development/",
                        default_open: true,
                        children: rsx! {
                            {LANGS.iter().enumerate().map(|(i, lang)| {
                                let is_last = i == LANGS.len() - 1;
                                let branch = if is_last { "└──" } else { "├──" };
                                let continuation = if is_last { "   " } else { "│  " };
                                let children = &lang.children;
                                let children_empty = children.is_empty();
                                let name = &lang.name;
                                let badge = &lang.badge;
                                rsx! {
                                    div {
                                        class: "flex items-start gap-2 py-1.5 flex-wrap",
                                        span { class: "text-mute", "{branch}" }
                                        img {
                                            src: badge,
                                            alt: name,
                                            height: "20",
                                            loading: "lazy",
                                            class: "h-5 inline-block align-middle",
                                        }
                                        span {
                                            class: "text-ink font-medium self-center",
                                            "{name}"
                                        }
                                        if !children_empty {
                                            div {
                                                class: "w-full",
                                                {children.iter().enumerate().map(|(ci, fw)| {
                                                    let fw_last = ci == children.len() - 1;
                                                    let fw_branch = if fw_last { "└──" } else { "├──" };
                                                    let fw_name = &fw.name;
                                                    let fw_desc = &fw.desc;
                                                    rsx! {
                                                        div {
                                                            class: "flex items-baseline gap-3 py-0.5",
                                                            span { class: "text-mute whitespace-pre", "{continuation}{fw_branch}" }
                                                            span { class: "text-ink min-w-[7rem]", "{fw_name}" }
                                                            span { class: "text-mute whitespace-nowrap", "{fw_desc}" }
                                                        }
                                                    }
                                                })}
                                            }
                                        }
                                    }
                                }
                            })}
                        }
                    }
                }

                div {
                    class: "border-t border-hairline mt-8",

                    Collapsible {
                        label: "infrastructure-engineering/",
                        default_open: true,
                        children: rsx! {
                            div { class: "flex flex-col overflow-x-auto",
                                {DEVOPS.iter().enumerate().map(|(i, tool)| {
                                    let is_last = i == DEVOPS.len() - 1;
                                    let branch = if is_last { "└──" } else { "├──" };
                                    let badge = &tool.badge;
                                    let desc = if i == 0 {
                                        format!("Declarative service management \u{2014} {SERVER_COUNT} servers, ~{STACK_COUNT} compose stacks, {CONTAINER_COUNT} containers")
                                    } else {
                                        tool.desc.to_string()
                                    };
                                    rsx! {
                                        div {
                                            class: "flex items-start gap-3 py-1.5 flex-nowrap",
                                            span { class: "text-mute", "{branch}" }
                                            img {
                                                src: badge,
                                                height: "20",
                                                loading: "lazy",
                                                class: "h-5 inline-block align-middle",
                                            }
                                            span { class: "text-body self-center whitespace-nowrap", "{desc}" }
                                        }
                                    }
                                })}
                            }
                        }
                    }
                }
            }
        }
    }
}
