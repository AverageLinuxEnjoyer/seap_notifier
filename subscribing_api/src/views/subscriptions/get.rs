use crate::views::query_params::{Email, Pagination};
use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    Json,
};
use database_api::service::Service;
use serde_json::{json, Value};
use std::sync::Arc;

async fn get_user_subscriptions(
    sv: Extension<Arc<Service>>,
    params: Query<Email>,
) -> (StatusCode, Json<Value>) {
    let res = sv.subscriptions.get_by_email(&params.email).await;

    match res {
        Ok(subscriptions) => (
            StatusCode::ACCEPTED,
            Json(serde_json::to_value(subscriptions).unwrap()),
        ),
        Err(err) => (StatusCode::NOT_FOUND, Json(json!({"Err":err.to_string()}))),
    }
}

async fn get_all_subscriptions(
    sv: Extension<Arc<Service>>,
    params: Query<Pagination>,
) -> (StatusCode, Json<Value>) {
    let res = sv
        .subscriptions
        .get_all(params.start_index, params.count)
        .await;

    match res {
        Ok(subscriptions) => (
            StatusCode::ACCEPTED,
            Json(serde_json::to_value(subscriptions).unwrap()),
        ),
        Err(err) => (StatusCode::NOT_FOUND, Json(json!({"Err":err.to_string()}))),
    }
}

pub async fn get_subscriptions(
    sv: Extension<Arc<Service>>,
    email_params: Option<Query<Email>>,
    pagination_params: Option<Query<Pagination>>,
) -> (StatusCode, Json<Value>) {
    match (email_params, pagination_params) {
        (Some(_), Some(_)) => (
            StatusCode::CONFLICT,
            Json(
                json!({"Err":"Expected either an email parameter or pagination parameters, but both were provided.".to_string()}),
            ),
        ),
        (Some(params), None) => get_user_subscriptions(sv, params).await,
        (None, Some(params)) => get_all_subscriptions(sv, params).await,
        (None, None) => (
            StatusCode::CONFLICT,
            Json(
                json!({"Err":"Expected either an email parameter or pagination parameters, but neither were provided.".to_string()}),
            ),
        ),
    }
}

pub async fn get_subscription(
    sv: Extension<Arc<Service>>,
    Path(id): Path<u32>,
) -> (StatusCode, Json<Value>) {
    let sub = sv.subscriptions.get(id).await;

    match sub {
        Ok(sub) => (
            StatusCode::ACCEPTED,
            Json(serde_json::to_value(sub).unwrap()),
        ),
        Err(err) => (StatusCode::NOT_FOUND, Json(json!({"Err": err.to_string()}))),
    }
}
