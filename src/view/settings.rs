use crate::TaskPlanner;
use crate::message::Message;

use iced::widget::{Space, button, column, container, text};
use iced::{Element, Length};

pub fn view(_state: &TaskPlanner) -> Element<'_, Message> {
    column![
        text("Settings")
            .size(50)
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center),
        Space::new().height(Length::Fixed(20.0)),
        container(
            column![
                button("Save")
                    .on_press(Message::Save)
                    .width(Length::Fixed(200.0)),
                button("Export")
                    .on_press(Message::Export)
                    .width(Length::Fixed(200.0)),
                button("Import")
                    .on_press(Message::Import)
                    .width(Length::Fixed(200.0)),
                button("Reset")
                    .on_press(Message::Reset)
                    .width(Length::Fixed(200.0))
            ]
            .spacing(20)
        )
        .width(Length::Fill)
        .align_x(iced::alignment::Horizontal::Center),
    ]
    .padding(20)
    .spacing(20)
    .into()
}
