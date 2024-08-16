use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

mod controllers;
mod models;
mod routes;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load environment variables from .env file
    let host = env::var("HOST").expect("HOST must be set");
    let port = env::var("PORT").expect("PORT must be set");

    HttpServer::new(|| {
        App::new()
            .configure(routes::init) // Initialize routes
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
