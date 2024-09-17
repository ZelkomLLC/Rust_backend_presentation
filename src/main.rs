use actix_web::{get, web::resource, App, HttpResponse, HttpServer, Responder};

mod actix_web_files;

use actix_web_files::configure_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(configure_routes))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
