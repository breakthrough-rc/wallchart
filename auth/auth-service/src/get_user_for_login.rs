use std::sync::Arc;

use thiserror::Error;

// Example repo dependency
// use crate::ports::worksite_repository::WorksiteRepository;

#[derive(Clone)]
pub struct GetUserForLogin {
    // Put infra dependencies in this struct
    // Below is an example of a repo dependency
    // pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct GetUserForLoginInput {
    // Put input fields here
    pub id: String
}

// Change the return type, if needed
pub type GetUserForLoginOutput = Result<(), GetUserForLoginFailure>;

impl GetUserForLogin {
    pub async fn get_user_for_login(&self, input: GetUserForLoginInput) -> GetUserForLoginOutput {
        todo!("Implement get_user_for_login")
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum GetUserForLoginFailure {
    #[error("Something went wrong")]
    Unknown(String),
}
