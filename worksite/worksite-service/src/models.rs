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

    pub fn get_workers_for_shift(&self, shift_id: String) -> Vec<Worker> {
        let shift = self.get_shift(shift_id);
        let shift = match shift {
            Some(shift) => shift,
            None => return vec![],
        };

        self.workers
            .iter()
            .filter(|worker| shift.worker_ids.contains(&worker.id))
            .cloned()
            .collect::<Vec<Worker>>()
    }

    fn get_shift(&self, shift_id: String) -> Option<Shift> {
        self.locations
            .iter()
            .flat_map(|location| location.shifts.clone())
            .find(|shift| shift.id == shift_id)
    }

    pub fn add_location(&self, location_name: String) -> Worksite {
        let mut updated_worksite = self.clone();

        let location = Location {
            id: uuid::Uuid::new_v4().to_string(),
            name: location_name,
            shifts: vec![],
        };

        updated_worksite.locations.push(location);

        updated_worksite
    }

    pub fn add_worker(&self, worker: Worker) -> Worksite {
        let mut updated_worksite = self.clone();

        updated_worksite.workers.push(worker);

        updated_worksite
    }

    // TODO! Should assign_worker take an owned worker?
    pub fn assign_worker(
        &self,
        worker: Worker,
        shift_id: String,
        _location_id: String,
    ) -> Worksite {
        let mut updated_worksite = self.clone();

        updated_worksite.locations.iter_mut().for_each(|location| {
            location.shifts.iter_mut().for_each(|shift| {
                if shift.id == shift_id {
                    shift.worker_ids.push(worker.id.clone())
                }
            })
        });

        updated_worksite
    }

    pub fn update_worker(
        &self,
        worker_id: String,
        update_fn: impl FnOnce(Worker) -> Worker,
    ) -> Worksite {
        let mut updated_worksite = self.clone();

        let worker = self.get_worker(worker_id.clone());

        match worker {
            Some(worker) => {
                let updated_worker = update_fn(worker);

                updated_worksite.workers.iter_mut().for_each(|worker| {
                    if worker.id == worker_id {
                        *worker = updated_worker.clone();
                    }
                });

                updated_worksite
            }
            None => updated_worksite,
        }
    }

    /**
     * Removes the given worker from the given shift.
     *
     * This function won't fail and will treat the worker/shift not existing as a trivial success.
     */
    pub fn remove_worker(&self, shift_id: String, worker_id: String) -> Worksite {
        let mut updated_worksite = self.to_owned();

        updated_worksite.locations.iter_mut().for_each(|location| {
            location.shifts.iter_mut().for_each(|shift| {
                if shift.id == shift_id {
                    shift.worker_ids.retain(|w_id| w_id != &worker_id)
                }
            })
        });

        updated_worksite
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
    pub worker_ids: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Worker {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub last_assessment: Option<Assessment>,
    pub tags: Vec<Tag>,
}

impl Worker {
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
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
