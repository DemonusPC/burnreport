use actix_web::{HttpRequest, HttpResponse, Responder};
use serde_derive::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteRow, Row, SqlitePool};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchSuggestion {
    id: i32,
    text: String,
    sub_text: Option<String>,
    entity: Option<String>,
}

impl SearchSuggestion {
    pub fn new(id: i32, text: String, sub_text: Option<String>, entity: Option<String>) -> Self {
        SearchSuggestion {
            id,
            text,
            sub_text,
            entity,
        }
    }
}

impl Responder for SearchSuggestion {
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
    }
}

pub async fn search_product_suggestions(
    pool: &SqlitePool,
    term: &str,
) -> Result<Vec<SearchSuggestion>, sqlx::Error> {
    // SELECT *  FROM Food WHERE name LIKE "%Spag%";
    let result = sqlx::query("SELECT id, name, unit FROM Products WHERE name LIKE $1 LIMIT 15")
        .bind(format!("%{}%", term))
        .map(|row: SqliteRow| {
            let id: i32 = row.get(0);
            let text: String = row.get(1);
            let sub_text: String = row.get(2);
            SearchSuggestion::new(id, text, Some(sub_text), Some("Product".to_owned()))
        })
        .fetch_all(pool)
        .await?;
    Ok(result)
}
