use async_trait::async_trait;
use thiserror::Error;

use crate::{models::Worksite, remove_worker_from_shift::Event};

#[async_trait]
pub trait WorksiteRepository: Send + Sync + 'static {
    async fn get_worksite(&self, id: String) -> Result<Option<Worksite>, RepositoryFailure>;

    async fn save(&self, id: String, worksite: &Worksite) -> Result<(), RepositoryFailure>;
}

#[derive(Error, Debug, PartialEq)]
pub enum RepositoryFailure {
    #[error("Failed to get connection from pool")]
    FailedToGetConnectionFromPool,
    #[error("Something went wrong")]
    Unknown(String),
}
