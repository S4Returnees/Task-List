use crate::TaskPlanner;
use crate::app::Tab;
use crate::message::Message;

use crate::task_manager::category::Category;
use iced::widget::{Column, button, column, container, rule, scrollable, text};
use iced::{Element, Length, alignment};

pub fn view(state: &TaskPlanner) -> Element<'_, Message> {
    column![
        button(text("All Tasks").size(20).center())
            .on_press(Message::TabSelected(Tab::AllTasks))
            .width(Length::Fill)
            .height(40),
        button(text("Calendar").size(20).center())
            .on_press(Message::TabSelected(Tab::Calendar))
            .width(Length::Fill)
            .height(40),
        rule::horizontal(5),
        text("Categories")
            .size(20)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Center),
        categories(state),
        add_category_button(),
        rule::horizontal(5),
        button(text("Settings").size(20).center())
            .on_press(Message::TabSelected(Tab::Settings))
            .width(Length::Fill)
            .height(40),
    ]
    .padding(20)
    .spacing(15)
    .width(300)
    .height(Length::Fill)
    .into()
}

fn categories(state: &TaskPlanner) -> Element<'_, Message> {
    let mut col: Column<Message> = column![].spacing(10);

    let uncategorized = button(text("Uncategorized").size(20).center())
        .on_press(Message::TabSelected(Tab::Category(0)))
        .width(Length::Fill)
        .height(40);

    col = col.push(uncategorized);
    for category in &state.category_list.list {
        col = col.push(category_button(category));
    }
    scrollable(container(col)).height(Length::Fill).into()
}

fn category_button(category: &Category) -> Element<'static, Message> {
    button(text(category.name.clone()).size(20).center())
        .on_press(Message::TabSelected(Tab::Category(category.id)))
        .width(Length::Fill)
        .height(40)
        .into()
}

fn add_category_button<'a>() -> Element<'a, Message> {
    container(
        button(
            container(text("+").size(20))
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(alignment::Horizontal::Center)
                .align_y(alignment::Vertical::Center),
        )
        .on_press(Message::OpenAddCategoryPopup)
        .width(Length::Fixed(40.0))
        .height(Length::Fixed(40.0))
        .style(|theme, status| {
            let mut style = button::primary(theme, status);
            style.border.radius = 20.0.into();
            style
        }),
    )
    .padding(5)
    .width(Length::Fill)
    .align_x(alignment::Horizontal::Center)
    .into()
}
