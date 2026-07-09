use dioxus::prelude::*;

static AVATAR: Asset = asset!("/assets/profile.jpg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        section {
            id: "top",
            class: "pt-32 pb-24 md:pt-40 md:pb-32",
            div {
                class: "max-w-[960px] mx-auto px-6 grid md:grid-cols-2 gap-12 md:gap-16 items-center",
                div {
                    class: "flex flex-col gap-6",
                    img {
                        src: AVATAR,
                        alt: "FAZuH profile picture",
                        class: "w-28 h-28 md:w-32 md:h-32 object-cover rounded-sm border border-hairline",
                        loading: "eager",
                    }
                    div {
                        h1 {
                            class: "text-3xl md:text-4xl font-bold tracking-tighter leading-[1.1] text-ink",
                            "FAZuH"
                        }
                        p {
                            class: "text-sm md:text-base text-mute mt-2 leading-relaxed max-w-[32ch]",
                            "Software Developer · Infrastructure Engineer · Statistics Student"
                        }
                    }
                    div {
                        class: "flex flex-col sm:flex-row gap-3 mt-2",
                        a {
                            href: "#projects",
                            class: "inline-block text-center px-5 py-2 text-sm font-medium bg-ink text-canvas rounded-sm hover:bg-ink-deep transition-colors",
                            "[>] View Projects"
                        }
                        a {
                            href: "#skills",
                            class: "inline-block text-center px-5 py-2 text-sm font-medium bg-canvas text-ink border border-hairline-strong rounded-sm hover:bg-surface-soft transition-colors",
                            "[>] View Skills"
                        }
                    }
                }

                // TUI-card: dark surface with block-pixel ASCII wordmark + neofetch-style stats
                div {
                    class: "bg-surface-dark text-on-dark p-8 md:p-10 font-mono leading-[1.4] text-xs md:text-sm",
                    div {
                        class: "mb-6",
                        div { class: "whitespace-pre", {WORDMARK} }
                    }
                    div {
                        class: "border-t border-on-dark-mute/20 pt-4 flex flex-col gap-1.5",
                        StatLine { label: "OS", value: "Arch Linux" }
                        StatLine { label: "Shell", value: "zsh" }
                        StatLine { label: "WM", value: "Hyprland" }
                        StatLine { label: "Editor", value: "Neovim" }
                        StatLine { label: "Lang", value: "Rust, Python" }
                        StatLine { label: "Servers", value: "4 nodes · 99 ctnrs" }
                    }
                    div {
                        class: "mt-4 pt-3 border-t border-on-dark-mute/20 text-on-dark-mute text-xs",
                        "[+] uptime: ~4 years ops"
                        br {}
                        "[+] projects: 38 public on GitHub"
                    }
                }
            }
        }
    }
}

#[component]
fn StatLine(label: &'static str, value: &'static str) -> Element {
    rsx! {
        div {
            class: "flex justify-between",
            span { class: "text-on-dark-mute", "{label}" }
            span { class: "text-on-dark", "{value}" }
        }
    }
}

const WORDMARK: &str = r#"
┏┓┏┓┏┓  ┓┏
┣ ┣┫┏┛┓┏┣┫
┻ ┛┗┗┛┗┻┛┗
"#;
