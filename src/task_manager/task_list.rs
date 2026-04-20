use crate::task_manager::task::*;

pub struct TaskList {
    list: Vec<Task>,
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
        match criteria {
            "Name" => {
                self.list.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
            }

            "Priority" => {
                self.list.sort_by(|a, b| a.priority.cmp(&b.priority));
            }

            "Due Date" => {
                self.list.sort_by(|a, b| match (a.due_date, b.due_date) {
                    (Some(d1), Some(d2)) => d1.cmp(&d2),
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (Some(_), None) => std::cmp::Ordering::Less,
                    _ => std::cmp::Ordering::Equal,
                });
            }

            "Status" => {
                self.list.sort_by(|a, b| a.status.cmp(&b.status));
            }

            _ => {}
        }
    }
}
