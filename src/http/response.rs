use axum::body::{Bytes, Full};
use axum::response::IntoResponse;
use axum::http::{header, HeaderValue};

#[derive(Clone, Copy, Debug)]
pub struct Css<T>(pub T);

impl<T> IntoResponse for Css<T>
    where T: Into<Full<Bytes>>
{
    fn into_response(self) -> axum::response::Response {
        (
            [(
                header::CONTENT_TYPE,
                HeaderValue::from_static("text/css"),
            )],
            self.0.into(),
        ).into_response()
    }
}

impl<T> From<T> for Css<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Js<T>(pub T);

impl<T> IntoResponse for Js<T>
    where T: Into<Full<Bytes>>
{
    fn into_response(self) -> axum::response::Response {
        (
            [(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/javascript"),
            )],
            self.0.into(),
        ).into_response()
    }
}

impl<T> From<T> for Js<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}
