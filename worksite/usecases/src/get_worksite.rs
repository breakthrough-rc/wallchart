use std::sync::Arc;

use thiserror::Error;

use crate::models::Worksite;
use crate::ports::worksite_repository::WorksiteRepository;

pub struct GetWorksite {
    worksite_repository: Arc<dyn WorksiteRepository>,
}

impl GetWorksite {
    async fn get_worksite(&self, id: String) -> Result<Option<Worksite>, GetWorksiteFailure> {
        self.worksite_repository
            .get_worksite(id)
            .await
            .map_err(|e| GetWorksiteFailure::Unknown(e.to_string()))
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum GetWorksiteFailure {
    #[error("Something went wrong")]
    Unknown(String),
}
