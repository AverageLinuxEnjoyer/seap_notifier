use axum::{extract::Extension, http::StatusCode, Json};
use database_api::{full_subscription::FullSubscription, service::Service};
use std::sync::Arc;

pub async fn create_subscription(
    sv: Extension<Arc<Service>>,
    Json(payload): Json<FullSubscription>,
) -> (StatusCode, Json<Result<FullSubscription, String>>) {
    let res = sv.subscriptions.create(payload).await;

    match res {
        Ok(subscription) => (StatusCode::CREATED, Json(Ok(subscription))),
        Err(err) => (StatusCode::NOT_ACCEPTABLE, Json(Err(err.to_string()))),
    }
}
