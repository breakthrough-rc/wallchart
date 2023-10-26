#[derive(Debug, Clone)]
pub struct Worksite {
    pub id: String,
    pub name: String,
    pub locations: Vec<Location>,
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
    pub last_assessment: Assessment,
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

pub enum Event {
    WorksiteCreated { id: String, name: String },
    LocationAdded { id: String, name: String },
    ShiftAdded { id: String, name: String },
    WorkerCreated { id: String, name: String },
    ShiftAssigned { shift_id: String, worker_id: String },
    ShiftUnassigned { shift_id: String, worker_id: String },
}
