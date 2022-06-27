use thiserror::Error;
use axum::response::IntoResponse;
use axum::http::StatusCode;

#[derive(Debug, Error)]
pub enum HttpError {
    /// Return `404 Not Found`
    #[error("Not Found")]
    NotFound,

    /// Return `500 Internal Server  Error`
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Return `500 Internal Server  Error`
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    /// Return `500 Internal Server  Error`
    #[error(transparent)]
    Sql(#[from] sqlx::Error),

    /// Return `500 Internal Server  Error`
    #[error(transparent)]
    Askama(#[from] askama::Error),
}

impl HttpError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Io(ref e) => tracing::error!(%e, "IO error"),
            Self::Anyhow(ref e) => tracing::error!(%e, "Generic error"),
            Self::Sql(ref e) => tracing::error!(%e, "Sql error"),
            _ => (),
        }

        (self.status_code(), self.to_string()).into_response()
    }
}
