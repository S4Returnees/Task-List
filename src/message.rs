use crate::app::Tab;
use iced::widget::text_editor;
use crate::task_manager::task::{Priority, Status};

#[derive(Debug, Clone)]
pub enum Message {
    TabSelected(Tab),
    OpenAddTaskPopup(Option<String>),
    CloseAddTaskPopup,
    CloseTaskDetailPopup,
    TaskNameChanged(String),
    TaskStatusChanged(Status),
    CategoryItemSelected(String),
    PriorityItemSelected(Priority),
    TaskDueDateChanged(String),
    TaskDescriptionChanged(text_editor::Action),
    AddTaskButtonPressed,
    SortBySelectedItem(String),
    SelectTask(usize)
}
