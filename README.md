# Task List

## 📖 Introduction
Task List is a task management application developed in Rust as part of a group project for the S4 semester at EPITA.

The goal of this project is to provide a simple, fast, and accessible tool for organizing, tracking, and managing tasks. It is designed for both personal use and team collaboration.

Developed as a group project, Task List focuses on performance, reliability, and clean code practices, while exploring modern Rust development techniques.

## 💾 Installation
### 🪟 On Windows (7, 8, 10, 11):
* Download the application [Task_List.exe](https://github.com/S4Returnees/Task-List/releases/download/v1.0/Task_List.exe)
* Run the application

### 🐧 On Linux:
* Download the tar file [Task_List.tar.gz](https://github.com/S4Returnees/Task-List/releases/download/v1.0/Task_List.tar.gz) 
```bash
tar -xvzf Task_List.tar.gz
```
* Run the application
```bash
./Task_List
```

## ❌ Uninstallation
### 🪟 On Windows:
* Delete the Task_List.exe and save.json files 

### 🐧 On Linux:
```bash
rm Task_List
rm save.json
```

## 🛠️ Usage
### 📋 All Task
The All Tasks tab allows users to create, edit, and manage all their tasks in one place.

Each task includes:
* a name
* a status (pending, in progress, done)
* optional attributes such as category, priority, due date, and description

Tasks can also be set as recurring on a daily, weekly, or monthly basis.

Users can visually sort tasks by:
* name
* priority
* due date
* status

### 📅 Calendar
The Calendar tab provides a monthly view of all tasks with a defined due date.

Users can easily navigate between months to track upcoming and past deadlines.

### 🗂️ Categories

The Categories tab allows users to organize tasks by category.

It offers the same functionality as the All Tasks tab, but filtered by category.

Users can:

* create new categories
* rename existing ones
* delete categories

⚠️ Deleting a category does not remove the associated tasks.

### ⚙️ Settings

The Settings tab provides data management features.

Users can:
* save application data
* export and import data
* reset the entire application state

## 📜 License
Copyright (c) 2026 S4Returnees
Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 International (CC BY-NC-ND 4.0)

You are allowed to use this project for personal and non-commercial purposes only. 
Modification, redistribution, or commercial use of this project, in whole or in part, is strictly prohibited.


For full license details, see: https://creativecommons.org/licenses/by-nc-nd/4.0/
