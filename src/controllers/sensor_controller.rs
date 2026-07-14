use actix_web::{web, HttpRequest, HttpResponse};
use std::sync::Mutex;
use crate::services::{AppState, SensorService};
use crate::policy::pep::Pep;
use crate::errors::AppError;

fn session_id_from_req(req: &HttpRequest) -> Result<String, AppError> {
    req.headers()
        .get("X-Session-Id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .ok_or_else(|| AppError::Unauthorized("Missing X-Session-Id header (G2)".into()))
}

/// GET /sensor/{id}
pub async fn fetch_sensor(
    req:  HttpRequest,
    data: web::Data<Mutex<AppState>>,
    pep:  web::Data<Pep>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let session_id = session_id_from_req(&req)?;
    let sensor_id = path.into_inner();
    let state = data.lock().map_err(|_| AppError::Internal("Lock error".into()))?;
    let sensor = SensorService::fetch_sensor(&state, &session_id, &sensor_id, &pep)?;
    Ok(HttpResponse::Ok().json(sensor))
}
