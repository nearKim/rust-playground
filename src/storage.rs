use crate::task::Task;
use serde_json::{from_reader, to_string};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};

pub fn save_tasks(tasks: &[Task], file_name: &str) -> Result<(), String> {
    // Open the file with OpenOptions:
    // - write(true): Enable writing
    // - create(true): Create if it doesn’t exist
    // - truncate(true): Overwrite (clear) if it exists
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name)
        .map_err(|e| format!("Failed to open file: {}", e))?;
    let json = to_string(tasks).map_err(|e| format!("Failed to serialize tasks: {}", e))?;

    file.write_all(json.as_bytes())
        .map_err(|e| format!("Failed to write to file: {}", e))?;

    Ok(())
}

pub fn load_tasks(file_name: &str) -> Result<Vec<Task>, String> {
    // ? operator는 error발생시 early return
    let file = File::open(file_name).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    from_reader(reader).map_err(|e| e.to_string())
}
