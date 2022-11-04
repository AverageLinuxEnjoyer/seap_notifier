use super::Users;
use anyhow::Result;
use sqlx::query;

impl Users {
    pub async fn update(&mut self, email: &str, new_email: &str) -> Result<()> {
        query!(
            "UPDATE users SET email = ? WHERE email = ?",
            new_email,
            email
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
