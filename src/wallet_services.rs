// src/service/wallet_service.rs
use crate::{
    wallet_model::{CreateWalletRequest, Wallet, CurrencyBalance},
    wallet_repository::WalletRepository,
    errors::ApiError,
    update_pin_request::UpdatePinRequest,
};
use bigdecimal::BigDecimal;
use chrono::Utc;
use uuid::Uuid;
use serde_json::{json, Value}; 


pub async fn create_wallet(
    request: CreateWalletRequest,
    repo: &impl WalletRepository,
) -> Result<String, ApiError> {
    let wallet_id = Uuid::new_v4();

    // Create wallet with initialized balances
    let wallet = Wallet {
        id: Some(Uuid::new_v4()),
        user_id: request.user_id,
        password: None,
        balances: initialize_currencies(),
        created_on: Utc::now(),
        updated_on: Utc::now(),
    };

    // Save to database via repository
    repo.save_new_created_wallet(&wallet).await?;

    // Return success message
    Ok(json!({
        "status": "success",
        "message": "Wallet created successfully",
        "wallet_id": wallet_id
    })
    .to_string())
}


fn initialize_currencies() -> Vec<CurrencyBalance> {
    vec![
        CurrencyBalance {
            currency_code: "USD".to_string(),
            currency_symbol: "$".to_string(),
            balance: BigDecimal::from(0),
        },
        CurrencyBalance {
            currency_code: "EUR".to_string(),
            currency_symbol: "€".to_string(),
            balance: BigDecimal::from(0),
        },
        CurrencyBalance {
            currency_code: "NGN".to_string(),
            currency_symbol: "₦".to_string(),
            balance: BigDecimal::from(0),
        },
        CurrencyBalance {
            currency_code: "GBP".to_string(),
            currency_symbol: "£".to_string(),
            balance: BigDecimal::from(0),
        },
        CurrencyBalance {
            currency_code: "JPY".to_string(),
            currency_symbol: "¥".to_string(),
            balance: BigDecimal::from(0),
        },
        CurrencyBalance {
            currency_code: "AUD".to_string(),
            currency_symbol: "$".to_string(),
            balance: BigDecimal::from(0),
        },
        CurrencyBalance {
            currency_code: "CAD".to_string(),
            currency_symbol: "$".to_string(),
            balance: BigDecimal::from(0),
        },
        CurrencyBalance {
            currency_code: "CHF".to_string(),
            currency_symbol: "CHF".to_string(),
            balance: BigDecimal::from(0),
        },
        CurrencyBalance {
            currency_code: "CNY".to_string(),
            currency_symbol: "¥".to_string(),
            balance: BigDecimal::from(0),
        },
        CurrencyBalance {
            currency_code: "INR".to_string(),
            currency_symbol: "₹".to_string(),
            balance: BigDecimal::from(0),
        },
    ]
}

#[allow(dead_code)]
impl CurrencyBalance {
    pub fn new(currency_code: &str, currency_symbol: &str, balance: impl Into<BigDecimal>) -> Self {
        Self {
            currency_code: currency_code.to_string(),
            currency_symbol: currency_symbol.to_string(),
            balance: balance.into(),
        }
    }
}

pub async fn update_transfer_pin_service(
    user_id: i64,
    request: UpdatePinRequest,
    repo: &(dyn WalletRepository + Send + Sync),
) -> Result<Value, ApiError> {
    if request.pin.len() < 4 {
        return Err(ApiError::BadRequest("PIN must be at least 4 digits".into()));
    }

    repo.update_transfer_pin(user_id, &request.pin).await?;

    Ok(json!({
        "status": "success",
        "message": "Transfer PIN updated successfully"
    }))
}