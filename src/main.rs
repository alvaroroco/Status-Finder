mod dates;
mod status;
mod ui;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        ui::questions();
        return;
    }

    let input_arg: &String = &args[1];

    if input_arg == "today" {
        println!("{}", status::get_current_status());
    } else if input_arg.len() != 4 {
        match dates::str_to_date(input_arg) {
            Some(date) => println!("{}", status::get_status_by_date(date)),
            None => println!("Date no valid"),
        }
    } else {
        println!("{}", status::get_monday_and_sunday(input_arg))
    }
}
