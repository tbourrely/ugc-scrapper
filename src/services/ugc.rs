use chrono::{Days, Utc};
use const_format::{concatcp, formatcp};

const CONFLUENCE_THEATER: i8 = 36;
const ASTORIA_THEATER: i8 = 33;
const PART_DIEU_THEATER: i8 = 58;
const CITE_INTERNATIONAL_THEATER: i8 = 32;
const UGC_THEATER_URL: &str = "https://www.ugc.fr/showingsCinemaAjaxAction!getShowingsForCinemaPage.action";

pub struct Ugc {
    html_pages_per_theaters_per_dates: Vec<String>,
}

impl Ugc {
    pub fn get_theater_movies(theaters: Vec<i8>, dates: Vec<String>) {
        let mut targeted_theaters = vec![CONFLUENCE_THEATER, ASTORIA_THEATER, PART_DIEU_THEATER, CITE_INTERNATIONAL_THEATER];

        if theaters.len() > 0 {
            targeted_theaters = theaters
        }

        println!("Theaters: {:?}", targeted_theaters);

        let mut targeted_dates = Vec::new();
        let current_date = Utc::now().naive_utc().date().checked_add_days(Days::new(1)).unwrap();
        for date in current_date.iter_days().take(7) {
            targeted_dates.push(date.to_string());
        }

        if dates.len() > 0 {
            targeted_dates = dates
        }

        println!("Dates: {:?}", targeted_dates);

        // maybe one day it would work
        // let formatted_theaters = targeted_theaters.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        // let formatted_dates = targeted_dates.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        // let url_formatted: String = UGC_THEATER_URL.to_owned() + "?cinemaId=" + &formatted_theaters + "&date=" + &formatted_dates;

        // println!("{}", url_formatted);

        // flatten theaters & dates
        for theater in targeted_theaters.iter() {
            for targeted_date in targeted_dates.iter() {

            }
        }
    }
}



