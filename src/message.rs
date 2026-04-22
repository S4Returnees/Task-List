use crate::app::Tab;
use crate::task_manager::task::{Priority, Status};
use iced::widget::text_editor;

#[derive(Debug, Clone)]
pub enum Message {
    TabSelected(Tab),

    OpenAddTaskPopup(usize),
    CloseAddTaskPopup,

    TaskNameChanged(String),
    TaskStatusChanged(Status),
    CategoryItemSelected(String),
    PriorityItemSelected(Priority),
    TaskDueDateChanged(String),
    TaskDescriptionChanged(text_editor::Action),
    AddTaskButtonPressed,

    SortBySelectedItem(String),

    SelectTask(usize),
    CloseTaskDetailPopup,
    StatusButton(usize),
    
    OpenAddCategoryPopup,
    CloseAddCategoryPopup,
    CategoryNameChanged(String),
    AddCategoryButtonPressed,
    RenameCategory(usize),
    DeleteCategory(usize),

    NextMonth,
    PrevMonth,
}
