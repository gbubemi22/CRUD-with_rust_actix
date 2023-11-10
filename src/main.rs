mod routes;
mod models;
use actix_web::{App, HttpServer, middleware::Logger};
use sqlx::PgPool;
use dotenv::dotenv;
use actix_cors::Cors;
use actix_web::http::header;


use crate::routes::{health_route::health_checker_handler, config::config};


pub struct AppState {
    db: PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
   
    env_logger::init();

    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");


   

    let pool = match PgPool::connect(&database_url).await {
        Ok(pool) => {
            println!("Connection to the db is successful");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database {:?}", err);
            std::process::exit(1);
        }
    };
    println!("Server connected successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"]) // Corrected method name
            .allowed_header(header::CONTENT_TYPE) // Corrected header name
            .allowed_header(header::AUTHORIZATION) // Corrected header name
            .allowed_header(header::ACCEPT) // Corrected header name
            .supports_credentials();

        App::new()
            .app_data(AppState { db: pool.clone() })
            .service(health_checker_handler)
            .configure(config)
            
            .wrap(cors)
            .wrap(Logger::default()) // You can configure Logger here
    })
    .bind("127.0.0.1:8080") // Corrected binding
    .unwrap()
    .run()
    .await
}
