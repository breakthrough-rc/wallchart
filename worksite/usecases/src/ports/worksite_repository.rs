use async_trait::async_trait;
use thiserror::Error;

use crate::{models::Worksite, remove_worker_from_shift::Event};

#[async_trait]
pub trait WorksiteRepository {
    async fn get_worksite(&self, id: String) -> Result<Option<Worksite>, RepositoryFailure>;
    /**
     * NOTE: temporarily, we save the entire worksite as a snapshot, just to save time
     * when experimenting. If we decided to go the events route, we can and should remove this
     * and instead maybe just reconstitute the aggregate based on the events later.
     *
     * But since I'm currently using mocked data, all the events don't exist yet, so its
     * easiest just to save the current state and return it on demand.
     *
     * Will also need to think if it makes sense to "hide" the apply function definition in the
     * repository adapters, or if it should be part of the domain.
     */
    async fn save(
        &self,
        id: String,
        worksite: &Worksite,
        events: Vec<Event>,
    ) -> Result<(), RepositoryFailure>;
}

#[derive(Error, Debug, PartialEq)]
pub enum RepositoryFailure {
    #[error("Something went wrong")]
    Unknown(String),
}
