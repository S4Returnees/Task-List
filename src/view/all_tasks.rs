use crate::TaskPlanner;
use crate::message::Message;

use iced::widget::{Space, button, column, container, rule, text};
use iced::{Element, Length};

pub fn view(_state: &TaskPlanner) -> Element<'_, Message> {
    column![
        tab_title(),
        Space::new().height(Length::Fill),
        add_task_button(),
    ]
    .spacing(10)
    .into()
}

pub fn tab_title<'a>() -> Element<'a, Message> {
    column![
        container(
            text("All Tasks")
                .size(25)
                .width(Length::Fill)
                .align_x(iced::alignment::Horizontal::Left),
        )
        .width(Length::Fill)
        .padding(20),
        rule::horizontal(1),
    ]
    .width(Length::Fill)
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
        .on_press(Message::OpenAddTaskPopup(None))
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
