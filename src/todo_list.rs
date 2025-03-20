use crate::storage::{load_tasks, save_tasks};
use crate::task::Task;

pub struct ToDoList {
    pub tasks: Vec<Task>,
    pub next_id: u32,
}

impl ToDoList {
    pub fn new() -> Self {
        todo!("Initialize an empty ToDoList")
    }

    pub fn load() -> Result<Self, String> {
        todo!("Load ToDoList from storage")
    }

    pub fn save(&self) -> Result<(), String> {
        todo!("Save ToDoList to storage")
    }

    pub fn add_task(&mut self, description: String, due_date: Option<String>) -> u32 {
        todo!("Add a new task")
    }

    pub fn list_tasks(&self, filter: Option<String>) -> Vec<&Task> {
        todo!("List tasks with optional filter")
    }

    pub fn complete_task(&mut self, id: u32) {
        todo!("Mark a task as completed")
    }

    pub fn remove_task(&mut self, id: u32) {
        todo!("Remove a task")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task_valid() {
        let mut todo_list = ToDoList::new();
        let id = todo_list.add_task("Task 1".to_string(), None);
        let tasks = todo_list.list_tasks(None);
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].description, "Task 1");
        assert_eq!(tasks[0].id, id); // ID matches the one returned by add_task
    }

    #[test]
    fn test_add_task_empty_description() {
        let mut todo_list = ToDoList::new();
        let id = todo_list.add_task("".to_string(), None);
        let tasks = todo_list.list_tasks(None);
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].description, ""); // Tests behavior with empty description
    }

    #[test]
    fn test_complete_task_valid() {
        let mut todo_list = ToDoList::new();
        let id = todo_list.add_task("Task to complete".to_string(), None);
        todo_list.complete_task(id);
        let tasks = todo_list.list_tasks(None);
        assert!(tasks[0].completed);
    }

    #[test]
    fn test_complete_task_nonexistent() {
        let mut todo_list = ToDoList::new();
        todo_list.complete_task(999); // Should not panic, just do nothing
        let tasks = todo_list.list_tasks(None);
        assert!(tasks.is_empty());
    }

    #[test]
    fn test_remove_task_valid() {
        let mut todo_list = ToDoList::new();
        let id = todo_list.add_task("Task to remove".to_string(), None);
        todo_list.remove_task(id);
        let tasks = todo_list.list_tasks(None);
        assert!(tasks.is_empty());
    }

    #[test]
    fn test_remove_task_nonexistent() {
        let mut todo_list = ToDoList::new();
        let id = todo_list.add_task("Remaining task".to_string(), None);
        todo_list.remove_task(999); // Should not affect existing tasks
        let tasks = todo_list.list_tasks(None);
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].id, id);
    }

    #[test]
    fn test_list_tasks_all() {
        let mut todo_list = ToDoList::new();
        let _id1 = todo_list.add_task("Task 1".to_string(), None);
        let _id2 = todo_list.add_task("Task 2".to_string(), None);
        let tasks = todo_list.list_tasks(None);
        assert_eq!(tasks.len(), 2);
    }

    #[test]
    fn test_list_tasks_pending_only() {
        let mut todo_list = ToDoList::new();
        let id1 = todo_list.add_task("Pending".to_string(), None);
        let id2 = todo_list.add_task("Completed".to_string(), None);
        todo_list.complete_task(id2);
        let pending = todo_list.list_tasks(Some("pending".to_string()));
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].id, id1);
        assert_eq!(pending[0].description, "Pending");
    }

    #[test]
    fn test_list_tasks_completed_only() {
        let mut todo_list = ToDoList::new();
        let _id1 = todo_list.add_task("Pending".to_string(), None);
        let id2 = todo_list.add_task("Completed".to_string(), None);
        todo_list.complete_task(id2);
        let completed = todo_list.list_tasks(Some("completed".to_string()));
        assert_eq!(completed.len(), 1);
        assert_eq!(completed[0].id, id2);
        assert_eq!(completed[0].description, "Completed");
    }

    #[test]
    fn test_list_tasks_empty() {
        let todo_list = ToDoList::new();
        let tasks = todo_list.list_tasks(None);
        assert!(tasks.is_empty());
    }

    #[test]
    fn test_save_and_load_success() {
        let mut todo_list = ToDoList::new();
        let id = todo_list.add_task(
            "Persistent task".to_string(),
            Some("2023-12-01".to_string()),
        );
        todo_list.save().expect("Failed to save tasks");
        let loaded = ToDoList::load().expect("Failed to load tasks");
        let tasks = loaded.list_tasks(None);
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].id, id);
        assert_eq!(tasks[0].description, "Persistent task");
    }

    #[test]
    fn test_load_nonexistent_file() {
        std::fs::remove_file("tasks.json").ok(); // Remove file if it exists
        let result = ToDoList::load();
        assert!(result.is_err()); // Should fail when no file exists
    }

    #[test]
    fn test_save_with_no_tasks() {
        let todo_list = ToDoList::new();
        todo_list.save().expect("Failed to save empty list");
        let loaded = ToDoList::load().expect("Failed to load empty list");
        let tasks = loaded.list_tasks(None);
        assert!(tasks.is_empty());
    }
}
