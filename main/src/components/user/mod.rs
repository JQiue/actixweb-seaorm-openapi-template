pub mod handler;
pub mod model;
mod service;

use utoipa_actix_web::service_config::ServiceConfig;

pub fn config(cfg: &mut ServiceConfig) {
  cfg.service(handler::user_register);
  cfg.service(handler::user_login);
  cfg.service(handler::set_user_type);
  cfg.service(handler::set_user_profile);
  cfg.service(handler::get_user_info);
}
