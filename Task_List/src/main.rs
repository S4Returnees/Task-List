#![windows_subsystem = "windows"] // In order to compile for windows


mod app;
mod message;


use app::TaskPlanner;
use iced::{Theme, Size, window};

pub fn main() -> iced::Result {
    iced::application(TaskPlanner::default, TaskPlanner::update, TaskPlanner::view)
        .title(|_state: &TaskPlanner| String::from("Task List"))
        .theme(|_state: &TaskPlanner| Theme::Dark)
        .window(window::Settings {
            size: Size::new(1280.0, 720.0),
            ..Default::default()
        })
        .run()
}