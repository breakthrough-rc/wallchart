use std::sync::Arc;

use thiserror::Error;

// Example repo dependency
// use crate::ports::worksite_repository::WorksiteRepository;

#[derive(Clone)]
pub struct CreateUser {
    // Put infra dependencies in this struct
    // Below is an example of a repo dependency
    // pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct CreateUserInput {
    // Put input fields here
    pub id: String
}

// Change the return type, if needed
pub type CreateUserOutput = Result<(), CreateUserFailure>;

impl CreateUser {
    pub async fn create_user(&self, input: CreateUserInput) -> CreateUserOutput {
        todo!("Implement create_user")
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum CreateUserFailure {
    #[error("Something went wrong")]
    Unknown(String),
}
