use super::utils::get_contract_objects;
use super::Subscriptions;
use crate::service::subscriptions::utils::get_contract_descriptions;
use crate::{full_subscription::FullSubscription, models::subscription::Subscription};
use anyhow::Result;
use sqlx::query_as;

impl Subscriptions {
    #[cfg(feature = "read")]
    pub async fn get_by_email(&self, email: &str) -> Result<Vec<FullSubscription>> {
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
            let contract_objects = get_contract_objects(&mut tx, sub.id)
                .await
                .unwrap_or_default();

            let contract_descriptions = get_contract_descriptions(&mut tx, sub.id)
                .await
                .unwrap_or_default();

            res.push(FullSubscription {
                id: Some(sub.id),
                email: email.into(),
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
