use actix_web::{
  get, post, put,
  web::{Data, Json, Path, Query},
  HttpRequest, HttpResponse,
};

use crate::{
  app::AppState,
  components::user::{model::*, service},
  error::AppError,
  helpers::header::{extract_ip, extract_token},
  response::Response,
};

#[utoipa::path(
  tag = "User",
  params(
     ("lang" = String, Query, description = "语言设置")
  ),
  responses(
    (status = 200, body = Response<UserRegisterResponseData>),
  ),
)]
#[post("/user")]
pub async fn user_register(
  req: HttpRequest,
  state: Data<AppState>,
  query: Query<UserRegisterQuery>,
  body: Json<UserRegisterBody>,
) -> HttpResponse {
  let pass = state.rate_limiter.check_and_update(&extract_ip(&req), 1);
  if !pass {
    return HttpResponse::Ok().json(Response::<()>::error(AppError::FrequencyLimited, None));
  }
  let Query(UserRegisterQuery { lang }) = query;
  let Json(UserRegisterBody {
    nickname,
    email,
    password,
  }) = body;
  match service::user_register(&state, nickname, email, password).await {
    Ok(data) => HttpResponse::Ok().json(Response::success(Some(data), Some(&lang))),
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err, Some(&lang))),
  }
}

#[utoipa::path(tag = "User", responses((status = OK)))]
#[post("/token")]
pub async fn user_login(state: Data<AppState>, body: Json<UserLoginBody>) -> HttpResponse {
  let Json(UserLoginBody { email, password }) = body;
  match service::user_login(&state, email, password).await {
    Ok(data) => HttpResponse::Ok().json(Response::success(Some(data), None)),
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err, None)),
  }
}

#[utoipa::path(tag = "User", responses((status = OK)))]
#[get("/user")]
async fn get_user_info(req: HttpRequest, state: Data<AppState>) -> HttpResponse {
  match extract_token(&req) {
    Ok(token) => match service::get_login_user_info(&state, token).await {
      Ok(data) => HttpResponse::Ok().json(Response::success(Some(data), None)),
      Err(err) => HttpResponse::Ok().json(Response::<()>::error(err, None)),
    },
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err, None)),
  }
}

#[utoipa::path(tag = "User", responses((status = OK)))]
#[put("/user")]
pub async fn set_user_profile(
  req: HttpRequest,
  state: Data<AppState>,
  body: Json<SetUserProfileBody>,
) -> HttpResponse {
  let Json(SetUserProfileBody { nickname, password }) = body;
  match extract_token(&req) {
    Ok(token) => match service::set_user_profile(&state, token, nickname, password).await {
      Ok(_) => HttpResponse::Ok().json(Response::<()>::success(None, None)),
      Err(err) => HttpResponse::Ok().json(Response::<()>::error(err, None)),
    },
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err, None)),
  }
}

#[utoipa::path(tag = "User", responses((status = OK)))]
#[put("/user/{user_id}")]
pub async fn set_user_type(
  req: HttpRequest,
  state: Data<AppState>,
  path: Path<u32>,
  body: Json<SetUserTypeBody>,
) -> HttpResponse {
  let user_id = path.into_inner();
  let Json(SetUserTypeBody { r#type }) = body;
  match extract_token(&req) {
    Ok(token) => match service::set_user_type(&state, token, user_id, r#type).await {
      Ok(_) => HttpResponse::Ok().json(Response::<()>::success(None, None)),
      Err(err) => HttpResponse::Ok().json(Response::<()>::error(err, None)),
    },
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err, None)),
  }
}
