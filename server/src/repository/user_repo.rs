use crate::models::user::User;
use sqlx::PgPool;

pub async fn create_user(pool: &PgPool, email: &str, password_hash: &str) -> User {
    sqlx::query_as::<_, User>(
        "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING *",
    )
    .bind(email)
    .bind(password_hash)
    .fetch_one(pool)
    .await
    .unwrap()
}
