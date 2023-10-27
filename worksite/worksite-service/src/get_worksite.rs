use std::sync::Arc;

use thiserror::Error;

use crate::models::Worksite;
use crate::ports::worksite_repository::WorksiteRepository;

#[derive(Clone)]
pub struct GetWorksite {
    pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct GetWorksiteInput {
    pub id: String
}

impl GetWorksite {
    pub async fn get_worksite(&self, input: GetWorksiteInput) -> Result<Option<Worksite>, GetWorksiteFailure> {
        self.worksite_repository
            .get_worksite(input.id)
            .await
            .map_err(|e| GetWorksiteFailure::Unknown(e.to_string()))
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum GetWorksiteFailure {
    #[error("Something went wrong")]
    Unknown(String),
}
