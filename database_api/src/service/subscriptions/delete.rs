use super::Subscriptions;
use anyhow::Result;
use sqlx::query;

impl Subscriptions {
    #[cfg(feature = "delete")]
    pub async fn delete(&self, id: u32) -> Result<()> {
        use sqlx::query_as;

        let mut tx = self.pool.begin().await?;

        query!("DELETE FROM subscriptions WHERE id = ?", id)
            .execute(&mut tx)
            .await?;

        Ok(())
    }
}
