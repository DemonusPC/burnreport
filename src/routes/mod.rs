use actix_web::web;
mod body;
mod products;
mod report;

pub fn api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(products::get_search_products);
}


