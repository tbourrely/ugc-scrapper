use std::collections::HashMap;
use crate::domain::{TheatersHtmlMap};
use chrono::{DateTime, NaiveDate, Utc};
use reqwest::Error;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, HOST, USER_AGENT};

const CONFLUENCE_THEATER: i8 = 36;
const ASTORIA_THEATER: i8 = 33;
const PART_DIEU_THEATER: i8 = 58;
const CITE_INTERNATIONAL_THEATER: i8 = 32;

pub struct HttpAgent {}
impl HttpAgent {
    pub fn verify_or_set_default_theaters(theaters_from_rq: Option<Vec<i8>>) -> Result<Vec<i8>, Err> {
        match theaters_from_rq {
            Some(inner)   => {
                Ok(inner)
            },
            None => {
                Ok(vec![CONFLUENCE_THEATER, ASTORIA_THEATER, PART_DIEU_THEATER, CITE_INTERNATIONAL_THEATER])
            },
        }
    }

    pub fn verify_or_set_default_dates(dates_from_rq: Option<Vec<String>>) -> Result<Vec<NaiveDate>, Err> {
        match dates_from_rq {
            Some(inner)   => {
                let current_year = Utc::now().date_naive().year();
                let mut dates = Vec::new();
                for date in inner {
                    let dt = DateTime::parse_from_str(&date, "%Y-%m-%d").unwrap();
                    if dt.unwrap().year() != current_year {
                        return Err("Date matched incorrect format");
                    }
                    dates.push(dt);
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

    pub async fn get_theaters_html_pages_by_dates(theaters: Vec<i8>, dates: Vec<NaiveDate>) -> TheatersHtmlMap {
        let mut theaters_html_pages_by_dates: TheatersHtmlMap = HashMap::new();
        for theater in theaters.iter() {
            println!("{}", theater);
            for date in dates.iter() {
                println!("{}", date);
                let mut html_by_date: HashMap<String, String> = HashMap::new();

                let html_page = Self::get_ugc_screening_page_by_theater_by_date(theater, date).await.unwrap();

                html_by_date.insert(date.to_string(), html_page);
                theaters_html_pages_by_dates.insert(*theater, html_by_date);
            }
        }
        theaters_html_pages_by_dates
    }

    async fn get_ugc_screening_page_by_theater_by_date(theater: &i8, date: &NaiveDate) -> Result<String, Box<Error>> {
        let base_url = "https://www.ugc.fr/showingsCinemaAjaxAction!getShowingsForCinemaPage.action";

        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("fr-FR"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/png,image/svg+xml,*/*;q=0.8"));
        headers.insert(HOST, HeaderValue::from_static("www.ugc.fr"));
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:128.0) Gecko/20100101 Firefox/128.0"));

        let client = reqwest::Client::new();
        let url = format!("{}?cinemaId={}&date={}", base_url, theater, date.to_string());
        let response = match client.get(&url)
            .headers(headers)
            .send()
            .await {
                Ok(resp) => resp,
                Err(e) => {
                    eprintln!("Erreur lors de l'envoi de la requête: {}", e);
                    return Err(Box::new(e));
                }
            };

        if !response.status().is_success() {
            return Err(Box::new(response.status().as_str().into()))
        }

        let html_content = match response.text().await {
            Ok(text) => text,
            Err(e) => {
                eprintln!("Erreur lors de la lecture de la réponse: {}", e);
                return Err(Box::new(e));
            }
        };
        Ok(html_content)
    }
}