mod brand;
mod model;
mod poi;

use actix_web::{App, HttpServer, web};
use log::{debug, info};
use model::DatabaseState;
use std::env;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    if env::var("RUST_LOG").is_err() {
        unsafe {
            env::set_var("RUST_LOG", "info");
        }
    }
    env_logger::init();
    debug!("Logger Initialized");
    debug!("Reading environment variables");
    debug!("Creating app state");
    let poi_db_url = env::var("POSTGRES_POI_DB_URL").expect("DBURL must be set!");
    let brand_db_url = env::var("POSTGRES_BRAND_DB_URL").expect("DBURL must be set!");
    let app_state = DatabaseState::init(&poi_db_url, &brand_db_url).await;
    let app_data = web::Data::new(app_state);
    info!("Starting server...");
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(poi::get_poi_by_id)
            .service(poi::get_random_pois)
            .service(poi::get_poi_count_for_brand_id)
            .service(brand::get_brand_by_id)
            .service(brand::get_random_brands)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
