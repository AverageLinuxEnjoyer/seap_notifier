use axum::{extract::Extension, http::StatusCode, Json};
use database_api::{models::user::User, service::Service};
use serde_json::{json, Value};
use std::sync::Arc;

pub async fn create_user(
    sv: Extension<Arc<Service>>,
    Json(payload): Json<User>,
) -> (StatusCode, Json<Value>) {
    let res = sv.users.create(&payload.email).await;

    match res {
        Ok(user) => (
            StatusCode::CREATED,
            Json(serde_json::to_value(user).unwrap()),
        ),
        Err(err) => (
            StatusCode::NOT_ACCEPTABLE,
            Json(json!({"Err":err.to_string()})),
        ),
    }
}
