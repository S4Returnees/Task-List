use crate::task_manager::task::*;
use chrono::NaiveDate;
use crate::app::SortBy;

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

    pub fn sort_by(&mut self, sort: SortBy) {
        match sort {
            SortBy::Name => {
                self.list.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
            }

            SortBy::Priority => {
                self.list.sort_by(|a, b| a.priority.cmp(&b.priority));
            }

            SortBy::DueDate => {
                self.list.sort_by(|a, b| match (a.due_date, b.due_date) {
                    (Some(d1), Some(d2)) => d1.cmp(&d2),
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (Some(_), None) => std::cmp::Ordering::Less,
                    _ => std::cmp::Ordering::Equal,
                });
            }

            SortBy::Status => {
                self.list.sort_by(|a, b| a.status.cmp(&b.status));
            }
        }
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
