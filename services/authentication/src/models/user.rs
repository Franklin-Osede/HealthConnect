se serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub facial_data: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub facial_data: Option<String>,
}

impl User {
    pub async fn create(pool: &PgPool, user: CreateUserRequest) -> Result<User, sqlx::Error> {
        let password_hash = bcrypt::hash(user.password.as_bytes(), 12)
            .expect("Failed to hash password");

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, password_hash, facial_data)
            VALUES ($1, $2, $3)
            RETURNING id, username, password_hash, facial_data
            "#,
            user.username,
            password_hash,
            user.facial_data
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }