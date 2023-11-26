use std::sync::Arc;

use chrono::Utc;
use thiserror::Error;

use crate::{
    models::{Assessment, Worker},
    ports::worksite_repository::WorksiteRepository,
};

#[derive(Clone)]
pub struct AddAssessment {
    pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct AddAssessmentInput {
    // Put input fields here
    pub worksite_id: String,
    pub worker_id: String,
    pub value: u8,
    pub notes: String,
    pub assessor: String,
}

// Change the return type, if needed
pub type AddAssessmentOutput = Result<(), AddAssessmentFailure>;

impl AddAssessment {
    pub async fn add_assessment(&self, input: AddAssessmentInput) -> AddAssessmentOutput {
        let worksite = self
            .worksite_repository
            .get_worksite(input.worksite_id)
            .await
            .map_err(|e| AddAssessmentFailure::Unknown(e.to_string()))?
            .ok_or(AddAssessmentFailure::NotFound)?;

        let updated_worksite = worksite.update_worker(input.worker_id, |worker| -> Worker {
            worker.add_assessment(Assessment {
                id: uuid::Uuid::new_v4().to_string(),
                value: input.value,
                notes: input.notes,
                assessor: input.assessor,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
        });
        self.worksite_repository
            .save(updated_worksite)
            .await
            .map_err(|e| AddAssessmentFailure::Unknown(e.to_string()))?;

        Ok(())
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum AddAssessmentFailure {
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
