use actix_web::{web::Query, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Car {
    pub model: String,
    pub age: u8,
}

pub async fn handler(params: Query<Car>) -> impl Responder {
    format!("Car's model {} and age {}", params.model, params.age)
}
