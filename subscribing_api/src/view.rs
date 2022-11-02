use anyhow::Result;
use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_macros::debug_handler;
use database_api::{full_subscription::FullSubscription, service::Service};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct Email {
    pub email: String,
}

pub async fn get_subscriptions(
    sv: Arc<Service>,
    params: Query<Email>,
) -> (StatusCode, Json<Option<Vec<FullSubscription>>>) {
    let res = sv.subscriptions.get(&params.email).await;

    match res {
        Ok(subscriptions) => (StatusCode::ACCEPTED, Json(Some(subscriptions))),
        Err(_) => (StatusCode::NOT_FOUND, Json(None)),
    }
}

#[derive(Serialize, Deserialize)]
pub struct Pagination {
    pub start_index: u32,
    pub count: u32,
}

pub async fn get_all_subscriptions(
    sv: Arc<Service>,
    params: Query<Pagination>,
) -> (StatusCode, Json<Option<Vec<FullSubscription>>>) {
    let res = sv
        .subscriptions
        .get_all(params.start_index, params.count)
        .await;

    match res {
        Ok(subscriptions) => (StatusCode::ACCEPTED, Json(Some(subscriptions))),
        Err(err) => (StatusCode::NOT_FOUND, Json(None)),
    }
}

pub async fn create_subscription(
    sv: Arc<Service>,
    Json(payload): Json<FullSubscription>,
) -> impl IntoResponse {
    let res = sv.subscriptions.create(payload).await;

    match res {
        Ok(subscription) => (StatusCode::CREATED, Json(Some(subscription))),
        Err(_) => (StatusCode::NOT_ACCEPTABLE, Json(None)),
    }
}

#[debug_handler]
pub async fn delete_subscription(
    sv: Extension<Arc<Service>>,
    Path(id): Path<u32>,
) -> (StatusCode, Result<String, String>) {
    let res = sv.subscriptions.delete(id).await;

    match res {
        Ok(_) => (StatusCode::OK, Ok("Subscription deleted.".to_string())),
        Err(err) => (StatusCode::NOT_ACCEPTABLE, Err(err.to_string())),
    }
}

pub async fn update_subscription(
    sv: Arc<Service>,
    Path(id): Path<u32>,
    Json(payload): Json<FullSubscription>,
) -> impl IntoResponse {
    let res = sv.subscriptions.update(id, payload).await;

    match res {
        Ok(_) => (StatusCode::OK, Json("Subscription updated.".to_string())),
        Err(err) => (StatusCode::NOT_ACCEPTABLE, Json(err.to_string())),
    }
}
