mod model;
use actix_web::{
    App, HttpResponse, HttpServer, Responder, get,
    web::{Data, Path},
};
use dotenv::dotenv;
use model::{Brand, Poi};
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

#[get("/poi/{id}")]
async fn get_poi_by_id(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, Poi>("SELECT * FROM poi WHERE id = $1")
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
    let limit = path.into_inner();
    let max_limit = 15;
    if limit > 15 {
        return HttpResponse::BadRequest().body(format!("Limit must be less than {}", max_limit));
    }
    match sqlx::query_as::<_, Poi>("SELECT * FROM poi LIMIT $1")
        .bind(limit)
        .fetch_all(&state.db)
        .await
    {
        Err(why) => {
            HttpResponse::NotFound().body(format!("Error while getting random POIs: {}", why))
        }
        Ok(pois) => HttpResponse::Ok().json(pois),
    }
}

#[get("/brand/{id}")]
async fn get_brand_by_id(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, Brand>("SELECT * FROM brand WHERE id = $1")
        .bind(id)
        .fetch_one(&state.db)
        .await
    {
        Err(why) => {
            HttpResponse::NotFound().body(format!("No poi found with id: {}, error: {}", id, why))
        }
        Ok(brand) => HttpResponse::Ok().json(brand),
    }
}

#[get("/brand/random/{count}")]
async fn get_random_brands(state: Data<AppState>, path: Path<i64>) -> impl Responder {
    let limit: i64 = path.into_inner();
    let max_limit = 15;
    if limit > 15 {
        return HttpResponse::BadRequest().body(format!("Limit must be less than {}", max_limit));
    }
    match sqlx::query_as::<_, Brand>("SELECT * FROM brand LIMIT $1")
        .bind(limit)
        .fetch_all(&state.db)
        .await
    {
        Err(why) => {
            HttpResponse::NotFound().body(format!("Error while getting random Brands: {}", why))
        }
        Ok(brands) => HttpResponse::Ok().json(brands),
    }
}
