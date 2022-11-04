use super::Subscriptions;
use crate::{full_subscription::FullSubscription, models::subscription::Subscription};
use anyhow::Result;
use sqlx::{query_as, query_scalar};

impl Subscriptions {
    #[cfg(feature = "read")]
    pub async fn get_all(&self, start_index: u32, count: u32) -> Result<Vec<FullSubscription>> {
        let mut tx = self.pool.begin().await?;

        let subscriptions = query_as!(
            Subscription,
            "SELECT * FROM subscriptions ORDER BY id LIMIT ? OFFSET ?",
            count,
            start_index
        )
        .fetch_all(&mut tx)
        .await?;

        let mut res = vec![];

        for sub in subscriptions {
            let contract_objects = query_scalar!(
                "SELECT value FROM contract_objects WHERE subscription_id = ?",
                sub.id
            )
            .fetch_all(&mut tx)
            .await
            .ok();

            let contract_descriptions = query_scalar!(
                "SELECT value FROM contract_descriptions WHERE subscription_id = ?",
                sub.id
            )
            .fetch_all(&mut tx)
            .await
            .ok();

            let email = query_scalar!("SELECT email FROM users WHERE id = ?", sub.id_user)
                .fetch_one(&mut tx)
                .await?;

            res.push(FullSubscription {
                id: Some(sub.id),
                email,
                min_value: sub.min_value,
                max_value: sub.max_value,
                contract_object: contract_objects,
                contract_desc: contract_descriptions,
            })
        }

        Ok(res)
    }
}
