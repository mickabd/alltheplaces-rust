use crate::{AppState, model::Poi};
use actix_web::{
    HttpResponse, Responder, get,
    web::{Data, Path},
};

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

#[get("/poi/{brand_id}/count")]
async fn get_poi_count_for_brand_id(state: Data<AppState>, path: Path<i64>) -> impl Responder {
    let brand_id = path.into_inner();
    match sqlx::query_scalar::<_, i64>("SELECT COUNT(1) FROM poi WHERE brand_id = $1")
        .bind(brand_id)
        .fetch_one(&state.db)
        .await
    {
        Err(why) => {
            HttpResponse::NotFound().body(format!("Error while getting POI count: {}", why))
        }
        Ok(count) => HttpResponse::Ok().body(count.to_string()),
    }
}
