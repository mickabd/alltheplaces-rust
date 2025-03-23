use actix_web::{
    HttpResponse, Responder, get,
    web::{Data, Path},
};

use crate::{AppState, model::Brand};

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
