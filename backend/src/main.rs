mod brand;
mod model;
mod poi;

use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use model::AppState;
use std::env;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    let db_url = env::var("DBURL").expect("DBURL must be set!");
    let app_state = AppState::init(db_url.clone()).await;
    let app_data = web::Data::new(app_state);

    println!("Starting server...");
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
