pub struct Worksite {
    pub id: String,
    pub name: String,
    pub locations: Vec<Location>,
}

pub struct Location {
    pub id: String,
    pub name: String,
    pub shifts: Vec<Shift>,
}

pub struct Shift {
    pub id: String,
    pub name: String,
    pub workers: Vec<Worker>,
}

pub struct Worker {
    pub id: String,
    pub name: String,
    pub last_assessment: Assessment,
    pub tags: Vec<Tag>,
}

pub struct Assessment {
    pub id: String,
    pub value: u8,
}

pub struct Tag {
    pub id: String,
    pub name: String,
    pub icon: String,
}
