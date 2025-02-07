use std::fmt::Display;

use serde::Serialize;
use utoipa::ToSchema;

use crate::error::AppError;

#[derive(Debug, Serialize, ToSchema)]
pub struct Response<T> {
  pub code: i32,
  pub msg: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data: Option<T>,
}

impl<T> Response<T> {
  pub fn success(data: Option<T>, lang: Option<&str>) -> Self {
    Self {
      data,
      code: 0,
      msg: AppError::Success.message(lang.unwrap_or("en")),
    }
  }

  pub fn error(error: AppError, lang: Option<&str>) -> Self {
    Self {
      data: None,
      code: error.code(),
      msg: error.message(lang.unwrap_or("en")),
    }
  }
}

impl<T> Display for Response<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, r#"{{ "code": {}, "msg": "{}" }}"#, self.code, self.msg)
  }
}
