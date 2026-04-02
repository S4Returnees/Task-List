use crate::app::Tab;

#[derive(Debug, Clone)]
pub enum Message {
    TabSelected(Tab),
}
