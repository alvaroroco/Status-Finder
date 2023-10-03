use chrono::{Datelike, NaiveDate, Utc, Weekday};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::env;

fn get_week_number(date_obj: NaiveDate) -> u32 {
    date_obj.iso_week().week()
}

fn str_to_date(string_date: &str) -> Option<NaiveDate> {
    let formatos = vec!["%Y-%m-%d", "%d/%m/%Y", "%m/%d/%Y", "%d-%m-%Y", "%d.%m.%Y"];
    for formato in formatos {
        if let Ok(fecha) = NaiveDate::parse_from_str(string_date, formato) {
            return Some(fecha);
        }
    }
    None
}

fn get_status_by_date(date_obj: NaiveDate) -> String {
    // TODO: Proteger ante errores
    let year = format!("{:02}", date_obj.year() % 100);
    let week_number = get_week_number(date_obj);
    String::from(format!("Status: {}{}", year, week_number))
}

fn get_current_status() -> String {
    get_status_by_date(Utc::now().date_naive())
}

fn get_monday_and_sunday(status: &str) -> String {
    // Parseamos el status a year y week number
    let year = status[..2].parse::<i32>().unwrap() + 2000;
    let week_number = status[status.len() - 2..].parse::<u32>().unwrap();

    // Obtenemos lunes y domingo
    let monday_date = NaiveDate::from_isoywd_opt(year, week_number, Weekday::Mon).unwrap();
    let sunday_date = NaiveDate::from_isoywd_opt(year, week_number, Weekday::Sun).unwrap();

    // Parseamos al formato que nos gusta y devolvemos el String
    let str_monday = monday_date.format("%d.%m.%Y").to_string();
    let str_sunday = sunday_date.format("%d.%m.%Y").to_string();

    String::from(format!("Monday: {}\nSunday: {}", str_monday, str_sunday))
}

fn questions() {
    let selections = &["Get date by status", "Get status by date"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an action")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selections[selection] {
        "Get status by date" => {
            let user_date: String = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter the date (Day-Month-Year)")
                .interact()
                .unwrap();

            match str_to_date(&user_date) {
                Some(date) => println!("{}", get_status_by_date(date)),
                None => println!("Date no valid"),
            }
        }
        "Get date by status" => {
            let user_status: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter the status")
                .interact_text()
                .unwrap();

            println!("{}", get_monday_and_sunday(&user_status));
        }
        _ => unreachable!(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        questions();
        return;
    }

    let input_arg = &args[1];
    if input_arg == "today" {
        println!("{}", get_current_status());
    } else if input_arg.len() != 4 {
        match str_to_date(input_arg) {
            Some(date) => println!("{}", get_status_by_date(date)),
            None => println!("Date no valid"),
        }
    } else {
        println!("{}", get_monday_and_sunday(input_arg));
    }
}
