use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    Json,
};
use database_api::service::Service;
use serde_json::{json, Value};
use std::sync::Arc;

pub async fn delete_subscription(
    sv: Extension<Arc<Service>>,
    Path(id): Path<u32>,
) -> (StatusCode, Json<Value>) {
    let res = sv.subscriptions.delete(id).await;

    match res {
        Ok(sub) => (
            StatusCode::ACCEPTED,
            Json(serde_json::to_value(sub).unwrap()),
        ),
        Err(err) => (
            StatusCode::NOT_ACCEPTABLE,
            Json(json!({"Err":err.to_string()})),
        ),
    }
}
