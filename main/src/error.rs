use crate::locales::get_translation;

#[derive(Debug)]
pub enum AppError {
  Success,
  Error,
  Database,
  UserNotFound,
  UserExists,
  Unauthorized,
  Forbidden,
  InvalidToken,
  FrequencyLimited,
  PasswordIncorrect,
}

impl AppError {
  pub fn code(&self) -> i32 {
    match self {
      Self::Success => 0,
      Self::Error => 1000,
      Self::Database => 1001,
      Self::UserNotFound => 1002,
      Self::Unauthorized => 1003,
      Self::InvalidToken => 1004,
      Self::Forbidden => 1005,
      Self::FrequencyLimited => 1006,
      Self::UserExists => 1007,
      Self::PasswordIncorrect => 1008,
    }
  }
  pub fn message(&self, lang: &str) -> String {
    match self {
      Self::Success => get_translation(lang, "Success"),
      Self::Error => get_translation(lang, "Error"),
      Self::Database => get_translation(lang, "Database error"),
      Self::UserNotFound => get_translation(lang, "User not found"),
      Self::Unauthorized => get_translation(lang, "Unauthorized"),
      Self::InvalidToken => get_translation(lang, "Token invalid"),
      Self::Forbidden => get_translation(lang, "Forbidden"),
      Self::FrequencyLimited => get_translation(lang, "Frequency limited"),
      Self::UserExists => get_translation(lang, "User exists"),
      Self::PasswordIncorrect => get_translation(lang, "Password incorrect"),
    }
  }
}

impl From<sea_orm::DbErr> for AppError {
  fn from(err: sea_orm::DbErr) -> Self {
    tracing::error!("{:#?}", err);
    AppError::Database
  }
}

impl From<std::io::Error> for AppError {
  fn from(err: std::io::Error) -> Self {
    tracing::error!("{:#?}", err);
    AppError::Error
  }
}

impl From<envy::Error> for AppError {
  fn from(err: envy::Error) -> Self {
    tracing::error!("{:#?}", err);
    AppError::Error
  }
}

impl From<helpers::jwt::Error> for AppError {
  fn from(err: helpers::jwt::Error) -> Self {
    tracing::error!("{:#?}", err);
    AppError::InvalidToken
  }
}

impl From<helpers::hash::BcryptError> for AppError {
  fn from(err: helpers::hash::BcryptError) -> Self {
    tracing::error!("{:#?}", err);
    AppError::Error
  }
}

impl From<actix_web::http::header::ToStrError> for AppError {
  fn from(err: actix_web::http::header::ToStrError) -> Self {
    tracing::error!("{:#?}", err);
    AppError::Error
  }
}

// impl ResponseError for AppError {
//   fn status_code(&self) -> StatusCode {
//     match self {
//       AppError::Error => todo!(),
//       AppError::DatabaseError => todo!(),
//       AppError::UserNotFound => todo!(),
//       AppError::AuthorizationError => todo!(),
//     }
//   }
//   fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
//     match self {
//       AppError::DatabaseError => HttpResponse::Ok().json(Response::error(self.status_code(), lang)),
//       AppError::Error => HttpResponse::Ok().json(Response::error(code, lang)),
//       AppError::UserNotFound => HttpResponse::Ok().json(Response::error(code, lang)),
//       AppError::AuthorizationError => HttpResponse::Ok().json(Response::error(code, lang)),
//     }
//   }
// }

// impl std::fmt::Display for AppError {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     match self {
//       AppError::DatabaseError => write!(f, "Database error"),
//       AppError::Error => write!(f, "Internal server error"),
//       AppError::UserNotFound => write!(f, "User not found"),
//       AppError::AuthorizationError => write!(f, "Authorization error"),
//     }
//   }
// }
