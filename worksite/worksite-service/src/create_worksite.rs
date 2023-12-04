use std::sync::Arc;

use thiserror::Error;

use crate::{models::Worksite, ports::worksite_repository::WorksiteRepository};

#[derive(Clone)]
pub struct CreateWorksite {
    pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct CreateWorksiteInput {
    // Put input fields here
    pub worksite_name: String,
}

// Change the return type, if needed
pub type CreateWorksiteOutput = Result<Worksite, CreateWorksiteFailure>;

impl CreateWorksite {
    pub async fn create_worksite(&self, input: CreateWorksiteInput) -> CreateWorksiteOutput {
        let worksite: Worksite = Worksite {
            id: uuid::Uuid::new_v4().to_string(),
            name: input.worksite_name,
            locations: vec![],
            tags: vec![],
            workers: vec![],
        };

        self.worksite_repository
            .save(worksite.clone())
            .await
            .map_err(|e| CreateWorksiteFailure::Unknown(e.to_string()))?;

        Ok(worksite)
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum CreateWorksiteFailure {
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
