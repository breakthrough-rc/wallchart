use std::sync::Arc;

use thiserror::Error;

use crate::{models::Worker, ports::worksite_repository::WorksiteRepository};

#[derive(Clone)]
pub struct FilterWorkers {
    pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct FilterWorkersInput {
    pub worksite_id: String,
    pub filter: String,
}

// Change the return type, if needed
pub type FilterWorkersOutput = Result<Vec<Worker>, FilterWorkersFailure>;

impl FilterWorkers {
    pub async fn filter_workers(&self, input: FilterWorkersInput) -> FilterWorkersOutput {
        let worksite = self
            .worksite_repository
            .get_worksite(input.worksite_id.clone())
            .await
            .map_err(|e| FilterWorkersFailure::Unknown(e.to_string()))?
            .ok_or(FilterWorkersFailure::NotFound)?;

        let filtered_workers:Vec<Worker> = 
            worksite.workers.iter()                     
            .filter(|worker| worker.matches_filter(&input.filter.to_lowercase())).cloned()
            .collect();
        Ok(filtered_workers)
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum FilterWorkersFailure {
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
