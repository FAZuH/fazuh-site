use dioxus::prelude::*;
use fazuh_utils::embed::HeadEmbed;

static TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
static FAVICON: Asset = asset!("/assets/profile.jpg");
static FONT_REG: Asset = asset!("/assets/fonts/JetBrainsMono-Regular.ttf");
static FONT_MED: Asset = asset!("/assets/fonts/JetBrainsMono-Medium.ttf");
static FONT_BOLD: Asset = asset!("/assets/fonts/JetBrainsMono-Bold.ttf");

#[component]
pub fn Head() -> Element {
    rsx! {
        document::Title { "FAZuH" }
        document::Link {
            rel: "icon",
            r#type: "image/jpeg",
            href: FAVICON,
        }
        document::Stylesheet { href: TAILWIND_CSS }
        document::Link {
            rel: "preload",
            href: FONT_REG,
            r#as: "font",
            r#type: "font/ttf",
            crossorigin: "anonymous",
        }
        document::Link {
            rel: "preload",
            href: FONT_MED,
            r#as: "font",
            r#type: "font/ttf",
            crossorigin: "anonymous",
        }
        document::Link {
            rel: "preload",
            href: FONT_BOLD,
            r#as: "font",
            r#type: "font/ttf",
            crossorigin: "anonymous",
        }
        document::Style {
            "
            @font-face {{
              font-family: 'JetBrains Mono';
              src: url('{FONT_REG}') format('truetype');
              font-weight: 400;
              font-style: normal;
              font-display: swap;
            }}
            @font-face {{
              font-family: 'JetBrains Mono';
              src: url('{FONT_MED}') format('truetype');
              font-weight: 500;
              font-style: normal;
              font-display: swap;
            }}
            @font-face {{
              font-family: 'JetBrains Mono';
              src: url('{FONT_BOLD}') format('truetype');
              font-weight: 700;
              font-style: normal;
              font-display: swap;
            }}
            "
        }
        HeadEmbed {
            title: "FAZuH",
            description: "Cloud Infrastructure Engineer \u{00b7} Software Developer \u{00b7} Statistics Student",
            url: "https://fazuh.com",
            image: format!("https://fazuh.com{FAVICON}"),
        }
    }
}
