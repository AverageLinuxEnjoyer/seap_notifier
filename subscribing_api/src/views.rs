use self::{
    subscriptions::get::{self, get_subscription},
    users::{create::create_user, delete::delete_user, get::get_users},
};
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

pub mod query_params;
pub mod subscriptions;
pub mod users;

pub trait FromService {
    fn from_service(service: Service) -> Self;
}

impl FromService for Router {
    fn from_service(service: Service) -> Router {
        let service = Arc::new(service);

        Router::new()
            .route("/subscriptions/get", get(get_subscriptions))
            .route("/subscriptions/get/:id", get(get_subscription))
            .route("/subscriptions/create", post(create_subscription))
            .route("/subscriptions/delete/:id", delete(delete_subscription))
            .route("/subscriptions/update/:id", put(update_subscription))
            .route("/users/get", get(get_users))
            .route("/users/get/:id", get(get_users))
            .route("/users/create", post(create_user))
            .route("/users/delete", delete(delete_user))
            .layer(Extension(service))
    }
}
