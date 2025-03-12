use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let db_url = env::var("DBURL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    let row: (i64,) = sqlx::query_as("SELECT count(1) FROM poi LIMIT 1")
        .fetch_one(&pool)
        .await?;

    println!("row: {}", row.0);

    Ok(())
}
