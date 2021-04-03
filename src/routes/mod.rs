use actix_web::web;
mod body;
mod frontend;
mod products;
mod report;

pub use self::frontend::frontend;

pub fn frontend_helper_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(frontend::favicon);
}

pub fn api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(products::get_search_products);
    cfg.service(products::get_single_product);
    cfg.service(products::post_product);
    cfg.service(products::post_product_batch);
    cfg.service(products::delete_single_product);
    cfg.service(products::get_product_portions);
    cfg.service(products::post_portions);
    cfg.service(products::delete_portion);
    cfg.service(body::get_body_overview);
    cfg.service(body::post_body_log);
    cfg.service(body::put_body_log);
    cfg.service(report::post_report);
}
