use chrono::NaiveDate;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    None,
    Optional,
    Low,
    Medium,
    High,
    Critical,
}

impl Priority {
    pub const ALL: [Priority; 6] = [
        Priority::None,
        Priority::Optional,
        Priority::Low,
        Priority::Medium,
        Priority::High,
        Priority::Critical,
    ];
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Pending,
    InProgress,
    Done,
}

impl Status {
    pub const ALL: [Status; 3] = [Status::Pending, Status::InProgress, Status::Done];
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Status::Pending => write!(f, "Pending"),
            Status::InProgress => write!(f, "In Progress"),
            Status::Done => write!(f, "Done"),
        }
    }
}

#[derive(Clone)]
pub struct Task {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub category_id: usize,
    pub priority: Priority,
    pub status: Status,
    pub due_date: Option<NaiveDate>,
}

impl Task {
    pub fn new(
        name: String,
        description: String,
        category_id: usize,
        priority: Priority,
        due_date: Option<NaiveDate>,
    ) -> Self {
        Task {
            id: 0,
            name,
            description,
            category_id,
            priority,
            status: Status::Pending,
            due_date,
        }
    }

    pub fn get_due_date(&self) -> String {
        match self.due_date {
            Some(date) => date.format("%Y-%m-%d").to_string(),
            None => "".to_string(),
        }
    }
}
