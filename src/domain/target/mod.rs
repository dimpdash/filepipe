pub trait Target {}

pub type Targets = Vec<Box<dyn Target>>;
