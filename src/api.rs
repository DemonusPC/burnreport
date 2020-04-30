use crate::db::{one_single_product, search_products, single_product};
use crate::nutrients::TotalAble;
use crate::products::{Product, ProductSubmission, Report};

use sqlx::SqlitePool;

use serde_json::json;

use warp::http::StatusCode;
use warp::Filter;

use serde_derive::{Deserialize, Serialize};

use chrono::prelude::{DateTime, Utc};

pub fn todos(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // GET / -> index.html
    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("./frontend/build/index.html"));

    //  GET /...
    // e.g. /favicon.ico -> favicon.ico
    // e.g. /static/js/main.chunk.js -> /static/js/main.chunk.js
    let assets = warp::get().and(warp::fs::dir("./frontend/build"));

    search(pool.clone())
        .or(get_single_product(pool.clone()))
        .or(index)
        .or(assets)
        .or(post_report(pool.clone()))
}

fn with_db(
    db: SqlitePool,
) -> impl Filter<Extract = (SqlitePool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

#[derive(Debug, Serialize, Deserialize)]
struct Quer {
    p: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Id {
    id: String,
}

fn search(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("search")
        .and(warp::get())
        .and(with_db(pool))
        .and(warp::query::<Quer>())
        .and_then(test)
}

fn get_single_product(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("products" / i32)
        .and(warp::get())
        .and(with_db(pool))
        .and_then(get_single_product_handler)
}

fn post_report(
    pool: SqlitePool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("report")
        .and(warp::post())
        .and(with_db(pool))
        // Only accept bodies smaller than 16kb...
        // .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(process_report)
}

async fn test(pool: SqlitePool, query: Quer) -> Result<impl warp::Reply, warp::Rejection> {
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

async fn get_single_product_handler(
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

async fn process_report(
    pool: SqlitePool,
    report: Report,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut result: Vec<Product> = vec![];

    let mut total_kcal: f64 = 0.0;
    let mut total_carbs: f64 = 0.0;
    let mut total_fat: f64 = 0.0;
    let mut total_protein: f64 = 0.0;

    for v in &report.consumed {
        match one_single_product(&pool, v.id(), v.amount()).await {
            Ok(product) => {
                total_kcal += &product.energy().kcal();
                total_carbs += &product.carbohydrates().total();
                total_fat += &product.fat().total();
                total_protein += &product.protein().total();
                result.push(product);
            }
            Err(err) => println!("{:?}", err),
        }
    }

    let utc: DateTime<Utc> = Utc::now();

    let reply = json!({
        "timeDone": utc,
        "result": {
        "total" : {
            "kcal": total_kcal,
            "carbohydrates": total_carbs,
            "fat": total_fat,
            "protein": total_protein
        },
         "consumed": result,
        }
    });

    Ok(warp::reply::json(&reply))
}

// SELECT id, manufacturer, kcal, kj, carbohydrates, fiber, sugar, added_sugar, starch, fat, saturated, monounsaturated, trans, protein, salt FROM Food
