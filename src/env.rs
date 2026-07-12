use dotenvy::dotenv;
use env_logger::{Builder, Env};

/// Load environment configuration from `.env` file and initialize logging
pub fn init_env() {
    // Load environment variables
    dotenv().ok();

    // Set default log level to "info"
    // Can be adjusted via environment variable `RUST_LOG`
    let log_level = Env::default().default_filter_or("info");

    // Initialize global logger
    Builder::from_env(log_level).init();
}
