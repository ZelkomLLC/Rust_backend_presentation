use actix_web::{web::Json, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Car {
    pub model: String,
    pub age: u8,
}

pub async fn handler(car: Json<Car>) -> impl Responder {
    HttpResponse::Ok().json(car)
}
