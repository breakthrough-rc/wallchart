use std::sync::Arc;

use thiserror::Error;

use crate::{models::Worksite, ports::worksite_repository::WorksiteRepository};

#[derive(Clone)]
pub struct GetWorksites {
    pub worksite_repository: Arc<dyn WorksiteRepository>,
}

// Change the return type, if needed
pub type GetWorksitesOutput = Result<Vec<Worksite>, GetWorksitesFailure>;

impl GetWorksites {
    pub async fn get_worksites(&self) -> GetWorksitesOutput {
        let worksites = self
            .worksite_repository
            .get_all()
            .await
            .map_err(|e| GetWorksitesFailure::Unknown(e.to_string()))?;

        Ok(worksites)
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum GetWorksitesFailure {
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
