use std::sync::Arc;

use thiserror::Error;

use crate::ports::worksite_repository::WorksiteRepository;

#[derive(Clone)]
pub struct AddLocation {
    pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct AddLocationInput {
    // Put input fields here
    pub worksite_id: String,
    pub location_name: String,
}

// Change the return type, if needed
pub type AddLocationOutput = Result<(), AddLocationFailure>;

impl AddLocation {
    pub async fn add_location(&self, input: AddLocationInput) -> AddLocationOutput {
        let worksite = self
            .worksite_repository
            .get_worksite(input.worksite_id.clone())
            .await
            .map_err(|e| AddLocationFailure::Unknown(e.to_string()))?
            .ok_or(AddLocationFailure::NotFound)?;

        let updated_worksite = worksite.add_new_location(input.location_name);

        self.worksite_repository
            .save(updated_worksite.clone())
            .await
            .map_err(|e| AddLocationFailure::Unknown(e.to_string()))?;

        Ok(())
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum AddLocationFailure {
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
