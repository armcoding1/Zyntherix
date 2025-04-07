use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct SleepLog {
    id: i32,
    user_id: i32,
    slept_at: chrono::NaiveDateTime,
    woke_up_at: chrono::NaiveDateTime,
    quality: i32,
    recorded_at: chrono::NaiveDateTime,
}