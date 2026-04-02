use crate::message::Message;
use crate::view::view::render_view;

use iced::Element;

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
}

impl Default for TaskPlanner {
    fn default() -> Self {
        Self {
            active_tab: Tab::AllTasks,
            show_add_task_popup: false,
            add_task_name: String::new(),
        }
    }
}

impl TaskPlanner {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::TabSelected(tab) => self.active_tab = tab,
            Message::OpenAddTaskPopup(category) => {
                self.show_add_task_popup = true;
            }
            Message::CloseAddTaskPopup => self.show_add_task_popup = false,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        render_view(self)
    }
}
