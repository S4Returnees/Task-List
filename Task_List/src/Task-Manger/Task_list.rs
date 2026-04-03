mod task;
use crate::task::Task;

struct TypeList{
    Today,
    Week,
    Important,
    ShoppingList,
    Personalised(String),
}

struct Task_List {
    typeL : TypeList,
    list : Vec<Task>,
    done : bool,
}

impl Task_list {
    fn new_list(typeL : TypeList, list : Vec<Task>, done : bool){
        Self {
            typeL,
            list,
            done: done.unwrap_or(bool::false),
            }
    }
    fn add_task(self, Task){
        self.list.push(Task);
    }
    fn rm_task(self, )name : String{
        todo!()
    }
}

