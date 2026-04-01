use crate::message::Message;
use crate::TaskPlanner;

use iced::{Element, Length};
use iced::widget::{button, column, container, Space, text};

pub fn view(_state: &TaskPlanner) -> Element<'_, Message> {
    column![
        text("All Tasks").size(50)
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center),

        Space::new().height(Length::Fill),

        add_task_button(),
    ]
    .spacing(20)
    .into()
}

fn add_task_button<'a>() -> Element<'a, Message> {
    container(
        button(
            container(text("+").size(30))
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(iced::alignment::Horizontal::Center),
        )
            .width(Length::Fixed(60.0))
            .height(Length::Fixed(60.0))
            .style(|theme, status| {
                let mut style = button::primary(theme, status);
                style.border.radius = 30.0.into();
                style
            }),
    )
    .padding(20)
    .width(Length::Fill)
    .align_x(iced::alignment::Horizontal::Center)
    .into()
}