pub struct Project {
    pub project: String,
    pub version: String,
    pub dist: String,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            project: "".to_string(),
            version: "".to_string(),
            dist: "".to_string(),
        }
    }
}
