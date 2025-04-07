use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Task {
    id: i32,
    user_id: i32,
    title: String,
    description: String,
    status: String,
    priority: i32,
    deadline: chrono::NaiveDateTime,
    created_at: chrono::NaiveDateTime,
}