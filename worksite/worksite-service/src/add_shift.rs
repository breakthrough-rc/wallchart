use std::sync::Arc;

use thiserror::Error;

use crate::ports::worksite_repository::WorksiteRepository;

#[derive(Clone)]
pub struct AddShift {
    pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct AddShiftInput {
    pub worksite_id: String,
    pub location_id: String,
    pub shift_name: String,
}

// Change the return type, if needed
pub type AddShiftOutput = Result<(), AddShiftFailure>;

impl AddShift {
    pub async fn add_shift(&self, input: AddShiftInput) -> AddShiftOutput {
        let worksite = self
            .worksite_repository
            .get_worksite(input.worksite_id.clone())
            .await
            .map_err(|e| AddShiftFailure::Unknown(e.to_string()))?
            .ok_or(AddShiftFailure::NotFound)?;

        let updated_worksite = worksite
            .add_shift(input.location_id, input.shift_name)
            .ok_or(AddShiftFailure::LocationNotFound)?;

        self.worksite_repository
            .save(updated_worksite.clone())
            .await
            .map_err(|e| AddShiftFailure::Unknown(e.to_string()))?;

        Ok(())
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum AddShiftFailure {
    #[error("Location does not exist")]
    LocationNotFound,
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
