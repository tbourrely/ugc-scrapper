use crate::database::domain::Theater;

const CONFLUENCE_THEATER: Theater = 36;
const ASTORIA_THEATER: Theater = 33;
const PART_DIEU_THEATER: Theater = 58;
const CITE_INTERNATIONAL_THEATER: Theater = 32;

const LYON_THEATERS: [Theater; 4] = [CONFLUENCE_THEATER, ASTORIA_THEATER, PART_DIEU_THEATER, CITE_INTERNATIONAL_THEATER];

pub fn get_lyon_theaters() -> Vec<Theater> {
    LYON_THEATERS.to_vec()
}