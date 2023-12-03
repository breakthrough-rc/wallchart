use std::sync::Arc;

use async_trait::async_trait;
use auth_service::models::User;
use auth_service::ports::user_repository::{RepositoryFailure, UserRepository};
use axum_login::{AuthnBackend, UserId};
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct UserRecord {
    pub id: String,
    pub email: String,
    pub hashed_password: String,
}

impl UserRecord {
    pub fn to_user(&self) -> User {
        User {
            id: self.id.clone(),
            email: self.email.clone(),
            hashed_password: self.hashed_password.clone(),
        }
    }
}

fn to_user_record(user: &User) -> UserRecord {
    UserRecord {
        id: user.id.clone(),
        email: user.email.clone(),
        hashed_password: user.hashed_password.clone(),
    }
}

#[derive(Clone, Debug)]
pub struct MongoUserRepository {
    collection: mongodb::Collection<UserRecord>,
}

// let url = format!("mongodb://127.0.0.1:{host_port}/");
//
// let client: Client = Client::with_uri_str(&url).await.unwrap();
// let db = client.database("some_db");
// let coll = db.collection("some-coll");
//
impl MongoUserRepository {
    pub async fn new(url: &String) -> Result<Self, mongodb::error::Error> {
        Ok(Self {
            collection: mongodb::Client::with_uri_str(url)
                .await?
                .database("auth")
                .collection::<UserRecord>("users"),
        })
    }
}

#[async_trait]
impl UserRepository for MongoUserRepository {
    async fn find_by_id(&self, id: String) -> Result<Option<User>, RepositoryFailure> {
        let filter = doc! { "id": id };
        let maybe_user = self
            .collection
            .find_one(filter, None)
            .await
            .map_err(|e| RepositoryFailure::Unknown(e.to_string()))?;

        Ok(maybe_user.map(|u| u.to_user()))
    }

    async fn get_users(&self) -> Result<Vec<User>, RepositoryFailure> {
        let cursor = self
            .collection
            // Get all of the users
            .find(None, None)
            .await
            .map_err(|e| RepositoryFailure::Unknown(e.to_string()))?;

        let users: Vec<UserRecord> = cursor
            .try_collect()
            .await
            .map_err(|e| RepositoryFailure::Unknown(e.to_string()))?;

        Ok(users.iter().map(|u| u.to_user()).collect())
    }

    async fn find_by_email(&self, email: String) -> Result<Option<User>, RepositoryFailure> {
        let filter = doc! { "email": email };
        let maybe_user = self
            .collection
            .find_one(filter, None)
            .await
            .map_err(|e| RepositoryFailure::Unknown(e.to_string()))?;

        Ok(maybe_user.map(|u| u.to_user()))
    }

    async fn save(&self, user: User) -> Result<(), RepositoryFailure> {
        let record = to_user_record(&user);
        self.collection
            .insert_one(record, None)
            .await
            .map_err(|e| RepositoryFailure::Unknown(e.to_string()))?;

        Ok(())
    }

    async fn delete_by_id(&self, id: String) -> Result<(), RepositoryFailure> {
        let filter = doc! { "id": id };
        self.collection
            .delete_one(filter, None)
            .await
            .map_err(|e| RepositoryFailure::Unknown(e.to_string()))?;

        Ok(())
    }
}

#[derive(Clone)]
pub struct MongoUserStore {
    pub users: Arc<MongoUserRepository>,
}

#[derive(Clone)]
pub struct Credentials {
    pub user_id: String,
}

/**
* Also implement the UserStore trait from the auth_service crate.
*/
#[async_trait]
impl AuthnBackend for MongoUserStore {
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
