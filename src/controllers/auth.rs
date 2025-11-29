use std::sync::Arc;

use axum::{
    Json, Router, debug_handler, extract::State, http::StatusCode, response::IntoResponse,
    routing::post,
};

use crate::{
    AppContext, Result, dto::auth::RegisterUser, models::user::UserModel, validator::Validator,
};

#[debug_handler]
#[tracing::instrument(
    name = "Register a new user",
    skip(ctx, payload),
    fields(email = %payload.email())
)]
async fn register(
    State(ctx): State<Arc<AppContext>>,
    Json(payload): Json<RegisterUser<'static>>,
) -> Result<impl IntoResponse> {
    let validator = Validator::new(payload);
    let valid_payload = validator.validate()?;

    let user = UserModel::register_user(ctx.db(), valid_payload).await?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "message": format!("User {} registered successfully", user.email()),
        })),
    )
        .into_response())
}

pub fn router(ctx: &Arc<AppContext>) -> Router {
    Router::new()
        .route("/sign-up", post(register))
        .with_state(ctx.clone())
}
