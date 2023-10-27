use std::sync::Arc;

use thiserror::Error;

// Example repo dependency
// use crate::ports::worksite_repository::WorksiteRepository;

#[derive(Clone)]
pub struct AssignWorker {
    // Put infra dependencies in this struct
    // Below is an example of a repo dependency
    // pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct AssignWorkerInput {
    // Put input fields here
    pub id: String,
}

// Change the return type, if needed
pub type AssignWorkerOutput = Result<(), AssignWorkerFailure>;

impl AssignWorker {
    pub async fn assign_worker(&self, input: AssignWorkerInput) -> AssignWorkerOutput {
        todo!("Implement assign_worker")
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum AssignWorkerFailure {
    #[error("Something went wrong")]
    Unknown(String),
}
