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

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .json()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Load configuration
    let config = Config::from_env();

    // Create database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .unwrap_or_else(|e| {
            error!("Failed to create database pool: {e}");
            std::process::exit(1);
        });

    // Run the app
    let app = create_router(AppState { db: pool });
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("Starting HTTP server at http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| {
            error!("Failed to bind to address {addr}: {e}");
            std::process::exit(1);
        });

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap_or_else(|e| {
            error!("Server error: {e}");
            std::process::exit(1);
        });
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
    info!("Shutdown signal received");
}

#[cfg(test)]
mod test;
