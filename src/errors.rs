use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde_json::json;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),         
    Unauthorized(String),       
    NotFound(String),          
    InternalServerError(String),
}

#[allow(dead_code)]
impl ApiError {
    pub fn bad_request(msg: &str) -> Self {
        ApiError::BadRequest(msg.to_string())
    }
    
    pub fn unauthorized(msg: &str) -> Self {
        ApiError::Unauthorized(msg.to_string())
    }
    
    pub fn not_found(msg: &str) -> Self {
        ApiError::NotFound(msg.to_string())
    }
    
    pub fn internal(msg: &str) -> Self {
        ApiError::InternalServerError(msg.to_string())
    }
}


impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            ApiError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            ApiError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            ApiError::InternalServerError(msg) => write!(f, "Internal Server Error: {}", msg),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let (status_code, error_type) = match self {
            ApiError::BadRequest(_msg) => (StatusCode::BAD_REQUEST, "Bad Request"),
            ApiError::Unauthorized(_msg) => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            ApiError::NotFound(_msg) => (StatusCode::NOT_FOUND, "Not Found"),
            ApiError::InternalServerError(_msg) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
        };

        HttpResponse::build(status_code).json(json!({
            "error": error_type,
            "message": self.to_string(),
            "status": "error",
            "status_code": status_code.as_u16()
        }))
    }

    // Override status code for actix-web to use
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        ApiError::InternalServerError(err.to_string())
    }
}