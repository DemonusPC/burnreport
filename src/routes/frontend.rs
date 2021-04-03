use actix_files::NamedFile;
use actix_web::{get, HttpRequest, HttpResponse, Responder};
use log::error;

pub async fn frontend(req: HttpRequest) -> impl Responder {
    let index = match NamedFile::open("./frontend/build/index.html") {
        Ok(v) => v,
        Err(err) => {
            error!("Could not open the index file with error: {}", err);

            return HttpResponse::InternalServerError().finish();
        }
    };
    //
    index.into_response(&req)
}

#[get("/static/favicon.ico")]
pub async fn favicon(req: HttpRequest) -> impl Responder {
    let index = match NamedFile::open("./frontend/build/favicon.ico") {
        Ok(v) => v,
        Err(err) => {
            error!("Could not open the favicon file with error: {}", err);

            return HttpResponse::InternalServerError().finish();
        }
    };
    //
    index.into_response(&req)
}
