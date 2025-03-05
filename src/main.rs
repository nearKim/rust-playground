mod task;
mod storage;

use std::io;
use task::Task;
use storage::{load_tasks, save_tasks};

struct ToDoList {
    tasks: Vec<Task>,
    next_id: u32,
}

impl ToDoList {
    fn new() -> Self {
        ToDoList {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, description: String, due_date: Option<String>) {
        let task = Task {
            id: self.next_id,
            description,
            completed: false,
            due_date,
        };
        self.tasks.push(task);
        self.next_id += 1;
    }

    fn list_tasks(&self, filter: Option<bool>) {
        for task in &self.tasks {
            if filter.is_none() || filter == Some(task.completed) {
                println!("{}", task.to_string());
            }
        }
    }

    fn complete_task(&mut self, id: u32) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
        } else {
            println!("Task not found");
        }
    }

    fn remove_task(&mut self, id: u32) {
        self.tasks.retain(|t| t.id != id);
    }
}

fn main() {
    let mut todo_list = ToDoList::new();

    // Load tasks from file if exists
    if let Ok(tasks) = load_tasks() {
        todo_list.tasks = tasks;
        if let Some(max_id) = todo_list.tasks.iter().map(|t| t.id).max() {
            todo_list.next_id = max_id + 1;
        }
    }

    loop {
        println!("Enter command (add, list, complete, remove, exit):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "add" => {
                if parts.len() < 2 {
                    println!("Usage: add <description>");
                    continue;
                }
                let description = parts[1..].join(" ");
                todo_list.add_task(description, None);  // Due date omitted for simplicity
                save_tasks(&todo_list.tasks).expect("Failed to save tasks");
            }
            "list" => {
                let filter = if parts.len() > 1 {
                    match parts[1] {
                        "pending" => Some(false),
                        "completed" => Some(true),
                        _ => None,
                    }
                } else {
                    None
                };
                todo_list.list_tasks(filter);
            }
            "complete" => {
                if let Some(id_str) = parts.get(1) {
                    if let Ok(id) = id_str.parse::<u32>() {
                        todo_list.complete_task(id);
                        save_tasks(&todo_list.tasks).expect("Failed to save tasks");
                    } else {
                        println!("Invalid task ID");
                    }
                } else {
                    println!("Usage: complete <id>");
                }
            }
            "remove" => {
                if let Some(id_str) = parts.get(1) {
                    if let Ok(id) = id_str.parse::<u32>() {
                        todo_list.remove_task(id);
                        save_tasks(&todo_list.tasks).expect("Failed to save tasks");
                    } else {
                        println!("Invalid task ID");
                    }
                } else {
                    println!("Usage: remove <id>");
                }
            }
            "exit" => break,
            _ => println!("Unknown command"),
        }
    }
}