use crate::TaskPlanner;
use crate::message::Message;

use iced::widget::{button, column, container, text, text_input};
use iced::{Color, Element, Length};

pub fn view<'a>(state: &TaskPlanner) -> Element<'a, Message> {
    let popup_box = container(
        column![
            text("New Task").size(25),
            text_input("Task Name", &state.add_task_name),
            button("Add Task").width(Length::Fixed(200.0)),
        ]
        .spacing(20)
        .padding(20),
    )
    .width(Length::Fixed(400.0))
    .height(Length::Fill)
    .style(|_theme| {
        let dark_overlay = Color::from_rgba(0.0, 0.0, 0.0, 1.0);

        container::Style {
            background: Some(dark_overlay.into()),
            ..container::Style::default()
        }
    });

    container(popup_box)
        .height(Length::Fill)
        .width(Length::Fill)
        .center(Length::Fill)
        .style(|_theme| {
            let dark_overlay = Color::from_rgba(0.0, 0.0, 0.0, 0.6);

            container::Style {
                background: Some(dark_overlay.into()),
                ..container::Style::default()
            }
        })
        .into()
}
