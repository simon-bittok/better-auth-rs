use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ServerConfig {
    protocol: String,
    host: String,
    port: u16,
}

impl ServerConfig {
    pub fn url(&self) -> String {
        format!("{}://{}:{}", &self.protocol, &self.host, self.port)
    }

    pub fn address(&self) -> String {
        format!("{}:{}", &self.host, self.port)
    }
}
