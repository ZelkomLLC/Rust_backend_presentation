use actix_web::Responder;

pub async fn handler() -> impl Responder {
    "Hello rent"
}
