use std::sync::Arc;

use async_trait::async_trait;
use auth_service::models::User;
use auth_service::ports::user_repository::{RepositoryFailure, UserRepository};
use axum_login::{AuthnBackend, UserId};
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub struct InMemoryUserRepository {
    pub users: Arc<RwLock<Vec<User>>>,
}

impl InMemoryUserRepository {
    pub fn empty() -> Self {
        Self {
            users: Arc::new(RwLock::new(vec![])),
        }
    }

    pub fn with(users: Vec<User>) -> Self {
        Self {
            users: Arc::new(RwLock::new(users)),
        }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn find_by_id(&self, id: String) -> Result<Option<User>, RepositoryFailure> {
        let users = self.users.read().await;
        Ok(users.iter().find(|u| u.id == id).map(|u| u.to_owned()))
    }

    async fn get_users(&self) -> Result<Vec<User>, RepositoryFailure> {
        let users = self.users.read().await;
        Ok(users.to_vec())
    }

    async fn find_by_email(&self, email: String) -> Result<Option<User>, RepositoryFailure> {
        let users = self.users.read().await;
        Ok(users
            .iter()
            .find(|u| u.email == email)
            .map(|u| u.to_owned()))
    }

    async fn save(&self, user: User) -> Result<(), RepositoryFailure> {
        let mut users = self.users.write().await;

        users.retain(|w| w.id != user.id);
        users.push(user.to_owned());

        Ok(())
    }

    async fn delete_by_id(&self, id: String) -> Result<(), RepositoryFailure> {
        let mut users = self.users.write().await;

        users.retain(|w| w.id != id);

        Ok(())
    }
}

#[derive(Clone)]
pub struct InMemoryUserStore {
    pub users: Arc<InMemoryUserRepository>,
}

#[derive(Clone)]
pub struct Credentials {
    pub user_id: String,
}

/**
* Also implement the UserStore trait from the auth_service crate.
*/
#[async_trait]
impl AuthnBackend for InMemoryUserStore {
    type User = User;
    type Credentials = Credentials;
    type Error = RepositoryFailure;

    async fn authenticate(
        &self,
        Credentials { user_id }: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        self.get_user(&user_id).await
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        self.users.find_by_id(user_id.to_string()).await
    }
}

// pub type AuthContext = axum_login::extractors::AuthContext<String, User, InMemoryUserStore>;
