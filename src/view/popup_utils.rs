use crate::app::TaskPlanner;
use crate::message::Message;

use iced::widget::{combo_box, container, text};
use iced::{Color, Element, Length};

pub fn dark_overlay(alpha: f32) -> container::Style {
    let dark_overlay = Color::from_rgba(0.0, 0.0, 0.0, alpha);

    container::Style {
        background: Some(dark_overlay.into()),
        ..container::Style::default()
    }
}

pub fn category_combo_box(state: &'_ TaskPlanner) -> Element<'_, Message> {
    iced::widget::column![
        text("Category").size(14),
        combo_box(
            &state.category_combo_state,
            "",
            state.category_selected_item.as_ref(),
            Message::CategoryItemSelected
        )
        .width(Length::Fill),
    ]
        .into()
}

pub fn priority_combo_box(state: &'_ TaskPlanner) -> Element<'_, Message> {
    iced::widget::column![
        text("Priority").size(14),
        combo_box(
            &state.priority_combo_state,
            "",
            state.priority_selected_item.as_ref(),
            Message::PriorityItemSelected
        )
        .width(Length::Fill),
    ]
        .into()
}
