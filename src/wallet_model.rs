use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;
use bigdecimal::BigDecimal;

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Validate, Default)]
pub struct CurrencyBalance {
    #[validate(length(min = 3, max = 3))]
    pub currency_code: String,  // e.g., USD, EUR, NGN
    
    #[validate(length(min = 1, max = 5))]
    pub currency_symbol: String,  // e.g., $, €, ₦
    
    pub balance: BigDecimal,  
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, Validate)]
#[sqlx(type_name = "wallet")]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    
    #[validate(range(min = 1))]
    #[serde(rename = "userId")]
    pub user_id: i64,
    
    #[sqlx(json)]
    pub balances: Vec<CurrencyBalance>,
    
    #[serde(skip_serializing)]
    #[validate(length(min = 8))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>, 
    
    pub created_on: DateTime<Utc>,
    
    #[serde(skip_serializing)]
    pub updated_on: DateTime<Utc>,
}

#[allow(dead_code)]
// Implement Default manually for Wallet since it has required fields
impl Default for Wallet {
    fn default() -> Self {
        Self {
            id: None,
            user_id: 0,  // Will be validated when built
            balances: Vec::new(),
            password: None, 
            created_on: Utc::now(),
            updated_on: Utc::now(),
        }
    }
}

#[allow(dead_code)]
#[derive(Default)]
pub struct WalletBuilder {
    id: Option<Uuid>,
    user_id: Option<i64>,
    balances: Vec<CurrencyBalance>,
    password: Option<String>,
}

#[allow(dead_code)]
impl WalletBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn id(mut self, id: Uuid) -> Self {
        self.id = Some(id);
        self
    }
    
    pub fn user_id(mut self, user_id: i64) -> Self {
        self.user_id = Some(user_id);
        self
    }
    
    pub fn balances(mut self, balances: Vec<CurrencyBalance>) -> Self {
        self.balances = balances;
        self
    }
    
    pub fn password(mut self, password: String) -> Self {
        self.password = Some(password);
        self
    }
    
    pub fn build(self) -> Result<Wallet, &'static str> {
        Ok(Wallet {
            id: self.id,
            user_id: self.user_id.ok_or("user_id is required")?,
            balances: self.balances,
            password: self.password,
            created_on: Utc::now(),
            updated_on: Utc::now(),
        })
    }
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateWalletRequest {
    #[validate(range(min = 1))]
    pub user_id: i64,
    
    // #[validate(length(min = 8))]
    // pub password: String,
    
    // #[validate(length(min = 1))]
    // pub initial_balances: Vec<CurrencyBalance>,
}