use std::sync::Arc;

use thiserror::Error;

use crate::{models::Tag, ports::worksite_repository::WorksiteRepository};

#[derive(Clone)]
pub struct AddTag {
    pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct AddTagInput {
    // Put input fields here
    pub worksite_id: String,
    pub name: String,
    pub icon: String,
}

// Change the return type, if needed
pub type AddTagOutput = Result<(), AddTagFailure>;

impl AddTag {
    pub async fn add_tag(&self, input: AddTagInput) -> AddTagOutput {
        let worksite = self
            .worksite_repository
            .get_worksite(input.worksite_id)
            .await
            .map_err(|e| AddTagFailure::Unknown(e.to_string()))?
            .ok_or(AddTagFailure::NotFound)?;

        // TODO! Implement uuid generation as a port
        let worksite = worksite.add_tag(Tag {
            id: uuid::Uuid::new_v4().to_string(),
            name: input.name.clone(),
            icon: input.icon.clone(),
        });

        self.worksite_repository
            .save(worksite)
            .await
            .map_err(|e| AddTagFailure::Unknown(e.to_string()))?;

        Ok(())
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum AddTagFailure {
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
