use crate::actix_web_files::services::Car;
use actix_web::{
    web::{Data, Query},
    HttpResponse, Responder,
};
use sqlx::{query_as, PgPool};

pub async fn handler(params: Query<Car>, pool: Data<PgPool>) -> impl Responder {
    let expression = "select model, age from Cars where model = $1 and age = $2";
    match query_as(expression)
        .bind(params.model.clone())
        .bind(params.age)
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(rows) => {
            let cars: Vec<Car> = rows;
            HttpResponse::Ok().json(cars)
        }
        Err(..) => HttpResponse::InternalServerError().body("Database error"),
    }
}
