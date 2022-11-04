use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    Json,
};
use database_api::service::Service;
use std::sync::Arc;

pub async fn delete_subscription(
    sv: Extension<Arc<Service>>,
    Path(id): Path<u32>,
) -> (StatusCode, Json<Result<String, String>>) {
    let res = sv.subscriptions.delete(id).await;

    println!("bro what");
    match res {
        Ok(_) => (
            StatusCode::NO_CONTENT,
            Json(Ok("Subscription deleted.".to_string())),
        ),
        Err(err) => (StatusCode::NOT_ACCEPTABLE, Json(Err(err.to_string()))),
    }
}
