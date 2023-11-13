use crate::{models::Worker, ports::worksite_repository::WorksiteRepository};
use nonempty::NonEmpty;
use std::sync::Arc;
use thiserror::Error;

// Example repo dependency
// use crate::ports::worksite_repository::WorksiteRepository;

#[derive(Clone)]
pub struct UpdateWorker {
    pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct UpdateWorkerInput {
    // Put input fields here
    pub worker_id: String,
    pub worksite_id: String,
    pub first_name: String,
    pub last_name: String,
}

// Change the return type, if needed
pub type UpdateWorkerOutput = Result<(), UpdateWorkerFailure>;

impl UpdateWorker {
    pub async fn update_worker(&self, input: UpdateWorkerInput) -> UpdateWorkerOutput {
        let worksite_id = input.worksite_id.clone();

        let worksite = self
            .worksite_repository
            .get_worksite(input.worksite_id)
            .await
            .map_err(|e| UpdateWorkerFailure::Unknown(e.to_string()))?
            .ok_or(UpdateWorkerFailure::NotFound)?;

        let (_, events) = worksite.update_worker(input.worker_id, |worker| -> Worker {
            Worker {
                first_name: input.first_name,
                last_name: input.last_name,
                ..worker.clone()
            }
        });

        if events.is_empty() {
            return Err(UpdateWorkerFailure::NotFound);
        }

        match NonEmpty::from_vec(events) {
            Some(events) => self
                .worksite_repository
                .save(worksite_id, events)
                .await
                .map_err(|e| UpdateWorkerFailure::Unknown(e.to_string()))?,
            None => (),
        };

        Ok(())
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum UpdateWorkerFailure {
    #[error("Something went wrong")]
    Unknown(String),
    #[error("Worksite does not exist")]
    NotFound,
}
