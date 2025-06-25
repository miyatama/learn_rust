use crate::{command_extractor::CommandExtractor, state::ApplicationState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use cqrs_es::persist::ViewRepository;

pub async fn query_handler(
    Path(image_id): Path<String>,
    State(state): State<ApplicationState>,
) -> Response {
    match state.image_edit_query.load(&image_id).await {
        Ok(Some(image_edit_view)) => (StatusCode::OK, Json(image_edit_view)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub async fn command_handler(
    Path(image_id): Path<String>,
    State(state): State<ApplicationState>,
    CommandExtractor(metadata, command): CommandExtractor,
) -> Response {
    match state
        .cqrs
        .execute_with_metadata(&image_id, command, metadata)
        .await
    {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()).into_response(),
    }
}
