use crate::message::Message;
use crate::app::{TaskPlanner, Tab};

use iced::widget::{button, column, container, row, rule, Space, text};
use iced::{Element, Length};

pub fn render_view(state: &TaskPlanner) -> Element<'_, Message> {
    let sidebar = column![
            button("All Task")
                .on_press(Message::TabSelected(Tab::AllTasks))
                .width(Length::Fill),

            button("Calendar")
                .on_press(Message::TabSelected(Tab::Calendar))
            .width(Length::Fill),

            Space::new().height(Length::Fill),

            button("Settings")
                .on_press(Message::TabSelected(Tab::Settings))
                .width(Length::Fill),
        ]
        .padding(10)
        .spacing(10)
        .width(Length::Fixed(200.0))
        .height(Length::Fill);

    let title = match state.active_tab {
        Tab::AllTasks => "All Tasks",
        Tab::Calendar => "Calendar",
        Tab::Settings => "Settings",
    };

    let content = container(
        text(title).size(50)
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center)
    )
        .width(Length::Fill)
        .height(Length::Fill);

    row![
            sidebar,
            rule::vertical(1),
            content,
        ]
        .into()
}