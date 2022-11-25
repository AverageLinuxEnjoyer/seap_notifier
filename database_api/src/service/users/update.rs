use crate::models::user::User;

use super::Users;
use anyhow::{Error, Result};
use sqlx::{query, query_as};

impl Users {
    pub async fn update(&self, email: &str, new_user: User) -> Result<User> {
        let mut tx = self.pool.begin().await?;

        let user = Users::get_by_email(&mut tx, email).await.ok();
        if user.is_none() {
            return Err(Error::msg(
                "No user with this email was found. Nothing was updated.",
            ));
        }

        query!(
            "UPDATE users SET email = ? WHERE email = ?",
            new_user.email,
            email
        )
        .execute(&mut tx)
        .await?;

        let user = Users::get_by_email(&mut tx, &new_user.email).await?;

        tx.commit().await?;

        Ok(user)
    }
}
