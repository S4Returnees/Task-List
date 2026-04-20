use crate::message::Message;
use crate::task_manager::task::{Priority, Status, Task};
use crate::task_manager::task_list::TaskList;
use crate::view::view::render_view;

use chrono::{Datelike, Local, NaiveDate};
use iced::Element;
use iced::widget::{combo_box, text_editor};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tab {
    AllTasks,
    Calendar,
    Settings,
    Category(usize),
}

pub struct TaskPlanner {
    pub task_list: TaskList,
    pub active_tab: Tab,
    pub show_add_task_popup: bool,
    pub show_task_detail_popup: Option<usize>,
    pub add_task_name: String,
    pub status_combo_state: combo_box::State<Status>,
    pub task_status: Option<Status>,
    pub add_task_category: Option<String>,
    pub category_combo_state: combo_box::State<String>,
    pub category_selected_item: Option<String>,
    pub priority_combo_state: combo_box::State<Priority>,
    pub priority_selected_item: Option<Priority>,
    pub add_task_due_date: String,
    pub add_task_description: text_editor::Content,
    pub sort_by_combo_state: combo_box::State<String>,
    pub sort_by_selected_item: Option<String>,
    pub current_year: i32,
    pub current_month: u32,
}

impl Default for TaskPlanner {
    fn default() -> Self {
        Self {
            task_list: TaskList::new(),
            active_tab: Tab::AllTasks,
            show_add_task_popup: false,
            show_task_detail_popup: None,
            add_task_name: String::new(),
            status_combo_state: combo_box::State::new(Status::ALL.to_vec()),
            task_status: None,
            add_task_category: None,
            category_combo_state: combo_box::State::new(vec!["None".to_string()]),
            category_selected_item: Some("None".to_string()),
            priority_combo_state: combo_box::State::new(Priority::ALL.to_vec()),
            priority_selected_item: Some(Priority::None),
            add_task_due_date: String::new(),
            add_task_description: text_editor::Content::new(),
            sort_by_combo_state: combo_box::State::new(vec![
                "Name".to_string(),
                "Priority".to_string(),
                "Due Date".to_string(),
                "Status".to_string(),
            ]),
            sort_by_selected_item: Some("Name".to_string()),
            current_year: Local::now().year(),
            current_month: Local::now().month(),
        }
    }
}

impl TaskPlanner {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::TabSelected(tab) => self.active_tab = tab,
            Message::OpenAddTaskPopup(category) => {
                self.show_add_task_popup = true;
                self.add_task_category = category;
            }
            Message::CloseAddTaskPopup => self.close_add_task_popup(),
            Message::TaskNameChanged(new_name) => self.add_task_name = new_name,
            Message::TaskStatusChanged(status) => self.task_status = Some(status),
            Message::CategoryItemSelected(category) => self.category_selected_item = Some(category),
            Message::PriorityItemSelected(priority) => self.priority_selected_item = Some(priority),
            Message::TaskDueDateChanged(due_date) => self.add_task_due_date = due_date,
            Message::TaskDescriptionChanged(description) => {
                self.add_task_description.perform(description)
            }
            Message::AddTaskButtonPressed => self.add_task_handler(),
            Message::SortBySelectedItem(sort_by) => {
                self.sort_by_selected_item = Some(sort_by);
                //sort
            }
            Message::SelectTask(id) => self.select_task_detail_popup_handler(id),
            Message::CloseTaskDetailPopup => self.close_task_detail_popup_handler(),
            Message::StatusButton(id) => self.status_button_handler(id),
        }
    }

    fn close_add_task_popup(&mut self) {
        self.show_add_task_popup = false;
        self.add_task_name.clear();
        self.category_selected_item = Some("None".to_string());
        self.priority_selected_item = Some(Priority::None);
        self.add_task_due_date.clear();
        self.add_task_description = text_editor::Content::new();
    }

    fn add_task_handler(&mut self) {
        if self.add_task_name.is_empty() || self.verify_due_date() {
            return;
        }
        let new_task = Task::new(
            self.add_task_name.clone(),
            self.add_task_description.text(),
            None, //category id
            self.priority_selected_item.unwrap(),
            NaiveDate::parse_from_str(&self.add_task_due_date, "%Y-%m-%d").ok(),
        );
        self.task_list.add(new_task);
        self.close_add_task_popup()
    }
    fn select_task_detail_popup_handler(&mut self, id: usize) {
        let task = self.task_list.list.iter().find(|t| t.id == id).unwrap();
        self.add_task_name = task.name.clone();
        self.task_status = Some(task.status.clone());
        self.category_selected_item = Some("None".to_string()); //self.category_selected_item = task.category_id
        self.priority_selected_item = Some(task.priority);
        self.add_task_due_date = task.get_due_date();
        self.add_task_description = text_editor::Content::with_text(task.description.as_str());

        self.show_task_detail_popup = Some(id);
    }

    fn close_task_detail_popup_handler(&mut self) {
        if self.verify_due_date() {
            return;
        }
        let task = self
            .task_list
            .list
            .iter_mut()
            .find(|t| t.id == self.show_task_detail_popup.unwrap())
            .unwrap();

        task.name = self.add_task_name.clone();
        task.status = self.task_status.unwrap();
        //task.category_id = self.category_selected_item
        task.priority = self.priority_selected_item.unwrap();
        task.due_date = NaiveDate::parse_from_str(&self.add_task_due_date, "%Y-%m-%d").ok();
        task.description = self.add_task_description.text();
        self.close_add_task_popup();
        self.show_task_detail_popup = None;
    }

    fn verify_due_date(&mut self) -> bool {
        !self.add_task_due_date.is_empty()
            && NaiveDate::parse_from_str(&self.add_task_due_date, "%Y-%m-%d")
                .ok()
                .is_none()
    }

    fn status_button_handler(&mut self, id: usize) {
        let task = self.task_list.list.iter_mut().find(|t| t.id == id).unwrap();
        let next_status = match task.status.clone() {
            Status::Pending => Status::InProgress,
            Status::InProgress => Status::Done,
            Status::Done => Status::Pending,
        };
        task.status = next_status;
    }

    pub fn view(&self) -> Element<'_, Message> {
        render_view(self)
    }
}
