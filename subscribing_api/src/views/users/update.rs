use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    Json,
};
use database_api::{models::user::User, service::Service};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::views::query_params::Email;

pub async fn update_user(
    sv: Extension<Arc<Service>>,
    params: Path<Email>,
    Json(payload): Json<User>,
) -> (StatusCode, Json<Value>) {
    let res = sv.users.update(&params.email, payload).await;

    match res {
        Ok(sub) => (
            StatusCode::ACCEPTED,
            Json(serde_json::to_value(sub).unwrap()),
        ),
        Err(err) => (StatusCode::NOT_FOUND, Json(json!({"Err":err.to_string()}))),
    }
}
