use sqlx::PgPool;

pub struct AppState {
    db: PgPool,
}

impl AppState {
    pub fn new(db: PgPool) -> Self {
        Self {
            db,
        }
    }

    pub fn db(&self) -> &PgPool {
        &self.db
    }
}
