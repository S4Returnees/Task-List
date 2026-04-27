use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Category {
    pub id: usize,
    pub name: String,
}

impl Category {
    pub fn new(name: String) -> Self {
        Category { id: 0, name }
    }
}
