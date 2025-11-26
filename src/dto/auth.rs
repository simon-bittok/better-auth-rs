use std::borrow::Cow;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct RegisterUser<'a> {
    email: Cow<'a, str>,
    name: Cow<'a, str>,
    password: Cow<'a, str>,
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
