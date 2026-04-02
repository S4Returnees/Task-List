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
}

impl Default for TaskPlanner {
    fn default() -> Self {
        Self {
            active_tab: Tab::AllTasks,
        }
    }
}

impl TaskPlanner {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::TabSelected(tab) => self.active_tab = tab,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        render_view(self)
    }
}
