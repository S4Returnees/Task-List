use crate::app::TaskPlanner;
use crate::message::Message;
use crate::view::popup_utils::dark_overlay;

use iced::widget::{Space, button, container, row, text, text_input};
use iced::{Element, Length};

pub fn view(state: &TaskPlanner, id: usize) -> Element<'_, Message> {
    let category_name = state.category_list.get_name(id);
    let popup_box = container(
        iced::widget::column![
            header(id),
            text_input(category_name.as_ref(), &state.add_category_name)
                .on_input(Message::CategoryNameChanged),
        ]
        .spacing(25)
        .padding(20),
    )
    .width(400)
    .style(|_theme| dark_overlay(0.8));

    container(popup_box)
        .height(Length::Fill)
        .width(Length::Fill)
        .center(Length::Fill)
        .style(|_theme| dark_overlay(0.4))
        .into()
}

fn header(id: usize) -> Element<'static, Message> {
    container(
        row![
            text("Rename Category").size(30),
            Space::new().width(Length::Fill),
            button("X").on_press(Message::CloseRenameCategoryPopup(id)),
        ]
        .spacing(10),
    )
    .into()
}
