pub mod cars;
pub mod rent;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, FromRow, Serialize)]
pub struct Car {
    pub model: String,
    pub age: i32,
}
