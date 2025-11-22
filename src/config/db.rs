use serde::Deserialize;
use sqlx::{ConnectOptions, PgPool, migrate::Migrator, postgres::PgConnectOptions};
use tracing::log::LevelFilter;

use crate::config::ConfigResult;

/// Configuration for PostgreSQL database connections.
///
/// This struct holds all necessary connection parameters for establishing
/// a connection to a PostgreSQL database. It supports both URI-based and
/// options-based connection methods.
///
/// # Fields
///
/// The struct contains the following connection parameters:
/// - `uri`: Complete connection URI string
/// - `protocol`: Database protocol (typically "postgresql")
/// - `user`: Database username
/// - `password`: Database password
/// - `host`: Database host address
/// - `name`: Database name
/// - `port`: Database port number
///
/// # Examples
///
/// ```no_run
/// use betterauth::config::DatabaseConfig;
///
/// // Typically loaded from configuration file
/// let config = DatabaseConfig {
///     uri: "postgresql://user:pass@localhost:5432/mydb".to_string(),
///     protocol: "postgresql".to_string(),
///     user: "user".to_string(),
///     password: "pass".to_string(),
///     host: "localhost".to_string(),
///     port: 5432,
///     name: "mydb".into()
/// };
///
/// // Connect using options
/// let pool = config.connect_using_options().await;
/// ````
#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    uri: String,
    protocol: String,
    user: String,
    password: String,
    host: String,
    name: String,
    port: u16,
    truncate: bool,
    recreate: bool,
    auto_migrate: bool,
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

    #[must_use]
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Establishes a lazy PostgreSQL connection pool using individual connection options.
    ///
    /// This method constructs a connection using the individual configuration fields
    /// (host, username, password, database name, and port) rather than a connection URI.
    /// The connection pool is created lazily, meaning the actual database connection
    /// is not established until the first query is executed.
    ///
    /// Statement logging is enabled at the `Debug` level for all queries executed
    /// through this connection pool.
    ///
    /// # Returns
    ///
    /// Returns a [`PgPool`] that can be used to execute queries against the database.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use betterauth::config::DatabaseConfig;
    /// # async fn example_query(config: DatabaseConfig) -> Result<(), Box<dyn std::error::Error>> {
    /// let pool = config.connect_using_options().await;
    ///
    /// // The actual connection is established on first use
    /// sqlx::query("SELECT 1").execute(&pool).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// This method does not panic. However, subsequent operations on the returned
    /// pool may fail if the connection parameters are invalid.
    pub async fn connect_using_options(&self) -> PgPool {
        let mut options = PgConnectOptions::new()
            .host(&self.host)
            .username(&self.user)
            .password(&self.password)
            .database(&self.name)
            .port(self.port);

        options = options.log_statements(LevelFilter::Debug);

        PgPool::connect_lazy_with(options)
    }

    /// Establishes a lazy PostgreSQL connection pool using the connection URI.
    ///
    /// This method creates a connection pool using the full connection URI string
    /// stored in the configuration. The connection pool is created lazily, meaning
    /// the actual database connection is not established until the first query is executed.
    ///
    /// # Returns
    ///
    /// Returns `Ok(PgPool)` if the URI can be parsed successfully, or an error if
    /// the URI format is invalid.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The connection URI format is invalid
    /// - The URI cannot be parsed by sqlx
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use betterauth::config::DatabaseConfig;
    /// # async fn example_query(config: DatabaseConfig) -> Result<(), Box<dyn std::error::Error>> {
    /// let pool = config.connect_using_uri().await?;
    ///
    /// // The actual connection is established on first use
    /// sqlx::query("SELECT 1").execute(&pool).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn connect_using_uri(&self) -> ConfigResult<PgPool> {
        PgPool::connect_lazy(&self.uri).map_err(Into::into)
    }

    pub fn truncate(&self) -> bool {
        self.truncate
    }

    pub fn recreate(&self) -> bool {
        self.recreate
    }

    pub fn auto_migrate(&self) -> bool {
        self.auto_migrate
    }

    pub async fn init(&self) -> ConfigResult<()> {
        let pool = self.connect_using_options().await;
        let migrator = Migrator::new(std::path::Path::new("migrations")).await?;

        let migrations = migrator.iter().count() as i64;

        if self.recreate && self.auto_migrate {
            // truncate the db then migrate again
            migrator.undo(&pool, migrations).await?;
            migrator.run(&pool).await?;

            return Ok(());
        }

        if self.recreate {
            migrator.undo(&pool, migrations).await?;
        }

        if self.auto_migrate {
            migrator.run(&pool).await?;
        }

        Ok(())
    }
}
