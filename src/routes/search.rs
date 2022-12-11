use crate::product::ResultList;
use crate::search::SearchEntity;
use crate::search::SearchStore;
use actix_web::{get, web, HttpResponse, Responder};
use log::error;
use serde_derive::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchQuery {
    pub p: String,
    pub e: Option<SearchEntity>,
}

#[get("/api/v2/search")]
async fn get_search(
    pool: web::Data<SqlitePool>,
    web::Query(search): web::Query<SearchQuery>,
) -> impl Responder {
    let search_result = match search.e {
        Some(entity) => match SearchStore::search_by_entity(&pool, &search.p, entity).await {
            Ok(res) => res,
            Err(err) => {
                error!("Search product suggestions failed due to error: {}", err);

                return HttpResponse::InternalServerError().finish();
            }
        },
        None => match SearchStore::search(&pool, &search.p).await {
            Ok(res) => res,
            Err(err) => {
                error!("Search product suggestions failed due to error: {}", err);

                return HttpResponse::InternalServerError().finish();
            }
        },
    };

    let result = ResultList {
        result: search_result,
    };

    HttpResponse::Ok().json(result)
}
