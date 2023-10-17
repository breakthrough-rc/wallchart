use std::sync::Arc;

use thiserror::Error;

use crate::models::Worksite;
use crate::ports::worksite_repository::WorksiteRepository;

pub struct RemoveWorkerFromShift {
    worksite_repository: Arc<dyn WorksiteRepository>,
}

impl RemoveWorkerFromShift {
    async fn remove_worker_from_shift(
        &self,
        id: String,
        shift_id: String,
        worker_id: String,
    ) -> Result<Worksite, GetWorksiteFailure> {
        let worksite = self
            .worksite_repository
            .get_worksite(id)
            .await
            .map_err(|e| GetWorksiteFailure::Unknown(e.to_string()));

        todo!()
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum GetWorksiteFailure {
    #[error("Something went wrong")]
    Unknown(String),
}
