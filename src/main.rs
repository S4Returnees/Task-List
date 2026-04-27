#![windows_subsystem = "windows"] // In order to compile for windows

mod app;
mod message;
mod view;
mod task_manager;

use app::TaskPlanner;
use iced::{Size, Theme, window};

use std::path::Path;
use task_manager::task::*;
use task_manager::task_list::*;
use task_manager::category::*;
use task_manager::category_list::*;
use task_manager::saving::*;
use chrono::NaiveDate;

pub fn main() /*-> iced::Result*/ {
    /*iced::application(TaskPlanner::default, TaskPlanner::update, TaskPlanner::view)
        .title(|_state: &TaskPlanner| String::from("Task List"))
        .theme(|_state: &TaskPlanner| Theme::Dark)
        .window(window::Settings {
            //maximized: true,
            size: Size::new(1280.0, 720.0),
            ..Default::default()
        })
        .run()  */




    let fichier = Path::new("test.json");
    let date_limite = NaiveDate::from_ymd_opt(2026, 5, 20);

    let mut categories = CategoryList
    {
        list: Vec::new(),
    };

    let category_1 = Category::new(String::from("Test1"));
    let category_2 = Category::new(String::from("Octave"));
    let category_3 = Category::new(String::from("TP"));

    CategoryList::add(&mut categories, category_1);
    CategoryList::add(&mut categories, category_2);
    CategoryList::add(&mut categories, category_3);


    let mut tasks = TaskList
    {
        list: Vec::new(),
    };

    let tache_1 = 
    Task::new(
        String::from("Test"),
        String::from("Test"), 
        1,                                     
        Priority::High,                        
        date_limite,                           
        Recurrence::None,                    
    );
   
    let tache_2 = 
    Task::new(
        String::from("Test2"),
        String::from("Test2"), 
        5,                                     
        Priority::Critical,                        
        date_limite,                           
        Recurrence::Monthly,                    
    );
    
    let tache_3 = 
    Task::new(
        String::from("Test3"),
        String::from("Test3"), 
        50000000,                                     
        Priority::Medium,                        
        date_limite,                           
        Recurrence::Daily,                    
    );

    TaskList::add(&mut tasks, tache_1);
    TaskList::add(&mut tasks, tache_2);
    TaskList::add(&mut tasks, tache_3);
    let test = SaveData
    {
        category: categories.list,
        tasks: tasks.list,
    };

    save(&test, fichier);

    let data = load(fichier);
    println!("{} categories", data.category.len());
    println!("{} tasks", data.tasks.len());



}
