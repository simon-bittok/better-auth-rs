use serde::Deserialize;

/// Server configuration for network binding and URL generation.
///
/// Contains the protocol, host, and port settings for the application server.
/// Used to generate bind addresses and public URLs.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ServerConfig {
    protocol: String,
    host: String,
    port: u16,
}

impl ServerConfig {
    /// Generates the full server URL with protocol.
    ///
    /// Combines protocol, host, and port into a complete URL string
    /// in the format `protocol://host:port`.
    ///
    /// ## Examples
    /// ```
    /// # use server::ServerConfig;
    /// let config = ServerConfig {
    ///     protocol: "http".to_string(),
    ///     host: "0.0.0.0".to_string(),
    ///     port: 8080,
    /// };
    /// assert_eq!(config.url(), "http://0.0.0.0:8080");
    /// ```
    #[must_use]
    pub fn url(&self) -> String {
        format!("{}://{}:{}", &self.protocol, &self.host, self.port)
    }

    /// Generates the server bind address without protocol.
    ///
    /// Combines host and port into an address string in the format
    /// `host:port`, to be used for binding to network sockets,such
    /// as [`tokio::net::TcpListener`]
    ///
    /// ## Examples
    /// ```
    /// # use server::ServerConfig;
    /// let config = ServerConfig {
    ///     protocol: "http".to_string(),
    ///     host: "127.0.0.1".to_string(),
    ///     port: 3000,
    /// };
    /// assert_eq!(config.address(), "127.0.0.1:3000");
    /// ```
    #[must_use]
    pub fn address(&self) -> String {
        format!("{}:{}", &self.host, self.port)
    }
}
