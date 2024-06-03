use dialoguer::{theme::ColorfulTheme, Input, Select};

use crate::dates;
use crate::status;

pub fn questions() {
    let selections: &[&str; 2] = &["Get date by status", "Get status by date"];

    let selection: usize = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an action")
        .default(0).items(&selections[..])
        .interact().unwrap();

    match selections[selection] {
        "Get status by date" => {
            let user_date: String = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter the date (Day-Month-Year)")
                .interact().unwrap();

            match dates::str_to_date(&user_date) {
                Some(date) => println!("{}", status::get_status_by_date(date)),
                None => println!("Date no valid"),
            }
        }
        "Get date by status" => {
            let user_status: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter the status")
                .interact_text()
                .unwrap();

            println!("{}", status::get_monday_and_sunday(&user_status));
        }
        _ => unreachable!(),
    }
}