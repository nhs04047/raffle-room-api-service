extern crate dotenv;

use dotenv::dotenv;
use std::env;
use actix_web::{ web, App, HttpServer};

mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let server_host = env::var("SERVER_HOST").unwrap();
    let server_port = env::var("SERVER_PORT").unwrap();
    let server_location = server_host + ":" + &server_port;
    
    let db_url = env::var("DB_URL").unwrap();
    let db = db::init(&db_url).await.unwrap();

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(db.clone()))
    })
    .bind(&server_location)?
    .run()
    .await
}