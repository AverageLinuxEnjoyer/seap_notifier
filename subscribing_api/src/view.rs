use axum::{
    body::Full,
    extract::{Extension, Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use database_api::{full_subscription::FullSubscription, service::Service};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct Email {
    pub email: String,
}

async fn get_user_subscriptions(
    sv: Extension<Arc<Service>>,
    params: Query<Email>,
) -> (StatusCode, Json<Result<Vec<FullSubscription>, String>>) {
    let res = sv.subscriptions.get(&params.email).await;

    match res {
        Ok(subscriptions) => (StatusCode::ACCEPTED, Json(Ok(subscriptions))),
        Err(err) => (StatusCode::NOT_FOUND, Json(Err(err.to_string()))),
    }
}

#[derive(Serialize, Deserialize)]
pub struct Pagination {
    pub start_index: u32,
    pub count: u32,
}

async fn get_all_subscriptions(
    sv: Extension<Arc<Service>>,
    params: Query<Pagination>,
) -> (StatusCode, Json<Result<Vec<FullSubscription>, String>>) {
    let res = sv
        .subscriptions
        .get_all(params.start_index, params.count)
        .await;

    match res {
        Ok(subscriptions) => (StatusCode::ACCEPTED, Json(Ok(subscriptions))),
        Err(err) => (StatusCode::NOT_FOUND, Json(Err(err.to_string()))),
    }
}

pub async fn get_subscriptions(
    sv: Extension<Arc<Service>>,
    email_params: Option<Query<Email>>,
    pagination_params: Option<Query<Pagination>>,
) -> (StatusCode, Json<Result<Vec<FullSubscription>, String>>) {
    match (email_params, pagination_params) {
        (Some(_), Some(_)) => (StatusCode::CONFLICT, Json(Err("Expected either an email parameter or pagination parameters, but both were provided.".to_string()))),
        (Some(params), None) => get_user_subscriptions(sv, params).await,
        (None, Some(params)) => get_all_subscriptions(sv, params).await,
        (None, None) => (StatusCode::CONFLICT, Json(Err("Expected either an email parameter or pagination parameters, but neither were provided.".to_string()))),
    }
}

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

pub async fn delete_subscription(
    sv: Extension<Arc<Service>>,
    Path(id): Path<u32>,
) -> (StatusCode, Json<Result<String, String>>) {
    let res = sv.subscriptions.delete(id).await;

    match res {
        Ok(_) => (
            StatusCode::OK,
            Json(Ok("Subscription deleted.".to_string())),
        ),
        Err(err) => (StatusCode::NOT_ACCEPTABLE, Json(Err(err.to_string()))),
    }
}

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
