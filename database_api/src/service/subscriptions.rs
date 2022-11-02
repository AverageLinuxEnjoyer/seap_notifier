use crate::{
    full_subscription::FullSubscription,
    models::{contract_description, subscription::Subscription},
};
use anyhow::Result;
use sqlx::{query, query_as, query_scalar, Acquire, MySql, MySqlExecutor, MySqlPool, Transaction};

pub struct Subscriptions {
    pool: MySqlPool,
}

impl Subscriptions {
    pub async fn from_url(db_url: &str) -> Result<Subscriptions> {
        Ok(Self {
            pool: MySqlPool::connect(db_url).await?,
        })
    }

    pub fn from_pool(db_pool: MySqlPool) -> Subscriptions {
        Self { pool: db_pool }
    }

    async fn create_contract_object<'c>(
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

    async fn create_contract_description<'c>(
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

    async fn delete_contract_object<'c>(
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

    async fn delete_contract_description<'c>(
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

    #[cfg(feature = "read")]
    pub async fn get(&self, email: &str) -> Result<Vec<FullSubscription>> {
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
            let contract_objects = query_scalar!(
                "SELECT value FROM contract_objects WHERE subscription_id = ?",
                sub.id
            )
            .fetch_all(&mut tx)
            .await
            .ok();

            let contract_descriptions = query_scalar!(
                "SELECT value FROM contract_descriptions WHERE subscription_id = ?",
                sub.id
            )
            .fetch_all(&mut tx)
            .await
            .ok();

            res.push(FullSubscription {
                id: Some(sub.id),
                email: email.into(),
                min_value: sub.min_value,
                max_value: sub.max_value,
                contract_object: contract_objects,
                contract_desc: contract_descriptions,
            })
        }

        Ok(res)
    }

    #[cfg(feature = "read")]
    pub async fn get_all(&self, start_index: u32, count: u32) -> Result<Vec<FullSubscription>> {
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
            let contract_objects = query_scalar!(
                "SELECT value FROM contract_objects WHERE subscription_id = ?",
                sub.id
            )
            .fetch_all(&mut tx)
            .await
            .ok();

            let contract_descriptions = query_scalar!(
                "SELECT value FROM contract_descriptions WHERE subscription_id = ?",
                sub.id
            )
            .fetch_all(&mut tx)
            .await
            .ok();

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

        Ok(res)
    }

    #[cfg(feature = "create")]
    pub async fn create(&self, subscription: FullSubscription) -> Result<FullSubscription> {
        let mut tx = self.pool.begin().await?;

        let clone = subscription.clone();

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
            Subscriptions::create_contract_object(&mut tx, subscription_id, contract_object)
                .await?;
        }

        if let Some(contract_description) = subscription.contract_desc {
            Subscriptions::create_contract_description(
                &mut tx,
                subscription_id,
                contract_description,
            )
            .await?;
        }

        tx.commit().await?;

        Ok(clone)
    }

    #[cfg(feature = "update")]
    pub async fn update(&self, id: u32, subscription: FullSubscription) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        query!(
            "UPDATE subscriptions SET min_value = ?, max_value = ?",
            subscription.min_value,
            subscription.max_value
        )
        .execute(&mut tx)
        .await?;

        Subscriptions::delete_contract_object(&mut tx, id).await?;
        Subscriptions::delete_contract_description(&mut tx, id).await?;

        if let Some(contract_object) = subscription.contract_object {
            Subscriptions::create_contract_object(&mut tx, id, contract_object).await?;
        }

        if let Some(contract_description) = subscription.contract_desc {
            Subscriptions::create_contract_description(&mut tx, id, contract_description).await?;
        }

        tx.commit().await?;

        Ok(())
    }

    #[cfg(feature = "delete")]
    pub async fn delete(&self, id: u32) -> Result<()> {
        query!("DELETE FROM subscriptions WHERE id = ?", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
