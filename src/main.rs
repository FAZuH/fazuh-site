mod app;
mod components;
pub mod server;
pub mod server_stats;
pub mod validation;

use app::App;

fn main() {
    #[cfg(feature = "server")]
    {
        dotenvy::dotenv().ok();
        server::init_logging();
    }

    dioxus::launch(App);
}
