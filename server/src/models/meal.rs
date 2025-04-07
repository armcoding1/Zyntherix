use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Meal {
    id: i32,
    user_id: i32,
    meal_type: String,
    description: String,
    calories: i32,
    recorded_at: chrono::NaiveDateTime,
}