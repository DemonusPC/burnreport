use actix_web::{Responder, delete, get, post, put, web::{self, Query}};
use sqlx::{Pool, Sqlite, SqlitePool};
use crate::{api::{ApiError, SearchQuery}, products::ResultList};
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