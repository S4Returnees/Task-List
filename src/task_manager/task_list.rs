use crate::task_manager::task::*;
use chrono::{NaiveDate, Duration};
use chrono::{Datelike};

pub struct TaskList {
    pub list: Vec<Task>,
}

fn last_day_of_month(year: i32, month: u32) -> u32 {
    let next_month = if month == 12 { 1 } else { month + 1 };
    let next_year = if month == 12 { year + 1 } else { year };

    let first_of_next =
        NaiveDate::from_ymd_opt(next_year, next_month, 1).unwrap();

    let last_day = first_of_next - Duration::days(1);
    last_day.day()
}

fn add_one_month_clamped(date: NaiveDate) -> NaiveDate {
    let year = date.year();
    let month = date.month();

    let (new_year, new_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };

    let day = date.day();
    let last_day = last_day_of_month(new_year, new_month);

    let new_day = day.min(last_day);

    NaiveDate::from_ymd_opt(new_year, new_month, new_day).unwrap()
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
    pub fn handle_recurring_task(&mut self, task: &Task) {
        let Some(date) = task.due_date else {
            return;
        };

        let new_date = match task.recurrence {
            Recurrence::Daily => date + Duration::days(1),
            Recurrence::Weekly => date + Duration::days(7),
            Recurrence::Monthly => add_one_month_clamped(date)
            Recurrence::None => return,
        };

        let mut new_task = Task {
            id: 0,
            name: task.name.clone(),
            description: task.description.clone(),
            category_id: task.category_id,
            priority: task.priority,
            status: Status::Pending,
            due_date: Some(new_date),
            recurrence: task.recurrence,
        };

        self.add(new_task);
    }
}
