use std::collections::HashMap;
use crate::domain::{TheatersHtmlMap};
use chrono::{NaiveDate, Utc, Datelike};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, HOST, USER_AGENT};

const CONFLUENCE_THEATER: i8 = 36;
const ASTORIA_THEATER: i8 = 33;
const PART_DIEU_THEATER: i8 = 58;
const CITE_INTERNATIONAL_THEATER: i8 = 32;

pub struct HttpAgent {}
impl HttpAgent {
    pub fn verify_or_set_default_theaters(theaters_from_rq: Option<Vec<i8>>) -> Result<Vec<i8>, String> {
        let all_theaters = vec![CONFLUENCE_THEATER, ASTORIA_THEATER, PART_DIEU_THEATER, CITE_INTERNATIONAL_THEATER];
        match theaters_from_rq {
            Some(theaters)   => {
                for theater in theaters.iter() {
                    if !all_theaters.contains(&theater) {
                        return Err(String::from("An error occurred while trying to verify theaters."))
                    }
                }
                Ok(theaters)
            },
            None => {
                Ok(all_theaters)
            }
        }
    }

    pub fn verify_or_set_default_dates(dates_from_rq: Option<Vec<String>>) -> Result<Vec<NaiveDate>, String> {
        match dates_from_rq {
            Some(inner)   => {
                let current_year = Utc::now().date_naive().year();
                let mut dates = Vec::new();
                for date in inner {
                    let dt = NaiveDate::parse_from_str(&date, "%Y-%m-%d");
                    match dt {
                        Ok(_date) => (),
                        Err(_date) => return Err(String::from("A date is malformed, valid format : %Y-%m-%d"))
                    }
                    let valid_date = dt.unwrap();
                    if valid_date.year() != current_year {
                        return Err(String::from("A date is malformed, year must be current one or future"))
                    }
                    dates.push(valid_date);
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

    pub async fn get_html_from_theaters_per_dates(theaters: Vec<i8>, dates: Vec<NaiveDate>) -> TheatersHtmlMap {
        let mut theaters_html_pages_by_dates: TheatersHtmlMap = HashMap::new();
        for theater in theaters.iter() {
            for date in dates.iter() {
                let mut html_by_date: HashMap<String, String> = HashMap::new();

                let html_page = Self::get_ugc_screening_page_by_theater_by_date(theater, date).await.unwrap();

                html_by_date.insert(date.to_string(), html_page);
                theaters_html_pages_by_dates.insert(*theater, html_by_date);
            }
        }
        theaters_html_pages_by_dates
    }

    async fn get_ugc_screening_page_by_theater_by_date(theater: &i8, date: &NaiveDate) -> Result<String, Box<dyn std::error::Error>> {
        let base_url = "https://www.ugc.fr/showingsCinemaAjaxAction!getShowingsForCinemaPage.action";

        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("fr-FR"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/png,image/svg+xml,*/*;q=0.8"));
        headers.insert(HOST, HeaderValue::from_static("www.ugc.fr"));
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:128.0) Gecko/20100101 Firefox/128.0"));

        let url = format!("{}?cinemaId={}&date={}", base_url, theater, date.to_string());
        let client = reqwest::Client::new();
        let response = client.get(&url)
            .headers(headers)
            .send()
            .await?
            .text()
            .await?;

        Ok(response)
    }
}