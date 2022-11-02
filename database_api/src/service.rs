use anyhow::Result;
use sqlx::MySqlPool;

use self::subscriptions::Subscriptions;

#[cfg(feature = "admin")]
use self::users::Users;

pub mod subscriptions;

#[cfg(feature = "admin")]
pub mod users;

pub struct Service {
    #[cfg(feature = "admin")]
    pub users: Users,
    pub subscriptions: Subscriptions,
}

impl Service {
    pub async fn new() -> Result<Service> {
        let pool = MySqlPool::connect("mysql://root:root@localhost/seap_subscriptions").await?;

        Ok(Self {
            #[cfg(feature = "admin")]
            users: Users::from_pool(pool.clone()),

            subscriptions: Subscriptions::from_pool(pool),
        })
    }
}
