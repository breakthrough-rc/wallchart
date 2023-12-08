use std::sync::Arc;

use thiserror::Error;

use crate::{models::Worksite, ports::worksite_repository::WorksiteRepository};

#[derive(Clone)]
pub struct UpdateWorksite {
    pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct UpdateWorksiteInput {
    pub worksite_id: String,
    pub worksite_name: String,
}

// Change the return type, if needed
pub type UpdateWorksiteOutput = Result<(), UpdateWorksiteFailure>;

impl UpdateWorksite {
    pub async fn update_worksite(&self, input: UpdateWorksiteInput) -> UpdateWorksiteOutput {
        let worksite = self
            .worksite_repository
            .get_worksite(input.worksite_id)
            .await
            .map_err(|e| UpdateWorksiteFailure::Unknown(e.to_string()))?
            .ok_or(UpdateWorksiteFailure::NotFound)?;

        let worksite = Worksite {
            name: input.worksite_name,
            ..worksite
        };

        self.worksite_repository
            .save(worksite)
            .await
            .map_err(|e| UpdateWorksiteFailure::Unknown(e.to_string()))?;

        Ok(())
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum UpdateWorksiteFailure {
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
