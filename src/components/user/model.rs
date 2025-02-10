use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct UserRegisterQuery {
  pub lang: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UserRegisterBody {
  pub nickname: String,
  pub email: String,
  pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct UserRegisterResponseData;

#[derive(Deserialize, ToSchema)]
pub struct UserLoginBody {
  pub email: String,
  pub password: String,
}

#[derive(Deserialize, ToSchema)]
pub struct SetUserProfileBody {
  pub nickname: Option<String>,
  pub password: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct SetUserTypeBody {
  pub r#type: String,
}
