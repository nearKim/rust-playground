use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
    pub due_date: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_task_creation() {
        let task = Task {
            id: 1,
            description: "Buy groceries".to_string(),
            completed: false,
            due_date: Some("2023-12-01".to_string()),
        };
        assert_eq!(task.id, 1);
        assert_eq!(task.description, "Buy groceries");
        assert_eq!(task.completed, false);
        assert_eq!(task.due_date, Some("2023-12-01".to_string()));
    }

    #[test]
    fn test_task_creation_empty_description() {
        let task = Task {
            id: 2,
            description: "".to_string(),
            completed: false,
            due_date: None,
        };
        assert_eq!(task.description, ""); // Should allow empty descriptions or reject based on design
    }

    #[test]
    fn test_task_to_string_pending() {
        let task = Task {
            id: 3,
            description: "Write tests".to_string(),
            completed: false,
            due_date: None,
        };
        assert_eq!(task.to_string(), "[✗] Write tests");
    }

    #[test]
    fn test_task_to_string_completed() {
        let task = Task {
            id: 4,
            description: "Finish project".to_string(),
            completed: true,
            due_date: Some("2023-12-01".to_string()),
        };
        assert_eq!(task.to_string(), "[✓] Finish project (Due: 2023-12-01)");
    }

    #[test]
    fn test_task_with_invalid_due_date_format() {
        let task = Task {
            id: 5,
            description: "Invalid date task".to_string(),
            completed: false,
            due_date: Some("not-a-date".to_string()),
        };
        // Assuming validation exists, this could panic or be handled gracefully
        assert_eq!(task.due_date, Some("not-a-date".to_string()));
    }
}
