pub trait Dependency {}

pub type Dependencies = Vec<Box<dyn Dependency>>;
