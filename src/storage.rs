use std::fs::File;
use std::io::{Read, Write};
use serde_json;
use crate::task::Task;

pub fn save_tasks(tasks: &Vec<Task>) -> Result<(), String> {
    let json = serde_json::to_string(tasks).map_err(|e| e.to_string())?;
    std::fs::write("todo.json", json).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn load_tasks() -> Result<Vec<Task>, String> {
    match File::open("todo.json") {
        Ok(mut file) => {
            let mut json = String::new();
            file.read_to_string(&mut json).map_err(|e| e.to_string())?;
            let tasks: Vec<Task> = serde_json::from_str(&json).map_err(|e| e.to_string())?;
            Ok(tasks)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(e) => Err(e.to_string()),
    }
}