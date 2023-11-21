use std::sync::Arc;

use crate::{
    create_user::{CreateUser, CreateUserInput, CreateUserOutput},
    //##PLOP INSERT COMMAND IMPORTS HOOK##
    delete_user::{DeleteUser, DeleteUserInput, DeleteUserOutput},
    get_user_for_login::{GetUserForLogin, GetUserForLoginInput, GetUserForLoginOutput},
    get_users::{GetUsers, GetUsersOutput},
    ports::user_repository::UserRepository,
};

#[derive(Clone)]
pub struct AuthService {
    //##PLOP INSERT COMMAND HOOK##
    pub delete_user: DeleteUser,
    pub get_users: GetUsers,
    pub get_user_for_login: GetUserForLogin,
    pub create_user: CreateUser,
}

impl AuthService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self {
            //##PLOP INSERT COMMAND INSTANTIATION HOOK##
            delete_user: DeleteUser {
                user_repository: user_repository.clone(),
            },
            get_users: GetUsers {
                user_repository: user_repository.clone(),
            },
            get_user_for_login: GetUserForLogin {
                user_repository: user_repository.clone(),
            },
            create_user: CreateUser { user_repository },
        }
    }
    //##PLOP INSERT DELEGATE HOOK##
    pub async fn delete_user(&self, input: DeleteUserInput) -> DeleteUserOutput {
        self.delete_user.delete_user(input).await
    }

    pub async fn get_users(&self) -> GetUsersOutput {
        self.get_users.get_users().await
    }

    pub async fn get_user_for_login(&self, input: GetUserForLoginInput) -> GetUserForLoginOutput {
        self.get_user_for_login.get_user_for_login(input).await
    }

    pub async fn create_user(&self, input: CreateUserInput) -> CreateUserOutput {
        self.create_user.create_user(input).await
    }
}
