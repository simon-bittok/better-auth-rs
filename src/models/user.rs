use argon2::{
    Argon2, PasswordHasher,
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
    pub async fn register_user<'e, C>(db: &C, params: RegisterUser<'_>) -> ModelResult<Self>
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
}

fn hash_password(password: &str) -> ModelResult<String> {
    let argon = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    Ok(argon
        .hash_password(password.as_bytes(), &salt)
        .map_err(ModelError::PasswordHash)?
        .to_string())
}
