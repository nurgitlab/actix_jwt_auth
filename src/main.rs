mod auth;
mod erros;
mod handlers;
mod migrations;
mod models;
mod repositories;
use std::env;

use actix_web::{App, HttpServer, middleware::Logger, web::Data};

use sqlx::postgres::PgPoolOptions;

use crate::{auth::jwt::{create_jwt::create_jwt, decode_jwt::decode_jwt}, handlers::ping_pong_handler::get_ping_pong, migrations::apply_migrations::apply_migrations};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    unsafe {
        std::env::set_var("RUST_LOG", "debug");
        std::env::set_var("BACKTRACE", "1");
    }
    env_logger::init();

    dotenv::dotenv().ok();

    // Create DB pool
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("database_url: {database_url}");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    apply_migrations(&pool).await.expect("Failed to apply migrations");

    println!("-----");
    //eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VybmFtZSI6InRlc3RfdXNlciIsImV4cCI6MTc1NTM2NTU1N30.JrPuYJAov5JMRVdKFM1-QIIB4fbe0CKgWzghmsOICfA

    let jwt = create_jwt("test_user".to_string())
        .expect("Failed to create JWT");
    println!("JWT: {:?}", jwt);
    println!("-----");

    let decoded = decode_jwt(&jwt)
        .expect("Failed to decode JWT");
    println!("Decoded JWT: {:?}", decoded);
    println!("-----");


    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(Data::new(pool.clone()))
            .service(get_ping_pong)
            .configure(handlers::users_handler::users_routes)
            .configure(handlers::cookies_handler::cookie_routes)
            .configure(handlers::posts_handler::posts_routes)
    })
    .bind(("127.0.0.1", 3030))?
    .run()
    .await
}
