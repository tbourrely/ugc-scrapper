use std::borrow::Cow;
use chrono::{NaiveDate, Utc, Datelike, Weekday, Local, Duration};
use reqwest::Error;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use crate::database::domain::LYON_THEATERS;
use crate::services::discord::{Discord};

#[derive(Debug)]
pub struct UgcFilterPayload {
    pub theaters: Vec<i16>,
    pub dates: Vec<NaiveDate>
}

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct JsonFromRequest {
    #[validate(custom(function = "validate_theaters"))]
    pub theaters: Option<Vec<i16>>,
    #[validate(custom(function = "validate_dates"))]
    pub dates: Option<Vec<String>>
}

impl JsonFromRequest {
    pub async fn transform(&self) -> Result<UgcFilterPayload, Error> {
        let transformed_dates = match self.dates.clone() {
            Some(d) => {
                JsonFromRequest::get_naive_dates(d)
            }
            None => {
                let discord_naives_dates = JsonFromRequest::get_naive_dates_from_discord().await;
                if discord_naives_dates.is_err() {
                    return Err(discord_naives_dates.err().unwrap())
                }
                discord_naives_dates?
            }
        };

        Ok(UgcFilterPayload {
            theaters: JsonFromRequest::get_theaters(self.theaters.clone()).unwrap(),
            dates: transformed_dates
        })
    }

    fn get_theaters(theaters: Option<Vec<i16>>) -> Result<Vec<i16>, ValidationError> {
        match theaters {
            Some(theaters)   => {
                Ok(theaters)
            },
            None => {
                Ok(LYON_THEATERS.to_vec())
            }
        }
    }

    fn get_naive_dates(dates_from_rq: Vec<String>) -> Vec<NaiveDate> {
        let mut dates = Vec::new();
        for date in dates_from_rq {
            dates.push(NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap());
        }
        dates
    }

    async fn get_naive_dates_from_discord() -> Result<Vec<NaiveDate>, Error> {
        let answers = match Discord::get_answers_from_last_poll().await {
            Ok(poll) => poll,
            Err(e) => return Err(e)
        };

        let mut dates: Vec<NaiveDate> = Vec::new();
        for poll_answer in answers {
            let target_weekday = match poll_answer.answer.as_str() {
                "lundi" => Weekday::Mon,
                "mardi" => Weekday::Tue,
                "mercredi" => Weekday::Wed,
                "jeudi" => Weekday::Thu,
                "vendredi" => Weekday::Fri,
                "samedi" => Weekday::Sat,
                "dimanche" => Weekday::Sun,
                _ => unreachable!(),
            };

            let today = Local::now().date_naive();
            let current_weekday = today.weekday();

            let days_diff = (target_weekday.num_days_from_monday() as i64) -
                (current_weekday.num_days_from_monday() as i64);

            dates.push(today + Duration::days(days_diff));
        }

        Ok(dates)
    }
}

pub fn validate_theaters(theaters: &Vec<i16>) -> Result<(), ValidationError> {
    if theaters.is_empty() {
        return Ok(());
    }

    for theater in theaters.iter() {
        if !LYON_THEATERS.contains(&theater) {
            return Err(ValidationError {
                code: "invalid_date_format".into(),
                message: Some(format!(
                    "theater {} is unknown",
                    theater
                ).into()),
                params: Default::default(),
            });
        }
    }

    Ok(())
}

pub fn validate_dates(dates: &Vec<String>) -> Result<(), ValidationError> {

    if dates.is_empty() {
        return Ok(());
    }

    let current_year = Utc::now().date_naive().year();
    for date_str in dates.iter() {
        let valid_date = match NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
            Ok(dt) => { dt }
            Err(_) => {
                return Err(ValidationError {
                    code: "invalid_date_format".into(),
                    message: Some(format!(
                        "Date {} isn't in the correct format : %Y-%m-%d",
                        date_str
                    ).into()),
                    params: Default::default(),
                });
            }
        };
        if valid_date.year() != current_year {
            return Err(ValidationError {
                code: "invalid_date_format".into(),
                message: Some(Cow::from("A date is malformed, year must be current one or future.")),
                params: Default::default(),
            });
        }
    }
    Ok(())
}