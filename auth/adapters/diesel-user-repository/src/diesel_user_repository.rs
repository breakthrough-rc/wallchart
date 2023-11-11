use async_trait::async_trait;
use db::db::PgPool;
use db::schema::*;
use diesel::prelude::*;
use diesel::OptionalExtension;
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::RunQueryDsl;

use auth_service::ports::user_repository::{RepositoryFailure, UserRepository};
use auth_service::models::User;
use crate::records::*;

#[derive(Clone)]
pub struct DieselUserRepository {
    pub pg_pool: PgPool,
}

#[async_trait]
impl UserRepository for DieselUserRepository {
    async fn get_user(&self, id: String) -> Result<Option<User>, RepositoryFailure> {
        let conn = &mut self
            .pg_pool
            .get()
            .await
            .map_err(|_| RepositoryFailure::FailedToGetConnectionFromPool)?;

        todo!("Fetch record and map to domain model")

    }

    async fn save(&self, user: User) -> Result<(), RepositoryFailure> {
        let conn = &mut self
            .pg_pool
            .get()
            .await
            .map_err(|_| RepositoryFailure::FailedToGetConnectionFromPool)?;
        
        todo!("Save record to database")
    }
}
