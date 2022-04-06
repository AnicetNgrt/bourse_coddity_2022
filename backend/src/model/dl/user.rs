use async_trait::async_trait;

use super::super::user;
use super::FindError;

#[async_trait]
pub trait Repository {
    async fn find_user_id_by_login(&self, login: &String) -> Result<i64, FindError>;
    async fn find_user_data(&self, user_id: i64) -> Result<&user::Data, FindError>;
    async fn create_user(&self, user_data: &user::Data) -> Result<i64, CreateError>;
}

pub enum CreateError {
    Failure,
    EmailNotUnique,
}