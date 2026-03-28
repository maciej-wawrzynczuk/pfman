use axum::{
    body::Bytes,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    adapters::csv_parser,
    domain::transaction_log::TransactionLog,
    state::AppState,
};

pub async fn handler(State(state): State<AppState>, body: Bytes) -> impl IntoResponse {
    match csv_parser::parse(&body) {
        Err(e) => {
            tracing::error!("{e}");
            (StatusCode::BAD_REQUEST, e.to_string()).into_response()
        }
        Ok(entries) => {
            *state.write().await = Some(TransactionLog::new(entries));
            StatusCode::OK.into_response()
        }
    }
}
