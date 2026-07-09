use std::env;
use std::sync::OnceLock;

pub struct Config {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_from: String,
    pub smtp_to: String,
    pub log_level: String,
    pub default_log_dir: String,
    pub log_filename: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            smtp_host: env::var("SMTP_HOST").unwrap_or_else(|_| "localhost".to_string()),
            smtp_port: env::var("SMTP_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(587),
            smtp_username: env::var("SMTP_USERNAME").unwrap_or_default(),
            smtp_password: env::var("SMTP_PASSWORD").unwrap_or_default(),
            smtp_from: env::var("SMTP_FROM")
                .unwrap_or_else(|_| "FAZuH Site <site@fazuh.com>".to_string()),
            smtp_to: env::var("SMTP_TO").unwrap_or_else(|_| "mail@fazuh.com".to_string()),
            log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "INFO".to_string()),
            default_log_dir: env::var("DEFAULT_LOG_DIR").unwrap_or_else(|_| "./logs".to_string()),
            log_filename: env::var("LOG_FILENAME").unwrap_or_else(|_| "app.log".to_string()),
        }
    }

    pub fn get() -> &'static Config {
        static CONFIG: OnceLock<Config> = OnceLock::new();
        CONFIG.get_or_init(Config::new)
    }
}
