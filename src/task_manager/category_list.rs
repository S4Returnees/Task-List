use crate::task_manager::category::*;

pub struct CategoryList {
    pub list: Vec<Category>,
}

impl CategoryList {
    pub fn new() -> Self {
        CategoryList { list: Vec::new() }
    }

    pub fn add(&mut self, mut new_category: Category) {
        let mut category_id = 1;
        for category in self.list.iter() {
            if category.id >= category_id {
                category_id = category.id + 1;
            }
        }
        new_category.id = category_id;
        self.list.push(new_category);
    }

    pub fn remove(&mut self, category_id: usize) {
        self.list.retain(|x| x.id != category_id);
    }

    pub fn get_names_list(&self) -> Vec<String> {
        let mut res: Vec<String> = self.list.iter().map(|x| x.name.clone()).collect();
        res.insert(0, String::from("None"));
        res
    }

    pub fn get_name(&self, category_id: Option<usize>) -> String {
        if let Some(id) = category_id {
            self.list.iter().find(|x| x.id == id).unwrap().name.clone()
        } else {
            "None".to_string()
        }
    }

    pub fn get_id(&self, name: &str) -> Option<usize> {
        let category = self.list.iter().find(|x| x.name == name);
        if let Some(category) = category {
            Some(category.id)
        } else {
            None
        }
    }
}
