use std::env;
use tracing::error;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        let database_url = env::var("SERVICE_DSN").unwrap_or_else(|_| {
            error!("SERVICE_DSN environment variable is required");
            std::process::exit(1);
        });

        Self { database_url }
    }
}
