use helpers::{
  hash, jwt,
  time::utc_now,
  uuid::{self, Alphabet},
};
use sea_orm::{IntoActiveModel, Set};
use serde_json::{json, Value};

use crate::{app::AppState, entity::prelude::*, error::AppError};

use super::model::UserRegisterResponseData;

pub async fn user_register(
  state: &AppState,
  nickname: String,
  email: String,
  password: String,
) -> Result<UserRegisterResponseData, AppError> {
  if state.repo.user().has_user(&email).await?.is_some() {
    return Err(AppError::UserExists);
  }
  let hashed = hash::bcrypt_custom(&password, 8, hash::Version::TwoA)?;
  let mut user = UserActiveModel {
    user_id: Set(uuid::uuid(&Alphabet::DEFAULT, 8)),
    nickname: Set(nickname),
    password: Set(hashed),
    email: Set(email),
    r#type: Set("normal".to_string()),
    avatar: Set("v2/avatars/default.png".to_string()),
    is_email_verified: Set(0),
    is_phone_verified: Set(0),
    created_at: Set(utc_now()),
    ..Default::default()
  };
  if state.repo.user().is_first_user().await? {
    user.r#type = Set("root".to_owned());
  }
  state.repo.user().create_user(user).await?;
  Ok(UserRegisterResponseData {})
}

pub async fn user_login(
  state: &AppState,
  email: String,
  password: String,
) -> Result<Value, AppError> {
  if let Some(user) = state.repo.user().get_user_by_email(&email).await? {
    let matched = hash::verify_bcrypt(&password, &user.password)?;
    if matched {
      let token = jwt::sign(user.email, &state.jwt_token, 2592000)?;
      Ok(json!({
        "token": token
      }))
    } else {
      Err(AppError::PasswordIncorrect)
    }
  } else {
    Err(AppError::UserNotFound)
  }
}

pub async fn get_login_user_info(state: &AppState, token: String) -> Result<Value, AppError> {
  let email = jwt::verify::<String>(&token, &state.jwt_token)?.claims.data;
  if let Some(user) = state.repo.user().get_user_by_email(&email).await? {
    Ok(json! ({
        "nickname": user.nickname,
        "email": user.email,
        "type": user.r#type,
    }))
  } else {
    Err(AppError::UserNotFound)
  }
}

pub async fn set_user_profile(
  state: &AppState,
  token: String,
  nickname: Option<String>,
  password: Option<String>,
) -> Result<bool, AppError> {
  let email = jwt::verify::<String>(&token, &state.jwt_token)?.claims.data;
  let mut active_user = state
    .repo
    .user()
    .get_user_by_email(&email)
    .await?
    .ok_or(AppError::UserNotFound)?
    .into_active_model();
  if let Some(nickname) = nickname {
    active_user.nickname = Set(nickname);
  }
  if let Some(password) = password {
    active_user.password = Set(hash::bcrypt(&password)?);
  }
  let res = state.repo.user().update_user(active_user).await;
  Ok(res.is_ok())
}

pub async fn set_user_type(
  state: &AppState,
  token: String,
  user_id: u32,
  r#type: String,
) -> Result<bool, AppError> {
  let email = jwt::verify::<String>(&token, &state.jwt_token)?.claims.data;
  if state.repo.user().is_admin_user(&email).await? {
    let mut active_user = state
      .repo
      .user()
      .get_user_by_id(user_id)
      .await?
      .ok_or(AppError::Unauthorized)?
      .into_active_model();

    if state.repo.user().is_root_user(user_id).await? {
      return Err(AppError::Forbidden);
    }
    active_user.r#type = Set(r#type);
    state.repo.user().update_user(active_user).await?;
    Ok(true)
  } else {
    Err(AppError::Forbidden)
  }
}
