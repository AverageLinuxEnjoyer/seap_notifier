use axum::{extract::Extension, http::StatusCode, Json};
use database_api::{full_subscription::FullSubscription, service::Service};
use std::sync::Arc;

pub async fn create_subscription(
    sv: Extension<Arc<Service>>,
    Json(payload): Json<FullSubscription>,
) -> (StatusCode, String) {
    //Json<Result<FullSubscription, String>>) {
    let res = sv.subscriptions.create(payload).await;

    match res {
        Ok(subscription) => (
            StatusCode::CREATED,
            serde_json::to_string(&subscription).unwrap_or_default(),
        ),
        Err(err) => (StatusCode::NOT_ACCEPTABLE, err.to_string()),
    }
}
