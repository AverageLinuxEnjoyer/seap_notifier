use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    Json,
};
use database_api::{full_subscription::FullSubscription, service::Service};
use serde_json::{json, Value};
use std::sync::Arc;

pub async fn update_subscription(
    sv: Extension<Arc<Service>>,
    Path(id): Path<u32>,
    Json(payload): Json<FullSubscription>,
) -> (StatusCode, Json<Value>) {
    let res = sv.subscriptions.update(id, payload).await;

    match res {
        Ok(sub) => (
            StatusCode::ACCEPTED,
            Json(serde_json::to_value(sub).unwrap()),
        ),
        Err(err) => (StatusCode::NOT_FOUND, Json(json!({"Err":err.to_string()}))),
    }
}
