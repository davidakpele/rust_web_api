mod db;
mod router;
mod wallet_model;
mod wallet_controller;
mod wallet_services;
mod wallet_repository;
mod config;
mod middleware;
mod errors;
mod update_pin_request;

use actix_web::{
    http::{header, StatusCode},
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    web, App, HttpServer, Result, dev,
};

use actix_cors::Cors;
use db::get_pg_pool;
use router::config_routes;
use crate::config::Config;
use crate::wallet_repository::PostgresWalletRepository;

fn add_error_header<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("Error"),
    );

    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment variables and config
    let config = Config::init();

    // Initialize PostgreSQL connection pool using config
    let pool = get_pg_pool(&config.database_url).await;
    
    // Create wallet repository
    let wallet_repo = PostgresWalletRepository::new(pool.clone());

    println!("🚀 Starting server at http://localhost:9088");

    HttpServer::new(move || {
        // CORS configuration
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors) 
            .wrap(
                ErrorHandlers::new()
                    .handler(StatusCode::INTERNAL_SERVER_ERROR, add_error_header),
            )
            // Register both the pool and repository
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(wallet_repo.clone()))
            .configure(config_routes)
    })
    .bind(("127.0.0.1", 9088))?
    .run()
    .await
}