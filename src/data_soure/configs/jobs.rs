use std::fs::File;

use serde::{Deserialize, Serialize};

pub type Jobs = Vec<Job>;

#[derive(Serialize, Deserialize)]
pub struct Job {
    pub dependencies: Dependencies,
    pub targets: Targets,
    pub transformer: TransformerId,
}

#[derive(Serialize, Deserialize)]
pub struct FileSpecifier(pub FileSpecifierType, pub String);
#[derive(Serialize, Deserialize)]
pub enum FileSpecifierType {
    SingleFile,
    Regex,
}
pub type Dependencies = Vec<FileSpecifier>;
pub type Targets = Vec<FileSpecifier>;
pub type TransformerId = String;
