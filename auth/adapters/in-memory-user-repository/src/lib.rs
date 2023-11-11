use std::sync::Arc;

use async_trait::async_trait;
use auth_service::models::User;
use auth_service::ports::user_repository::{RepositoryFailure, UserRepository};
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
    async fn get_user(&self, id: String) -> Result<Option<User>, RepositoryFailure> {
        let users = self.users.read().await;
        Ok(users.iter().find(|u| u.id == id).map(|u| u.to_owned()))
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
}
