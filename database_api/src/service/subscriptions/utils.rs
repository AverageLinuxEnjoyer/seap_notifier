use anyhow::Result;
use sqlx::{query, query_as, query_scalar, MySql, Transaction};

use crate::{full_subscription::FullSubscription, models::subscription::Subscription};

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
) -> Result<Option<Vec<String>>> {
    let contract_objects = query_scalar!(
        "SELECT value FROM contract_objects WHERE subscription_id = ?",
        subscription_id
    )
    .fetch_all(tx)
    .await?;

    match contract_objects.is_empty() {
        true => Ok(None),
        false => Ok(Some(contract_objects)),
    }
}

pub(crate) async fn get_contract_descriptions<'c>(
    tx: &mut Transaction<'c, MySql>,
    subscription_id: u32,
) -> Result<Option<Vec<String>>> {
    let contract_descriptions = query_scalar!(
        "SELECT value FROM contract_descriptions WHERE subscription_id = ?",
        subscription_id
    )
    .fetch_all(tx)
    .await?;

    match contract_descriptions.is_empty() {
        true => Ok(None),
        false => Ok(Some(contract_descriptions)),
    }
}

pub(crate) async fn get_by_id<'c>(
    tx: &mut Transaction<'c, MySql>,
    subscription_id: u32,
) -> Result<FullSubscription> {
    let contract_objects = get_contract_objects(&mut *tx, subscription_id)
        .await
        .unwrap_or_default();

    let contract_descriptions = get_contract_descriptions(&mut *tx, subscription_id)
        .await
        .unwrap_or_default();

    let sub = query_as!(
        Subscription,
        "SELECT * FROM subscriptions WHERE id = ?",
        subscription_id
    )
    .fetch_one(&mut *tx)
    .await?;

    let email = query_scalar!("SELECT email FROM users WHERE id = ?", sub.id_user)
        .fetch_one(&mut *tx)
        .await?;

    Ok(FullSubscription {
        id: Some(subscription_id),
        email,
        min_value: sub.min_value,
        max_value: sub.max_value,
        contract_object: contract_objects,
        contract_desc: contract_descriptions,
    })
}
