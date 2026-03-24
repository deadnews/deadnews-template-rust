use axum::{Router, routing::get};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tower::ServiceBuilder;
use tower_http::{
    catch_panic::CatchPanicLayer,
    request_id::{MakeRequestUuid, SetRequestIdLayer},
    trace::TraceLayer,
};

use crate::config::Config;
use crate::handler::{database_test, health_check};

#[derive(Clone)]
pub struct App {
    pub db: PgPool,
}

impl App {
    pub fn into_router(self) -> Router {
        Router::new()
            .route("/health", get(health_check))
            .route("/test", get(database_test))
            .layer(
                ServiceBuilder::new()
                    .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
                    .layer(TraceLayer::new_for_http())
                    .layer(CatchPanicLayer::new()),
            )
            .with_state(self)
    }

    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        let db = PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.database_url)
            .await?;

        Ok(Self { db })
    }
}
