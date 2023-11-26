use std::sync::Arc;

use thiserror::Error;

use crate::ports::worksite_repository::WorksiteRepository;

#[derive(Clone)]
pub struct CsvUpload {
    pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct CsvUploadInput {
    // Put input fields here
    pub worksite_id: String,
}

// Change the return type, if needed
pub type CsvUploadOutput = Result<(), CsvUploadFailure>;

impl CsvUpload {
    pub async fn csv_upload(&self, _input: CsvUploadInput) -> CsvUploadOutput {
        // let worksite = self
        //     .worksite_repository
        //     .get_worksite(input.worksite_id)
        //     .await
        //     .map_err(|e| CsvUploadFailure::Unknown(e.to_string()))?
        //     .ok_or(CsvUploadFailure::NotFound)?;
        //
        // let worksite = todo!("update the worksite or whatever you like")
        //
        // self.worksite_repository
        //     .save(worksite)
        //     .await
        //     .map_err(|e| CsvUploadFailure::Unknown(e.to_string()))?;

        Ok(())
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum CsvUploadFailure {
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
