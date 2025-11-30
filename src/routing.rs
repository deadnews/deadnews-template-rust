use axum::{Json, Router, extract::State, response::IntoResponse, routing::get};
use sqlx::PgPool;

use crate::db::get_database_info;
use crate::error::AppError;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/health", get(health_check))
        .route("/test", get(database_test))
        .with_state(state)
}

async fn index() -> &'static str {
    "Hello world!"
}

async fn health_check() -> impl IntoResponse {
    Json("Healthy!")
}

pub async fn database_test(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let info = get_database_info(&state.db).await?;
    Ok(Json(info))
}
