use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    Error, HttpMessage,
};
use futures_util::future::{LocalBoxFuture, ready, Ready};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
    pub roles: Vec<String>,
    pub exp: usize,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: i32,
    pub roles: Vec<String>,
}

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware { service }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req.headers()
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|header| {
                let parts: Vec<&str> = header.split_whitespace().collect();
                if parts.len() == 2 && parts[0] == "Bearer" {
                    Some(parts[1].to_string())
                } else {
                    None
                }
            });

        let token = match token {
            Some(t) => t,
            None => {
                return Box::pin(async {
                    Err(ErrorUnauthorized(
                        serde_json::json!({
                            "error": "Access Denied",
                            "status": "error",
                            "title": "Authentication Error",
                            "message": "Authorization header missing or invalid",
                            "details": "Bearer token required",
                            "code": "missing_authorization_header"
                        })
                    ))
                });
            }
        };

        let secret = std::env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");
        let decoding_key = DecodingKey::from_secret(secret.as_ref());

        match decode::<Claims>(&token, &decoding_key, &Validation::new(Algorithm::HS256)) {
            Ok(token_data) => {
                let auth_user = AuthUser {
                    id: token_data.claims.id,
                    roles: token_data.claims.roles,
                };
                req.extensions_mut().insert(auth_user);
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            }
            Err(e) => {
                let masked_token = if token.len() > 8 {
                    format!("...{}", &token[token.len()-8..])
                } else {
                    "[redacted]".to_string()
                };
        
                let masked_secret = if secret.len() > 4 {
                    format!("...{}", &secret[secret.len()-4..])
                } else {
                    "[redacted]".to_string()
                };
        
                Box::pin(async move {
                    Err(ErrorUnauthorized(
                        serde_json::json!({
                            "error": "Access Denied",
                            "status": "error",
                            "title": "Authentication Error",
                            "message": "Invalid or expired token",
                            "details": {
                                "validation_error": e.to_string(),
                                "token_sample": masked_token,
                                "secret_sample": masked_secret,
                                "algorithm": "HS256"
                            },
                            "code": "invalid_token",
                            "debug_info": "Partial samples shown for debugging"
                        })
                    ))
                })
            }
        }
    }
}

pub struct RoleMiddleware {
    required_roles: Vec<String>,
}

impl RoleMiddleware {
    pub fn new(required_roles: Vec<&str>) -> Self {
        RoleMiddleware {
            required_roles: required_roles.into_iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RoleMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RoleMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RoleMiddlewareService {
            service,
            required_roles: self.required_roles.clone(),
        }))
    }
}

pub struct RoleMiddlewareService<S> {
    service: S,
    required_roles: Vec<String>,
}

impl<S, B> Service<ServiceRequest> for RoleMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_user = req.extensions().get::<AuthUser>().cloned();
        println!("Extractiong....");
        match auth_user {
            Some(user) => {
                let has_role = self.required_roles.iter()
                    .any(|required_role| user.roles.contains(required_role));

                if has_role || self.required_roles.is_empty() {
                    let fut = self.service.call(req);
                    Box::pin(async move { fut.await })
                } else {
                    Box::pin(async {
                        Err(ErrorUnauthorized(
                            serde_json::json!({
                                "error": "Access Denied",
                                "status": "error",
                                "title": "Authorization Error",
                                "message": "Insufficient permissions",
                                "details": "You are not authorized to access this endpoint",
                                "code": "insufficient_permissions"
                            })
                        ))
                    })
                }
            }
            
            None => Box::pin(async {
                Err(ErrorUnauthorized(
                    serde_json::json!({
                        "error": "Access Denied",
                        "status": "error",
                        "title": "Authentication Error",
                        "message": "User not authenticated",
                        "details": "Authentication required",
                        "code": "authentication_required"
                    })
                ))
            }),
            
        }
    }
}