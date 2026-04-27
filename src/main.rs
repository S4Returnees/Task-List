#![windows_subsystem = "windows"] // In order to compile for windows

mod app;
mod message;
mod task_manager;
mod view;

use app::TaskPlanner;
use iced::{Size, Theme, window};

pub fn main() -> iced::Result {
    iced::application(TaskPlanner::default, TaskPlanner::update, TaskPlanner::view)
        .title(|_state: &TaskPlanner| String::from("Task List"))
        .theme(|_state: &TaskPlanner| Theme::Dark)
        .window(window::Settings {
            //maximized: true,
            size: Size::new(1280.0, 720.0),
            ..Default::default()
        })
        .run()
}
