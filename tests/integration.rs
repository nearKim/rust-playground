#[cfg(test)]
mod integration_tests {
    use std::fs;
    use crate::todo_list::ToDoList;
    use crate::commands::parse_command;

    #[test]
    fn test_full_workflow() {
        // Clean up any existing file
        fs::remove_file("tasks.json").ok();

        let mut todo_list = ToDoList::new();
        todo_list.add_task("Integration task".to_string(), None);
        assert_eq!(todo_list.tasks.len(), 1);

        todo_list.save().expect("Failed to save tasks");
        let loaded = ToDoList::load().expect("Failed to load tasks");
        assert_eq!(loaded.tasks.len(), 1);
        assert_eq!(loaded.tasks[0].description, "Integration task");

        let id = loaded.tasks[0].id;
        let command = parse_command(&format!("complete {}", id)).expect("Failed to parse complete");
        if let Command::Complete(task_id) = command {
            todo_list.complete_task(task_id);
        }
        assert!(todo_list.tasks[0].completed);

        fs::remove_file("tasks.json").expect("Failed to clean up");
    }

    #[test]
    fn test_load_corrupted_file() {
        fs::write("tasks.json", "invalid json").expect("Failed to write corrupted file");
        let result = ToDoList::load();
        assert!(result.is_err()); // Should fail gracefully
        fs::remove_file("tasks.json").expect("Failed to clean up");
    }
}