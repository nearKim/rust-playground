use crate::task::Task;

const TODO_FILE: &str = "data/todo.json";

pub fn save_tasks(tasks: &[Task]) -> Result<(), String> {
    todo!("Implement task saving to JSON")
}

pub fn load_tasks() -> Result<Vec<Task>, String> {
    todo!("Implement task loading from JSON")
}
