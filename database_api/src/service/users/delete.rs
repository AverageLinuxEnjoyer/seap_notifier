use super::Users;
use anyhow::Result;
use sqlx::query;

impl Users {
    pub async fn delete(&mut self, email: &str) -> Result<()> {
        query!("DELETE FROM users WHERE email = ?", email)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
