use crate::api::db::delete_product_size;
use crate::api::db::insert_product_sizes;
use crate::api::db::delete_product;
use crate::api::db::import_file;
use crate::api::db::insert_product;
use crate::api::db::one_single_product;
use crate::api::db::search_products;
use crate::api::db::single_product;
use crate::api::db::list_product_sizes;
use crate::api::{SearchQuery};
use crate::nutrients::TotalAble;
use crate::products::{Product, Portion, Report};
use chrono::prelude::{DateTime, Utc};
use serde_json::json;
use sqlx::SqlitePool;
use warp::http::StatusCode;
use warp::multipart;

use bytes::BufMut;
use futures::{TryFutureExt, TryStreamExt};

// CSV
use crate::nutrients::{Carbohydrates, Energy, Fat, Protein, Salt};
use serde::Deserialize;
use std::error::Error;

pub async fn test(
    pool: SqlitePool,
    query: SearchQuery,
) -> Result<impl warp::Reply, warp::Rejection> {
    let c: Vec<i32> = vec![];
    let result = match search_products(&pool, &query.p).await {
        Ok(res) => {
            let cc = warp::reply::json(&res);
            warp::reply::with_status(cc, StatusCode::OK)
        }
        Err(_err) => {
            let r = warp::reply::json(&c);
            warp::reply::with_status(r, StatusCode::INTERNAL_SERVER_ERROR)
        }
    };

    Ok(result)
}

pub async fn get_single_product_handler(
    id: i32,
    pool: SqlitePool,
) -> Result<impl warp::Reply, warp::Rejection> {
    let c: Vec<i32> = vec![];
    let result = match single_product(&pool, id).await {
        Ok(res) => {
            let cc = warp::reply::json(&res);
            warp::reply::with_status(cc, StatusCode::OK)
        }
        Err(_err) => {
            let r = warp::reply::json(&c);
            warp::reply::with_status(r, StatusCode::INTERNAL_SERVER_ERROR)
        }
    };

    Ok(result)
}

pub async fn process_report(
    pool: SqlitePool,
    report: Report,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut result: Vec<Product> = vec![];

    // Energy
    let mut total_kcal: f64 = 0.0;
    let mut total_kj: f64 = 0.0;

    // Carbs
    let mut total_carbs: f64 = 0.0;
    let mut total_sugar: f64 = 0.0;
    let mut total_added_sugar: f64 = 0.0;
    let mut total_fiber: f64 = 0.0;
    let mut total_starch: f64 = 0.0;

    // Fat
    let mut total_fat: f64 = 0.0;
    let mut total_saturated: f64 = 0.0;
    let mut total_monounsaturated: f64 = 0.0;
    let mut total_trans: f64 = 0.0;

    // Protein
    let mut total_protein: f64 = 0.0;

    // Salt
    let mut total_salt: f64 = 0.0;

    for v in &report.consumed {
        match one_single_product(&pool, v.id(), v.amount()).await {
            Ok(product) => {
                total_kcal += &product.energy().kcal();
                total_kj += &product.energy().k_j();

                total_carbs += &product.carbohydrates().total();
                total_sugar += &product.carbohydrates().sugar();
                total_added_sugar += &product.carbohydrates().added_sugar();
                total_fiber += &product.carbohydrates().fiber();
                total_starch += &product.carbohydrates().starch();

                total_fat += &product.fat().total();
                total_saturated += &product.fat().saturated();
                total_monounsaturated += &product.fat().monounsaturated();
                total_trans += &product.fat().trans();

                total_protein += &product.protein().total();

                total_salt += &product.salt().total();

                result.push(product);
            }
            Err(err) => println!("{:?}", err),
        }
    }

    let utc: DateTime<Utc> = Utc::now();

    let reply = json!({
        "timeDone": utc,
        "result": {
        "total" : Product::new_from_raw_values(-1, "Total".to_owned(), "Total".to_owned(), total_kcal, total_kj, total_carbs, total_fiber, total_sugar, total_added_sugar, total_starch, total_fat, total_saturated, total_monounsaturated, total_trans, total_protein, total_salt),
        "consumed": result,
        }
    });

    Ok(warp::reply::json(&reply))
}

pub async fn insert_single_product_handler(
    pool: SqlitePool,
    product: Product,
) -> Result<impl warp::Reply, warp::Rejection> {
    match insert_product(&pool, product).await {
        Ok(res) => {
            let json = json!({
                "status": "CREATED",
                "id": res
            });
            let base_reply = warp::reply::json(&json);
            Ok(warp::reply::with_status(base_reply, StatusCode::OK))
        }
        Err(_err) => Err(warp::reject::reject()),
    }
}

pub async fn delete_single_product_handler(
    id: i32,
    pool: SqlitePool,
) -> Result<impl warp::Reply, warp::Rejection> {
    match delete_product(&pool, id).await {
        Ok(_res) => {
            let json = json!({
                "status": "DELETED",
                "id": id
            });

            let base_reply = warp::reply::json(&json);
            Ok(warp::reply::with_status(base_reply, StatusCode::OK))
        }
        Err(err) => {
            println!("{:?}", err);
            Err(warp::reject::reject())
        }
    }
}

#[derive(Debug, Deserialize)]
struct Record {
    pub name: String,
    pub manufacturer: String,
    pub kcal: f64,
    pub kj: f64,
    pub carbs: f64,
    pub fiber: f64,
    pub sugar: f64,
    pub added_sugar: f64,
    pub starch: f64,
    pub fat: f64,
    pub saturated: f64,
    pub monounsat: f64,
    pub trans: f64,
    pub protein: f64,
    pub salt: f64,
}

pub fn import_products(data: &[u8]) -> Result<Vec<Product>, Box<dyn Error>> {
    // let mut rdr = csv::Reader::from_path(path)?;
    let mut rdr = csv::ReaderBuilder::new().from_reader(data);

    let mut pass: Vec<Product> = vec![];

    for record in rdr.records() {
        let record = record?;
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

        pass.push(Product::new(
            -1,
            name.to_owned(),
            manufacturer.to_owned(),
            Energy::new(kcal, kj),
            Carbohydrates::new(carbs, fiber, sugar, added_sugar, starch),
            Fat::new(fat, saturated, monounsat, trans),
            Protein::new(protein),
            Salt::new(salt),
        ));
    }
    Ok(pass)
}

pub async fn products_csv(
    form: multipart::FormData,
    pool: SqlitePool,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Collect the fields into (name, value): (String, Vec<u8>)
    let part: Result<Vec<(String, Vec<u8>)>, warp::Rejection> = form
        .and_then(|part| {
            let name = part.name().to_string();
            let value = part.stream().try_fold(Vec::new(), |mut vec, data| {
                vec.put(data);
                async move { Ok(vec) }
            });
            value.map_ok(move |vec| (name, vec))
        })
        .try_collect()
        .await
        .map_err(|e| {
            panic!("multipart error: {:?}", e);
        });

    let c = match part {
        Ok(v) => v,
        Err(_err) => panic!("Failed on matching part"),
    };

    let data: &[u8] = &c[0].1;

    let result = match import_products(data) {
        Ok(v) => v,
        Err(_err) => panic!("Died on processing"),
    };

    let json = match import_file(&pool, &result).await {
        Ok(()) => json!({
            "status": "CREATED",
        }),
        Err(_err) => json!({
            "status": "FAILED",
        }),
    };
    // let string = String::from_utf8(data.to_vec());

    let base_reply = warp::reply::json(&json);
    Ok(warp::reply::with_status(base_reply, StatusCode::OK))
}



pub async fn get_product_sizes_handler(
    product_id: i32,
    pool: SqlitePool,
) -> Result<impl warp::Reply, warp::Rejection> {
    let c: Vec<i32> = vec![];
    let result = match list_product_sizes(&pool, product_id).await {
        Ok(res) => {
            let cc = warp::reply::json(&res);
            warp::reply::with_status(cc, StatusCode::OK)
        }
        Err(_err) => {
            let r = warp::reply::json(&c);
            warp::reply::with_status(r, StatusCode::INTERNAL_SERVER_ERROR)
        }
    };

    Ok(result)
}

pub async fn insert_product_sizes_handler(
    pool: SqlitePool,
    products: Vec<Portion>
) -> Result<impl warp::Reply, warp::Rejection> {
    let c: Vec<i32> = vec![];
    let result = match insert_product_sizes(&pool, products).await {
        Ok(res) => {
            let cc = warp::reply::json(&res);
            warp::reply::with_status(cc, StatusCode::OK)
        }
        Err(err) => {
            println!("{:?}", err);
            let r = warp::reply::json(&c);
            warp::reply::with_status(r, StatusCode::INTERNAL_SERVER_ERROR)
        }
    };

    Ok(result)
}

pub async fn delete_product_sizes_handler(
    id: i32,
    name: String,
    pool: SqlitePool,
) -> Result<impl warp::Reply, warp::Rejection> {
    match delete_product_size(&pool, id, &name).await {
        Ok(res) => {
            let json = json!({
                "status": "DELETED",
                "id": id,
                "rows": res
            });

            let base_reply = warp::reply::json(&json);
            Ok(warp::reply::with_status(base_reply, StatusCode::OK))
        }
        Err(err) => {
            println!("{:?}", err);
            Err(warp::reject::reject())
        }
    }
}
