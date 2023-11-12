use std::sync::Arc;

use nonempty::NonEmpty;
use thiserror::Error;

// Example repo dependency
use crate::{
    models::{Event, Worker},
    ports::worksite_repository::WorksiteRepository,
};

#[derive(Clone)]
pub struct AssignWorker {
    // Put infra dependencies in this struct
    // Below is an example of a repo dependency
    worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct AssignWorkerInput {
    pub id: String,
    pub location_id: String,
    pub shift_id: String,
    pub first_name: String,
    pub last_name: String,
    pub street_address: String,
    pub city: String,
    pub region: String,
    pub postal_code: String,
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
        // TODO! "Implement assign_worker"

        let worksite = &self
            .worksite_repository
            .get_worksite(input.id)
            .await
            .map_err(|e| AssignWorkerFailure::Unknown(e.to_string()))?
            .ok_or(AssignWorkerFailure::NotFound)?;

        // TODO! Implement uuid generation as a port
        let worker = Worker {
            id: uuid::Uuid::new_v4().to_string(),
            first_name: input.first_name,
            last_name: input.last_name,
            last_assessment: None,
            tags: Vec::new(),
        };

        let AssignWorkerInput {
            shift_id,
            location_id,
            ..
        } = input;

        let events = {
            // TODO! Reconsider assumption that assigning worker to shit, also comes with creating new worker.
            let (_, add_worker_events) = worksite.add_worker(worker.clone());
            let (_, assign_worker_events) = worksite.assign_worker(worker, shift_id, location_id);

            let events: Vec<Event> = add_worker_events
                .into_iter()
                .chain(assign_worker_events.into_iter())
                .collect();

            NonEmpty::from_vec(events).unwrap()
        };

        self.worksite_repository
            .save(worksite.id.clone(), events)
            .await
            .map_err(|e| AssignWorkerFailure::Unknown(e.to_string()))?;

        Ok(())
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum AssignWorkerFailure {
    #[error("Something went wrong")]
    Unknown(String),
    #[error("Worksite does not exist")]
    NotFound,
}
