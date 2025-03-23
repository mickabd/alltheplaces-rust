mod brand;
mod model;
mod poi;

use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use log::{debug, info};
use model::AppState;
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
    dotenv().ok();
    debug!("Creating app state");
    let db_url = env::var("DBURL").expect("DBURL must be set!");
    let app_state = AppState::init(db_url.clone()).await;
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
