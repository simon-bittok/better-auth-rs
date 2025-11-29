use std::sync::Arc;

use axum::Router;

use crate::AppContext;

pub mod auth;

pub fn routes(ctx: &Arc<AppContext>) -> Router {
    Router::new().nest("/auth", auth::router(ctx))
}
