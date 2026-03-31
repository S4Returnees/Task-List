use crate::message::Message;
use iced::widget::{button, column, container, row, rule, Space};
use iced::{Element, Length};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    AllTasks,
    Calendar,
    Settings,
}

pub struct TaskPlanner {
    active_tab: Tab,
}

impl Default for TaskPlanner {
    fn default() -> Self {
        Self {
            active_tab: Tab::AllTasks,
        }
    }
}

impl TaskPlanner {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::TabSelected(tab) => self.active_tab = tab,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let sidebar = column![
            button("All Task")
                .on_press(Message::TabSelected(Tab::AllTasks))
                .width(Length::Fill),

            button("Calendar")
                .on_press(Message::TabSelected(Tab::Calendar))
            .width(Length::Fill),

            Space::new().height(Length::Fill),

            button("Settings")
                .on_press(Message::TabSelected(Tab::Settings))
                .width(Length::Fill),
        ]
            .padding(10)
            .spacing(10)
            .width(Length::Fixed(200.0))
            .height(Length::Fill);

        let content = container("")
            .width(Length::Fill)
            .height(Length::Fill);

        row![
            sidebar,
            rule::vertical(1),
            content,
        ]
            .into()
    }
}