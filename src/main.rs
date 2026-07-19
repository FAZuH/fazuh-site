mod app;
mod components;
pub mod server_stats;

use app::App;

fn main() {
    #[cfg(feature = "server")]
    {
        match dotenvy::dotenv() {
            Ok(_) => {}
            Err(e) if e.not_found() => {}
            Err(e) => panic!("Failed to load .env: {e}"),
        }
        fazuh_utils::logging::init_logging();
    }

    dioxus::launch(App);
}
