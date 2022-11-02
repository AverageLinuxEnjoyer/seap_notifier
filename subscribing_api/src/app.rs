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
    create_subscription, delete_subscription, get_all_subscriptions, get_subscriptions,
    update_subscription, Email, Pagination,
};

pub trait App {
    fn from_service(service: Service) -> Self;
}

impl App for Router {
    fn from_service(service: Service) -> Router {
        let service = Arc::new(service);

        let clone = service.clone();
        let get_subscriptions =
            move |email_params: Option<Query<Email>>,
                  pagination_params: Option<Query<Pagination>>| async move {
                match (email_params, pagination_params) {
                    (Some(_), Some(_)) => (StatusCode::CONFLICT, Json(None)),
                    (Some(params), None) => get_subscriptions(clone, params).await,
                    (None, Some(params)) => get_all_subscriptions(clone, params).await,
                    (None, None) => (StatusCode::CONFLICT, Json(None)),
                }
            };

        let clone = service.clone();
        let create_subscription = move |json: Json<FullSubscription>| async move {
            create_subscription(clone, json).await
        };

        // let clone = service.clone(); //.clone();
        // let delete_subscription =
        //     move |id_path: Path<u32>| async move { delete_subscription(clone, id_path).await };

        let clone = service.clone();
        let update_subscription = move |json: Json<FullSubscription>, id_path: Path<u32>| async move {
            update_subscription(clone, id_path, json).await
        };

        Router::new()
            .route("/get", get(get_subscriptions))
            .route("/create", post(create_subscription))
            .route("/delete/:id", delete(delete_subscription))
            .route("/update/:id", put(update_subscription))
            .layer(Extension(service))
    }
}
