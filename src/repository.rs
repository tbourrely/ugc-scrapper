use crate::domain::Screening;

pub struct Repository {}
impl Repository {
    pub fn save(screening: Screening) {}
    pub fn get(id: uuid::Uuid) -> Screening {}
    pub fn delete(screening: Screening) {}
}
