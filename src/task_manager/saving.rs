use std::fs;
use std::path::Path;
use crate::task_manager::task::Task;

pub fn save(tasks: &Vec<Task>, path: &Path)
{
    let data = serde_json::to_string_pretty(tasks).expect("Erreur conversion");
    fs::write(path, data).expect("Erreur ecriture");
}

pub fn load(path: &Path) -> Vec<Task>
{
    if !path.exists()
    {
        return Vec::new();
    }
    let data = fs::read_to_string(path).expect("Erreur lecture");
    if data.trim().is_empty()
    {
        return Vec::new();
    }
    let task: Vec<Task> = serde_json::from_str(&data).expect("Erreur lecture JSON");
    task
}