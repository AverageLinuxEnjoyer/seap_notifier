use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    Json,
};
use database_api::{full_subscription::FullSubscription, service::Service};
use std::sync::Arc;

pub async fn update_subscription(
    sv: Extension<Arc<Service>>,
    Path(id): Path<u32>,
    Json(payload): Json<FullSubscription>,
) -> (StatusCode, Json<Result<String, String>>) {
    let res = sv.subscriptions.update(id, payload).await;

    match res {
        Ok(_) => (
            StatusCode::OK,
            Json(Ok("Subscription updated.".to_string())),
        ),
        Err(err) => (StatusCode::NOT_ACCEPTABLE, Json(Err(err.to_string()))),
    }
}
