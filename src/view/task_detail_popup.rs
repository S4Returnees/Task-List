use crate::app::{Popup, TaskPlanner};
use crate::message::Message;
use crate::view::popup_utils::*;

use iced::widget::{
    Space, button, column, combo_box, container, row, text, text_editor, text_input,
};
use iced::{Element, Length};

pub fn view(state: &'_ TaskPlanner) -> Element<'_, Message> {
    let task_id = match state.popup {
        Popup::TaskDetails(id) => id,
        _ => unreachable!(),
    };

    let task = state
        .task_list
        .list
        .iter()
        .find(|t| t.id == task_id)
        .unwrap();

    let popup_box = container(
        column![
            header(),
            text_input(task.name.as_ref(), &state.add_task_name).on_input(Message::TaskNameChanged),
            status_combo_box(state),
            category_combo_box(state),
            priority_combo_box(state),
            text_input("Due Date (yyyy-MM-dd)", &state.add_task_due_date)
                .on_input(Message::TaskDueDateChanged),
            text_editor(&state.add_task_description)
                .on_action(Message::TaskDescriptionChanged)
                .height(Length::Fill),
        ]
        .spacing(25)
        .padding(20),
    )
    .width(Length::Fixed(400.0))
    .height(Length::Fill)
    .style(|_theme| dark_overlay(0.8));
    container(popup_box)
        .height(Length::Fill)
        .width(Length::Fill)
        .center(Length::Fill)
        .style(|_theme| dark_overlay(0.4))
        .into()
}

fn header() -> Element<'static, Message> {
    container(row![
        Space::new().width(Length::Fill),
        button("X").on_press(Message::CloseTaskDetailPopup),
    ])
    .into()
}

fn status_combo_box(state: &'_ TaskPlanner) -> Element<'_, Message> {
    column![
        text("Status").size(14),
        combo_box(
            &state.status_combo_state,
            "",
            state.task_status.as_ref(),
            Message::TaskStatusChanged,
        )
        .width(Length::Fill),
    ]
    .into()
}
