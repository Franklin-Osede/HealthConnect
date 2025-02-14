use actix_web::{web, HttpResponse};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::models::User;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn register(
    pool: web::Data<PgPool>,
    user_data: web::Json<CreateUserRequest>,
) -> HttpResponse {
    match User::create(&pool, user_data.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn login(
    pool: web::Data<PgPool>,
    credentials: web::Json<LoginRequest>,
) -> HttpResponse {
    // Implementar la lógica de login aquí
    // Verificar credenciales
    // Generar JWT
    // Retornar token
}