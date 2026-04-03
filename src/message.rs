use crate::app::Tab;

#[derive(Debug, Clone)]
pub enum Message {
    TabSelected(Tab),
    OpenAddTaskPopup(Option<String>),
    CloseAddTaskPopup,
    TaskNameChanged(String),
    CategoryItemSelected(String),
}
