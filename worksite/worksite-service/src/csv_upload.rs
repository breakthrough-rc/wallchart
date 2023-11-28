use std::{collections::HashMap, sync::Arc};

use serde::Deserialize;
use thiserror::Error;

use crate::{
    models::{Worksite, WorksiteName},
    ports::worksite_repository::WorksiteRepository,
};

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

#[allow(dead_code)]
impl CsvUpload {
    pub async fn csv_upload(&self, input: CsvUploadInput) -> CsvUploadOutput {
        let mut rdr = csv::Reader::from_reader(input.csv_input.as_bytes());
        let records: Vec<WorkerRecord> = rdr.deserialize().map(|result| result.unwrap()).collect();
        let worksites: HashMap<WorksiteName, Worksite> = HashMap::new();

        // TODO:
        // 1. Validate input and return helpful errors
        //

        for record in records.iter() {
            let _worksite = worksites
                .get(&record.worksite).cloned()
                .unwrap_or_else(|| Worksite::new(record.worksite.clone()));

            // TODO:
            // 1. Get or create location
            // 2. Get or create shifts
            // 3. Get or create worker
            // 4. Get or create shift assignments
            // 5. Insert the worksite back into the hashmap
        }

        // TODO:
        // 1. Insert all worksites into the database
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
