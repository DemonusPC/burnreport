use crate::{
    api::{db::import_file, search_products},
    nutrients::{Carbohydrates, Energy, Fat, Protein, Salt},
};
use crate::{
    api::{
        db::{
            delete_product, insert_portion, insert_product, list_portions, remove_portion,
            single_product,
        },
        ApiError,
    },
    products::{ApiResult, Portion, Product, ResultList},
};
use actix_multipart::Multipart;
use actix_web::{delete, get, post, web, Responder};
use futures::{StreamExt, TryStreamExt};
use sqlx::SqlitePool;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchQuery {
    pub p: String,
}

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
#[post("/api/products/csv")]
async fn post_product_batch(pool: web::Data<SqlitePool>, mut payload: Multipart) -> impl Responder {
    while let Ok(Some(mut field)) = payload.try_next().await {
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();

            let mut rdr = csv::Reader::from_reader(data.as_ref());

            let products: Vec<Product> = rdr
                .records()
                .map(|res| {
                    let record = res.unwrap();
                    let name = record.get(0).unwrap_or("");
                    let manufacturer = record.get(1).unwrap_or("");

                    let kcal = record.get(2).unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
                    let kj = record.get(3).unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
                    let carbs = record.get(4).unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
                    let fiber = record.get(5).unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
                    let sugar = record.get(6).unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
                    let added_sugar = record.get(7).unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
                    let starch = record.get(8).unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
                    let fat = record.get(9).unwrap_or("0.0").parse::<f64>().unwrap_or(0.0);
                    let saturated = record
                        .get(10)
                        .unwrap_or("0.0")
                        .parse::<f64>()
                        .unwrap_or(0.0);
                    let monounsat = record
                        .get(11)
                        .unwrap_or("0.0")
                        .parse::<f64>()
                        .unwrap_or(0.0);
                    let trans = record
                        .get(12)
                        .unwrap_or("0.0")
                        .parse::<f64>()
                        .unwrap_or(0.0);
                    let protein = record
                        .get(13)
                        .unwrap_or("0.0")
                        .parse::<f64>()
                        .unwrap_or(0.0);
                    let salt = record
                        .get(14)
                        .unwrap_or("0.0")
                        .parse::<f64>()
                        .unwrap_or(0.0);

                    return Product::new(
                        -1,
                        name.to_owned(),
                        manufacturer.to_owned(),
                        Energy::new(kcal, kj),
                        Carbohydrates::new(carbs, fiber, sugar, added_sugar, starch),
                        Fat::new(fat, saturated, monounsat, trans),
                        Protein::new(protein),
                        Salt::new(salt),
                    );
                })
                .collect();

            match import_file(&pool, &products).await {
                Ok(()) => {}
                Err(err) => {
                    return Err(ApiError::InternalServer);
                }
            }
        }
    }

    Ok(ApiResult::new(201, Some("CREATED".to_owned()), Some(0)))
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
