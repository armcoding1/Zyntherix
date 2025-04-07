use sqlx::PgPool;
use crate::models::user::User;
use crate::repository::user_repo;

pub async fn create_new_user(pool: &PgPool, email: &str, password_hash: &str) -> User {
    user_repo::create_user(pool, email, password_hash).await
}