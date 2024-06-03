use chrono::NaiveDate;

pub fn str_to_date(string_date: &str) -> Option<NaiveDate> {
    let formatos: Vec<&str> = vec!["%Y-%m-%d", "%d/%m/%Y", "%m/%d/%Y", "%d-%m-%Y", "%d.%m.%Y"];

    for formato in formatos {
        if let Ok(fecha) = NaiveDate::parse_from_str(string_date, formato) {
            return Some(fecha)
        }
    }

    None
}
