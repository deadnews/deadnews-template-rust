use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let database_url = env::var("SERVICE_DSN")
            .map_err(|_| anyhow::anyhow!("SERVICE_DSN environment variable is required"))?;

        Ok(Self { database_url })
    }
}
