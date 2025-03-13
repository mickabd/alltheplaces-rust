mod poi;
use actix_web::{
    App, HttpResponse, HttpServer, Responder, get,
    web::{Data, Path},
};
use dotenv::dotenv;
use poi::POI;
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
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[get("/poi/{id}")]
async fn get_poi_by_id(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, POI>("SELECT * FROM poi WHERE id = $1")
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
        Err(why) => {
            HttpResponse::NotFound().body(format!("No poi found with id: {}, error: {}", id, why))
        }
        Ok(poi) => HttpResponse::Ok().json(poi),
    }
}

#[get("/poi/random/{count}")]
async fn get_random_pois(state: Data<AppState>, path: Path<i64>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, POI>("SELECT * FROM poi LIMIT $1")
        .bind(id)
        .fetch_all(&state.db)
        .await
    {
        Err(why) => {
            HttpResponse::NotFound().body(format!("Error while getting random POIs: {}", why))
        }
        Ok(pois) => HttpResponse::Ok().json(pois),
    }
}
