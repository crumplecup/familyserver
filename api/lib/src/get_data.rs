use crate::state::{AppState, API_VERSION};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::sync::Arc;

pub async fn check(State(data): State<Arc<AppState>>) -> impl IntoResponse {
    tracing::info!("Getting version");
    let ver;
    let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
        .fetch_one(data.db())
        .await;

    match result {
        Ok(version) => ver = version,
        Err(e) => ver = format!("Error: {:?}", e),
    };

    (StatusCode::OK, [("family_server", API_VERSION)], ver)
}
