use crate::TaskPlanner;
use crate::message::Message;

use chrono::{Datelike, Month, NaiveDate};
use iced::widget::{Space, column, container, row, rule, text};
use iced::{Element, Length, alignment};
use num_traits::FromPrimitive;

pub fn view(state: &TaskPlanner) -> Element<'_, Message> {
    column![title(state), calendar_grid(state)]
        .spacing(10)
        .into()
}

fn title(state: &TaskPlanner) -> Element<'static, Message> {
    let date_name = format!(
        "{} - {}",
        Month::from_u32(state.current_month).unwrap().name(),
        state.current_year
    );

    column![
        text("Calendar")
            .size(25)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Center),
        text(date_name)
            .size(25)
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Center),
        rule::horizontal(1),
    ]
    .into()
}

fn calendar_grid(state: &TaskPlanner) -> Element<'_, Message> {
    let first_day = NaiveDate::from_ymd_opt(state.current_year, state.current_month, 1).unwrap();
    let start_offset = first_day.weekday().number_from_monday();
    let day_in_month = get_days_in_month(state.current_year, state.current_month);
    let n_row = get_row_count(start_offset, day_in_month);
    let mut grid = column![].width(Length::Fill).height(Length::Fill);
    let days_of_week = ["Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim"];
    let mut header_row = row![].width(Length::Fill);

    for day in days_of_week {
        header_row = header_row.push(
            container(text(day).size(15))
                .width(Length::Fill)
                .align_x(alignment::Horizontal::Center),
        );
    }
    grid = grid.push(header_row);

    let mut current_day = 1;
    let mut cell_index = 0;

    for _week in 0..n_row {
        let mut week_row = row![].height(Length::Fill);
        for _day_of_week in 0..7 {
            if cell_index < start_offset || current_day > day_in_month {
                week_row = week_row.push(Space::new().width(Length::Fill).height(Length::Fill));
            } else {
                let cell_date =
                    NaiveDate::from_ymd_opt(state.current_year, state.current_month, current_day)
                        .unwrap();
                week_row = week_row.push(day_cell(state, cell_date));
                current_day += 1;
            }
            cell_index = cell_index + 1;
        }
        grid = grid.push(week_row);
    }

    container(grid)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn get_days_in_month(year: i32, month: u32) -> u32 {
    let (next_year, next_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };

    let first_day = NaiveDate::from_ymd_opt(next_year, next_month, 1).unwrap();
    let last_day = first_day.pred_opt().unwrap();
    last_day.day()
}

fn get_row_count(start_offset: u32, days_in_month: u32) -> u32 {
    let total_cells = start_offset + days_in_month;
    (total_cells + 6) / 7
}
fn day_cell(state: &TaskPlanner, date: NaiveDate) -> Element<'_, Message> {
    let day_number = text(date.day().to_string())
        .size(15)
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Center);

    let content = column![day_number].spacing(5);
    container(content).width(Length::Fill).height(Length::Fill)
        .style(|_theme: &iced::Theme| {
            container::Style {
                text_color: None,
                background: None,
                border: iced::Border {
                    color: iced::Color::from_rgb(0.7, 0.7, 0.7), // Un gris clair pour la séparation
                    width: 1.0,                                  // Épaisseur de 1 pixel
                    radius: 2.0.into(),                          // Coins très légèrement arrondis
                },
                shadow: Default::default(),
                snap: false,
            }
        })
        .into()
}