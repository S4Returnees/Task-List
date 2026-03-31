use crate::app::Tab;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    TabSelected(Tab),
}