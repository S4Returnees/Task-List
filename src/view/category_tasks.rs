use crate::TaskPlanner;
use crate::message::Message;

use crate::view::tasks_view_utils::{add_task_button, show_task, sort_by_combo_box};
use iced::widget::{Space, column, container, row, rule, text};
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

    column![
        container(row![
            text(category_name)
                .size(25)
                .width(Length::Fill)
                .align_x(alignment::Horizontal::Left),
            Space::new().width(Length::Fill),
            // todo rem and edit button
            sort_by_combo_box(state)
        ])
        .width(Length::Fill)
        .padding(20),
        rule::horizontal(1)
    ]
    .width(Length::Fill)
    .into()
}
