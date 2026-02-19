use crate::{error::AppError, AppState};
use axum::{extract::State, response::IntoResponse};

pub async fn health(State(_state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    Ok("OK")
}
