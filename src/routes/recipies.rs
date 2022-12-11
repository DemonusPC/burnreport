use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use log::error;
use sqlx::SqlitePool;

use crate::product::ApiResult;
use crate::recipie::{RecipieCreateCommand, RecipieStore};

#[get("/api/recipies/{id}")]
async fn get_single_recipies(pool: web::Data<SqlitePool>, path: web::Path<i64>) -> impl Responder {
    let product = match RecipieStore::get_by_id(&pool, path.to_owned()).await {
        Ok(res) => res,
        Err(err) => match err {
            sqlx::Error::RowNotFound => {
                return HttpResponse::NotFound().finish();
            }
            _ => {
                return HttpResponse::InternalServerError().finish();
            }
        },
    };

    HttpResponse::Ok().json(product)
}

#[post("/api/recipies")]
async fn post_recipie(
    pool: web::Data<SqlitePool>,
    recipie: web::Json<RecipieCreateCommand>,
) -> impl Responder {
    let new_id = match RecipieStore::create(&pool, recipie.0).await {
        Ok(res) => res,
        Err(err) => {
            error!("Error: {}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Created().json(ApiResult::new(
        201,
        Some("CREATED".to_owned()),
        Some(new_id),
    ))
}

#[put("/api/recipies/{id}")]
async fn put_recipie(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
    recipie: web::Json<RecipieCreateCommand>,
) -> impl Responder {
    let id = path.to_owned();
    match RecipieStore::update(&pool, id, recipie.0).await {
        Ok(res) => res,
        Err(err) => {
            error!("Error: {}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Created().json(ApiResult::new(200, Some("UPDATED".to_owned()), Some(id)))
}

#[delete("/api/recipies/{id}")]
async fn delete_recipie(pool: web::Data<SqlitePool>, path: web::Path<i64>) -> impl Responder {
    match RecipieStore::delete(&pool, path.to_owned()).await {
        Ok(res) => res,
        Err(err) => {
            error!("Failed to delete a recipie due error: {}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok().json(ApiResult::new(200, Some("DELETED".to_owned()), None))
}
