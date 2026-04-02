use crate::app::{Tab, TaskPlanner};
use crate::message::Message;
use crate::view::*;

use iced::widget::{container, opaque, row, rule, stack};
use iced::{Element, Length};

pub fn render_view(state: &TaskPlanner) -> Element<'_, Message> {
    let tab_content: Element<'_, Message> = match &state.active_tab {
        Tab::AllTasks => all_tasks::view(state),
        Tab::Calendar => calendar::view(state),
        Tab::Settings => settings::view(state),
        Tab::Category(_category) => todo!(),
    };

    let content = container(tab_content)
        .width(Length::Fill)
        .height(Length::Fill);

    let main_content = row![sidebar::view(state), rule::vertical(1), content,];

    let popup_content = add_task_popup::view(state);

    if state.show_add_task_popup {
        stack![main_content, opaque(popup_content)].into()
    } else {
        main_content.into()
    }
}
