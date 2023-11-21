use std::sync::Arc;

use thiserror::Error;

use crate::ports::user_repository::UserRepository;

#[derive(Clone)]
pub struct DeleteUser {
    pub user_repository: Arc<dyn UserRepository>,
}

#[derive(Clone, Debug)]
pub struct DeleteUserInput {
    pub user_id: String,
}

// Change the return type, if needed
pub type DeleteUserOutput = Result<(), DeleteUserFailure>;

impl DeleteUser {
    pub async fn delete_user(&self, input: DeleteUserInput) -> DeleteUserOutput {
        self.user_repository
            .delete_by_id(input.user_id)
            .await
            .map_err(|e| DeleteUserFailure::Internal(e.to_string()))?;

        Ok(())
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum DeleteUserFailure {
    #[error("Oops")]
    Internal(String),
    #[error("Something went wrong")]
    Unknown(String),
}
