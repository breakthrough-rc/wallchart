use std::sync::Arc;

use async_trait::async_trait;
use auth_service::models::User;
use auth_service::ports::user_repository::{RepositoryFailure, UserRepository};
use axum_login::UserStore;
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
/**
* Also implement the UserStore trait from the auth_service crate.
*/
#[async_trait]
impl UserStore<String, ()> for InMemoryUserStore {
    type User = User;
    type Error = RepositoryFailure;
    async fn load_user(&self, user_id: &String) -> Result<Option<Self::User>, Self::Error> {
        self.users.get_user(user_id.to_owned()).await
    }
}

/**
 * Provide a couple required types that are tight coupled to the DB implementation
*  We can probably find a better way to implement this but for now just sticking it here.
 */
pub type AuthContext = axum_login::extractors::AuthContext<String, User, InMemoryUserStore>;
