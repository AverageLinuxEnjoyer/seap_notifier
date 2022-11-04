use super::Subscriptions;
use crate::{full_subscription::FullSubscription, models::subscription::Subscription};
use anyhow::Result;
use sqlx::{query_as, query_scalar};

impl Subscriptions {
    #[cfg(feature = "read")]
    pub async fn get(&self, email: &str) -> Result<Vec<FullSubscription>> {
        let mut tx = self.pool.begin().await?;

        let subscriptions = query_as!(
            Subscription,
            "SELECT * FROM subscriptions WHERE id_user IN (SELECT id FROM users WHERE email = ?)",
            email
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

            res.push(FullSubscription {
                id: Some(sub.id),
                email: email.into(),
                min_value: sub.min_value,
                max_value: sub.max_value,
                contract_object: contract_objects,
                contract_desc: contract_descriptions,
            })
        }

        Ok(res)
    }
}
