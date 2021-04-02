use actix_web::web;
mod body;
mod products;
mod report;

pub fn api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(products::get_search_products);
    cfg.service(products::get_single_product);
    cfg.service(products::post_product);
    cfg.service(products::delete_single_product);
    cfg.service(products::get_product_portions);
    cfg.service(products::post_portions);
    cfg.service(products::delete_portion);
}
