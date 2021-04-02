use crate::api::search_products;
use crate::{
    api::{
        db::{
            delete_product, insert_portion, insert_product, list_portions, remove_portion,
            single_product,
        },
        ApiError, SearchQuery,
    },
    products::{ApiResult, Portion, Product, ProductSubmission, ResultList},
};
use actix_web::{delete, get, post, web, Responder};
use sqlx::SqlitePool;

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

    let result = ResultList {
        result: search_result,
    };

    Ok(result)
}

// TODO: List Products with pagination

#[get("/api/products/{id}")]
async fn get_single_product(pool: web::Data<SqlitePool>, path: web::Path<i32>) -> impl Responder {
    let search_result = match single_product(&pool, path.to_owned()).await {
        Ok(res) => res,
        Err(err) => {
            return Err(ApiError::InternalServer);
        }
    };

    let result = ResultList {
        result: search_result,
    };

    Ok(result)
}

#[post("/api/products")]
async fn post_product(pool: web::Data<SqlitePool>, product: web::Json<Product>) -> impl Responder {
    let new_id = match insert_product(&pool, product.0).await {
        Ok(res) => res,
        Err(err) => {
            return Err(ApiError::InternalServer);
        }
    };

    Ok(ApiResult::new(
        201,
        Some("CREATED".to_owned()),
        Some(new_id),
    ))
}

#[delete("/api/products/{id}")]
async fn delete_single_product(
    pool: web::Data<SqlitePool>,
    path: web::Path<i32>,
) -> impl Responder {
    match delete_product(&pool, path.to_owned()).await {
        Ok(res) => res,
        Err(err) => {
            return Err(ApiError::InternalServer);
        }
    };

    Ok(ApiResult::new(200, Some("DELETED".to_owned()), None))
}

// Portions
#[get("/api/products/{id}/portions")]
async fn get_product_portions(pool: web::Data<SqlitePool>, path: web::Path<i32>) -> impl Responder {
    let search_result = match list_portions(&pool, path.to_owned()).await {
        Ok(res) => res,
        Err(err) => {
            return Err(ApiError::InternalServer);
        }
    };

    let result = ResultList {
        result: search_result,
    };

    Ok(result)
}

// Add a portion

#[post("/api/products/portions")]
async fn post_portions(
    pool: web::Data<SqlitePool>,
    product: web::Json<Vec<Portion>>,
) -> impl Responder {
    let new_id = match insert_portion(&pool, product.0).await {
        Ok(res) => res,
        Err(err) => {
            return Err(ApiError::InternalServer);
        }
    };

    Ok(ApiResult::new(201, Some("CREATED".to_owned()), None))
}

// Delete a portion

#[delete("/api/products/{id}/portions/{name}")]
async fn delete_portion(
    pool: web::Data<SqlitePool>,
    path: web::Path<(i32, String)>,
) -> impl Responder {
    match remove_portion(&pool, path.0, &path.1).await {
        Ok(res) => res,
        Err(err) => {
            return Err(ApiError::InternalServer);
        }
    };

    Ok(ApiResult::new(200, Some("DELETED".to_owned()), None))
}
