use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
    pub due_date: Option<String>,
}

impl Task {
    pub fn to_string(&self) -> String {
        format!(
            "{}: {} [{}]",
            self.id,
            self.description,
            if self.completed { "Completed" } else { "Pending" }
        )
    }
}