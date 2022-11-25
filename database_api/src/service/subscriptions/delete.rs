use super::utils::get_by_id;
use super::Subscriptions;
use crate::full_subscription::FullSubscription;
use anyhow::{Error, Result};
use sqlx::query;

impl Subscriptions {
    #[cfg(feature = "delete")]
    pub async fn delete(&self, id: u32) -> Result<FullSubscription> {
        let mut tx = self.pool.begin().await?;

        let subscription = match get_by_id(&mut tx, id).await.ok() {
            Some(x) => x,
            None => {
                return Err(Error::msg(
                    "No subscription with this id was found. Nothing was deleted.",
                ))
            }
        };

        query!("DELETE FROM subscriptions WHERE id = ?", id)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(subscription)
    }
}
