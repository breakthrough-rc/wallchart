use std::sync::Arc;

use password_auth::generate_hash;
use thiserror::Error;

use crate::{models::User, ports::user_repository::UserRepository};

#[derive(Clone)]
pub struct CreateUser {
    pub user_repository: Arc<dyn UserRepository>,
}

#[derive(Clone, Debug)]
pub struct CreateUserInput {
    pub email: String,
    pub password: String,
}

pub type CreateUserOutput = Result<User, CreateUserFailure>;

impl CreateUser {
    pub async fn create_user(&self, input: CreateUserInput) -> CreateUserOutput {
        // Fail if username is already in use
        self.user_repository
            .find_by_email(input.email.clone())
            .await
            .map_err(|e| CreateUserFailure::Internal(e.to_string()))
            .and_then(|user| match user {
                Some(_) => Err(CreateUserFailure::UserAlreadyExists(input.email.clone())),
                None => Ok(()),
            })?;

        // Salt and hash the pw
        let hashed_password = generate_hash(input.password);

        let new_user = User {
            id: uuid::Uuid::new_v4().to_string(),
            email: input.email,
            hashed_password,
            role: "Organizer".to_string(),
        };
        self.user_repository
            .save(new_user.clone())
            .await
            .map_err(|e| CreateUserFailure::Internal(e.to_string()))?;

        Ok(new_user)
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum CreateUserFailure {
    #[error("User already exists!")]
    UserAlreadyExists(String),
    #[error("Internal Error")]
    Internal(String),
    #[error("Something went wrong")]
    Unknown(String),
}
