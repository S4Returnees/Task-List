use crate::message::Message;
use crate::{TaskPlanner, task_manager};

use iced::widget::{Space, button, column, combo_box, container, row, rule, text};
use iced::{Element, Fill, Length};

pub fn view(state: &TaskPlanner) -> Element<'_, Message> {
    column![
        tab_title(state),
        show_task(state),
        Space::new().height(Length::Fill),
        add_task_button(),
    ]
    .spacing(10)
    .into()
}

pub fn tab_title(state: &TaskPlanner) -> Element<'_, Message> {
    column![
        container(row![
            text("All Tasks")
                .size(25)
                .width(Length::Fill)
                .align_x(iced::alignment::Horizontal::Left),
            Space::new().width(Length::Fill),
            sort_by_combo_box(state)
        ])
        .width(Length::Fill)
        .padding(20),
        rule::horizontal(1),
    ]
    .width(Length::Fill)
    .into()
}

pub fn sort_by_combo_box(state: &TaskPlanner) -> Element<'_, Message> {
    column![
        text("Sort by")
            .size(14)
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center),
        combo_box(
            &state.sort_by_combo_state,
            "",
            state.sort_by_selected_item.as_ref(),
            Message::SortBySelectedItem
        ),
    ]
    .width(Length::Fixed(100.0))
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

fn show_task(state: &TaskPlanner) -> Element<'_, Message> {
    let mut col = column![].spacing(10);

    for task in &state.task_list.list {
        col = col.push(task_button(state, task));
    }

    container(col).padding(20).into()
}

fn task_button<'a>(
    state: &TaskPlanner,
    task: &'a task_manager::task::Task,
) -> Element<'a, Message> {
    button(text(&task.name)).width(Length::Fill).into()
}