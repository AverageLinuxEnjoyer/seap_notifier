use super::Subscriptions;
use anyhow::Result;
use sqlx::{query, query_scalar, MySql, Transaction};

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

pub(crate) async fn get_contract_objects<'c>(
    tx: &mut Transaction<'c, MySql>,
    subscription_id: u32,
) -> Result<Vec<String>> {
    let contract_objects = query_scalar!(
        "SELECT value FROM contract_objects WHERE subscription_id = ?",
        subscription_id
    )
    .fetch_all(tx)
    .await?;

    Ok(contract_objects)
}

pub(crate) async fn get_contract_descriptions<'c>(
    tx: &mut Transaction<'c, MySql>,
    subscription_id: u32,
) -> Result<Vec<String>> {
    let contract_descriptions = query_scalar!(
        "SELECT value FROM contract_descriptions WHERE subscription_id = ?",
        subscription_id
    )
    .fetch_all(tx)
    .await?;

    Ok(contract_descriptions)
}
