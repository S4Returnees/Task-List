use crate::TaskPlanner;
use crate::message::Message;

use iced::widget::{button, column, container, text};
use iced::{Element, Length};

pub fn view(_state: &TaskPlanner) -> Element<'_, Message> {
    column![
        text("Settings")
            .size(100)
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center),
        container(
            column![
                button(text("Save").size(25).center())
                    .on_press(Message::Save)
                    .width(300)
                    .height(50),
                button(text("Export").size(25).center())
                    .on_press(Message::Export)
                    .width(300)
                    .height(50),
                button(text("Import").size(25).center())
                    .on_press(Message::Import)
                    .width(300)
                    .height(50),
                button(text("Reset").size(25).center())
                    .on_press(Message::Reset)
                    .width(300)
                    .height(50),
            ]
            .spacing(40)
        )
        .width(Length::Fill)
        .align_x(iced::alignment::Horizontal::Center),
    ]
    .padding(20)
    .spacing(40)
    .into()
}
