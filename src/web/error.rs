use axum::Json;

pub enum ProfileApiError {
    NotFound,
    BadRequest(String),
    InternalError(String),
}

#[derive(serde::Serialize)]
struct ErrorResponse {
    pub reason: String,
}

impl axum::response::IntoResponse for ProfileApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ProfileApiError::NotFound => (http::StatusCode::NOT_FOUND).into_response(),
            ProfileApiError::BadRequest(reason) => (
                http::StatusCode::BAD_REQUEST,
                Json(ErrorResponse { reason }),
            )
                .into_response(),
            ProfileApiError::InternalError(reason) => (
                http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { reason }),
            )
                .into_response(),
        }
    }
}
