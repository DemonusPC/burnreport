use crate::{
    api::{
        db::{body_overview, insert_body_log_db, update_body_log_db},
        ApiError,
    },
    body::BodyLog,
    products::ApiResult,
};
use actix_web::{get, post, put, web, Responder};
use sqlx::SqlitePool;

#[get("/api/body/overview")]
async fn get_body_overview(pool: web::Data<SqlitePool>) -> impl Responder {
    let result = match body_overview(&pool).await {
        Ok(res) => res,
        Err(err) => {
            return Err(ApiError::InternalServer);
        }
    };

    Ok(result)
}

#[post("/api/body")]
async fn post_body_log(
    pool: web::Data<SqlitePool>,
    body_log: web::Json<BodyLog>,
) -> impl Responder {
    let new_id = match insert_body_log_db(&pool, body_log.0).await {
        Ok(res) => res,
        Err(err) => {
            return Err(ApiError::InternalServer);
        }
    };

    Ok(ApiResult::new(201, Some("CREATED".to_owned()), None))
}

#[put("/api/body")]
async fn put_body_log(pool: web::Data<SqlitePool>, body_log: web::Json<BodyLog>) -> impl Responder {
    let new_id = match update_body_log_db(&pool, body_log.0).await {
        Ok(res) => res,
        Err(err) => {
            return Err(ApiError::InternalServer);
        }
    };

    Ok(ApiResult::new(200, Some("UPDATED".to_owned()), None))
}
