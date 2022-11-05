use super::Subscriptions;
use crate::full_subscription::FullSubscription;
use anyhow::Result;
use sqlx::query;

impl Subscriptions {
    #[cfg(feature = "update")]
    pub async fn update(&self, id: u32, subscription: FullSubscription) -> Result<()> {
        use crate::service::subscriptions::utils::{
            create_contract_description, create_contract_object, delete_contract_object,
        };

        use super::utils::delete_contract_description;

        let mut tx = self.pool.begin().await?;

        query!(
            "UPDATE subscriptions SET min_value = ?, max_value = ?",
            subscription.min_value,
            subscription.max_value
        )
        .execute(&mut tx)
        .await?;

        delete_contract_object(&mut tx, id).await?;
        delete_contract_description(&mut tx, id).await?;

        if let Some(contract_object) = subscription.contract_object {
            create_contract_object(&mut tx, id, contract_object).await?;
        }

        if let Some(contract_description) = subscription.contract_desc {
            create_contract_description(&mut tx, id, contract_description).await?;
        }

        tx.commit().await?;

        Ok(())
    }
}
