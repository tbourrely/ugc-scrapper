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

pub fn get_date_from_day_name(days: Vec<String>) -> Result<Vec<NaiveDate>, ()> {
    let mut dates_from_days: Vec<NaiveDate> = Vec::new();
    let today = Local::now().date_naive();

    for day in days {
        let weekday = match day.as_str() {
            "lundi" => Weekday::Mon,
            "mardi" => Weekday::Tue,
            "mercredi" => Weekday::Wed,
            "jeudi" => Weekday::Thu,
            "vendredi" => Weekday::Fri,
            "samedi" => Weekday::Sat,
            "dimanche" => Weekday::Sun,
            _ => return Err(()),
        };

        // get last Tuesday
        let mut start_of_week = today;
        while start_of_week.weekday() != Weekday::Tue {
            start_of_week = start_of_week.pred_opt().unwrap();
        }

        let offset = match weekday {
            Weekday::Tue => 0,
            Weekday::Wed => 1,
            Weekday::Thu => 2,
            Weekday::Fri => 3,
            Weekday::Sat => 4,
            Weekday::Sun => 5,
            Weekday::Mon => 6,
        };

        dates_from_days.push(
            start_of_week.checked_add_days(chrono::Days::new(offset as u64)).unwrap()
        );
    }

    Ok(dates_from_days)
}