use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    id: usize,
    title: String,
}

impl Task {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn id(&self) -> usize {
        self.id
    }
}
