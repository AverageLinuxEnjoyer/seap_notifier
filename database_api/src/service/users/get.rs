use super::Users;
use crate::models::user::User;
use anyhow::Result;
use sqlx::query_as;

impl Users {
    pub async fn get(&mut self, email: &str) -> Result<User> {
        let user = query_as!(User, "SELECT * FROM users WHERE email = ?", email)
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }
}
