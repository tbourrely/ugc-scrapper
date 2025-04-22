use std::collections::HashMap;
use chrono::{NaiveDate};
use reqwest::Error;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, HOST, USER_AGENT};
use crate::database::domain::{HtmlFromTheatersByDate, Theater};

pub struct Ugc {}
impl Ugc {

    pub async fn get_html_from_theaters_per_dates(theaters: Vec<Theater>, dates: Vec<NaiveDate>) -> Result<HtmlFromTheatersByDate, Error> {
        let mut theaters_html_pages_by_dates: HtmlFromTheatersByDate = HashMap::new();
        for theater in theaters.iter() {
            for date in dates.iter() {
                let mut html_by_date: HashMap<String, String> = HashMap::new();

                let html_page = match Self::get_ugc_screening_page_by_theater_by_date(theater, date).await {
                    Ok(page) => page,
                    Err(reqwest) => return Err(reqwest),
                };

                html_by_date.insert(date.to_string(), html_page);
                theaters_html_pages_by_dates.insert(*theater, html_by_date);
            }
        }
        Ok(theaters_html_pages_by_dates)
    }

    async fn get_ugc_screening_page_by_theater_by_date(theater: &Theater, date: &NaiveDate) -> Result<String, Error> {
        let base_url = "https://www.ugc.fr/showingsCinemaAjaxAction!getShowingsForCinemaPage.action";

        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("fr-FR"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/png,image/svg+xml,*/*;q=0.8"));
        headers.insert(HOST, HeaderValue::from_static("www.ugc.fr"));
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:128.0) Gecko/20100101 Firefox/128.0"));

        let url = format!("{}?cinemaId={}&date={}", base_url, theater, date.to_string());
        let client = reqwest::Client::new();
        let response = match client.get(&url).headers(headers).send().await {
            Ok(response) => response,
            Err(reqwest_error) => return Err(reqwest_error)
        };

        let response_content = match response.text().await {
            Ok(response_content) => response_content,
            Err(reqwest_error) => return Err(reqwest_error)
        };

        Ok(response_content)
    }
}