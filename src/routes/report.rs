use crate::{nutrients::Nutrients, product::amount_adjusted_product};
use actix_web::{post, web, Responder};
use chrono::{DateTime, Utc};
use log::error;
use serde_json::json;
use sqlx::SqlitePool;

use crate::product::{Product, Report};

#[post("/api/report")]
async fn post_report(pool: web::Data<SqlitePool>, report: web::Json<Report>) -> impl Responder {
    let mut result: Vec<Product> = vec![];

    let mut total = Nutrients::default();

    for v in &report.consumed {
        match amount_adjusted_product(&pool, v.id(), v.amount()).await {
            Ok(product) => {
                total = total + product.nutrients();
                result.push(product);
            }
            Err(err) => {
                error!(
                    "Failed to return amount adjusted product due to error: {}",
                    err
                );
                return web::HttpResponse::InternalServerError().finish();
            }
        }
    }

    let utc: DateTime<Utc> = Utc::now();

    let reply = json!({
        "timeDone": utc,
        "result": {
        "total" : total,
        "consumed": result,
        }
    });

    web::HttpResponse::Ok().json(reply)
}
