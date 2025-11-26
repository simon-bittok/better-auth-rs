-- Add down migration script here

-- Drop Indices
DROP INDEX IF EXISTS idx_users_email;
DROP INDEX IF EXISTS idx_oauth_account_user_id;
DROP INDEX IF EXISTS idx_oauth_accounts_provider;

-- Drop Tables
DROP TABLE IF EXISTS oauth_accounts;
DROP TABLE IF EXISTS users;
