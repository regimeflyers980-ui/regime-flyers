use crate::models::User;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use sqlx::PgPool;
use tower_sessions::Session;
use uuid::Uuid;

pub const SESSION_USER_ID: &str = "user_id";
pub const SESSION_ROLE: &str = "role";

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: Uuid,
    pub role: String,
}

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("failed to hash password")
        .to_string()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).ok();
    parsed_hash
        .map(|h| Argon2::default().verify_password(password.as_bytes(), &h).is_ok())
        .unwrap_or(false)
}

pub async fn perform_login(
    pool: &PgPool,
    session: &Session,
    email: &str,
    password: &str,
) -> Result<(), String> {
    let user: Option<User> = sqlx::query_as(
        "SELECT id, email, password_hash, role, full_name, created_at FROM users WHERE email = $1",
    )
    .bind(email)
    .fetch_optional(pool)
    .await
    .map_err(|e| e.to_string())?;

    let user = user.ok_or("Invalid email or password")?;

    if !verify_password(password, &user.password_hash) {
        return Err("Invalid email or password".into());
    }

    session.insert(SESSION_USER_ID, user.id).await.map_err(|e| e.to_string())?;
    session.insert(SESSION_ROLE, user.role.clone()).await.map_err(|e| e.to_string())?;

    Ok(())
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, _state)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let user_id: Option<Uuid> = session.get(SESSION_USER_ID).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let role: Option<String> = session.get(SESSION_ROLE).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        match (user_id, role) {
            (Some(id), Some(role)) => Ok(AuthUser { id, role }),
            _ => Err(StatusCode::UNAUTHORIZED),
        }
    }
}

pub fn require_admin(user: &AuthUser) -> Result<(), StatusCode> {
    if user.role == "ADMIN" { Ok(()) } else { Err(StatusCode::FORBIDDEN) }
}
