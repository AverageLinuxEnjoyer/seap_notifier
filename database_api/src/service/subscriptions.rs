use anyhow::Result;
use sqlx::MySqlPool;

mod create;
mod delete;
mod get;
mod get_all;
mod get_by_email;
mod update;
mod utils;

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
}
