use crate::models::user::User;
use anyhow::Result;
use sqlx::{query, query_as, MySqlPool};

#[cfg(feature = "admin")]
pub struct Users {
    pool: MySqlPool,
}

#[cfg(feature = "admin")]
impl Users {
    pub async fn new(db_url: &str) -> Result<Users> {
        Ok(Self {
            pool: MySqlPool::connect(db_url).await?,
        })
    }

    pub fn from_pool(db_pool: MySqlPool) -> Users {
        Self { pool: db_pool }
    }

    pub async fn get(&mut self, email: &str) -> Result<User> {
        let user = query_as!(User, "SELECT * FROM users WHERE email = ?", email)
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn create(&mut self, email: &str) -> Result<()> {
        query!("INSERT INTO users (email) VALUES (?)", email)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

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

    pub async fn delete(&mut self, email: &str) -> Result<()> {
        query!("DELETE FROM users WHERE email = ?", email)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
