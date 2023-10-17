use async_trait::async_trait;
use thiserror::Error;

use crate::models::Worksite;

#[async_trait]
pub trait WorksiteRepository {
    async fn get_worksite(&self, id: String) -> Result<Worksite, RepositoryFailure>;
}

#[derive(Error, Debug, PartialEq)]
pub enum RepositoryFailure {
    #[error("Something went wrong")]
    Unknown(String),
}
