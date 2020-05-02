use dotenv::dotenv;
use std::env;

mod api;
mod db;
mod file;
mod nutrients;
mod products;

use sqlx::SqlitePool;

use warp::Filter;

// use db::import_file;

use warp::http::StatusCode;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    println!("Hello, world!");
    dotenv().ok();

    std::env::set_var("RUST_LOG", "info");

    env_logger::init();

    let db_path = match env::var("DATABASE_URL") {
        Ok(val) => val,
        Err(e) => panic!("No DATABASE_URL specified"),
    };

    let pool = SqlitePool::new(&db_path).await?;

    // let product = search_products("Sains").await?;

    // println!("{:?}", product.len());

    // import_file(&pool).await?;
    let aa = api::routes(pool);

    // let cors = warp::cors();
    
    let cors = warp::cors()
    .allow_any_origin()
    .allow_header("content-type")
    .allow_methods(vec!["GET", "POST", "PUT", "DELETE"]);
    // let cors = warp::cors()
    // .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
    // .allow_origin("*")
    // .max_age(30);

    let routes = aa.with(warp::log("nutrition")).with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;

    Ok(())
}
