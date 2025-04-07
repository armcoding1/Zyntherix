use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Goal {
    id: i32,
    user_id: i32,
    title: String,
    description: String,
    deadline: chrono::NaiveDate,
    status: String,
    created_at: chrono::NaiveDateTime,
}