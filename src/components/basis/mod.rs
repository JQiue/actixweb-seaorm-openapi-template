mod handler;
mod model;
mod service;

use utoipa_actix_web::service_config::ServiceConfig;

pub fn config(cfg: &mut ServiceConfig) {
  cfg.service(handler::health_check);
}
