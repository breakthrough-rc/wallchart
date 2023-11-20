use std::sync::Arc;

use thiserror::Error;

use crate::{models::Tag, ports::worksite_repository::WorksiteRepository};

#[derive(Clone)]
pub struct GetTag {
    pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct GetTagInput {
    // Put input fields here
    pub worksite_id: String,
    pub tag_id: String,
}

// Change the return type, if needed
pub type GetTagOutput = Result<Option<Tag>, GetTagFailure>;

impl GetTag {
    pub async fn get_tag(&self, input: GetTagInput) -> GetTagOutput {
        let worksite = self
            .worksite_repository
            .get_worksite(input.worksite_id)
            .await
            .map_err(|e| GetTagFailure::Unknown(e.to_string()))?
            .ok_or(GetTagFailure::NotFound)?;

        Ok(worksite.get_tag(input.tag_id))
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum GetTagFailure {
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
