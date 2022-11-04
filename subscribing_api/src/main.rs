use anyhow::Result;
use axum::Router;
use database_api::service::Service;
use std::net::SocketAddr;
use views::FromService;
// use app::App;

// pub mod app;
// pub mod view;
pub mod views;

#[tokio::main]
async fn main() -> Result<()> {
    let service = Service::new().await?;
    let app = Router::from_service(service);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
