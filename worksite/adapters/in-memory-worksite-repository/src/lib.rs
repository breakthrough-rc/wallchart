use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;
use usecases::models::Worksite;
use usecases::ports::worksite_repository::{RepositoryFailure, WorksiteRepository};
use usecases::remove_worker_from_shift::Event;

#[derive(Clone, Debug)]
pub struct InMemoryWorksiteRepository {
    pub worksites: Arc<RwLock<Vec<Worksite>>>,
}

#[async_trait]
impl WorksiteRepository for InMemoryWorksiteRepository {
    async fn get_worksite(&self, id: String) -> Result<Option<Worksite>, RepositoryFailure> {
        let worksites = self.worksites.read().await;
        Ok(worksites.iter().find(|w| w.id == id).map(|w| w.to_owned()))
    }

    async fn save(
        &self,
        id: String,
        worksite: &Worksite,
        events: Vec<Event>,
    ) -> Result<(), RepositoryFailure> {
        let mut worksites = self.worksites.write().await;
        worksites.retain(|w| w.id != id);
        worksites.push(worksite.to_owned());
        Ok(())
    }
}
