use std::sync::Arc;

use crate::{models::Worker, ports::worksite_repository::WorksiteRepository};
use thiserror::Error;

// Example repo dependency
// use crate::ports::worksite_repository::WorksiteRepository;

#[derive(Clone)]
pub struct GetWorker {
    pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct GetWorkerInput {
    // Put input fields here
    pub id: String,
    pub worksite_id: String,
}

// Change the return type, if needed
pub type GetWorkerOutput = Result<Option<Worker>, GetWorkerFailure>;

impl GetWorker {
    pub async fn get_worker(&self, input: GetWorkerInput) -> GetWorkerOutput {
        let worksite = self
            .worksite_repository
            .get_worksite(input.worksite_id)
            .await
            .map_err(|e| GetWorkerFailure::Unknown(e.to_string()))?
            .ok_or(GetWorkerFailure::NotFound)?;

        let worker = worksite.get_worker(input.id);

        Ok(worker)
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum GetWorkerFailure {
    #[error("Something went wrong")]
    Unknown(String),
    #[error("Worksite does not exist")]
    NotFound,
}
