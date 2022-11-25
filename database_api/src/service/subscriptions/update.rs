use super::utils::delete_contract_description;
use super::Subscriptions;
use crate::full_subscription::FullSubscription;
use crate::service::subscriptions::utils::{
    create_contract_description, create_contract_object, delete_contract_object, get_by_id,
};
use anyhow::{Error, Result};
use sqlx::query;

impl Subscriptions {
    #[cfg(feature = "update")]
    pub async fn update(
        &self,
        id: u32,
        subscription: FullSubscription,
    ) -> Result<FullSubscription> {
        let mut tx = self.pool.begin().await?;

        if get_by_id(&mut tx, id).await.ok().is_none() {
            return Err(Error::msg(
                "No subscription with this id was found. Nothing was updated.",
            ));
        };

        query!(
            "UPDATE subscriptions SET min_value = ?, max_value = ? WHERE id = ?",
            subscription.min_value.clone(),
            subscription.max_value.clone(),
            id
        )
        .execute(&mut tx)
        .await?;

        delete_contract_object(&mut tx, id).await?;
        delete_contract_description(&mut tx, id).await?;

        if let Some(contract_object) = subscription.contract_object.clone() {
            create_contract_object(&mut tx, id, contract_object).await?;
        }

        if let Some(contract_description) = subscription.contract_desc.clone() {
            create_contract_description(&mut tx, id, contract_description).await?;
        }

        tx.commit().await?;

        Ok(subscription)
    }
}
