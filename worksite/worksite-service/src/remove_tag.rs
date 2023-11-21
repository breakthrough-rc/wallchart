use std::sync::Arc;

use thiserror::Error;

use crate::ports::worksite_repository::WorksiteRepository;

#[derive(Clone)]
pub struct RemoveTag {
    pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct RemoveTagInput {
    // Put input fields here
    pub worksite_id: String,
    pub tag_id: String,
}

// Change the return type, if needed
pub type RemoveTagOutput = Result<(), RemoveTagFailure>;

impl RemoveTag {
    pub async fn remove_tag(&self, input: RemoveTagInput) -> RemoveTagOutput {
        let worksite = self
            .worksite_repository
            .get_worksite(input.worksite_id)
            .await
            .map_err(|e| RemoveTagFailure::Unknown(e.to_string()))?
            .ok_or(RemoveTagFailure::NotFound)?;

        let worksite = worksite.remove_tag(input.tag_id);

        self.worksite_repository
            .save(worksite)
            .await
            .map_err(|e| RemoveTagFailure::Unknown(e.to_string()))?;

        Ok(())
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum RemoveTagFailure {
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
