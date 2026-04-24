#![windows_subsystem = "windows"] // In order to compile for windows

mod app;
mod message;
mod view;
mod task_manager;

use app::TaskPlanner;
use iced::{Size, Theme, window};

use std::path::Path;
use task_manager::task::*;
use task_manager::saving::{save, load};
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

    let tache_1 = 
    Task::new(
        String::from("Test"),
        String::from("Test"), 
        1,                                     
        Priority::High,                        
        date_limite,                           
        Recurrence::None,                    
    );
    let mut liste_de_taches: Vec<Task> = Vec::new();
    liste_de_taches.push(tache_1);

    save(&liste_de_taches, fichier);

    let nbr_tache = load(fichier);
    println!("Nbr taches : {}", nbr_tache.len());

    let tache_2 = 
    Task::new(
        String::from("Test2"),
        String::from("Test2"), 
        5,                                     
        Priority::Critical,                        
        date_limite,                           
        Recurrence::Monthly,                    
    );
    liste_de_taches.push(tache_2);
    save(&liste_de_taches, fichier);

    let nbr_tache_2 = load(fichier);
    println!("Nbr taches : {}", nbr_tache_2.len());

    let tache_3 = 
    Task::new(
        String::from("Test3"),
        String::from("Test3"), 
        50000000,                                     
        Priority::Medium,                        
        date_limite,                           
        Recurrence::Daily,                    
    );
    liste_de_taches.push(tache_3);
    save(&liste_de_taches, fichier);

    let nbr_tache_3 = load(fichier);
    println!("Nbr taches : {}", nbr_tache_3.len());

}
