use crate::{app::App, scraper::Scraper};
use anyhow::Result;
use database_api::service::Service;
use mailer::Mailer;
use reqwest::Client;
use std::time::Duration;
use time::{macros::datetime, OffsetDateTime};

mod advertisment;
mod app;
mod mailer;
mod scraper;

#[tokio::main]
async fn main() -> Result<()> {
    let mailer = Mailer::new()?;
    let service = Service::new().await?;
    let scraper = Scraper::new(Client::new());

    let mut app = App::new(service, mailer, scraper, Duration::from_secs(60));

    // let test_date = datetime!(2022-11-17 00:00 UTC);
    // let now = OffsetDateTime::now_utc();
    // let ads = scraper.get_last_adv_ids(5, now).await?;
    // println!("ids: {:?}", ads);

    // for (i, ad) in ads.iter().enumerate() {
    //     println!("{} --> {:?}\n", i, ad);
    //     println!("{:?}\n", scraper.get_adv(*ad).await?.publication_date);
    //     println!("===================\n");
    // }
    app.run().await;

    Ok(())
}
