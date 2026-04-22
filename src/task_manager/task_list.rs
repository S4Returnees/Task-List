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
    pub fn sort_by(&mut self, criteria: &str) {
        self.list.sort_by(|a, b| {
        match (a.status, b.status) {
            (Status::Done, Status::Done) => {}
            (Status::Done, _) => return std::cmp::Ordering::Greater,
            (_, Status::Done) => return std::cmp::Ordering::Less,
            _ => {}
        }
        match criteria {
            "Name" => a.name.cmp(&b.name),

            "Priority" => a.priority.cmp(&b.priority),

            "Due Date" => a.due_date.cmp(&b.due_date),

            "Status" => a.status.cmp(&b.status),

            _ => a.id.cmp(&b.id),
            }
        });
    }
    pub fn sort_default(&mut self) {
    self.list.sort_by(|a, b| {
        match (a.status, b.status) {
            (Status::Done, Status::Done) => a.id.cmp(&b.id),
            (Status::Done, _) => std::cmp::Ordering::Greater,
            (_, Status::Done) => std::cmp::Ordering::Less,
            _ => a.id.cmp(&b.id),
            }
        });
    }
}
