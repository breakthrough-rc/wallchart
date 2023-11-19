use std::sync::Arc;

use thiserror::Error;

use crate::{models::User, ports::user_repository::UserRepository};

// Example repo dependency
// use crate::ports::worksite_repository::WorksiteRepository;

#[derive(Clone)]
pub struct GetUsers {
    pub user_repository: Arc<dyn UserRepository>,
}

// Change the return type, if needed
pub type GetUsersOutput = Result<Vec<User>, GetUsersFailure>;

impl GetUsers {
    pub async fn get_users(&self) -> GetUsersOutput {
        self.user_repository
            .get_users()
            .await
            .map_err(|e| GetUsersFailure::Unknown(e.to_string()))
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum GetUsersFailure {
    #[error("Something went wrong")]
    Unknown(String),
}
