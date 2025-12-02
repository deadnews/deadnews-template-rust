use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

/// Application error wrapper.
/// Wraps any error and converts it to an HTTP 500 response.
///
/// Usage in handlers:
/// ```ignore
/// async fn handler() -> Result<impl IntoResponse, AppError> {
///     let data = fallible_operation().await?;
///     Ok(Json(data))
/// }
/// ```
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
        tracing::error!("{}", self.0);

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Internal server error",
            }),
        )
            .into_response()
    }
}
