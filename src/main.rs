mod commands;
mod storage;
mod task;
mod todo_list;

use commands::{parse_command, Command};
use std::io;
use todo_list::ToDoList;
use storage::init_save_thread;
use std::sync::{
    Arc, Mutex,
};

fn main() {
    // let mut todo_list = ToDoList::load().unwrap_or_else(|_| ToDoList::new());
    let shared_list = Arc::new(Mutex::new(
        ToDoList::load().unwrap_or_else(|_| ToDoList::new())
    ));
    init_save_thread(shared_list.clone());

    loop {
        println!("Enter command (add, list, complete, remove, exit):");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match parse_command(&input) {
            Ok(command) => match command {
                Command::Add(description, due_date) => {
                    let mut todo_list = shared_list.lock().unwrap();
                    todo_list.add_task(description, due_date);
                    todo_list.save().expect("Failed to save tasks");
                }
                Command::List(filter) => {
                    let mut todo_list = shared_list.lock().unwrap();
                    let tasks = todo_list.list_tasks(filter);
                    for task in tasks {
                        println!("{}", task.description);
                    }
                }
                Command::Complete(id) => {
                    let mut todo_list = shared_list.lock().unwrap();
                    todo_list.complete_task(id);
                    todo_list.save().expect("Failed to save tasks");
                }
                Command::Remove(id) => {
                    let mut todo_list = shared_list.lock().unwrap();
                    todo_list.remove_task(id);
                    todo_list.save().expect("Failed to save tasks");
                }
                Command::Exit => break,
            },
            Err(e) => println!("Error: {}", e),
        }
    }
}
