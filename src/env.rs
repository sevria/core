use anyhow::Result;
use dotenvy::dotenv;
use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Env {
    #[envconfig(from = "HTTP_ADDRESS", default = "0.0.0.0:3000")]
    pub http_address: String,

    #[envconfig(from = "HTTP_SERVER_URL", default = "/")]
    pub http_server_url: String,
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
