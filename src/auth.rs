use crate::{error::AppError, models::UserRole};
use actix_web::{
    dev::ServiceRequest,
    error::Error,
    http::header,
    web, HttpMessage, HttpRequest,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use time::{Duration, OffsetDateTime};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

const JWT_SECRET: &[u8] = b"your-secret-key-change-in-production";
const JWT_EXPIRATION_HOURS: i64 = 24;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub exp: i64,    // expiration time
    pub iat: i64,    // issued at
    pub role: UserRole,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
    pub role: UserRole,
}

pub async fn hash_password(password: &str) -> Result<String, AppError> {
    let hashed = hash(password, DEFAULT_COST)?;
    Ok(hashed)
}

pub async fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let is_valid = verify(password, hash)?;
    Ok(is_valid)
}

pub fn create_jwt(user_id: Uuid, role: UserRole) -> Result<String, AppError> {
    let now = OffsetDateTime::now_utc();
    let expires_at = now + Duration::hours(JWT_EXPIRATION_HOURS);

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expires_at.unix_timestamp(),
        iat: now.unix_timestamp(),
        role,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )?;

    Ok(token)
}

pub fn verify_jwt(token: &str) -> Result<Claims, AppError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

pub async fn get_current_user(
    req: &HttpRequest,
    pool: &PgPool,
) -> Result<AuthUser, AppError> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or_else(|| AppError::Authentication("Missing authorization header".to_string()))?;

    let auth_str = auth_header
        .to_str()
        .map_err(|_| AppError::Authentication("Invalid authorization header".to_string()))?;

    if !auth_str.starts_with("Bearer ") {
        return Err(AppError::Authentication(
            "Invalid authorization header format".to_string(),
        ));
    }

    let token = &auth_str[7..];
    let claims = verify_jwt(token)?;

    let user_id = Uuid::parse_str(&claims.sub)?;

    // Verify user still exists and is active
    let row = sqlx::query("SELECT * FROM users WHERE id = $1 AND is_active = true")
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

    let user = match row {
        Some(row) => crate::db::row_to_user(row)?,
        None => return Err(AppError::Authentication("User not found or inactive".to_string())),
    };

    Ok(AuthUser {
        id: user.id,
        email: user.email,
        role: user.role,
    })
}

pub async fn require_auth(
    req: ServiceRequest,
    pool: web::Data<PgPool>,
) -> Result<ServiceRequest, Error> {
    let auth_user = get_current_user(req.request(), &pool).await;

    match auth_user {
        Ok(user) => {
            req.extensions_mut().insert(user);
            Ok(req)
        }
        Err(_) => Err(actix_web::error::ErrorUnauthorized("Unauthorized")),
    }
}

pub async fn require_role(
    req: ServiceRequest,
    pool: web::Data<PgPool>,
    required_role: UserRole,
) -> Result<ServiceRequest, Error> {
    let auth_user = get_current_user(req.request(), &pool).await;

    match auth_user {
        Ok(user) => {
            if user.role == required_role || user.role == UserRole::Admin {
                req.extensions_mut().insert(user);
                Ok(req)
            } else {
                Err(actix_web::error::ErrorForbidden("Insufficient permissions"))
            }
        }
        Err(_) => Err(actix_web::error::ErrorUnauthorized("Unauthorized")),
    }
}

pub async fn require_admin(
    req: ServiceRequest,
    pool: web::Data<PgPool>,
) -> Result<ServiceRequest, Error> {
    require_role(req, pool, UserRole::Admin).await
}

pub async fn require_property_owner(
    req: ServiceRequest,
    pool: web::Data<PgPool>,
) -> Result<ServiceRequest, Error> {
    require_role(req, pool, UserRole::PropertyOwner).await
}

pub fn extract_auth_user(req: &HttpRequest) -> Result<AuthUser, AppError> {
    req.extensions()
        .get::<AuthUser>()
        .cloned()
        .ok_or_else(|| AppError::Authentication("User not found in request".to_string()))
}
