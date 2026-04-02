use crate::TaskPlanner;
use crate::message::Message;

use iced::widget::{column, text};
use iced::{Element, Length};

pub fn view(_state: &TaskPlanner) -> Element<'_, Message> {
    column![
        text("Calendar")
            .size(50)
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center),
    ]
    .spacing(20)
    .into()
}
