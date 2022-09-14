use chrono::prelude::*;

pub fn birthday() {
    let dt = Utc::now();

    let dt1_year: i64 = 2011;
    let dt1_month: i64 = 9;
    let dt1_day: i64 = 12;

    let age: i64;

    if dt1_month < dt.month() as i64 {
        age = dt.year() as i64 - dt1_year;
    } else if dt1_month == dt.month() as i64 {
        if dt1_day <= dt.day() as i64 {
            age = dt.year() as i64 - dt1_year;
        } else {
            age = dt.year() as i64 - dt1_year - 1;
        }
    } else {
        age = dt.year() as i64 - dt1_year - 1;
    }

    println!("{}歳", age);
}