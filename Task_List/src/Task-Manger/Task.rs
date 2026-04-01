enum ImportanceType{
    VeryLow,
    Low,
    Medium,
    High,
    Critical,
}

struct Task {
    pub name : String,
    pub description : String,
    pub importance : ImportanceType,
    pub finish : bool,
}

impl Task {
    fn new_task(
        name: String,
        description: Option<String>,
        importance: Option<ImportanceType>,
    ) -> Self {
        Self {
            name,
            description: description.unwrap_or(String::from("")),
            importance: importance.unwrap_or(ImportanceType::Medium),
            finish: false,
        }
    }
    
    fn set_importance(&mut self, imp : ImportanceType){
        self.imortance = imp;
    }
     fn set_description(&mut self, desc : String){
        self.description = desc;
    }
     fn set_name(&mut self, name : String){
        self.name = name;
    }
     fn set_finish (&mut self, fin : bool){
        self.finish = fin;
    }
    
}
