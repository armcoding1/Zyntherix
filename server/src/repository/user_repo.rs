use crate::models::user::{CreateUser, User};
use sqlx::{Error, PgPool, query, query_as};

pub async fn create_user(
    pool: &PgPool,
    email: &str,
    password_hash: &str,
) -> Result<CreateUser, Error> {
    query_as::<_, CreateUser>(
        "INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING email, password_hash",
    )
    .bind(email)
    .bind(password_hash)
    .fetch_one(pool)
    .await
}

pub async fn find_all(pool: &PgPool) -> Result<Vec<User>, Error> {
    query_as::<_, User>("SELECT * FROM users")
        .fetch_all(pool)
        .await
}

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<User, Error> {
    query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn delete_by_id(pool: &PgPool, id: i32) -> Result<(), Error> {
    query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn update_by_id(
    pool: &PgPool,
    id: i32,
    email: Option<&str>,
    password_hash: Option<&str>,
) -> Result<User, Error> {
    query_as::<_, User>("UPDATE users SET email = $1, password_hash = $2 WHERE id = $3 RETURNING id, email, password_hash, created_at")
        .bind(email)
        .bind(password_hash)
        .bind(id)
        .fetch_one(pool)
        .await
}
