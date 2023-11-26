use crate::{
    models::{Address, Worker},
    ports::worksite_repository::WorksiteRepository,
};

use std::sync::Arc;
use thiserror::Error;

#[derive(Clone)]
pub struct UpdateWorker {
    pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct UpdateWorkerInput {
    pub worker_id: String,
    pub worksite_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub street_address: String,
    pub city: String,
    pub region: String,
    pub postal_code: String,
}

pub type UpdateWorkerOutput = Result<(), UpdateWorkerFailure>;

impl UpdateWorker {
    pub async fn update_worker(&self, input: UpdateWorkerInput) -> UpdateWorkerOutput {
        let _worksite_id = input.worksite_id.clone();

        let worksite = self
            .worksite_repository
            .get_worksite(input.worksite_id)
            .await
            .map_err(|e| UpdateWorkerFailure::Unknown(e.to_string()))?
            .ok_or(UpdateWorkerFailure::NotFound)?;

        let updated_worksite = worksite.update_worker(input.worker_id, |worker| -> Worker {
            Worker {
                first_name: input.first_name,
                last_name: input.last_name,
                email: input.email,
                address: Address {
                    street_address: input.street_address,
                    city: input.city,
                    region: input.region,
                    postal_code: input.postal_code,
                },
                ..worker
            }
        });

        self.worksite_repository
            .save(updated_worksite)
            .await
            .map_err(|e| UpdateWorkerFailure::Unknown(e.to_string()))?;

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
