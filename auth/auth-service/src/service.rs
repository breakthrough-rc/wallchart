use std::sync::Arc;

use crate::{
    //##PLOP INSERT COMMAND IMPORTS HOOK##
    get_user_for_login::{
      GetUserForLogin, GetUserForLoginInput, GetUserForLoginOutput, 
    },
    create_user::{
      CreateUser, CreateUserInput, CreateUserOutput, 
    },
};

#[derive(Clone)]
pub struct AuthService {
    //##PLOP INSERT COMMAND HOOK##
    pub get_user_for_login: GetUserForLogin,
    pub create_user: CreateUser,
    // Add service infra dependencies here
}

impl AuthService {
    pub fn new() -> Self {
        Self {
            //##PLOP INSERT COMMAND INSTANTIATION HOOK##
            get_user_for_login: GetUserForLogin {
              // Add any dependencies for the command here. They should be passed into this function and supplied by main.rs.
            },
            create_user: CreateUser {
              // Add any dependencies for the command here. They should be passed into this function and supplied by main.rs.
            },
        }
    }
    //##PLOP INSERT DELEGATE HOOK##
    pub async fn get_user_for_login(
        &self,
        input: GetUserForLoginInput,
    ) -> GetUserForLoginOutput {
        self.get_user_for_login.get_user_for_login(input).await
    }

    pub async fn create_user(
        &self,
        input: CreateUserInput,
    ) -> CreateUserOutput {
        self.create_user.create_user(input).await
    }

}
