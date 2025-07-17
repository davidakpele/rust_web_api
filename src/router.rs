// src/routes.rs
use actix_web::web;
use crate::wallet_controller::{get_wallet, create_wallet, update_withdrawal_pin, wallet_withdraw, deposit_wallet};
use crate::middleware::{Authentication, RoleMiddleware};

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    // Public auth routes (no authentication required)
    cfg.service(
        web::scope("/wallet")
            .service(get_wallet)
            .service(create_wallet)
    );

    // Protected user routes (require authentication)
    cfg.service(
        web::scope("/action")
            .wrap(Authentication)
            .wrap(RoleMiddleware::new(vec!["USER", "ADMIN", "SUPER_USER"]))  // Requires valid JWT
            .service(deposit_wallet)
            .service(wallet_withdraw)
            .service(update_withdrawal_pin)
    );

    // Default route handler
    cfg.service(
        web::resource("/")
            .route(web::get().to(|| async {
                actix_web::HttpResponse::Unauthorized().json(serde_json::json!({
                    "error": "Access Denied",
                    "status": "error",
                    "title": "Authentication Error",
                    "message": "Authorization Access",
                    "details": "Something went wrong with authentication",
                    "code": "generic_authentication_error"
                }))
            })),
    );

    // Handle undefined routes
    cfg.default_service(
        web::route().to(|| async {
            actix_web::HttpResponse::NotFound().json(serde_json::json!({
                "error": "Endpoint Not Found",
                "message": "Access Denied",
                "status": "error", 
                "title": "Authentication Error", 
                "details": "Something went wrong with authentication", 
                "code": "generic_authentication_error",
            }))
        })
    );
}