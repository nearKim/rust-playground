use crate::task::Task;
use std::io;
use std::fs;

const TODO_FILE: &str = "src/data/todo.json";

pub fn save_tasks(tasks: &[Task]) -> Result<(), String> {
    todo!("Implement task saving to JSON")
}

pub fn load_tasks() -> Result<Vec<Task>, String> {
    let lines = fs::read_to_string(TODO_FILE).expect("Failed to read file");
    println!("{}", lines);
    let tasks = Vec::<Task>::new();
    Ok(tasks)
}
