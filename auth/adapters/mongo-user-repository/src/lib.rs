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
    pub fn from_client(client: &mongodb::Client) -> Result<Self, mongodb::error::Error> {
        Ok(Self {
            collection: client.database("auth").collection::<UserRecord>("users"),
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

#[cfg(test)]
mod tests {
    use auth_service::ports::user_repository::UserRepository;
    use mongo_testcontainer::Mongo;
    use mongodb::Client;
    use pretty_assertions::assert_eq;
    use testcontainers::clients;

    use crate::MongoUserRepository;

    // I attempted to spin up a single static test container for the entirety of the tests
    // but the best I could do was something lke this which did work to spin up a single container
    // but it was not dropped at the end of the tests. I started looking into libs that improve
    // static functionality but it looked like it would have required a ton more trial and error.
    // So just falling back to this method of a single test calling other tests :(
    //
    // Keeping this code here just for reference so its visible (and so it enters version control
    // at least once)
    //
    // static mut DOCKER_CLI: Cli = OnceCell::const_new();
    // static mut CONTAINER: OnceCell<Container<'static, Mongo>> = OnceCell::const_new();
    // static mut CLIENT: OnceCell<Client> = OnceCell::const_new();
    //
    // async fn get_docker_cli() -> &'static Cli {
    //     DOCKER_CLI
    //         .get_or_init(|| async {
    //             let docker = clients::Cli::default();
    //             docker
    //         })
    //         .await
    // }
    // async fn get_container() -> &'static Container<'static, Mongo> {
    //     CONTAINER
    //         .get_or_init(|| async {
    //             let container = get_docker_cli().await.run(Mongo);
    //             container
    //         })
    //         .await
    // }
    // async fn get_client() -> &'static Client {
    //     CLIENT
    //         .get_or_init(|| async {
    //             let container = get_container().await;
    //             let host_port = container.get_host_port_ipv4(27017);
    //             let url = format!("mongodb://127.0.0.1:{host_port}/");
    //
    //             let client: Client = Client::with_uri_str(&url).await.unwrap();
    //
    //             client
    //         })
    //         .await
    // }

    #[tokio::test]
    async fn tests() {
        let docker_cli = clients::Cli::default();
        let container = docker_cli.run(Mongo);
        let host_port = container.get_host_port_ipv4(27017);
        let url = format!("mongodb://127.0.0.1:{host_port}/");
        let mongo_client: Client = Client::with_uri_str(&url).await.unwrap();
        let repo: MongoUserRepository = MongoUserRepository::from_client(&mongo_client).unwrap();

        test_create_and_fetch(&repo).await;
    }

    async fn test_create_and_fetch(repo: &MongoUserRepository) {
        let user = auth_service::models::User {
            id: "123".to_string(),
            email: "user@test.com".into(),
            hashed_password: "hashed_password".to_string(),
        };

        repo.save(user.clone()).await.unwrap();

        let result = repo.find_by_id(user.id.clone()).await.unwrap();
        assert_eq!(result, Some(user));
    }
}
