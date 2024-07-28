extern crate dotenv;

use dotenv::dotenv;
use std::env;
use actix_web::{ web, App, HttpServer};

mod db;
mod common;

mod router;
mod repository;
mod service;
mod domain;
mod framework;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let valid_key = env::var("AUTH_KEY").unwrap();
    let server_host = env::var("SERVER_HOST").unwrap();
    let server_port = env::var("SERVER_PORT").unwrap();
    let server_location = server_host + ":" + &server_port;
    
    let db_url = env::var("DB_URL").unwrap();
    let db = db::init(&db_url).await.unwrap();

    HttpServer::new(move || {
        App::new()
        .wrap(framework::middleware::auth_middleware::AuthKeyMiddleware::new(valid_key.clone()))
        .app_data(web::Data::new(db.clone()))
        .configure(router::init)
    })
    .bind(&server_location)?
    .run()
    .await
}