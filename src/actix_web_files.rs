use actix_web::web::{resource, ServiceConfig};

mod services;

pub fn configure_routes(cfg: &mut ServiceConfig) {
    cfg.service(resource("/cars").get(self::services::cars::handler))
        .service(resource("/rent").post(self::services::rent::post::handler));
}
