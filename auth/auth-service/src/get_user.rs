use std::sync::Arc;

use thiserror::Error;

use crate::{models::User, ports::user_repository::UserRepository};

#[derive(Clone)]
pub struct GetUser {
    pub user_repository: Arc<dyn UserRepository>,
}

#[derive(Clone, Debug)]
pub struct GetUserInput {
    pub user_id: String,
}

pub type GetUserOutput = Result<Option<User>, GetUserFailure>;

impl GetUser {
    pub async fn get_user(&self, input: GetUserInput) -> GetUserOutput {
        let user = self
            .user_repository
            .find_by_id(input.user_id)
            .await
            .map_err(|e| GetUserFailure::Internal(e.to_string()))?;

        Ok(user)
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum GetUserFailure {
    #[error("Internal Error")]
    Internal(String),
    #[error("Something went wrong")]
    Unknown(String),
}
