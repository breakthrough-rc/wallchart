use std::sync::Arc;

use thiserror::Error;

use crate::{models::Tag, ports::worksite_repository::WorksiteRepository};

#[derive(Clone)]
pub struct UpdateTag {
    pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct UpdateTagInput {
    // Put input fields here
    pub worksite_id: String,
    pub tag_id: String,
    pub name: String,
    pub icon: String,
}

// Change the return type, if needed
pub type UpdateTagOutput = Result<(), UpdateTagFailure>;

impl UpdateTag {
    pub async fn update_tag(&self, input: UpdateTagInput) -> UpdateTagOutput {
        let worksite = self
            .worksite_repository
            .get_worksite(input.worksite_id)
            .await
            .map_err(|e| UpdateTagFailure::Unknown(e.to_string()))?
            .ok_or(UpdateTagFailure::NotFound)?;

        let worksite = worksite.update_tag(input.tag_id, |tag| -> Tag {
            Tag {
                id: tag.id.clone(),
                name: input.name,
                icon: input.icon,
            }
        });

        self.worksite_repository
            .save(worksite)
            .await
            .map_err(|e| UpdateTagFailure::Unknown(e.to_string()))?;

        Ok(())
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum UpdateTagFailure {
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
