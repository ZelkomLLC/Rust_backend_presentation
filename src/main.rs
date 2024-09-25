use actix_web::{web::Data, App, HttpServer};
use sqlx::PgPool;
use std::env;

mod actix_web_files;

use actix_web_files::configure_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_connection = env::var("DATABASE_CONNECTION").unwrap();
    let pool = PgPool::connect(&database_connection).await.unwrap();
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .configure(configure_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
