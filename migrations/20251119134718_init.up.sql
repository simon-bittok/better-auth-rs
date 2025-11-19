-- Add up migration script here
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT (gen_random_uuid()),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255),
    name VARCHAR(255),
    email_verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_users_email ON users(email);

CREATE TABLE oauth_accounts (
    id UUID PRIMARY KEY DEFAULT (gen_random_uuid()),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider VARCHAR(50) NOT NULL,
    provider_user_id VARCHAR(255) NOT NULL,
    access_token TEXT,
    refresh_token TEXT,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL,
    UNIQUE(provider, provider_user_id)
);

CREATE INDEX idx_oauth_accounts_user_id ON oauth_accounts(user_id);
