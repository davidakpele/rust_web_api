// src/wallet_repository.rs
use crate::{
    wallet_model::Wallet,
    errors::ApiError
};
use sqlx::{PgPool};
use argon2::{password_hash::{self, rand_core}, Argon2, PasswordHasher, PasswordVerifier};
use rand_core::OsRng;
use password_hash::{SaltString, PasswordHash};

#[async_trait::async_trait]
pub trait WalletRepository {
    async fn save_new_created_wallet(&self, wallet: &Wallet) -> Result<(), ApiError>;
    async fn update_transfer_pin(&self, user_id: i64, pin: &str) -> Result<(), ApiError>;
}

#[derive(Clone)]
pub struct PostgresWalletRepository {
    pool: PgPool,
}

impl PostgresWalletRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl WalletRepository for PostgresWalletRepository {

    async fn save_new_created_wallet(&self, wallet: &Wallet) -> Result<(), ApiError> {
        let mut tx = self.pool.begin().await.map_err(|e| {
            ApiError::InternalServerError(format!("Failed to begin transaction: {}", e))
        })?;

        // Check if wallet already exists for the user
        let existing_wallet = sqlx::query!(
            r#"SELECT id FROM wallet WHERE user_id = $1"#,
            wallet.user_id
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| {
            ApiError::InternalServerError(format!("Failed to check existing wallet: {}", e))
        })?;

        if existing_wallet.is_some() {
            return Err(ApiError::BadRequest(format!(
                "Wallet already exists for user ID: {}",
                wallet.user_id
            )));
        }

        // Insert wallet
        sqlx::query!(
            r#"
            INSERT INTO wallet (id, user_id, password)
            VALUES ($1, $2, $3)
            "#,
            wallet.id,
            wallet.user_id,
            wallet.password
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            ApiError::InternalServerError(format!("Failed to insert wallet: {}", e))
        })?;

        // Insert balances
        for balance in &wallet.balances {
            sqlx::query!(
                r#"
                INSERT INTO wallet_balances 
                (wallet_id, currency_code, currency_symbol, balance)
                VALUES ($1, $2, $3, $4)
                "#,
                wallet.id,
                balance.currency_code,
                balance.currency_symbol,
                balance.balance
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                ApiError::InternalServerError(format!(
                    "Failed to insert wallet balance for currency {}: {}",
                    balance.currency_code, e
                ))
            })?;
        }

        tx.commit().await.map_err(|e| {
            ApiError::InternalServerError(format!("Failed to commit transaction: {}", e))
        })?;

        Ok(())
    }

    async fn update_transfer_pin(&self, user_id: i64, pin: &str) -> Result<(), ApiError> {
        let hashed_pin = hash_password(pin);
        let result = sqlx::query!(
            r#"
            UPDATE wallet SET password = $1, updated_on = NOW()
            WHERE user_id = $2
            "#,
            hashed_pin,
            user_id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(ApiError::NotFound("Wallet not found".into()));
        }

        Ok(())
    }
}

fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string();
    password_hash
}

#[allow(dead_code)]
fn verify_password(hash: &str, password: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).unwrap();
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
}
