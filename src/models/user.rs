use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use sqlx::{Encode, prelude::FromRow};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone, FromRow, Encode)]
pub struct UserModel {
    id: Uuid,
    email: String,
    name: String,
    password_hash: String,
    created_at: DateTime<FixedOffset>,
    updated_at: DateTime<FixedOffset>,
    email_verified: bool,
}

impl UserModel {
    pub fn register_user() {}
}
