use super::Users;
use crate::models::user::User;
use anyhow::Result;
use sqlx::{query_as, MySql, Transaction};

impl Users {
    pub(crate) async fn get_by_email<'c>(
        tx: &mut Transaction<'c, MySql>,
        email: &str,
    ) -> Result<User> {
        let user = query_as!(
            User,
            "SELECT id AS \"id?\", email FROM users WHERE email = ?",
            email
        )
        .fetch_one(&mut *tx)
        .await?;

        Ok(user)
    }
}
