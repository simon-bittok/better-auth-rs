use sqlx::PgPool;

use crate::config::Config;

/// Shared application state container.
///
/// This struct holds shared resources that need to be initialized once at
/// application startup and then shared across all request handlers. It serves
/// as the central state management structure for the web application.
///
/// # Architecture
///
/// [`AppContext`] will be wrapped inside a [`std::sync::Arc`] where it
/// is typically used with Axum's state management system, where
/// it's cloned for each request handler. The underlying logic is that since
/// it has been wrapped in an Arc this will making cloning cheap and efficient.
///
/// # Lifecycle
///
/// 1. **Initialization**: Created once at application startup via [`AppContext::from_config()`]
/// 2. **Sharing**: Passed to Axum router as shared state
/// 3. **Access**: Cloned and injected into each handler via `State<Arc<AppContext>>`
///
/// # Fields
///
/// - `config`: Application configuration loaded from files and environment variables
/// - `db`: PostgreSQL connection pool for database operations
///
/// # Examples
///
/// ## Application Setup
///
/// ```no_run
/// use std::sync::Arc;
///
/// use betterauth::{AppContext, Config};
/// use axum::{Router, routing::get};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Load configuration
///     let config = Config::load()?;
///     
///     // Initialize logging
///     config.logger().setup()?;
///     
///     // Create application context
///     let app_context = AppContext::from_config(&config).await;
///     
///     // Build router with shared state
///     let app = Router::new()
///         .route("/", get(handler))
///         .with_state(Arc::new(app_context));
///     
///     // Start server
///     let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
///     axum::serve(listener, app).await?;
///     
///     Ok(())
/// }
///
/// async fn handler(
///     State(ctx): State<Arc<AppContext>>,
/// ) -> &'static str {
///     // Access database pool
///     let pool = ctx.db();
///     // Access configuration
///     let config = ctx.config();
///     "Hello, World!"
/// }
/// ```
///
/// ## Using in Handlers
///
/// ```no_run
/// use axum::{extract::State, Json};
/// use serde_json::{json, Value};
/// use betterauth::AppContext;
///
/// async fn get_users(
///     State(ctx): State<Arc<AppContext>>,
/// ) -> Json<Value> {
///     let users = sqlx::query!("SELECT id, name FROM users")
///         .fetch_all(ctx.db())
///         .await
///         .unwrap();
///     
///     Json(json!({ "users": users }))
/// }
/// ```
#[derive(Clone)]
pub struct AppContext {
    config: Config,
    db: PgPool,
}

impl AppContext {
    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn db(&self) -> &PgPool {
        &self.db
    }

    pub async fn from_config(config: &Config) -> Self {
        let db = config.database().connect_using_options().await;

        Self {
            config: config.clone(),
            db,
        }
    }
}
