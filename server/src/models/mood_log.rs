use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct MoodLog {
    id: i32,
    user_id: i32,
    mood_level: i32,
    description: String,
    recorded_at: chrono::NaiveDateTime,
}