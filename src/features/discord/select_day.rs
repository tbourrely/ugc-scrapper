use crate::database::models::PollType;
use crate::database::repositories::poll::init_poll_repository;
use crate::errors::Error;
use crate::features::discord::poll_domain::PollApiUpsertPayload;
use crate::features::discord::use_cases::poll_api_use_case::PollApiUseCase;
use chrono::{DateTime, Datelike, Utc};
use sqlx::PgPool;

fn translate_weekday(weekday: chrono::Weekday) -> &'static str {
    match weekday {
        chrono::Weekday::Mon => "lundi",
        chrono::Weekday::Tue => "mardi",
        chrono::Weekday::Wed => "mercredi",
        chrono::Weekday::Thu => "jeudi",
        chrono::Weekday::Fri => "vendredi",
        chrono::Weekday::Sat => "samedi",
        chrono::Weekday::Sun => "dimanche",
    }
}

fn format_day(day: chrono::NaiveDate) -> String {
    format!(
        "{} {:02}/{:02}",
        translate_weekday(day.weekday()),
        day.day(),
        day.month()
    )
}

fn find_next_monday(date: chrono::NaiveDate) -> chrono::NaiveDate {
    let mut next_day = date;
    // if it's already monday, go to next day before looping
    if next_day.weekday() == chrono::Weekday::Mon {
        next_day = next_day + chrono::Duration::days(1);
    }
    while next_day.weekday() != chrono::Weekday::Mon {
        next_day = next_day + chrono::Duration::days(1);
    }
    next_day
}

// generate the next 7 days starting from tuesday to monday of the given week
// example: if today is friday 14 june 2024, the result will be:
// mardi 11/06, mercredi 12/06, jeudi 13/06, vendredi 14/06, samedi 15/06, dimanche 16/06, lundi 17/06
fn generate_week_days(datetime: DateTime<Utc>) -> Vec<String> {
    let mut days = Vec::new();
    // Find the next Monday after the given date
    let monday = find_next_monday(datetime.date_naive());

    // monday is in the end
    for i in (0..7).rev() {
        let day = monday - chrono::Duration::days(i);
        days.push(format_day(day));
    }
    return days;
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_generate_week_days_starts_on_monday() {
        // 2024-06-10 is a Monday
        let datetime = Utc.with_ymd_and_hms(2024, 6, 10, 12, 0, 0);
        let days = generate_week_days(datetime.unwrap());
        let expected = vec![
            "mardi 11/06",
            "mercredi 12/06",
            "jeudi 13/06",
            "vendredi 14/06",
            "samedi 15/06",
            "dimanche 16/06",
            "lundi 17/06",
        ];
        assert_eq!(days, expected);
    }

    #[test]
    fn test_generate_week_days_starts_on_friday() {
        // 2024-06-14 is a Friday
        let datetime = Utc.with_ymd_and_hms(2024, 6, 14, 8, 30, 0);
        let days = generate_week_days(datetime.unwrap());
        // note that it returns the current week (therefore some days in the past)
        let expected = vec![
            "mardi 11/06",
            "mercredi 12/06",
            "jeudi 13/06",
            "vendredi 14/06",
            "samedi 15/06",
            "dimanche 16/06",
            "lundi 17/06",
        ];
        assert_eq!(days, expected);
    }

    #[test]
    fn test_generate_week_days_crosses_month() {
        // 2024-01-29 is a Monday, crosses into February
        let datetime = Utc.with_ymd_and_hms(2024, 1, 29, 0, 0, 0);
        let days = generate_week_days(datetime.unwrap());
        let expected = vec![
            "mardi 30/01",
            "mercredi 31/01",
            "jeudi 01/02",
            "vendredi 02/02",
            "samedi 03/02",
            "dimanche 04/02",
            "lundi 05/02",
        ];
        assert_eq!(days, expected);
    }
}

pub async fn generate_poll_to_select_days(db: &PgPool) -> Result<(), Error> {
    let mut poll: PollApiUpsertPayload = PollApiUpsertPayload::new(
        String::from("00 09 * * 1"),
        String::from("Quel jour ?"),
        generate_week_days(Utc::now()),
    );
    poll.onetime = true;
    poll.duration = 82800; // 23 hours

    let poll_api_use_case = PollApiUseCase::new();
    let poll = match poll_api_use_case
        .initiate_poll_creation(poll, PollType::SelectDay)
        .await
    {
        Ok(poll) => poll,
        Err(e) => return Err(Error::Reqwest(e)),
    };

    let poll_repository = init_poll_repository(db);
    match poll_repository.save(poll).await {
        Ok(poll) => poll,
        Err(e) => return Err(Error::Sqlx(e)),
    };

    Ok(())
}
