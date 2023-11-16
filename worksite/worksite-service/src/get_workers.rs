use std::sync::Arc;

use thiserror::Error;

use crate::{models::Worker, ports::worksite_repository::WorksiteRepository};

#[derive(Clone)]
pub struct GetWorkers {
    pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct GetWorkersInput {
    pub worksite_id: String,
}

// Change the return type, if needed
pub type GetWorkersOutput = Result<Vec<Worker>, GetWorkersFailure>;

impl GetWorkers {
    pub async fn get_workers(&self, input: GetWorkersInput) -> GetWorkersOutput {
        let worksite = self
            .worksite_repository
            .get_worksite(input.worksite_id.clone())
            .await
            .map_err(|e| GetWorkersFailure::Unknown(e.to_string()))?
            .ok_or(GetWorkersFailure::NotFound)?;

        Ok(worksite.workers)
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum GetWorkersFailure {
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
