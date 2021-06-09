use crate::nutrients::TotalAble;
use crate::nutrients::Vitamins;
use actix_web::{post, web, Responder};
use chrono::{DateTime, Utc};
use serde_json::json;
use sqlx::SqlitePool;

use crate::{
    api::db::one_single_product,
    products::{Product, Report},
};

#[post("/api/report")]
async fn post_report(pool: web::Data<SqlitePool>, report: web::Json<Report>) -> impl Responder {
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

    let mut total_vitamins: Vitamins = Vitamins::default();

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

                match product.vitamins() {
                    Some(v) => {
                        total_vitamins = total_vitamins + v;
                    }
                    None => {}
                };

                result.push(product);
            }
            Err(err) => println!("{:?}", err),
        }
    }

    let utc: DateTime<Utc> = Utc::now();

    let reply = json!({
        "timeDone": utc,
        "result": {
        "total" : Product::new_from_raw_values(-1, "Total".to_owned(), "Total".to_owned(), total_kcal, total_kj, total_carbs, total_fiber, total_sugar, total_added_sugar, total_starch, total_fat, total_saturated, total_monounsaturated, total_trans, total_protein, total_salt, Some(total_vitamins)),
        "consumed": result,
        }
    });

    web::HttpResponse::Ok().json(reply)
}
