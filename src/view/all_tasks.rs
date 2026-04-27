use crate::TaskPlanner;
use crate::message::Message;
use crate::view::tasks_view_utils::{add_task_button, show_task, sort_by_combo_box};

use iced::widget::{Space, column, container, row, rule, text};
use iced::{Element, Length, alignment};

pub fn view(state: &TaskPlanner) -> Element<'_, Message> {
    column![tab_title(state), show_task(state, None), add_task_button(0),]
        .spacing(20)
        .into()
}

fn tab_title(state: &TaskPlanner) -> Element<'_, Message> {
    column![
        container(row![
            text("All Tasks")
                .size(50)
                .width(Length::Fill)
                .align_x(alignment::Horizontal::Left)
                .align_y(alignment::Vertical::Center),
            Space::new().width(Length::Fill),
            sort_by_combo_box(state)
        ])
        .width(Length::Fill)
        .padding(40),
        rule::horizontal(1),
    ]
    .width(Length::Fill)
    .into()
}
