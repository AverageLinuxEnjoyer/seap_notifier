use super::Subscriptions;
use crate::{full_subscription::FullSubscription, models::subscription::Subscription};
use anyhow::Result;
use sqlx::{query_as, query_scalar};

impl Subscriptions {
    #[cfg(feature = "read")]
    pub async fn get_all(&self, start_index: u32, count: u32) -> Result<Vec<FullSubscription>> {
        use crate::service::subscriptions::utils::get_contract_descriptions;

        use super::utils::get_contract_objects;

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
            let contract_objects = get_contract_objects(&mut tx, sub.id).await.ok();

            let contract_descriptions = get_contract_descriptions(&mut tx, sub.id).await.ok();

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

        tx.commit().await?;

        Ok(res)
    }
}
