//! app
use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
  time::{Duration, Instant},
};

use crate::{
  api::modify_api,
  components::{
    basis,
    user::{self},
  },
  config::EnvConfig,
  error::AppError,
  repository::RepositoryManager,
};

use actix_cors::Cors;
use actix_web::{
  middleware,
  web::{self},
  App, HttpServer,
};
use sea_orm::Database;
use utoipa_actix_web::{service_config::ServiceConfig, AppExt};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Debug)]
pub struct RateLimiter {
  qps: u64,
  counter: Mutex<HashMap<String, (usize, Instant)>>,
}

impl RateLimiter {
  fn new(qps: u64) -> Self {
    RateLimiter {
      qps,
      counter: Mutex::new(HashMap::new()),
    }
  }
  pub fn check_and_update(&self, client_ip: &str, count: usize) -> bool {
    let mut counter = self.counter.lock().unwrap();
    counter.retain(|_, &mut (_, timestamp)| timestamp.elapsed() < Duration::from_secs(self.qps));
    match counter.get_mut(client_ip) {
      Some((cnt, timestamp)) => {
        if *cnt >= count {
          false
        } else {
          *cnt += 1;
          *timestamp = Instant::now();
          true
        }
      }
      None => {
        counter.insert(client_ip.to_string(), (1, Instant::now()));
        true
      }
    }
  }
}

#[derive(Debug, Clone)]
pub struct AppState {
  pub repo: RepositoryManager,
  pub rate_limiter: Arc<RateLimiter>,
  pub jwt_token: String,
}

pub fn config_app(cfg: &mut ServiceConfig) {
  cfg.configure(basis::config);
  cfg.configure(user::config);
}

pub async fn start() -> Result<(), AppError> {
  let EnvConfig {
    workers,
    host,
    port,
    database_url,
    jwt_token,
    ipqps,
    ..
  } = EnvConfig::load_env()?;
  let conn = Database::connect(database_url).await?;
  conn.ping().await?;
  let state = AppState {
    repo: RepositoryManager::new(conn),
    jwt_token,
    rate_limiter: Arc::new(RateLimiter::new(ipqps)),
  };
  HttpServer::new(move || {
    let (app, mut api) = App::new()
      .into_utoipa_app()
      .app_data(web::Data::new(state.clone()))
      .service(utoipa_actix_web::scope("/api/v1").configure(config_app))
      .split_for_parts();
    modify_api(&mut api);
    app
      .wrap(Cors::permissive())
      .wrap(middleware::Logger::default())
      .service(SwaggerUi::new("/swagger/{_:.*}").url("/api-docs/openapi.json", api))
  })
  .bind((host, port))?
  .workers(workers)
  .run()
  .await
  .map_err(AppError::from)
}
