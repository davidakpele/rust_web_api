use std::sync::Arc;

use actix_web::{get,  post, put, web, HttpResponse, Responder};
use crate::update_pin_request::UpdatePinRequest;
use crate::wallet_repository::{PostgresWalletRepository, WalletRepository};
use crate::wallet_services;
use crate::{
    wallet_model::CreateWalletRequest,
    errors::ApiError,
};



#[post("/create")]
pub async fn create_wallet(
    request: web::Json<CreateWalletRequest>,
    repo: web::Data<PostgresWalletRepository>,  
) -> Result<impl Responder, ApiError> {
    // Validate input
    if request.user_id <= 0 {
        return Err(ApiError::bad_request("User ID must be positive"));
    }

    // Call service layer with repository reference
    let response = wallet_services::create_wallet(request.into_inner(), &**repo).await?;
    
    // Return JSON response
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(response))
}

#[put("/pin/update")]
pub async fn update_withdrawal_pin(
    req: web::Json<UpdatePinRequest>,
    repo: web::Data<Arc<dyn WalletRepository + Send + Sync>>,
    user_id: web::ReqData<String>,
) -> Result<HttpResponse, ApiError> {
    // Convert user_id from String to i64
    let user_id: i64 = user_id.parse().map_err(|_| ApiError::bad_request("Invalid user ID"))?;

    let response = wallet_services::update_transfer_pin_service(
        user_id,
        req.into_inner(),
        repo.get_ref().as_ref(),
    ).await?;

    Ok(HttpResponse::Ok().json(response))
}


#[put("/withdraw")]
pub async fn wallet_withdraw() -> impl Responder {
    HttpResponse::Ok().body("Wallet Withdraw")
}

#[post("/deposit")]
pub async fn deposit_wallet() -> impl Responder {
    HttpResponse::Ok().body("Wallet Deposit")
}

#[get("/")]
pub async fn get_wallet() -> impl Responder {
    HttpResponse::Ok().body("Get wallet")
}