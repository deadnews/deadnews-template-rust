use std::env;
use tracing::error;

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        let port = env::var("SERVICE_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(8000);

        let database_url = env::var("SERVICE_DSN").unwrap_or_else(|_| {
            error!("SERVICE_DSN environment variable is required");
            std::process::exit(1);
        });

        Self { port, database_url }
    }
}
