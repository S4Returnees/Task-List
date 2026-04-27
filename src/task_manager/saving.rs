use std::fs;
use std::path::Path;
use crate::task_manager::task::Task;
use crate::task_manager::category::Category;
use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize)]
pub struct SaveData 
{
    pub category: Vec<Category>,
    pub tasks: Vec<Task>,
}

pub fn save(tasks: &SaveData, path: &Path)
{
    let data = serde_json::to_string_pretty(tasks).expect("Erreur conversion");
    fs::write(path, data).expect("Erreur ecriture");
}

pub fn load(path: &Path) -> SaveData
{
    if !path.exists()
    {
        return SaveData 
        {
            category: Vec::new(),
            tasks: Vec::new(),
        }
    }
    let data = fs::read_to_string(path).expect("Erreur lecture");
    if data.trim().is_empty()
    {
        return SaveData
        {
            category: Vec::new(),
            tasks: Vec::new(),
        }
    }
    let task: SaveData = serde_json::from_str(&data).expect("Erreur lecture JSON");
    task
}