use std::sync::Arc;

use thiserror::Error;

use crate::models::{Event, Worksite};
use crate::ports::worksite_repository::WorksiteRepository;
use nonempty::{nonempty, NonEmpty};

#[derive(Clone)]
pub struct RemoveWorkerFromShift {
    pub worksite_repository: Arc<dyn WorksiteRepository>,
}

impl RemoveWorkerFromShift {
    pub async fn remove_worker_from_shift(
        &self,
        id: String,
        shift_id: String,
        worker_id: String,
    ) -> Result<Worksite, RemoveWorkerFromShiftFailure> {
        let worksite = self
            .worksite_repository
            .get_worksite(id.clone())
            .await
            .map_err(|e| RemoveWorkerFromShiftFailure::Unknown(e.to_string()))?
            .ok_or(RemoveWorkerFromShiftFailure::NotFound)?;

        let (updated_worksite, events) = worksite.remove_worker(shift_id, worker_id);

        self.worksite_repository
            .save(id, events)
            .await
            .map_err(|e| RemoveWorkerFromShiftFailure::Unknown(e.to_string()))?;

        Ok(updated_worksite)
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum RemoveWorkerFromShiftFailure {
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
