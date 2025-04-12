use crate::models::user::{CreateUser, User};
use crate::repository::user_repo;
use bcrypt::{DEFAULT_COST, hash};
use sqlx::{Error, PgPool};

pub async fn create_user(pool: &PgPool, email: &str, password: &str) -> Result<CreateUser, Error> {
    let password_hash = match hash(password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => return Err(Error::Protocol("Password hashing failed.".to_string())),
    };

    user_repo::create_user(pool, email, &password_hash).await
}

pub async fn find_all(pool: &PgPool) -> Result<Vec<User>, Error> {
    user_repo::find_all(pool).await
}

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<User, Error> {
    user_repo::find_by_id(pool, id).await
}

pub async fn delete_by_id(pool: &PgPool, id: i32) -> Result<(), Error> {
    user_repo::delete_by_id(pool, id).await
}

pub async fn update_by_id(
    pool: &PgPool,
    id: i32,
    email: Option<&str>,
    password_hash: Option<&str>,
) -> Result<User, Error> {
    user_repo::update_by_id(pool, id, email, password_hash).await
}
