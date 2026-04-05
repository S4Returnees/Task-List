use crate::app::Tab;
use iced::widget::text_editor;

#[derive(Debug, Clone)]
pub enum Message {
    TabSelected(Tab),
    OpenAddTaskPopup(Option<String>),
    CloseAddTaskPopup,
    TaskNameChanged(String),
    CategoryItemSelected(String),
    PriorityItemSelected(String),
    TaskDueDateChanged(String),
    TaskDescriptionChanged(text_editor::Action),
    AddTaskButtonPressed,
}
