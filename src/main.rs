mod config;
mod db;


use actix_web::{App, HttpServer, middleware::Logger};
use crate::db::init_db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let pool = init_db().await.expect("Failed to connect to DB");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(actix_web::web::Data::new(pool.clone()))
           
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
