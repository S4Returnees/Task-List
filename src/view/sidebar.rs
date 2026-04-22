use crate::TaskPlanner;
use crate::app::Tab;
use crate::message::Message;

use crate::task_manager::category::Category;
use iced::widget::{Column, button, column, container, rule, scrollable, text};
use iced::{Element, Length, alignment};

pub fn view(state: &TaskPlanner) -> Element<'_, Message> {
    column![
        button("All Tasks")
            .on_press(Message::TabSelected(Tab::AllTasks))
            .width(Length::Fill),
        button("Calendar")
            .on_press(Message::TabSelected(Tab::Calendar))
            .width(Length::Fill),
        rule::horizontal(5),
        text("Categories")
            .size(15)
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center),
        categories(state),
        add_category_button(),
        rule::horizontal(5),
        button("Settings")
            .on_press(Message::TabSelected(Tab::Settings))
            .width(Length::Fill),
    ]
    .padding(10)
    .spacing(10)
    .width(Length::Fixed(200.0))
    .height(Length::Fill)
    .into()
}

fn categories(state: &TaskPlanner) -> Element<'_, Message> {
    let mut col: Column<Message> = column![].spacing(10);

    let uncategorized = button("Uncategorized")
        .on_press(Message::TabSelected(Tab::Category(0)))
        .width(Length::Fill);

    col = col.push(uncategorized);
    for category in &state.category_list.list {
        col = col.push(category_button(category));
    }
    scrollable(container(col)).height(Length::Fill).into()
}

fn category_button(category: &Category) -> Element<'static, Message> {
    button(text(category.name.clone()))
        .on_press(Message::TabSelected(Tab::Category(category.id)))
        .width(Length::Fill)
        .into()
}

fn add_category_button<'a>() -> Element<'a, Message> {
    container(
        button(
            container(text("+").size(15))
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(alignment::Horizontal::Center)
                .align_y(alignment::Vertical::Center),
        )
        .on_press(Message::OpenAddCategoryPopup)
        .width(Length::Fixed(30.0))
        .height(Length::Fixed(30.0))
        .style(|theme, status| {
            let mut style = button::primary(theme, status);
            style.border.radius = 15.0.into();
            style
        }),
    )
    .padding(5)
    .width(Length::Fill)
    .align_x(alignment::Horizontal::Center)
    .into()
}
