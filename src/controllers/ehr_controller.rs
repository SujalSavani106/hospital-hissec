use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;
use std::sync::Mutex;
use crate::services::{AppState, EhrService};
use crate::policy::pep::Pep;
use crate::errors::AppError;

fn session_id_from_req(req: &HttpRequest) -> Result<String, AppError> {
    req.headers()
        .get("X-Session-Id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .ok_or_else(|| AppError::Unauthorized("Missing X-Session-Id header (G2)".into()))
}

#[derive(Deserialize)]
pub struct CreateEhrRequest {
    pub ward_id:    String,
    pub patient_id: Option<String>,
    pub notes:      Option<String>,
}

/// GET /ehr/{id}
pub async fn read_ehr(
    req:  HttpRequest,
    data: web::Data<Mutex<AppState>>,
    pep:  web::Data<Pep>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let session_id = session_id_from_req(&req)?;
    let ehr_id = path.into_inner();
    let state = data.lock().map_err(|_| AppError::Internal("Lock error".into()))?;
    let ehr = EhrService::read_ehr(&state, &session_id, &ehr_id, &pep)?;
    Ok(HttpResponse::Ok().json(ehr))
}

/// POST /ehr
pub async fn create_ehr(
    req:  HttpRequest,
    data: web::Data<Mutex<AppState>>,
    pep:  web::Data<Pep>,
    body: web::Json<CreateEhrRequest>,
) -> Result<HttpResponse, AppError> {
    let session_id = session_id_from_req(&req)?;
    let mut state = data.lock().map_err(|_| AppError::Internal("Lock error".into()))?;
    let ehr = EhrService::create_ehr(
        &mut state, &session_id, &body.ward_id,
        body.patient_id.clone(), body.notes.clone(), &pep,
    )?;
    Ok(HttpResponse::Created().json(ehr))
}

/// DELETE /ehr/{id}
pub async fn delete_ehr(
    req:  HttpRequest,
    data: web::Data<Mutex<AppState>>,
    pep:  web::Data<Pep>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let session_id = session_id_from_req(&req)?;
    let ehr_id = path.into_inner();
    let mut state = data.lock().map_err(|_| AppError::Internal("Lock error".into()))?;
    EhrService::delete_ehr(&mut state, &session_id, &ehr_id, &pep)?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "message": "EHR deleted" })))
}
