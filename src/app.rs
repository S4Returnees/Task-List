use crate::message::Message;
use crate::view::view::render_view;

use iced::Element;
use iced::widget::{combo_box, text_editor};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tab {
    AllTasks,
    Calendar,
    Settings,
    Category(String),
}

pub struct TaskPlanner {
    pub active_tab: Tab,
    pub show_add_task_popup: bool,
    pub add_task_name: String,
    pub add_task_category: Option<String>,
    pub category_combo_state: combo_box::State<String>,
    pub category_selected_item: Option<String>,
    pub priority_combo_state: combo_box::State<String>,
    pub priority_selected_item: Option<String>,
    pub add_task_due_date: String,
    pub add_task_description: text_editor::Content,
    pub sort_by_combo_state: combo_box::State<String>,
    pub sort_by_selected_item: Option<String>,
}

impl Default for TaskPlanner {
    fn default() -> Self {
        Self {
            active_tab: Tab::AllTasks,
            show_add_task_popup: false,
            add_task_name: String::new(),
            add_task_category: None,
            category_combo_state: combo_box::State::new(vec!["None".to_string()]),
            category_selected_item: Some("None".to_string()),
            priority_combo_state: combo_box::State::new(vec![
                "None".to_string(),
                "Optional".to_string(),
                "Low".to_string(),
                "Medium".to_string(),
                "High".to_string(),
                "Critical".to_string(),
            ]),
            priority_selected_item: Some("None".to_string()),
            add_task_due_date: String::new(),
            add_task_description: text_editor::Content::new(),
            sort_by_combo_state: combo_box::State::new(vec![
                "Name".to_string(),
                "Priority".to_string(),
                "Due Date".to_string(),
                "Status".to_string(),
            ]),
            sort_by_selected_item: Some("Name".to_string()),
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
        }
    }

    fn close_add_task_popup(&mut self) {
        self.show_add_task_popup = false;
        self.add_task_name.clear();
        self.category_selected_item = Some("None".to_string());
        self.priority_selected_item = Some("None".to_string());
        self.add_task_due_date.clear();
        self.add_task_description = text_editor::Content::new();
    }

    fn add_task_handler(&mut self) {
        if self.add_task_name.is_empty() {
            return;
        }
        self.close_add_task_popup()
    }

    pub fn view(&self) -> Element<'_, Message> {
        render_view(self)
    }
}
