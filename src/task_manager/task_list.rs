use crate::task_manager::task::*;
use chrono::NaiveDate;

pub struct TaskList {
    pub list: Vec<Task>,
}

impl TaskList {
    pub fn new() -> Self {
        TaskList { list: Vec::new() }
    }

    pub fn add(&mut self, mut new_task: Task) {
        let mut task_id = 0;
        for task in self.list.iter() {
            if task.id == task_id {
                task_id += 1;
            }
        }
        new_task.id = task_id;
        self.list.push(new_task);
    }

    pub fn get_task_by_date(&self, date: NaiveDate) -> Vec<Task> {
        self.list
            .iter()
            .filter(|task| task.due_date == Some(date))
            .cloned()
            .collect()
    }
}
