use dotenv::dotenv;
use sqlx::{Error, PgPool, postgres::PgPoolOptions};
use std::{env::var, time::Duration};

pub async fn init_db() -> Result<PgPool, Error> {
    dotenv().ok();

    let database_url: String = var("DATABASE_URL").expect("KEY DATABASE_URL must be set.");

    let pool: PgPool = PgPoolOptions::new()
        .max_connections(20)
        .min_connections(5)
        .max_lifetime(Duration::from_secs(1800))
        .idle_timeout(Duration::from_secs(600))
        .acquire_timeout(Duration::from_secs(10))
        .test_before_acquire(true)
        .connect(&database_url)
        .await?;

    Ok(pool)
}
