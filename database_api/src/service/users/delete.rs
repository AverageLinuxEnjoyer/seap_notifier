use super::Users;
use crate::models::user::User;
use anyhow::{Error, Result};
use sqlx::{query, query_as};

impl Users {
    pub async fn delete(&self, email: &str) -> Result<User> {
        let mut tx = self.pool.begin().await?;

        let user = Users::get_by_email(&mut tx, email).await.ok();

        if user.is_none() {
            return Err(Error::msg(
                "No user with this email was found. Nothing was deleted.",
            ));
        };

        query!("DELETE FROM users WHERE email = ?", email)
            .execute(&self.pool)
            .await?;

        tx.commit().await?;

        // guaranteed not to panic, since there was a previous check
        // that would've returned if user was none
        Ok(user.unwrap())
    }
}
