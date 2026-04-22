pub struct Category {
    pub id: usize,
    pub name: String,
}

impl Category {
    pub fn new(name: String) -> Self {
        Category { id: 0, name }
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }
}
