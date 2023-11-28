use std::sync::Arc;

use serde::Deserialize;
use thiserror::Error;

use crate::ports::worksite_repository::WorksiteRepository;

#[derive(Clone)]
pub struct CsvUpload {
    pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct CsvUploadInput {
    /*
     * The stringified CSV content. We will attempt to deserialize this into
     *  worker records
     */
    pub csv_input: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
pub struct WorkerRecord {
    worksite: String,
    location: String,
    shift1: String,
    shift2: String,
    shift3: String,
    name: String,
    email: String,
    phone: String,
}

// Change the return type, if needed
pub type CsvUploadOutput = Result<Vec<WorkerRecord>, CsvUploadFailure>;

impl CsvUpload {
    pub async fn csv_upload(&self, input: CsvUploadInput) -> CsvUploadOutput {
        let mut rdr = csv::Reader::from_reader(input.csv_input.as_bytes());
        let records: Vec<WorkerRecord> = rdr.deserialize().map(|result| result.unwrap()).collect();

        Ok(records)
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum CsvUploadFailure {
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
