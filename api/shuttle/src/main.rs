use anyhow::anyhow;
use api_lib::health::check;
use api_lib::interface::user::FamilyUser;
use api_lib::state::AppState;
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
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
        .route("/health/check_user", get(check_user))
        .route("/api/users", get(get_users).post(create_user))
        .route(
            "/api/users/:id",
            get(get_user).put(update_user).delete(delete_user),
        )
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
        Ok(result) => Ok((StatusCode::OK, Json(result))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

pub async fn get_users(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    tracing::info!("Getting all users.");
    let user = data.get_all().await;
    match user {
        Ok(result) => Ok((StatusCode::OK, Json(result))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

pub async fn create_user(
    State(data): State<Arc<AppState>>,
    Json(user): Json<serde_json::Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    // tracing::info!("Creating user {}.", &user.username_ref());
    tracing::info!("Creating user {}.", &user["username"]);
    let usr = user::User::new(
        &user["username"].to_string(),
        &user["password_hash"].to_string(),
    );
    let user = data.create(&usr).await;
    match user {
        Ok(result) => Ok((StatusCode::CREATED, Json(result))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

pub async fn update_user(
    State(data): State<Arc<AppState>>,
    Json(user): Json<serde_json::Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    // tracing::info!("Creating user {}.", &user.username_ref());
    tracing::info!("Updating user {}.", &user["username"]);
    let mut usr = user::User::new(
        &user["username"].to_string(),
        &user["password_hash"].to_string(),
    );
    usr.set_id(uuid::Uuid::parse_str(&user["id"].to_string()).unwrap());
    let res = data.update(&usr).await;
    match res {
        Ok(result) => Ok((StatusCode::OK, Json(result))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

pub async fn delete_user(
    State(data): State<Arc<AppState>>,
    Json(user): Json<serde_json::Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    // tracing::info!("Creating user {}.", &user.username_ref());
    tracing::info!("Deleting user {}.", &user["username"]);
    let mut usr = user::User::new(
        &user["username"].to_string(),
        &user["password_hash"].to_string(),
    );
    usr.set_id(uuid::Uuid::parse_str(&user["id"].to_string()).unwrap());
    let res = data.delete(&usr).await;
    match res {
        Ok(result) => Ok((StatusCode::OK, Json(result))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

pub async fn check_user() -> impl IntoResponse {
    let username = "crumplecup";
    let password = "password";
    let user = user::User::new(username, password);
    (StatusCode::OK, Json(user))
}
