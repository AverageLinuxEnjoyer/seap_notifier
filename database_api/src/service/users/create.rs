use crate::models::user::User;

use super::Users;
use anyhow::{Error, Result};
use sqlx::{query, query_as};

impl Users {
    pub async fn create(&self, email: &str) -> Result<User> {
        let mut tx = self.pool.begin().await?;

        let user = Users::get_by_email(&mut tx, email).await.ok();

        if user.is_some() {
            return Err(Error::msg("An user with this email already exists"));
        }

        query!("INSERT INTO users (email) VALUES (?)", email)
            .execute(&self.pool)
            .await?;

        tx.commit().await?;

        let mut tx = self.pool.begin().await?;
        let created_user = Users::get_by_email(&mut tx, email).await?;
        tx.commit().await?;

        Ok(created_user)
    }
}
