use crate::report::{run_report, ReportRequest};
use actix_web::{post, web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use log::error;
use serde_json::json;
use sqlx::SqlitePool;

#[post("/api/report")]
async fn post_report(
    pool: web::Data<SqlitePool>,
    report: web::Json<ReportRequest>,
) -> impl Responder {
    let result = match run_report(&pool, report.0).await {
        Ok(report_result) => report_result,
        Err(e) => {
            error!("Report processing failed with error: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let utc: DateTime<Utc> = Utc::now();

    let reply = json!({
        "timeDone": utc,
        "result": result
    });

    HttpResponse::Ok().json(reply)
}
