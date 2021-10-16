use crate::api::db::{export_file, import_file};
use crate::products::FlatProduct;
use crate::{
    api::{
        db::{
            delete_product, insert_portion, insert_product, list_portions, remove_portion,
            search_product_suggestions, single_product,
        },
        ApiError,
    },
    products::{ApiResult, Portion, Product, ResultList},
};
use actix_multipart::Multipart;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use futures::{StreamExt, TryStreamExt};
use log::error;
use serde_derive::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchQuery {
    pub p: String,
}

#[get("/api/search/suggestions")]
async fn get_search_products_suggestions(
    pool: web::Data<SqlitePool>,
    web::Query(search): web::Query<SearchQuery>,
) -> impl Responder {
    let search_result = match search_product_suggestions(&pool, &search.p).await {
        Ok(res) => res,
        Err(err) => {
            error!("Search product suggestions failed due to error: {}", err);
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
    let product = match single_product(&pool, path.to_owned()).await {
        Ok(res) => res,
        Err(err) => match err {
            sqlx::Error::RowNotFound => {
                return Err(ApiError::NotFound);
            }
            _ => {
                return Err(ApiError::InternalServer);
            }
        },
    };

    Ok(product)
}

#[post("/api/products")]
async fn post_product(pool: web::Data<SqlitePool>, product: web::Json<Product>) -> impl Responder {
    let new_id = match insert_product(&pool, product.0).await {
        Ok(res) => res,
        Err(err) => {
            error!("Error: {}", err);
            return Err(ApiError::InternalServer);
        }
    };

    Ok(ApiResult::new(
        201,
        Some("CREATED".to_owned()),
        Some(new_id),
    ))
}

#[post("/api/products/csv")]
async fn post_product_batch(pool: web::Data<SqlitePool>, mut payload: Multipart) -> impl Responder {
    while let Ok(Some(mut field)) = payload.try_next().await {
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();

            let mut rdr = csv::Reader::from_reader(data.as_ref());

            let products: Vec<FlatProduct> = rdr
                .deserialize()
                .map(|res| {
                    // TODO: This unwrap needs fixing
                    let product: FlatProduct = res.unwrap();
                    println!("{:?}", product);
                    product
                })
                .collect();

            match import_file(&pool, &products).await {
                Ok(()) => {
                    return Ok(ApiResult::new(
                        201,
                        Some("CREATED".to_owned()),
                        Option::None,
                    ))
                }
                Err(err) => {
                    error!(
                        "Could not complete importing the csv file due to error: {}",
                        err
                    );
                    return Err(ApiError::InternalServer);
                }
            }
        }
    }

    Ok(ApiResult::new(201, Some("CREATED".to_owned()), Some(0)))
}

fn to_csv(products: &[FlatProduct]) -> Result<String, Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(vec![]);

    for p in products {
        wtr.serialize(p)?;
    }

    wtr.flush()?;
    let data = String::from_utf8(wtr.into_inner()?)?;

    Ok(data)
}

#[get("/api/data/products/csv")]
async fn get_product_batch(pool: web::Data<SqlitePool>) -> HttpResponse {
    let all_products = match export_file(&pool).await {
        Ok(p) => p,
        Err(err) => {
            error!(
                "Could not export products from database due to error: {}",
                err
            );
            return HttpResponse::InternalServerError().finish();
        }
    };

    match to_csv(&all_products) {
        Ok(body) => HttpResponse::Ok()
            .content_type("text/csv")
            .insert_header(("Content-Disposition", "attachment;filename=products.csv"))
            .body(body),
        Err(err) => {
            error!("Could not generate the csv file due to error: {}", err);
            return HttpResponse::InternalServerError().finish();
        }
    }
}

#[delete("/api/products/{id}")]
async fn delete_single_product(
    pool: web::Data<SqlitePool>,
    path: web::Path<i32>,
) -> impl Responder {
    match delete_product(&pool, path.to_owned()).await {
        Ok(res) => res,
        Err(err) => {
            error!("Failed to delete the product due error: {}", err);
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
            error!("Could not list portions due to error: {}", err);
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
    match insert_portion(&pool, product.0).await {
        Ok(res) => res,
        Err(err) => {
            error!("Could not create a portion due to error: {}", err);
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
            error!("Could not delete a portion due to error: {}", err);
            return Err(ApiError::InternalServer);
        }
    };

    Ok(ApiResult::new(200, Some("DELETED".to_owned()), None))
}
