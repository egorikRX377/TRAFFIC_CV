use crate::management_engine::clients::requests::auth::*;
use crate::management_engine::clients::traits::auth::AuthClient;
use async_trait::async_trait;
use sqlx::PgPool;

pub struct PgAuthClient {
    pub pool: PgPool,
}

#[async_trait]
impl AuthClient for PgAuthClient {
    async fn create_user(
        &self,
        username: &str,
        password_hash: &str,
        role_id: i32,
        user_info_id: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(INSERT_USER)
            .bind(username)
            .bind(password_hash)
            .bind(role_id)
            .bind(user_info_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn create_user_info(
        &self,
        full_name: &str,
        email: &str,
        phone_number: Option<&str>,
        organization: Option<&str>,
    ) -> Result<i32, sqlx::Error> {
        let row: (i32,) = sqlx::query_as(INSERT_USER_INFO)
            .bind(full_name)
            .bind(email)
            .bind(phone_number)
            .bind(organization)
            .fetch_one(&self.pool)
            .await?;
        Ok(row.0)
    }

    async fn get_user_details(
        &self,
        username: &str,
    ) -> Result<Option<(String, String)>, sqlx::Error> {
        let row: Option<(String, String)> = sqlx::query_as(SELECT_USER_DETAILS)
            .bind(username)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row)
    }

    async fn get_role_id(&self, role_name: &str) -> Result<Option<i32>, sqlx::Error> {
        let row: Option<(i32,)> = sqlx::query_as(SELECT_ROLE_ID_BY_NAME)
            .bind(role_name)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|r| r.0))
    }
}
