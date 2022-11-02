use std::sync::Arc;

use axum::{
    body::Full,
    extract::{Extension, Path, Query},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use database_api::{full_subscription::FullSubscription, service::Service};

use crate::view::{
    create_subscription, delete_subscription, get_subscriptions, update_subscription, Email,
    Pagination,
};

pub trait App {
    fn from_service(service: Service) -> Self;
}

impl App for Router {
    fn from_service(service: Service) -> Router {
        let service = Arc::new(service);

        Router::new()
            .route("/get", get(get_subscriptions))
            .route("/create", post(create_subscription))
            .route("/delete/:id", delete(delete_subscription))
            .route("/update/:id", put(update_subscription))
            .layer(Extension(service))
    }
}
