use dotenv::dotenv;
use std::env;

mod api;
mod config;
mod nutrients;
mod products;

use crate::config::setup;
use sqlx::SqlitePool;

use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "info");

    env_logger::init();

    let db_path = match env::var("DATABASE_URL") {
        Ok(val) => val,
        Err(_e) => panic!("No DATABASE_URL specified"),
    };

    let pool = SqlitePool::connect(&db_path).await?;

    let db_configured = setup(&pool).await?;

    if !db_configured {
        panic!("Error while setting up database");
    }
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
