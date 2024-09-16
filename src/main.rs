use actix_web::{get, web::resource, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(resource("/").to(|| async { "Example server" })))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
