use crate::model::{DatabaseState, Poi};
use actix_web::{
    HttpResponse, Responder, get,
    web::{Data, Path},
};
use log::{error, info, warn};

#[get("/poi/{id}")]
async fn get_poi_by_id(state: Data<DatabaseState>, path: Path<i32>) -> impl Responder {
    let id = path.into_inner();
    info!("Received request to get POI by id: {}", id);

    match sqlx::query_as::<_, Poi>("SELECT * FROM poi WHERE id = $1")
        .bind(id)
        .fetch_one(&state.poi_db)
        .await
    {
        Err(why) => {
            error!("Error while retrieving POI with id {}: {}", id, why);
            HttpResponse::NotFound().body(format!("No poi found with id: {}, error: {}", id, why))
        }
        Ok(poi) => {
            info!("Successfully retrieved POI with id: {}", id);
            HttpResponse::Ok().json(poi)
        }
    }
}

#[get("/poi/random/{count}")]
async fn get_random_pois(state: Data<DatabaseState>, path: Path<i64>) -> impl Responder {
    let limit = path.into_inner();
    let max_limit = 15;

    info!("Received request to get random POIs with limit: {}", limit);

    if limit > max_limit {
        warn!(
            "Requested limit {} exceeds max limit {}. Returning BadRequest.",
            limit, max_limit
        );
        return HttpResponse::BadRequest().body(format!("Limit must be less than {}", max_limit));
    }

    match sqlx::query_as::<_, Poi>("SELECT * FROM poi LIMIT $1")
        .bind(limit)
        .fetch_all(&state.poi_db)
        .await
    {
        Err(why) => {
            error!("Error while getting random POIs: {}", why);
            HttpResponse::NotFound().body(format!("Error while getting random POIs: {}", why))
        }
        Ok(pois) => {
            info!("Successfully retrieved {} random POIs.", pois.len());
            HttpResponse::Ok().json(pois)
        }
    }
}

#[get("/poi/{brand_id}/count")]
async fn get_poi_count_for_brand_id(state: Data<DatabaseState>, path: Path<i64>) -> impl Responder {
    let brand_id = path.into_inner();
    info!(
        "Received request to get POI count for brand_id: {}",
        brand_id
    );

    match sqlx::query_scalar::<_, i64>("SELECT COUNT(1) FROM poi WHERE brand_id = $1")
        .bind(brand_id)
        .fetch_one(&state.poi_db)
        .await
    {
        Err(why) => {
            error!(
                "Error while getting POI count for brand_id {}: {}",
                brand_id, why
            );
            HttpResponse::NotFound().body(format!("Error while getting POI count: {}", why))
        }
        Ok(count) => {
            info!(
                "Successfully retrieved POI count for brand_id {}: {}",
                brand_id, count
            );
            HttpResponse::Ok().body(count.to_string())
        }
    }
}
