-- Add down migration script here
DROP TABLE IF EXISTS "wallet_balances";

DROP TABLE IF EXISTS "wallet";

DROP EXTENSION IF EXISTS "uuid-ossp";