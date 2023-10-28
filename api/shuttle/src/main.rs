use anyhow::anyhow;
use api_lib::health::check;
use api_lib::interface::user::FamilyUser;
use api_lib::state::AppState;
use axum::extract::{Json, Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post, Router};
use shared::models::user;
use shuttle_runtime::CustomError;
use shuttle_secrets::SecretStore;
use sqlx::{Executor, PgPool};
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing::info;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
    #[shuttle_static_folder::StaticFolder] static_folder: std::path::PathBuf,
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
        .route("/api/users", post(create_user))
        .route("/api/users/:id", get(get_user))
        .nest_service("/", ServeDir::new(assets))
        .with_state(app_state)
}

pub async fn get_user(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    tracing::info!("Getting user {}", &id);
    let user = data.get(id).await;
    match user {
        Ok(result) => Ok((axum::http::StatusCode::OK, Json(result))),
        Err(e) => Err((axum::http::StatusCode::BAD_REQUEST, e.to_string())),
    }
}

pub async fn create_user(
    State(data): State<Arc<AppState>>,
    Json(user): Json<user::User>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    tracing::info!("Creating user");
    let user = data.create(&user).await;
    match user {
        Ok(result) => Ok((axum::http::StatusCode::CREATED, Json(result))),
        Err(e) => Err((axum::http::StatusCode::BAD_REQUEST, e.to_string())),
    }
}
