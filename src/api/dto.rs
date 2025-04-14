use chrono::{NaiveDate, Utc, Datelike};
use chrono::format::ParseErrorKind;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

const CONFLUENCE_THEATER: i8 = 36;
const ASTORIA_THEATER: i8 = 33;
const PART_DIEU_THEATER: i8 = 58;
const CITE_INTERNATIONAL_THEATER: i8 = 32;

#[derive(Debug)]
pub struct UgcFilterPayload {
    pub theaters: Vec<i8>,
    pub dates: Vec<NaiveDate>
}

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct JsonFromRequest {
    #[validate(custom(function = "validate_theaters"))]
    pub theaters: Option<Vec<i8>>,
    #[validate(custom(function = "validate_dates"))]
    pub dates: Option<Vec<String>>
}

impl JsonFromRequest {
    pub fn transform(&self) -> UgcFilterPayload {
        UgcFilterPayload {
            theaters: JsonFromRequest::get_theaters(self.theaters.clone()).unwrap(),
            dates: JsonFromRequest::get_naive_dates(self.dates.clone()).unwrap()
        }
    }

    fn get_theaters(theaters: Option<Vec<i8>>) -> Result<Vec<i8>, ValidationError> {
        match theaters {
            Some(theaters)   => {
                Ok(theaters)
            },
            None => {
                Ok(vec![CONFLUENCE_THEATER, ASTORIA_THEATER, PART_DIEU_THEATER, CITE_INTERNATIONAL_THEATER])
            }
        }
    }

    fn get_naive_dates(dates: Option<Vec<String>>) -> Result<Vec<NaiveDate>, ParseErrorKind> {
        match dates {
            Some(inner)   => {
                let mut dates = Vec::new();
                for date in inner {
                    dates.push(NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap());
                }
                Ok(dates)
            },
            None => {
                let mut dates = Vec::new();
                let today = Utc::now().naive_utc().date();
                for date in today.iter_days().take(7) {
                    dates.push(date);
                }
                Ok(dates)
            },
        }
    }
}

pub fn validate_theaters(theaters: &Vec<i8>) -> Result<(), ValidationError> {
    if theaters.is_empty() {
        return Ok(());
    }

    let all_theaters = vec![CONFLUENCE_THEATER, ASTORIA_THEATER, PART_DIEU_THEATER, CITE_INTERNATIONAL_THEATER];
    for theater in theaters.iter() {
        if !all_theaters.contains(&theater) {
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
            return Err(ValidationError::new("A date is malformed, year must be current one or future."))
        }
    }
    Ok(())
}