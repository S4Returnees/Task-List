use crate::TaskPlanner;
use crate::app::Tab;
use crate::message::Message;

use iced::widget::{Space, button, column, rule, text};
use iced::{Element, Length};

pub fn view(_state: &TaskPlanner) -> Element<'_, Message> {
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
        
        button("Uncategorized")
            .on_press(Message::TabSelected(Tab::Category(
                "Uncategorized".to_string()
            )))
            .width(Length::Fill),
        
        Space::new().height(Length::Fill),
        
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
