use std::{thread::sleep, time::Duration};

use database_api::{
    full_subscription::FullSubscription, models::contract_description, service::Service,
};
use time::OffsetDateTime;

use crate::{advertisment::Advertisment, mailer::Mailer, scraper::Scraper};

pub struct App {
    service: Service,
    mailer: Mailer,
    scraper: Scraper,
    scraping_interval: Duration,
}

impl App {
    pub fn new(
        service: Service,
        mailer: Mailer,
        scraper: Scraper,
        scraping_interval: Duration,
    ) -> Self {
        Self {
            service,
            mailer,
            scraper,
            scraping_interval,
        }
    }

    pub async fn run(&mut self) {
        let mut previous_ads_ids = vec![];

        let mut first = true;
        'outer: loop {
            if !first {
                sleep(self.scraping_interval);
            } else {
                first = false
            }

            let today = OffsetDateTime::now_utc();
            println!("Current time: {}", today);

            // get the last 5 ads today
            let all_ads_ids = match self.scraper.get_last_adv_ids(5, today).await {
                Ok(ads) => ads,
                Err(e) => {
                    eprintln!("Error while getting the last ads: {}", e);
                    continue;
                }
            };

            println!("Last fetched ads ids: {:?}", all_ads_ids);

            let ads_ids = all_ads_ids
                .clone()
                .into_iter()
                .filter(|id| !previous_ads_ids.contains(id))
                .collect::<Vec<u32>>();

            println!("Filtered ads ids: {:?}", ads_ids);

            let mut new_ads = Vec::new();

            for id in &ads_ids {
                let ad = match self.scraper.get_adv(*id).await {
                    Ok(ad) => ad,
                    Err(e) => {
                        eprintln!("Error while getting the ad with id {}: {}", id, e);
                        continue;
                    }
                };

                new_ads.push(ad);
            }

            println!("New ads:");
            new_ads.iter().for_each(|ad| {
                println!("{}", ad.contract_object);
            });

            let mut start_index = 0;
            let count = 10;

            loop {
                let subs = match self.service.subscriptions.get_all(start_index, count).await {
                    Ok(subs) => {
                        if subs.is_empty() {
                            break;
                        }
                        subs
                    }
                    Err(e) => {
                        eprintln!("Error while getting the subscriptions: {}", e);
                        continue 'outer;
                    }
                };

                println!(
                    "Subscriptions: {:?}",
                    subs.iter()
                        .map(|s| s.id.unwrap_or_default())
                        .collect::<Vec<u32>>()
                );

                for sub in &subs {
                    for ad in &new_ads {
                        if App::ad_matches_sub(ad, sub).await {
                            println!(
                                "Ad {} matches the subscription {:?}",
                                ad.contract_object, sub.id
                            );

                            let a = self.mailer.notify(&sub.email, ad).await;

                            println!("Mail sent: {:?}", a);
                        }
                    }
                }

                start_index += count;
            }

            previous_ads_ids = all_ads_ids;
            println!("\n");
        }
    }

    async fn ad_matches_sub(ad: &Advertisment, sub: &FullSubscription) -> bool {
        let title_match = if let Some(contract_object) = &sub.contract_object {
            let a = ad
                .contract_object
                .split(' ')
                .map(|s| s.trim_matches(','))
                .any(|word| contract_object.iter().any(|sub_word| sub_word == word));
            a
        } else {
            true
        };

        let desc_match = if let Some(contract_desc) = &sub.contract_desc {
            let a = ad
                .contract_description
                .split(' ')
                .map(|s| s.trim_matches(','))
                .any(|word| contract_desc.iter().any(|sub_word| sub_word == word));
            a
        } else {
            true
        };

        let min_value_match = if let Some(min_value) = sub.min_value {
            ad.value > min_value
        } else {
            true
        };

        let max_value_match = if let Some(max_value) = sub.max_value {
            ad.value < max_value
        } else {
            true
        };

        title_match && desc_match && min_value_match && max_value_match
    }
}
