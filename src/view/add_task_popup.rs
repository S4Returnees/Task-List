use crate::TaskPlanner;
use crate::message::Message;
use crate::view::popup_utils::*;

use iced::widget::{Space, button, column, container, row, rule, text, text_editor, text_input};
use iced::{Element, Length};

pub fn view(state: &'_ TaskPlanner) -> Element<'_, Message> {
    let popup_box = container(
        column![
            header(),
            text_input("Task Name", &state.add_task_name).on_input(Message::TaskNameChanged),
            category_combo_box(state),
            priority_combo_box(state),
            text_input("Due Date (yyyy-MM-dd)", &state.add_task_due_date)
                .on_input(Message::TaskDueDateChanged),
            recurring_combo_box(state),
            text_editor(&state.add_task_description)
                .on_action(Message::TaskDescriptionChanged)
                .height(Length::Fill),
            button(text("Add Task").size(20).width(Length::Fill).center())
                .on_press(Message::AddTaskButtonPressed)
                .width(Length::Fill).height(40),
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
    container(
        column![
            row![
                text("New Task").size(30),
                Space::new().width(Length::Fill),
                button("X").on_press(Message::CloseAddTaskPopup),
            ],
            rule::horizontal(1)
        ]
        .spacing(10),
    )
    .into()
}
