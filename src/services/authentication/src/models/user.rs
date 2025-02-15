use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: Option<String>,
    pub provider_id: Option<String>,
    pub auth_provider: String,
    pub facial_data: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: Option<String>,
    pub provider_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FacialDataRequest {
    pub user_id: i32,
    pub facial_data: String,
}

