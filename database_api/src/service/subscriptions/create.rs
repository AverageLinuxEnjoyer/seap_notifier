use super::Subscriptions;
use crate::{full_subscription::FullSubscription, models::subscription::Subscription};
use anyhow::Result;
use sqlx::{query, query_as, query_scalar};

impl Subscriptions {
    #[cfg(feature = "create")]
    pub async fn create(&self, subscription: FullSubscription) -> Result<FullSubscription> {
        use crate::service::subscriptions::utils::create_contract_description;

        use super::utils::create_contract_object;

        let mut tx = self.pool.begin().await?;

        let created_subscription = subscription.clone();

        let user_id = query_scalar!("SELECT id FROM users WHERE email = ?", subscription.email)
            .fetch_one(&mut tx)
            .await;

        // todo: get rid of this side effect, or make it obvious
        let user_id = match user_id {
            Ok(id) => id,
            Err(_) => {
                query!("INSERT INTO users (email) VALUES (?)", subscription.email)
                    .execute(&mut tx)
                    .await?;

                query_scalar!("SELECT id FROM users WHERE email = ?", subscription.email)
                    .fetch_one(&mut tx)
                    .await?
            }
        };

        query!(
            "INSERT INTO subscriptions (id_user, min_value, max_value) VALUES (?, ?, ?)",
            user_id,
            subscription.min_value,
            subscription.max_value
        )
        .execute(&mut tx)
        .await?;

        let subscription_id =
            query_scalar!("SELECT id FROM subscriptions WHERE id= LAST_INSERT_ID()")
                .fetch_one(&mut tx)
                .await?;

        if let Some(contract_object) = subscription.contract_object {
            create_contract_object(&mut tx, subscription_id, contract_object).await?;
        }

        if let Some(contract_description) = subscription.contract_desc {
            create_contract_description(&mut tx, subscription_id, contract_description).await?;
        }

        tx.commit().await?;

        Ok(created_subscription)
    }
}
