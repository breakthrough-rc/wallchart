use std::sync::Arc;

use thiserror::Error;

use crate::{
    models::{Assessment, Worker},
    ports::worksite_repository::WorksiteRepository,
};

#[derive(Clone)]
pub struct EditAssessment {
    pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct EditAssessmentInput {
    // Put input fields here
    pub worksite_id: String,
    pub worker_id: String,
    pub assessment_id: String,
    pub value: u8,
    pub notes: String,
}

// Change the return type, if needed
pub type EditAssessmentOutput = Result<(), EditAssessmentFailure>;

impl EditAssessment {
    pub async fn edit_assessment(&self, input: EditAssessmentInput) -> EditAssessmentOutput {
        let worksite = self
            .worksite_repository
            .get_worksite(input.worksite_id)
            .await
            .map_err(|e| EditAssessmentFailure::Unknown(e.to_string()))?
            .ok_or(EditAssessmentFailure::NotFound)?;

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
            .map_err(|e| EditAssessmentFailure::Unknown(e.to_string()))?;

        Ok(())
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum EditAssessmentFailure {
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
