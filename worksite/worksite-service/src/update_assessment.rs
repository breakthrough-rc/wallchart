use std::sync::Arc;

use thiserror::Error;

use crate::{
    models::{Assessment, Worker},
    ports::worksite_repository::WorksiteRepository,
};

#[derive(Clone)]
pub struct UpdateAssessment {
    pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct UpdateAssessmentInput {
    // Put input fields here
    pub worksite_id: String,
    pub worker_id: String,
    pub assessment_id: String,
    pub value: u8,
    pub notes: String,
}

// Change the return type, if needed
pub type UpdateAssessmentOutput = Result<(), UpdateAssessmentFailure>;

impl UpdateAssessment {
    pub async fn update_assessment(&self, input: UpdateAssessmentInput) -> UpdateAssessmentOutput {
        let worksite = self
            .worksite_repository
            .get_worksite(input.worksite_id)
            .await
            .map_err(|e| UpdateAssessmentFailure::Unknown(e.to_string()))?
            .ok_or(UpdateAssessmentFailure::NotFound)?;

        let updated_worksite = worksite.update_worker(input.worker_id, |worker| -> Worker {
            worker.update_assessment(input.assessment_id, |assessment| -> Assessment {
                Assessment {
                    id: assessment.id,
                    value: input.value,
                    notes: input.notes,
                }
            })
        });

        self.worksite_repository
            .save(updated_worksite)
            .await
            .map_err(|e| UpdateAssessmentFailure::Unknown(e.to_string()))?;

        Ok(())
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum UpdateAssessmentFailure {
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
