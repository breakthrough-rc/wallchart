use std::sync::Arc;

use thiserror::Error;

use crate::{models::Tag, ports::worksite_repository::WorksiteRepository};

#[derive(Clone)]
pub struct GetTags {
    pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct GetTagsInput {
    pub worksite_id: String,
}

// Change the return type, if needed
pub type GetTagsOutput = Result<Vec<Tag>, GetTagsFailure>;

impl GetTags {
    pub async fn get_tags(&self, input: GetTagsInput) -> GetTagsOutput {
        let worksite = self
            .worksite_repository
            .get_worksite(input.worksite_id)
            .await
            .map_err(|e| GetTagsFailure::Unknown(e.to_string()))?
            .ok_or(GetTagsFailure::NotFound)?;

        Ok(worksite.tags)
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum GetTagsFailure {
    #[error("Worksite does not exist")]
    NotFound,
    #[error("Something went wrong")]
    Unknown(String),
}
