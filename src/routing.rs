use axum::{Json, Router, extract::State, response::IntoResponse, routing::get};
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::{
    catch_panic::CatchPanicLayer,
    request_id::{MakeRequestUuid, SetRequestIdLayer},
    trace::TraceLayer,
};

use crate::db::get_database_info;
use crate::error::AppError;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/test", get(database_test))
        .layer(
            ServiceBuilder::new()
                .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
                .layer(TraceLayer::new_for_http())
                .layer(CatchPanicLayer::new()),
        )
        .with_state(state)
}

async fn health_check() -> &'static str {
    "Healthy"
}

pub async fn database_test(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let info = get_database_info(&state.db).await?;
    Ok(Json(info))
}
