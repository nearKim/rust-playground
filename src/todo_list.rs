use crate::storage::{load_tasks, save_tasks};
use crate::task::Task;

pub struct ToDoList {
    tasks: Vec<Task>,
    next_id: u32,
}

impl ToDoList {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 0,
        }
    }

    pub fn load() -> Result<Self, String> {
        load_tasks().map(|tasks| {
            let next_id = tasks.last().map(|last| last.id + 1).unwrap_or(0);
            Self { tasks, next_id }
        })
    }

    pub fn save(&self) -> Result<(), String> {
        // &self.tasks는 &Vec<Task> 타입이지만, Rust의 "deref coercion" 때문에 slice reference에 넘길 수 있음.
        // Vec<Task> 는 Deref trait을 구현하므로, [Task]로 deref 될 수 있음.
        // 내부적으로 알아서 deref 연산 &(*vec)을 수행
        // Rust의 타입 시스템은 T가 Deref<Target = U> 를 구현할 경우, &T가 &U로 coerce 되는 것을 허용함
        // The purpose of Deref coercion is to make smart pointer types, like Box, behave as much like the underlying value as possible.
        // Using a Box<Chessboard> is mostly just like using a plain Chessboard, thanks to Deref.
        save_tasks(&self.tasks)
    }

    pub fn add_task(&mut self, description: String, due_date: Option<String>) {
        let new_task = Task::new(self.next_id, description, due_date);
        self.tasks.push(new_task);
    }

    pub fn list_tasks(&self, filter: Option<bool>) -> Vec<&Task> {
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
        todo_list.add_task("Task 1".to_string(), None);
        assert_eq!(todo_list.tasks.len(), 1);
        assert_eq!(todo_list.tasks[0].description, "Task 1");
        assert_eq!(todo_list.tasks[0].id, 1); // Assuming IDs start at 1
    }

    #[test]
    fn test_add_task_empty_description() {
        let mut todo_list = ToDoList::new();
        todo_list.add_task("".to_string(), None);
        assert_eq!(todo_list.tasks.len(), 1);
        assert_eq!(todo_list.tasks[0].description, ""); // Edge case: allowed or rejected?
    }

    #[test]
    fn test_complete_task_valid() {
        let mut todo_list = ToDoList::new();
        todo_list.add_task("Task to complete".to_string(), None);
        let id = todo_list.tasks[0].id;
        todo_list.complete_task(id);
        assert!(todo_list.tasks[0].completed);
    }

    #[test]
    fn test_complete_task_nonexistent() {
        let mut todo_list = ToDoList::new();
        todo_list.complete_task(999); // Should not panic, just do nothing or log
        assert!(todo_list.tasks.is_empty());
    }

    #[test]
    fn test_remove_task_valid() {
        let mut todo_list = ToDoList::new();
        todo_list.add_task("Task to remove".to_string(), None);
        let id = todo_list.tasks[0].id;
        todo_list.remove_task(id);
        assert!(todo_list.tasks.is_empty());
    }

    #[test]
    fn test_remove_task_nonexistent() {
        let mut todo_list = ToDoList::new();
        todo_list.add_task("Remaining task".to_string(), None);
        todo_list.remove_task(999); // Should not affect existing tasks
        assert_eq!(todo_list.tasks.len(), 1);
    }

    #[test]
    fn test_list_tasks_all() {
        let mut todo_list = ToDoList::new();
        todo_list.add_task("Task 1".to_string(), None);
        todo_list.add_task("Task 2".to_string(), None);
        todo_list.complete_task(todo_list.tasks[1].id);
        let tasks = todo_list.list_tasks(None);
        assert_eq!(tasks.len(), 2);
    }

    #[test]
    fn test_list_tasks_pending_only() {
        let mut todo_list = ToDoList::new();
        todo_list.add_task("Pending".to_string(), None);
        todo_list.add_task("Completed".to_string(), None);
        todo_list.complete_task(todo_list.tasks[1].id);
        let pending = todo_list.list_tasks(Some(false));
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].description, "Pending");
    }

    #[test]
    fn test_list_tasks_completed_only() {
        let mut todo_list = ToDoList::new();
        todo_list.add_task("Pending".to_string(), None);
        todo_list.add_task("Completed".to_string(), None);
        todo_list.complete_task(todo_list.tasks[1].id);
        let completed = todo_list.list_tasks(Some(true));
        assert_eq!(completed.len(), 1);
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
        todo_list.add_task(
            "Persistent task".to_string(),
            Some("2023-12-01".to_string()),
        );
        todo_list.save().expect("Failed to save tasks");
        let loaded = ToDoList::load().expect("Failed to load tasks");
        assert_eq!(loaded.tasks.len(), 1);
        assert_eq!(loaded.tasks[0].description, "Persistent task");
    }

    #[test]
    fn test_load_nonexistent_file() {
        // Ensure no tasks.json exists before loading
        std::fs::remove_file("tasks.json").ok();
        let result = ToDoList::load();
        assert!(result.is_err()); // Should return error for missing file
    }

    #[test]
    fn test_save_with_no_tasks() {
        let todo_list = ToDoList::new();
        todo_list.save().expect("Failed to save empty list");
        let loaded = ToDoList::load().expect("Failed to load empty list");
        assert!(loaded.tasks.is_empty());
    }
}
