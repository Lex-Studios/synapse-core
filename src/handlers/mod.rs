use crate::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse};

pub async fn health(State(_state): State<AppState>) -> impl IntoResponse {
    "OK"
}

pub async fn callback_transaction(State(_state): State<AppState>) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}
