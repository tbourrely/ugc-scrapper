use crate::domain::Screening;

pub struct Repository {}
impl Repository {
    pub fn save(screening: Vec<Screening>) {}
    pub fn get(id: uuid::Uuid) -> Screening {}
    pub fn delete(screening: Screening) {}
}
