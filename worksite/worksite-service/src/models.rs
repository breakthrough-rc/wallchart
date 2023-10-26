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

pub enum Events {
    WorksiteCreated(WorksiteCreatedEvent),
    LocationAdded(LocationAddedEvent),
    ShiftAdded(ShiftAddedEvent),
    WorkerCreated(WorkerCreatedEvent),
    ShiftAssigned(ShiftAssignedEvent),
    ShiftUnassigned(ShiftUnassignedEvent),
}

pub struct WorksiteCreatedEvent {
    pub id: String,
    pub name: String,
}

pub struct LocationAddedEvent {
    pub id: String,
    pub name: String,
}

pub struct ShiftAddedEvent {
    pub id: String,
    pub name: String,
}

pub struct WorkerCreatedEvent {
    pub id: String,
    pub name: String,
}

pub struct ShiftAssignedEvent {
    pub shift_id: String,
    pub worker_id: String,
}

pub struct ShiftUnassignedEvent {
    pub shift_id: String,
    pub worker_id: String,
}
