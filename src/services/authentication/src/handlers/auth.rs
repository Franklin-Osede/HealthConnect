use actix_web::{web, HttpResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

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
    let user = user_data.into_inner();
    let (password_hash, auth_provider, provider_id) = if let Some(password) = user.password {
        (Some(hash(password, DEFAULT_COST).unwrap()), "local".to_string(), None)
    } else if let Some(provider) = user.provider_id {
        (None, "google".to_string(), Some(provider))
    } else {
        return HttpResponse::BadRequest().body("Invalid registration data");
    };
    
    let result = sqlx::query!(
        r#"
        INSERT INTO users (username, email, password_hash, auth_provider, provider_id)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, username, email, auth_provider
        "#,
        user.username,
        user.email,
        password_hash,
        auth_provider,
        provider_id
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().body("Error creating user"),
    }
}

pub async fn add_facial_data(
    pool: web::Data<PgPool>,
    facial_data: web::Json<FacialDataRequest>,
) -> HttpResponse {
    let result = sqlx::query!(
        "UPDATE users SET facial_data = $1 WHERE id = $2 RETURNING id, username, email, auth_provider, facial_data",
        facial_data.facial_data,
        facial_data.user_id
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().body("Error updating facial data"),
    }
}

