#![windows_subsystem = "windows"] // In order to compile for windows

mod app;
mod message;
mod task_manager;
mod view;

use app::TaskPlanner;
use iced::window::icon;
use iced::{Size, Theme, window};

pub fn main() -> iced::Result {
    let icon = load_icon();
    iced::application(TaskPlanner::default, TaskPlanner::update, TaskPlanner::view)
        .title(|_state: &TaskPlanner| String::from("Task List"))
        .theme(|_state: &TaskPlanner| Theme::Dark)
        .window(window::Settings {
            maximized: true,
            icon,
            ..Default::default()
        })
        .run()
}

fn load_icon() -> Option<icon::Icon> {
    let bytes = include_bytes!("../assets/icon.png");

    let img = image::load_from_memory(bytes).ok()?.to_rgba8();
    let (width, height) = img.dimensions();
    let rgba = img.into_raw();

    icon::from_rgba(rgba, width, height).ok()
}
