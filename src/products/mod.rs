mod base;
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};

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