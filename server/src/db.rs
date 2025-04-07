use dotenv::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

pub async fn init_db() -> PgPool {
    dotenv().expect("Failed To Load Environment Variables");

    let database_url: String = env::var("DATABASE_URL").expect("KEY DATABASE_URL Must Be Set");

    PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed To Connect To Database")
}
