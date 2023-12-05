use std::sync::Arc;

use thiserror::Error;

use crate::ports::user_repository::UserRepository;

#[derive(Clone)]
pub struct UpdateUser {
    pub user_repository: Arc<dyn UserRepository>,
}

#[derive(Clone, Debug)]
pub struct UpdateUserInput {
    // Put input fields here
    pub user_id: String,
    pub email: String,
    pub role: String,
}

// Change the return type, if needed
pub type UpdateUserOutput = Result<(), UpdateUserFailure>;

impl UpdateUser {
    pub async fn update_user(&self, input: UpdateUserInput) -> UpdateUserOutput {
        let user = self
            .user_repository
            .find_by_email(input.email.clone())
            .await
            .map_err(|e| UpdateUserFailure::Internal(e.to_string()))?;

        let user = user
            .map(|u| u.update(input.email, input.role))
            .ok_or(UpdateUserFailure::NotFound)?;

        self.user_repository
            .save(user)
            .await
            .map_err(|e| UpdateUserFailure::Internal(e.to_string()))?;

        Ok(())
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum UpdateUserFailure {
    #[error("Internal Error")]
    Internal(String),
    #[error("Something went wrong")]
    Unknown(String),
    #[error("user does not exist")]
    NotFound,
}
