use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use sqlx::{Encode, Executor, Postgres, prelude::FromRow};
use uuid::Uuid;

use crate::{
    dto::auth::RegisterUser,
    models::{ModelError, ModelResult},
};

#[derive(Debug, Deserialize, Serialize, Clone, FromRow, Encode)]
pub struct UserModel {
    id: Uuid,
    email: String,
    name: String,
    password_hash: Option<String>,
    avatar_url: Option<String>,
    created_at: DateTime<FixedOffset>,
    updated_at: DateTime<FixedOffset>,
    email_verified: bool,
    is_active: bool,
}

impl UserModel {
    #[tracing::instrument(
        name = "Registering a new user in the database",
        skip(db, params),
        fields(email = %params.email())
    )]
    pub async fn register_user<'e, C>(db: &C, params: &RegisterUser<'_>) -> ModelResult<Self>
    where
        for<'a> &'a C: Executor<'e, Database = Postgres>,
    {
        let password_hash = hash_password(params.password().trim())?;

        let new_user = sqlx::query_as::<_, Self>(
            r"
            INSERT INTO users (email, name, password_hash)
            VALUES ($1, $2, $3)
            RETURNING *
        ",
        )
        .bind(params.email().trim())
        .bind(params.name().trim())
        .bind(password_hash)
        .fetch_one(db)
        .await?;

        Ok(new_user)
    }

    pub async fn find_user_by_email<'e, C>(db: &C, email: &str) -> ModelResult<Option<Self>>
    where
        for<'a> &'a C: Executor<'e, Database = Postgres>,
    {
        let user = sqlx::query_as(
            r"
            SELECT * FROM users WHERE email = $1
        ",
        )
        .bind(email.trim())
        .fetch_optional(db)
        .await?;

        Ok(user)
    }

    pub fn verify_password(&self, password: &str) -> ModelResult<()> {
        let stored_hash = match &self.password_hash {
            Some(hash) => hash,
            None => return Err(ModelError::InvalidCredentials),
        };

        let parsed_hash = PasswordHash::new(&stored_hash)?;

        Argon2::default().verify_password(password.as_bytes(), &parsed_hash)?;

        Ok(())
    }
}

fn hash_password(password: &str) -> ModelResult<String> {
    let argon = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    Ok(argon
        .hash_password(password.as_bytes(), &salt)
        .map_err(ModelError::PasswordHash)?
        .to_string())
}

impl UserModel {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn avatar_url(&self) -> Option<&str> {
        self.avatar_url.as_deref()
    }

    pub fn created_at(&self) -> DateTime<FixedOffset> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<FixedOffset> {
        self.updated_at
    }

    pub fn is_email_verified(&self) -> bool {
        self.email_verified
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }
}
