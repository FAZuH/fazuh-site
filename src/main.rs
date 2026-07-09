mod app;
mod components;
mod server;
mod utils;

use app::App;

fn main() {
    #[cfg(feature = "server")]
    {
        dotenvy::dotenv().ok();
        utils::logging::init_logging();
    }

    dioxus::launch(App);
}
