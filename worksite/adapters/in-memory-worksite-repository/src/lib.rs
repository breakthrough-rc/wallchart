use std::sync::Arc;

use async_trait::async_trait;
use nonempty::NonEmpty;
use tokio::sync::RwLock;
use worksite_service::models::{Event, Location, Worker, Worksite};
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

    async fn save(&self, id: String, events: NonEmpty<Event>) -> Result<(), RepositoryFailure> {
        let mut worksites = self.worksites.write().await;

        let maybe_worksite = worksites.iter().find(|w| w.id == id);
        let (worksite, remaining_events) = get_or_create_worksite(maybe_worksite, events)?;
        let worksite = apply_events(worksite, remaining_events);

        worksites.retain(|w| w.id != id);
        worksites.push(worksite);

        Ok(())
    }
}

fn apply_events(worksite: Worksite, events: Vec<Event>) -> Worksite {
    events.iter().fold(worksite, apply_event)
}

fn apply_event(worksite: Worksite, event: &Event) -> Worksite {
    let ignore = worksite.clone();
    match event {
        Event::WorksiteCreated { id: _, name: _ } => ignore,
        Event::LocationAdded { id, name } => {
            let location = Location {
                id: id.clone(),
                name: name.clone(),
                shifts: vec![],
            };

            Worksite {
                locations: vec![vec![location], worksite.locations].concat(),
                ..worksite
            }
        }

        Event::ShiftAdded {
            id: _,
            location_id: _,
            name: _,
        } => todo!(),

        Event::WorkerCreated {
            id,
            first_name,
            last_name,
        } => {
            let worker = Worker {
                id: id.to_owned(),
                first_name: first_name.to_owned(),
                last_name: last_name.to_owned(),
                last_assessment: None,
                tags: vec![],
            };

            let (state, _) = worksite.add_worker(worker);
            state
        }

        Event::WorkerUpdated {
            id,
            first_name,
            last_name,
        } => {
            let (state, _) = worksite.update_worker(id.to_owned(), |w| Worker {
                first_name: first_name.to_owned(),
                last_name: last_name.to_owned(),
                ..w
            });
            state
        }

        Event::ShiftAssigned {
            shift_id,
            worker_id,
            location_id,
        } => {
            let worker = worksite.get_worker(worker_id.to_owned()).unwrap();

            let (state, _) =
                worksite.assign_worker(worker, shift_id.to_owned(), location_id.to_owned());
            state
        }

        Event::ShiftUnassigned {
            shift_id,
            worker_id,
        } => {
            let (state, _) = worksite.remove_worker(shift_id.to_owned(), worker_id.to_owned());
            state
        }
    }
}

fn get_or_create_worksite(
    maybe_worksite: Option<&Worksite>,
    events: NonEmpty<Event>,
) -> Result<(Worksite, Vec<Event>), RepositoryFailure> {
    // Three cases here:
    // 1 - The worksite already exists, so just return it and all of the remaining events to be
    //   applied
    // 2 - The worksite does not exist but the first event creates the worksite, so create the
    //   new worksite and return the remaining events
    // 3 - The worksite does not exist and the first event is not creating the worksite =>
    //   error.
    let (worksite, remaining_events) = match maybe_worksite {
        Some(worksite) => (worksite.clone(), events.into()),
        None => {
            match events.first() {
                Event::WorksiteCreated { id, name } => (
                    Worksite {
                        id: id.to_owned(),
                        name: name.to_owned(),
                        locations: vec![],
                        workers: vec![],
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
    Ok((worksite, remaining_events))
}
