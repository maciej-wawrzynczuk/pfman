use axum::{extract::State, response::IntoResponse, Json};

use crate::{adapters::json_dto::TransactionEntryDto, state::AppState};

pub async fn handler(State(state): State<AppState>) -> impl IntoResponse {
    let guard = state.read().await;
    let dtos: Vec<TransactionEntryDto> = guard
        .as_ref()
        .map(|log| log.iter().map(TransactionEntryDto::from).collect())
        .unwrap_or_default();
    Json(dtos)
}
