-- migrations/YYYYMMDDHHMMSS_create_wallet_tables.up.sql
-- PostgreSQL version

-- Main wallet table
CREATE TABLE wallet (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id BIGINT NOT NULL UNIQUE,
    password TEXT NULL,
    created_on TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_on TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create update trigger function
CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_on = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Add trigger to wallet table
CREATE TRIGGER update_wallet_timestamp
BEFORE UPDATE ON wallet
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();

-- Wallet balances table
CREATE TABLE wallet_balances (
    id BIGSERIAL PRIMARY KEY,
    wallet_id UUID NOT NULL,
    currency_code CHAR(3) NOT NULL,
    currency_symbol VARCHAR(5) NOT NULL,
    balance DECIMAL(38, 2) NOT NULL DEFAULT 0.00 CHECK (balance >= 0),
    created_on TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_on TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (wallet_id) REFERENCES wallet(id) ON DELETE CASCADE,
    UNIQUE (wallet_id, currency_code)
);

-- Add trigger to balances table
CREATE TRIGGER update_balances_timestamp
BEFORE UPDATE ON wallet_balances
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();
