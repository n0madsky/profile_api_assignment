pub enum ProfileApiError {
    NotFound,
    BadRequest,
    InternalError,
}

impl axum::response::IntoResponse for ProfileApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ProfileApiError::NotFound => (http::StatusCode::NOT_FOUND).into_response(),
            ProfileApiError::BadRequest => (http::StatusCode::BAD_REQUEST).into_response(),
            ProfileApiError::InternalError => {
                (http::StatusCode::INTERNAL_SERVER_ERROR).into_response()
            }
        }
    }
}
