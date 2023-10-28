use shared::models::user;
use uuid::Uuid;

pub type Error = String;
pub type FamilyResult<T> = Result<T, Error>;

#[async_trait::async_trait]
pub trait FamilyUser: Send + Sync + 'static {
    async fn get(&self, id: Uuid) -> FamilyResult<user::User>;
    async fn create(&self, user: &user::User) -> FamilyResult<user::User>;
    async fn update(&self, user: &user::User) -> FamilyResult<user::User>;
}
