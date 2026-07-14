use actix_web::web;
use crate::controllers::{
    login, logout,
    add_user, remove_user, assign_role, change_ward,
    read_ehr, create_ehr, delete_ehr,
    fetch_sensor,
};

/// Register all API routes on the Actix service config.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Authentication
        .route("/auth/login",  web::post().to(login))
        .route("/auth/logout", web::post().to(logout))

        // User management
        .route("/users",               web::post().to(add_user))
        .route("/users/{id}",          web::delete().to(remove_user))
        .route("/users/{id}/role",     web::put().to(assign_role))
        .route("/users/{id}/ward",     web::put().to(change_ward))

        // EHR
        .route("/ehr",        web::post().to(create_ehr))
        .route("/ehr/{id}",   web::get().to(read_ehr))
        .route("/ehr/{id}",   web::delete().to(delete_ehr))

        // Sensors
        .route("/sensor/{id}", web::get().to(fetch_sensor));
}
