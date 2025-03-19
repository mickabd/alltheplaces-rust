mod brand;
mod poi;

use actix_web::{App, HttpServer, web::Data};
use brand::{get_brand_by_id, get_random_brands};
use dotenv::dotenv;
use poi::{get_poi_by_id, get_random_pois};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::env;

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    let db_url = env::var("DBURL").expect("DBURL must be set!");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Error building a connection pool");

    println!("Starting server...");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(get_poi_by_id)
            .service(get_random_pois)
            .service(get_brand_by_id)
            .service(get_random_brands)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
