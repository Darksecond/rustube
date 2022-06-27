use super::prelude::*;
use axum::extract::Path;
use axum::routing::put;
use axum::Json;
use crate::model::playstate::{UpsertPlaystate, upsert_playstate, get_playstate};

async fn put_playstates_api(context: Extension<HttpContext>,
                           Json(playstate): Json<UpsertPlaystate>) -> Result<impl IntoResponse, HttpError> {
    upsert_playstate(playstate, &context.db)
        .await?;

    Ok(())
}

async fn get_playstate_api(context: Extension<HttpContext>,
                       Path(id): Path<String>) -> Result<impl IntoResponse, HttpError> {

    let playstate = get_playstate(&id, &context.db)
        .await?
        .ok_or(HttpError::NotFound)?;

    Ok(Json(playstate))
}

pub fn router() -> Router {
    axum::Router::new()
        .route("/api/playstates/:id", get(get_playstate_api))
        .route("/api/playstates", put(put_playstates_api))
}
