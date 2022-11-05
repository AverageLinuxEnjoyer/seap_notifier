use axum::{extract::Extension, http::StatusCode, Json};
use database_api::{models::user::User, service::Service};
use std::sync::Arc;

pub async fn create_user(
    sv: Extension<Arc<Service>>,
    Json(payload): Json<User>,
) -> (StatusCode, Json<Result<User, String>>) {
    let res = sv.users.create(&payload.email).await;

    match res {
        Ok(user) => (StatusCode::CREATED, Json(Ok(user))),
        Err(err) => (StatusCode::NOT_ACCEPTABLE, Json(Err(err.to_string()))),
    }
}
