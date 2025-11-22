use std::sync::Arc;

use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

use crate::{AppContext, config::Config, trace};

use super::Result;

pub struct App;

impl App {
    pub async fn run() -> Result<()> {
        let config = Config::load()?;

        config.logger().setup()?;
        config.database().init().await?;

        let ctx = Arc::new(AppContext::from_config(&config).await);

        let router = Router::new()
            .route("/", get(|| async { "Hello from axum" }))
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(trace::make_span_with)
                    .on_request(trace::on_request)
                    .on_response(trace::on_response)
                    .on_failure(trace::on_failure),
            )
            .with_state(ctx.clone());

        let listener = TcpListener::bind(config.server().address()).await?;

        tracing::info!("Listening on {}", config.server().url());

        axum::serve(listener, router).await.map_err(Into::into)
    }
}
