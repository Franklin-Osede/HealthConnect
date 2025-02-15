use actix_web::web;
use crate::handlers::auth::{register, login, login_with_google, add_facial_data};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/login/google", web::post().to(login_with_google))
            .route("/add-facial-data", web::post().to(add_facial_data))
    );
}
