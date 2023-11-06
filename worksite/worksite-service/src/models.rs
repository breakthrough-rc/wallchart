use nonempty::{nonempty, NonEmpty};

#[derive(Debug, Clone)]
pub struct Worksite {
    pub id: String,
    pub name: String,
    pub locations: Vec<Location>,
    pub workers: Vec<Worker>,
}

impl Worksite {
    pub fn get_worker(&self, worker_id: String) -> Option<Worker> {
        self.workers.iter().find(|w| w.id == worker_id).cloned()
    }

    pub fn add_worker(&self, worker: Worker) -> (Worksite, NonEmpty<Event>) {
        let mut updated_worksite = self.clone();

        updated_worksite.workers.push(worker.clone());

        (
            updated_worksite,
            nonempty![Event::WorkerCreated {
                id: worker.id,
                name: worker.name,
            }],
        )
    }

    // TODO! Should assign_worker take an owned worker?
    pub fn assign_worker(
        &self,
        worker: Worker,
        shift_id: String,
        location_id: String,
    ) -> (Worksite, NonEmpty<Event>) {
        let mut updated_worksite = self.clone();

        updated_worksite.locations.iter_mut().for_each(|location| {
            location.shifts.iter_mut().for_each(|shift| {
                if shift.id == shift_id {
                    shift.workers.push(worker.clone())
                }
            })
        });

        let worker_id = worker.id;

        // TODO! Reconsider the name of the event.
        (
            updated_worksite,
            nonempty![Event::ShiftAssigned {
                shift_id,
                worker_id: worker_id.clone(),
                location_id,
            }],
        )
    }

    /**
     * Removes the given worker from the given shift.
     *
     * This function won't fail and will treat the worker/shift not existing as a trivial success.
     */
    pub fn remove_worker(
        &self,
        shift_id: String,
        worker_id: String,
    ) -> (Worksite, NonEmpty<Event>) {
        let mut updated_worksite = self.to_owned();

        updated_worksite.locations.iter_mut().for_each(|location| {
            location.shifts.iter_mut().for_each(|shift| {
                if shift.id == shift_id {
                    shift.workers.retain(|worker| worker.id != worker_id)
                }
            })
        });

        (
            updated_worksite,
            nonempty![Event::ShiftUnassigned {
                shift_id,
                worker_id,
            }],
        )
    }
}

#[derive(Debug, Clone)]
pub struct Location {
    pub id: String,
    pub name: String,
    pub shifts: Vec<Shift>,
}

#[derive(Debug, Clone)]
pub struct Shift {
    pub id: String,
    pub name: String,
    pub workers: Vec<Worker>,
}

#[derive(Debug, Clone)]
pub struct Worker {
    pub id: String,
    pub name: String,
    pub last_assessment: Option<Assessment>,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone)]
pub struct Assessment {
    pub id: String,
    pub value: u8,
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub icon: String,
}

#[derive(Debug, Clone)]
pub enum Event {
    WorksiteCreated {
        id: String,
        name: String,
    },
    LocationAdded {
        id: String,
        name: String,
    },
    ShiftAdded {
        id: String,
        location_id: String,
        name: String,
    },
    WorkerCreated {
        id: String,
        name: String,
    },
    ShiftAssigned {
        shift_id: String,
        worker_id: String,
        location_id: String,
    },
    ShiftUnassigned {
        shift_id: String,
        worker_id: String,
    },
}
