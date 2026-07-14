use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use crate::services::{AppState, AuthService};
use crate::policy::pep::Pep;
use crate::errors::AppError;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub role: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub session_id: String,
    pub user_id: String,
    pub active_role: String,
    pub message: String,
}

#[derive(Deserialize)]
pub struct LogoutRequest {
    pub session_id: String,
}

/// POST /auth/login
pub async fn login(
    data: web::Data<Mutex<AppState>>,
    pep:  web::Data<Pep>,
    body: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    let mut state = data.lock().map_err(|_| AppError::Internal("Lock error".into()))?;
    let subject = AuthService::login(
        &mut state, &body.username, &body.password, &body.role, &pep,
    )?;
    Ok(HttpResponse::Ok().json(LoginResponse {
        session_id: subject.session_id.clone(),
        user_id: subject.user_id.clone(),
        active_role: subject.active_role.to_string(),
        message: "Login successful".into(),
    }))
}

/// POST /auth/logout
pub async fn logout(
    data: web::Data<Mutex<AppState>>,
    pep:  web::Data<Pep>,
    body: web::Json<LogoutRequest>,
) -> Result<HttpResponse, AppError> {
    let mut state = data.lock().map_err(|_| AppError::Internal("Lock error".into()))?;
    AuthService::logout(&mut state, &body.session_id, &pep)?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "message": "Logged out successfully" })))
}
