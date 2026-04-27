use crate::app::Tab;
use crate::task_manager::task::{Priority, Recurrence, Status};
use crate::task_manager::task_list::SortBy;

use iced::widget::text_editor;
use std::path::PathBuf;

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
    RecurrenceItemSelected(Recurrence),
    TaskDescriptionChanged(text_editor::Action),
    AddTaskButtonPressed,

    SortBySelectedItem(SortBy),

    SelectTask(usize),
    CloseTaskDetailPopup,
    StatusButton(usize),

    OpenAddCategoryPopup,
    CloseAddCategoryPopup,
    CategoryNameChanged(String),
    AddCategoryButtonPressed,
    RenameCategory(usize),
    CloseRenameCategoryPopup(usize),
    DeleteCategory(usize),

    NextMonth,
    PrevMonth,

    Save,
    Export,
    Import,
    Reset,

    PathSelected(Option<PathBuf>),
}
