use std::sync::Arc;

use async_trait::async_trait;
use thiserror::Error;

pub struct GetWorksite {
    worksite_repository: Arc<dyn WorksiteRepository>,
}

impl GetWorksite {
    async fn get_worksite(&self, id: String) -> Result<Worksite, GetWorksiteFailure> {
        self.worksite_repository
            .get_worksite(id)
            .await
            .map_err(|e| GetWorksiteFailure::Unknown(e.to_string()))
    }
}

#[async_trait]
pub trait WorksiteRepository {
    async fn get_worksite(&self, id: String) -> Result<Worksite, RepositoryFailure>;
}

#[derive(Error, Debug, PartialEq)]
pub enum RepositoryFailure {
    #[error("Something went wrong")]
    Unknown(String),
}

#[derive(Error, Debug, PartialEq)]
pub enum GetWorksiteFailure {
    #[error("Something went wrong")]
    Unknown(String),
}

pub struct Worksite {
    pub id: String,
    pub name: String,
    pub locations: Vec<Location>,
}

pub struct Location {
    pub id: String,
    pub name: String,
    pub shifts: Vec<Shift>,
}

pub struct Shift {
    pub id: String,
    pub name: String,
    pub workers: Vec<Worker>,
}

pub struct Worker {
    pub id: String,
    pub name: String,
    pub last_assessment: Assessment,
    pub tags: Vec<Tag>,
}

pub struct Assessment {
    pub id: String,
    pub value: u8,
}

pub struct Tag {
    pub id: String,
    pub name: String,
    pub icon: String,
}
