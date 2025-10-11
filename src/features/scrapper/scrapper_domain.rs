use crate::database::models::Theater;
use std::collections::HashMap;

pub type HtmlFromTheatersByDate = HashMap<Theater, HashMap<String, String>>;
