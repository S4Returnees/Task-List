use crate::TaskPlanner;
use crate::message::Message;
use crate::task_manager::task::{Priority, Status, Task};
use chrono::NaiveDate;

use iced::border::Radius;
use iced::widget::{Space, button, column, combo_box, container, row, rule, svg, text};
use iced::window::Position::Default;
use iced::{Border, Color, Element, Length, alignment};

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
                .align_x(alignment::Horizontal::Left),
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
            .align_x(alignment::Horizontal::Center),
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
                .align_x(alignment::Horizontal::Center),
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
    .align_x(alignment::Horizontal::Center)
    .into()
}

fn show_task(state: &TaskPlanner) -> Element<'_, Message> {
    let mut col = column![].spacing(10);

    for task in &state.task_list.list {
        col = col.push(task_button(task));
    }

    container(col).padding(20).into()
}

fn task_button(task: &'_ Task) -> Element<'_, Message> {
    let status_icon = match task.status {
        Status::Pending => "assets/pending.svg",
        Status::InProgress => "assets/in-progress.svg",
        Status::Done => "assets/done.svg",
    };

    row![
        button(
            container(
                svg(svg::Handle::from_path(status_icon))
                    .width(20)
                    .height(20)
            )
            .center(Length::Fill)
        )
        .on_press(Message::StatusButton(task.id))
        .width(40)
        .height(Length::Fill)
        .style(|theme, status| {
            let mut style = button::primary(theme, status);
            style.border.radius = Radius::new(0.0).left(8.0);
            style
        }),
        button(
            row![
                text(&task.name).align_y(alignment::Vertical::Center),
                Space::new().width(Length::Fill),
                task_due_date_indicator(task.get_due_date()),
                task_priority_indicator(task.priority)
            ]
            .spacing(10)
            .align_y(alignment::Vertical::Center)
            .height(Length::Fill),
        )
        .on_press(Message::SelectTask(task.id))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|theme, status| {
            let mut style = button::primary(theme, status);
            style.border.radius = Radius::new(0.0).right(8.0);
            style
        }),
    ]
    .height(40)
    .spacing(0)
    .into()
}

fn task_priority_indicator(priority: Priority) -> Element<'static, Message> {
    if priority == Priority::None {
        return Space::new().into();
    }
    let color = match priority {
        Priority::Critical => Color::from_rgb8(224, 49, 49),
        Priority::High => Color::from_rgb8(247, 103, 7),
        Priority::Medium => Color::from_rgb8(245, 159, 0),
        Priority::Low => Color::from_rgb8(34, 139, 230),
        Priority::Optional => Color::from_rgb8(173, 181, 189),
        Priority::None => unreachable!(),
    };

    container(Space::new())
        .width(20)
        .height(20)
        .style(move |_theme| container::Style {
            background: Some(color.into()),
            border: Border {
                radius: Radius::from(10.0),
                ..Border::default()
            },
            ..container::Style::default()
        })
        .into()
}

fn task_due_date_indicator(due_date: String) -> Element<'static, Message> {
    if due_date.is_empty() {
        return Space::new().into();
    }
    text(due_date).align_x(alignment::Horizontal::Right).into()
}
