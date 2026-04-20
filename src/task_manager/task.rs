use chrono::NaiveDate;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Optional,
    Low,
    Medium,
    High,
    Critical,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
    Pending,
    InProgress,
    Done,
}

pub struct Task {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub category_id: Option<usize>,
    pub priority: Option<Priority>,
    pub status: Option<Status>,
    pub due_date: Option<NaiveDate>,
}

impl Task {
    pub fn new(
        name: String,
        description: String,
        category_id: Option<usize>,
        priority: Option<Priority>,
        due_date: Option<NaiveDate>,
    ) -> Self {
        Task {
            id: 0,
            name,
            description,
            category_id,
            priority,
            status: Some(Status::Pending),
            due_date,
        }
    }

    fn update_name(&mut self, name: String) {
        self.name = name;
    }
    fn update_description(&mut self, description: String) {
        self.description = description;
    }

    fn update_category_id(&mut self, category_id: Option<usize>) {
        self.category_id = category_id;
    }

    fn update_priority(&mut self, priority: Option<Priority>) {
        self.priority = priority;
    }
    fn update_status(&mut self, status: Option<Status>) {
        self.status = status;
    }

    fn update_due_date(&mut self, due_date: Option<NaiveDate>) {
        self.due_date = due_date;
    }
}
