use axum::Router;
use axum::routing::get;
use axum::response::IntoResponse;
use super::response::{Css, Js};

pub fn router() -> Router {
    axum::Router::new()
        .route("/assets/style.css", get(style_css))
        .route("/assets/normalize.css", get(normalize_css))
        .route("/assets/milligram.css", get(milligram_css))
        .route("/assets/watch.js", get(watch_js))
        .route("/assets/touch-icon.png", get(touch_icon_png))
        .route("/assets/mask-icon.svg", get(mask_icon_svg))
        .route("/assets/favicon.png", get(favicon_png))
}

async fn style_css() -> impl IntoResponse {
    Css(include_str!("../../assets/style.css"))
}

async fn normalize_css() -> impl IntoResponse {
    Css(include_str!("../../assets/normalize.css"))
}

async fn milligram_css() -> impl IntoResponse {
    Css(include_str!("../../assets/milligram.css"))
}

async fn watch_js() -> impl IntoResponse {
    Js(include_str!("../../assets/watch.js"))
}

async fn touch_icon_png() -> impl IntoResponse {
    use axum::http::{header, HeaderValue};
    (
            [(
                header::CONTENT_TYPE,
                HeaderValue::from_static("image/png"),
            )],
            include_bytes!("../../assets/touch-icon.png") as &[u8]
    )
}

async fn mask_icon_svg() -> impl IntoResponse {
    use axum::http::{header, HeaderValue};
    (
            [(
                header::CONTENT_TYPE,
                HeaderValue::from_static("image/svg+xml"),
            )],
            include_bytes!("../../assets/mask-icon.svg") as &[u8]
    )
}

async fn favicon_png() -> impl IntoResponse {
    use axum::http::{header, HeaderValue};
    (
            [(
                header::CONTENT_TYPE,
                HeaderValue::from_static("image/png"),
            )],
            include_bytes!("../../assets/favicon.png") as &[u8]
    )
}
