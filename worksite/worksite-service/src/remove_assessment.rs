use std::sync::Arc;

use thiserror::Error;

use crate::{models::Worker, ports::worksite_repository::WorksiteRepository};

#[derive(Clone)]
pub struct RemoveAssessment {
    pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct RemoveAssessmentInput {
    // Put input fields here
    pub worksite_id: String,
    pub worker_id: String,
    pub assessment_id: String,
}

// Change the return type, if needed
pub type RemoveAssessmentOutput = Result<(), RemoveAssessmentFailure>;

impl RemoveAssessment {
    pub async fn remove_assessment(&self, input: RemoveAssessmentInput) -> RemoveAssessmentOutput {
        let worksite = self
            .worksite_repository
            .get_worksite(input.worksite_id)
            .await
            .map_err(|e| RemoveAssessmentFailure::Unknown(e.to_string()))?
            .ok_or(RemoveAssessmentFailure::NotFound)?;

        let updated_worksite = worksite.update_worker(input.worker_id, |worker| -> Worker {
            worker.remove_assessment(input.assessment_id)
        });

        self.worksite_repository
            .save(updated_worksite)
            .await
            .map_err(|e| RemoveAssessmentFailure::Unknown(e.to_string()))?;

        Ok(())
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum RemoveAssessmentFailure {
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
