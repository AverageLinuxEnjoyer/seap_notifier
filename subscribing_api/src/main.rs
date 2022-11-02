use anyhow::Result;
use app::App;
use axum::{
    routing::{get, post},
    Json, Router,
};
use database_api::{full_subscription::FullSubscription, service::Service};
use std::{net::SocketAddr, sync::Arc};
use view::create_subscription;

pub mod app;
pub mod view;

#[tokio::main]
async fn main() -> Result<()> {
    let service = Service::new().await?;
    let app = Router::from_service(service);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
