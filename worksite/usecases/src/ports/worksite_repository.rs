use async_trait::async_trait;
use thiserror::Error;

use crate::{models::Worksite, remove_worker_from_shift::Event};

#[async_trait]
pub trait WorksiteRepository {
    async fn get_worksite(&self, id: String) -> Result<Worksite, RepositoryFailure>;
    async fn save(&self, id: String, events: Vec<Event>) -> Result<(), RepositoryFailure>;
}

#[derive(Error, Debug, PartialEq)]
pub enum RepositoryFailure {
    #[error("Something went wrong")]
    Unknown(String),
}
