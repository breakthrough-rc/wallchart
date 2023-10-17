#[derive(Clone)]
pub struct Worksite {
    pub id: String,
    pub name: String,
    pub locations: Vec<Location>,
}

#[derive(Clone)]
pub struct Location {
    pub id: String,
    pub name: String,
    pub shifts: Vec<Shift>,
}

#[derive(Clone)]
pub struct Shift {
    pub id: String,
    pub name: String,
    pub workers: Vec<Worker>,
}

#[derive(Clone)]
pub struct Worker {
    pub id: String,
    pub name: String,
    pub last_assessment: Assessment,
    pub tags: Vec<Tag>,
}

#[derive(Clone)]
pub struct Assessment {
    pub id: String,
    pub value: u8,
}

#[derive(Clone)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub icon: String,
}
