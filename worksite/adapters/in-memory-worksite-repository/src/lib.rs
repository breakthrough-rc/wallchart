use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;
use worksite_service::models::{Event, Worksite};
use worksite_service::ports::worksite_repository::{RepositoryFailure, WorksiteRepository};

#[derive(Clone, Debug)]
pub struct InMemoryWorksiteRepository {
    pub worksites: Arc<RwLock<Vec<Worksite>>>,
}

impl InMemoryWorksiteRepository {
    pub fn empty() -> Self {
        Self {
            worksites: Arc::new(RwLock::new(vec![])),
        }
    }

    pub fn with(worksites: Vec<Worksite>) -> Self {
        Self {
            worksites: Arc::new(RwLock::new(worksites)),
        }
    }
}

#[async_trait]
impl WorksiteRepository for InMemoryWorksiteRepository {
    async fn get_worksite(&self, id: String) -> Result<Option<Worksite>, RepositoryFailure> {
        let worksites = self.worksites.read().await;
        Ok(worksites.iter().find(|w| w.id == id).map(|w| w.to_owned()))
    }

    async fn save(&self, id: String, events: Vec<Event>) -> Result<(), RepositoryFailure> {
        let mut worksites = self.worksites.write().await;
        // Get the worksite if it already exists
        let maybe_worksite = &worksites.iter().find(|w| w.id == id);

        let maybe_first_event = &events.first();

        let first_event = match maybe_first_event {
            Some(first_event) => first_event,
            None => {
                // Implies no events were passed in. This is a no-op.
                // Alternatively, we could make this a failure.
                return Ok(());
            }
        };

        // Three cases here:
        // 1 - The worksite already exists, so just return it and all of the remaining events to be
        //   applied
        // 2 - The worksite does not exist but the first event creates the worksite, so create the
        //   new worksite and return the remaining events
        // 3 - The worksite does not exist and the first event is not creating the worksite =>
        //   error.
        let (worksite, remaining_events) = match maybe_worksite {
            Some(worksite) => (worksite.clone().to_owned(), events),
            None => {
                match first_event {
                    Event::WorksiteCreated { id, name } => (
                        Worksite {
                            id: id.to_owned(),
                            name: name.to_owned(),
                            locations: vec![],
                        },
                        events
                            .iter()
                            .skip(1)
                            .map(|e| e.to_owned())
                            .collect::<Vec<Event>>(),
                    ),
                    _ => {
                        // Implies the worksite doesn't exist yet, and the first event isn't a
                        // WorksiteCreated event. This is a failure.
                        return Err(RepositoryFailure::Unknown("Worksite does not exist".into()));
                    }
                }
            }
        };
        todo!()
        // worksites.retain(|w| w.id != id);
        // worksites.push(worksite.to_owned());
        // Ok(())
    }
}
