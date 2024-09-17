use actix_web::web::{get, post, ServiceConfig};

mod services;

pub fn configure_routes(cfg: &mut ServiceConfig) {
    cfg.route("/cars", get().to(self::services::cars::handler))
        .route("/rent", post().to(self::services::rent::handler));
}
