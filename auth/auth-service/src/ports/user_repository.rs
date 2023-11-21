use async_trait::async_trait;
use thiserror::Error;

use crate::models::User;

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn get_user(&self, id: String) -> Result<Option<User>, RepositoryFailure>;
    async fn get_users(&self) -> Result<Vec<User>, RepositoryFailure>;
    async fn save(&self, user: User) -> Result<(), RepositoryFailure>;
    async fn find_by_email(&self, email: String) -> Result<Option<User>, RepositoryFailure>;
    async fn delete_by_id(&self, id: String) -> Result<(), RepositoryFailure>;
}

#[derive(Error, Debug, PartialEq)]
pub enum RepositoryFailure {
    #[error("Failed to get connection from pool")]
    FailedToGetConnectionFromPool,
    #[error("Something went wrong")]
    Unknown(String),
}
