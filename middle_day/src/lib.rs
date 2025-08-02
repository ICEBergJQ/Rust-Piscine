use chrono::{NaiveDate, Datelike};

fn if_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    if if_leap_year(year) {
        return None;
    } 
    return Some(NaiveDate::from_yo_opt(year as i32, 183)?.weekday());
}
