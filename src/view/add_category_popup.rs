use crate::app::TaskPlanner;
use crate::message::Message;
use crate::view::popup_utils::dark_overlay;

use iced::widget::{Space, button, column, container, row, text, text_input};
use iced::{Element, Length};

pub fn view(state: &TaskPlanner) -> Element<'_, Message> {
    let popup_box = container(
        column![
            header(),
            text_input("Category Name", &state.add_category_name)
                .on_input(Message::CategoryNameChanged),
            button("Add Category")
                .on_press(Message::AddCategoryButtonPressed)
                .width(Length::Fill)
        ]
        .spacing(25)
        .padding(20),
    )
    .width(Length::Fixed(400.0))
    .height(Length::Fixed(400.0))
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
        row![
            text("New Category").size(25),
            Space::new().width(Length::Fill),
            button("X").on_press(Message::CloseAddCategoryPopup),
        ]
        .spacing(10),
    )
    .into()
}
