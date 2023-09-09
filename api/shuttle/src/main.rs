use api_lib::health::check;
use api_lib::state::AppState;
use axum::{routing::get, Router};
use shuttle_runtime::CustomError;
use sqlx::{Executor, PgPool};
use std::sync::Arc;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn axum(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    pool.execute(include_str!("../schema.sql"))
        .await
        .map_err(CustomError::new)?;
    let router = app(pool);

    Ok(router.into())
}

fn app(pool: PgPool) -> Router {
    let app_state = Arc::new(AppState::new(pool.clone()));
    Router::new()
        .route("/", get(hello_world))
        .route("/health", get(check))
        .with_state(app_state)
}
