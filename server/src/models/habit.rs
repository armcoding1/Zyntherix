use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Habit {
    id: i32,
    user_id: i32,
    name: String,
    habit_type: String,
    frequency: String,
    is_active: bool,
    created_at: chrono::NaiveDateTime,
}