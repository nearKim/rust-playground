use crate::task::Task;
use crate::todo_list::ToDoList;
use std::io;
use std::fs;
use std::thread;
use std::time::Duration;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};
use serde_json;

const TODO_FILE: &str = "src/data/todo.json";

pub fn init_save_thread(todolist: Arc<Mutex<ToDoList>>) {
    println!("init_save_thread"); 
    let handle = thread::spawn(move || {
        save_tasks(todolist)
    });
}

fn __save_tasks(tasks: &Vec<Task>) {
    let json_data = serde_json::to_string_pretty(tasks)
        .map_err(|e| format!("Failed to serialize JSON: {}", e)).unwrap();

    fs::write(TODO_FILE, json_data)
        .map_err(|e| format!("Failed to write file: {}", e)).unwrap();

    println!("ToDoList is saved to storage");
}

pub fn save_tasks(todolist: Arc<Mutex<ToDoList>>) {
    println!("save_takss"); 
    loop {
        let mut todolist_guard = todolist.lock().unwrap();
        if !todolist_guard.synced.load(Ordering::SeqCst) {
            __save_tasks(&todolist_guard.tasks);
        }
        todolist_guard.synced.store(true, Ordering::SeqCst);
        drop(todolist_guard);
        thread::sleep(Duration::from_millis(300));
    }
}

pub fn load_tasks() -> Result<Vec<Task>, String> {
    let json_str = fs::read_to_string(TODO_FILE)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    println!("{}", json_str);

    let tasks: Vec<Task> = serde_json::from_str(&json_str)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    Ok(tasks)
}
