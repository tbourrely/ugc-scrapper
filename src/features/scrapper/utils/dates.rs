use chrono::{NaiveDate, Datelike, Weekday, Local, Duration};

pub fn get_each_dates_of_current_week (starting_from: Option<Weekday>) -> Vec<NaiveDate> {
    let mut dates = Vec::new();
    let starting_from = starting_from.unwrap_or(Weekday::Tue);

    let today = Local::now().date_naive();
    let current_weekday = today.weekday();

    let days_diff = (starting_from.num_days_from_monday() as i64) -
        (current_weekday.num_days_from_monday() as i64);

    let date_calculated  = today + Duration::days(days_diff);

    for date in date_calculated.iter_days().take(7) {
        dates.push(date);
    }

    dates
}