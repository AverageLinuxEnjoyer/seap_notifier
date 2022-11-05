use crate::models::user::User;

use super::Users;
use anyhow::{Error, Result};
use sqlx::{query, query_as};

impl Users {
    pub async fn create(&self, email: &str) -> Result<User> {
        let mut tx = self.pool.begin().await?;

        let user = query_as!(User, "SELECT * FROM users WHERE email = ?", email)
            .fetch_optional(&mut tx)
            .await?;

        if user.is_some() {
            return Err(Error::msg("An user with this email already exists"));
        }

        query!("INSERT INTO users (email) VALUES (?)", email)
            .execute(&self.pool)
            .await?;

        let created_user = query_as!(User, "SELECT * FROM users WHERE email = ?", email)
            .fetch_one(&mut tx)
            .await?;

        tx.commit().await?;

        Ok(created_user)
    }
}
