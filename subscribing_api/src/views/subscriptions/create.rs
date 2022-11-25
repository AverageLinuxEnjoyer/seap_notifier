use axum::{extract::Extension, http::StatusCode, Json};
use database_api::{full_subscription::FullSubscription, service::Service};
use serde_json::{json, Value};
use std::sync::Arc;

pub async fn create_subscription(
    sv: Extension<Arc<Service>>,
    Json(payload): Json<FullSubscription>,
) -> (StatusCode, Json<Value>) {
    let res = sv.subscriptions.create(payload).await;

    match res {
        Ok(subscription) => (
            StatusCode::CREATED,
            Json(serde_json::to_value(subscription).unwrap()),
        ),
        Err(err) => (
            StatusCode::NOT_ACCEPTABLE,
            Json(json!({"Err":err.to_string()})),
        ),
    }
}
