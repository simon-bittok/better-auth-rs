use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    uri: String,
    protocol: String,
    user: String,
    password: String,
    host: String,
    name: String,
    port: u16,
}

impl DatabaseConfig {
    pub fn uri(&self) -> &str {
        &self.uri
    }

    pub fn protocol(&self) -> &str {
        &self.protocol
    }

    pub fn user(&self) -> &str {
        &self.user
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}
