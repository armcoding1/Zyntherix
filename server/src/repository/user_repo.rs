use crate::models::user::{CreateUser, User};
use sqlx::{Error, PgPool, query_as};

pub async fn create_user(
    pool: &PgPool,
    email: &str,
    password_hash: &str,
) -> Result<CreateUser, Error> {
    query_as(
        "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING email, password_hash",
    )
    .bind(email)
    .bind(password_hash)
    .fetch_one(pool)
    .await
}

pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<User, Error> {
    query_as("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_one(pool)
        .await
}
