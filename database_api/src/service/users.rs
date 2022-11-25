use anyhow::Result;
use sqlx::MySqlPool;

mod create;
mod delete;
mod get;
mod get_all;
mod update;
mod utils;

#[cfg(feature = "admin")]
pub struct Users {
    pool: MySqlPool,
}

#[cfg(feature = "admin")]
impl Users {
    pub async fn new(db_url: &str) -> Result<Users> {
        Ok(Self {
            pool: MySqlPool::connect(db_url).await?,
        })
    }

    pub fn from_pool(db_pool: MySqlPool) -> Users {
        Self { pool: db_pool }
    }
}
