mod commands;
mod storage;
mod task;
mod todo_list;

use commands::{parse_command, Command};
use std::io;
use todo_list::ToDoList;

fn main() {
    let mut todo_list = ToDoList::load().unwrap_or_else(|_| ToDoList::new());

    loop {
        println!("Enter command (add, list, complete, remove, exit):");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match parse_command(&input) {
            Ok(command) => match command {
                Command::Add(description, due_date) => {
                    todo_list.add_task(description, due_date);
                    todo_list.save().expect("Failed to save tasks");
                }
                Command::List(filter) => {
                    let tasks = todo_list.list_tasks(filter);
                    for task in tasks {
                        println!("{}", task.description);
                    }
                }
                Command::Complete(id) => {
                    todo_list.complete_task(id);
                    todo_list.save().expect("Failed to save tasks");
                }
                Command::Remove(id) => {
                    todo_list.remove_task(id);
                    todo_list.save().expect("Failed to save tasks");
                }
                Command::Exit => break,
            },
            Err(e) => println!("Error: {}", e),
        }
    }
}
