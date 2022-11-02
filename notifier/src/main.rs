use anyhow::Result;
use time::OffsetDateTime;
//use common::advertisment::Advertisment;

mod advertisment;
mod seap;

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::new();
    let ads = seap::get_last_adv_ids(client.clone(), 5, OffsetDateTime::now_utc()).await?;

    for (i, ad) in ads.iter().enumerate() {
        println!("{} --> {:?}\n", i, ad);
        println!("{:?}\n", seap::get_adv(client.clone(), *ad).await?);
        println!("===================\n");
    }

    Ok(())
}
