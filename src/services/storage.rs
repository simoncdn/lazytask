use std::{fs, io};

use crate::models::Task;

pub fn load_tasks(path: &str) -> io::Result<Vec<Task>> {
    let json_data = fs::read_to_string(path)?;
    let tasks: Vec<Task> = serde_json::from_str(&json_data)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(tasks)
}
