use crate::message::Message;
use iced::widget::{button, center, column, text};
use iced::{Alignment, Element};

#[derive(Default)]
pub struct TaskPlanner {
}

impl TaskPlanner {
    pub fn update(&mut self, message: Message) {
    }

    pub fn view(&self) -> Element<'_, Message> {
        let content = text("Page Vierge").size(30);

        center(content).into()
    }
}