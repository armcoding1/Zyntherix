use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct PhysicalData {
    id: i32,
    user_id: i32,
    height_cm: i32,
    weight_kg: rust_decimal::Decimal,
    body_fat_percent: rust_decimal::Decimal,
    recorded_at: chrono::NaiveDateTime,
}