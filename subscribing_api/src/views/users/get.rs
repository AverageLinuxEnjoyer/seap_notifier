use crate::views::query_params::Pagination;
use axum::{
    extract::{Extension, Path, Query},
    Json,
};
use database_api::service::Service;
use hyper::StatusCode;
use serde_json::{json, Value};
use std::sync::Arc;

pub async fn get_users(
    sv: Extension<Arc<Service>>,
    path: Option<Path<u32>>,
    params: Option<Query<Pagination>>,
) -> (StatusCode, Json<Value>) {
    match (path, params) {
        (Some(_), Some(_)) => (
            StatusCode::CONFLICT,
            Json(
                json!({"Err":"Expected either a user id or pagination parameters, but both were provided.".to_string()}),
            ),
        ),
        (Some(path), None) => {
            let res = sv.users.get_by_id(path.0).await;

            match res {
                Ok(user) => (
                    StatusCode::ACCEPTED,
                    Json(serde_json::to_value(user).unwrap()),
                ),
                Err(err) => (StatusCode::NOT_FOUND, Json(json!({"Err":err.to_string()}))),
            }
        }
        (None, Some(params)) => {
            let res = sv.users.get_all(params.start_index, params.count).await;

            match res {
                Ok(users) => (
                    StatusCode::ACCEPTED,
                    Json(serde_json::to_value(users).unwrap()),
                ),
                Err(err) => (StatusCode::NOT_FOUND, Json(json!({"Err": err.to_string()}))),
            }
        }
        (None, None) => (
            StatusCode::CONFLICT,
            Json(
                json!({"Err":"Expected either a user id or pagination parameters, but neither were provided.".to_string()}),
            ),
        ),
    }
}
