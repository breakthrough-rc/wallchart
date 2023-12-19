use std::sync::Arc;

use thiserror::Error;

use crate::{models::UserRole, ports::user_repository::UserRepository};

#[derive(Clone)]
pub struct UpdateUser {
    pub user_repository: Arc<dyn UserRepository>,
}

#[derive(Clone, Debug)]
pub struct UpdateUserInput {
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

        let role =
            UserRole::new(&input.role).ok_or(UpdateUserFailure::UnknownRole(input.role.clone()))?;

        let user = user
            .map(|u| u.update(input.email, role))
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
    #[error("UserRole not recongized")]
    UnknownRole(String),
    #[error("Something went wrong")]
    Unknown(String),
    #[error("user does not exist")]
    NotFound,
}
