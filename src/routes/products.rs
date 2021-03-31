use actix_web::{Responder, delete, get, post, put, web::{self, Query}};
use sqlx::{ SqlitePool };
use crate::{api::{ApiError, SearchQuery, db::{delete_product, insert_product, single_product}}, products::{ApiResult, Product, ProductSubmission, ResultList}};
use crate::api::search_products;

#[get("/api/search")]
async fn get_search_products(
    pool: web::Data<SqlitePool>,
    web::Query(search): web::Query<SearchQuery>,
) -> impl Responder {

    let search_result = match search_products(&pool, &search.p).await {
        Ok(res) => res,
        Err(err) => {
            return Err(ApiError::InternalServer);
        }
    };

    let result = ResultList { result: search_result };

    Ok(result)
}

// TODO: List Products with pagination

#[get("/api/products/{id}")]
async fn get_single_product(
    pool: web::Data<SqlitePool>,
    path: web::Path<i32>
) -> impl Responder {

    let search_result = match single_product(&pool, path.to_owned()).await {
        Ok(res) => res,
        Err(err) => {
            return Err(ApiError::InternalServer);
        }
    };

    let result = ResultList { result: search_result };

    Ok(result)
}

#[post("/api/products")]
async fn post_product(
    pool: web::Data<SqlitePool>,
    product: web::Json<Product>,
) -> impl Responder {

    let new_id = match insert_product(&pool, product.0).await {
        Ok(res) => res,
        Err(err) => {
            return Err(ApiError::InternalServer);
        }
    };


    Ok(ApiResult::new(201, Some("CREATED".to_owned()), Some(new_id)))
}

#[delete("/api/products/{id}")]
async fn delete_single_product(
    pool: web::Data<SqlitePool>,
    path: web::Path<i32>
) -> impl Responder {

    match delete_product(&pool, path.to_owned()).await {
        Ok(res) => res,
        Err(err) => {
            return Err(ApiError::InternalServer);
        }
    };

    Ok(ApiResult::new(200, Some("DELETED".to_owned()), None))
}