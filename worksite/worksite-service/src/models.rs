#[derive(Debug, Clone)]
pub struct Worksite {
    pub id: String,
    pub name: String,
    pub locations: Vec<Location>,
    pub tags: Vec<Tag>,
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
            .filter(|worker| shift.contains_worker(worker))
            .cloned()
            .collect::<Vec<Worker>>()
    }

    pub fn get_tag(&self, tag_id: String) -> Option<Tag> {
        self.tags.iter().find(|t| t.id == tag_id).cloned()
    }

    pub fn get_tags_for_worker(&self, worker: Worker) -> Vec<Tag> {
        let mut tags = vec![];

        for tag in &self.tags {
            if worker.tags.iter().any(|t| t.0 == tag.id) {
                tags.push(tag.clone());
            }
        }

        tags
    }

    pub fn get_assessments_for_worker(&self, worker_id: String) -> Vec<Assessment> {
        match self.get_worker(worker_id) {
            Some(worker) => worker.assessments,
            None => vec![],
        }
    }

    fn get_shift(&self, shift_id: String) -> Option<Shift> {
        self.locations
            .iter()
            .flat_map(|location| location.shifts.clone())
            .find(|shift| shift.id == shift_id)
    }

    pub fn add_shift(&self, location_id: String, shift_name: String) -> Option<Worksite> {
        let mut updated_worksite = self.clone();

        let shift = Shift {
            id: uuid::Uuid::new_v4().to_string(),
            name: shift_name,
            workers: vec![],
        };

        let location = updated_worksite
            .locations
            .iter_mut()
            .find(|l| l.id == location_id)?;

        location.shifts.push(shift);

        Some(updated_worksite)
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

    pub fn add_tag(&self, tag: Tag) -> Worksite {
        let mut updated_worksite = self.clone();
        updated_worksite.tags.push(tag);

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
                    *shift = shift.assign_worker(&worker);
                }
            })
        });

        updated_worksite
    }

    pub fn update_tag(&self, tag_id: String, update_fn: impl FnOnce(Tag) -> Tag) -> Worksite {
        let mut updated_worksite = self.clone();

        let tag = self.get_tag(tag_id.clone());

        match tag {
            Some(tag) => {
                let updated_tag = update_fn(tag);

                updated_worksite.tags.iter_mut().for_each(|tag| {
                    if tag.id == tag_id {
                        *tag = updated_tag.clone();
                    }
                });

                updated_worksite
            }
            None => updated_worksite,
        }
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
    pub fn remove_worker(&self, shift_id: String, worker: Worker) -> Worksite {
        let mut updated_worksite = self.to_owned();

        updated_worksite.locations.iter_mut().for_each(|location| {
            location.shifts.iter_mut().for_each(|shift| {
                if shift.id == shift_id {
                    *shift = shift.remove_worker(&worker);
                }
            })
        });

        updated_worksite
    }

    pub fn remove_tag(&self, tag_id: String) -> Worksite {
        let mut updated_worksite = self.clone();

        updated_worksite.tags.retain(|tag| tag.id != tag_id);
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
pub struct ShiftWorker(String);

impl ShiftWorker {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone)]
pub struct Shift {
    pub id: String,
    pub name: String,
    pub workers: Vec<ShiftWorker>,
}

impl Shift {
    pub fn assign_worker(&self, worker: &Worker) -> Shift {
        let mut updated_shift = self.clone();
        updated_shift.workers.push(ShiftWorker(worker.id.clone()));

        updated_shift
    }
    pub fn contains_worker(&self, worker: &Worker) -> bool {
        self.workers.iter().any(|w| w.0 == worker.id)
    }
    pub fn remove_worker(&self, worker: &Worker) -> Shift {
        let mut updated_shift = self.clone();
        updated_shift.workers.retain(|w| w.0 != worker.id);

        updated_shift
    }
}

#[derive(Debug, Clone)]
pub struct Worker {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub assessments: Vec<Assessment>,
    pub tags: Vec<AssignedTag>,
}

impl Worker {
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    pub fn has_tag(&self, tag: &Tag) -> bool {
        self.tags.iter().any(|t| t.0 == tag.id)
    }
    pub fn assign_tags(&self, tags: Vec<String>) -> Worker {
        let mut updated_worker = self.clone();
        updated_worker.tags = tags.into_iter().map(AssignedTag::new).collect();

        updated_worker
    }
    pub fn last_assessment(&self) -> Option<Assessment> {
        self.assessments.last().cloned()
    }
    pub fn add_assessment(&self, assessment: Assessment) -> Worker {
        let mut updated_worker = self.clone();
        updated_worker.assessments.push(assessment);

        updated_worker
    }
}

#[derive(Debug, Clone)]
pub struct Assessment {
    pub id: String,
    pub value: u8,
    pub notes: String,
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub icon: String,
}

#[derive(Debug, Clone)]
pub struct AssignedTag(String);

impl AssignedTag {
    pub fn new(id: String) -> Self {
        Self(id)
    }
}
