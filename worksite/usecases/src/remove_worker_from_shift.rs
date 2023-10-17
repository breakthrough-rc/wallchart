use std::sync::Arc;

use thiserror::Error;

use crate::models::{Location, Shift, Worksite};
use crate::ports::worksite_repository::WorksiteRepository;

pub struct RemoveWorkerFromShift {
    worksite_repository: Arc<dyn WorksiteRepository>,
}

impl RemoveWorkerFromShift {
    async fn remove_worker_from_shift(
        &self,
        id: String,
        shift_id: String,
        worker_id: String,
    ) -> Result<Worksite, GetWorksiteFailure> {
        let worksite = self
            .worksite_repository
            .get_worksite(id.clone())
            .await
            .map_err(|e| GetWorksiteFailure::Unknown(e.to_string()))?;

        let (updated_worksite, events) = remove_worker(&worksite, shift_id, worker_id);

        self.worksite_repository
            .save(id, events)
            .await
            .map_err(|e| GetWorksiteFailure::Unknown(e.to_string()))?;

        Ok(updated_worksite)
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum GetWorksiteFailure {
    #[error("Something went wrong")]
    Unknown(String),
}

#[derive(Debug, PartialEq)]
pub enum Event {
    WorkerRemoved(WorkerRemovedData),
}

#[derive(Debug, PartialEq)]
pub struct WorkerRemovedData {
    worker_id: String,
    shift_id: String,
}

/**
* Removes the given worker from the given shift.
*
* This function won't fail and will treat the worker/shift not existing as a trivial success.
*/
pub fn remove_worker(
    worksite: &Worksite,
    shift_id: String,
    worker_id: String,
) -> (Worksite, Vec<Event>) {
    let mut updated_worksite = worksite.to_owned();

    updated_worksite.locations.iter_mut().for_each(|location| {
        location.shifts.iter_mut().for_each(|shift| {
            if shift.id == shift_id {
                shift.workers.retain(|worker| worker.id != worker_id)
            }
        })
    });

    (
        updated_worksite,
        vec![Event::WorkerRemoved(WorkerRemovedData {
            worker_id,
            shift_id,
        })],
    )
}
