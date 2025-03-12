#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct CacheData {
    pub name: String,
    pub id: String,
    pub start_message_id: u64,
    pub end_message_id: u64,
    pub total_files: i32,
}

impl CacheData {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            id: String::new(),
            start_message_id: 0,
            end_message_id: 0,
            total_files: 0,
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn id(mut self, id: String) -> Self {
        self.id = id;
        self
    }

    pub fn start_message_id(mut self, start_id: u64) -> Self {
        self.start_message_id = start_id;
        self
    }

    pub fn end_message_id(mut self, end_id: u64) -> Self {
        self.end_message_id = end_id;
        self
    }

    pub fn total_files(mut self, total_files: i32) -> Self {
        self.total_files = total_files;
        self
    }
}