use super::Users;
use crate::models::user::User;
use anyhow::Result;
use sqlx::query_as;

impl Users {
    pub async fn get(&self, email: &str) -> Result<User> {
        let mut tx = self.pool.begin().await?;

        Users::get_by_email(&mut tx, email).await
    }

    pub async fn get_by_id(&self, id: u32) -> Result<User> {
        let mut tx = self.pool.begin().await?;

        let user = query_as!(
            User,
            "SELECT id AS \"id?\", email FROM users WHERE id = ?",
            id
        )
        .fetch_one(&mut tx)
        .await?;

        Ok(user)
    }
}
