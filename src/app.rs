use crate::message::Message;
use crate::task_manager::category::Category;
use crate::task_manager::category_list::CategoryList;
use crate::task_manager::saving::*;
use crate::task_manager::task::{Priority, Recurrence, Status, Task};
use crate::task_manager::task_list::{SortBy, TaskList};
use crate::view::view::render_view;

use chrono::{Datelike, Local, NaiveDate};
use iced::Element;
use iced::widget::{combo_box, text_editor};
use rfd::AsyncFileDialog;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tab {
    AllTasks,
    Calendar,
    Settings,
    Category(usize),
}

pub enum Popup {
    None,
    AddTask,
    TaskDetails(usize),
    AddCategory,
    RenameCategory(usize),
}

pub struct TaskPlanner {
    pub task_list: TaskList,
    pub category_list: CategoryList,
    pub active_tab: Tab,
    pub popup: Popup,
    pub add_task_name: String,
    pub status_combo_state: combo_box::State<Status>,
    pub task_status: Option<Status>,
    pub category_combo_state: combo_box::State<String>,
    pub category_selected_item: Option<String>,
    pub priority_combo_state: combo_box::State<Priority>,
    pub priority_selected_item: Option<Priority>,
    pub add_task_due_date: String,
    pub recurrence_combo_state: combo_box::State<Recurrence>,
    pub recurrence_selected_item: Option<Recurrence>,
    pub add_task_description: text_editor::Content,
    pub add_category_name: String,
    pub sort_by_combo_state: combo_box::State<SortBy>,
    pub sort_by_selected_item: Option<SortBy>,
    pub current_year: i32,
    pub current_month: u32,
    pub save_path: PathBuf,
}

impl Default for TaskPlanner {
    fn default() -> Self {
        let save_path = PathBuf::from("save.json");
        let save_data = load(&save_path);

        let mut task_list = TaskList::new();
        task_list.list = save_data.tasks;
        task_list.sort_by(SortBy::Id);

        let mut category_list = CategoryList::new();
        category_list.list = save_data.category;
        category_list.list.sort_unstable_by_key(|c| c.name.clone());
        let category_combo_state = combo_box::State::new(category_list.get_names_list().to_vec());

        Self {
            task_list,
            category_list,
            active_tab: Tab::AllTasks,
            popup: Popup::None,
            add_task_name: String::new(),
            status_combo_state: combo_box::State::new(Status::ALL.to_vec()),
            task_status: None,
            category_combo_state,
            category_selected_item: None,
            priority_combo_state: combo_box::State::new(Priority::ALL.to_vec()),
            priority_selected_item: Some(Priority::None),
            add_task_due_date: String::new(),
            recurrence_combo_state: combo_box::State::new(Recurrence::ALL.to_vec()),
            recurrence_selected_item: Some(Recurrence::None),
            add_task_description: text_editor::Content::new(),
            add_category_name: String::new(),
            sort_by_combo_state: combo_box::State::new(SortBy::ALL.to_vec()),
            sort_by_selected_item: Some(SortBy::Id),
            current_year: Local::now().year(),
            current_month: Local::now().month(),
            save_path,
        }
    }
}

impl TaskPlanner {
    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        let mut task_to_run = iced::Task::none();

        match message {
            Message::TabSelected(tab) => self.active_tab = tab,

            Message::OpenAddTaskPopup(category_id) => {
                self.popup = Popup::AddTask;
                self.category_selected_item = Some(self.category_list.get_name(category_id));
            }
            Message::CloseAddTaskPopup => self.close_add_task_popup(),

            Message::TaskNameChanged(new_name) => self.add_task_name = new_name,
            Message::TaskStatusChanged(status) => self.task_status = Some(status),
            Message::CategoryItemSelected(category) => self.category_selected_item = Some(category),
            Message::PriorityItemSelected(priority) => self.priority_selected_item = Some(priority),
            Message::TaskDueDateChanged(due_date) => self.add_task_due_date = due_date,
            Message::RecurrenceItemSelected(recurrence) => {
                if self.verify_due_date() || self.add_task_name.is_empty() {
                    self.recurrence_selected_item = Some(Recurrence::None)
                } else {
                    self.recurrence_selected_item = Some(recurrence)
                }
            }
            Message::TaskDescriptionChanged(description) => {
                self.add_task_description.perform(description)
            }
            Message::AddTaskButtonPressed => self.add_task_handler(),

            Message::SortBySelectedItem(sort_by) => {
                self.sort_by_selected_item = Some(sort_by);
                self.task_list.sort_by(sort_by);
            }

            Message::SelectTask(id) => self.select_task_detail_popup_handler(id),
            Message::CloseTaskDetailPopup => self.close_task_detail_popup_handler(),

            Message::StatusButton(id) => self.status_button_handler(id),

            Message::OpenAddCategoryPopup => self.popup = Popup::AddCategory,
            Message::CloseAddCategoryPopup => {
                self.add_category_name.clear();
                self.popup = Popup::None;
            }
            Message::CategoryNameChanged(new_name) => self.add_category_name = new_name,
            Message::AddCategoryButtonPressed => self.add_category_popup_handler(),
            Message::RenameCategory(id) => self.rename_category_popup_handler(id),
            Message::CloseRenameCategoryPopup(id) => self.close_rename_category_popup_handler(id),
            Message::DeleteCategory(id) => self.delete_category_handler(id),

            Message::PrevMonth => {
                if self.current_month == 1 {
                    self.current_month = 12;
                    self.current_year -= 1;
                } else {
                    self.current_month -= 1;
                }
            }
            Message::NextMonth => {
                if self.current_month == 12 {
                    self.current_month = 1;
                    self.current_year += 1;
                } else {
                    self.current_month += 1;
                }
            }
            Message::Save => self.save(),
            Message::Export => task_to_run = self.export_file_dialog(),
            Message::Import => task_to_run = self.import_file_dialog(),
            Message::Reset => self.reset(),

            Message::PathSelected(path) => {
            match path {
                Some(path) => self.path_selected_handler(path),
                None => {}
                }
            }
        }
        task_to_run
    }

    fn close_add_task_popup(&mut self) {
        self.popup = Popup::None;
        self.add_task_name.clear();
        self.category_selected_item = Some("None".to_string());
        self.priority_selected_item = Some(Priority::None);
        self.add_task_due_date.clear();
        self.add_task_description = text_editor::Content::new();
    }

    fn add_task_handler(&mut self) {
        if self.add_task_name.is_empty() || self.verify_due_date() {
            return;
        }
        let new_task = Task::new(
            self.add_task_name.clone(),
            self.add_task_description.text(),
            self.category_list.get_id(
                self.category_selected_item
                    .clone()
                    .unwrap_or("None".to_string())
                    .as_str(),
            ),
            self.priority_selected_item.unwrap(),
            NaiveDate::parse_from_str(&self.add_task_due_date, "%Y-%m-%d").ok(),
            self.recurrence_selected_item.unwrap(),
        );
        self.task_list.add(new_task);

        self.task_list.sort_by(self.sort_by_selected_item.unwrap());
        self.save();

        self.close_add_task_popup()
    }
    fn select_task_detail_popup_handler(&mut self, id: usize) {
        let task = self.task_list.list.iter().find(|t| t.id == id).unwrap();
        self.add_task_name = task.name.clone();
        self.task_status = Some(task.status.clone());
        self.category_selected_item = Some(self.category_list.get_name(task.category_id));
        self.priority_selected_item = Some(task.priority);
        self.add_task_due_date = task.get_due_date();
        self.recurrence_selected_item = Some(task.recurrence);
        self.add_task_description = text_editor::Content::with_text(task.description.as_str());

        self.popup = Popup::TaskDetails(id);
    }

    fn close_task_detail_popup_handler(&mut self) {
        if self.verify_due_date() || self.add_task_name.is_empty() {
            return;
        }

        let task_id = match self.popup {
            Popup::TaskDetails(id) => id,
            _ => unreachable!(),
        };
        let task = self
            .task_list
            .list
            .iter_mut()
            .find(|t| t.id == task_id)
            .unwrap();

        task.name = self.add_task_name.clone();
        task.status = self.task_status.unwrap();
        task.category_id = self.category_list.get_id(
            self.category_selected_item
                .clone()
                .unwrap_or("None".to_string())
                .as_str(),
        );
        task.priority = self.priority_selected_item.unwrap();
        task.due_date = NaiveDate::parse_from_str(&self.add_task_due_date, "%Y-%m-%d").ok();
        task.recurrence = self.recurrence_selected_item.unwrap();
        task.description = self.add_task_description.text();

        if task.status == Status::Done {
            self.task_list.handle_recurring_task(task_id);
        }
        self.task_list.sort_by(self.sort_by_selected_item.unwrap());

        self.save();
        self.close_add_task_popup();
    }

    fn verify_due_date(&mut self) -> bool {
        !self.add_task_due_date.is_empty()
            && NaiveDate::parse_from_str(&self.add_task_due_date, "%Y-%m-%d")
                .ok()
                .is_none()
    }

    fn status_button_handler(&mut self, id: usize) {
        let task = self.task_list.list.iter_mut().find(|t| t.id == id).unwrap();

        task.status = match task.status {
            Status::Pending => Status::InProgress,
            Status::InProgress => Status::Done,
            Status::Done => Status::Pending,
        };

        if task.status == Status::Done {
            self.task_list.handle_recurring_task(id);
        }
        self.task_list.sort_by(self.sort_by_selected_item.unwrap());
        self.save();
    }

    fn add_category_popup_handler(&mut self) {
        let new_category = Category::new(self.add_category_name.clone());
        self.category_list.add(new_category);
        self.category_list
            .list
            .sort_unstable_by_key(|c| c.name.clone());
        self.add_category_name.clear();
        self.category_combo_state =
            combo_box::State::new(self.category_list.get_names_list().to_vec());

        self.save();
        self.popup = Popup::None;
    }

    fn delete_category_handler(&mut self, id: usize) {
        self.task_list
            .list
            .iter_mut()
            .filter(|t| t.category_id == id)
            .for_each(|t| t.category_id = 0);
        self.category_list.remove(id);

        self.save();
        self.active_tab = Tab::Category(0)
    }

    fn rename_category_popup_handler(&mut self, id: usize) {
        self.add_category_name = self.category_list.get_name(id);

        self.save();
        self.popup = Popup::RenameCategory(id);
    }

    fn close_rename_category_popup_handler(&mut self, id: usize) {
        if self.add_category_name.is_empty() {
            return;
        }

        let category = self
            .category_list
            .list
            .iter_mut()
            .find(|c| c.id == id)
            .unwrap();
        category.name = self.add_category_name.clone();
        self.category_list
            .list
            .sort_unstable_by_key(|c| c.name.clone());
        self.add_category_name.clear();
        self.category_combo_state =
            combo_box::State::new(self.category_list.get_names_list().to_vec());

        self.save();
        self.popup = Popup::None;
    }

    fn save(&self) {
        let save_data = SaveData {
            tasks: self.task_list.list.clone(),
            category: self.category_list.list.clone(),
        };
        save(&save_data, &self.save_path);
    }

    fn export_file_dialog(&self) -> iced::Task<Message> {
        iced::Task::perform(
            async {
                let handle = AsyncFileDialog::new()
                    .set_title("Select a folder")
                    .pick_folder()
                    .await;
                handle.map(|h| h.path().to_path_buf())
            },
            Message::PathSelected,
        )
    }

    fn import_file_dialog(&self) -> iced::Task<Message> {
        iced::Task::perform(
            async {
                let handle = AsyncFileDialog::new()
                    .set_title("Select a file")
                    .add_filter("JSON File", &["json"])
                    .pick_file()
                    .await;
                handle.map(|h| h.path().to_path_buf())
            },
            Message::PathSelected,
        )
    }

    fn reset(&mut self) {
        self.task_list = TaskList::new();
        self.category_list = CategoryList::new();
        self.save();
        self.sort_by_selected_item = Some(SortBy::Id);
    }

    fn path_selected_handler(&mut self, path: PathBuf) {
        if path.is_dir() {
            self.export(path)
        } else {
            self.import(path)
        }
    }

    fn export(&self, path: PathBuf) {
        self.save();
        let dst = path.join("test.json");
        std::fs::copy(&self.save_path, &dst).expect("Erreur export");
    }

    fn import(&self, path: PathBuf) {
        todo!()
    }

    pub fn view(&self) -> Element<'_, Message> {
        render_view(self)
    }
}
