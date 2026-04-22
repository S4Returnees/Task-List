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
                button("Export").width(Length::Fixed(200.0)),
                button("Import").width(Length::Fixed(200.0)),
                button("Reset").width(Length::Fixed(200.0))
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
