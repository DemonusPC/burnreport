use actix_web::web;
mod frontend;
mod products;
mod recipies;
mod report;
mod search;

pub use self::frontend::frontend;

pub fn frontend_helper_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(frontend::favicon);
}

pub fn api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(products::get_search_products_suggestions);
    cfg.service(search::get_search);
    cfg.service(recipies::get_search_recipie_suggestions);
    cfg.service(products::get_single_product);
    cfg.service(products::post_product);
    cfg.service(products::post_product_batch);
    cfg.service(products::get_product_batch);
    cfg.service(products::delete_single_product);
    cfg.service(products::get_product_portions);
    cfg.service(products::post_portions);
    cfg.service(products::delete_portion);
    cfg.service(report::post_report);
    cfg.service(recipies::get_single_recipies);
    cfg.service(recipies::post_recipie);
    cfg.service(recipies::put_recipie);
    cfg.service(recipies::delete_recipie);
}
