use async_trait::async_trait;
use db::db::PgPool;
use usecases::{
    models::Worksite,
    ports::worksite_repository::{RepositoryFailure, WorksiteRepository},
    remove_worker_from_shift::Event,
};

#[derive(Clone)]
pub struct DieselWorksiteRepository {
    pub pg_pool: PgPool,
}

#[async_trait]
impl WorksiteRepository for DieselWorksiteRepository {
    async fn get_worksite(&self, id: String) -> Result<Option<Worksite>, RepositoryFailure> {
        todo!()
    }
    async fn save(&self, id: String, worksite: &Worksite) -> Result<(), RepositoryFailure> {
        todo!()
    }
}
