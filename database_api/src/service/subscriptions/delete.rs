use super::Subscriptions;
use anyhow::Result;
use sqlx::query;

impl Subscriptions {
    #[cfg(feature = "delete")]
    pub async fn delete(&self, id: u32) -> Result<()> {
        query!("DELETE FROM subscriptions WHERE id = ?", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
