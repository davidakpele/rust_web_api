# Escrow Wallet & Transaction System (Rust)

This project implements a secure and scalable escrow wallet and transaction system using **Rust**, **PostgreSQL**, and **SQLx**. It supports multiple currencies and maintains transactional integrity while handling deposits into escrow wallets.

## 📌 Features

- Create and manage escrow wallets
- Record transactions with different currencies
- Automatically update wallet balances
- Support for currency-specific symbols
- Enum-based transaction statuses and reasons
- SQL-level validation with enums and constraints
- Clear and modular code using the **Rust** ecosystem

---

## ⚙️ Technologies Used

### 🦀 Rust Frameworks & Libraries

| Library         | Purpose                                         |
|----------------|-------------------------------------------------|
| [`sqlx`]        | Async, compile-time checked SQL queries         |
| [`serde`]       | Serialization and deserialization               |
| [`uuid`]        | Universally unique identifiers                  |
| [`chrono`]      | Time and date handling                          |
| [`bigdecimal`]  | Accurate representation of currency values      |
| [`validator`]   | Input validation using annotations              |
| [`tokio`]       | Asynchronous runtime                            |

> Rust Edition: **2021**

---
## 🔐 Auth & Role-Based Middleware for Axum
- ➡️ Reference the full implementation: “Axum Auth Middleware” text doc.
- ✅ How It Works
1. Jwt-based Auth Extraction
- The FromRequestParts impl for AuthUser:
  - Validates 'Bearer <token>' header
  - Uses jsonwebtoken::decode with HS256 and your JWT_SECRET_KEY
  - Inserts AuthUser { id, roles } into Request extensions
  - Returns a 401 Unauthorized JSON error for missing or invalid tokens

2. Role Enforcement
- The require_roles handler function:
  - Accepts a State<HashSet<String>> of required roles
  - Checks if AuthUser.roles intersects
  - Responds with 200 and forwards to handler if OK; otherwise 401 error JSON

## 🛠️ Database Schema

### 🗃 PostgreSQL Tables

#### `escrow_wallet`

| Column       | Type                  | Description                         |
|--------------|------------------------|-------------------------------------|
| `id`         | `BIGSERIAL` or `UUID`  | Primary Key                         |
| `created_on` | `TIMESTAMPTZ`          | Wallet creation timestamp           |
| `updated_on` | `TIMESTAMPTZ`          | Last update timestamp               |

#### `currency_balance`

| Column          | Type           | Description                         |
|------------------|----------------|-------------------------------------|
| `id`             | `SERIAL`       | Primary Key                         |
| `wallet_id`      | `BIGINT`       | FK to `escrow_wallet(id)`          |
| `currency_code`  | `VARCHAR(3)`   | ISO currency code (e.g., USD, EUR) |
| `currency_symbol`| `VARCHAR(5)`   | Currency symbol (e.g., $, €)       |
| `balance`        | `NUMERIC(20,2)`| Wallet balance                      |

#### `escrow_transaction`

| Column         | Type             | Description                         |
|----------------|------------------|-------------------------------------|
| `id`           | `BIGSERIAL` or `UUID` | Primary Key                    |
| `wallet_id`    | `BIGINT`         | FK to `escrow_wallet(id)`           |
| `amount`       | `NUMERIC(20,2)`  | Transaction amount                  |
| `currency_code`| `VARCHAR(3)`     | Transaction currency                |
| `status`       | `escrow_status`  | Enum: `pending`, `reverse`, `inprocess` |
| `reason`       | `escrow_reason`  | Enum: `failed`, `suspicious`, etc.  |
| `created_at`   | `TIMESTAMP`      | Transaction creation time           |
| `updated_on`   | `TIMESTAMPTZ`    | Last updated time                   |

---

## 💼 Business Logic

### 💸 `create_transaction(txn: EscrowTransaction)`

- Inserts a new transaction
- If the wallet exists:
  - Updates the balance for the transaction’s currency
- If the wallet doesn't exist:
  - Creates the wallet
  - Inserts the initial balance

### 💱 Currency Symbols Support

Currency symbol is automatically selected based on ISO currency code:

```rust
match code.to_uppercase().as_str() {
    "USD" => "$",
    "EUR" => "€",
    "GBP" => "£",
    "NGN" => "₦",
    ...
}
```

## ✅ Input Validation
- Using validator crate for:
  - Length checks on currency code & symbol
  - Balance must be a valid BigDecimal
  - Wallet and transaction IDs must match expected types (i64, Uuid, etc.)

  ## 🧪 Example Request Payload
```
  {
  "walletId": 3,
  "amount": "150.00",
  "currency": "USD",
  "userId": 12
  }
```
### 🚀 Running the Project
- 🧱 Prerequisites
   - Rust (stable)
   - PostgreSQL 14+
   - sqlx-cli (optional for migrations):

```
cargo install sqlx-cli
```

## 🔧 Setup
1. Clone the repo
```
git clone https://github.com/davidakpele/rust_web_api.git
cd rust_web_api
```
2. Set your DB URL in .env:
```
DATABASE_URL=postgres://user:password@localhost:5432/escrow_db
```
3. Run migrations (if using sqlx-cli)
```
sqlx migrate run
```
4. Build & run the app
```
cargo build
cargo run
```
## 🧪 Testing
- Tests can be added using:
   - #[tokio::test] for async tests
   - assert_eq!, sqlx::query! to validate DB operations
  <hr/>
## 📌 Future Improvements
- Add REST API endpoints / Websocket (using Axum, Actix-web, or Rocket)
- Full test coverage
- JWT-based authentication
- Support for deposits, withdrawals, and holds


# 📄 License
### MIT License

## 👨‍💻 Author
## David Ak.
### Backend Engineer | Rust & Distributed Systems


