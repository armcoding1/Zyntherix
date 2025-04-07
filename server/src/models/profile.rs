use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Profile {
    id: i32,
    user_id: i32,
    name: String,
    birthdate: chrono::NaiveDate,
    gender: String,
    created_at: chrono::NaiveDateTime,
}