use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, Eq, Clone, PartialEq, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Eq, Clone, PartialEq, FromRow)]
pub struct CreateUser {
    pub email: String,
    pub password_hash: String,
}
