use std::sync::Arc;
use thiserror::Error;

use crate::models::Assessment;
use crate::ports::worksite_repository::WorksiteRepository;

#[derive(Clone)]
pub struct GetAssessments {
    pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct GetAssessmentsInput {
    // Put input fields here
    pub worksite_id: String,
    pub worker_id: String,
}

// Change the return type, if needed
pub type GetAssessmentsOutput = Result<Vec<Assessment>, GetAssessmentsFailure>;

impl GetAssessments {
    pub async fn get_assessments(&self, input: GetAssessmentsInput) -> GetAssessmentsOutput {
        let worksite = self
            .worksite_repository
            .get_worksite(input.worksite_id)
            .await
            .map_err(|e| GetAssessmentsFailure::Unknown(e.to_string()))?
            .ok_or(GetAssessmentsFailure::NotFound)?;

        let mut assessments = worksite.get_assessments_for_worker(input.worker_id);
        assessments.reverse(); // Show most recent first (descending order)

        Ok(assessments)
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum GetAssessmentsFailure {
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
