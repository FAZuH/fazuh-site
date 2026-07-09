use dioxus::prelude::*;

use crate::components::Collapsible;

struct Framework {
    name: &'static str,
    desc: &'static str,
}

struct Lang {
    name: &'static str,
    badge: &'static str,
    children: &'static [Framework],
}

struct Tool {
    badge: &'static str,
    desc: &'static str,
}

const LANGS: &[Lang] = &[
    Lang {
        name: "Rust",
        badge: "https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust&logoColor=white",
        children: &[
            Framework {
                name: "Poise",
                desc: "Discord bot",
            },
            Framework {
                name: "ratatui",
                desc: "TUI",
            },
            Framework {
                name: "Diesel",
                desc: "ORM",
            },
            Framework {
                name: "sqlx",
                desc: "SQLite",
            },
            Framework {
                name: "Axum",
                desc: "Web backend",
            },
        ],
    },
    Lang {
        name: "Python",
        badge: "https://img.shields.io/badge/Python-3776AB?style=flat-square&logo=python&logoColor=white",
        children: &[
            Framework {
                name: "Nextcord",
                desc: "Discord bot",
            },
            Framework {
                name: "SQLAlchemy",
                desc: "Database ORM",
            },
            Framework {
                name: "Pandas, NumPy",
                desc: "Data analysis",
            },
            Framework {
                name: "Matplotlib",
                desc: "Plotting",
            },
            Framework {
                name: "Playwright",
                desc: "Web automation",
            },
            Framework {
                name: "Streamlit",
                desc: "Rapid web prototyping",
            },
        ],
    },
    Lang {
        name: "MySQL",
        badge: "https://img.shields.io/badge/MySQL-4479A1?style=flat-square&logo=mysql&logoColor=white",
        children: &[],
    },
];

const DEVOPS: &[Tool] = &[
    Tool {
        badge: "https://img.shields.io/badge/Docker-2496ED?style=flat-square&logo=docker&logoColor=white",
        desc: "Declarative service management \u{2014} 4 servers, ~20 compose stacks, 99 containers",
    },
    Tool {
        badge: "https://img.shields.io/badge/Ansible-EE0000?style=flat-square&logo=ansible&logoColor=white",
        desc: "Configuration management and node provisioning",
    },
    Tool {
        badge: "https://img.shields.io/badge/Tailscale-242424?style=flat-square&logo=tailscale&logoColor=white",
        desc: "Secure connectivity for private services, gaming, exit nodes",
    },
    Tool {
        badge: "https://img.shields.io/badge/NGINX-009639?style=flat-square&logo=nginx&logoColor=white",
        desc: "Reverse proxy",
    },
    Tool {
        badge: "https://img.shields.io/badge/Consul-F24C53?style=flat-square&logo=consul&logoColor=white",
        desc: "Service configs for auto-discovery and routing with lab-ops",
    },
    Tool {
        badge: "https://img.shields.io/badge/Restic-00ADEE?style=flat-square",
        desc: "Automatic backups",
    },
    Tool {
        badge: "https://img.shields.io/badge/Autorestic-000000?style=flat-square",
        desc: "Automatic backups (wrapper)",
    },
    Tool {
        badge: "https://img.shields.io/badge/Bash-4EAA25?style=flat-square&logo=gnu-bash&logoColor=white",
        desc: "Simple scripting & automation",
    },
    Tool {
        badge: "https://img.shields.io/badge/SOPS-5E6772?style=flat-square",
        desc: "Secrets encryption",
    },
    Tool {
        badge: "https://img.shields.io/badge/age-000000?style=flat-square",
        desc: "Secrets encryption",
    },
    Tool {
        badge: "https://img.shields.io/badge/Grafana-F46800?style=flat-square&logo=grafana&logoColor=white",
        desc: "Server monitoring",
    },
    Tool {
        badge: "https://img.shields.io/badge/Prometheus-E6522C?style=flat-square&logo=prometheus&logoColor=white",
        desc: "Server monitoring",
    },
    Tool {
        badge: "https://img.shields.io/badge/Uptime_Kuma-607D8B?style=flat-square&logo=uptime-kuma&logoColor=white",
        desc: "Server monitoring",
    },
    Tool {
        badge: "https://img.shields.io/badge/Wazuh-3585F8?style=flat-square&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCA0OCA0OCI+PHBhdGggZmlsbD0iIzM1ODVmOCIgZD0iTTI0IDBDMTAuOTcxIDAgMCAxMC45NzEgMCAyNHMxMC42MjkgMjQgMjQgMjQgMjQtMTAuNjI5IDI0LTI0UzM3LjAyOSAwIDI0IDBtMS4zNzEgMzIuOTE0LTQuMTE0LTEzLjAyOS00LjExNCAxMy4wMjloLTMuMDg2TDguNTcxIDE0Ljc0M0gxMmwzLjc3MSAxMi4zNDMgMy43NzItMTIuMzQzaDMuMDg2bDMuNzcgMTIuMzQzIDMuNzcyLTEyLjM0M0gzMy42bC01LjE0MyAxOC4xNzF6bTEwLjk3MS4zNDNjLTEuNzE0IDAtMi43NDMtMS4zNzItMi43NDMtMi43NDMgMC0xLjcxNCAxLjM3Mi0yLjc0MyAyLjc0My0yLjc0MyAxLjM3MiAwIDIuNzQzIDEuMzcxIDIuNzQzIDIuNzQzcy0xLjAyOCAyLjc0My0yLjc0MyAyLjc0MyIvPjwvc3ZnPg==",
        desc: "Security monitoring (SIEM / XDR)",
    },
    Tool {
        badge: "https://img.shields.io/badge/Terraform-7B42BC?style=flat-square&logo=terraform&logoColor=white",
        desc: "Infrastructure as Code for cloud infrastructure and VM provisioning",
    },
    Tool {
        badge: "https://img.shields.io/badge/OVH-0050D7?style=flat-square&logo=ovh&logoColor=white",
        desc: "Cloud provider \u{2014} 1 VPS & 1 dedicated (+2 VM) \u{2014} 4 nodes",
    },
    Tool {
        badge: "https://img.shields.io/badge/Proxmox-E57000?style=flat-square&logo=proxmox&logoColor=white",
        desc: "Virtualization in dedicated server for VMs and resource allocation",
    },
];

#[component]
pub fn Skills() -> Element {
    rsx! {
        section {
            id: "skills",
            class: "py-16 md:py-20",
            div {
                class: "max-w-[960px] mx-auto px-6",
                h2 {
                    class: "text-base font-bold text-ink mb-2",
                    "[+] skills"
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
                                let children = lang.children;
                                let children_empty = children.is_empty();
                                let name = lang.name;
                                let badge = lang.badge;
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
                                                    let fw_name = fw.name;
                                                    let fw_desc = fw.desc;
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
                                    let badge = tool.badge;
                                    let desc = tool.desc;
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
