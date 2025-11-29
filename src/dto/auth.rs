use std::borrow::Cow;

use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct RegisterUser<'a> {
    #[validate(email(message = "Invalid email format"))]
    email: Cow<'a, str>,

    #[validate(length(min = 2, message = "Name must be at least 2 characters long"))]
    name: Cow<'a, str>,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    password: Cow<'a, str>,

    #[validate(must_match(other = "password", message = "Passwords do not match"))]
    confirm_password: Cow<'a, str>,
}

impl<'a> RegisterUser<'a> {
    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}

#[derive(Debug, Deserialize, Clone, Validate)]
pub struct LoginUser<'a> {
    #[validate(email(message = "Invalid email format"))]
    email: Cow<'a, str>,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    password: Cow<'a, str>,
}

impl<'a> LoginUser<'a> {
    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}
