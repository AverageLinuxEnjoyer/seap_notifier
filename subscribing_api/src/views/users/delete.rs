use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    Json,
};
use database_api::{models::user::User, service::Service};
use serde_json::{json, Value};
use std::sync::Arc;

pub async fn delete_user(
    sv: Extension<Arc<Service>>,
    params: Option<Query<User>>,
) -> (StatusCode, Json<Value>) {
    let params = match params {
        None => {
            return (
                StatusCode::NOT_ACCEPTABLE,
                Json(json!({"Err":"Expected an email parameter. Nohing was deleted."})),
            )
        }
        Some(email) => email,
    };

    let res = sv.users.delete(&params.email).await;

    match res {
        Ok(user) => (
            StatusCode::ACCEPTED,
            Json(serde_json::to_value(user).unwrap()),
        ),
        Err(err) => (
            StatusCode::NOT_ACCEPTABLE,
            Json(json!({"Err":err.to_string()})),
        ),
    }
}
