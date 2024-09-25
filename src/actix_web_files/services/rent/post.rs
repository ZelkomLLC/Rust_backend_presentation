use crate::actix_web_files::services::Car;
use actix_web::{
    web::{Data, Json},
    HttpResponse, Responder,
};
use sqlx::{query, PgPool};

pub async fn handler(car: Json<Car>, pool: Data<PgPool>) -> impl Responder {
    match query("insert into Cars (model, age) values ($1, $2);")
        .bind(car.model.clone())
        .bind(car.age)
        .execute(pool.get_ref())
        .await
    {
        Ok(..) => HttpResponse::Ok().finish(),
        Err(..) => HttpResponse::InternalServerError().finish(),
    }
}
