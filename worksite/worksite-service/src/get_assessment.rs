use std::sync::Arc;

use thiserror::Error;

use crate::{models::Assessment, ports::worksite_repository::WorksiteRepository};

#[derive(Clone)]
pub struct GetAssessment {
    pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct GetAssessmentInput {
    // Put input fields here
    pub worksite_id: String,
    pub worker_id: String,
    pub assessment_id: String,
}

// Change the return type, if needed
pub type GetAssessmentOutput = Result<Option<Assessment>, GetAssessmentFailure>;

impl GetAssessment {
    pub async fn get_assessment(&self, input: GetAssessmentInput) -> GetAssessmentOutput {
        let worksite = self
            .worksite_repository
            .get_worksite(input.worksite_id)
            .await
            .map_err(|e| GetAssessmentFailure::Unknown(e.to_string()))?
            .ok_or(GetAssessmentFailure::NotFound)?;

        Ok(worksite.get_assessment_for_worker(input.worker_id, input.assessment_id))
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum GetAssessmentFailure {
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
