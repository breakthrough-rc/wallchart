use std::sync::Arc;

use password_auth::verify_password;
use thiserror::Error;

use crate::{models::User, ports::user_repository::UserRepository};

#[derive(Clone)]
pub struct GetUserForLogin {
    pub user_repository: Arc<dyn UserRepository>,
}

#[derive(Clone, Debug)]
pub struct GetUserForLoginInput {
    pub email: String,
    pub password: String,
}

// Change the return type, if needed
pub type GetUserForLoginOutput = Result<User, GetUserForLoginFailure>;

impl GetUserForLogin {
    pub async fn get_user_for_login(&self, input: GetUserForLoginInput) -> GetUserForLoginOutput {
        let user = self
            .user_repository
            .find_by_email(input.email)
            .await
            .map_err(|e| GetUserForLoginFailure::Internal(e.to_string()))?;

        match user {
            Some(user) => verify_password(input.password, &user.hashed_password)
                .map(|_| user)
                .map_err(|_| GetUserForLoginFailure::WrongPassword),
            None => Err(GetUserForLoginFailure::UserNotFound),
        }
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum GetUserForLoginFailure {
    #[error("User does not exist!")]
    UserNotFound,
    #[error("Wrong password!")]
    WrongPassword,
    #[error("Internal Error")]
    Internal(String),
    #[error("Something went wrong")]
    Unknown(String),
}
