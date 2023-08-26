pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<Story>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Open,
            stories: vec![],
        }
        // by default the status should be set to open and the stories should be an empty vector
    }
}

pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Open,
        } // by default the status should be set to open
    }
}

pub struct DBState {
    pub last_item_id: u32,
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>,
}
