use super::Users;
use anyhow::Result;
use sqlx::query;

impl Users {
    pub async fn create(&mut self, email: &str) -> Result<()> {
        query!("INSERT INTO users (email) VALUES (?)", email)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
