use chrono::{Datelike, NaiveDate, Utc, Weekday};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::env;

fn get_week_number(date_obj: NaiveDate) -> u32 {
    date_obj.iso_week().week()
}

fn str_to_date(string_date: &str) -> Option<NaiveDate> {
    // Definimos los formatos válidos
    let formatos: Vec<&str> = vec!["%Y-%m-%d", "%d/%m/%Y", "%m/%d/%Y", "%d-%m-%Y", "%d.%m.%Y"];

    // En caso de que consiga parsear la fecha a String, lo devolvemos
    for formato in formatos {
        if let Ok(fecha) = NaiveDate::parse_from_str(string_date, formato) {
            return Some(fecha);
        }
    }

    // En caso de no haber conseguido parsear la fecha, devolvemos None
    None
}

fn get_status_by_date(date: NaiveDate) -> String {
    // Cojo el año de la fecha
    let year: String = format!("{:02}", date.year() % 100);

    // Cojo el numero de la semana de la fecha
    let week_number: u32 = get_week_number(date);

    // Devuelvo el String formateado
    String::from(format!("Status: {}{}", year, week_number))
}

fn get_current_status() -> String {
    // Cojo la fecha actual y ejecuto la función get_status_by_date
    get_status_by_date(Utc::now().date_naive())
}

fn get_monday_and_sunday(status: &str) -> String {
    // Parseamos el status a year y week number
    let year: i32 = status[..2].parse::<i32>().unwrap() + 2000;
    let week_number: u32 = status[status.len() - 2..].parse::<u32>().unwrap();

    // Obtenemos lunes y domingo
    let monday_date: NaiveDate =
        NaiveDate::from_isoywd_opt(year, week_number, Weekday::Mon).unwrap();
    let sunday_date: NaiveDate =
        NaiveDate::from_isoywd_opt(year, week_number, Weekday::Sun).unwrap();

    // Parseamos al formato que nos gusta y devolvemos el String
    let str_monday: String = monday_date.format("%d.%m.%Y").to_string();
    let str_sunday: String = sunday_date.format("%d.%m.%Y").to_string();

    // Devolvemos el String formateado
    String::from(format!("Monday: {}\nSunday: {}", str_monday, str_sunday))
}

fn questions() {
    // Definimos las selecciones
    let selections: &[&str; 2] = &["Get date by status", "Get status by date"];

    // Con dialoguer hacemos la selección
    let selection: usize = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an action")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    // Dependiendo de lo seleccionado, hacemos una acción u otra
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
    // Cogemos los argumentos y los metemos en un vector
    let args: Vec<String> = env::args().collect();

    // Si no hay argumentos, ejecutamos la interfaz TUI
    if args.len() <= 1 {
        questions();
        return;
    }

    // Cogemos el argumento pasado por el usuario
    let input_arg: &String = &args[1];

    if input_arg == "today" {
        // En caso de ser today, devolvemos el status actual
        println!("{}", get_current_status());
    } else if input_arg.len() != 4 {
        // En caso de ser una fecha, devolvemos el status correspondiente
        match str_to_date(input_arg) {
            Some(date) => println!("{}", get_status_by_date(date)),
            None => println!("Date no valid"),
        }
    } else {
        // En caso de ser un status, devolvemos las fechas
        println!("{}", get_monday_and_sunday(input_arg));
    }
}
