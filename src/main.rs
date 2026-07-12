mod app;
mod components;
pub mod server_stats;

use app::App;

fn main() {
    #[cfg(feature = "server")]
    {
        dotenvy::dotenv().ok();
        fazuh_utils::logging::init_logging();
    }

    dioxus::launch(App);
}
