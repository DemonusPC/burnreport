mod base;
mod portion;
mod product;
mod search;

use actix_web::{HttpRequest, HttpResponse, Responder};

pub use self::base::ProductSubmission;
pub use self::base::Report;
pub use self::product::amount_adjusted_product;
pub use self::product::delete_product;
pub use self::product::export_file;
pub use self::product::import_file;
pub use self::product::insert_product;
pub use self::product::single_product;
pub use self::product::FlatProduct;
pub use self::product::Product;
pub use self::product::Unit;

pub use self::portion::insert_portion;
pub use self::portion::list_portions;
pub use self::portion::remove_portion;
pub use self::portion::Portion;
pub use self::search::search_product_suggestions;
use self::search::SearchSuggestion;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultList<T> {
    pub result: Vec<T>,
}

impl Responder for ResultList<Product> {
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
    }
}

impl Responder for ResultList<Portion> {
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
    }
}

impl Responder for ResultList<SearchSuggestion> {
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResult {
    code: u16,
    status: Option<String>,
    id: Option<i64>,
}

impl ApiResult {
    pub fn new(code: u16, status: Option<String>, id: Option<i64>) -> Self {
        ApiResult { code, status, id }
    }
}

impl Responder for ApiResult {
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
        let body = serde_json::to_string(&self).unwrap();

        match self.code {
            201 => HttpResponse::Created()
                .content_type("application/json")
                .body(body),
            _ => HttpResponse::Ok()
                .content_type("application/json")
                .body(body),
        }
    }
}
