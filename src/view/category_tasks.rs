use crate::TaskPlanner;
use crate::message::Message;
use crate::view::tasks_view_utils::{add_task_button, show_task, sort_by_combo_box};

use iced::widget::{Space, button, column, container, row, rule, svg, text};
use iced::{Element, Length, alignment};

pub fn view(state: &TaskPlanner, id: usize) -> Element<'_, Message> {
    column![
        tab_title(state, id),
        show_task(state, Some(id)),
        add_task_button(id)
    ]
    .spacing(10)
    .into()
}

fn tab_title(state: &TaskPlanner, id: usize) -> Element<'_, Message> {
    let mut category_name = state.category_list.get_name(id);
    if category_name == "None".to_string() {
        category_name = String::from("Uncategorized");
    }

    let mut row = row![
        text(category_name)
            .size(50)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Left),
        Space::new().width(Length::Fill),
        sort_by_combo_box(state),
    ]
    .spacing(20);

    if id != 0 {
        row = row.push(column!(rename_button(id), delete_button(id)).spacing(30))
    }

    column![
        container(row).width(Length::Fill).padding(40),
        rule::horizontal(1)
    ]
    .width(Length::Fill)
    .into()
}

fn delete_button(id: usize) -> Element<'static, Message> {
    button(
        container(
            svg(svg::Handle::from_path("assets/delete.svg"))
                .width(20)
                .height(20),
        )
        .center(Length::Fill),
    )
    .on_press(Message::DeleteCategory(id))
    .width(40)
    .height(40)
    .style(|theme, status| {
        let mut style = button::primary(theme, status);
        style.border.radius = 20.0.into();
        style
    })
    .into()
}

fn rename_button(id: usize) -> Element<'static, Message> {
    button(
        container(
            svg(svg::Handle::from_path("assets/rename.svg"))
                .width(20)
                .height(20),
        )
        .center(Length::Fill),
    )
    .on_press(Message::RenameCategory(id))
    .width(40)
    .height(40)
    .style(|theme, status| {
        let mut style = button::primary(theme, status);
        style.border.radius = 20.0.into();
        style
    })
    .into()
}
