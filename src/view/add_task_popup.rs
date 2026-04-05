use crate::TaskPlanner;
use crate::message::Message;

use iced::widget::{
    Space, button, column, combo_box, container, row, rule, text, text_editor, text_input,
};
use iced::{Color, Element, Length};

pub fn view(state: &'_ TaskPlanner) -> Element<'_, Message> {
    let popup_box = container(
        column![
            header(),
            text_input("Task Name", &state.add_task_name).on_input(Message::TaskNameChanged),
            category_combo_box(state),
            priority_combo_box(state),
            text_input("Due Date (yyyy-MM-dd)", &state.add_task_due_date)
                .on_input(Message::TaskDueDateChanged),
            text_editor(&state.add_task_description)
                .on_action(Message::TaskDescriptionChanged)
                .height(Length::Fill),
            button("Add Task")
                .on_press(Message::AddTaskButtonPressed)
                .width(Length::Fill),
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

fn dark_overlay(alpha: f32) -> container::Style {
    let dark_overlay = Color::from_rgba(0.0, 0.0, 0.0, alpha);

    container::Style {
        background: Some(dark_overlay.into()),
        ..container::Style::default()
    }
}
fn header() -> Element<'static, Message> {
    container(
        column![
            row![
                text("New Task").size(25),
                Space::new().width(Length::Fill),
                button("X").on_press(Message::CloseAddTaskPopup),
            ],
            rule::horizontal(1)
        ]
        .spacing(10),
    )
    .into()
}

fn category_combo_box(state: &'_ TaskPlanner) -> Element<'_, Message> {
    column![
        text("Category").size(14),
        combo_box(
            &state.category_combo_state,
            "",
            state.category_selected_item.as_ref(),
            Message::CategoryItemSelected
        )
        .width(Length::Fill),
        //state.category_combo_state.push(new_category); to add a category
    ]
    .into()
}

fn priority_combo_box(state: &'_ TaskPlanner) -> Element<'_, Message> {
    column![
        text("Priority").size(14),
        combo_box(
            &state.priority_combo_state,
            "",
            state.priority_selected_item.as_ref(),
            Message::PriorityItemSelected
        )
        .width(Length::Fill),
    ]
    .into()
}
