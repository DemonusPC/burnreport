use dotenv::dotenv;
use log::error;
use routes::{api_routes, frontend, frontend_helper_routes};
use std::env;

mod config;
mod nutrients;
mod product;
mod report;
mod routes;
mod recipie;

use crate::config::setup;
use sqlx::SqlitePool;

use actix_files::Files;
use actix_web::{
    middleware,
    web::{self, Data},
    App, HttpServer,
};

#[actix_web::main]
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

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .configure(api_routes)
            .configure(frontend_helper_routes)
            .service(Files::new("/static", "./frontend/build/static/"))
            .default_service(web::get().to(frontend))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await;

    if server.is_err() {
        error!(
            "Failed creating the server due to error: {}",
            server.unwrap_err()
        );
    }

    Ok(())
}
