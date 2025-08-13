use actix_web::{App, HttpServer, web, middleware::Logger};
use actix_cors::Cors;
use dotenvy::dotenv;
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use airbnb_backend::{db, routes};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "airbnb_backend=debug,actix_web=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Get database URL from environment
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // Create database connection pool
    let pool = db::create_pool(&database_url)
        .await
        .expect("Failed to create database pool");

    // Get server configuration
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    tracing::info!("Starting server at {}:{}", host, port);

    // Start HTTP server
    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::configure_routes)
    })
    .bind((host, port))?
    .run()
    .await
}
