use std::sync::Arc;

use crate::{models::Worker, ports::worksite_repository::WorksiteRepository};
use thiserror::Error;

#[derive(Clone)]
pub struct AssignTags {
    pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct AssignTagsInput {
    pub worksite_id: String,
    pub worker_id: String,
    pub tags: Vec<String>,
}

// Change the return type, if needed
pub type AssignTagsOutput = Result<(), AssignTagsFailure>;

impl AssignTags {
    pub async fn assign_tags(&self, input: AssignTagsInput) -> AssignTagsOutput {
        let worksite = self
            .worksite_repository
            .get_worksite(input.worksite_id)
            .await
            .map_err(|e| AssignTagsFailure::Unknown(e.to_string()))?
            .ok_or(AssignTagsFailure::NotFound)?;

        let updated_worksite = worksite.update_worker(input.worker_id, |worker| -> Worker {
            worker.assign_tags(input.tags)
        });

        self.worksite_repository
            .save(updated_worksite)
            .await
            .map_err(|e| AssignTagsFailure::Unknown(e.to_string()))?;

        Ok(())
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum AssignTagsFailure {
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
