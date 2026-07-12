use crate::widgets::task::Task;
use std::fs;
use std::path::Path;

const FILE_PATH: &str = "tasks.json";

pub fn save_tasks(tasks: &[Task]) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(tasks).map_err(std::io::Error::other)?;

    fs::write(FILE_PATH, json)
}

pub fn load_tasks() -> std::io::Result<Vec<Task>> {
    if !Path::new(FILE_PATH).exists() {
        return Ok(vec![]);
    }

    let data = fs::read_to_string(FILE_PATH)?;
    let tasks: Vec<Task> = serde_json::from_str(&data).map_err(std::io::Error::other)?;

    Ok(tasks)
}
