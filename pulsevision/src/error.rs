use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("substrate error: {0}")]
    Substrate(String),

    #[error("session store error: {0}")]
    SessionStore(String),

    #[error("projection error: {0}")]
    Projection(String),

    #[error("websocket error: {0}")]
    WebSocket(String),

    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("invalid request: {0}")]
    InvalidRequest(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match &self {
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::InvalidRequest(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = serde_json::json!({
            "error": self.to_string(),
        });

        (status, axum::Json(body)).into_response()
    }
}

impl From<pulsedb::PulseDBError> for Error {
    fn from(e: pulsedb::PulseDBError) -> Self {
        match e {
            pulsedb::PulseDBError::NotFound(_) => Error::NotFound("Resource not found".into()),
            other => Error::Substrate(other.to_string()),
        }
    }
}
