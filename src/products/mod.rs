mod base;
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};

use crate::api::ApiError;

pub use self::base::Product;
pub use self::base::ProductSubmission;
pub use self::base::Report;

pub use self::base::Portion;
use serde_derive::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct ResultList<T> {
    pub result: Vec<T>
}

impl Responder for ResultList<Product> {

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
    id: Option<i64>
}

impl ApiResult {
    pub fn new(code: u16, status: Option<String>, id: Option<i64>) -> Self {
        ApiResult {
            code,
            status,
            id
        }
    }
}

impl Responder for ApiResult {

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
        let body = serde_json::to_string(&self).unwrap();

        match self.code {
            201 => {
                HttpResponse::Created()
                .content_type("application/json")
                .body(body)
            }
            _ => {
                HttpResponse::Ok()
                .content_type("application/json")
                .body(body)
            }
        }
    }
}
