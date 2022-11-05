use crate::views::subscriptions::{
    create::create_subscription, delete::delete_subscription, get::get_subscriptions,
    update::update_subscription,
};
use axum::{
    extract::Extension,
    routing::{delete, get, post, put},
    Router,
};
use database_api::service::Service;
use std::sync::Arc;

pub mod subscriptions;
pub mod users;

pub trait FromService {
    fn from_service(service: Service) -> Self;
}

impl FromService for Router {
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
