use std::sync::Arc;

use thiserror::Error;

// Example repo dependency
use crate::{models::Worker, ports::worksite_repository::WorksiteRepository};

#[derive(Clone)]
pub struct AssignWorker {
    // Put infra dependencies in this struct
    // Below is an example of a repo dependency
    worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct AssignWorkerInput {
    pub worksite_id: String,
    pub location_id: String,
    pub shift_id: String,
    pub worker_id: String,
}

// Change the return type, if needed
pub type AssignWorkerOutput = Result<(), AssignWorkerFailure>;

impl AssignWorker {
    pub fn new(worksite_repository: Arc<dyn WorksiteRepository>) -> Self {
        Self {
            worksite_repository,
        }
    }

    pub async fn assign_worker(&self, input: AssignWorkerInput) -> AssignWorkerOutput {
        let worksite = &self
            .worksite_repository
            .get_worksite(input.worksite_id)
            .await
            .map_err(|e| AssignWorkerFailure::Unknown(e.to_string()))?
            .ok_or(AssignWorkerFailure::NotFound)?;

        let worker = worksite
            .get_worker(input.worker_id)
            .ok_or(AssignWorkerFailure::WorkerNotFound)?;

        let AssignWorkerInput {
            shift_id,
            location_id,
            ..
        } = input;

        let worksite = worksite.assign_worker(worker, shift_id, location_id);

        self.worksite_repository
            .save(worksite)
            .await
            .map_err(|e| AssignWorkerFailure::Unknown(e.to_string()))?;

        Ok(())
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum AssignWorkerFailure {
    #[error("Worker does not exist")]
    WorkerNotFound,
    #[error("Something went wrong")]
    Unknown(String),
    #[error("Worksite does not exist")]
    NotFound,
}
