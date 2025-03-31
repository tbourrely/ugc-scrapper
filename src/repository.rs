use crate::domain::Screening;

pub struct Repository {}
impl Repository {
    pub fn save(_screening: Vec<Screening>) {}
    pub fn _get(_id: uuid::Uuid) {}
    pub fn _delete(_screening: Screening) {}
}
