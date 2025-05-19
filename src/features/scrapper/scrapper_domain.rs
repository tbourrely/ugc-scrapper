use std::collections::HashMap;
use crate::database::models::{Theater};

pub type HtmlFromTheatersByDate = HashMap<Theater, HashMap<String, String>>;

