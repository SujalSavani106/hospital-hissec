use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use crate::services::{AppState, UserService};
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
pub struct AddUserRequest {
    pub username: String,
    pub password: String,
    pub ward_id:  String,
}

#[derive(Deserialize)]
pub struct AssignRoleRequest {
    pub role: String,
}

#[derive(Deserialize)]
pub struct ChangeWardRequest {
    pub new_ward_id: String,
}

/// POST /users
pub async fn add_user(
    req:  HttpRequest,
    data: web::Data<Mutex<AppState>>,
    pep:  web::Data<Pep>,
    body: web::Json<AddUserRequest>,
) -> Result<HttpResponse, AppError> {
    let session_id = session_id_from_req(&req)?;
    let mut state = data.lock().map_err(|_| AppError::Internal("Lock error".into()))?;
    let user = UserService::add_user(
        &mut state, &session_id, &body.username, &body.password, &body.ward_id, &pep,
    )?;
    Ok(HttpResponse::Created().json(user))
}

/// DELETE /users/{id}
pub async fn remove_user(
    req:     HttpRequest,
    data:    web::Data<Mutex<AppState>>,
    pep:     web::Data<Pep>,
    path:    web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let session_id = session_id_from_req(&req)?;
    let target_id = path.into_inner();
    let mut state = data.lock().map_err(|_| AppError::Internal("Lock error".into()))?;
    UserService::remove_user(&mut state, &session_id, &target_id, &pep)?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "message": "User removed" })))
}

/// PUT /users/{id}/role
pub async fn assign_role(
    req:  HttpRequest,
    data: web::Data<Mutex<AppState>>,
    pep:  web::Data<Pep>,
    path: web::Path<String>,
    body: web::Json<AssignRoleRequest>,
) -> Result<HttpResponse, AppError> {
    let session_id = session_id_from_req(&req)?;
    let target_id = path.into_inner();
    let mut state = data.lock().map_err(|_| AppError::Internal("Lock error".into()))?;
    UserService::assign_role(&mut state, &session_id, &target_id, &body.role, &pep)?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "message": "Role assigned" })))
}

/// PUT /users/{id}/ward
pub async fn change_ward(
    req:  HttpRequest,
    data: web::Data<Mutex<AppState>>,
    pep:  web::Data<Pep>,
    path: web::Path<String>,
    body: web::Json<ChangeWardRequest>,
) -> Result<HttpResponse, AppError> {
    let session_id = session_id_from_req(&req)?;
    let target_id = path.into_inner();
    let mut state = data.lock().map_err(|_| AppError::Internal("Lock error".into()))?;
    UserService::change_ward(&mut state, &session_id, &target_id, &body.new_ward_id, &pep)?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "message": "Ward updated" })))
}
