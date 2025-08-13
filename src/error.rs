use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    #[error("Authorization error: {0}")]
    Authorization(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Internal server error: {0}")]
    Internal(String),
    
    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    
    #[error("Bcrypt error: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status_code, error_type, message) = match self {
            AppError::Database(_) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Database Error",
                "An internal database error occurred",
            ),
            AppError::Validation(msg) => (
                actix_web::http::StatusCode::BAD_REQUEST,
                "Validation Error",
                msg.as_str(),
            ),
            AppError::Authentication(msg) => (
                actix_web::http::StatusCode::UNAUTHORIZED,
                "Authentication Error",
                msg.as_str(),
            ),
            AppError::Authorization(msg) => (
                actix_web::http::StatusCode::FORBIDDEN,
                "Authorization Error",
                msg.as_str(),
            ),
            AppError::NotFound(msg) => (
                actix_web::http::StatusCode::NOT_FOUND,
                "Not Found",
                msg.as_str(),
            ),
            AppError::BadRequest(msg) => (
                actix_web::http::StatusCode::BAD_REQUEST,
                "Bad Request",
                msg.as_str(),
            ),
            AppError::Internal(msg) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
                msg.as_str(),
            ),
            AppError::Jwt(_) => (
                actix_web::http::StatusCode::UNAUTHORIZED,
                "JWT Error",
                "Invalid or expired token",
            ),
            AppError::Bcrypt(_) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Password Error",
                "Error processing password",
            ),
        };

        HttpResponse::build(status_code).json(ErrorResponse {
            error: error_type.to_string(),
            message: message.to_string(),
        })
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(errors: validator::ValidationErrors) -> Self {
        AppError::Validation(errors.to_string())
    }
}

impl From<uuid::Error> for AppError {
    fn from(error: uuid::Error) -> Self {
        AppError::Validation(error.to_string())
    }
}

impl From<time::error::Parse> for AppError {
    fn from(error: time::error::Parse) -> Self {
        AppError::Validation(error.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;
