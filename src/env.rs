use anyhow::Result;
use dotenvy::dotenv;
use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Env {
    #[envconfig(from = "CARGO_PKG_NAME")]
    pub service_name: String,

    #[envconfig(from = "CARGO_PKG_VERSION")]
    pub service_version: String,
}

/// Load environment configuration from `.env` file and initialize logging
pub fn init_env() -> Result<Env> {
    // Load environment variables
    dotenv().ok();

    // Set default log level to "info"
    // Can be adjusted via environment variable `RUST_LOG`
    let log_level = env_logger::Env::default().default_filter_or("info");

    // Initialize global logger
    env_logger::Builder::from_env(log_level).init();

    Ok(Env::init_from_env()?)
}
