mod app;
mod components;
#[cfg(feature = "server")]
pub mod config;
#[cfg(feature = "server")]
pub mod logging;
pub mod rate_limit;
mod server;
#[cfg(feature = "server")]
pub mod smtp;
pub mod validation;

use app::App;

fn main() {
    #[cfg(feature = "server")]
    {
        dotenvy::dotenv().ok();
        logging::init_logging();
    }

    dioxus::launch(App);
}
