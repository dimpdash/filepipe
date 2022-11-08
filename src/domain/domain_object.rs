use std::error::Error;
pub trait DomainObject {
    fn insert(&self) -> Result<(), Box<dyn Error>>;
    fn update(&self) -> Result<(), Box<dyn Error>>;
    fn delete(&self) -> Result<(), Box<dyn Error>>;
}