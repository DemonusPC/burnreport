use actix_web::{dev::HttpResponseBuilder, error, http::header, http::StatusCode, HttpResponse};
use derive_more::{Display, Error};
use log::error;
use serde::Serialize;

#[derive(Serialize, Debug, Display, Error)]
#[serde(tag = "error", content = "context")]
pub enum ApiError {
    #[display(fmt = "InternalServer")]
    InternalServer,
    #[display(fmt = "NotFound")]
    NotFound,
}

impl error::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponseBuilder::new(self.status_code())
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .body(body)
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::InternalServer => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

impl From<actix_web::Error> for ApiError {
    fn from(error: actix_web::Error) -> Self {
        error!("ApiError failed due to error: {}", error);
        ApiError::InternalServer
    }
}
