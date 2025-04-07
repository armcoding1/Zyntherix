use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Workout {
    id: i32,
    user_id: i32,
    workout_type: String,
    duration_minutes: i32,
    calories_burned: i32,
    notes: String,
    performed_at: chrono::NaiveDateTime,
}