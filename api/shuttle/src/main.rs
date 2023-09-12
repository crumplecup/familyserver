use anyhow::anyhow;
use api_lib::health::check;
use api_lib::state::AppState;
use axum::{routing::get, Router};
use shuttle_runtime::CustomError;
use shuttle_secrets::SecretStore;
use sqlx::{Executor, PgPool};
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing::info;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
    #[shuttle_static_folder::StaticFolder(folder = "static")] static_folder: std::path::PathBuf,
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    let secret = if let Some(pass) = secret_store.get("ADMIN_PASSWORD") {
        pass
    } else {
        return Err(anyhow!("Password was not found").into());
    };
    pool.execute(include_str!("../schema.sql"))
        .await
        .map_err(CustomError::new)?;
    let router = app(pool, static_folder);

    info!("🚀 Server started successfully");
    Ok(router.into())
}

fn app(pool: PgPool, assets: std::path::PathBuf) -> Router {
    let app_state = Arc::new(AppState::new(pool.clone()));
    Router::new()
        // .route("/", get(hello_world))
        .route("/health", get(check))
        .nest_service("/", ServeDir::new(assets))
        .with_state(app_state)
}
