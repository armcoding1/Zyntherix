use serde::{Serialize, Deserialize};
use sqlx::FromRow;
#[derive(Serialize, Deserialize, FromRow)]
pub struct Finance {
    id: i32,
    user_id: i32,
    income: rust_decimal::Decimal,
    expenses: rust_decimal::Decimal,
    savings: rust_decimal::Decimal,
    recorded_at: chrono::NaiveDateTime,
}