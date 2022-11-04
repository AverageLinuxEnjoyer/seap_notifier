use super::Subscriptions;
use anyhow::Result;
use sqlx::{query, MySql, Transaction};

impl Subscriptions {
    pub(crate) async fn create_contract_object<'c>(
        tx: &mut Transaction<'c, MySql>,
        subscription_id: u32,
        contract_object: Vec<String>,
    ) -> Result<()> {
        for word in contract_object {
            query!(
                "INSERT INTO contract_objects (subscription_id, value) VALUES (?,?)",
                subscription_id,
                word
            )
            .execute(&mut *tx)
            .await?;
        }

        Ok(())
    }

    pub(crate) async fn create_contract_description<'c>(
        tx: &mut Transaction<'c, MySql>,
        subscription_id: u32,
        contract_description: Vec<String>,
    ) -> Result<()> {
        for word in contract_description {
            query!(
                "INSERT INTO contract_descriptions (subscription_id, value) VALUES (?,?)",
                subscription_id,
                word
            )
            .execute(&mut *tx)
            .await?;
        }

        Ok(())
    }

    pub(crate) async fn delete_contract_object<'c>(
        tx: &mut Transaction<'c, MySql>,
        subscription_id: u32,
    ) -> Result<()> {
        query!(
            "DELETE FROM contract_objects WHERE subscription_id = ?",
            subscription_id
        )
        .execute(&mut *tx)
        .await?;

        Ok(())
    }

    pub(crate) async fn delete_contract_description<'c>(
        tx: &mut Transaction<'c, MySql>,
        subscription_id: u32,
    ) -> Result<()> {
        query!(
            "DELETE FROM contract_descriptions WHERE subscription_id = ?",
            subscription_id
        )
        .execute(&mut *tx)
        .await?;

        Ok(())
    }
}
