//! config

use std::net::Ipv4Addr;

use serde::Deserialize;

use crate::error::AppError;

fn default_workers() -> usize {
  1
}

fn default_port() -> u16 {
  std::env::var("LEANCLOUD_APP_PORT")
    .unwrap_or_else(|_| "3000".to_string())
    .parse()
    .unwrap_or(3000)
}

fn default_ipqps() -> u64 {
  60
}

fn default_host() -> Ipv4Addr {
  Ipv4Addr::new(0, 0, 0, 0)
}

#[derive(Deserialize)]
pub struct EnvConfig {
  #[serde(default = "default_workers")]
  pub workers: usize,
  #[serde(default = "default_host")]
  pub host: Ipv4Addr,
  #[serde(default = "default_port")]
  pub port: u16,
  pub database_url: String,
  pub jwt_token: String,
  pub smtp_service: Option<String>,
  pub smtp_host: Option<String>,
  pub smtp_port: Option<u16>,
  pub smtp_user: Option<String>,
  pub smtp_pass: Option<String>,
  #[serde(default = "default_ipqps")]
  pub ipqps: u64,
}

impl EnvConfig {
  pub fn load_env() -> Result<EnvConfig, AppError> {
    dotenvy::dotenv_override().ok();
    envy::from_env().map_err(AppError::from)
  }
}
