// Avoid musl's default allocator
#[cfg(target_env = "musl")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod config;
mod db;
mod error;
mod routing;

use config::Config;
use routing::{AppState, create_router};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, prelude::*};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(tracing_logfmt::layer())
        .init();

    if let Err(e) = run().await {
        error!("{e:#}");
        std::process::exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    let config = Config::from_env()?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    let app = create_router(AppState { db: pool });
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    info!("Starting HTTP server at http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
    info!("Shutdown signal received");
}

#[cfg(test)]
mod test;
