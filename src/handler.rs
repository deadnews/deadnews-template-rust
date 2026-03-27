use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Serialize;

use crate::app::App;
use crate::db::get_database_info;

pub async fn health_check() -> &'static str {
    "Healthy\n"
}

pub async fn database_test(State(app): State<App>) -> Result<impl IntoResponse, AppError> {
    let info = get_database_info(&app.db).await?;
    Ok(Json(info))
}

/// Wraps any error into an HTTP 500 JSON response via `?` in handlers.
pub struct AppError(anyhow::Error);

impl<E: Into<anyhow::Error>> From<E> for AppError {
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: &'static str,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("{:#}", self.0);

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Internal server error",
            }),
        )
            .into_response()
    }
}
