use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};
use serde_json::json;
use sqlx::PgPool;
use tracing::error;

use crate::db::get_database_info;

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
    Json(json!("Healthy!"))
}

pub async fn database_test(State(state): State<AppState>) -> impl IntoResponse {
    match get_database_info(&state.db).await {
        Ok(info) => Json(info).into_response(),
        Err(e) => {
            error!("Failed to get database info: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Internal server error"})),
            )
                .into_response()
        }
    }
}
