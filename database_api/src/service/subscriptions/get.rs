use super::utils::get_by_id;
use super::utils::get_contract_objects;
use super::Subscriptions;
use crate::service::subscriptions::utils::get_contract_descriptions;
use crate::{full_subscription::FullSubscription, models::subscription::Subscription};
use anyhow::Result;
use sqlx::query_as;

impl Subscriptions {
    #[cfg(feature = "read")]
    pub async fn get(&self, id: u32) -> Result<FullSubscription> {
        let mut tx = self.pool.begin().await?;

        let res = get_by_id(&mut tx, id).await?;

        tx.commit().await?;

        Ok(res)
    }
}
