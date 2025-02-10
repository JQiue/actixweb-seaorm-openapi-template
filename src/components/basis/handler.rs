use actix_web::{get, HttpResponse};

use crate::components::basis::model::HealthCheckResponseData;

#[utoipa::path(tag = "Health", responses((status = OK, description = "成功", body = HealthCheckResponseData)))]
#[get("/health")]
async fn health_check() -> HttpResponse {
  HttpResponse::Ok().json(HealthCheckResponseData {
    status: "ok".to_string(),
  })
}
