use crate::task_manager::category::*;

pub struct CategoryList
{
    pub list: Vec<Category>,
}

impl CategoryList
{
    pub fn new() -> Self
    {
        CategoryList
        {
            list: Vec::new(),
        }
    }

    pub fn add(&mut self, mut new_category: Category)
    {
        let mut category_id = 1;
        for category in self.list.iter()
        {
            if category.id >= category_id
            {
                category_id = category.id + 1;
            }
        }
        new_category.id = category_id;
        self.list.push(new_category);
    }

    pub fn remove(&mut self, category_id: usize)
    {
        self.list.retain(|x| x.id != category_id);
    }
}
