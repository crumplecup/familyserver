use super::interface::user::{FamilyResult, FamilyUser};
use shared::models::user;
use sqlx::PgPool;

pub const API_VERSION: &str = "v0.0.1";

pub struct AppState {
    db: PgPool,
}

impl AppState {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &PgPool {
        &self.db
    }
}

#[async_trait::async_trait]
impl FamilyUser for AppState {
    async fn get(&self, id: uuid::Uuid) -> FamilyResult<user::User> {
        sqlx::query_as::<_, user::User>(
            r#"
      SELECT id, username, password_hash
      FROM users
      WHERE id = $1
      "#,
        )
        .bind(id)
        .fetch_one(self.db())
        .await
        .map_err(|e| e.to_string())
    }

    async fn create(&self, user: &user::User) -> FamilyResult<user::User> {
        sqlx::query_as::<_, user::User>(
            r#"
      INSERT INTO users (username, password_hash)
      VALUES ($1 $2 $3)
      RETURNING id, username, password_hash
      "#,
        )
        .bind(user.id())
        .bind(user.username_ref())
        .bind(user.password_hash_ref())
        .fetch_one(self.db())
        .await
        .map_err(|e| e.to_string())
    }

    async fn update(&self, user: &user::User) -> FamilyResult<user::User> {
        sqlx::query_as::<_, user::User>(
            r#"
      UPDATE users
      SET id = $1, username = $2, password_hash = $3
      WHERE id = $1
      RETURNING id, username, password_hash
      "#,
        )
        .bind(user.id())
        .bind(user.username_ref())
        .bind(user.password_hash_ref())
        .fetch_one(self.db())
        .await
        .map_err(|e| e.to_string())
    }
}
