use async_trait::async_trait;

#[async_trait]
pub trait AuthClient {
    async fn create_user(
        &self,
        username: &str,
        password_hash: &str,
        role_id: i32,
        user_info_id: i32,
    ) -> Result<(), sqlx::Error>;

    async fn create_user_info(
        &self,
        full_name: &str,
        email: &str,
        phone_number: Option<&str>,
        organization: Option<&str>,
    ) -> Result<i32, sqlx::Error>;

    async fn get_user_details(
        &self,
        username: &str,
    ) -> Result<Option<(String, String)>, sqlx::Error>;

    async fn get_role_id(&self, role_name: &str) -> Result<Option<i32>, sqlx::Error>;
}
