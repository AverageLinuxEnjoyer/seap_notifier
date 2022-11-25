use super::Users;
use crate::models::user::User;
use anyhow::Result;
use sqlx::query_as;

impl Users {
    pub async fn get_all(&self, start_index: u32, count: u32) -> Result<Vec<User>> {
        let mut tx = self.pool.begin().await?;

        let users = query_as!(
            User,
            "SELECT id AS \"id?\", email FROM users ORDER BY id LIMIT ? OFFSET ?",
            count,
            start_index
        )
        .fetch_all(&mut tx)
        .await?;

        Ok(users)
    }
}
