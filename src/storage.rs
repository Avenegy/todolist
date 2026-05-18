
use std::fs;
use std::collections::HashMap;
use crate::models::Task;

pub fn save_task(tasks: &HashMap<i64, Task>) {
    let data = serde_json::to_string_pretty(tasks).unwrap();
    fs::write("tasks.json", data).unwrap_or_else(|_| {
        println!("The file could not be saved!")
    });
}

pub fn load_tasks() -> HashMap<i64, Task> {
    match fs::read_to_string("tasks.json") {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| HashMap::new()),
        Err(_) => {
            println!("Unable to upload the file! A new one has been created");
            HashMap::new()
        }
    }
}