use std::sync::Arc;

use thiserror::Error;

use crate::models::Worksite;
use crate::ports::worksite_repository::WorksiteRepository;

#[derive(Clone)]
pub struct RemoveWorkerFromShift {
    pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct RemoveWorkerFromShiftInput {
    pub id: String,
    pub shift_id: String,
    pub worker_id: String,
}

impl RemoveWorkerFromShift {
    pub async fn remove_worker_from_shift(
        &self,
        input: RemoveWorkerFromShiftInput,
    ) -> Result<Worksite, RemoveWorkerFromShiftFailure> {
        let worksite = self
            .worksite_repository
            .get_worksite(input.id.clone())
            .await
            .map_err(|e| RemoveWorkerFromShiftFailure::Unknown(e.to_string()))?
            .ok_or(RemoveWorkerFromShiftFailure::WorksiteNotFound)?;

        let worker = worksite
            .get_worker(input.worker_id)
            .ok_or(RemoveWorkerFromShiftFailure::WorkerNotFound)?;

        let updated_worksite = worksite.remove_worker(input.shift_id, worker);

        self.worksite_repository
            .save(updated_worksite.clone())
            .await
            .map_err(|e| RemoveWorkerFromShiftFailure::Unknown(e.to_string()))?;

        Ok(updated_worksite)
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum RemoveWorkerFromShiftFailure {
    #[error("Worksite does not exist")]
    WorksiteNotFound,
    #[error("Worker does not exist")]
    WorkerNotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
